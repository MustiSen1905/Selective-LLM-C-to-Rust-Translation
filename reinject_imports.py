"""Strip broken cross-module import headers and re-inject with fixed logic."""
import os
import sys
sys.path.insert(0, os.path.join(os.path.dirname(__file__), "translation"))
from translation import inject_cross_module_safe_imports

RUST_SRC = os.path.join(
    os.path.dirname(__file__),
    "Rust-Outcome", "abc2mtex", "rust_out", "src",
)

MARKER = "// --- auto-generated cross-module Safe imports ---"

for fname in os.listdir(RUST_SRC):
    if not fname.endswith(".rs"):
        continue
    fpath = os.path.join(RUST_SRC, fname)
    with open(fpath, "r", encoding="utf-8") as f:
        code = f.read()
    if MARKER not in code:
        continue
    # Entferne alles von MARKER bis zur ersten Leerzeile nach den use-Lines.
    lines = code.split("\n")
    out = []
    skip = False
    for line in lines:
        if line.strip() == MARKER:
            skip = True
            continue
        if skip:
            if line.strip() == "":
                skip = False
            continue
        out.append(line)
    new_code = "\n".join(out).lstrip("\n")
    with open(fpath, "w", encoding="utf-8") as f:
        f.write(new_code)
    print(f"  [-] Header aus {fname} entfernt.")

print("--- Re-inject ---")
inject_cross_module_safe_imports(RUST_SRC)
