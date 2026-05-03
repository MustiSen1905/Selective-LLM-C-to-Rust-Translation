#!/usr/bin/env bash
# Performance benchmark: pdfresurrect C-only vs hybrid (selective-rust).
# Reports wall-clock mean+stddev across N iterations per PDF input.
set -euo pipefail

WT=/mnt/c/Users/musti/OneDrive/Dokumente/Uni/Bachelorarbeit/Project/Selective-LLM-C-to-Rust-Translation/.claude/worktrees/tender-newton-c89e88
OUT=$WT/benchmark/bin
RESULTS=$WT/benchmark/results
mkdir -p "$RESULTS"
N=${N:-20}   # iterations per (binary, input) pair
WARMUP=${WARMUP:-3}

# Collect a zoo of PDFs spanning 4 orders of magnitude in size.
PDFS=(
    "/mnt/c/Users/musti/Downloads/01-assignment-bomblab.pdf"          # ~230 KB
    "/mnt/c/Users/musti/Downloads/02-assignment-cachelab.pdf"         # ~316 KB
    "/mnt/c/Users/musti/Downloads/machines-11-00677.pdf"              # ~6 MB
    "/mnt/c/Users/musti/Downloads/978-3-031-37709-9.pdf"              # ~15 MB
    "/mnt/c/Users/musti/Downloads/Analysis.pdf"                       # ~20 MB
    "/mnt/c/Users/musti/Downloads/Norman Welz - Tradingpsychologie-1.pdf"  # ~89 MB
)

# Filter to existing files; sort by size.
EXISTING=()
for p in "${PDFS[@]}"; do
    [[ -f "$p" ]] && EXISTING+=("$p")
done
if [[ ${#EXISTING[@]} -eq 0 ]]; then
    echo "No PDFs found"; exit 1
fi

CSV=$RESULTS/pdfresurrect_perf.csv
MEM_CSV=$RESULTS/pdfresurrect_mem.csv
echo "binary,input,size_bytes,run,wall_ns" > "$CSV"
echo "binary,input,size_bytes,max_rss_kb" > "$MEM_CSV"

# Helper to extract max RSS from /usr/bin/time -v stderr.
measure_rss() {
    local bin=$1
    local pdf=$2
    /usr/bin/time -v "$OUT/$bin" "$pdf" >/dev/null 2> /tmp/_time_stderr
    grep "Maximum resident set size" /tmp/_time_stderr | awk -F: '{gsub(/ /,"",$2); print $2}'
}

for pdf in "${EXISTING[@]}"; do
    sz=$(stat -c %s "$pdf")
    echo ""
    echo "=== $(basename "$pdf") ($sz bytes) ==="
    for bin in pdfresurrect_c pdfresurrect_hybrid; do
        echo "--- $bin ---"
        # Warmup (also primes filesystem cache).
        for ((w=0; w<WARMUP; w++)); do
            "$OUT/$bin" "$pdf" >/dev/null 2>&1 || true
        done
        # Wall-time runs.
        for ((i=0; i<N; i++)); do
            ns=$(date +%s%N)
            "$OUT/$bin" "$pdf" >/dev/null 2>&1 || true
            ne=$(date +%s%N)
            wall=$((ne - ns))
            echo "$bin,$(basename "$pdf"),$sz,$i,$wall" >> "$CSV"
        done
        # Peak RSS (single shot — memory is mostly deterministic).
        rss=$(measure_rss "$bin" "$pdf")
        echo "$bin,$(basename "$pdf"),$sz,$rss" >> "$MEM_CSV"
    done
done

echo ""
echo "=== Summary ==="
python3 << PYEOF
import csv, statistics
from collections import defaultdict
rows = list(csv.DictReader(open("$CSV")))
mrows = list(csv.DictReader(open("$MEM_CSV")))
groups = defaultdict(list)
for r in rows:
    key = (r["binary"], r["input"], int(r["size_bytes"]))
    groups[key].append(int(r["wall_ns"]))
mem = {(r["binary"], r["input"]): int(r["max_rss_kb"]) for r in mrows}

# Aggregate by (binary, input).
def short(name, n=42):
    return name if len(name) <= n else name[:n-3]+"..."

print()
print(f"{'binary':<22}{'input':<45}{'size MB':>9}{'mean ms':>10}{'stdev':>9}{'rss MB':>9}{'n':>4}")
print("-" * 110)
table = {}
for (b, inp, sz), ts in sorted(groups.items(), key=lambda x: (x[0][2], x[0][0])):
    mean_ms = statistics.mean(ts) / 1e6
    stdev_ms = statistics.stdev(ts) / 1e6 if len(ts) > 1 else 0
    rss_mb = mem.get((b, inp), 0) / 1024
    print(f"{b:<22}{short(inp):<45}{sz/1e6:>9.2f}{mean_ms:>10.2f}{stdev_ms:>9.2f}{rss_mb:>9.1f}{len(ts):>4}")
    table[(b, inp)] = (mean_ms, rss_mb, sz)
print()
print(f"{'input':<45}{'size MB':>9}{'C ms':>9}{'hyb ms':>9}{'Δ time':>9}{'C MB':>8}{'hyb MB':>8}{'Δ mem':>8}")
print("-" * 105)
inputs = sorted({(i, sz) for (_, i, sz) in groups.keys()}, key=lambda x: x[1])
for inp, sz in inputs:
    c = table.get(("pdfresurrect_c", inp))
    h = table.get(("pdfresurrect_hybrid", inp))
    if c and h:
        ct, cm, _ = c; ht, hm, _ = h
        dt = (ht - ct) / ct * 100
        dm = (hm - cm) / cm * 100 if cm > 0 else 0
        print(f"{short(inp):<45}{sz/1e6:>9.2f}{ct:>9.2f}{ht:>9.2f}{dt:>+8.1f}%{cm:>8.1f}{hm:>8.1f}{dm:>+7.1f}%")
PYEOF

echo ""
echo "Detailed CSV: $CSV"
