//! Shader benchmark: rust-gpu-emitted SPIR-V vs hand-written WGSL, same
//! algorithms, same workgroup sizes, same buffers.
//!
//! Arms (the naga-tax decomposition — Experiment 1 in EXPERIMENTS.md):
//!   rustgpu-spv         rust-gpu SPIR-V via PASSTHROUGH_SHADERS (no naga, baseline)
//!   rustgpu-naga        rust-gpu SPIR-V through naga; wgpu re-injects runtime checks
//!   rustgpu-naga-unchk  same, but ShaderRuntimeChecks::unchecked() — no injected checks
//!   hand-wgsl           hand-written WGSL twins through naga's WGSL frontend (checked)
//!   hand-wgsl-unchk     hand-WGSL, checks off — the floor naga can reach
//!
//! Decomposition: (naga vs naga-unchk) isolates the bounds-check tax; (naga-unchk
//! vs spv) isolates the transpilation/structure tax; (naga-unchk vs hand-wgsl-unchk)
//! isolates rust-gpu's emitted-code quality from hand-written WGSL, checks held equal.
//!
//! Timing: GPU-side timestamp queries around the compute pass only. Uploads,
//! pipeline creation, and readback are excluded (pipeline creation reported
//! separately). N timed runs after warmup; medians reported.
//! Every arm is correctness-gated against the CPU implementation first —
//! a wrong kernel's timing is noise, not data.

use std::io::Write as _;
use std::time::Instant;

use gpu_shared::RenderParams;

const WARMUP: usize = 3;
const RUNS: usize = 30;

const COLLATZ_LEN: u32 = 1 << 20;
const MATMUL_N: u32 = 1024;
const RENDER: RenderParams = RenderParams {
    width: 800,
    height: 450,
    samples: 32,
    seed: 7,
};

struct Ctx {
    device: wgpu::Device,
    queue: wgpu::Queue,
    ts_period_ns: f32,
    adapter_name: String,
}

