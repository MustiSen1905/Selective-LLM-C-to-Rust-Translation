#!/usr/bin/env bash
# Benchmark MiBench programs: hyperfine (wall time) + /usr/bin/time -v (peak RSS).
# Measures all available variants per program: V-C, V-c2rust, V-hybrid.
#
# Prerequisites:
#   1. bash benchmark/harness/build_mibench.sh         (builds V-C binaries)
#   2. bash benchmark/harness/gen_mibench_inputs.sh    (generates input files)
#   3. python3 translation/mibench_pipeline.py --all   (translation pipeline)
#   4. bash benchmark/harness/build_mibench_hybrid.sh  (builds V-c2rust / V-hybrid)
#
# Run from WSL:  bash benchmark/harness/measure_mibench.sh
#
# If you get "pipefail: invalid option" the file has Windows CRLF endings. Fix:
#   sed -i 's/\r//' benchmark/harness/measure_mibench.sh
#
# Output:
#   benchmark/results/mibench_perf.csv   — wall-time statistics (all variants)
#   benchmark/results/mibench_mem.csv    — peak RSS (all variants)
set -uo pipefail

WT=$(cd "$(dirname "$0")/../.." && pwd)
MIBENCH="$WT/C-projects/mibench-master"
BIN="$MIBENCH/bin"
INPUT="$MIBENCH/input_data"
RESULTS="$WT/benchmark/results"
mkdir -p "$RESULTS"

PERF_CSV="$RESULTS/mibench_perf.csv"
MEM_CSV="$RESULTS/mibench_mem.csv"
WARMUP=3
RUNS=10
TMP_JSON="/tmp/mibench_hf.json"
TMP_TIME="/tmp/mibench_time.txt"

# CSV headers — 'variant' column: c | c2rust | hybrid
echo "program,variant,workload,runs,mean_s,stddev_s,median_s,min_s,max_s" > "$PERF_CSV"
echo "program,variant,workload,rss_kb"                                     > "$MEM_CSV"

# ---------------------------------------------------------------------------
# run_bench NAME VARIANT WORKLOAD_LABEL CMD [ARGS...]
#   Runs hyperfine + time -v and appends one row to each CSV.
#   Silently skips if the binary does not exist.
# ---------------------------------------------------------------------------
run_bench() {
  local name="$1" variant="$2" workload="$3"
  shift 3
  local -a cmd=("$@")

  echo ""
  echo "--- $name  [$variant]  [$workload] ---"

  # Skip if binary not built
  if [[ ! -x "${cmd[0]}" ]]; then
    echo "  SKIP: binary not found: ${cmd[0]}"
    return 0
  fi

  # Build command string for hyperfine (output suppressed so terminal stays clean)
  local hf_cmd
  hf_cmd="$(printf '%q ' "${cmd[@]}") >/dev/null 2>&1"

  # --- Wall-time measurement (hyperfine) ---
  hyperfine \
    --warmup "$WARMUP" \
    --runs   "$RUNS"   \
    --export-json "$TMP_JSON" \
    --ignore-failure \
    -- "$hf_cmd"

  # Parse JSON → CSV row
  python3 - "$TMP_JSON" "$name" "$variant" "$workload" "$RUNS" << 'PYEOF' | tee -a "$PERF_CSV"
import sys, json
jf, prog, variant, wl, runs = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5]
with open(jf) as f:
    r = json.load(f)["results"][0]
print(
    f"{prog},{variant},{wl},{runs},"
    f"{r['mean']:.6f},{r['stddev']:.6f},{r['median']:.6f},"
    f"{r['min']:.6f},{r['max']:.6f}"
)
print(f"  mean = {r['mean']*1000:.1f} ms", file=sys.stderr)
PYEOF

  # --- Peak-RSS measurement (/usr/bin/time -v) ---
  { /usr/bin/time -v "${cmd[@]}" >/dev/null ; } 2>"$TMP_TIME" || true
  local rss
  rss=$(grep -oP '(?<=Maximum resident set size \(kbytes\): )\d+' "$TMP_TIME" || echo "0")
  echo "$name,$variant,$workload,$rss" | tee -a "$MEM_CSV"
  echo "  rss  = $rss KB"
}

