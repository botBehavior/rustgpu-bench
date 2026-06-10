//! Independent, hand-audited reproduction of the two confirmed rust-gpu
//! miscompiles. Shares NO code with the fuzzer's batch indexing — inputs are
//! fed DIRECTLY from a buffer (no LCG derivation), so we can probe the exact
//! edge cases the unified hypothesis predicts and hand-check every value.
//!
//! Unified hypothesis: rust-gpu folds `a < b` to `true` whenever it can prove
//! `a <= b`, dropping the `a == b` case. Both findings are strict-< where the
//! false branch is reachable ONLY at equality:
//!   423: `0 < x.wrapping_mul(EVEN)`  false  iff  x*EVEN == 0
//!   303: `y < (C | y)`               false  iff  (y & C) == C
//!
//! The host oracle functions below are the SAME source as the shader bodies,
//! compiled by native rustc — so this is a true differential test: one source,
//! two compilers (rustc vs rust-gpu→SPIR-V→GPU).
//!
//! Usage: repro 423 | 303    (default: both)

use std::process::Command;

// ---- oracles: identical source to the shader f_0 bodies, native rustc ----
fn f0_423(x: u32, y: u32) -> u32 {
    if 0u32 < x.wrapping_mul(4137330452u32) {
        y
    } else {
        y % (3838954833u32 | 1)
    }
}
fn f0_303(x: u32, y: u32) -> u32 {
    let mut acc = x;
    let mut i = 0u32;
    while i < 4u32 {
        acc = acc.wrapping_mul(1664525).wrapping_add(y.wrapping_add(
            if y < (745348641u32 | y) {
                x
            } else if x < 3850752092u32 {
                x
            } else {
                y
            },
        ));
        i += 1;
    }
    acc
}

// the exact f_0 body text emitted into the shader crate (must match oracle)
const BODY_423: &str =
    "(if 0u32 < (x.wrapping_mul(4137330452u32)) { y } else { (y % (3838954833u32 | 1)) })";
const BODY_303: &str = "({ let mut acc = x; let mut i = 0u32; while i < 4u32 { acc = acc.wrapping_mul(1664525).wrapping_add((y.wrapping_add((if y < (745348641u32 | y) { x } else { (if x < 3850752092u32 { x } else { y }) })))); i += 1; } acc })";

fn shader_src(body: &str) -> String {
    format!(
        "//! hand-authored repro shader — reads (x,y) pairs directly from a buffer\n#![no_std]\n#![allow(unused_parens, unused_variables, clippy::all)]\nuse spirv_std::glam::UVec3;\nuse spirv_std::spirv;\n\npub fn f_0(x: u32, y: u32) -> u32 {{ {body} }}\n\n#[spirv(compute(threads(64)))]\npub fn repro_cs(\n    #[spirv(global_invocation_id)] id: UVec3,\n    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] inp: &[u32],\n    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] out: &mut [u32],\n) {{\n    let i = id.x as usize;\n    let n = inp[0] as usize;\n    if i >= n {{ return; }}\n    let x = inp[1 + i * 2];\n    let y = inp[2 + i * 2];\n    out[i] = f_0(x, y);\n}}\n"
    )
}

fn main() {
    let which: Vec<String> = std::env::args().skip(1).collect();
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let gpu = pollster::block_on(init_gpu());
    if which.first().map(|s| s.as_str()) == Some("probe") {
        probe(&root, &gpu);
        return;
    }
    if which.first().map(|s| s.as_str()) == Some("control") {
        wgsl_control(&gpu);
        return;
    }
    let cases: Vec<&str> = if which.is_empty() {
        vec!["423", "303"]
    } else {
        which.iter().map(|s| s.as_str()).collect()
    };
    let mut all_real = true;
    for c in cases {
        let (body, oracle): (&str, fn(u32, u32) -> u32) = match c {
            "423" => (BODY_423, f0_423 as fn(u32, u32) -> u32),
            "303" => (BODY_303, f0_303 as fn(u32, u32) -> u32),
            _ => {
                println!("unknown case {c}");
                continue;
            }
        };
        let real = run_case(c, body, oracle, &root, &gpu);
        all_real &= real;
    }
    if !all_real {
        std::process::exit(1);
    }
}

