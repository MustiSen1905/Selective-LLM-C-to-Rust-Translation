#!/usr/bin/env bash
# One-shot MiBench benchmark runner.
# Runs: build → generate inputs → measure
#
# Run from WSL:  bash benchmark/harness/run_mibench_all.sh
#
# If you get "pipefail: invalid option" the file has Windows CRLF endings. Fix:
#   sed -i 's/\r//' benchmark/harness/run_mibench_all.sh
set -uo pipefail

HARNESS=$(cd "$(dirname "$0")" && pwd)

echo "=========================================="
echo " MiBench: Build + Inputs + Measure"
echo "=========================================="
echo ""

step() {
  echo ""
  echo ">>> $1"
  echo "-------------------------------------------"
}

step "STEP 1 / 3 — Building benchmark programs"
bash "$HARNESS/build_mibench.sh"

step "STEP 2 / 3 — Generating input files"
bash "$HARNESS/gen_mibench_inputs.sh"

step "STEP 3 / 3 — Running measurements"
bash "$HARNESS/measure_mibench.sh"

echo ""
echo "=========================================="
echo " Done. Results are in benchmark/results/:"
WT=$(cd "$HARNESS/../.." && pwd)
ls -lh "$WT/benchmark/results"/mibench_*.csv 2>/dev/null || echo "  (no CSV files found)"
echo "=========================================="
