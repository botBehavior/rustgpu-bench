//! Differential fuzzer for rust-gpu: generate pure integer Rust functions,
//! compile them to SPIR-V via cargo-gpu, run on the GPU, and diff against a
//! CPU interpreter of the same AST. Integer/bool/control-flow domain only —
//! results must be bit-exact, so ANY mismatch is a finding.
//!
//! Usage: diff-fuzz [batches] [seed0]   (default 1 batch, seed 1)
//!
//! Ground truth on mismatch: the generated source is plain Rust — recompile
//! the repro natively with rustc and the CPU binary is the arbiter (the
//! interpreter itself could be wrong; that's a finding about the fuzzer).

use std::fmt::Write as _;
use std::process::Command;
use std::time::Instant;

const N_FNS: u32 = 48;
const N_INPUTS: u32 = 4096;
const MAX_DEPTH: u32 = 5;

// ---------- deterministic rng ----------
struct Rng(u32);
impl Rng {
    fn next(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u32) -> u32 {
        self.next() % n
    }
}

fn lcg(s: u32) -> u32 {
    s.wrapping_mul(1664525).wrapping_add(1013904223)
}

// ---------- AST ----------
#[derive(Clone, Debug)]
enum Node {
    X,
    Y,
    C(u32),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Or(Box<Node>, Box<Node>),
    Xor(Box<Node>, Box<Node>),
    Shl(Box<Node>, Box<Node>),  // rhs masked & 31
    Shr(Box<Node>, Box<Node>),  // rhs masked & 31
    Div(Box<Node>, Box<Node>),  // rhs | 1 (no div-by-zero)
    Rem(Box<Node>, Box<Node>),  // rhs | 1
    Sel(Box<Node>, Box<Node>, Box<Node>, Box<Node>), // if a < b { t } else { f }
    Loop(Box<Node>, Box<Node>, u32), // acc=e0; k times: acc = acc*M + e1
}

fn gen(rng: &mut Rng, depth: u32) -> Node {
    use Node::*;
    if depth >= MAX_DEPTH || rng.below(100) < 18 {
        return match rng.below(3) {
            0 => X,
            1 => Y,
            _ => C(rng.next()),
        };
    }
    macro_rules! a {
        () => {
            Box::new(gen(rng, depth + 1))
        };
    }
    let op = rng.below(14);
    match op {
        0 => Add(a!(), a!()),
        1 => Sub(a!(), a!()),
        2 => Mul(a!(), a!()),
        3 => And(a!(), a!()),
        4 => Or(a!(), a!()),
        5 => Xor(a!(), a!()),
        6 => Shl(a!(), a!()),
        7 => Shr(a!(), a!()),
        8 => Div(a!(), a!()),
        9 => Rem(a!(), a!()),
        10 | 11 => Sel(a!(), a!(), a!(), a!()),
        12 => {
            let k = 1 + rng.below(8);
            Loop(a!(), a!(), k)
        }
        _ => Sel(a!(), a!(), a!(), a!()),
    }
}

fn emit(n: &Node, out: &mut String) {
    use Node::*;
    match n {
        X => out.push('x'),
        Y => out.push('y'),
        C(v) => {
            let _ = write!(out, "{v}u32");
        }
        Add(a, b) => bin(out, a, "wrapping_add", b),
        Sub(a, b) => bin(out, a, "wrapping_sub", b),
        Mul(a, b) => bin(out, a, "wrapping_mul", b),
        And(a, b) => infix(out, a, "&", b),
        Or(a, b) => infix(out, a, "|", b),
        Xor(a, b) => infix(out, a, "^", b),
        Shl(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" << (");
            emit(b, out);
            out.push_str(" & 31))");
        }
        Shr(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" >> (");
            emit(b, out);
            out.push_str(" & 31))");
        }
        Div(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" / (");
            emit(b, out);
            out.push_str(" | 1))");
        }
        Rem(a, b) => {
            out.push('(');
            emit(a, out);
            out.push_str(" % (");
            emit(b, out);
            out.push_str(" | 1))");
        }
        Sel(a, b, t, f) => {
            out.push_str("(if ");
            emit(a, out);
            out.push_str(" < ");
            emit(b, out);
            out.push_str(" { ");
            emit(t, out);
            out.push_str(" } else { ");
            emit(f, out);
            out.push_str(" })");
        }
        Loop(e0, e1, k) => {
            out.push_str("({ let mut acc = ");
            emit(e0, out);
            let _ = write!(out, "; let mut i = 0u32; while i < {k}u32 {{ acc = acc.wrapping_mul(1664525).wrapping_add(");
            emit(e1, out);
            out.push_str("); i += 1; } acc })");
        }
    }
}

