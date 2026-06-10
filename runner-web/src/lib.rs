//! WASM CPU arm: raw C-ABI exports, no wasm-bindgen. JS instantiates the
//! module directly, calls `alloc`, then renders row chunks into that buffer
//! as RGBA8 (gamma 2.0), and blits via ImageData.

use gpu_shared::{render_pixel, RenderParams};

/// Leak a buffer and hand the offset to JS. One allocation per page load —
/// deliberate, the "allocator" is the demo's scratch image.
#[no_mangle]
pub extern "C" fn alloc(len: u32) -> *mut u8 {
    let mut v = vec![0u8; len as usize];
    let ptr = v.as_mut_ptr();
    std::mem::forget(v);
    ptr
}

/// Physarum CPU arm: spawn agents into a buffer JS allocated via `alloc`.
#[no_mangle]
pub extern "C" fn sim_spawn(agents_ptr: *mut u8, width: u32, height: u32, n: u32) {
    use gpu_shared::physarum::{spawn_agent, Agent, SimParams};
    let p = SimParams::default_for(width, height, n);
    let agents =
        unsafe { std::slice::from_raw_parts_mut(agents_ptr as *mut Agent, n as usize) };
    for (i, a) in agents.iter_mut().enumerate() {
        *a = spawn_agent(i as u32, &p);
    }
}

/// One sim step: update+deposit every agent on `trail`, then diffuse into
/// `tmp` and copy back. Slider params come in as scalars.
///
/// # Safety
/// Pointers must be `alloc`ations of n*16 / w*h*4 / w*h*4 bytes respectively.
#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub unsafe extern "C" fn sim_step(
    agents_ptr: *mut u8,
    trail_ptr: *mut u8,
    tmp_ptr: *mut u8,
    width: u32,
    height: u32,
    n: u32,
    frame: u32,
    move_speed: f32,
    turn_speed: f32,
    sensor_angle: f32,
    sensor_dist: f32,
    deposit: f32,
    decay: f32,
) {
    use gpu_shared::physarum::{diffuse_at, update_agent, Agent, SimParams};
    let p = SimParams {
        width,
        height,
        n_agents: n,
        frame,
        move_speed,
        turn_speed,
        sensor_angle,
        sensor_dist,
        deposit,
        decay,
    };
    let agents = std::slice::from_raw_parts_mut(agents_ptr as *mut Agent, n as usize);
    let cells = (width * height) as usize;
    let trail = std::slice::from_raw_parts_mut(trail_ptr as *mut f32, cells);
    let tmp = std::slice::from_raw_parts_mut(tmp_ptr as *mut f32, cells);
    for (i, a) in agents.iter_mut().enumerate() {
        let (next, cell) = update_agent(a, trail, &p, i as u32);
        *a = next;
        trail[cell as usize] += p.deposit;
    }
    for y in 0..height {
        for x in 0..width {
            tmp[(y * width + x) as usize] = diffuse_at(trail, x, y, &p);
        }
    }
    trail.copy_from_slice(tmp);
}

/// Render rows [y0, y1) into `ptr` (RGBA8, full-image stride).
///
/// # Safety
/// `ptr` must point at an `alloc`ation of at least width*height*4 bytes.
#[no_mangle]
pub unsafe extern "C" fn render_rows(
    ptr: *mut u8,
    width: u32,
    height: u32,
    samples: u32,
    seed: u32,
    y0: u32,
    y1: u32,
) {
    let params = RenderParams {
        width,
        height,
        samples,
        seed,
    };
    let buf = std::slice::from_raw_parts_mut(ptr, (width * height * 4) as usize);
    for y in y0..y1.min(height) {
        for x in 0..width {
            let c = render_pixel(x, y, &params);
            let base = ((y * width + x) * 4) as usize;
            for (k, ch) in [c.x, c.y, c.z].into_iter().enumerate() {
                buf[base + k] = (ch.max(0.0).sqrt().min(1.0) * 255.999) as u8;
            }
            buf[base + 3] = 255;
        }
    }
}
