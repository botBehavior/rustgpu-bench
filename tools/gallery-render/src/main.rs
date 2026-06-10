//! Render every gallery shader on the GPU, gate each against its CPU oracle
//! (the same Rust function), and write PNG thumbnails for the gallery page.

use std::io::BufWriter;

use glam::vec2;
use gpu_shader_lib::gallery;

const W: u32 = 800;
const H: u32 = 450;
const T_MS: u32 = 2000; // fixed time for reproducible thumbnails
const TOLERANCE_MEAN: f64 = 1.0e-3;

fn main() {
    let (device, queue, module) = pollster::block_on(init());
    let pipeline = {
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[bgl_entry(0, false), bgl_entry(1, true)],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("gallery"),
            layout: Some(&layout),
            module: &module,
            entry_point: Some("gallery_cs"),
            compilation_options: Default::default(),
            cache: None,
        })
    };

    let out_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets/gallery");
    std::fs::create_dir_all(&out_dir).unwrap();
    let mut failed = false;

    for (idx, name) in gallery::NAMES.iter().enumerate() {
        let params = [W, H, T_MS, idx as u32];
        let pixels = render(&device, &queue, &pipeline, params);

        // CPU oracle: sampled statistical gate (every 11th pixel)
        let t = T_MS as f32 * 1.0e-3;
        let mut sum = 0.0f64;
        let mut count = 0usize;
        let mut i = 0u32;
        while i < W * H {
            let uv = (vec2((i % W) as f32, (i / W) as f32) - 0.5 * vec2(W as f32, H as f32))
                / H as f32;
            let c = gallery::by_index(idx as u32, uv, t);
            let b = (i * 3) as usize;
            sum += ((pixels[b] - c.x).abs() + (pixels[b + 1] - c.y).abs()
                + (pixels[b + 2] - c.z).abs()) as f64;
            count += 3;
            i += 11;
        }
        let mean = sum / count as f64;
        let verdict = if mean < TOLERANCE_MEAN { "PASS" } else { "FAIL" };
        println!("{name:12} mean CPU-GPU diff {mean:.2e}  {verdict}");
        if mean >= TOLERANCE_MEAN {
            failed = true;
        }

        // PNG (shader output is already sRGB-encoded)
        let path = out_dir.join(format!("{name}.png"));
        let file = std::fs::File::create(&path).unwrap();
        let mut enc = png::Encoder::new(BufWriter::new(file), W, H);
        enc.set_color(png::ColorType::Rgb);
        enc.set_depth(png::BitDepth::Eight);
        let mut writer = enc.write_header().unwrap();
        let bytes: Vec<u8> = pixels
            .iter()
            .map(|v| (v.clamp(0.0, 1.0) * 255.999) as u8)
            .collect();
        writer.write_image_data(&bytes).unwrap();
        println!("  wrote {}", path.display());
    }
    if failed {
        std::process::exit(1);
    }
}

fn bgl_entry(binding: u32, writable: bool) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage {
                read_only: !writable,
            },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

async fn init() -> (wgpu::Device, wgpu::Queue, wgpu::ShaderModule) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("no adapter");
    let passthrough = adapter
        .features()
        .contains(wgpu::Features::PASSTHROUGH_SHADERS);
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
    let spv = std::fs::read(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../shaders/spv/gpu_shaders.spv"),
    )
    .expect("run cargo gpu build first");
    let module = if passthrough {
        unsafe {
            device.create_shader_module_passthrough(wgpu::ShaderModuleDescriptorPassthrough {
                label: Some("gallery"),
                spirv: Some(wgpu::util::make_spirv_raw(&spv)),
                ..Default::default()
            })
        }
    } else {
        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("gallery"),
            source: wgpu::util::make_spirv(&spv),
        })
    };
    (device, queue, module)
}

fn render(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pipeline: &wgpu::ComputePipeline,
    params: [u32; 4],
) -> Vec<f32> {
    let mk = |size: u64, usage: wgpu::BufferUsages| {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        })
    };
    let pbuf = mk(16, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
    queue.write_buffer(&pbuf, 0, bytemuck::cast_slice(&params));
    let out_size = (W * H * 3 * 4) as u64;
    let obuf = mk(out_size, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC);
    let staging = mk(out_size, wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST);
    let bind = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: pbuf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: obuf.as_entire_binding(),
            },
        ],
    });
    let mut enc = device.create_command_encoder(&Default::default());
    {
        let mut pass = enc.begin_compute_pass(&Default::default());
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &bind, &[]);
        pass.dispatch_workgroups(W.div_ceil(8), H.div_ceil(8), 1);
    }
    enc.copy_buffer_to_buffer(&obuf, 0, &staging, 0, out_size);
    queue.submit([enc.finish()]);
    let slice = staging.slice(..);
    slice.map_async(wgpu::MapMode::Read, |r| r.expect("map"));
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("poll");
    let v = bytemuck::cast_slice(&slice.get_mapped_range()).to_vec();
    staging.unmap();
    v
}