fn bin(out: &mut String, a: &Node, m: &str, b: &Node) {
    out.push('(');
    emit(a, out);
    let _ = write!(out, ".{m}(");
    emit(b, out);
    out.push_str("))");
}
fn infix(out: &mut String, a: &Node, op: &str, b: &Node) {
    out.push('(');
    emit(a, out);
    let _ = write!(out, " {op} ");
    emit(b, out);
    out.push(')');
}

fn interp(n: &Node, x: u32, y: u32) -> u32 {
    use Node::*;
    match n {
        X => x,
        Y => y,
        C(v) => *v,
        Add(a, b) => interp(a, x, y).wrapping_add(interp(b, x, y)),
        Sub(a, b) => interp(a, x, y).wrapping_sub(interp(b, x, y)),
        Mul(a, b) => interp(a, x, y).wrapping_mul(interp(b, x, y)),
        And(a, b) => interp(a, x, y) & interp(b, x, y),
        Or(a, b) => interp(a, x, y) | interp(b, x, y),
        Xor(a, b) => interp(a, x, y) ^ interp(b, x, y),
        Shl(a, b) => interp(a, x, y) << (interp(b, x, y) & 31),
        Shr(a, b) => interp(a, x, y) >> (interp(b, x, y) & 31),
        Div(a, b) => interp(a, x, y) / (interp(b, x, y) | 1),
        Rem(a, b) => interp(a, x, y) % (interp(b, x, y) | 1),
        Sel(a, b, t, f) => {
            if interp(a, x, y) < interp(b, x, y) {
                interp(t, x, y)
            } else {
                interp(f, x, y)
            }
        }
        Loop(e0, e1, k) => {
            let mut acc = interp(e0, x, y);
            let e1v = interp(e1, x, y);
            let mut i = 0u32;
            while i < *k {
                acc = acc.wrapping_mul(1664525).wrapping_add(e1v);
                i += 1;
            }
            acc
        }
    }
}

// NOTE: in Loop, e1 is loop-invariant by construction (no acc/i references),
// so hoisting it in the interpreter matches the emitted while-loop exactly.

fn write_batch(fns: &[Node], dir: &std::path::Path) {
    let mut src = String::from(
        "//! REGENERATED BY tools/diff-fuzz — do not edit by hand.\n#![no_std]\n#![allow(unused_parens, unused_variables, clippy::all)]\n\nuse spirv_std::glam::UVec3;\nuse spirv_std::spirv;\n\n",
    );
    let _ = write!(src, "pub const N_FNS: u32 = {};\n\n", fns.len());
    src.push_str("fn lcg(s: u32) -> u32 { s.wrapping_mul(1664525).wrapping_add(1013904223) }\n\n");
    for (i, f) in fns.iter().enumerate() {
        let _ = write!(src, "pub fn f_{i}(x: u32, y: u32) -> u32 {{ ");
        emit(f, &mut src);
        src.push_str(" }\n");
    }
    src.push_str("\n#[spirv(compute(threads(8, 8)))]\npub fn fuzz_cs(\n    #[spirv(global_invocation_id)] id: UVec3,\n    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &[u32; 4],\n    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] out: &mut [u32],\n) {\n    let ii = id.x;\n    let fi = id.y;\n    if ii >= params[0] || fi >= params[1] { return; }\n    let x = lcg(params[2] ^ ii);\n    let y = lcg(x);\n    let v = match fi {\n");
    for i in 0..fns.len() {
        let _ = write!(src, "        {i} => f_{i}(x, y),\n");
    }
    src.push_str("        _ => 0,\n    };\n    out[(fi * params[0] + ii) as usize] = v;\n}\n");
    std::fs::write(dir.join("src/lib.rs"), src).expect("write generated lib.rs");
}