fn main() {
    let ctx = pollster::block_on(init());
    println!("adapter: {}", ctx.adapter_name);

    let spv = std::fs::read(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../shaders/spv/gpu_shaders.spv"),
    )
    .expect("run `cargo gpu build` first (gpu/shaders -> gpu/shaders/spv)");

    let arms: Vec<(&str, wgpu::ShaderModule, f64)> = vec![
        {
            let t = Instant::now();
            let m = unsafe {
                ctx.device.create_shader_module_passthrough(
                    wgpu::ShaderModuleDescriptorPassthrough {
                        label: Some("rustgpu-spv"),
                        spirv: Some(wgpu::util::make_spirv_raw(&spv)),
                        ..Default::default()
                    },
                )
            };
            ("rustgpu-spv", m, t.elapsed().as_secs_f64() * 1e3)
        },
        {
            let t = Instant::now();
            let m = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("rustgpu-naga"),
                source: wgpu::util::make_spirv(&spv),
            });
            ("rustgpu-naga", m, t.elapsed().as_secs_f64() * 1e3)
        },
        {
            // same naga path, but tell wgpu to inject NO runtime checks — the
            // control that splits the check-tax from the transpilation-tax.
            // Safe here: every kernel is correctness-gated and has no real OOB.
            let t = Instant::now();
            let m = unsafe {
                ctx.device.create_shader_module_trusted(
                    wgpu::ShaderModuleDescriptor {
                        label: Some("rustgpu-naga-unchk"),
                        source: wgpu::util::make_spirv(&spv),
                    },
                    wgpu::ShaderRuntimeChecks::unchecked(),
                )
            };
            ("rustgpu-naga-unchk", m, t.elapsed().as_secs_f64() * 1e3)
        },
    ];

    let wgsl_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../shaders-wgsl");

    // --saturate: sweep problem size upward and report THROUGHPUT, not latency
    // (Firestar99's #614 critique — single small dispatches barely load the GPU).
    if std::env::args().any(|a| a == "--saturate") {
        saturate(&ctx, &arms, &wgsl_dir);
        return;
    }

    let mut results = Vec::new();

    for workload in ["collatz", "matmul", "matmul_unchecked", "render", "render_v2"] {
        // hand-WGSL module is per-workload; experiment workloads (B3) have no
        // hand twin — they compare rust-gpu variants against each other.
        // Built both checked and unchecked so the floor naga can reach is visible.
        let hand_src = std::fs::read_to_string(wgsl_dir.join(format!("{workload}.wgsl"))).ok();
        let hand: Vec<(&str, wgpu::ShaderModule, f64)> = hand_src
            .iter()
            .flat_map(|src| {
                let t = Instant::now();
                let checked = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("hand-wgsl"),
                    source: wgpu::ShaderSource::Wgsl(src.clone().into()),
                });
                let c_ms = t.elapsed().as_secs_f64() * 1e3;
                let t = Instant::now();
                let unchk = unsafe {
                    ctx.device.create_shader_module_trusted(
                        wgpu::ShaderModuleDescriptor {
                            label: Some("hand-wgsl-unchk"),
                            source: wgpu::ShaderSource::Wgsl(src.clone().into()),
                        },
                        wgpu::ShaderRuntimeChecks::unchecked(),
                    )
                };
                let u_ms = t.elapsed().as_secs_f64() * 1e3;
                [("hand-wgsl", checked, c_ms), ("hand-wgsl-unchk", unchk, u_ms)]
            })
            .collect();

        for (arm_name, module, module_ms) in arms
            .iter()
            .map(|(n, m, c)| (*n, m, *c))
            .chain(hand.iter().map(|(n, m, c)| (*n, m, *c)))
        {
            let r = run_workload(&ctx, workload, module);
            match r {
                Ok(stats) => {
                    println!(
                        "{workload:8} {arm_name:13} median {:8.3} ms  (p25 {:7.3}, p75 {:7.3})  module {:7.1} ms  {}",
                        stats.median, stats.p25, stats.p75, module_ms, stats.note
                    );
                    results.push(format!(
                        "{{\"workload\":\"{workload}\",\"arm\":\"{arm_name}\",\"median_ms\":{:.4},\"p25_ms\":{:.4},\"p75_ms\":{:.4},\"module_create_ms\":{:.2},\"runs\":{RUNS},\"note\":\"{}\"}}",
                        stats.median, stats.p25, stats.p75, module_ms, stats.note
                    ));
                }
                Err(e) => {
                    println!("{workload:8} {arm_name:13} FAILED: {e}");
                    results.push(format!(
                        "{{\"workload\":\"{workload}\",\"arm\":\"{arm_name}\",\"error\":\"{e}\"}}"
                    ));
                }
            }
        }
    }

    let json = format!(
        "{{\n  \"adapter\": \"{}\",\n  \"versions\": {{\"rust-gpu\": \"0.10.0-alpha.1\", \"toolchain\": \"nightly-2026-04-11\", \"wgpu\": \"29.0.3\", \"naga\": \"29.0.3\"}},\n  \"config\": {{\"warmup\": {WARMUP}, \"runs\": {RUNS}, \"collatz_len\": {COLLATZ_LEN}, \"matmul_n\": {MATMUL_N}, \"render\": \"{}x{}@{}spp\"}},\n  \"results\": [\n    {}\n  ]\n}}\n",
        ctx.adapter_name,
        RENDER.width,
        RENDER.height,
        RENDER.samples,
        results.join(",\n    ")
    );
    let out = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../bench-results.json");
    std::fs::File::create(&out)
        .unwrap()
        .write_all(json.as_bytes())
        .unwrap();
    println!("wrote {}", out.display());
}

