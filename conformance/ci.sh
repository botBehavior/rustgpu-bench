#!/usr/bin/env bash
# Deterministic conformance regression sweep for rust-gpu.
# Generates pseudo-random pure functions (fixed seeds), compiles them through
# cargo-gpu, runs on the GPU, and diffs bit-exact (integer) or by per-function
# relative-divergence fraction (float) against a CPU interpreter, with native
# rustc as the final arbiter. Exits non-zero if ANY divergence is found.
#
# Intended as a CI job: fast, fixed seeds, no findings expected on a correct
# compiler. Catches codegen regressions before users hit them. ~3 min on a
# warm toolchain. Tune batch counts up for a deeper nightly run.
#
# Usage: conformance/ci.sh            (default: 8 integer + 8 float batches)
#        SEED=42 INT=20 FLT=20 conformance/ci.sh
set -euo pipefail
cd "$(dirname "$0")/.."

SEED="${SEED:-1}"
INT="${INT:-8}"
FLT="${FLT:-8}"
BIN=./target/release/diff-fuzz

echo "== building fuzzer =="
cargo build -p diff-fuzz --release

echo "== integer/control-flow/call sweep: $INT batches from seed $SEED =="
"$BIN" "$INT" "$SEED"   # exits 2 on any bit-exact mismatch

echo "== float-arithmetic sweep: $FLT batches from seed $SEED =="
"$BIN" --float "$FLT" "$SEED"   # exits 2 on systematic relative divergence

echo "== CONFORMANCE SWEEP CLEAN: no miscompiles =="
