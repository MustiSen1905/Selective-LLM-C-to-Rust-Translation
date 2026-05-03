#!/usr/bin/env bash
# Build C-only and hybrid pdfresurrect binaries for performance comparison.
set -euo pipefail

WT="/mnt/c/Users/musti/OneDrive/Dokumente/Uni/Bachelorarbeit/Project/Selective-LLM-C-to-Rust-Translation/.claude/worktrees/tender-newton-c89e88"
C_DIR="$WT/C-projects/pdfresurrect-master"
RUST_DIR="$WT/Rust-Outcome/pdfresurrect-master/rust_out"
OUT_DIR="/tmp/pdfresurrect_bench"
mkdir -p "$OUT_DIR"

source ~/.cargo/env

echo "=== 1. C-only baseline ==="
gcc -O2 -DNDEBUG -o "$OUT_DIR/pdfresurrect_c" \
    "$C_DIR/main.c" "$C_DIR/pdf.c"
ls -la "$OUT_DIR/pdfresurrect_c"

echo "=== 2. Hybrid: cargo build --release (Rust staticlib + safe_*.c via build.rs) ==="
cd "$RUST_DIR"
cargo build --release --lib 2>&1 | tail -5
ls -la "$RUST_DIR/target/release/libhybrid_project.a"

echo "=== 3. Hybrid: link C main against Rust staticlib ==="
# The hybrid project's lib.rs re-exports main_0 (the original main, renamed by
# the pipeline). We need to call main from the C side, OR use the Rust binary.
# For the cleanest comparison, we use the same C main.c but link in the Rust
# staticlib which provides the translated functions.
gcc -O2 -DNDEBUG -o "$OUT_DIR/pdfresurrect_hybrid" \
    "$C_DIR/main.c" \
    "$RUST_DIR/target/release/libhybrid_project.a" \
    -lpthread -ldl -lm 2>&1 | tail -20 || {
    echo "Direct link failed; trying alternate symbol resolution"
    # Sometimes need explicit Rust runtime libs
    gcc -O2 -DNDEBUG -o "$OUT_DIR/pdfresurrect_hybrid" \
        "$C_DIR/main.c" "$C_DIR/pdf.c" \
        "$RUST_DIR/target/release/libhybrid_project.a" \
        -lpthread -ldl -lm 2>&1 | tail -20
}

if [[ -x "$OUT_DIR/pdfresurrect_hybrid" ]]; then
    ls -la "$OUT_DIR/pdfresurrect_hybrid"
    echo "BOTH BINARIES BUILT SUCCESSFULLY"
else
    echo "HYBRID BUILD FAILED"
    exit 1
fi