/// Throughput sweep: grow the problem until the GPU saturates, report work/time
/// per arm at each size. The point is to show the arm RATIOS hold (or shift) at
/// full occupancy — answering "your latencies barely load the GPU".
fn saturate(ctx: &Ctx, arms: &[(&str, wgpu::ShaderModule, f64)], wgsl_dir: &std::path::Path) {
    // build hand-WGSL modules (checked + unchecked) once; reused across sizes
    let mut hand: std::collections::HashMap<&str, (wgpu::ShaderModule, wgpu::ShaderModule)> =
        std::collections::HashMap::new();
    for w in ["render", "matmul"] {
        if let Ok(src) = std::fs::read_to_string(wgsl_dir.join(format!("{w}.wgsl"))) {
            let checked = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("hand-wgsl"),
                source: wgpu::ShaderSource::Wgsl(src.clone().into()),
            });
            let unchk = unsafe {
                ctx.device.create_shader_module_trusted(
                    wgpu::ShaderModuleDescriptor {
                        label: Some("hand-wgsl-unchk"),
                        source: wgpu::ShaderSource::Wgsl(src.into()),
                    },
                    wgpu::ShaderRuntimeChecks::unchecked(),
                )
            };
            hand.insert(w, (checked, unchk));
        }
    }
    let m = |n: &str| -> &wgpu::ShaderModule { &arms.iter().find(|a| a.0 == n).unwrap().1 };
    let spv = m("rustgpu-spv");
    let naga = m("rustgpu-naga");
    let nunchk = m("rustgpu-naga-unchk");

    // --- render (the tracer): samples sweep at 1280x720; work = ray-samples ---
    println!("\n=== render saturation @ 1280x720 — throughput (Mraysamples/s, higher better) ===");
    let (h_chk, h_unchk) = &hand["render"];
    let render_arms: [(&str, &wgpu::ShaderModule); 5] = [
        ("rustgpu-spv", spv),
        ("rustgpu-naga", naga),
        ("rustgpu-naga-unchk", nunchk),
        ("hand-wgsl", h_chk),
        ("hand-wgsl-unchk", h_unchk),
    ];
    for samples in [32u32, 128, 512] {
        let r = RenderParams { width: 1280, height: 720, samples, seed: 7 };
        let rays = (r.width as f64) * (r.height as f64) * (r.samples as f64);
        for (name, module) in render_arms {
            let spec = render_spec(ctx, "render_cs", r);
            match time_pipeline(ctx, &spec, module) {
                Ok(s) => println!(
                    "  spp {samples:4}  {name:18} {:8.3} ms  {:9.1} Mray/s",
                    s.median,
                    rays / (s.median / 1e3) / 1e6
                ),
                Err(e) => println!("  spp {samples:4}  {name:18} FAILED: {e}"),
            }
        }
    }

    // --- matmul: N sweep; work = 2 N^3 FLOPs; isolates the bounds-check tax ---
    println!("\n=== matmul saturation — throughput (GFLOP/s, higher better) ===");
    let (mh_chk, mh_unchk) = &hand["matmul"];
    let matmul_arms: [(&str, &wgpu::ShaderModule, &str); 4] = [
        ("rustgpu-checked", spv, "matmul_cs"),
        ("rustgpu-unchecked", spv, "matmul_unchecked_cs"),
        ("hand-wgsl", mh_chk, "matmul_cs"),
        ("hand-wgsl-unchk", mh_unchk, "matmul_cs"),
    ];
    for n in [512u32, 1024, 2048] {
        let flops = 2.0 * (n as f64).powi(3);
        for (name, module, entry) in matmul_arms {
            let spec = matmul_spec(ctx, entry, n);
            match time_pipeline(ctx, &spec, module) {
                Ok(s) => println!(
                    "  N {n:5}  {name:18} {:8.3} ms  {:8.1} GFLOP/s",
                    s.median,
                    flops / (s.median / 1e3) / 1e9
                ),
                Err(e) => println!("  N {n:5}  {name:18} FAILED: {e}"),
            }
        }
    }
}

