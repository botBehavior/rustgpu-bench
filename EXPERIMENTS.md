# Experiments — where rustgpu-bench goes next

Self-contained handoff for two larger investigations that grew out of the
[discussion #614](https://github.com/Rust-GPU/rust-gpu/discussions/614) exchange with
rust-gpu maintainer Firestar99 and user nazar-pc. A fresh contributor (human or agent)
should be able to execute either from this doc alone. Last updated 2026-06-10.

---

## Where we are (verified findings, consolidated)

Hardware/versions for our numbers: NVIDIA RTX 5070 Ti, driver 32.0.15.9649, Windows 11,
rust-gpu/spirv-std 0.10.0-alpha.1 (nightly-2026-04-11), wgpu/naga 29.0.3. Timing =
GPU timestamp queries around the compute pass, medians of 30, wall-clock cross-checked.
Full data: `bench-results.json`; methodology: `RESULTS.md`; gap analysis: `ANALYSIS.md`.

**Benchmark (median ms):**

| workload | rustgpu-spv | rustgpu→naga | hand-WGSL |
|---|---|---|---|
| collatz 1M | 0.186 | 0.347 | 0.197 |
| matmul 1024³ | 1.798 | 1.794 | 1.566 |
| matmul `get_unchecked` | **0.696** | 1.668 | — |
| path tracer 800×450@32 | 1.098 | 1.360 | **0.598** |

**Established conclusions:**
- Parity (slight win) on branchy integer code.
- The matmul gap is **bounds checks**: `get_unchecked` → 2.5× on native passthrough,
  beating hand-WGSL 2.1× (WGSL can't opt out). Through naga the win evaporates — wgpu
  re-injects runtime checks.
- The 1.84× tracer gap is **codegen shape, not bloat/math**: rust-gpu emits one flattened
  74-block, 40-OpPhi function (logical SPIR-V can't pass pointers → inline everything);
  naga emits 11 small structured functions. "Software libm" and "instruction bloat"
  REFUTED with opcode counts.
- **The gap is DRIVER-SPECIFIC.** Firestar99 ran our suite on AMD Strix Halo / RADV:
  render rustgpu-spv 1.447 vs hand-WGSL 1.514 — *parity*. NVIDIA digests the Phi-heavy
  form worse than RADV. The structural mechanism stands; its *cost* is driver-dependent.

**Fuzzer findings (`tools/diff-fuzz`, `findings/`):** 24,000 generated pure-integer
functions / ~98M comparisons → **2 confirmed rust-gpu miscompiles**, both reproducing
identically through NVIDIA passthrough AND the naga frontend (two independent SPIR-V
consumers agree with each other and disagree with native rustc):
- `findings/min-module-seed423.rs` (9 nodes): `if 0 < x.wrapping_mul(EVEN_CONST) { y } else { y % (C|1) }`
- `findings/min-module-seed303.rs` (15 nodes): a bounded loop accumulating
  `acc*1664525 + (y + select(y < (C|y), x, select(x < C2, x, y)))`
- **Shared motif:** a `<` comparison whose operand is `wrapping_mul`-by-even or
  `bitwise-or`-with-constant, feeding a select. Smells like a const-prop / range-analysis
  pass folding the comparison to a constant. (Hypothesis — we have not read the passes.)

**#614 feedback to act on:**
- *Latency vs throughput*: our single-dispatch latency measurements barely load the GPU;
  GPUs are throughput machines. Real GPU timing wants Vulkan extensions wgpu doesn't
  expose. **Methodology needs a saturation/throughput variant.**
- At 2048²@32spp (Firestar): render rustgpu-spv 25.2 vs hand 18.4 (1.37×), **render_v2
  18.2 = parity**, and **naga 51.3 ms = 2.04× the passthrough cost**. The naga tax is
  large and consistent at scale on RADV (vs only +24% on our NVIDIA at small scale).
- Toolchain: use `cargo install cargo-gpu@0.10.0-alpha.1` (not the stub); glam trap fixed
  in #613; adopt the **build.rs flow** (rust-gpu-template) instead of manual shader builds;
  cargo-gpu lacks a lockfile (the race is broader than Windows).
- nazar-pc's escape from naga's checks: `create_shader_module_unchecked` +
  `ExperimentalFeatures::enabled()`, or naga `BoundsCheckPolicy::Unchecked`, or
  self-transpile SPIR-V→MSL with checks off. **Directly relevant to Experiment 1.**

---

## Experiment 1 — The naga / WebGPU transpilation tax

**Thesis.** The web is the entire "Rust everywhere" claim. The only rust-gpu→WebGPU path
is SPIR-V→naga→WGSL, and naga roughly **doubles** runtime at scale (Firestar's RADV:
2.04×). If that's an inherent cost, it's a dealbreaker to state honestly; if it's bounds
checks or lost optimization, it's a fixable compiler gap to hand upstream. **Nobody has
isolated which.** That is the highest-leverage open question for the mission.

**STATUS: core question answered 2026-06-10 (commit 89efe97, `naga-tax.md`).** Added the
`rustgpu-naga-unchk` + `hand-wgsl-unchk` control arms. **The naga tax IS removable bounds
checks** — `unchecked()` collapses naga onto passthrough fully (render 1.356→1.091=spv;
matmul_unchecked 1.741→0.701=spv, a 2.48× swing). The rust-gpu↔hand gap *survives*
checks-off: matmul → parity (0.702 vs 0.704), render keeps 1.85× = codegen shape
(driver-specific). Web-path "double-check trap" documented (need `get_unchecked` in Rust
AND `unchecked()` in wgpu). **Remaining:** saturation/throughput sweep (Firestar's core
methodology critique — absolute numbers still single-dispatch); structural WGSL diff;
cross-vendor confirmation on RADV; #614 follow-up (RED).

**What we already know (now confirmed):**
- The naga overhead is entirely wgpu's injected bounds checks; no transpilation penalty on
  NVIDIA once checks are off.
- rust-gpu bakes its own checks into the SPIR-V — wgpu's `unchecked()` can't strip those,
  so `get_unchecked` in the Rust source is also required.

**Method (the matrix):** for each workload (collatz, matmul, tracer + at least one heavy
new one), measure these arms, at small AND saturating sizes:
1. rustgpu-spv passthrough (baseline, no naga, checks per shader)
2. rustgpu-spv passthrough + `get_unchecked` kernels
3. rustgpu→naga (current web path; wgpu re-injects checks)
4. rustgpu→naga + **`create_shader_module_unchecked`** / `ShaderRuntimeChecks::unchecked()`
   (isolates the check tax from the transpilation tax)
5. hand-WGSL through naga (checked)
6. hand-WGSL through naga unchecked

The decomposition we want: **tax = (checks naga injects) + (optimization naga loses vs the
native SPIR-V compiler) + (structure naga can't see through)**. Arms 3 vs 4 isolate the
check component; 4 vs 1 isolates the transpilation/structure component; 4 vs 6 isolates
rust-gpu's WGSL quality vs hand-WGSL.

Plus a **structural study**: diff the naga-emitted WGSL of the rust-gpu tracer vs the hand
tracer (we have `tools/spirv-stats` for SPIR-V; need the WGSL-text equivalent — count
functions, branches, redundant temps, whether naga preserves or re-flattens the CFG).

**Deliverables:**
- A `naga-tax.md` decomposition with numbers, per driver if we get cross-vendor runs.
- A concrete fix direction for upstream (e.g. "naga re-flattens X" or "the cost is purely
  re-injected checks, which `unchecked` recovers — document the escape hatch").
- Updated `RESULTS.md`/`ANALYSIS.md` and a #614 follow-up.

**Unknowns / risks:** `create_shader_module_unchecked` may need a wgpu experimental flag;
naga's WGSL output for our `[repr(C)]` structs may differ from what we assume; some
kernels may not transpile (naga spv-in gaps — wgpu#4449/#6672). Record every failure.

**Start here:** `bench/src/main.rs` already has the arm-and-workload harness; add the
unchecked-naga arm (binding-time `ShaderRuntimeChecks`) and a `--saturate` size sweep.

---

## Experiment 2 — Differential fuzzer → the rust-gpu conformance suite

**Thesis.** We differential-tested a compiler nobody had tested and found 2 real
miscompiles in 24k functions in an afternoon. **What is the actual defect density, and
what classes of bug exist?** Differential testing of a Rust→GPU compiler has never been
published. The deliverable isn't a demo — it's a categorized bug corpus + a reusable
CI-shaped conformance harness, which Firestar explicitly wants (issue #315 asks for test
infra). This is the bigger long-term contribution.

**Current state (`tools/diff-fuzz`):** AST generator for pure `fn(u32,u32)->u32`
(wrapping arith, bit ops, guarded div/rem, masked shifts, select, bounded loops; depth≤5),
emits a regenerated `fuzz-shaders` crate, compiles via cargo-gpu, runs 4096 input pairs/fn
on GPU, diffs bit-exact against a CPU AST interpreter. Native rustc is the final arbiter
(`findings/*.rs` are standalone valid Rust with asserting `main`). `--bisect` does ddmin
over sibling functions + in-context expression shrinking. `DIFF_FUZZ_NAGA=1` routes
through the naga frontend (both findings reproduce there too).

**Expansion axes (roughly in order):**
1. **Wider integer grammar:** function calls (cross-function inlining is where the
   logical-pointer legalizer does its riskiest work), nested structs, `match` on enums,
   early returns, `u64` emulation paths.
2. **Float domain** with ULP-tolerant comparison + a divergence classifier (separate
   "miscompile" from "legitimate fma/transcendental ulp difference"). This is where most
   *new* territory is — float semantics on SPIR-V are subtle.
3. **Scale:** millions of functions, multiple seeds, longer campaigns; track
   defect-density-per-construct (which AST shapes produce miscompiles).
4. **Triage automation:** auto-minimize every finding, cluster by motif (the comparison-
   fold motif is finding #1 — are there others?), produce a categorized corpus.
5. **Coverage of the matrix:** run each finding through passthrough vs naga vs (if
   reachable) qptr pipeline to localize bugs to rust-gpu codegen vs SPIR-T vs naga.

**Deliverables:**
- `conformance/` corpus: minimized repros grouped by bug class, each with the failing
  input and the native-rustc-verified expected value.
- A defect-density writeup ("N functions, M miscompiles across K classes").
- A CI-shaped runner (deterministic seed list, fast subset) offered to rust-gpu #315.
- Upstream issue(s) per confirmed bug class (RED — needs Carter sign-off to file; draft
  exists at `drafts/upstream-miscompile-issue.md`).

**Unknowns / risks:** the interpreter can be wrong (a fuzzer bug masquerading as a
compiler bug) — native rustc arbitration guards this, keep it. Float classification is
genuinely hard; budget for false positives. Large campaigns are compile-bound (~6 s/batch);
the 10-min background-task cap means chunked runs (continue from last seed).

**Start here:** `tools/diff-fuzz/src/main.rs`. Add a float mode behind a flag with a
classifier; widen `gen()` with a function-call node; run a 5000-batch campaign and
auto-bisect every finding.

---

## Lower-priority companions (publish-readiness, not new science)
- **Real workload** (FFT / BVH tracer / NN inference layer) ported to rust-gpu AND
  hand-WGSL — answers "does it hold up on something that matters."
- **Cross-vendor perf map** — the bench already runs on contributors' machines; build
  auto-collection + aggregation (the gap is driver-specific, so the map is the real story).
- **Compile-time / dev-velocity study** — quantify the out-of-tree nightly-pin cost.

## Operational pointers
- Reply draft for #614 (needs refresh + Carter sign-off to post): `drafts/614-reply-1.md`.
- Miscompile issue draft (RED to file): `drafts/upstream-miscompile-issue.md`.
- Build everything: `cargo gpu build --shader-crate shaders --output-dir shaders/spv
  --auto-install-rust-toolchain`; then `cargo run -p bench --release`.
- Sibling project oscilla (github.com/botBehavior/oscilla) consumes `gpu-shader-lib` as a
  git dep — its first cross-repo user; changes to the lib ripple there.
