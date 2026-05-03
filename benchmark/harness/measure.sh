#!/usr/bin/env bash
# Measure V-C, V-c2rust, V-Hybrid for one or all projects.
# Requires: hyperfine (wall-clock), /usr/bin/time -v (peak RSS).
#
# Usage:
#   bash benchmark/harness/measure.sh [project]        # one project
#   bash benchmark/harness/measure.sh                  # all known projects
#
# Workload files are read from benchmark/harness/workloads/<project>.conf
# Each non-empty, non-comment line in the .conf file is one CLI argument string
# passed to the binary (can include flags, e.g. "/path/to/file.pdf").
#
# Environment overrides:
#   RUNS=30          number of hyperfine benchmark runs (default 30)
#   WARMUP=5         number of hyperfine warmup runs (default 5)
#   SKIP_MISSING=1   skip binaries that don't exist instead of aborting
set -euo pipefail

WT=$(cd "$(dirname "$0")/../.." && pwd)
BIN_DIR="$WT/benchmark/bin"
RESULTS_DIR="$WT/benchmark/results"
WORKLOADS_DIR="$(dirname "$0")/workloads"
mkdir -p "$RESULTS_DIR"

RUNS=${RUNS:-30}
WARMUP=${WARMUP:-5}
SKIP_MISSING=${SKIP_MISSING:-0}

ALL_PROJECTS=(pdfresurrect-master abc2mtex tiny-AES-c-master)

