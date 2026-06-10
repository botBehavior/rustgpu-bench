//! GPU entry points. Thin wrappers only — all real math lives in `gpu_shared`
//! so it can be unit-tested on the CPU and reused by every runner.
#![no_std]

use spirv_std::glam::UVec3;
use spirv_std::spirv;

use gpu_shader_lib::{color, noise, sdf2};
use spirv_std::glam::{vec2, Vec3};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

/// Gallery smoke entry: every shaderlib module in one image.
/// params: [width, height, time_ms, _pad]; out: rgb f32 per pixel.
#[spirv(compute(threads(8, 8)))]
pub fn demo_plasma_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &[u32; 4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] out: &mut [f32],
) {
    let w = params[0];
    let h = params[1];
    if id.x >= w || id.y >= h {
        return;
    }
    let t = params[2] as f32 * 1.0e-3;
    let res = vec2(w as f32, h as f32);
    let uv = (vec2(id.x as f32, id.y as f32) - 0.5 * res) / res.y;

    // FBM-warped cosine-palette plasma, masked by a smooth-union SDF scene
    let n = noise::fbm2(uv * 3.0 + vec2(t * 0.3, -t * 0.2), 5, 7);
    let d = sdf2::smooth_union(
        sdf2::circle(uv - vec2(0.25 * (t * 0.7).cos(), 0.0), 0.35),
        sdf2::rounded_box(uv + vec2(0.25 * (t * 0.5).sin(), 0.0), vec2(0.28, 0.18), 0.05),
        0.25,
    );
    let glow = (-6.0 * d.abs()).exp();
    let base = color::palette_rainbow(n + t * 0.05);
    let lit = base * (0.25 + 0.75 * glow) + Vec3::splat(glow * glow * 0.6);
    let c = color::srgb_encode(color::tonemap_aces(lit));

    let base_i = ((id.y * w + id.x) * 3) as usize;
    out[base_i] = c.x;
    out[base_i + 1] = c.y;
    out[base_i + 2] = c.z;
}

use gpu_shared::physarum::{Agent, SimParams};
use gpu_shared::RenderParams;

#[spirv(compute(threads(64)))]
pub fn physarum_spawn_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &SimParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] agents: &mut [Agent],
) {
    let i = id.x;
    if i < params.n_agents {
        agents[i as usize] = gpu_shared::physarum::spawn_agent(i, params);
    }
}

#[spirv(compute(threads(64)))]
pub fn physarum_update_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &SimParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] agents: &mut [Agent],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] trail: &mut [f32],
) {
    let i = id.x;
    if i >= params.n_agents {
        return;
    }
    let (next, cell) = gpu_shared::physarum::update_agent(&agents[i as usize], trail, params, i);
    agents[i as usize] = next;
    // non-atomic deposit: races lose a few deposits, visually irrelevant (v1)
    trail[cell as usize] += params.deposit;
}

#[spirv(compute(threads(8, 8)))]
pub fn physarum_diffuse_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &SimParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] trail_in: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] trail_out: &mut [f32],
) {
    let x = id.x;
    let y = id.y;
    if x < params.width && y < params.height {
        trail_out[(y * params.width + x) as usize] =
            gpu_shared::physarum::diffuse_at(trail_in, x, y, params);
    }
}

#[spirv(compute(threads(8, 8)))]
pub fn render_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &RenderParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] out: &mut [f32],
) {
    let x = id.x;
    let y = id.y;
    if x >= params.width || y >= params.height {
        return;
    }
    let c = gpu_shared::render_pixel(x, y, params);
    let base = ((y * params.width + x) * 3) as usize;
    out[base] = c.x;
    out[base + 1] = c.y;
    out[base + 2] = c.z;
}

#[spirv(compute(threads(8, 8)))]
pub fn render_v2_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &RenderParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] out: &mut [f32],
) {
    let x = id.x;
    let y = id.y;
    if x >= params.width || y >= params.height {
        return;
    }
    let c = gpu_shared::tracer_v2::render_pixel_v2(x, y, params);
    let base = ((y * params.width + x) * 3) as usize;
    out[base] = c.x;
    out[base + 1] = c.y;
    out[base + 2] = c.z;
}

#[spirv(compute(threads(16, 16)))]
pub fn matmul_unchecked_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] dims: &u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] a: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] b: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] c: &mut [f32],
) {
    let n = *dims;
    let col = id.x;
    let row = id.y;
    if row >= n || col >= n {
        return;
    }
    c[(row * n + col) as usize] =
        unsafe { gpu_shared::matmul::matmul_element_unchecked(a, b, n, row, col) };
}

#[spirv(compute(threads(16, 16)))]
pub fn matmul_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] dims: &u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] a: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] b: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] c: &mut [f32],
) {
    let n = *dims;
    let col = id.x;
    let row = id.y;
    if row >= n || col >= n {
        return;
    }
    c[(row * n + col) as usize] = gpu_shared::matmul::matmul_element(a, b, n, row, col);
}

#[spirv(compute(threads(64)))]
pub fn collatz_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] data: &mut [u32],
) {
    let i = id.x as usize;
    if i < data.len() {
        data[i] = gpu_shared::collatz_steps(data[i]);
    }
}