async fn init() -> Ctx {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("no adapter");
    let info = adapter.get_info();
    let need = wgpu::Features::TIMESTAMP_QUERY | wgpu::Features::PASSTHROUGH_SHADERS;
    assert!(
        adapter.features().contains(need),
        "adapter lacks TIMESTAMP_QUERY or PASSTHROUGH_SHADERS"
    );
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("bench"),
            required_features: need,
            ..Default::default()
        })
        .await
        .expect("device");
    let ts_period_ns = queue.get_timestamp_period();
    Ctx {
        device,
        queue,
        ts_period_ns,
        adapter_name: format!("{} ({:?})", info.name, info.backend),
    }
}

struct Stats {
    median: f64,
    p25: f64,
    p75: f64,
    note: String,
}

/// Carries the size knobs verify needs, so a swept spec self-describes.
#[derive(Clone, Copy)]
enum VerifyKind {
    Collatz(u32),
    Matmul(u32),
    Render(RenderParams),
}

struct Bufs {
    bufs: Vec<wgpu::Buffer>,
    /// per-binding shader writability — must match the shader exactly for
    /// naga-validated modules
    writable: Vec<bool>,
    /// (binding index, fresh contents) re-uploaded before every run
    reupload: Vec<(usize, Vec<u8>)>,
    read_binding: usize,
    workgroups: (u32, u32, u32),
    entry: &'static str,
    verify: VerifyKind,
}

fn base_spec(ctx: &Ctx, workload: &str) -> Bufs {
    match workload {
        "collatz" => collatz_spec(ctx, COLLATZ_LEN),
        "matmul" => matmul_spec(ctx, "matmul_cs", MATMUL_N),
        "matmul_unchecked" => matmul_spec(ctx, "matmul_unchecked_cs", MATMUL_N),
        "render" => render_spec(ctx, "render_cs", RENDER),
        "render_v2" => render_spec(ctx, "render_v2_cs", RENDER),
        _ => unreachable!(),
    }
}

fn run_workload(ctx: &Ctx, workload: &str, module: &wgpu::ShaderModule) -> Result<Stats, String> {
    let spec = base_spec(ctx, workload);
    time_pipeline(ctx, &spec, module)
}

