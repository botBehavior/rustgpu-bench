//! 2D signed distance functions and combinators.
//! Conventions: negative inside, positive outside, distances in the same
//! units as the input point.

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

use glam::{vec2, Vec2};

/// Distance to a circle of radius `r` centered at origin.
pub fn circle(p: Vec2, r: f32) -> f32 {
    p.length() - r
}

/// Distance to an axis-aligned box with half-extents `b`, rounded by `r`.
pub fn rounded_box(p: Vec2, b: Vec2, r: f32) -> f32 {
    let q = p.abs() - b;
    q.max(Vec2::ZERO).length() + q.x.max(q.y).min(0.0) - r
}

/// Distance to the line segment `a`–`b`.
pub fn segment(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = (pa.dot(ba) / ba.length_squared()).clamp(0.0, 1.0);
    (pa - ba * h).length()
}

/// Hard union.
pub fn union(d1: f32, d2: f32) -> f32 {
    d1.min(d2)
}

/// Subtract shape 2 from shape 1.
pub fn subtract(d1: f32, d2: f32) -> f32 {
    d1.max(-d2)
}

/// Polynomial smooth union (IQ); `k` is the blend radius.
pub fn smooth_union(d1: f32, d2: f32, k: f32) -> f32 {
    let h = (0.5 + 0.5 * (d2 - d1) / k).clamp(0.0, 1.0);
    // lerp(d2, d1, h) - k*h*(1-h)
    d2 + (d1 - d2) * h - k * h * (1.0 - h)
}

/// Cheap normal-ish gradient of any SDF via central differences.
/// `f` is monomorphized — works on GPU (no function pointers involved).
pub fn gradient(f: impl Fn(Vec2) -> f32, p: Vec2, eps: f32) -> Vec2 {
    vec2(
        f(p + vec2(eps, 0.0)) - f(p - vec2(eps, 0.0)),
        f(p + vec2(0.0, eps)) - f(p - vec2(0.0, eps)),
    ) / (2.0 * eps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_known_values() {
        assert_eq!(circle(vec2(0.0, 0.0), 1.0), -1.0);
        assert_eq!(circle(vec2(1.0, 0.0), 1.0), 0.0);
        assert_eq!(circle(vec2(2.0, 0.0), 1.0), 1.0);
    }

    #[test]
    fn box_inside_outside() {
        let b = vec2(1.0, 0.5);
        assert!(rounded_box(vec2(0.0, 0.0), b, 0.0) < 0.0);
        assert_eq!(rounded_box(vec2(2.0, 0.0), b, 0.0), 1.0);
        // rounding shrinks distance by r everywhere
        let d0 = rounded_box(vec2(2.0, 0.0), b, 0.0);
        let d1 = rounded_box(vec2(2.0, 0.0), b, 0.1);
        assert!((d0 - d1 - 0.1).abs() < 1e-6);
    }

    #[test]
    fn segment_endpoints_and_middle() {
        let a = vec2(0.0, 0.0);
        let b = vec2(2.0, 0.0);
        assert_eq!(segment(vec2(0.0, 1.0), a, b), 1.0);
        assert_eq!(segment(vec2(2.0, 1.0), a, b), 1.0);
        assert_eq!(segment(vec2(1.0, 1.0), a, b), 1.0);
        assert_eq!(segment(vec2(-1.0, 0.0), a, b), 1.0); // beyond endpoint
    }

    #[test]
    fn smooth_union_bounds() {
        // smooth union ≤ hard union, equal far from the blend zone
        for i in -20..20 {
            let d1 = i as f32 * 0.1;
            let d2 = 0.7 - d1;
            let s = smooth_union(d1, d2, 0.3);
            assert!(s <= d1.min(d2) + 1e-6);
        }
        assert!((smooth_union(5.0, 0.2, 0.1) - 0.2).abs() < 1e-6);
    }

    #[test]
    fn gradient_of_circle_is_radial() {
        let g = gradient(|p| circle(p, 1.0), vec2(3.0, 4.0), 1e-3);
        let expected = vec2(0.6, 0.8);
        assert!((g - expected).length() < 1e-3);
    }
}
