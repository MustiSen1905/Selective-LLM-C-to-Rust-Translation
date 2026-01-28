#!/usr/bin/env python3
# translation/c2rust.py
#
# Selective C->Rust translation with c2rust:
# - "safe" functions remain in C (compiled + linked via build.rs + cc crate)
# - "unsafe" functions (listed in JSON) are transpiled to Rust via c2rust
# - Works with c2rust-transpile versions that DO NOT support --main
#
# Usage:
#   python translation/c2rust.py --input translation/example.c --config translation/unsafe.json --out workdir
#
# Config format:
#   {"unsafe": ["unsafe_copy", "main"]}
#
# Requirements:
#   - clang
#   - c2rust in PATH (or pass --c2rust /path/to/c2rust)
#   - Rust toolchain (cargo)
#
import argparse
import json
import re
import shutil
import subprocess
from pathlib import Path


# ---------------------------
# Utilities: parsing/splitting
# ---------------------------

def strip_comments_and_strings(s: str) -> str:
    out = list(s)
    i, n = 0, len(s)
    state = "code"  # code, line_comment, block_comment, sq, dq
    while i < n:
        c = s[i]
        nxt = s[i + 1] if i + 1 < n else ""

        if state == "code":
            if c == "/" and nxt == "/":
                out[i] = out[i + 1] = " "
                i += 2
                state = "line_comment"
                continue
            if c == "/" and nxt == "*":
                out[i] = out[i + 1] = " "
                i += 2
                state = "block_comment"
                continue
            if c == "'":
                out[i] = " "
                i += 1
                state = "sq"
                continue
            if c == '"':
                out[i] = " "
                i += 1
                state = "dq"
                continue
            i += 1
            continue

        if state == "line_comment":
            out[i] = " "
            if c == "\n":
                state = "code"
            i += 1
            continue

        if state == "block_comment":
            out[i] = " "
            if c == "*" and nxt == "/":
                out[i + 1] = " "
                i += 2
                state = "code"
                continue
            i += 1
            continue

        if state in ("sq", "dq"):
            out[i] = " "
            if c == "\\" and i + 1 < n:
                out[i + 1] = " "
                i += 2
                continue
            if (state == "sq" and c == "'") or (state == "dq" and c == '"'):
                state = "code"
            i += 1
            continue

    return "".join(out)


def find_matching_brace(masked: str, open_idx: int) -> int:
    depth = 0
    for i in range(open_idx, len(masked)):
        if masked[i] == "{":
            depth += 1
        elif masked[i] == "}":
            depth -= 1
            if depth == 0:
                return i
    raise ValueError("No matching closing brace found")


def extract_function_by_name(src: str, name: str) -> tuple[int, int]:
    masked = strip_comments_and_strings(src)
    pattern = re.compile(rf"\b{name}\s*\(", re.MULTILINE)

    for m in pattern.finditer(masked):
        call_idx = m.start()

        start = masked.rfind("\n", 0, call_idx)
        start = 0 if start == -1 else start + 1
        prev_semicolon = masked.rfind(";", 0, call_idx)
        if prev_semicolon != -1 and prev_semicolon > start:
            start = prev_semicolon + 1

        i = m.end()
        paren = 1
        while i < len(masked) and paren > 0:
            if masked[i] == "(":
                paren += 1
            elif masked[i] == ")":
                paren -= 1
            i += 1
        if paren != 0:
            continue

        while i < len(masked) and masked[i].isspace():
            i += 1
        if i < len(masked) and masked[i] == ";":
            continue  # prototype

        while i < len(masked) and masked[i] != "{":
            if masked[i] == ";":
                break
            i += 1
        if i >= len(masked) or masked[i] != "{":
            continue

        close_brace = find_matching_brace(masked, i)
        return (start, close_brace)

    raise ValueError(f"Function definition not found: {name}")


