//! Gallery shaders: each is a pure `(uv, time) -> sRGB` function built on
//! this library, runnable on CPU (tests, oracles) and GPU (the gallery page).
//! uv convention: centered, y-up-ish, scaled so |y| ≤ ~0.5 at frame edges.
//! The `// --- name ---` markers are parsed by the gallery page to show
//! each shader's source next to its live render.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::{color, noise, sdf2};
use glam::{vec2, vec3, Vec2, Vec3};

// --- plasma ---

/// FBM-warped cosine-palette plasma flowing through a smooth-union SDF mask.
pub fn plasma(uv: Vec2, t: f32) -> Vec3 {
    let n = noise::fbm2(uv * 3.0 + vec2(t * 0.3, -t * 0.2), 5, 7);
    let d = sdf2::smooth_union(
        sdf2::circle(uv - vec2(0.25 * (t * 0.7).cos(), 0.0), 0.35),
        sdf2::rounded_box(uv + vec2(0.25 * (t * 0.5).sin(), 0.0), vec2(0.28, 0.18), 0.05),
        0.25,
    );
    let glow = (-6.0 * d.abs()).exp();
    let base = color::palette_rainbow(n + t * 0.05);
    let lit = base * (0.25 + 0.75 * glow) + Vec3::splat(glow * glow * 0.6);
    color::srgb_encode(color::tonemap_aces(lit))
}

// --- amoeba ---

/// The amoeba: five metaball lobes smooth-unioned into one organism, its
/// membrane wobbling with FBM, cytoplasm shaded by interior depth, a nucleus
/// drifting against the motion. Soft rim light at the membrane.
pub fn amoeba(uv: Vec2, t: f32) -> Vec3 {
    // body: smooth-union of orbiting lobes
    let mut d = 1.0e9f32;
    let mut i = 0u32;
    while i < 5 {
        let fi = i as f32;
        let ang = t * (0.3 + 0.07 * fi) + fi * 2.39996; // golden-angle spread
        let rad = 0.16 + 0.06 * (t * 0.9 + fi * 1.7).sin();
        let center = vec2(ang.cos(), ang.sin()) * (0.13 + 0.05 * fi.sin());
        let lobe = sdf2::circle(uv - center, rad);
        d = sdf2::smooth_union(d, lobe, 0.22);
        i += 1;
    }
    // membrane wobble: perturb the distance field with fine FBM
    let wob = (noise::fbm2(uv * 9.0 + vec2(t * 0.6, -t * 0.4), 4, 21) - 0.5) * 0.04;
    let d = d + wob;

    // nucleus lags behind the body's drift
    let nuc_c = vec2((t * 0.23).cos(), (t * 0.31).sin()) * 0.06;
    let nucleus = sdf2::circle(uv - nuc_c, 0.07 + 0.012 * (t * 1.3).sin());

    let bg = vec3(0.02, 0.03, 0.05) + Vec3::splat(noise::fbm2(uv * 2.0, 3, 5) * 0.03);
    if d > 0.0 {
        // outside: faint membrane glow
        let glow = (-14.0 * d).exp();
        return color::srgb_encode(bg + vec3(0.3, 0.9, 0.6) * glow * 0.35);
    }
    // inside: depth-shaded cytoplasm + organelle speckle + rim
    let depth = (-d).min(0.25) / 0.25;
    let speck = noise::fbm2(uv * 14.0 + vec2(t * 0.2, t * 0.15), 4, 33);
    let cyto = vec3(0.10, 0.45, 0.30) * (0.5 + 0.5 * depth) + vec3(0.05, 0.20, 0.12) * speck;
    let rim = (1.0 - depth).powf(3.0);
    let nuc = (-60.0 * nucleus.max(0.0)).exp();
    let body = cyto + vec3(0.5, 1.0, 0.7) * rim * 0.25 + vec3(0.45, 0.25, 0.50) * nuc;
    color::srgb_encode(color::tonemap_reinhard(body))
}

// --- clouds ---

/// Domain-warped FBM clouds with a cheap directional-light gradient.
pub fn clouds(uv: Vec2, t: f32) -> Vec3 {
    let drift = vec2(t * 0.05, t * 0.01);
    let warp = vec2(
        noise::fbm2(uv * 2.0 + drift, 4, 11),
        noise::fbm2(uv * 2.0 + drift + vec2(5.2, 1.3), 4, 12),
    );
    let density = noise::fbm2(uv * 3.0 + warp * 1.5 + drift, 5, 13);
    let cloud = ((density - 0.42) * 4.0).clamp(0.0, 1.0);
    // light from upper-left: sample density a step toward the light
    let toward = noise::fbm2((uv + vec2(-0.04, 0.04)) * 3.0 + warp * 1.5 + drift, 5, 13);
    let shade = (0.5 + 2.0 * (density - toward)).clamp(0.0, 1.0);
    let sky = vec3(0.25, 0.45, 0.75) - uv.y * vec3(0.15, 0.20, 0.15);
    let cloud_col = vec3(1.0, 0.98, 0.95) * (0.55 + 0.45 * shade);
    let lit = sky * (1.0 - cloud) + cloud_col * cloud;
    color::srgb_encode(color::tonemap_aces(lit))
}

// --- mandelbrot ---

/// Mandelbrot with smooth iteration coloring through the cosine palette,
/// slowly breathing around a seahorse-valley anchor.
pub fn mandelbrot(uv: Vec2, t: f32) -> Vec3 {
    let zoom = 1.6 + 0.9 * (t * 0.21).sin();
    let c = vec2(-0.745, 0.186) + uv * (0.8 / zoom.exp());
    let mut z = Vec2::ZERO;
    let mut esc = -1.0f32;
    let mut i = 0u32;
    while i < 96 {
        z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        let m2 = z.length_squared();
        if m2 > 64.0 {
            // smooth escape count
            esc = i as f32 - m2.ln().ln() * 1.442_695 + 4.0;
            break;
        }
        i += 1;
    }
    if esc < 0.0 {
        return Vec3::ZERO; // interior
    }
    let v = color::palette_rainbow(esc * 0.015 + t * 0.02);
    color::srgb_encode(color::tonemap_aces(v * (0.3 + 0.04 * esc).min(1.2)))
}

// --- end ---

/// Dispatch table for runners and tests; ids are stable.
pub const NAMES: [&str; 4] = ["plasma", "amoeba", "clouds", "mandelbrot"];

pub fn by_index(i: u32, uv: Vec2, t: f32) -> Vec3 {
    match i {
        0 => plasma(uv, t),
        1 => amoeba(uv, t),
        2 => clouds(uv, t),
        _ => mandelbrot(uv, t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_entries_finite_and_displayable() {
        for idx in 0..4u32 {
            for (px, py) in [(0, 0), (13, 7), (31, 17), (63, 35)] {
                let uv = vec2(px as f32 / 64.0 - 0.5, py as f32 / 36.0 - 0.5);
                for tm in [0.0f32, 2.0, 17.3] {
                    let c = by_index(idx, uv, tm);
                    for ch in [c.x, c.y, c.z] {
                        assert!(
                            ch.is_finite() && (0.0..=1.0 + 1e-4).contains(&ch),
                            "entry {idx} uv {uv:?} t {tm}: bad channel {ch}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn deterministic() {
        let uv = vec2(0.1, -0.2);
        for idx in 0..4u32 {
            assert_eq!(by_index(idx, uv, 5.0), by_index(idx, uv, 5.0));
        }
    }
}
