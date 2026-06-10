# Build the web demo: wasm CPU arm + WGSL GPU arm into gpu/web/.
# Run from anywhere. Serve with:  python -m http.server -d gpu/web 8080
$ErrorActionPreference = "Stop"
$gpuDir = Split-Path $PSScriptRoot -Parent

# 1. CPU arm: shared kernel -> wasm32 (raw C ABI, no bindgen)
cargo build -p runner-web --target wasm32-unknown-unknown --release --manifest-path "$gpuDir\Cargo.toml"
Copy-Item "$gpuDir\target\wasm32-unknown-unknown\release\runner_web.wasm" "$PSScriptRoot\runner_web.wasm" -Force

# 2. GPU arm: shared kernel -> SPIR-V (wgsl-constrained) -> WGSL text via naga
cargo gpu build --shader-crate "$gpuDir\shaders" --output-dir "$gpuDir\shaders\wgsl" --target spirv-unknown-naga-wgsl --auto-install-rust-toolchain
naga "$gpuDir\shaders\wgsl\gpu_shaders.spv" "$PSScriptRoot\kernels.wgsl"

Write-Host "web build done -> $PSScriptRoot"
