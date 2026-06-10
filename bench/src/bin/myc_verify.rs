//! GPU-vs-CPU determinism gate for the mycelium kernels (MYCELIA.md T4).
//!
//! Runs the transport solver and a deterministic grow step on the GPU — the
//! real shipped rust-gpu SPIR-V, loaded via PASSTHROUGH_SHADERS so no naga
//! sits in the path — and against the CPU reference in `gpu_shared::mycelium`,
//! then asserts they agree. This is the proof that the rendered organism is
//! exactly the math the CPU unit tests cover, not a lookalike.
//!
//!   transport: pure field arithmetic (no transcendentals) -> tight epsilon
//!              (only fma contraction can separate GPU from CPU).
//!   grow:      axis-aligned tips on a flat nutrient field hold heading, so
//!              cos/sin are exact and the whole step is bit-exact.
//!
//! Run `cargo gpu build --target spirv-unknown-vulkan` first (-> shaders/spv).
//! Exit 0 iff both gates pass.

use std::mem::size_of;

use gpu_shared::mycelium::{self, Params, Tip};

struct Ctx {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

fn main() {
    let ctx = pollster::block_on(init());
    let spv = std::fs::read(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../shaders/spv/gpu_shaders.spv"),
    )
    .expect("run `cargo gpu build --target spirv-unknown-vulkan` first (-> gpu/shaders/spv)");
    let module = unsafe {
        ctx.device
            .create_shader_module_passthrough(wgpu::ShaderModuleDescriptorPassthrough {
                label: Some("mycelium-spv"),
                spirv: Some(wgpu::util::make_spirv_raw(&spv)),
                ..Default::default()
            })
    };

    let mut ok = true;
    ok &= transport_gate(&ctx, &module);
    ok &= grow_gate(&ctx, &module);
    println!("\nGATE: {}", if ok { "OK" } else { "FAIL" });
    std::process::exit(if ok { 0 } else { 1 });
}

// ---------------------------------------------------------------- transport

fn transport_gate(ctx: &Ctx, module: &wgpu::ShaderModule) -> bool {
    let (w, h) = (32u32, 32u32);
    let n = (w * h) as usize;
    let mut p = Params::default_for(w, h, 0);
    p.alpha = 0.2;
    p.k_half = 2.0;
    let k_iters = 8;

    // deterministic non-negative fields
    let biomass: Vec<f32> = lcg(n, 11).iter().map(|x| x * 2.0 + 2.0).collect(); // ~[0,4]
    let resource0: Vec<f32> = lcg(n, 22).iter().map(|x| (x + 1.0) * 5.0).collect(); // ~[0,10]

    // CPU reference: K Jacobi sweeps, flux accumulated (same as the shader loop)
    let mut cpu_res = resource0.clone();
    let mut cpu_flux = vec![0.0f32; n];
    for _ in 0..k_iters {
        let mut next = cpu_res.clone();
        for y in 0..h {
            for x in 0..w {
                let f = mycelium::transport_at(x, y, &cpu_res, &biomass, &p);
                let i = (y * w + x) as usize;
                next[i] = f.resource;
                cpu_flux[i] += f.flux;
            }
        }
        cpu_res = next;
    }

    // GPU: same K iterations, ping-ponging resource, flux accumulating
    let pbuf = storage_buffer(ctx, &as_bytes(&p));
    let bmass = storage_buffer(ctx, bytes(&biomass));
    let res = [
        storage_buffer(ctx, bytes(&resource0)),
        storage_buffer(ctx, &vec![0u8; n * 4]),
    ];
    let flux = storage_buffer(ctx, &vec![0u8; n * 4]);
    let wg = (w.div_ceil(8), h.div_ceil(8), 1);
    let mut cur = 0usize;
    for _ in 0..k_iters {
        dispatch(
            ctx,
            module,
            "mycelium_transport_cs",
            &[
                (&pbuf, false),
                (&bmass, false),
                (&res[cur], false),
                (&res[1 - cur], true),
                (&flux, true),
            ],
            wg,
        );
        cur = 1 - cur;
    }
    let gpu_res = read_f32(ctx, &res[cur], n);
    let gpu_flux = read_f32(ctx, &flux, n);

    let re = max_rel(&cpu_res, &gpu_res);
    let fe = max_rel(&cpu_flux, &gpu_flux);
    let ok = re < 1e-4 && fe < 1e-4;
    println!(
        "transport ({w}x{h}, K={k_iters}): max rel err  resource={re:.2e}  flux={fe:.2e}  -> {}",
        if ok { "OK" } else { "FAIL" }
    );
    ok
}

// --------------------------------------------------------------------- grow

fn grow_gate(ctx: &Ctx, module: &wgpu::ShaderModule) -> bool {
    let (w, h) = (64u32, 48u32);
    let n = (w * h) as usize;
    let n_tips = 8u32;
    let p = Params::default_for(w, h, n_tips);

    // flat nutrient => sensors equal => tips hold heading 0 (cos/sin exact);
    // biomass 0 => no anastomosis; per-tip home resource straddles growth_cost
    // so some advance and some stall. Distinct rows keep cells disjoint (the
    // GPU dispatch is parallel; with no overlap there is nothing to race).
    let nutrient = vec![0.5f32; n];
    let biomass0 = vec![0.0f32; n];
    let mut resource0 = vec![0.0f32; n];
    let mut tips = vec![Tip::DEAD; n_tips as usize];
    for k in 0..n_tips {
        let (x, y) = (10.0f32, (4 + 5 * k) as f32);
        tips[k as usize] = Tip { x, y, heading: 0.0, colony: 0.0, alive: 1.0, age: 3.0 };
        resource0[mycelium::index(x, y, &p) as usize] = 0.1 * k as f32; // 0.0..0.7
    }

    // CPU reference: every tip reads the ORIGINAL fields (mirrors the parallel
    // GPU reads), then its disjoint writes are applied — exactly the shader body.
    let mut c_tips = tips.clone();
    let mut c_nut = nutrient.clone();
    let mut c_bio = biomass0.clone();
    let mut c_res = resource0.clone();
    for k in 0..n_tips {
        let step = mycelium::grow_tip(&tips[k as usize], &nutrient, &biomass0, &resource0, &p, k);
        c_tips[k as usize] = step.tip;
        let nu = c_nut[step.home as usize] - step.forage;
        c_nut[step.home as usize] = if nu > 0.0 { nu } else { 0.0 };
        c_res[step.home as usize] += step.resource_delta;
        if step.advanced == 1 {
            c_bio[step.cell as usize] += p.deposit;
        }
    }

    // GPU
    let pbuf = storage_buffer(ctx, &as_bytes(&p));
    let tbuf = storage_buffer(ctx, &tips_bytes(&tips));
    let nbuf = storage_buffer(ctx, bytes(&nutrient));
    let bbuf = storage_buffer(ctx, bytes(&biomass0));
    let rbuf = storage_buffer(ctx, bytes(&resource0));
    dispatch(
        ctx,
        module,
        "mycelium_grow_cs",
        &[
            (&pbuf, false),
            (&tbuf, true),
            (&nbuf, true),
            (&bbuf, true),
            (&rbuf, true),
        ],
        (n_tips.div_ceil(64), 1, 1),
    );
    let g_tips = read_f32(ctx, &tbuf, n_tips as usize * 6);
    let g_nut = read_f32(ctx, &nbuf, n);
    let g_bio = read_f32(ctx, &bbuf, n);
    let g_res = read_f32(ctx, &rbuf, n);

    // tips are 6 f32 each, packed identically on both sides
    let c_tipf = tips_to_f32(&c_tips);
    let tip_e = max_abs(&c_tipf, &g_tips);
    let ne = max_abs(&c_nut, &g_nut);
    let be = max_abs(&c_bio, &g_bio);
    let re = max_abs(&c_res, &g_res);
    let ok = tip_e <= 1e-6 && ne <= 1e-6 && be <= 1e-6 && re <= 1e-6;
    println!(
        "grow ({n_tips} tips): max abs diff  tips={tip_e:.2e}  nutrient={ne:.2e}  biomass={be:.2e}  resource={re:.2e}  -> {}",
        if ok { "OK" } else { "FAIL" }
    );
    ok
}

// ------------------------------------------------------------------ helpers

async fn init() -> Ctx {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("no adapter");
    let need = wgpu::Features::PASSTHROUGH_SHADERS;
    assert!(
        adapter.features().contains(need),
        "adapter lacks PASSTHROUGH_SHADERS"
    );
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("myc-verify"),
            required_features: need,
            ..Default::default()
        })
        .await
        .expect("device");
    Ctx { device, queue }
}

