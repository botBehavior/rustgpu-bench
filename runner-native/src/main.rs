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
    let ok = run_collatz(&gpu) & run_render_compare(&gpu);
    write_showcase(&gpu);
    if !ok {
        std::process::exit(1);
    }
}