# ---------------------------------------------------------------------------
# all_variants NAME WORKLOAD [ARGS...]
#   Runs V-C, V-c2rust, V-hybrid for the same program + arguments.
#   V-c2rust and V-hybrid are silently skipped if not yet built.
# ---------------------------------------------------------------------------
all_variants() {
  local name="$1" workload="$2"
  shift 2
  local -a args=("$@")

  run_bench "$name" "c"      "$workload" "$BIN/$name"         "${args[@]+"${args[@]}"}"
  run_bench "$name" "c2rust" "$workload" "$BIN/${name}_c2rust" "${args[@]+"${args[@]}"}"
  run_bench "$name" "hybrid" "$workload" "$BIN/${name}_hybrid" "${args[@]+"${args[@]}"}"
}

# ---------------------------------------------------------------------------
# Pre-compute file-dependent arguments
# ---------------------------------------------------------------------------
QSORT_IN="$INPUT/qsort_small_input.txt"
DIJKSTRA_IN="$INPUT/dijkstra_input.dat"

QSORT_N=1000
DIJKSTRA_N=100
[[ -f "$QSORT_IN"    ]] && QSORT_N=$(wc -l    < "$QSORT_IN")
[[ -f "$DIJKSTRA_IN" ]] && DIJKSTRA_N=$(wc -l < "$DIJKSTRA_IN")

SHA_IN="$INPUT/sha_input.asc"
BLOWFISH_IN="$INPUT/blowfish_input.asc"
PATRICIA_IN="$INPUT/patricia_input.udp"
SUSAN_IN="$INPUT/susan_input.pgm"
CRC_IN="$INPUT/crc_input.pcm"

BLOWFISH_KEY="1234567890abcdeffedcba0987654321"
BLOWFISH_OUT="/tmp/mibench_blowfish_bench.enc"

# ---------------------------------------------------------------------------
echo ""
echo "================================="
echo " MiBench Measurement (all variants)"
echo "================================="
printf "  Warmup : %d   Runs: %d\n" "$WARMUP" "$RUNS"
printf "  Perf   : %s\n"            "$PERF_CSV"
printf "  Memory : %s\n"            "$MEM_CSV"

# ---- Automotive ----
all_variants "basicmath"   "runs=5"     5
all_variants "bitcount"    "iter=5M"    5000000
all_variants "qsort_small" "small_dat"  "$QSORT_N"    "$QSORT_IN"
all_variants "susan"       "smooth"     "$SUSAN_IN"   /dev/null  -s

# ---- Network ----
all_variants "dijkstra"    "small_dat"  "$DIJKSTRA_N" "$DIJKSTRA_IN"
all_variants "patricia"    "small_udp"  "$PATRICIA_IN"

# ---- Office ----
all_variants "stringsearch" "iter=100"  100

# ---- Security ----
all_variants "blowfish"    "enc_small"  e  "$BLOWFISH_IN"  "$BLOWFISH_OUT"  "$BLOWFISH_KEY"
all_variants "sha"         "small_asc"  "$SHA_IN"

# ---- Telecomm ----
all_variants "crc"         "50MB"       "$CRC_IN"
all_variants "fft"         "256x4096"   256 4096

# ---------------------------------------------------------------------------
echo ""
echo "================================="
echo " All benchmarks complete"
echo "================================="
echo ""
echo "=== Performance (wall time) ==="
column -t -s, "$PERF_CSV" 2>/dev/null || cat "$PERF_CSV"
echo ""
echo "=== Memory (peak RSS) ==="
column -t -s, "$MEM_CSV"  2>/dev/null || cat "$MEM_CSV"
