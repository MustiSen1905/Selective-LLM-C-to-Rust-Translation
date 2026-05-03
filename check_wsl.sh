#!/usr/bin/env bash
cd /mnt/c/Users/musti/OneDrive/Dokumente/Uni/Bachelorarbeit/Project/Selective-LLM-C-to-Rust-Translation/.claude/worktrees/tender-newton-c89e88/Rust-Outcome/abc2mtex/rust_out
export PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin"
echo "=== cargo check ==="
cargo check --message-format=short 2>&1 | tail -60
echo "=== error summary ==="
cargo check --message-format=short 2>&1 | grep -E "^error" | sort | uniq -c | sort -rn | head -20
echo "=== total errors ==="
cargo check --message-format=short 2>&1 | grep -c "^error\["
