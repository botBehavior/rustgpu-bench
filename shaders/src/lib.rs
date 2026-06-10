//! GPU entry points. Thin wrappers only — all real math lives in `gpu_shared`
//! so it can be unit-tested on the CPU and reused by every runner.
#![no_std]

use spirv_std::glam::UVec3;
use spirv_std::spirv;

use spirv_std::glam::vec2;

/// Gallery entry: params = [width, height, time_ms, shader_index].
/// One pipeline serves every shader in `gpu_shader_lib::gallery`.
#[spirv(compute(threads(8, 8)))]
pub fn gallery_cs(
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
    let c = gpu_shader_lib::gallery::by_index(params[3], uv, t);
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

// ---------------- Mycelia: multi-species physarum spectacle ----------------
use gpu_shared::mycelia::{Agent as MyAgent, Params as MyParams, N_SPECIES};

#[spirv(compute(threads(64)))]
pub fn mycelia_spawn_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MyParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] agents: &mut [MyAgent],
) {
    let i = id.x;
    if i < params.n_agents {
        agents[i as usize] = gpu_shared::mycelia::spawn_agent(i, params);
    }
}

#[spirv(compute(threads(64)))]
pub fn mycelia_update_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MyParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] agents: &mut [MyAgent],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] trail: &mut [f32],
) {
    let i = id.x;
    if i >= params.n_agents {
        return;
    }
    let (next, cell) = gpu_shared::mycelia::update_agent(&agents[i as usize], trail, params, i);
    agents[i as usize] = next;
    // non-atomic deposit into the agent's own channel — races lose a few, fine at scale
    trail[cell as usize] += params.deposit;
}

#[spirv(compute(threads(8, 8)))]
pub fn mycelia_diffuse_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MyParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] trail_in: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] trail_out: &mut [f32],
) {
    let x = id.x;
    let y = id.y;
    if x >= params.width || y >= params.height {
        return;
    }
    let cells = params.width * params.height;
    let mut c = 0u32;
    while c < N_SPECIES {
        let idx = c * cells + y * params.width + x;
        trail_out[idx as usize] = gpu_shared::mycelia::diffuse_at(trail_in, c, x, y, params);
        c += 1;
    }
}

/// Trail channels -> luminous color (in tested Rust) -> packed RGBA8 in `out`.
#[spirv(compute(threads(8, 8)))]
pub fn mycelia_render_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MyParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] trail: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] out: &mut [u32],
) {
    let x = id.x;
    let y = id.y;
    if x >= params.width || y >= params.height {
        return;
    }
    let cells = params.width * params.height;
    let i = y * params.width + x;
    let c0 = trail[i as usize];
    let c1 = trail[(cells + i) as usize];
    let c2 = trail[(2 * cells + i) as usize];
    let col = gpu_shared::mycelia::shade(c0, c1, c2, params.exposure);
    let r = (col.x * 255.0) as u32;
    let g = (col.y * 255.0) as u32;
    let b = (col.z * 255.0) as u32;
    out[i as usize] = r | (g << 8) | (b << 16) | (255u32 << 24);
}

// ---------------- Mycelium: adaptive fungal ecosystem ----------------
use gpu_shared::mycelium::{Params as MycParams, Tip as MycTip};

#[spirv(compute(threads(64)))]
pub fn mycelium_spawn_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MycParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] tips: &mut [MycTip],
) {
    let i = id.x;
    if i < params.n_tips {
        tips[i as usize] = gpu_shared::mycelium::spawn_tip(i, params.seed_tips, params);
    }
}

