# Differential GPU-stack conformance — notes

> **CORRECTION (2026-06-10).** An earlier version of this file claimed "2 confirmed
> **rust-gpu** miscompiles (comparison-fold motif), triple-confirmed." **That attribution
> was wrong and is retracted.** On re-verification with an independent harness and a second
> GPU backend: one of the two did not reproduce standalone at all, and the other reproduces
> from **hand-written WGSL that never touches rust-gpu**, and is **correct on a different
> backend (WARP) and on the CPU**. The fault is in the **NVIDIA Vulkan driver**, not in
> rust-gpu. Our prior "two independent SPIR-V consumers" (passthrough + naga) were not
> independent — they share the one NVIDIA driver. **No rust-gpu miscompile was found at this
> scale.** Details below; methodology lesson in **Attribution** at the end.

`tools/diff-fuzz` generates pure Rust functions, compiles them through cargo-gpu to SPIR-V,
runs them on a GPU, and diffs the result against a CPU interpreter of the same AST, with
native rustc as arbiter. It is best understood as a **differential GPU-stack tester**: when
it flags a divergence, the culprit can be anywhere in `rust-gpu → SPIR-V → {naga →} driver`,
and isolating *which* requires re-testing across backends (see Attribution). `tools/diff-fuzz/src/bin/repro.rs`
is the cross-backend verifier built to do that isolation.

## Finding 1 — NVIDIA Vulkan driver miscompile of a nested select (NOT rust-gpu)

A wrong runtime value (not an ICE). Minimal trigger, isolated 2026-06-10:

```rust
fn f(x: u32, y: u32) -> u32 {
    if y < (745348641u32 | y) { x } else { if x < 3850752092u32 { x } else { y } }
}
```

The outer guard `y < (C | y)` is provably true whenever `y` lacks a bit of `C` (almost
always), so the function returns `x` for ~all inputs. On an **NVIDIA RTX 5070 Ti (Vulkan,
driver 32.0.15.9649)** the nested select instead returns `y` (the innermost else) — e.g.
`f(0,1)` returns `1`, should be `0`.

Cross-backend isolation (`repro control`), identical WGSL, same harness:

| backend | `f(0,1) f(2,3) f(100,7)` | verdict |
|---|---|---|
| native rustc (CPU) | `0, 2, 100` | ground truth |
| Python (CPU) | `0, 2, 100` | confirms |
| **Microsoft Basic Render Driver (WARP, DX12)** | `0, 2, 100` | **correct** |
| **NVIDIA RTX 5070 Ti (Vulkan)** | `1, 3, 7` | **wrong** |

Diagnostics prove the inputs and *each comparison in isolation* are correct on NVIDIA
(`y<(C|y)` and `x<D` both evaluate true); only the **nested** select misbehaves, and only on
this driver. It reproduces from rust-gpu's SPIR-V **and** from hand-WGSL→naga SPIR-V — i.e.
any SPIR-V with this control-flow shape — so it is the **NVIDIA Vulkan driver's** shader
compiler, downstream of rust-gpu. The single-select form (`if y<(C|y) {x} else {y}`) is fine;
the bug needs the nested `if A {x} else {if B {x} else {y}}` shape with the `y<(C|y)` guard.
This is a real, minimal, cross-backend-confirmed **driver** bug — reportable to NVIDIA / via
wgpu, **not** to rust-gpu.

The earlier `findings/min-module-seed303.rs` is the looping ancestor of this (the loop is
incidental; the nested select is the whole bug). `findings/min-module-seed423.rs`
(`0 < x.wrapping_mul(EVEN)`) **did not reproduce** in standalone direct-input form across
20k+ inputs including every predicted edge case — withdrawn.

## Finding 2 — pathological inlining on deep call chains (rust-gpu, robustness, not a miscompile)

This one *is* rust-gpu, and it's real. A batch of 48 functions where any may call any earlier
one **hangs the compiler** (rustc killed, exit 143) before emitting SPIR-V. Structural and
expected: logical-addressing SPIR-V cannot pass pointers, so rust-gpu's legalizer must
**inline every call** (confirmed by rust-gpu's own `linker/inline.rs` doc: "inline all
functions that take these 'illegal' pointers, then run mem2reg"); an unrestricted call DAG of
depth ~48 inlines to an exponential blowup. A compile-time robustness cliff (deep helper-call
nests in real shaders hit it too), not a correctness bug. The fuzzer restricts callees to a
small leaf set to stay polynomial.

## Float method (`--float`) — no findings, but the classifier is the keeper

Floats can't be bit-exact-compared: fma-contraction, division rounding, and cancellation make
the GPU legitimately differ. A per-input ULP threshold of 64 flagged **32 false positives in
2 batches** (ordinary rounding). The working classifier is **per-function and statistical** —
flag a function only when **>25%** of its finite inputs exceed **1% relative** error (a real
miscompile diverges on ~all inputs; cancellation is sparse). Transcendentals excluded
(driver-defined precision); non-finite results skipped. Calibrated: worst legitimate function
diverged on 16.3% of inputs, under the gate. 3.9M comparisons, 0 flags.

## Coverage

| grammar | functions | comparisons | rust-gpu miscompiles | notes |
|---|---|---|---|---|
| integer/bool/shift/div/select/loop | ~24,000 | ~98M | **0** | 1 NVIDIA-driver divergence surfaced (Finding 1) |
| + cross-function calls (leaf callees) | 1,440 | 5.9M | 0 | call lowering correct; ~178k call sites |
| float arithmetic (fraction-classified) | 960 | 3.9M | 0 | exact-ish ops |
| **total** | **~26,400** | **~108M** | **0** | + 1 driver bug, + 1 rust-gpu compile-hang (Finding 2) |

**Honest statement (2026-06-10).** Across **~108 million** differential comparisons spanning
integer, cross-function-call, and float-arithmetic programs, the harness found **zero
confirmed rust-gpu miscompiles**. It surfaced **one wrong-result that cross-backend testing
attributes to the NVIDIA Vulkan driver** (Finding 1), and **one rust-gpu compile-time hang**
on pathological call chains (Finding 2). This is a correctness signal for the audited surface
of rust-gpu's codegen — but note the harness's reach is the whole GPU stack, not rust-gpu
alone (Attribution). Untested frontier: structs/enums, transcendentals, inf/NaN/subnormal
floats, multi-backend campaigns, and range-specific bugs below the float gate.

## Attribution — the methodology lesson (the real contribution)

A CPU-oracle differential tester answers "did the GPU produce the value the source means?"
A *no* can come from **rust-gpu codegen, SPIR-T, naga, or the driver** — the oracle cannot
tell which. Finding 1 looked like a rust-gpu bug under single-GPU testing (it reproduced
through both the passthrough and naga frontends), but those frontends **share one driver**, so
that was not independent corroboration. The fix is **cross-backend differential**: re-run the
same shader on a second, independent backend (here WARP/DX12; ideally also RADV, lavapipe).
Agreement across backends + disagreement with CPU ⇒ a real cross-stack value bug; a single
backend disagreeing ⇒ that backend (driver/translator). This discipline is now in
`repro.rs` and must precede any upstream attribution. Deterministic regression sweep:
`conformance/ci.sh`.
