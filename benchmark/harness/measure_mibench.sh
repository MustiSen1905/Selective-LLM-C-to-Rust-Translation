#!/usr/bin/env bash
# Benchmark MiBench programs: hyperfine (wall time) + /usr/bin/time -v (peak RSS).
# Prerequisites: run build_mibench.sh and gen_mibench_inputs.sh first.
# Run from WSL:  bash benchmark/harness/measure_mibench.sh
#
# If you get "pipefail: invalid option" the file has Windows CRLF endings. Fix:
#   sed -i 's/\r//' benchmark/harness/measure_mibench.sh
#
# Output:
#   benchmark/results/mibench_perf.csv   — wall-time statistics per program
#   benchmark/results/mibench_mem.csv    — peak RSS per program
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

# CSV headers
echo "program,workload,runs,mean_s,stddev_s,median_s,min_s,max_s" > "$PERF_CSV"
echo "program,workload,rss_kb"                                     > "$MEM_CSV"

# ---------------------------------------------------------------------------
# run_bench NAME WORKLOAD_LABEL CMD [ARGS...]
#   Runs hyperfine + time -v and appends one row to each CSV.
# ---------------------------------------------------------------------------
run_bench() {
  local name="$1" workload="$2"
  shift 2
  local -a cmd=("$@")

  echo ""
  echo "--- $name  [$workload] ---"

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
  python3 - "$TMP_JSON" "$name" "$workload" "$RUNS" << 'PYEOF' | tee -a "$PERF_CSV"
import sys, json
jf, prog, wl, runs = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
with open(jf) as f:
    r = json.load(f)["results"][0]
print(
    f"{prog},{wl},{runs},"
    f"{r['mean']:.6f},{r['stddev']:.6f},{r['median']:.6f},"
    f"{r['min']:.6f},{r['max']:.6f}"
)
print(f"  mean = {r['mean']*1000:.1f} ms", file=sys.stderr)
PYEOF

  # --- Peak-RSS measurement (/usr/bin/time -v) ---
  { /usr/bin/time -v "${cmd[@]}" >/dev/null ; } 2>"$TMP_TIME" || true
  local rss
  rss=$(grep -oP '(?<=Maximum resident set size \(kbytes\): )\d+' "$TMP_TIME" || echo "0")
  echo "$name,$workload,$rss" | tee -a "$MEM_CSV"
  echo "  rss  = $rss KB"
}

# ---------------------------------------------------------------------------
# Pre-compute file-dependent arguments
# ---------------------------------------------------------------------------
QSORT_IN="$INPUT/qsort_small_input.txt"
DIJKSTRA_IN="$INPUT/dijkstra_input.dat"

# Number of strings / nodes = line count of the respective input files
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
echo " MiBench Measurement"
echo "================================="
printf "  Warmup : %d   Runs: %d\n"   "$WARMUP" "$RUNS"
printf "  Perf   : %s\n"              "$PERF_CSV"
printf "  Memory : %s\n"              "$MEM_CSV"

# ---- Automotive ----
run_bench "basicmath"    "runs=5"      "$BIN/basicmath"    5
run_bench "bitcount"     "iter=5M"     "$BIN/bitcount"     5000000
run_bench "qsort_small"  "small_dat"   "$BIN/qsort_small"  "$QSORT_N"    "$QSORT_IN"
run_bench "susan"        "smooth"      "$BIN/susan"        "$SUSAN_IN"   /dev/null  -s

# ---- Network ----
run_bench "dijkstra"     "small_dat"   "$BIN/dijkstra"     "$DIJKSTRA_N" "$DIJKSTRA_IN"
run_bench "patricia"     "small_udp"   "$BIN/patricia"     "$PATRICIA_IN"

# ---- Office ----
run_bench "stringsearch" "iter=100"    "$BIN/stringsearch" 100

# ---- Security ----
run_bench "blowfish"     "enc_small"   "$BIN/blowfish"     e  "$BLOWFISH_IN"  "$BLOWFISH_OUT"  "$BLOWFISH_KEY"
run_bench "sha"          "small_asc"   "$BIN/sha"          "$SHA_IN"

# ---- Telecomm ----
run_bench "crc"          "50MB"        "$BIN/crc"          "$CRC_IN"
run_bench "fft"          "256x4096"    "$BIN/fft"          256 4096

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
