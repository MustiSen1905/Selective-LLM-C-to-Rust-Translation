"""Add stub type aliases for undefined Safe* types referenced by struct defs.

The LLM sometimes hallucinates nested types like `Box<Safe_IO_marker>` but
never defines `Safe_IO_marker`. This script walks each .rs file, detects
Safe-prefixed type names that are *referenced* but not *defined* anywhere,
and appends `pub type SafeX = ();` stubs to the top of the file so the
struct definitions at least compile.
"""
import os
import re

RUST_SRC = os.path.join(
    os.path.dirname(__file__),
    "Rust-Outcome", "abc2mtex", "rust_out", "src",
)

DEF_RE = re.compile(r'\bpub\s+(?:struct|enum|union|type)\s+(Safe\w+)')
# Any `Safe...` identifier used in a type position (after `<`, `:`, `->`, `Box<`).
REF_RE = re.compile(r'\b(Safe\w+)\b')
MARKER = "// --- auto-generated Safe stub types ---"

for fname in sorted(os.listdir(RUST_SRC)):
    if not fname.endswith(".rs"):
        continue
    fpath = os.path.join(RUST_SRC, fname)
    with open(fpath, "r", encoding="utf-8") as f:
        code = f.read()

    defined = set(DEF_RE.findall(code))
    refs = set(REF_RE.findall(code))
    missing = sorted(refs - defined)

    if not missing:
        print(f"  [=] {fname}: keine fehlenden Safe-Typen")
        continue
    if MARKER in code:
        # Extract existing stubs
        existing_stub_block = code.split(MARKER, 1)[1].split("// --- end stubs ---", 1)[0]
        existing = set(re.findall(r'\bpub\s+type\s+(Safe\w+)', existing_stub_block))
        missing = [m for m in missing if m not in existing]
        if not missing:
            print(f"  [=] {fname}: Stubs bereits vollstaendig")
            continue

    stub_lines = [f"pub type {n} = ();" for n in missing]
    header = MARKER + "\n" + "\n".join(stub_lines) + "\n// --- end stubs ---\n\n"

    # Fuege NACH einem evtl. bestehenden cross-module-import-Block ein.
    if "// --- auto-generated cross-module Safe imports ---" in code:
        parts = code.split("\n\n", 1)
        code = parts[0] + "\n\n" + header + (parts[1] if len(parts) > 1 else "")
    else:
        code = header + code

    with open(fpath, "w", encoding="utf-8") as f:
        f.write(code)
    print(f"  [+] {fname}: {len(missing)} Stub(s): {', '.join(missing)}")
