#!/usr/bin/env bash
set -e
cd /mnt/c/Users/musti/OneDrive/Dokumente/Uni/Bachelorarbeit/Project/Selective-LLM-C-to-Rust-Translation/.claude/worktrees/tender-newton-c89e88
# PATH zuerst, dann venv aktivieren (sonst kickt export die venv/bin raus)
export PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin"
source ../../../.venv/bin/activate
echo "=== env ==="
which python && python --version
which c2rust && which gcc
python -c "import tree_sitter, tree_sitter_rust; print('tree-sitter OK')"
echo "=== run ==="
python main.py abc2mtex