fn run_case(
    name: &str,
    body: &str,
    oracle: fn(u32, u32) -> u32,
    root: &std::path::Path,
    gpu: &GpuCtx,
) -> bool {
    println!("\n================ repro {name} ================");
    let fuzz_dir = root.join("fuzz-shaders");
    std::fs::write(fuzz_dir.join("src/lib.rs"), shader_src(body)).expect("write shader");
    let ok = Command::new("cargo")
        .args([
            "gpu", "build", "--shader-crate", fuzz_dir.to_str().unwrap(),
            "--output-dir", fuzz_dir.join("spv").to_str().unwrap(),
            "--auto-install-rust-toolchain",
        ])
        .current_dir(root)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    if !ok {
        println!("BUILD FAILED");
        return false;
    }
    let spv = std::fs::read(fuzz_dir.join("spv/fuzz_shaders.spv")).expect("spv");

    // curated edge inputs predicted by the hypothesis, plus a broad sweep
    let mut inputs: Vec<(u32, u32)> = Vec::new();
    if name == "423" {
        // x*C == 0  <=>  x is a multiple of 2^30 (C = 4*odd)
        for k in 0u32..4 {
            let x = k << 30;
            for y in [0u32, 3838954833, 3838954834, 4_000_000_000, u32::MAX] {
                inputs.push((x, y));
            }
        }
        // a non-edge control: x with x*C != 0
        inputs.push((1, 4_000_000_000));
    } else {
        // (y & C) == C  <=>  y contains all bits of C = 745348641
        let cc = 745348641u32;
        for y in [cc, u32::MAX, cc | 0x0F0F0F0F, cc | 0xFF00FF00] {
            for x in [0u32, 1, 3_850_752_092, 4_000_000_000, u32::MAX] {
                inputs.push((x, y));
            }
        }
        // control: y NOT containing all of C
        inputs.push((123, 0));
    }
    // broad pseudo-random sweep (independent LCG, host-side)
    let mut s = 0x1234_5678u32;
    for _ in 0..20000 {
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        let x = s;
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        inputs.push((x, s));
    }

    let pass_out = run_gpu(gpu, &spv, &inputs, true);
    let naga_out = run_gpu(gpu, &spv, &inputs, false);

    let mut mismatches = 0u32;
    let mut first_edge: Option<(u32, u32, u32, u32, u32)> = None;
    for (i, &(x, y)) in inputs.iter().enumerate() {
        let cpu = oracle(x, y);
        let p = pass_out[i];
        let n = naga_out[i];
        if cpu != p || cpu != n {
            mismatches += 1;
            if first_edge.is_none() {
                first_edge = Some((x, y, cpu, p, n));
            }
            if mismatches <= 8 {
                println!("  MISMATCH x={x} y={y}: rustc={cpu}  gpu-passthrough={p}  gpu-naga={n}");
            }
        }
    }
    println!(
        "{} inputs: {} mismatches (rustc vs GPU). passthrough and naga {}.",
        inputs.len(),
        mismatches,
        if pass_out == naga_out { "agree with each other" } else { "DIFFER" }
    );
    if let Some((x, y, cpu, p, n)) = first_edge {
        // hand-audit print: show the predicted-equality structure
        if name == "423" {
            println!(
                "  hand-audit: x={x} -> x*C = {} (==0? {}); rustc takes else-branch = {cpu}, GPU took then-branch (y={y}) -> pass {p}, naga {n}",
                x.wrapping_mul(4137330452),
                x.wrapping_mul(4137330452) == 0
            );
        } else {
            let cc = 745348641u32;
            println!(
                "  hand-audit: y={y}, C|y={} (==y? {}); the inner `y < (C|y)` is FALSE here, rustc={cpu} vs GPU pass {p}/naga {n}",
                cc | y,
                (cc | y) == y
            );
        }
    }
    if mismatches > 0 {
        println!("=> CONFIRMED: rust-gpu miscompiles {name} (GPU disagrees with native rustc on the same source).");
        true
    } else {
        println!("=> NOT reproduced on this toolchain (no mismatch). Re-examine before claiming a bug.");
        false
    }
}

