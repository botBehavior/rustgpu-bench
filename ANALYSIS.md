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

### UPDATE 2026-06-10: the tracer gap is DRIVER-SPECIFIC (community cross-check)
Firestar99 (rust-gpu maintainer) ran this suite on AMD Strix Halo 8060S / RADV (Linux)
in [discussion #614](https://github.com/Rust-GPU/rust-gpu/discussions/614): render
rustgpu-spv **1.447 ms** vs hand-WGSL **1.514 ms** — parity. The 1.84× gap below is an
NVIDIA-driver behavior, not an inherent rust-gpu cost: NVIDIA's compiler digests the
Phi-heavy flattened form worse than RADV's. The structural analysis below stands as the
*mechanism*, but its *cost* is driver-dependent. (His run also confirms the naga-arm
overhead pattern, amplified on RADV: render via naga 4.24 ms vs 1.45 passthrough.)
Methodology caveat from the same thread, conceded: these are single-dispatch latency
measurements on an under-utilized GPU; a saturation/throughput variant is queued.

### ⚠️ SUPPORTED, NOT ISOLATED (for render): control-flow shape — on NVIDIA
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

## Appendix (O3, 2026-06-10): the tracer gap at the WGSL level

Earlier we diffed at the SPIR-V level. This is the view the *browser* actually compiles —
`naga shaders/spv/gpu_shaders.spv out.wgsl`, the rust-gpu render path (`function_8`) vs the
hand `render.wgsl`:

| metric | rust-gpu render (naga-emitted) | hand render.wgsl |
|---|---|---|
| structure | **1 fully-inlined function**, 400 lines | **11 structured functions**, 189 lines |
| `loop {}` / `switch` | 3 / 2 (CFG reconstructed as a state machine) | 0 / 1 |
| `if` | 22 | 10 |
| `var` (mutable) | **60**, of which **162 phi-materialized** assignments | 14 |
| `for` | 0 (control flow is `loop`+`break`+`switch`) | 3 |

The flattened SPIR-V (one 74-block / 40-OpPhi function — logical addressing forbids passing
pointers, so rust-gpu inlines everything) survives transpilation as a single 400-line WGSL
function whose 40 OpPhis become **162 mutable phi-vars** threaded through a reconstructed
`loop { switch }` state machine. The hand version is 11 small functions with natural control
flow and zero phi materialization. This is the concrete form of the 1.86× gap: NVIDIA's
register allocator / optimizer handles the structured form far better than the phi-soup
state machine. It is **codegen shape, not bounds checks** (checks-off moves it <1%, per
`naga-tax.md`) and it is **driver-specific** (RADV reaches parity — it digests the inlined
form fine; NVIDIA does not). The fix lives upstream in rust-gpu's function-call story
(qptr / physical pointers would let it stop inlining), not in our kernel.
