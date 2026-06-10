//! M0 bring-up kernel, kept as the simplest cross-target correctness probe.

/// Cap on Collatz iterations so the kernel is provably bounded on the GPU.
pub const MAX_STEPS: u32 = 1000;

/// Number of Collatz steps from `n` down to 1.
///
/// Returns `u32::MAX` for 0 (no trajectory), on exceeding `MAX_STEPS`, or on
/// u32 overflow. No checked_mul: rust-gpu can't lower overflow-detecting
/// intrinsics, so the overflow case is guarded by bound instead.
pub fn collatz_steps(mut n: u32) -> u32 {
    if n == 0 {
        return u32::MAX;
    }
    let mut steps = 0u32;
    while n != 1 {
        if steps >= MAX_STEPS {
            return u32::MAX;
        }
        n = if n % 2 == 0 {
            n / 2
        } else if n > (u32::MAX - 1) / 3 {
            return u32::MAX;
        } else {
            3 * n + 1
        };
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(3), 7);
        assert_eq!(collatz_steps(6), 8);
        assert_eq!(collatz_steps(27), 111);
    }

    #[test]
    fn edge_cases() {
        assert_eq!(collatz_steps(0), u32::MAX);
        assert_eq!(collatz_steps(u32::MAX), u32::MAX);
    }
}
