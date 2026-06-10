//! Color: HSV, sRGB transfer, tonemaps, IQ cosine palettes.
//! All functions take/return linear-light RGB in `Vec3` unless noted.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use glam::{vec3, Vec3};

/// HSV (h in turns [0,1), s,v in [0,1]) -> RGB.
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Vec3 {
    let h6 = (h - h.floor()) * 6.0;
    let c = v * s;
    let x = c * (1.0 - ((h6 % 2.0) - 1.0).abs());
    let (r, g, b) = if h6 < 1.0 {
        (c, x, 0.0)
    } else if h6 < 2.0 {
        (x, c, 0.0)
    } else if h6 < 3.0 {
        (0.0, c, x)
    } else if h6 < 4.0 {
        (0.0, x, c)
    } else if h6 < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let m = v - c;
    vec3(r + m, g + m, b + m)
}

fn srgb_encode_ch(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn srgb_decode_ch(c: f32) -> f32 {
    if c <= 0.040_45 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Linear -> sRGB (display encoding).
pub fn srgb_encode(c: Vec3) -> Vec3 {
    vec3(srgb_encode_ch(c.x), srgb_encode_ch(c.y), srgb_encode_ch(c.z))
}

/// sRGB -> linear.
pub fn srgb_decode(c: Vec3) -> Vec3 {
    vec3(srgb_decode_ch(c.x), srgb_decode_ch(c.y), srgb_decode_ch(c.z))
}

/// Reinhard tonemap (simple, hue-preserving enough for demos).
pub fn tonemap_reinhard(c: Vec3) -> Vec3 {
    c / (Vec3::ONE + c)
}

/// Narkowicz ACES-fit approximation.
pub fn tonemap_aces(c: Vec3) -> Vec3 {
    let a = 2.51;
    let b = 0.03;
    let cc = 2.43;
    let d = 0.59;
    let e = 0.14;
    ((c * (a * c + b)) / (c * (cc * c + d) + e)).clamp(Vec3::ZERO, Vec3::ONE)
}

/// Inigo Quilez cosine palette: `a + b * cos(TAU * (c*t + d))`.
pub fn palette(t: f32, a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Vec3 {
    const TAU: f32 = core::f32::consts::TAU;
    let arg = (c * t + d) * TAU;
    a + b * vec3(arg.x.cos(), arg.y.cos(), arg.z.cos())
}

/// The classic IQ rainbow-ish default palette.
pub fn palette_rainbow(t: f32) -> Vec3 {
    palette(
        t,
        vec3(0.5, 0.5, 0.5),
        vec3(0.5, 0.5, 0.5),
        vec3(1.0, 1.0, 1.0),
        vec3(0.0, 0.33, 0.67),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_primaries() {
        assert!((hsv_to_rgb(0.0, 1.0, 1.0) - vec3(1.0, 0.0, 0.0)).length() < 1e-6);
        assert!((hsv_to_rgb(1.0 / 3.0, 1.0, 1.0) - vec3(0.0, 1.0, 0.0)).length() < 1e-5);
        assert!((hsv_to_rgb(2.0 / 3.0, 1.0, 1.0) - vec3(0.0, 0.0, 1.0)).length() < 1e-5);
        // zero saturation = gray at v
        assert!((hsv_to_rgb(0.42, 0.0, 0.7) - vec3(0.7, 0.7, 0.7)).length() < 1e-6);
    }

    #[test]
    fn srgb_roundtrip() {
        for i in 0..=20 {
            let v = i as f32 / 20.0;
            let c = vec3(v, v * 0.5, 1.0 - v);
            assert!((srgb_decode(srgb_encode(c)) - c).length() < 1e-5);
        }
    }

    #[test]
    fn tonemaps_bounded_and_monotone() {
        let mut last_r = -1.0f32;
        for i in 0..50 {
            let x = i as f32 * 0.4;
            let r = tonemap_reinhard(vec3(x, x, x)).x;
            let a = tonemap_aces(vec3(x, x, x)).x;
            assert!((0.0..=1.0).contains(&r) && (0.0..=1.0).contains(&a));
            assert!(r >= last_r);
            last_r = r;
        }
    }

    #[test]
    fn palette_periodic() {
        let p0 = palette_rainbow(0.25);
        let p1 = palette_rainbow(1.25);
        assert!((p0 - p1).length() < 1e-5, "cosine palette must be 1-periodic");
    }
}
