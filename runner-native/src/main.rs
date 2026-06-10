//! Native GPU host: verifies the rust-gpu kernels against the identical CPU
//! functions from `gpu-shared`.
//!
//! Modes:
//!   runner-native              # collatz verify + render compare (passthrough if available)
//!   runner-native --naga       # same, but force the naga SPIR-V frontend path
//!
//! Optional: --spv <path> to point at a specific .spv artifact.

use std::io::Write as _;
use std::path::PathBuf;

use gpu_shared::RenderParams;

const COLLATZ_N: usize = 1 << 16;
const VERIFY: RenderParams = RenderParams {
    width: 320,
    height: 180,
    samples: 8,
    seed: 7,
};
const SHOWCASE: RenderParams = RenderParams {
    width: 800,
    height: 450,
    samples: 32,
    seed: 7,
};
/// f32 divergence allowed between CPU and GPU evaluation of the same math.
const TOLERANCE: f32 = 2.0e-3;

fn find_spv(cli: Option<String>) -> PathBuf {
    if let Some(p) = cli {
        return PathBuf::from(p);
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../shaders/spv/gpu_shaders.spv")
}

struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    module: wgpu::ShaderModule,
}

impl Gpu {
    async fn new(spv_bytes: &[u8], force_naga: bool) -> Self {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .expect("no GPU adapter found");
        let info = adapter.get_info();
        println!("adapter: {} ({:?})", info.name, info.backend);

        let passthrough = !force_naga
            && adapter
                .features()
                .contains(wgpu::Features::PASSTHROUGH_SHADERS);
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("device"),
                required_features: if passthrough {
                    wgpu::Features::PASSTHROUGH_SHADERS
                } else {
                    wgpu::Features::empty()
                },
                ..Default::default()
            })
            .await
            .expect("failed to create device");

        let module = if passthrough {
            println!("shader path: SPIR-V passthrough");
            unsafe {
                device.create_shader_module_passthrough(wgpu::ShaderModuleDescriptorPassthrough {
                    label: Some("kernels"),
                    spirv: Some(wgpu::util::make_spirv_raw(spv_bytes)),
                    ..Default::default()
                })
            }
        } else {
            println!("shader path: naga SPIR-V frontend");
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("kernels"),
                source: wgpu::util::make_spirv(spv_bytes),
            })
        };
        Gpu {
            device,
            queue,
            module,
        }
    }

    /// One storage-buffer-only compute dispatch + readback of buffer 0.
    fn dispatch_and_read(
        &self,
        entry: &str,
        buffers: &[(&[u8], bool)], // (initial contents, writable by shader)
        read_binding: u32,
        workgroups: (u32, u32, u32),
    ) -> Vec<u8> {
        let bufs: Vec<wgpu::Buffer> = buffers
            .iter()
            .map(|(data, _)| {
                let buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size: data.len() as u64,
                    usage: wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_DST
                        | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: false,
                });
                self.queue.write_buffer(&buf, 0, data);
                buf
            })
            .collect();

        let entries_layout: Vec<wgpu::BindGroupLayoutEntry> = buffers
            .iter()
            .enumerate()
            .map(|(i, (_, writable))| wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                        read_only: !writable,
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let bgl = self
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries_layout,
            });
        let entries: Vec<wgpu::BindGroupEntry> = bufs
            .iter()
            .enumerate()
            .map(|(i, b)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: b.as_entire_binding(),
            })
            .collect();
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bgl,
            entries: &entries,
        });
        let layout = self
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&bgl)],
                immediate_size: 0,
            });
        let pipeline = self
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: Some(&layout),
                module: &self.module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            });

        let read_buf = &bufs[read_binding as usize];
        let read_size = buffers[read_binding as usize].0.len() as u64;
        let readback = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: read_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
        }
        encoder.copy_buffer_to_buffer(read_buf, 0, &readback, 0, read_size);
        self.queue.submit([encoder.finish()]);

        let slice = readback.slice(..);
        slice.map_async(wgpu::MapMode::Read, |r| r.expect("map failed"));
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("poll failed");
        let out = slice.get_mapped_range().to_vec();
        readback.unmap();
        out
    }
}