fn dispatch(
    ctx: &Ctx,
    module: &wgpu::ShaderModule,
    entry: &str,
    bufs: &[(&wgpu::Buffer, bool)],
    wg: (u32, u32, u32),
) {
    let layout_entries: Vec<wgpu::BindGroupLayoutEntry> = bufs
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
    let bgl = ctx
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &layout_entries,
        });
    let entries: Vec<wgpu::BindGroupEntry> = bufs
        .iter()
        .enumerate()
        .map(|(i, (b, _))| wgpu::BindGroupEntry {
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
            entry_point: Some(entry),
            compilation_options: Default::default(),
            cache: None,
        });
    let mut encoder = ctx.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(wg.0, wg.1, wg.2);
    }
    ctx.queue.submit([encoder.finish()]);
    ctx.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("poll");
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

fn read_f32(ctx: &Ctx, buf: &wgpu::Buffer, n: usize) -> Vec<f32> {
    let size = (n * 4) as u64;
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
    let bytes = slice.get_mapped_range().to_vec();
    staging.unmap();
    bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

/// Deterministic pseudo-random f32s in [-1, 1) (same LCG the bench uses).
fn lcg(n: usize, mut s: u32) -> Vec<f32> {
    (0..n)
        .map(|_| {
            s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            (s >> 8) as f32 / 8_388_608.0 - 1.0
        })
        .collect()
}

fn as_bytes<T: Copy>(v: &T) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(v as *const T as *const u8, size_of::<T>()).to_vec() }
}
fn bytes(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)) }
}
fn tips_bytes(v: &[Tip]) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, std::mem::size_of_val(v)).to_vec() }
}
fn tips_to_f32(v: &[Tip]) -> Vec<f32> {
    v.iter()
        .flat_map(|t| [t.x, t.y, t.heading, t.colony, t.alive, t.age])
        .collect()
}

fn max_abs(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f32::max)
}
fn max_rel(a: &[f32], b: &[f32]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs() / x.abs().max(1e-6))
        .fold(0.0, f32::max)
}
