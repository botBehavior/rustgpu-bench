//! B3 experiment: identical tracer, but RNG state threaded BY VALUE instead of
//! `&mut u32`. Pointer-typed arguments force rust-gpu to inline every callee
//! (logical SPIR-V can't pass pointers); by-value threading removes that
//! pressure. If this changes the emitted structure/timing vs the v1 tracer,
//! the inline-flattening hypothesis for the 1.84× gap gains direct evidence.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use crate::rng;
use crate::RenderParams;
use glam::{vec3, Vec3};

const MAX_BOUNCES: u32 = 8;
const T_MAX: f32 = 1.0e30;

fn next_f32_v(state: u32) -> (u32, f32) {
    let mut s = state;
    let v = rng::next_f32(&mut s); // local &mut only — never crosses a call boundary
    (s, v)
}

fn random_unit_vector_v(state: u32) -> (u32, Vec3) {
    let (s1, a) = next_f32_v(state);
    let (s2, b) = next_f32_v(s1);
    let z = 1.0 - 2.0 * a;
    let phi = 2.0 * core::f32::consts::PI * b;
    let r = (1.0 - z * z).max(0.0).sqrt();
    (s2, vec3(r * phi.cos(), r * phi.sin(), z))
}

fn ray_color_v(origin_in: Vec3, dir_in: Vec3, state_in: u32) -> (u32, Vec3) {
    let mut origin = origin_in;
    let mut dir = dir_in;
    let mut state = state_in;
    let mut throughput = Vec3::ONE;
    let mut bounce = 0u32;
    while bounce < MAX_BOUNCES {
        let hit = crate::hit_scene(origin, dir);
        if hit.t >= T_MAX {
            return (state, throughput * crate::sky(dir));
        }
        let front = dir.dot(hit.normal) < 0.0;
        let normal = if front { hit.normal } else { -hit.normal };
        throughput *= hit.albedo;
        origin = hit.point;
        if hit.material == 0 {
            let (s, rv) = random_unit_vector_v(state);
            state = s;
            let mut scatter = normal + rv;
            if scatter.length_squared() < 1.0e-8 {
                scatter = normal;
            }
            dir = scatter;
        } else {
            let reflected = crate::reflect(dir.normalize(), normal);
            let (s, rv) = random_unit_vector_v(state);
            state = s;
            dir = reflected + hit.fuzz * rv;
            if dir.dot(normal) <= 0.0 {
                return (state, Vec3::ZERO);
            }
        }
        bounce += 1;
    }
    (state, Vec3::ZERO)
}

/// Same pixel function as v1, by-value RNG threading throughout.
pub fn render_pixel_v2(px: u32, py: u32, params: &RenderParams) -> Vec3 {
    let aspect = params.width as f32 / params.height as f32;
    let viewport_h = 2.0f32;
    let viewport_w = viewport_h * aspect;

    let mut acc = Vec3::ZERO;
    let pixel_index = py * params.width + px;
    let mut s = 0u32;
    while s < params.samples {
        let state0 = rng::seed(pixel_index, s, params.seed);
        let (s1, jx) = next_f32_v(state0);
        let (s2, jy) = next_f32_v(s1);
        let u = (px as f32 + jx) / params.width as f32;
        let v = 1.0 - (py as f32 + jy) / params.height as f32;
        let dir = vec3((u - 0.5) * viewport_w, (v - 0.5) * viewport_h, -1.0);
        let (_, c) = ray_color_v(Vec3::ZERO, dir, s2);
        acc += c;
        s += 1;
    }
    acc / params.samples as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_matches_v1_exactly() {
        let params = RenderParams {
            width: 64,
            height: 36,
            samples: 4,
            seed: 7,
        };
        for (px, py) in [(0, 0), (10, 10), (32, 18), (63, 35)] {
            assert_eq!(
                render_pixel_v2(px, py, &params),
                crate::render_pixel(px, py, &params),
                "v2 must be semantically identical to v1"
            );
        }
    }
}