fn run_collatz(gpu: &Gpu) -> bool {
    let input: Vec<u32> = (1..=COLLATZ_N as u32).collect();
    let raw = gpu.dispatch_and_read(
        "collatz_cs",
        &[(bytemuck::cast_slice(&input), true)],
        0,
        ((COLLATZ_N as u32).div_ceil(64), 1, 1),
    );
    let gpu_out: &[u32] = bytemuck::cast_slice(&raw);
    let mismatches = input
        .iter()
        .enumerate()
        .filter(|(i, &n)| gpu_out[*i] != gpu_shared::collatz_steps(n))
        .count();
    if mismatches == 0 {
        println!("collatz: PASS ({COLLATZ_N} elements, GPU == CPU)");
        true
    } else {
        eprintln!("collatz: FAIL ({mismatches}/{COLLATZ_N} mismatches)");
        false
    }
}

fn gpu_render(gpu: &Gpu, params: &RenderParams) -> Vec<f32> {
    let params_raw = [params.width, params.height, params.samples, params.seed];
    let out_len = (params.width * params.height * 3) as usize;
    let zeros = vec![0u8; out_len * 4];
    let raw = gpu.dispatch_and_read(
        "render_cs",
        &[(bytemuck::cast_slice(&params_raw), false), (&zeros, true)],
        1,
        (params.width.div_ceil(8), params.height.div_ceil(8), 1),
    );
    bytemuck::cast_slice(&raw).to_vec()
}

/// CPU vs GPU comparison is statistical, not bitwise: GPU sin/cos/fma differ
/// from CPU libm by ulps, and a path tracer is chaotic — a knife-edge branch
/// (grazing hit vs miss) occasionally flips, changing one sample entirely.
/// Identical *math* manifests as: tiny mean error + a small fraction of
/// single-sample outliers. Gate on both.
fn run_render_compare(gpu: &Gpu) -> bool {
    let gpu_px = gpu_render(gpu, &VERIFY);
    let mut worst = 0.0f32;
    let mut sum = 0.0f64;
    let mut over = 0usize;
    for y in 0..VERIFY.height {
        for x in 0..VERIFY.width {
            let c = gpu_shared::render_pixel(x, y, &VERIFY);
            let base = ((y * VERIFY.width + x) * 3) as usize;
            for (k, expected) in [c.x, c.y, c.z].into_iter().enumerate() {
                let diff = (gpu_px[base + k] - expected).abs();
                worst = worst.max(diff);
                sum += diff as f64;
                if diff > TOLERANCE {
                    over += 1;
                }
            }
        }
    }
    let total = (VERIFY.width * VERIFY.height * 3) as usize;
    let mean = sum / total as f64;
    let over_pct = 100.0 * over as f64 / total as f64;
    println!(
        "render: {}x{} @ {} spp — mean diff {:.2e}, worst {:.2e}, {:.3}% channels over {:.0e}",
        VERIFY.width, VERIFY.height, VERIFY.samples, mean, worst, over_pct, TOLERANCE
    );
    if mean < 1.0e-3 && over_pct < 1.0 {
        println!("render: PASS (statistically identical; outliers are single-sample branch flips)");
        true
    } else {
        eprintln!("render: FAIL (divergence beyond float-nondeterminism levels)");
        false
    }
}

fn write_showcase(gpu: &Gpu) {
    let px = gpu_render(gpu, &SHOWCASE);
    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../out-gpu.ppm");
    let mut f = std::io::BufWriter::new(std::fs::File::create(&out_path).unwrap());
    writeln!(f, "P6\n{} {}\n255", SHOWCASE.width, SHOWCASE.height).unwrap();
    for ch in &px {
        let g = ch.max(0.0).sqrt().min(1.0);
        f.write_all(&[(g * 255.999) as u8]).unwrap();
    }
    println!("wrote {}", out_path.display());
}