#[spirv(compute(threads(64)))]
pub fn mycelium_grow_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MycParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] tips: &mut [MycTip],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] nutrient: &mut [f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] biomass: &mut [f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] resource: &mut [f32],
) {
    let i = id.x;
    if i >= params.n_tips {
        return;
    }
    let tip = tips[i as usize];
    if tip.alive == 0.0 {
        return;
    }
    let step = gpu_shared::mycelium::grow_tip(&tip, nutrient, biomass, resource, params, i);
    tips[i as usize] = step.tip;

    // resource economy at home (non-atomic; T4 swaps these for atomics). The
    // growth gate guarantees resource stays >= 0 even when resource_delta < 0.
    let nu = nutrient[step.home as usize] - step.forage;
    nutrient[step.home as usize] = if nu > 0.0 { nu } else { 0.0 };
    resource[step.home as usize] += step.resource_delta;

    if step.advanced == 1 {
        biomass[step.cell as usize] += params.deposit; // lay new hypha (non-atomic)
    }
    // step.child (branching) is ignored here: dynamic tip-append needs an atomic
    // counter, which MYCELIA.md groups with T6's spore spawn. The interactive page
    // seeds a dense scattered population instead, so the network is full without it.
}

/// One Jacobi relaxation of the resource-transport solver (T2). One thread per
/// cell; ping-pong `resource_in -> resource_out`. `flux` accumulates throughput
/// across the K iterations a frame runs (caller zeroes it before the K loop).
#[spirv(compute(threads(8, 8)))]
pub fn mycelium_transport_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MycParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] biomass: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] resource_in: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] resource_out: &mut [f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] flux: &mut [f32],
) {
    let x = id.x;
    let y = id.y;
    if x >= params.width || y >= params.height {
        return;
    }
    let i = (y * params.width + x) as usize;
    let f = gpu_shared::mycelium::transport_at(x, y, resource_in, biomass, params);
    resource_out[i] = f.resource;
    flux[i] += f.flux;
}

/// T3 adaptation pass (Tero feedback): thicken/prune biomass from the flux the K
/// transport iterations accumulated, then reset flux for the next frame. Also
/// applies the interactive mouse nutrient drop (a soft disk of food at the cursor)
/// — folded in here because it shares this per-cell dispatch, and a standalone
/// single-`&mut[f32]` threads(8,8) entry is silently culled by this rust-gpu
/// build. One thread per cell.
#[spirv(compute(threads(8, 8)))]
pub fn mycelium_adapt_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MycParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] biomass: &mut [f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] flux: &mut [f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] nutrient: &mut [f32],
) {
    let x = id.x;
    let y = id.y;
    if x >= params.width || y >= params.height {
        return;
    }
    let i = (y * params.width + x) as usize;
    biomass[i] = gpu_shared::mycelium::adapt_at(biomass[i], flux[i], params);
    flux[i] = 0.0; // reset for next frame's transport accumulation

    // interactive feed: soft disk of nutrient at the mouse (mouse_r <= 0 disables)
    if params.mouse_r > 0.0 {
        let dx = x as f32 - params.mouse_x;
        let dy = y as f32 - params.mouse_y;
        let d2 = dx * dx + dy * dy;
        let r2 = params.mouse_r * params.mouse_r;
        if d2 <= r2 {
            nutrient[i] += params.mouse_food * (1.0 - d2 / r2);
        }
    }
}

/// Scattered inoculation: one thread per tip, random position + heading.
#[spirv(compute(threads(64)))]
pub fn mycelium_spawn_scatter_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MycParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] tips: &mut [MycTip],
) {
    let i = id.x;
    if i < params.n_tips {
        tips[i as usize] = gpu_shared::mycelium::spawn_scatter(i, params);
    }
}

#[spirv(compute(threads(8, 8)))]
pub fn mycelium_render_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] params: &MycParams,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] nutrient: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] biomass: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] resource: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] out: &mut [u32],
) {
    let x = id.x;
    let y = id.y;
    if x >= params.width || y >= params.height {
        return;
    }
    let i = (y * params.width + x) as usize;
    let col = gpu_shared::mycelium::shade(biomass[i], nutrient[i], resource[i], params.exposure);
    let r = (col.x * 255.0) as u32;
    let g = (col.y * 255.0) as u32;
    let b = (col.z * 255.0) as u32;
    out[i] = r | (g << 8) | (b << 16) | (255u32 << 24);
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
