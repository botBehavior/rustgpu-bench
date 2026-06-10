# rust-gpu conformance corpus — notes

Differential testing of rust-gpu (Rust → SPIR-V) via `tools/diff-fuzz`: generate pure
functions, compile through cargo-gpu, run on GPU, diff bit-exact against a CPU interpreter
of the same AST, with native rustc as the final arbiter. This file is the running catalog
of what the fuzzer has surfaced, grouped by class. Confirmed miscompile repros live beside
it; filing any of these upstream is a per-item RED gate (drafts in `../drafts/`).

## Class A — comparison-fold miscompiles (CONFIRMED, 2 repros)

Wrong runtime values (not ICEs). Both reproduce identically through NVIDIA passthrough AND
the naga frontend — two independent SPIR-V consumers agree with each other and disagree with
native rustc, so the fault is in rust-gpu codegen, not a driver.

- `../findings/min-module-seed423.rs` (9 nodes): `if 0 < x.wrapping_mul(EVEN_CONST) { y } else { y % (C|1) }`
- `../findings/min-module-seed303.rs` (15 nodes): bounded loop accumulating
  `acc*1664525 + (y + select(y < (C|y), x, select(x < C2, x, y)))`

Shared motif: a `<` comparison whose operand is `wrapping_mul`-by-even or `bitwise-or`-with
-constant, feeding a select. Reads like a const-prop / range-analysis pass folding the
comparison to a constant on the GPU path. (Hypothesis — passes not yet read.)

Campaign that found them: 500 batches / 24,000 functions / ~98M comparisons (integer grammar,
no calls).

## Class B — pathological inlining on deep call chains (ROBUSTNESS, not a miscompile)

Observed 2026-06-10 when cross-function calls were added to the grammar. A batch of 48
functions where any function may call any earlier one **hangs the compiler** (rustc killed,
exit 143) before emitting SPIR-V. Cause is structural and expected: logical-addressing
SPIR-V cannot pass pointers, so rust-gpu's legalizer must **inline every call**; an
unrestricted call DAG of depth ~48 inlines to an exponential blowup.

Not a correctness bug, but a real scaling cliff worth a maintainer's awareness — deep
helper-call nests in user shader code will compile-bomb the same way. The fuzzer now
restricts callees to a small leaf set (`CALL_TARGETS`) to keep inlining depth at 1 and stay
polynomial, so it can still hunt for *miscompiles* across call boundaries. A focused,
minimized "deep chain → hang" repro would be the artifact to attach if filed.

## Coverage so far

| grammar | functions | comparisons | miscompiles | notes |
|---|---|---|---|---|
| integer/bool/shift/div/select/loop | ~24,000 | ~98M | **2** (Class A) | bit-exact |
| + cross-function calls (leaf callees) | 1,440 | 5.9M | 0 | call lowering correct; ~178k call sites |
| float arithmetic (fraction-classified) | 960 | 3.9M | 0 | exact-ish ops; worst legit divergence 10% of inputs |
| **total** | **~26,400** | **~108M** | **2** | one shared motif |

## Class C — float arithmetic (method; `--float`)

Floats can't be bit-exact-compared: fma-contraction, division rounding, and catastrophic
cancellation make the GPU legitimately differ from the CPU. The naive metrics fail loudly —
a per-input ULP threshold of 64 flagged **32 "findings" in 2 batches, all false positives**
(e.g. cpu `-1.5665436e-1` vs gpu `-1.5665887e-1`, a ~3e-5 relative difference from rounding).

The working classifier is **per-function and statistical**, not per-input:
- grammar is "exact-ish" only (`+ - * /`, neg, abs, min, max, fma, select, a halving loop);
  **transcendentals are excluded** — their precision is driver-defined, so they are all
  false positives for *miscompile* hunting.
- inputs are finite normal f32 in ±[~0.008, 512), derived by the same `to_f32` on both sides;
  any input where either side reaches inf/NaN is **skipped** (unclassifiable).
- per function, count the fraction of finite inputs whose **relative** error exceeds 1%; flag
  the function only if that fraction exceeds **25%**. A real miscompile diverges on ~all
  inputs; legitimate cancellation spikes on only a sparse few.
- Calibration: across ~390K comparisons the worst *legitimate* function diverged on 16.3%
  of inputs — under the 25% gate, with headroom below the ~100% a systematic bug would show.

Limits (honest): input-range-specific miscompiles below the 25% fraction would be missed;
inf/NaN/subnormal/transcendental behavior is out of scope for v1 and is its own future axis.

**Defect-density statement (2026-06-10).** Across **~108 million** differential comparisons
spanning integer/bitwise/control-flow, cross-function-call, and float-arithmetic programs,
rust-gpu 0.10.0-alpha.1 produced exactly **2 miscompiles, both the same Class-A
comparison-fold motif**. Both are triple-confirmed: native rustc agrees with the AST
interpreter (the CPU value is ground truth), and both reproduce with identical wrong values
through *both* the SPIR-V passthrough and naga frontends (two independent consumers agree the
GPU is wrong). Cross-function call lowering, argument passing, and exact-ish float arithmetic
showed **no** miscompiles at this scale — a strong correctness signal for the audited surface.
Untested frontier: structs/enums, transcendentals, inf/NaN/subnormal floats, and range-
specific bugs below the 25%-fraction float gate.

Throughput: integer ~6 s/batch, float ~6 s/batch, calls ~18 s/batch (the must-inline
legalizer). Deterministic regression sweep: `conformance/ci.sh`.