fn params_bytes(p: &gpu_shared::physarum::SimParams) -> Vec<u8> {
    let mut v = Vec::new();
    for u in [p.width, p.height, p.n_agents, p.frame] {
        v.extend_from_slice(&u.to_le_bytes());
    }
    for f in [
        p.move_speed,
        p.turn_speed,
        p.sensor_angle,
        p.sensor_dist,
        p.deposit,
        p.decay,
    ] {
        v.extend_from_slice(&f.to_le_bytes());
    }
    v
}

struct SimPipelines {
    spawn: wgpu::ComputePipeline,
    update: wgpu::ComputePipeline,
    diffuse: wgpu::ComputePipeline,
    spawn_bgl: wgpu::BindGroupLayout,
    update_bgl: wgpu::BindGroupLayout,
    diffuse_bgl: wgpu::BindGroupLayout,
}

fn sim_pipelines(gpu: &Gpu) -> SimPipelines {
    let bgl = |writable: &[bool]| {
        let entries: Vec<wgpu::BindGroupLayoutEntry> = writable
            .iter()
            .enumerate()
            .map(|(i, w)| wgpu::BindGroupLayoutEntry {
                binding: i as u32,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: !w },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        gpu.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            })
    };
    let pipe = |entry: &str, layout: &wgpu::BindGroupLayout| {
        let pl = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(layout)],
                immediate_size: 0,
            });
        gpu.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: Some(&pl),
                module: &gpu.module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
    };
    let spawn_bgl = bgl(&[false, true]);
    let update_bgl = bgl(&[false, true, true]);
    let diffuse_bgl = bgl(&[false, false, true]);
    SimPipelines {
        spawn: pipe("physarum_spawn_cs", &spawn_bgl),
        update: pipe("physarum_update_cs", &update_bgl),
        diffuse: pipe("physarum_diffuse_cs", &diffuse_bgl),
        spawn_bgl,
        update_bgl,
        diffuse_bgl,
    }
}

