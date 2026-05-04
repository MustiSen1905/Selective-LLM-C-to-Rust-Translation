#!/usr/bin/env bash
# Generate / copy MiBench benchmark input files.
# Run from WSL:  bash benchmark/harness/gen_mibench_inputs.sh
#
# If you get "pipefail: invalid option" the file has Windows CRLF endings. Fix:
#   sed -i 's/\r//' benchmark/harness/gen_mibench_inputs.sh
#
# What this does:
#   - Copies the existing "small" inputs from project subdirs into input_data/
#     (sha, blowfish, qsort, susan, patricia, dijkstra)
#   - Generates a 50 MB random binary file for CRC32
#
# Output: C-projects/mibench-master/input_data/
set -uo pipefail

WT=$(cd "$(dirname "$0")/../.." && pwd)
MIBENCH="$WT/C-projects/mibench-master"
INPUT_DIR="$MIBENCH/input_data"

echo "================================="
echo " MiBench Input Generation"
echo "================================="
echo "  Input dir: $INPUT_DIR"
echo ""

copy_small() {
  local src="$1" dst_name="$2"
  local dst="$INPUT_DIR/$dst_name"
  printf "  %-38s " "$dst_name"
  if [[ -f "$dst" ]]; then
    echo "already exists  ($(du -h "$dst" | cut -f1))"
  elif [[ -f "$src" ]]; then
    cp "$src" "$dst"
    echo "copied          ($(du -h "$dst" | cut -f1))"
  else
    echo "WARN: source not found: $src"
  fi
}

echo "=== Copying small input files from project subdirs ==="
copy_small "$MIBENCH/security/sha/input_small.asc"          "sha_input.asc"
copy_small "$MIBENCH/security/blowfish/input_small.asc"     "blowfish_input.asc"
copy_small "$MIBENCH/automotive/qsort/input_small.dat"      "qsort_small_input.txt"
copy_small "$MIBENCH/automotive/susan/input_small.pgm"      "susan_input.pgm"
copy_small "$MIBENCH/network/patricia/small.udp"            "patricia_input.udp"
copy_small "$MIBENCH/network/dijkstra/input.dat"            "dijkstra_input.dat"

echo ""
echo "=== Generating CRC32 input (50 MB random binary) ==="
CRC_FILE="$INPUT_DIR/crc_input.pcm"
if [[ -f "$CRC_FILE" ]]; then
  echo "  crc_input.pcm  already exists  ($(du -h "$CRC_FILE" | cut -f1))"
else
  printf "  crc_input.pcm  generating 50 MB ... "
  dd if=/dev/urandom of="$CRC_FILE" bs=1M count=50 2>/dev/null
  echo "done  ($(du -h "$CRC_FILE" | cut -f1))"
fi

echo ""
echo "=== Input data ready ==="
ls -lh "$INPUT_DIR"/ 2>/dev/null | grep -vE '^total|\.gitignore'
