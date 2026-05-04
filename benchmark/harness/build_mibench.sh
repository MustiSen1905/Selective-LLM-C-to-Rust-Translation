#!/usr/bin/env bash
# Build MiBench C benchmark programs (11 simple projects).
# Run from WSL:  bash benchmark/harness/build_mibench.sh
#
# If you get "pipefail: invalid option" the file has Windows CRLF endings. Fix:
#   sed -i 's/\r//' benchmark/harness/build_mibench.sh
#
# Output: C-projects/mibench-master/bin/{basicmath, bitcount, ...}
set -uo pipefail

WT=$(cd "$(dirname "$0")/../.." && pwd)
MIBENCH="$WT/C-projects/mibench-master"
BIN="$MIBENCH/bin"

echo "================================="
echo " MiBench Build"
echo "================================="
echo "  Source : $MIBENCH"
echo "  Bin    : $BIN"
echo ""

ok=0; fail=0

# build_proj LABEL SUBDIR MAKE_TARGET OUTPUT_NAME
build_proj() {
  local label="$1" subdir="$2" target="$3" outname="$4"
  local bd="$MIBENCH/$subdir"
  printf "  %-18s ... " "$label"
  local out
  if out=$(make -C "$bd" "$target" -s 2>&1) && [[ -f "$bd/$target" ]]; then
    cp "$bd/$target" "$BIN/$outname"
    echo "OK"
    ok=$((ok + 1))
  else
    echo "FAILED"
    printf '%s\n' "$out" | tail -6 | sed 's/^/      /'
    fail=$((fail + 1))
  fi
}

echo "=== Automotive ==="
build_proj "basicmath"    "automotive/basicmath"  "basicmath"    "basicmath"
build_proj "bitcount"     "automotive/bitcount"   "bitcnts"      "bitcount"
build_proj "qsort_small"  "automotive/qsort"      "qsort_small"  "qsort_small"
build_proj "susan"        "automotive/susan"       "susan"        "susan"

echo ""
echo "=== Network ==="
build_proj "dijkstra"     "network/dijkstra"       "dijkstra"     "dijkstra"
build_proj "patricia"     "network/patricia"       "patricia"     "patricia"

echo ""
echo "=== Office ==="
build_proj "stringsearch" "office/stringsearch"    "search"       "stringsearch"

echo ""
echo "=== Security ==="
build_proj "blowfish"     "security/blowfish"      "bf"           "blowfish"
build_proj "sha"          "security/sha"           "sha"          "sha"

echo ""
echo "=== Telecomm ==="
build_proj "CRC32"        "telecomm/CRC32"         "crc"          "crc"
build_proj "FFT"          "telecomm/FFT"           "fft"          "fft"

echo ""
echo "================================="
printf "  OK: %d   Failed: %d\n" "$ok" "$fail"
echo "================================="
echo ""
echo "Binaries in $BIN:"
ls -lh "$BIN"/ 2>/dev/null | grep -vE '\.sh$|^total' || echo "  (none)"
