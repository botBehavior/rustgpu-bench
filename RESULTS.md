# One Rust function, four targets — demo + first published rust-gpu vs hand-WGSL benchmark

2026-06-10 · RTX 5070 Ti (driver 32.0.15.9649) · Windows 11 · rust-gpu **v0.10.0-alpha.1**
(nightly-2026-04-11) · wgpu/naga **29.0.3** · stable rustc 1.93.1 for all CPU/WASM arms

## What this is

`shared/src/lib.rs` contains a path tracer (`render_pixel`), a Collatz kernel, and a naive
matmul — ordinary `no_std`-compatible Rust. That **one source** runs, verified, on:

1. **Native CPU** — stable rustc + rayon (`runner-cpu`)
2. **WASM in the browser** — wasm32, raw C ABI, no bindgen, 31 KB module (`runner-web`)
3. **Native GPU** — rust-gpu → SPIR-V → Vulkan via wgpu (`runner-native`)
4. **Browser GPU** — rust-gpu → SPIR-V → naga → WGSL → WebGPU (`web/`)

No shader language was written for the demo. (Hand-WGSL exists only as the benchmark
comparison arm.)

## Correctness (the part that makes the benchmark meaningful)

- Collatz: **1,048,576 / 1,048,576 outputs bit-exact** GPU vs CPU, every arm.
- Matmul: 1000 sampled elements, worst relative error 5.1e-6 (f32 reassociation), every arm.
- Render: mean |diff| 9.3e-5 vs CPU oracle, every arm. Bitwise identity is impossible for a
  chaotic path tracer (GPU sin/cos/fma differ by ulps; ~0.2% of channels are single-sample
  branch flips at 8 spp) — the criterion is statistical: mean < 1e-3 and outliers < 1%.
- Browser run (headless Chrome, WebGPU): center-pixel parity within ±1/255 of native.

## Benchmark: rust-gpu vs hand-written WGSL

Same algorithms, same workgroup sizes, same buffers. Hand-WGSL written fresh, idiomatic, not
transpiled. GPU-side timestamp queries around the compute pass only; 30 runs after 3 warmup;
medians. Independently cross-checked by amortized wall-clock (10 passes/submission) — all
ratios reproduce; the cross-check also caught a destructive-input bug in its own first
version, which is why it exists.

| workload | rustgpu-spv (passthrough) | rustgpu→naga (WGSL-path proxy) | hand-WGSL | gap (spv vs hand) |
|---|---|---|---|---|
| collatz, 1M elems | **0.186 ms** | 0.347 ms | 0.197 ms | **rust-gpu 6% faster** |
| matmul, 1024³ | 1.798 ms | 1.794 ms | **1.566 ms** | hand 15% faster |
| path tracer, 800×450@32 | 1.098 ms | 1.360 ms | **0.598 ms** | hand 1.84× faster |

(p25/p75 and the wall-clock cross-checks are in `bench-results.json`.)

### Reading the numbers honestly

- **Branchy integer code: parity.** rust-gpu actually edges out hand-WGSL on Collatz. The
  "equivalent SPIR-V → equivalent speed" argument from the maintainer talk holds here.
- **Matmul: ~15% — CONFIRMED as bounds checks** (see ANALYSIS.md). With `get_unchecked`,
  rust-gpu drops 1.763 → **0.696 ms**: 2.5× faster than its checked self and 2.1× faster
  than hand-WGSL (which carries wgpu's non-optional clamp checks). The biggest perf lever
  in the whole study.
- **Path tracer: 1.84× — codegen *shape*, not bloat.** Instruction counts are near-equal
  (466 vs 426); the difference is structure: rust-gpu emits one flattened 74-block,
  40-Phi mega-function (logical SPIR-V forbids pointer args → inline everything), naga
  emits 11 small structured functions. "Software transcendentals" and "verbose codegen"
  hypotheses are REFUTED (native ExtInst; near-equal counts). Supported-but-not-isolated;
  full evidence chain in ANALYSIS.md. Tractable compiler work, not an architecture wall —
  and now measured + characterized, which nobody had published before.
- **The naga frontend costs extra** on short kernels (collatz 0.186 → 0.347 ms): wgpu
  re-injects runtime bounds checks when consuming SPIR-V through naga (its security
  model) — confirmed by the unchecked-matmul arm losing its entire 2.5× win on the naga
  path. On the web every language pays this tax equally; budget for it.
- **Module creation:** passthrough ≈ 0 ms, naga arms ≈ 1 ms per module. Negligible at app
  scale, but the naga step isn't free at load time either.

## Demo timings (same scene, 800×450)

| target | config | time |
|---|---|---|
| Browser GPU (WebGPU, Chrome headless) | 8 spp, incl. first-run pipeline compile | 374 ms |
| Native GPU (Vulkan passthrough) | 32 spp, kernel only | **1.1 ms** |
| Native CPU (16 threads, rayon) | 32 spp | 205 ms |
| Browser WASM (1 thread, no bindgen) | 8 spp | ~1.1 s |

Native GPU vs native 16-thread CPU: **~190×**. Vs single-thread WASM: roughly three orders
of magnitude. That's the whole pitch in one table: the same function, and you choose the
hardware.

## Reproduce

```powershell
cargo install --locked --git https://github.com/Rust-GPU/rust-gpu cargo-gpu  # NOT crates.io (stub!)
rustup set auto-self-update disable
cargo gpu build --shader-crate gpu/shaders --output-dir gpu/shaders/spv --auto-install-rust-toolchain
cd gpu
cargo test -p gpu-shared                    # CPU truth
cargo run -p runner-cpu --release           # CPU render -> out-cpu.ppm
cargo run -p runner-native --release        # GPU verify (add --naga for naga path)
cargo run -p bench --release                # the benchmark -> bench-results.json
.\web\build.ps1                             # web demo -> gpu/web/
python -m http.server 8123 -d web           # then open http://localhost:8123
```

Toolchain gotchas (all hit for real, all documented with fixes in
`../research/rust-gpu-kernel-cheatsheet.md`): crates.io cargo-gpu is a fake stub; glam must
be lockfile-unified to 0.30.x; no `checked_*` arithmetic on SPIR-V; rustup self-update race
on first install; wgpu 29 API drift.

## Caveats

- One GPU, one driver, one OS. Alpha toolchain, pinned everything (see versions above).
- Hand-WGSL twins are idiomatic but not heroically optimized; neither is the Rust. The
  comparison measures *codegen*, not optimization effort.
- `rustgpu-naga` is a proxy for the WebGPU path measured on the Vulkan backend; in-browser
  absolute numbers will differ (browser WebGPU adds its own validation).
- Render CPU/GPU comparison is statistical by necessity (documented above).