fn main() {
    let args: Vec<u32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let batches = *args.first().unwrap_or(&1);
    let seed0 = *args.get(1).unwrap_or(&1);

    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fuzz_dir = root.join("fuzz-shaders");
    let spv_path = fuzz_dir.join("spv/fuzz_shaders.spv");
    let findings_dir = root.join("fuzz-findings");
    std::fs::create_dir_all(&findings_dir).ok();

    let gpu = pollster::block_on(init_gpu());
    let mut total_fns = 0u64;
    let mut total_cmp = 0u64;
    let mut findings = 0u32;
    let t0 = Instant::now();

    for batch in 0..batches {
        let seed = seed0.wrapping_add(batch);
        let mut rng = Rng(if seed == 0 { 0xDEAD_BEEF } else { lcg(seed) });
        let fns: Vec<Node> = (0..N_FNS).map(|_| gen(&mut rng, 0)).collect();
        write_batch(&fns, &fuzz_dir);

        let status = Command::new("cargo")
            .args([
                "gpu",
                "build",
                "--shader-crate",
                fuzz_dir.to_str().unwrap(),
                "--output-dir",
                fuzz_dir.join("spv").to_str().unwrap(),
                "--auto-install-rust-toolchain",
            ])
            .current_dir(&root)
            .output()
            .expect("spawn cargo gpu");
        if !status.status.success() {
            // a generated program that FAILS to compile is itself a finding
            let path = findings_dir.join(format!("compile-fail-seed{seed}.txt"));
            std::fs::write(
                &path,
                format!(
                    "seed {seed}: cargo gpu build failed\n\n{}",
                    String::from_utf8_lossy(&status.stderr)
                ),
            )
            .ok();
            println!("batch seed {seed}: COMPILE FAIL -> {}", path.display());
            findings += 1;
            continue;
        }

        let spv = std::fs::read(&spv_path).expect("read fuzz spv");
        let gpu_out = run_gpu(&gpu, &spv, seed, fns.len() as u32);

        for (fi, f) in fns.iter().enumerate() {
            for ii in 0..N_INPUTS {
                let x = lcg(seed ^ ii);
                let y = lcg(x);
                let cpu = interp(f, x, y);
                let got = gpu_out[fi * N_INPUTS as usize + ii as usize];
                total_cmp += 1;
                if cpu != got {
                    findings += 1;
                    let mut src = String::new();
                    emit(f, &mut src);
                    let path = findings_dir.join(format!("mismatch-seed{seed}-fn{fi}.rs"));
                    std::fs::write(
                        &path,
                        format!(
                            "// MISMATCH seed={seed} fn={fi} x={x} y={y}\n// cpu(interp)={cpu} gpu={got}\n// verify natively: this file is valid Rust\npub fn repro(x: u32, y: u32) -> u32 {{ {src} }}\nfn main() {{ assert_eq!(repro({x}, {y}), {cpu}u32, \"native rustc disagrees with interpreter\"); }}\n"
                        ),
                    )
                    .ok();
                    println!("batch seed {seed} fn {fi}: MISMATCH at x={x} y={y} (cpu {cpu} vs gpu {got}) -> {}", path.display());
                    break; // one repro per fn is enough
                }
            }
        }
        total_fns += fns.len() as u64;
        if batch % 10 == 9 || batches <= 5 {
            println!(
                "[{:>5.1}s] {} batches, {} fns, {} comparisons, {} findings",
                t0.elapsed().as_secs_f32(),
                batch + 1,
                total_fns,
                total_cmp,
                findings
            );
        }
    }
    println!(
        "DONE: {batches} batches / {total_fns} fns / {total_cmp} comparisons / {findings} findings in {:.1}s",
        t0.elapsed().as_secs_f32()
    );
    if findings > 0 {
        std::process::exit(2);
    }
}

struct GpuCtx {
    device: wgpu::Device,
    queue: wgpu::Queue,
    passthrough: bool,
}

async fn init_gpu() -> GpuCtx {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("no adapter");
    let passthrough = std::env::var("DIFF_FUZZ_NAGA").is_err()
        && adapter
            .features()
            .contains(wgpu::Features::PASSTHROUGH_SHADERS);
    println!(
        "shader path: {}",
        if passthrough { "passthrough" } else { "naga frontend" }
    );
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: if passthrough {
                wgpu::Features::PASSTHROUGH_SHADERS
            } else {
                wgpu::Features::empty()
            },
            ..Default::default()
        })
        .await
        .expect("device");
    GpuCtx {
        device,
        queue,
        passthrough,
    }
}

fn run_gpu(ctx: &GpuCtx, spv: &[u8], seed: u32, n_fns: u32) -> Vec<u32> {
    let module = if ctx.passthrough {
        unsafe {
            ctx.device
                .create_shader_module_passthrough(wgpu::ShaderModuleDescriptorPassthrough {
                    label: None,
                    spirv: Some(wgpu::util::make_spirv_raw(spv)),
                    ..Default::default()
                })
        }
    } else {
        ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::util::make_spirv(spv),
        })
    };
    let bgl = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
    let layout = ctx
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
    let pipeline = ctx
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&layout),
            module: &module,
            entry_point: Some("fuzz_cs"),
            compilation_options: Default::default(),
            cache: None,
        });
    let mk = |size: u64, usage: wgpu::BufferUsages| {
        ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        })
    };
    let params = mk(16, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
    ctx.queue
        .write_buffer(&params, 0, bytemuck::cast_slice(&[N_INPUTS, n_fns, seed, 0]));
    let out_size = (N_INPUTS * n_fns * 4) as u64;
    let out = mk(out_size, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);
    let staging = mk(out_size, wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST);
    let bind = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: params.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: out.as_entire_binding(),
            },
        ],
    });
    let mut enc = ctx.device.create_command_encoder(&Default::default());
    {
        let mut pass = enc.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(N_INPUTS.div_ceil(8), n_fns.div_ceil(8), 1);
    }
    enc.copy_buffer_to_buffer(&out, 0, &staging, 0, out_size);
    ctx.queue.submit([enc.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
    ctx.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("poll");
    let v = bytemuck::cast_slice(&slice.get_mapped_range()).to_vec();
    staging.unmap();
    v
}