/// GPU-vs-CPU verification of the Physarum kernels.
/// Multi-agent GPU runs are not bit-comparable (non-atomic deposits by
/// design), so the gates are: spawn parity, single-agent trajectory within
/// tolerance, diffuse near-exact, multi-agent smoke (finite, in-bounds, mass).
fn run_sim_verify(gpu: &Gpu) -> bool {
    use gpu_shared::physarum::{self, SimParams};
    let pipes = sim_pipelines(gpu);
    let mut ok = true;

    let storage = |data: &[u8]| {
        let buf = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: data.len() as u64,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        gpu.queue.write_buffer(&buf, 0, data);
        buf
    };
    let bind = |layout: &wgpu::BindGroupLayout, bufs: &[&wgpu::Buffer]| {
        let entries: Vec<wgpu::BindGroupEntry> = bufs
            .iter()
            .enumerate()
            .map(|(i, b)| wgpu::BindGroupEntry {
                binding: i as u32,
                resource: b.as_entire_binding(),
            })
            .collect();
        gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &entries,
        })
    };
    let read_f32s = |buf: &wgpu::Buffer| -> Vec<f32> {
        let staging = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: buf.size(),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let mut enc = gpu.device.create_command_encoder(&Default::default());
        enc.copy_buffer_to_buffer(buf, 0, &staging, 0, buf.size());
        gpu.queue.submit([enc.finish()]);
        let slice = staging.slice(..);
        slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("poll");
        let v = bytemuck::cast_slice(&slice.get_mapped_range()).to_vec();
        staging.unmap();
        v
    };
    let dispatch = |pipeline: &wgpu::ComputePipeline, bg: &wgpu::BindGroup, wg: (u32, u32, u32)| {
        let mut enc = gpu.device.create_command_encoder(&Default::default());
        {
            let mut pass = enc.begin_compute_pass(&Default::default());
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, bg, &[]);
            pass.dispatch_workgroups(wg.0, wg.1, wg.2);
        }
        gpu.queue.submit([enc.finish()]);
    };

    // --- 1. spawn parity + single-agent trajectory (20 steps, ping-pong) ---
    let p = SimParams::default_for(128, 96, 1);
    let cells = (p.width * p.height) as usize;
    let params_buf = storage(&params_bytes(&p));
    let agents = storage(&vec![0u8; 16]);
    let trail_a = storage(&vec![0u8; cells * 4]);
    let trail_b = storage(&vec![0u8; cells * 4]);

    dispatch(&pipes.spawn, &bind(&pipes.spawn_bgl, &[&params_buf, &agents]), (1, 1, 1));
    let spawned = read_f32s(&agents);
    let cpu_spawn = physarum::spawn_agent(0, &p);
    if (spawned[0] - cpu_spawn.x).abs() > 1e-3 || (spawned[1] - cpu_spawn.y).abs() > 1e-3 {
        eprintln!(
            "sim spawn: FAIL gpu=({},{}) cpu=({},{})",
            spawned[0], spawned[1], cpu_spawn.x, cpu_spawn.y
        );
        ok = false;
    } else {
        println!("sim spawn: PASS (GPU == CPU within 1e-3)");
    }

    let upd_a = bind(&pipes.update_bgl, &[&params_buf, &agents, &trail_a]);
    let upd_b = bind(&pipes.update_bgl, &[&params_buf, &agents, &trail_b]);
    let dif_ab = bind(&pipes.diffuse_bgl, &[&params_buf, &trail_a, &trail_b]);
    let dif_ba = bind(&pipes.diffuse_bgl, &[&params_buf, &trail_b, &trail_a]);
    let wg2 = (p.width.div_ceil(8), p.height.div_ceil(8), 1);

    let mut cpu_agent = cpu_spawn;
    let mut cpu_trail = vec![0.0f32; cells];
    for step in 0..20 {
        let a_side = step % 2 == 0;
        dispatch(&pipes.update, if a_side { &upd_a } else { &upd_b }, (1, 1, 1));
        dispatch(&pipes.diffuse, if a_side { &dif_ab } else { &dif_ba }, wg2);
        // CPU mirror
        let (next, cell) = physarum::update_agent(&cpu_agent, &cpu_trail, &p, 0);
        cpu_agent = next;
        cpu_trail[cell as usize] += p.deposit;
        cpu_trail = (0..cells)
            .map(|i| {
                physarum::diffuse_at(&cpu_trail, i as u32 % p.width, i as u32 / p.width, &p)
            })
            .collect();
    }
    let gpu_agent = read_f32s(&agents);
    let dx = (gpu_agent[0] - cpu_agent.x).abs();
    let dy = (gpu_agent[1] - cpu_agent.y).abs();
    let gpu_trail = read_f32s(&trail_a); // after even count of steps, current = A
    let gpu_mass: f64 = gpu_trail.iter().map(|v| *v as f64).sum();
    let cpu_mass: f64 = cpu_trail.iter().map(|v| *v as f64).sum();
    let mass_rel = ((gpu_mass - cpu_mass) / cpu_mass.max(1e-9)).abs();
    if dx < 0.05 && dy < 0.05 && mass_rel < 0.005 {
        println!(
            "sim 1-agent x20: PASS (pos diff {:.1e}/{:.1e}, mass rel {:.1e})",
            dx, dy, mass_rel
        );
    } else {
        eprintln!("sim 1-agent x20: FAIL (dx {dx} dy {dy} mass_rel {mass_rel})");
        ok = false;
    }

    // --- 2. diffuse determinism on random field ---
    let mut state = 1u32;
    let rand_trail: Vec<f32> = (0..cells)
        .map(|_| {
            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            (state >> 8) as f32 / 16_777_216.0
        })
        .collect();
    gpu.queue
        .write_buffer(&trail_a, 0, bytemuck::cast_slice(&rand_trail));
    dispatch(&pipes.diffuse, &dif_ab, wg2);
    let gpu_dif = read_f32s(&trail_b);
    let worst = (0..cells)
        .map(|i| {
            (gpu_dif[i]
                - physarum::diffuse_at(&rand_trail, i as u32 % p.width, i as u32 / p.width, &p))
            .abs()
        })
        .fold(0.0f32, f32::max);
    if worst < 1e-5 {
        println!("sim diffuse: PASS (worst diff {worst:.1e})");
    } else {
        eprintln!("sim diffuse: FAIL (worst diff {worst:.1e})");
        ok = false;
    }

    // --- 3. multi-agent smoke: 10k agents, 50 steps ---
    let p2 = SimParams::default_for(256, 192, 10_000);
    let cells2 = (p2.width * p2.height) as usize;
    let params2 = storage(&params_bytes(&p2));
    let agents2 = storage(&vec![0u8; 16 * p2.n_agents as usize]);
    let ta = storage(&vec![0u8; cells2 * 4]);
    let tb = storage(&vec![0u8; cells2 * 4]);
    dispatch(
        &pipes.spawn,
        &bind(&pipes.spawn_bgl, &[&params2, &agents2]),
        (p2.n_agents.div_ceil(64), 1, 1),
    );
    let u_a = bind(&pipes.update_bgl, &[&params2, &agents2, &ta]);
    let u_b = bind(&pipes.update_bgl, &[&params2, &agents2, &tb]);
    let d_ab = bind(&pipes.diffuse_bgl, &[&params2, &ta, &tb]);
    let d_ba = bind(&pipes.diffuse_bgl, &[&params2, &tb, &ta]);
    let wg2b = (p2.width.div_ceil(8), p2.height.div_ceil(8), 1);
    for step in 0..50 {
        let a_side = step % 2 == 0;
        dispatch(
            &pipes.update,
            if a_side { &u_a } else { &u_b },
            (p2.n_agents.div_ceil(64), 1, 1),
        );
        dispatch(&pipes.diffuse, if a_side { &d_ab } else { &d_ba }, wg2b);
    }
    let final_agents = read_f32s(&agents2);
    let final_trail = read_f32s(&ta);
    let mass: f64 = final_trail.iter().map(|v| *v as f64).sum();
    let all_finite = final_trail.iter().all(|v| v.is_finite())
        && final_agents.iter().all(|v| v.is_finite());
    let in_bounds = final_agents.chunks(4).all(|a| {
        a[0] >= 0.0 && a[0] < p2.width as f32 && a[1] >= 0.0 && a[1] < p2.height as f32
    });
    if all_finite && in_bounds && mass > 0.0 {
        println!(
            "sim 10k-agent smoke: PASS (finite, in-bounds, trail mass {:.0})",
            mass
        );
    } else {
        eprintln!("sim 10k-agent smoke: FAIL (finite={all_finite} bounds={in_bounds} mass={mass})");
        ok = false;
    }
    ok
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let force_naga = args.iter().any(|a| a == "--naga");
    let spv_cli = args
        .iter()
        .position(|a| a == "--spv")
        .and_then(|i| args.get(i + 1).cloned());

    let spv_path = find_spv(spv_cli);
    let spv_bytes = std::fs::read(&spv_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", spv_path.display()));
    println!("shader: {} ({} bytes)", spv_path.display(), spv_bytes.len());

    let gpu = pollster::block_on(Gpu::new(&spv_bytes, force_naga));
    if args.iter().any(|a| a == "--sim") {
        if !run_sim_verify(&gpu) {
            std::process::exit(1);
        }
        return;
    }
    let ok = run_collatz(&gpu) & run_render_compare(&gpu);
    write_showcase(&gpu);
    if !ok {
        std::process::exit(1);
    }
}
