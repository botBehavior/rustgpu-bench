// Hand-written WGSL twin of gpu_shared::collatz_steps. Same algorithm,
// written fresh from the algorithm description (not transpiled).

@group(0) @binding(0) var<storage, read_write> data: array<u32>;

const MAX_STEPS: u32 = 1000u;

fn collatz_steps(n_in: u32) -> u32 {
    var n = n_in;
    if (n == 0u) {
        return 0xffffffffu;
    }
    var steps = 0u;
    while (n != 1u) {
        if (steps >= MAX_STEPS) {
            return 0xffffffffu;
        }
        if (n % 2u == 0u) {
            n = n / 2u;
        } else if (n > (0xffffffffu - 1u) / 3u) {
            return 0xffffffffu;
        } else {
            n = 3u * n + 1u;
        }
        steps = steps + 1u;
    }
    return steps;
}

@compute @workgroup_size(64)
fn collatz_cs(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
    if (i < arrayLength(&data)) {
        data[i] = collatz_steps(data[i]);
    }
}
