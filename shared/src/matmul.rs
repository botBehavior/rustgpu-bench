//! Naive dense matmul element kernel — the classic GPU benchmark workload.
//! Deliberately naive (no tiling) so the rust-gpu and hand-WGSL arms are
//! trivially the same algorithm; this benchmarks codegen, not blocking skill.

/// C[row, col] for C = A × B, all square n×n, row-major.
pub fn matmul_element(a: &[f32], b: &[f32], n: u32, row: u32, col: u32) -> f32 {
    let mut acc = 0.0f32;
    let mut k = 0u32;
    while k < n {
        acc += a[(row * n + k) as usize] * b[(k * n + col) as usize];
        k += 1;
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_times_x() {
        let n = 3u32;
        #[rustfmt::skip]
        let a = [1.0, 0.0, 0.0,
                 0.0, 1.0, 0.0,
                 0.0, 0.0, 1.0];
        let b = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        for r in 0..n {
            for c in 0..n {
                assert_eq!(matmul_element(&a, &b, n, r, c), b[(r * n + c) as usize]);
            }
        }
    }

    #[test]
    fn known_product() {
        let a = [1.0, 2.0, 3.0, 4.0]; // [[1,2],[3,4]]
        let b = [5.0, 6.0, 7.0, 8.0]; // [[5,6],[7,8]]
        assert_eq!(matmul_element(&a, &b, 2, 0, 0), 19.0);
        assert_eq!(matmul_element(&a, &b, 2, 0, 1), 22.0);
        assert_eq!(matmul_element(&a, &b, 2, 1, 0), 43.0);
        assert_eq!(matmul_element(&a, &b, 2, 1, 1), 50.0);
    }
}
