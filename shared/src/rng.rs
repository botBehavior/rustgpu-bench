//! u32-only RNG: stateless seeding via Wang hash + xorshift32 stream.
//! Identical sequences on CPU, WASM, and SPIR-V (no u64, no usize).

/// Wang hash — good avalanche for turning correlated ids into seeds.
pub fn wang_hash(mut x: u32) -> u32 {
    x = (x ^ 61) ^ (x >> 16);
    x = x.wrapping_mul(9);
    x ^= x >> 4;
    x = x.wrapping_mul(0x27d4_eb2d);
    x ^= x >> 15;
    x
}

/// Derive a per-pixel, per-sample stream seed. Never returns 0
/// (xorshift32's fixed point).
pub fn seed(pixel_index: u32, sample: u32, frame_seed: u32) -> u32 {
    let s = wang_hash(pixel_index ^ wang_hash(sample ^ wang_hash(frame_seed)));
    if s == 0 {
        0x9e37_79b9
    } else {
        s
    }
}

/// xorshift32 step.
pub fn next_u32(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

/// Uniform f32 in [0, 1) using the top 24 bits.
pub fn next_f32(state: &mut u32) -> f32 {
    (next_u32(state) >> 8) as f32 * (1.0 / 16_777_216.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_zero_seed() {
        assert_ne!(seed(0, 0, 0), 0);
    }

    #[test]
    fn f32_in_unit_range() {
        let mut state = seed(123, 4, 5);
        for _ in 0..10_000 {
            let v = next_f32(&mut state);
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn streams_differ() {
        assert_ne!(seed(1, 0, 7), seed(2, 0, 7));
        assert_ne!(seed(1, 0, 7), seed(1, 1, 7));
    }
}