fn time_pipeline(ctx: &Ctx, spec: &Bufs, module: &wgpu::ShaderModule) -> Result<Stats, String> {
    let layout_entries: Vec<wgpu::BindGroupLayoutEntry> = spec
        .bufs
        .iter()
        .enumerate()
        .map(|(i, _)| wgpu::BindGroupLayoutEntry {
            binding: i as u32,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage {
                    read_only: !spec.writable[i],
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        })
        .collect();
    let bgl = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &layout_entries,
        });
    let entries: Vec<wgpu::BindGroupEntry> = spec
        .bufs
        .iter()
        .enumerate()
        .map(|(i, b)| wgpu::BindGroupEntry {
            binding: i as u32,
            resource: b.as_entire_binding(),
        })
        .collect();
    let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bgl,
        entries: &entries,
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
            module,
            entry_point: Some(spec.entry),
            compilation_options: Default::default(),
            cache: None,
        });

    let query_set = ctx.device.create_query_set(&wgpu::QuerySetDescriptor {
        label: None,
        ty: wgpu::QueryType::Timestamp,
        count: 2,
    });
    let query_buf = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 16,
        usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let query_read = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 16,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut times = Vec::with_capacity(RUNS);
    for iter in 0..WARMUP + RUNS {
        for (binding, data) in &spec.reupload {
            ctx.queue.write_buffer(&spec.bufs[*binding], 0, data);
        }
        let mut encoder = ctx.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: Some(wgpu::ComputePassTimestampWrites {
                    query_set: &query_set,
                    beginning_of_pass_write_index: Some(0),
                    end_of_pass_write_index: Some(1),
                }),
            });
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(spec.workgroups.0, spec.workgroups.1, spec.workgroups.2);
        }
        encoder.resolve_query_set(&query_set, 0..2, &query_buf, 0);
        encoder.copy_buffer_to_buffer(&query_buf, 0, &query_read, 0, 16);
        ctx.queue.submit([encoder.finish()]);

        let slice = query_read.slice(..);
        slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
        ctx.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| format!("poll: {e}"))?;
        let raw = slice.get_mapped_range().to_vec();
        query_read.unmap();
        let ts: &[u64] = bytemuck::cast_slice(&raw);
        let ms = (ts[1].wrapping_sub(ts[0])) as f64 * ctx.ts_period_ns as f64 / 1.0e6;
        if iter >= WARMUP {
            times.push(ms);
        }
    }

    // independent cross-check: 10 passes in one submission, wall-clock,
    // amortized — catches timestamp-query lies (cf. the cross-op
    // contamination incident in the wasm suite)
    // destructive workloads (collatz) get a pristine-input copy before each
    // pass, inside the same encoder — otherwise pass N runs on pass N-1's
    // output and both the timing and the final verify are garbage
    let pristine: Vec<(usize, wgpu::Buffer)> = spec
        .reupload
        .iter()
        .map(|(binding, data)| (*binding, storage_buffer(ctx, data)))
        .collect();
    let t = Instant::now();
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
    for _ in 0..10 {
        for (binding, src) in &pristine {
            encoder.copy_buffer_to_buffer(src, 0, &spec.bufs[*binding], 0, src.size());
        }
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(spec.workgroups.0, spec.workgroups.1, spec.workgroups.2);
    }
    ctx.queue.submit([encoder.finish()]);
    ctx.device
        .poll(wgpu::PollType::wait_indefinitely())
        .map_err(|e| format!("poll: {e}"))?;
    let xcheck = t.elapsed().as_secs_f64() * 1e3 / 10.0;

    // correctness gate (after timing so the readback isn't in the loop;
    // buffers still hold the last run's output)
    let note = format!("xcheck {xcheck:.3} ms; {}", verify(ctx, spec)?);

    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Ok(Stats {
        median: times[times.len() / 2],
        p25: times[times.len() / 4],
        p75: times[3 * times.len() / 4],
        note,
    })
}