/// Localization probe: test candidate functions of decreasing complexity to
/// find the MINIMAL construct that triggers the miscompile, and characterize it.
fn probe(root: &std::path::Path, gpu: &GpuCtx) {
    // (label, shader body, native oracle). Ordered simplest -> closest to 303.
    let candidates: Vec<(&str, &str, fn(u32, u32) -> u32)> = vec![
        (
            "A  nested, conds BOTH on x (control, was OK)",
            "(if x < 1u32 { x } else { (if x < 2u32 { x } else { y }) })",
            |x, y| if x < 1 { x } else if x < 2 { x } else { y },
        ),
        (
            "J  nested, outer cond on Y, inner on X, SIMPLE consts (if y<5 {x} else if x<5 {x} else {y})",
            "(if y < 5u32 { x } else { (if x < 5u32 { x } else { y }) })",
            |x, y| if y < 5 { x } else if x < 5 { x } else { y },
        ),
        (
            "G  nested, outer = OR-cond on y, inner SIMPLE x<2",
            "(if y < (745348641u32 | y) { x } else { (if x < 2u32 { x } else { y }) })",
            |x, y| if y < (745348641u32 | y) { x } else if x < 2 { x } else { y },
        ),
        (
            "H  nested, outer SIMPLE x<2, inner = large const x<D",
            "(if x < 2u32 { x } else { (if x < 3850752092u32 { x } else { y }) })",
            |x, y| if x < 2 { x } else if x < 3850752092u32 { x } else { y },
        ),
        (
            "K  C-structure with GENERIC constants (12345/99999)",
            "(if y < (12345u32 | y) { x } else { (if x < 99999u32 { x } else { y }) })",
            |x, y| if y < (12345u32 | y) { x } else if x < 99999u32 { x } else { y },
        ),
    ];

    // inputs: exercise both branches of x-conditions + the OR edge + random
    let mut inputs: Vec<(u32, u32)> = Vec::new();
    for x in [0u32, 1, 2, 3, 5, 10, 100] {
        for y in [0u32, 1, 7, 745348641, u32::MAX] {
            inputs.push((x, y));
        }
    }
    let mut s = 0xABCD_1234u32;
    for _ in 0..3000 {
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        let x = s;
        s = s.wrapping_mul(1664525).wrapping_add(1013904223);
        inputs.push((x, s));
    }

    println!("=== localization probe: {} inputs each ===", inputs.len());
    for (label, body, oracle) in &candidates {
        let fuzz_dir = root.join("fuzz-shaders");
        std::fs::write(fuzz_dir.join("src/lib.rs"), shader_src(body)).unwrap();
        let ok = Command::new("cargo")
            .args([
                "gpu", "build", "--shader-crate", fuzz_dir.to_str().unwrap(),
                "--output-dir", fuzz_dir.join("spv").to_str().unwrap(),
                "--auto-install-rust-toolchain",
            ])
            .current_dir(root)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if !ok {
            println!("[{label}] BUILD FAILED");
            continue;
        }
        let spv = std::fs::read(fuzz_dir.join("spv/fuzz_shaders.spv")).unwrap();
        let pass = run_gpu(gpu, &spv, &inputs, true);
        let naga = run_gpu(gpu, &spv, &inputs, false);
        let mut mm = 0u32;
        let mut sample = String::new();
        for (i, &(x, y)) in inputs.iter().enumerate() {
            let c = oracle(x, y);
            if c != pass[i] || c != naga[i] {
                mm += 1;
                if sample.is_empty() {
                    sample = format!("x={x} y={y}: rustc={c} pass={} naga={}", pass[i], naga[i]);
                }
            }
        }
        let verdict = if mm == 0 { "OK" } else { "MISCOMPILES" };
        println!("[{verdict}] {label}\n        {mm}/{} mismatch  {}", inputs.len(), sample);
    }
}

/// Hand-written WGSL with the SAME nested-select logic, run through the SAME
/// harness/driver. If THIS is correct while rust-gpu's SPIR-V is wrong, the
/// harness and driver are exonerated and the fault is rust-gpu's codegen.
fn wgsl_control(ctx: &GpuCtx) {
    let inputs: Vec<(u32, u32)> = vec![(0, 1), (2, 3), (100, 7)];
    let body = |stmt: &str| {
        format!(
            "@group(0) @binding(0) var<storage> inp: array<u32>;\n@group(0) @binding(1) var<storage, read_write> outp: array<u32>;\n@compute @workgroup_size(64)\nfn repro_cs(@builtin(global_invocation_id) gid: vec3<u32>) {{\n    let i = gid.x;\n    let n = inp[0];\n    if (i >= n) {{ return; }}\n    let x = inp[1u + i * 2u];\n    let y = inp[2u + i * 2u];\n    {stmt}\n}}\n"
        )
    };
    let diags: Vec<(&str, &str)> = vec![
        ("read x", "outp[i] = x;"),
        ("read y", "outp[i] = y;"),
        ("outer cmp y<(C|y) [want 8888 when true]", "outp[i] = select(7777u, 8888u, y < (745348641u | y));"),
        ("inner cmp x<D [want 8888 when true]", "outp[i] = select(7777u, 8888u, x < 3850752092u);"),
        ("nested select", "var r: u32; if (y < (745348641u | y)) { r = x; } else { if (x < 3850752092u) { r = x; } else { r = y; } } outp[i] = r;"),
    ];
    println!("=== WGSL harness diagnostics on PRIMARY adapter ===");
    for (label, stmt) in &diags {
        let module = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(body(stmt).into()),
        });
        let out = dispatch(ctx, &module, &inputs);
        println!("  [{label}] -> {:?}", &out[..inputs.len()]);
    }

    // Decisive: run the SAME nested-select WGSL on a software/fallback adapter
    // (non-NVIDIA). If it is CORRECT there and wrong on NVIDIA, the fault is the
    // NVIDIA driver, not rust-gpu and not naga's WGSL->SPIR-V.
    println!("\n=== same nested-select on a SOFTWARE/FALLBACK adapter ===");
    match pollster::block_on(init_fallback()) {
        Some(soft) => {
            println!("  fallback adapter: {}", soft.name);
            let module = soft.ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("nested"),
                source: wgpu::ShaderSource::Wgsl(
                    body("var r: u32; if (y < (745348641u | y)) { r = x; } else { if (x < 3850752092u) { r = x; } else { r = y; } } outp[i] = r;").into(),
                ),
            });
            let out = dispatch(&soft.ctx, &module, &inputs);
            println!("  [nested select on {}] -> {:?}", soft.name, &out[..inputs.len()]);
            println!("  (correct = [0, 2, 100])");
        }
        None => println!("  no fallback adapter available on this machine"),
    }
}

