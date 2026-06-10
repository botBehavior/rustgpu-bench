// Hand-written WGSL twin of gpu_shared::matmul. Naive n×n matmul,
// same algorithm and workgroup size as the rust-gpu arm.

@group(0) @binding(0) var<storage, read> dims: u32;
@group(0) @binding(1) var<storage, read> a: array<f32>;
@group(0) @binding(2) var<storage, read> b: array<f32>;
@group(0) @binding(3) var<storage, read_write> c: array<f32>;

@compute @workgroup_size(16, 16)
fn matmul_cs(@builtin(global_invocation_id) id: vec3<u32>) {
    let n = dims;
    let col = id.x;
    let row = id.y;
    if (row >= n || col >= n) {
        return;
    }
    var acc = 0.0;
    for (var k = 0u; k < n; k = k + 1u) {
        acc = acc + a[row * n + k] * b[k * n + col];
    }
    c[row * n + col] = acc;
}
