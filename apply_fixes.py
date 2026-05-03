"""Apply targeted sanitizer fixes to already-generated rust files."""
import os
import re

RUST_SRC = os.path.join(
    os.path.dirname(__file__),
    "Rust-Outcome", "abc2mtex", "rust_out", "src",
)

for fname in sorted(os.listdir(RUST_SRC)):
    if not fname.endswith(".rs"):
        continue
    fpath = os.path.join(RUST_SRC, fname)
    with open(fpath, "r", encoding="utf-8") as f:
        code = f.read()
    original = code

    # a) core::std::... -> std::...
    code = re.sub(r'(?:::)?core::std::', 'std::', code)

    # b) `public xxx:` -> `pub xxx:` (struct fields)
    code = re.sub(
        r'^(\s*)public(\s+\w+\s*:)',
        r'\1pub\2',
        code,
        flags=re.MULTILINE,
    )

    # c) Remove LLM truncation placeholders
    code = re.sub(r'^\s*//\s*\.\.\.\s*continue.*$', '', code, flags=re.MULTILINE | re.IGNORECASE)
    code = re.sub(r'^\s*//\s*Similar\b.*should (follow|be).*$', '', code, flags=re.MULTILINE | re.IGNORECASE)

    if code != original:
        with open(fpath, "w", encoding="utf-8") as f:
            f.write(code)
        print(f"  [+] Fixes applied to {fname}")
    else:
        print(f"  [=] No changes for {fname}")
