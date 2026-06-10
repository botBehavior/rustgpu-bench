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