if [[ $# -ge 1 ]]; then
  PROJECTS=("$1")
else
  PROJECTS=("${ALL_PROJECTS[@]}")
fi

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
check_tool() {
  if ! command -v "$1" &>/dev/null; then
    echo "ERROR: '$1' not found. Install with: $2" >&2
    exit 1
  fi
}

check_tool hyperfine "cargo install hyperfine  OR  apt install hyperfine"
check_tool /usr/bin/time ""   # part of util-linux / procps

measure_rss() {
  local bin=$1; shift
  /usr/bin/time -v "$bin" "$@" >/dev/null 2>/tmp/_harness_time_stderr
  grep "Maximum resident set size" /tmp/_harness_time_stderr \
    | awk -F: '{gsub(/ /,"",$2); print $2}'
}

short_name() {
  local p=$1
  p="${p%%-master}"
  echo "${p//-/_}"
}

# ---------------------------------------------------------------------------
# Per-project measurement
# ---------------------------------------------------------------------------
for PROJECT in "${PROJECTS[@]}"; do
  SHORT=$(short_name "$PROJECT")

  # Determine variant binaries
  declare -A VARIANTS
  VARIANTS=(
    [c]="$BIN_DIR/${SHORT}_c"
    [c2rust]="$BIN_DIR/${SHORT}_c2rust"
    [hybrid]="$BIN_DIR/${SHORT}_hybrid"
  )

  # Load workload lines
  WCONF="$WORKLOADS_DIR/$PROJECT.conf"
  if [[ ! -f "$WCONF" ]]; then
    echo "WARN: No workload config at $WCONF. Using empty workload (binary run with no args)." >&2
    WORKLOADS=("")
  else
    mapfile -t WORKLOADS < <(grep -v '^\s*#' "$WCONF" | grep -v '^\s*$')
    if [[ ${#WORKLOADS[@]} -eq 0 ]]; then
      WORKLOADS=("")
    fi
  fi

  PERF_CSV="$RESULTS_DIR/${SHORT}_perf.csv"
  MEM_CSV="$RESULTS_DIR/${SHORT}_mem.csv"

  # Write headers (overwrite previous run)
  echo "project,variant,workload,runs,mean_s,stddev_s,median_s,p95_s,min_s,max_s" > "$PERF_CSV"
  echo "project,variant,workload,max_rss_kb" > "$MEM_CSV"

  echo ""
  echo "======================================================================="
  echo " Project: $PROJECT"
  echo "======================================================================="

  for LABEL in c c2rust hybrid; do
    BIN="${VARIANTS[$LABEL]}"

    if [[ ! -x "$BIN" ]]; then
      if [[ "$SKIP_MISSING" == "1" ]]; then
        echo "SKIP: $BIN not found." >&2
        continue
      else
        echo "ERROR: $BIN not found. Run build.sh first (or set SKIP_MISSING=1)." >&2
        exit 1
      fi
    fi

    echo ""
    echo "--- Variant: $LABEL ($BIN) ---"

    for WORKLOAD in "${WORKLOADS[@]}"; do
      # Split workload string into args
      read -ra WARGS <<< "$WORKLOAD"
      WLABEL="${WORKLOAD:-<no-args>}"
      WLABEL_SAFE="${WLABEL//\//_}"   # safe for filenames
      WLABEL_SAFE="${WLABEL_SAFE// /_}"

      echo "  Workload: $WLABEL"

      # --- Wall-clock via hyperfine ---
      # Build a single shell-quoted command string so hyperfine treats the
      # binary + all its arguments as ONE command (not multiple competing ones).
      HYPERFINE_JSON="/tmp/_harness_hyperfine_${SHORT}_${LABEL}.json"
      if [[ ${#WARGS[@]} -gt 0 ]]; then
        # printf %q produces shell-safe quoting even for paths with spaces.
        CMD=$(printf '%q ' "$BIN" "${WARGS[@]}")
      else
        CMD=$(printf '%q' "$BIN")
      fi
      hyperfine \
        --warmup "$WARMUP" \
        --runs "$RUNS" \
        --export-json "$HYPERFINE_JSON" \
        --ignore-failure \
        -- "$CMD"

      # Parse hyperfine JSON (python one-liner, no extra deps)
      python3 - "$HYPERFINE_JSON" "$PROJECT" "$LABEL" "$WLABEL" "$RUNS" "$PERF_CSV" << 'PYEOF'
import json, sys, csv, os
jf, project, label, wlabel, runs, outcsv = sys.argv[1:]
with open(jf) as f:
    data = json.load(f)
r = data["results"][0]
mean = r["mean"]; std = r["stddev"]; med = r.get("median", mean)
times = sorted(r.get("times", [mean]))
p95 = times[int(len(times)*0.95)] if times else mean
mn = min(times) if times else mean; mx = max(times) if times else mean
row = [project, label, wlabel, runs,
       f"{mean:.6f}", f"{std:.6f}", f"{med:.6f}",
       f"{p95:.6f}", f"{mn:.6f}", f"{mx:.6f}"]
with open(outcsv, "a", newline="") as f:
    csv.writer(f).writerow(row)
PYEOF

      # --- Peak RSS via /usr/bin/time -v ---
      if [[ ${#WARGS[@]} -gt 0 ]]; then
        RSS=$(measure_rss "$BIN" "${WARGS[@]}" 2>/dev/null || echo 0)
      else
        RSS=$(measure_rss "$BIN" 2>/dev/null || echo 0)
      fi
      echo "$PROJECT,$LABEL,$WLABEL,$RSS" >> "$MEM_CSV"
      MEAN_MS=$(python3 - "$PERF_CSV" "$LABEL" "$WLABEL" << 'MEAN_PYEOF'
import sys, csv
pcsv, label, wlabel = sys.argv[1], sys.argv[2], sys.argv[3]
with open(pcsv) as f:
    for row in csv.reader(f):
        if len(row) >= 6 and row[1] == label and row[2] == wlabel:
            last = row
try:
    print(f"{float(last[4])*1000:.1f}")
except Exception:
    print("?")
MEAN_PYEOF
)
      echo "    mean=${MEAN_MS} ms | rss=${RSS} KB"
    done
  done
done

echo ""
echo "Results written to $RESULTS_DIR/"
echo "Run benchmark/harness/analyze.py for the summary tables."
