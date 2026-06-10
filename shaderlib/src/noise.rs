//! Hash-based procedural noise: u32 hashing, 2D value noise, bounded FBM.
//! Deterministic across CPU / WASM / SPIR-V (integer hashing, no trig seeds).

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use glam::{vec2, Vec2};

/// 2D lattice point -> [0,1) hash (Wang-style avalanche on packed coords).
pub fn hash2(x: i32, y: i32, seed: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343)
        ^ (y as u32).wrapping_mul(0xd816_3841)
        ^ seed.wrapping_mul(0x9e37_79b9);
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb_352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846c_a68b);
    h ^= h >> 16;
    (h >> 8) as f32 * (1.0 / 16_777_216.0)
}

fn smoothstep01(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// 2D value noise in [0,1), C1-continuous (smoothstep interpolation).
pub fn value2(p: Vec2, seed: u32) -> f32 {
    let xf = p.x.floor();
    let yf = p.y.floor();
    let xi = xf as i32;
    let yi = yf as i32;
    let tx = smoothstep01(p.x - xf);
    let ty = smoothstep01(p.y - yf);
    let a = hash2(xi, yi, seed);
    let b = hash2(xi + 1, yi, seed);
    let c = hash2(xi, yi + 1, seed);
    let d = hash2(xi + 1, yi + 1, seed);
    let top = a + (b - a) * tx;
    let bot = c + (d - c) * tx;
    top + (bot - top) * ty
}

/// Fractal Brownian motion: `octaves` capped at 8 (bounded loops on GPU).
/// Output normalized to ~[0,1).
pub fn fbm2(p: Vec2, octaves: u32, seed: u32) -> f32 {
    let n = octaves.min(8).max(1);
    let mut acc = 0.0f32;
    let mut amp = 0.5f32;
    let mut total = 0.0f32;
    let mut q = p;
    let mut i = 0u32;
    while i < n {
        acc += value2(q, seed.wrapping_add(i)) * amp;
        total += amp;
        amp *= 0.5;
        q = vec2(
            q.x * 2.03 + 17.13, // offset decorrelates octaves on the lattice
            q.y * 2.01 - 9.71,
        );
        i += 1;
    }
    acc / total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_in_unit_range_and_deterministic() {
        for x in -50..50 {
            let v = hash2(x, -x * 3, 7);
            assert!((0.0..1.0).contains(&v));
        }
        assert_eq!(hash2(5, 9, 1), hash2(5, 9, 1));
        assert_ne!(hash2(5, 9, 1), hash2(5, 9, 2));
    }

    #[test]
    fn value_noise_matches_lattice_at_integers() {
        // at integer coordinates, value2 == hash of that lattice point
        for (x, y) in [(0, 0), (3, -2), (-7, 11)] {
            let v = value2(vec2(x as f32, y as f32), 42);
            assert!((v - hash2(x, y, 42)).abs() < 1e-6);
        }
    }

    #[test]
    fn value_noise_continuous_across_cell_edge() {
        let eps = 1e-4f32;
        let a = value2(vec2(1.0 - eps, 0.5), 3);
        let b = value2(vec2(1.0 + eps, 0.5), 3);
        assert!((a - b).abs() < 1e-2, "discontinuity at cell edge: {a} vs {b}");
    }

    #[test]
    fn fbm_in_range_and_rougher_with_octaves() {
        let mut vals1 = 0.0;
        let mut vals6 = 0.0;
        for i in 0..100 {
            let p = vec2(i as f32 * 0.37, i as f32 * 0.61);
            let v1 = fbm2(p, 1, 9);
            let v6 = fbm2(p, 6, 9);
            assert!((0.0..=1.0).contains(&v1));
            assert!((0.0..=1.0).contains(&v6));
            // crude roughness proxy: accumulate |delta| between neighbors
            let q = p + vec2(0.01, 0.0);
            vals1 += (fbm2(q, 1, 9) - v1).abs();
            vals6 += (fbm2(q, 6, 9) - v6).abs();
        }
        assert!(vals6 > vals1, "more octaves should add high-frequency detail");
    }
}
