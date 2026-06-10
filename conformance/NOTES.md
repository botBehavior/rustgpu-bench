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

| grammar | functions tested | miscompiles | notes |
|---|---|---|---|
| integer/bool/shift/div/select/loop | ~24,000 | 2 (Class A) | bit-exact domain |
| + cross-function calls (leaf callees) | campaign in progress | — | validates call lowering |
| floats (ULP-classified) | not yet (P2) | — | the new-territory axis |
