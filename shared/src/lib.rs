//! Kernel math shared verbatim between CPU runners and GPU shaders.
//!
//! std on CPU, `no_std` on SPIR-V. Constraints for the GPU side: no recursion,
//! no checked/overflowing arithmetic, no `usize`/u64 in data, bounded loops only.
#![cfg_attr(target_arch = "spirv", no_std)]

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use glam::{vec3, Vec3};

pub mod collatz;
pub mod matmul;
pub mod mycelia;
pub mod mycelium;
pub mod physarum;
pub mod rng;
pub mod tracer_v2;

pub use collatz::{collatz_steps, MAX_STEPS};

/// Per-dispatch render parameters. u32-only on purpose: identical layout on
/// CPU, SPIR-V, and the WGSL path with zero padding hazards.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RenderParams {
    pub width: u32,
    pub height: u32,
    pub samples: u32,
    pub seed: u32,
}

const MAX_BOUNCES: u32 = 8;
const T_MIN: f32 = 0.001;
const T_MAX: f32 = 1.0e30;

struct Sphere {
    center: Vec3,
    radius: f32,
    albedo: Vec3,
    /// 0 = lambertian, 1 = metal
    material: u32,
    fuzz: f32,
}

/// Fixed scene: ground sphere + three spheres (classic RTiOW chapter-12 vibe).
const NUM_SPHERES: u32 = 4;

fn sphere(i: u32) -> Sphere {
    match i {
        0 => Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            albedo: vec3(0.8, 0.8, 0.0),
            material: 0,
            fuzz: 0.0,
        },
        1 => Sphere {
            center: vec3(0.0, 0.0, -1.2),
            radius: 0.5,
            albedo: vec3(0.1, 0.2, 0.5),
            material: 0,
            fuzz: 0.0,
        },
        2 => Sphere {
            center: vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            albedo: vec3(0.8, 0.8, 0.8),
            material: 1,
            fuzz: 0.05,
        },
        _ => Sphere {
            center: vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            albedo: vec3(0.8, 0.6, 0.2),
            material: 1,
            fuzz: 0.4,
        },
    }
}

pub(crate) fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

/// Branch-free random unit vector via spherical mapping. No rejection loop:
/// data-dependent float branches diverge chaotically between CPU and GPU
/// (fma contraction flips knife-edge comparisons), and GPU schedulers hate
/// divergent loops anyway.
fn random_unit_vector(state: &mut u32) -> Vec3 {
    let z = 1.0 - 2.0 * rng::next_f32(state);
    let phi = 2.0 * core::f32::consts::PI * rng::next_f32(state);
    let r = (1.0 - z * z).max(0.0).sqrt();
    vec3(r * phi.cos(), r * phi.sin(), z)
}

pub(crate) struct Hit {
    t: f32,
    point: Vec3,
    normal: Vec3,
    albedo: Vec3,
    material: u32,
    fuzz: f32,
}

pub(crate) fn hit_scene(origin: Vec3, dir: Vec3) -> Hit {
    let mut closest = Hit {
        t: T_MAX,
        point: Vec3::ZERO,
        normal: Vec3::ZERO,
        albedo: Vec3::ZERO,
        material: 0,
        fuzz: 0.0,
    };
    let mut i = 0u32;
    while i < NUM_SPHERES {
        let s = sphere(i);
        let oc = origin - s.center;
        let a = dir.length_squared();
        let half_b = oc.dot(dir);
        let c = oc.length_squared() - s.radius * s.radius;
        let disc = half_b * half_b - a * c;
        if disc > 0.0 {
            let sqrt_d = disc.sqrt();
            // Nearer root first, fall back to the farther one.
            let mut t = (-half_b - sqrt_d) / a;
            if t < T_MIN {
                t = (-half_b + sqrt_d) / a;
            }
            if t >= T_MIN && t < closest.t {
                let point = origin + t * dir;
                closest = Hit {
                    t,
                    point,
                    normal: (point - s.center) / s.radius,
                    albedo: s.albedo,
                    material: s.material,
                    fuzz: s.fuzz,
                };
            }
        }
        i += 1;
    }
    closest
}

pub(crate) fn sky(dir: Vec3) -> Vec3 {
    let t = 0.5 * (dir.normalize().y + 1.0);
    (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
}

/// Trace one ray iteratively (no recursion on SPIR-V).
fn ray_color(mut origin: Vec3, mut dir: Vec3, state: &mut u32) -> Vec3 {
    let mut throughput = Vec3::ONE;
    let mut bounce = 0u32;
    while bounce < MAX_BOUNCES {
        let hit = hit_scene(origin, dir);
        if hit.t >= T_MAX {
            return throughput * sky(dir);
        }
        // Face-forward the normal so glancing rays don't leak inside.
        let front = dir.dot(hit.normal) < 0.0;
        let normal = if front { hit.normal } else { -hit.normal };
        throughput *= hit.albedo;
        origin = hit.point;
        if hit.material == 0 {
            let mut scatter = normal + random_unit_vector(state);
            if scatter.length_squared() < 1.0e-8 {
                scatter = normal;
            }
            dir = scatter;
        } else {
            let reflected = reflect(dir.normalize(), normal);
            dir = reflected + hit.fuzz * random_unit_vector(state);
            if dir.dot(normal) <= 0.0 {
                return Vec3::ZERO; // absorbed into the surface
            }
        }
        bounce += 1;
    }
    Vec3::ZERO
}

/// Render one pixel: the function that runs identically on CPU, WASM, and GPU.
/// Returns linear RGB (gamma is the presentation layer's job).
pub fn render_pixel(px: u32, py: u32, params: &RenderParams) -> Vec3 {
    let aspect = params.width as f32 / params.height as f32;
    // Pinhole camera at origin, viewport height 2.0, focal length 1.0.
    let viewport_h = 2.0f32;
    let viewport_w = viewport_h * aspect;

    let mut acc = Vec3::ZERO;
    let pixel_index = py * params.width + px;
    let mut s = 0u32;
    while s < params.samples {
        let mut state = rng::seed(pixel_index, s, params.seed);
        let jx = rng::next_f32(&mut state);
        let jy = rng::next_f32(&mut state);
        let u = (px as f32 + jx) / params.width as f32;
        let v = 1.0 - (py as f32 + jy) / params.height as f32;
        let dir = vec3(
            (u - 0.5) * viewport_w,
            (v - 0.5) * viewport_h,
            -1.0,
        );
        acc += ray_color(Vec3::ZERO, dir, &mut state);
        s += 1;
    }
    acc / params.samples as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    const PARAMS: RenderParams = RenderParams {
        width: 64,
        height: 36,
        samples: 4,
        seed: 7,
    };

    #[test]
    fn render_is_deterministic() {
        let a = render_pixel(10, 10, &PARAMS);
        let b = render_pixel(10, 10, &PARAMS);
        assert_eq!(a, b);
    }

    #[test]
    fn output_is_sane() {
        for (px, py) in [(0, 0), (32, 18), (63, 35)] {
            let c = render_pixel(px, py, &PARAMS);
            for v in [c.x, c.y, c.z] {
                assert!(v.is_finite() && (0.0..=1.5).contains(&v), "bad channel {v}");
            }
        }
    }

    #[test]
    fn sky_pixels_are_blueish() {
        // Top-left corner looks at sky: blue channel should dominate red.
        let c = render_pixel(0, 0, &PARAMS);
        assert!(c.z > c.x, "expected sky gradient, got {c:?}");
    }
}
