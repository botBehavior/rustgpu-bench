# Why is the rust-gpu path tracer 1.84× slower than hand-WGSL? (B-phase findings)

2026-06-10. Method: `tools/spirv-stats` (rspirv opcode/CFG statistics), naga-CLI compile of
the hand twin (`naga shaders-wgsl/render.wgsl hand-render.spv`), three controlled
experiments added to the bench (`matmul_unchecked`, `render_v2`, qptr build). All numbers
RTX 5070 Ti, medians of 30, timestamp queries, wall-clock cross-checked.

## Verdicts on the original hypotheses

### ❌ REFUTED: "software libm transcendentals"
rust-gpu's `render_cs` contains exactly **9 ExtInst** ops (GLSL.std.450 — native
sin/cos/sqrt). Math functions lower to native GPU instructions. Not the cause.

### ❌ REFUTED: "verbose codegen / instruction bloat"
Tracer-vs-tracer instruction counts are nearly equal: rust-gpu `render_cs` **466**
instructions vs the hand module's **426** total. The transpiled WGSL *text* is 3× bigger,
but that's formatting, not operations. Not the cause.

### ✅ CONFIRMED (for matmul): slice bounds checks
`matmul_element` with `get_unchecked`: **1.763 → 0.696 ms (2.5×)** on the passthrough arm.
The unchecked rust-gpu kernel also beats hand-WGSL (1.487 ms) by **2.1×** — because the
WGSL arm carries wgpu/naga's own injected clamp-checks that WGSL authors cannot opt out of.
The original "15% matmul gap" was never rust-gpu-vs-WGSL; it was rust-gpu's
branch-style checks being slightly costlier than naga's clamp-style checks.
**Single biggest perf lever found.** Caveat: only meaningful on the native passthrough
path — see the naga-tax finding below.

### ⚠️ SUPPORTED, NOT ISOLATED (for render): control-flow shape
The structural difference is stark:

| | rust-gpu `render_cs` | naga compile of hand twin |
|---|---|---|
| functions | 1 (everything inlined) | 11 (calls preserved) |
| instructions | 466 | 426 total |
| blocks | 74 in one function | 84 across 11 small functions |
| Phi nodes | **40** | ~0 (memory-form locals) |

rust-gpu must inline every function taking a reference (logical SPIR-V forbids pointer
arguments — the maintainer-talk constraint, visible in the artifact), producing one
Phi-heavy flattened mega-function. naga emits small structured functions with
load/store locals. NVIDIA's compiler evidently optimizes the latter better here.

Discriminating experiment attempted: `render_v2` threads RNG state by value (removing
pointer-arg inlining *pressure*). Result: **1.085 ms ≈ baseline 1.098 ms** — no change,
because rust-gpu inlines by policy regardless; the experiment cannot separate "inlining"
from "Phi-flattened form". Render itself has only 7 slice accesses, so bounds checks are
negligible here — the gap is genuinely codegen shape, but isolating *which aspect*
(mega-function vs Phi-form vs block ordering) needs a tool we don't have in-repo
(e.g. spirv-opt function-inlining on the hand module to equalize structure).

### Bonus finding: the naga-frontend tax is wgpu's runtime checks
`matmul_unchecked` through the **naga frontend** arm: 1.668 ms — the 2.5× win evaporates
back to checked-speed. wgpu re-injects bounds checks when it consumes SPIR-V through naga
(its security model; `ShaderRuntimeChecks` defaults on). Consequences:
- On the **web path (WebGPU)** you cannot escape bounds checks, no matter the language.
- The collatz naga-arm penalty (0.186 → 0.347 ms) is the same effect: double-checking
  (rust-gpu's own checks + wgpu's).
- Fair-comparison rule for anyone replicating: passthrough-vs-WGSL comparisons must state
  check policy explicitly.

### qptr pipeline (experimental, opt-in)
`RUSTGPU_CODEGEN_ARGS="--no-infer-storage-classes --spirt-passes=qptr"` **builds and
verifies correct** on all five workloads (module grows 15→26 KB, 615→1152 instructions),
timing unchanged (render 1.079 ms). As expected: qptr targets pointer *semantics*
(more of Rust compiling at all), not performance.

## What this means

1. **Hot-loop indexing dominates**: in rust-gpu today, `get_unchecked` (or restructuring
   to avoid repeated slice indexing) is worth up to 2.5× on native. This is actionable for
   any rust-gpu user right now.
2. **The tracer-class gap (~1.8×) is codegen shape**, not math, not bloat, not checks.
   Plausibly addressable in SPIR-T (block layout / Phi reduction / selective outlining);
   this is the precise, reproducible finding worth handing upstream.
3. **Web-path numbers are check-burdened for everyone** — rust-gpu's relative position on
   WebGPU is better than the native numbers suggest, since hand-WGSL pays the same tax.

## Reproduce

```powershell
naga shaders-wgsl/render.wgsl hand-render.spv
cargo run -p spirv-stats --release -- shaders/spv/gpu_shaders.spv render_cs
cargo run -p spirv-stats --release -- hand-render.spv
cargo run -p bench --release   # includes matmul_unchecked + render_v2 experiments
```