struct Named {
    ctx: GpuCtx,
    name: String,
}

async fn init_fallback() -> Option<Named> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            force_fallback_adapter: true,
            ..Default::default()
        })
        .await
        .ok()?;
    let name = format!("{} ({:?})", adapter.get_info().name, adapter.get_info().backend);
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            ..Default::default()
        })
        .await
        .ok()?;
    Some(Named { ctx: GpuCtx { device, queue }, name })
}

/// dispatch a module over (x,y) inputs using the [n, x0,y0,...] layout
fn dispatch(ctx: &GpuCtx, module: &wgpu::ShaderModule, inputs: &[(u32, u32)]) -> Vec<u32> {
    use wgpu::util::DeviceExt;
    let n = inputs.len() as u32;
    let mut inp = vec![n];
    for &(x, y) in inputs {
        inp.push(x);
        inp.push(y);
    }
    let bgl = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[bgl_entry(0, true), bgl_entry(1, false)],
    });
    let layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(&bgl)],
        immediate_size: 0,
    });
    let pipeline = ctx.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&layout),
        module,
        entry_point: Some("repro_cs"),
        compilation_options: Default::default(),
        cache: None,
    });
    let in_buf = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&inp),
        usage: wgpu::BufferUsages::STORAGE,
    });
    let out_size = (inputs.len() * 4) as u64;
    let out = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: out_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: out_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: in_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: out.as_entire_binding() },
        ],
    });
    let mut enc = ctx.device.create_command_encoder(&Default::default());
    {
        let mut pass = enc.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    enc.copy_buffer_to_buffer(&out, 0, &staging, 0, out_size);
    ctx.queue.submit([enc.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
    ctx.device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let v = bytemuck::cast_slice(&slice.get_mapped_range()).to_vec();
    staging.unmap();
    v
}

// ---------------- minimal independent wgpu dispatch ----------------
struct GpuCtx {
    device: wgpu::Device,
    queue: wgpu::Queue,
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
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::PASSTHROUGH_SHADERS,
            ..Default::default()
        })
        .await
        .expect("device");
    GpuCtx { device, queue }
}

fn run_gpu(ctx: &GpuCtx, spv: &[u8], inputs: &[(u32, u32)], passthrough: bool) -> Vec<u32> {
    let module = if passthrough {
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
    let n = inputs.len() as u32;
    let mut inp: Vec<u32> = Vec::with_capacity(1 + inputs.len() * 2);
    inp.push(n);
    for &(x, y) in inputs {
        inp.push(x);
        inp.push(y);
    }
    let bgl = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                bgl_entry(0, true),
                bgl_entry(1, false),
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
            entry_point: Some("repro_cs"),
            compilation_options: Default::default(),
            cache: None,
        });
    let mk = |bytes: &[u8], usage: wgpu::BufferUsages| {
        use wgpu::util::DeviceExt;
        ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytes,
            usage,
        })
    };
    let in_buf = mk(bytemuck::cast_slice(&inp), wgpu::BufferUsages::STORAGE);
    let out_size = (inputs.len() * 4) as u64;
    let out = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: out_size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: out_size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let bind = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: in_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: out.as_entire_binding() },
        ],
    });
    let mut enc = ctx.device.create_command_encoder(&Default::default());
    {
        let mut pass = enc.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(n.div_ceil(64), 1, 1);
    }
    enc.copy_buffer_to_buffer(&out, 0, &staging, 0, out_size);
    ctx.queue.submit([enc.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
    ctx.device.poll(wgpu::PollType::wait_indefinitely()).expect("poll");
    let v = bytemuck::cast_slice(&slice.get_mapped_range()).to_vec();
    staging.unmap();
    v
}

fn bgl_entry(binding: u32, read_only: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}
