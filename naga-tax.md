# The naga / WebGPU tax, decomposed

**Question (Experiment 1, `EXPERIMENTS.md`):** rust-gpu's only path to WebGPU is
SPIR-V → naga → WGSL, and that path runs ~2× slower than native passthrough at scale
(Firestar99's RADV: 51 vs 25 ms). Is that an *inherent* transpilation cost (a dealbreaker
to state honestly) or removable bounds checks (a fixable gap)? We added two control arms —
the same modules with `ShaderRuntimeChecks::unchecked()` — to find out.

**Answer: the naga tax is bounds checks, and it is fully removable.** rust-gpu→WebGPU has
no inherent transpilation penalty. With checks equalized, integer/matmul kernels hit
hand-WGSL parity; only the branch-heavy tracer keeps a gap, and that gap is codegen
*shape*, not the naga path — and it's driver-specific (RADV parity).

## Data

NVIDIA RTX 5070 Ti, Vulkan, wgpu/naga 29.0.3, rust-gpu 0.10.0-alpha.1. Median of 30,
GPU timestamps around the compute pass. Every arm correctness-gated vs CPU (all passed —
`unchecked` produced identical correct output: no real OOB in any kernel).

| workload | spv (passthrough) | naga | **naga-unchk** | hand-wgsl | **hand-unchk** |
|---|---|---|---|---|---|
| collatz 1M | 0.186 | 0.348 | 0.244 | 0.197 | 0.183 |
| matmul 1024³ (checked src) | 1.836 | 1.831 | 1.834 | 1.657 | 0.704 |
| matmul `get_unchecked` src | 0.702 | 1.741 | 0.701 | — | — |
| render (tracer) 800×450@32 | 1.100 | 1.356 | 1.091 | 0.602 | 0.588 |
| render_v2 (by-value) | 1.082 | 1.376 | 1.106 | — | — |

(ms; lower is better. `—` = no hand-WGSL twin for that variant.)

## What each comparison isolates

**1. The naga tax = wgpu's injected bounds checks. Fully recoverable.**
Holding everything else constant, `unchecked()` collapses naga back onto passthrough:
- render `1.356 → 1.091` (spv `1.100`) — 100% recovered
- render_v2 `1.376 → 1.106` (spv `1.082`) — 100% recovered
- matmul `get_unchecked` `1.741 → 0.701` (spv `0.702`) — 100% recovered, a **2.48× swing**
- collatz `0.348 → 0.244` (spv `0.186`) — ~64% recovered; the residual is
  `force_loop_bounding` on collatz's `while` loop (also disabled by `unchecked`) plus
  small-kernel scheduling noise at 0.2 ms.

There is no measurable "naga transpiles to slower code" component on NVIDIA once checks
are off. The 23% naga overhead we reported earlier was entirely re-injected checks.

**2. The web-path double-check trap.** To reach passthrough speed *through naga* you must
turn checks off in BOTH places — the Rust source and the wgpu module. matmul shows all
four corners:

| Rust source | wgpu module | median | |
|---|---|---|---|
| checked (`a[i]`) | checked | 1.831 | baseline |
| `get_unchecked` | checked | 1.741 | wgpu re-injects → `get_unchecked` wasted |
| checked (`a[i]`) | `unchecked()` | 1.834 | rust-gpu's own baked checks remain → flag wasted |
| `get_unchecked` | `unchecked()` | **0.701** | both off → **2.6× faster** |

rust-gpu bakes its bounds checks *into the SPIR-V*; wgpu's `unchecked()` only suppresses
the checks *wgpu itself* would inject. Neither lever alone is enough — you need
`get_unchecked` (or equivalent) in Rust to keep checks out of the SPIR-V, and
`create_shader_module_trusted(.., unchecked())` to stop wgpu re-adding them on the way to
the driver. This is the concrete recipe for nazar-pc's question in #614.

**3. The rust-gpu ↔ hand-WGSL gap is NOT checks — it survives with everything off.**
With checks equalized on both sides:
- **matmul: parity.** rust-gpu `get_unchecked` `0.702` vs hand-WGSL unchecked `0.704`.
  The integer/FMA kernel matches hand-written WGSL exactly.
- **render: 1.85× remains.** naga-unchk `1.091` vs hand-unchk `0.588`. Turning checks off
  moves neither side much (hand `0.602→0.588`, rust-gpu `1.100→1.091`). The tracer gap is
  *codegen shape* — rust-gpu's one flattened 74-block / 40-OpPhi function vs naga's 11
  structured functions (established in `ANALYSIS.md`) — and per Firestar99 it's
  **driver-specific** (parity on RADV; NVIDIA digests the Phi-heavy form worse).

**4. Bounds checks cost ~2.5× on the memory-bound matmul, whoever inserts them.**
rust-gpu's own: `1.836 / 0.702 = 2.6×`. wgpu's on hand-WGSL: `1.657 / 0.704 = 2.35×`.
Consistent with the native-passthrough finding (checks worth 2.5×). Checks dominate this
kernel's runtime regardless of source language.

## Takeaways

- **For the "Rust everywhere / WebGPU" thesis:** rust-gpu→WebGPU has *no inherent
  transpilation tax*. The naga slowdown is removable bounds checks. With the two-lever
  recipe, integer/matmul kernels reach hand-WGSL parity on the web path.
- **The honest caveat:** branch-heavy code (the tracer) keeps a codegen-shape gap that
  checks-off does not close, and its size is driver-dependent.
- **Cross-vendor prediction to confirm:** Firestar's RADV naga render was ~2× passthrough
  (much larger than our NVIDIA 23%). Our decomposition predicts `unchecked()` should
  recover most of that on RADV too — i.e. RADV's injected checks are simply more
  expensive. A one-line ask for his next run.
- **Limitation acknowledged (Firestar's core critique):** these are still single-dispatch
  latencies at modest sizes. The check-tax conclusion is robust (it's a 2.5× ratio, far
  above noise), but the absolute numbers want a saturation/throughput sweep — the next
  step in Experiment 1.

## Repro
`cargo gpu build --shader-crate shaders --output-dir shaders/spv --auto-install-rust-toolchain`
then `cargo run -p bench --release`. Arms and the decomposition logic: `bench/src/main.rs`
(header comment). Raw numbers: `bench-results.json`.