fn read_buffer(ctx: &Ctx, buf: &wgpu::Buffer, size: u64) -> Vec<u8> {
    let staging = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(buf, 0, &staging, 0, size);
    ctx.queue.submit([encoder.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
    ctx.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("poll");
    let v = slice.get_mapped_range().to_vec();
    staging.unmap();
    v
}

fn storage_buffer(ctx: &Ctx, data: &[u8]) -> wgpu::Buffer {
    let buf = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: data.len() as u64,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    ctx.queue.write_buffer(&buf, 0, data);
    buf
}

fn collatz_spec(ctx: &Ctx, len: u32) -> Bufs {
    let input: Vec<u32> = (1..=len).collect();
    let raw: Vec<u8> = bytemuck::cast_slice(&input).to_vec();
    Bufs {
        bufs: vec![storage_buffer(ctx, &raw)],
        writable: vec![true],
        reupload: vec![(0, raw)],
        read_binding: 0,
        workgroups: (len.div_ceil(64), 1, 1),
        entry: "collatz_cs",
        verify: VerifyKind::Collatz(len),
    }
}

/// Deterministic pseudo-random f32s in [-1, 1) without pulling in a rand crate.
fn lcg_floats(n: usize, mut state: u32) -> Vec<f32> {
    (0..n)
        .map(|_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            (state >> 8) as f32 / 8_388_608.0 - 1.0
        })
        .collect()
}

fn matmul_spec(ctx: &Ctx, entry: &'static str, n: u32) -> Bufs {
    let a = lcg_floats((n * n) as usize, 1);
    let b = lcg_floats((n * n) as usize, 2);
    Bufs {
        bufs: vec![
            storage_buffer(ctx, bytemuck::cast_slice(&[n])),
            storage_buffer(ctx, bytemuck::cast_slice(&a)),
            storage_buffer(ctx, bytemuck::cast_slice(&b)),
            storage_buffer(ctx, &vec![0u8; (n * n * 4) as usize]),
        ],
        writable: vec![false, false, false, true],
        reupload: vec![],
        read_binding: 3,
        workgroups: (n.div_ceil(16), n.div_ceil(16), 1),
        entry,
        verify: VerifyKind::Matmul(n),
    }
}

fn render_spec(ctx: &Ctx, entry: &'static str, r: RenderParams) -> Bufs {
    let params = [r.width, r.height, r.samples, r.seed];
    let out_size = (r.width * r.height * 3 * 4) as usize;
    Bufs {
        bufs: vec![
            storage_buffer(ctx, bytemuck::cast_slice(&params)),
            storage_buffer(ctx, &vec![0u8; out_size]),
        ],
        writable: vec![false, true],
        reupload: vec![],
        read_binding: 1,
        workgroups: (r.width.div_ceil(8), r.height.div_ceil(8), 1),
        entry,
        verify: VerifyKind::Render(r),
    }
}

fn verify(ctx: &Ctx, spec: &Bufs) -> Result<String, String> {
    let raw = read_buffer(
        ctx,
        &spec.bufs[spec.read_binding],
        spec.bufs[spec.read_binding].size(),
    );
    match spec.verify {
        VerifyKind::Collatz(len) => {
            let out: &[u32] = bytemuck::cast_slice(&raw);
            let bad = (1..=len)
                .filter(|&n| out[(n - 1) as usize] != gpu_shared::collatz_steps(n))
                .count();
            if bad == 0 {
                Ok(format!("verified {len} exact"))
            } else {
                Err(format!("{bad} mismatches"))
            }
        }
        VerifyKind::Matmul(n) => verify_matmul(&raw, n),
        VerifyKind::Render(r) => verify_render(&raw, r),
    }
}

fn verify_matmul(raw: &[u8], n: u32) -> Result<String, String> {
    let out: &[f32] = bytemuck::cast_slice(raw);
    let a = lcg_floats((n * n) as usize, 1);
    let b = lcg_floats((n * n) as usize, 2);
    let mut state = 12345u32;
    let mut worst = 0.0f32;
    for _ in 0..1000 {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let row = (state >> 16) % n;
        let col = state % n;
        let expected = gpu_shared::matmul::matmul_element(&a, &b, n, row, col);
        let diff = (out[(row * n + col) as usize] - expected).abs();
        let rel = diff / expected.abs().max(1.0);
        worst = worst.max(rel);
    }
    if worst < 1.0e-3 {
        Ok(format!("verified 1000 samples, worst rel {worst:.1e}"))
    } else {
        Err(format!("worst rel diff {worst:.1e}"))
    }
}

fn verify_render(raw: &[u8], r: RenderParams) -> Result<String, String> {
    let out: &[f32] = bytemuck::cast_slice(raw);
    let mut sum = 0.0f64;
    let mut count = 0usize;
    // CPU oracle re-traces sampled pixels at full spp — bound to ~4000 pixels so
    // the gate stays cheap even when samples climbs in the saturation sweep
    let step = (r.width * r.height / 4000).max(7);
    let mut i = 0u32;
    while i < r.width * r.height {
        let c = gpu_shared::render_pixel(i % r.width, i / r.width, &r);
        let base = (i * 3) as usize;
        sum += ((out[base] - c.x).abs()
            + (out[base + 1] - c.y).abs()
            + (out[base + 2] - c.z).abs()) as f64;
        count += 3;
        i += step;
    }
    let mean = sum / count as f64;
    if mean < 1.0e-3 {
        Ok(format!("verified, mean diff {mean:.1e}"))
    } else {
        Err(format!("mean diff {mean:.1e} too high"))
    }
}