def split_c_file(input_c: Path, unsafe_names: set[str], out_dir: Path):
    src = input_c.read_text(encoding="utf-8")
    masked = strip_comments_and_strings(src)

    func_names: list[str] = []
    for m in re.finditer(r"\b([A-Za-z_]\w*)\s*\(", masked):
        name = m.group(1)
        if name in ("if", "for", "while", "switch", "return", "sizeof"):
            continue
        if name in func_names:
            continue
        try:
            extract_function_by_name(src, name)
            func_names.append(name)
        except Exception:
            pass

    if not func_names:
        raise SystemExit("No function definitions detected in input C file.")

    safe_funcs = [n for n in func_names if n not in unsafe_names]
    unsafe_funcs = [n for n in func_names if n in unsafe_names]
    if not unsafe_funcs:
        raise SystemExit("No unsafe functions found based on your JSON list.")

    first_fn_start = min(extract_function_by_name(src, n)[0] for n in func_names)
    preamble = src[:first_fn_start].rstrip() + "\n"

    out_dir.mkdir(parents=True, exist_ok=True)

    # safe.h prototypes
    safe_decls: list[str] = []
    for n in safe_funcs:
        start, end = extract_function_by_name(src, n)
        chunk = src[start:end + 1]
        brace_idx = chunk.find("{")
        header = chunk[:brace_idx].strip()
        safe_decls.append(header + ";")

    (out_dir / "safe.h").write_text(
        "#pragma once\n#include <stddef.h>\n\n" + ("\n\n".join(safe_decls) + "\n" if safe_decls else ""),
        encoding="utf-8",
    )

    # safe.c
    safe_chunks: list[str] = []
    for n in safe_funcs:
        start, end = extract_function_by_name(src, n)
        safe_chunks.append(src[start:end + 1].strip() + "\n")

    (out_dir / "safe.c").write_text(
        preamble + "\n#include \"safe.h\"\n\n" + "\n".join(safe_chunks),
        encoding="utf-8",
    )

    # unsafe.c
    unsafe_chunks: list[str] = []
    for n in unsafe_funcs:
        start, end = extract_function_by_name(src, n)
        unsafe_chunks.append(src[start:end + 1].strip() + "\n")

    (out_dir / "unsafe.c").write_text(
        preamble + "\n#include \"safe.h\"\n\n" + "\n".join(unsafe_chunks),
        encoding="utf-8",
    )

    return safe_funcs, unsafe_funcs


# ---------------------------
# c2rust + cargo scaffolding
# ---------------------------

def write_compile_commands(build_dir: Path, c_file: Path):
    compile_commands = [
        {
            "directory": str(build_dir.resolve()),
            "command": f"clang -I. -std=c11 -c {c_file.name}",
            "file": c_file.name,
        }
    ]
    (build_dir / "compile_commands.json").write_text(
        json.dumps(compile_commands, indent=2),
        encoding="utf-8",
    )


def run(cmd: list[str], cwd: Path):
    print(">>", " ".join(cmd))
    subprocess.run(cmd, cwd=str(cwd), check=True)


def find_cargo_root(rust_out: Path) -> Path:
    if (rust_out / "Cargo.toml").exists():
        return rust_out

    hits = list(rust_out.rglob("Cargo.toml"))
    if hits:
        hits.sort(key=lambda p: len(p.parts))
        return hits[0].parent

    return rust_out


def ensure_cargo_binary_project(proj_dir: Path):
    (proj_dir / "src").mkdir(exist_ok=True)

    cargo_toml = proj_dir / "Cargo.toml"
    if not cargo_toml.exists():
        cargo_toml.write_text(
            """[package]
name = "transpiled"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[dependencies]

[build-dependencies]
cc = "1"
""",
            encoding="utf-8",
        )
    else:
        text = cargo_toml.read_text(encoding="utf-8")
        if re.search(r'^\s*build\s*=\s*".*"', text, flags=re.MULTILINE) is None:
            text = re.sub(r"(\[package\]\s*)", r'\1build = "build.rs"\n', text, count=1)
        if "[build-dependencies]" not in text:
            text += "\n[build-dependencies]\ncc = \"1\"\n"
        elif re.search(r'^\s*cc\s*=\s*".*"', text, flags=re.MULTILINE) is None:
            text = re.sub(r"(\[build-dependencies\]\s*)", r"\1cc = \"1\"\n", text, count=1)
        cargo_toml.write_text(text, encoding="utf-8")

    # ensure build.rs exists in project root
    (proj_dir / "build.rs").write_text(
        """fn main() {
    cc::Build::new()
        .file("safe.c")
        .include(".")
        .compile("safe");
}
""",
        encoding="utf-8",
    )


def move_transpiled_rs_into_src(proj_dir: Path):
    src_dir = proj_dir / "src"
    src_dir.mkdir(exist_ok=True)

    for rs in proj_dir.glob("*.rs"):
        if rs.name == "build.rs":
            continue
        (src_dir / rs.name).write_text(rs.read_text(encoding="utf-8"), encoding="utf-8")
        rs.unlink()


def find_transpiled_module_file(proj_dir: Path) -> Path:
    src_dir = proj_dir / "src"
    if (src_dir / "unsafe.rs").exists():
        return src_dir / "unsafe.rs"

    candidates = [p for p in src_dir.glob("*.rs") if p.name != "main.rs"]
    if len(candidates) == 1:
        return candidates[0]

    for p in candidates:
        t = p.read_text(encoding="utf-8", errors="ignore")
        if 'fn main' in t or 'extern "C" fn main' in t:
            return p

    raise SystemExit("Could not identify the transpiled module (.rs).")


def rename_c_main(transpiled_rs: Path) -> tuple[str, bool]:
    """
    Rename transpiled main to c_main.
    Handles:
      - pub fn main() { ... }
      - fn main() { ... }
      - extern "C" fn main(...) -> ...
    Returns (kind, changed), kind in {"argc_argv","returns_int","unit"}.
    """
    text = transpiled_rs.read_text(encoding="utf-8")

    pat_args = re.compile(
        r'(\bextern\s+"C"\s+fn\s+)main(\s*\(\s*mut\s+argc\s*:\s*[^,]+,\s*mut\s+argv\s*:\s*\*mut\s*\*mut\s*[^)]+\)\s*->\s*[^ {]+)'
    )
    pat_void_ret = re.compile(
        r'(\bextern\s+"C"\s+fn\s+)main(\s*\(\s*\)\s*->\s*[^ {]+)'
    )
    pat_rust_main = re.compile(r'(\b(?:pub\s+)?fn\s+)main(\s*\()')

    kind = "unit"
    changed = False

    if pat_args.search(text):
        text = pat_args.sub(r"\1c_main\2", text, count=1)
        kind = "argc_argv"
        changed = True
    elif pat_void_ret.search(text):
        text = pat_void_ret.sub(r"\1c_main\2", text, count=1)
        kind = "returns_int"
        changed = True
    elif pat_rust_main.search(text):
        text = pat_rust_main.sub(r"\1c_main\2", text, count=1)
        kind = "unit"
        changed = True

    if changed:
        transpiled_rs.write_text(text, encoding="utf-8")
    return kind, changed


def write_rust_wrapper_main(proj_dir: Path, mod_name: str, kind: str):
    main_rs = proj_dir / "src" / "main.rs"

    if kind == "argc_argv":
        main_rs.write_text(
            f"""use std::ffi::CString;
use std::os::raw::c_char;

mod {mod_name};

fn main() {{
    let args: Vec<CString> = std::env::args()
        .map(|a| CString::new(a).expect("CString::new failed"))
        .collect();

    let mut c_argv: Vec<*mut c_char> = args.iter().map(|s| s.as_ptr() as *mut c_char).collect();
    c_argv.push(std::ptr::null_mut());

    let argc = args.len() as i32;
    let code = unsafe {{ {mod_name}::c_main(argc, c_argv.as_mut_ptr()) }};
    std::process::exit(code as i32);
}}
""",
            encoding="utf-8",
        )
    elif kind == "returns_int":
        main_rs.write_text(
            f"""mod {mod_name};

fn main() {{
    let code = unsafe {{ {mod_name}::c_main() }};
    std::process::exit(code as i32);
}}
""",
            encoding="utf-8",
        )
    else:
        # unit return ()
        main_rs.write_text(
            f"""mod {mod_name};

fn main() {{
    unsafe {{ {mod_name}::c_main() }};
}}
""",
            encoding="utf-8",
        )


# ---------------------------
# Main
# ---------------------------

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--input", required=True)
    ap.add_argument("--config", required=True)
    ap.add_argument("--out", default="workdir")
    ap.add_argument("--c2rust", default="c2rust")
    args = ap.parse_args()

    input_c = Path(args.input).resolve()
    cfg = json.loads(Path(args.config).read_text(encoding="utf-8"))
    unsafe_names = set(cfg.get("unsafe", []))

    work = Path(args.out).resolve()
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True, exist_ok=True)

    safe_funcs, unsafe_funcs = split_c_file(input_c, unsafe_names, work)
    print("SAFE kept in C:", safe_funcs)
    print("UNSAFE transpiled to Rust:", unsafe_funcs)

    write_compile_commands(work, work / "unsafe.c")

    rust_out = work / "rust_out"
    rust_out.mkdir(parents=True, exist_ok=True)

    run(
        [args.c2rust, "transpile", "--output-dir", str(rust_out), str(work / "compile_commands.json")],
        cwd=work,
    )

    proj_root = find_cargo_root(rust_out)

    # Copy safe artifacts into the cargo root (where build.rs will look)
    shutil.copy2(work / "safe.c", proj_root / "safe.c")
    shutil.copy2(work / "safe.h", proj_root / "safe.h")

    ensure_cargo_binary_project(proj_root)
    move_transpiled_rs_into_src(proj_root)

    transpiled_rs = find_transpiled_module_file(proj_root)

    # Avoid Rust keyword "unsafe"
    unsafe_mod = proj_root / "src" / "unsafe_mod.rs"
    if transpiled_rs.name != "unsafe_mod.rs":
        transpiled_rs.replace(unsafe_mod)

    kind, changed = rename_c_main(unsafe_mod)
    if not changed:
        print("WARN: Could not find a `main` function to rename in transpiled Rust file.")

    write_rust_wrapper_main(proj_root, "unsafe_mod", kind)

    print("\nDone.")
    print(f"Cargo project root: {proj_root}")
    print("Build:")
    print(f"  cd {proj_root}")
    print("  cargo build")
    print("Run:")
    print("  cargo run")


if __name__ == "__main__":
    main()
