#!/usr/bin/env python3
import argparse
import json
import re
import shutil
import subprocess
from pathlib import Path

# --- Hilfsfunktionen für das Parsen ---

def strip_comments_and_strings(s: str) -> str:
    out = list(s)
    i, n = 0, len(s)
    state = "code"
    while i < n:
        c = s[i]
        nxt = s[i + 1] if i + 1 < n else ""
        if state == "code":
            if c == "/" and nxt == "/": out[i] = out[i+1] = " "; i += 2; state = "line_comment"; continue
            if c == "/" and nxt == "*": out[i] = out[i+1] = " "; i += 2; state = "block_comment"; continue
            if c == "'": out[i] = " "; i += 1; state = "sq"; continue
            if c == '"': out[i] = " "; i += 1; state = "dq"; continue
            i += 1; continue
        if state == "line_comment":
            out[i] = " "
            if c == "\n": state = "code"
            i += 1; continue
        if state == "block_comment":
            out[i] = " "
            if c == "*" and nxt == "/": out[i+1] = " "; i += 2; state = "code"; continue
            i += 1; continue
        if state in ("sq", "dq"):
            out[i] = " "
            if c == "\\" and i + 1 < n: out[i+1] = " "; i += 2; continue
            if (state == "sq" and c == "'") or (state == "dq" and c == '"'): state = "code"
            i += 1; continue
    return "".join(out)

def find_matching_brace(masked: str, open_idx: int) -> int:
    depth = 0
    for i in range(open_idx, len(masked)):
        if masked[i] == "{": depth += 1
        elif masked[i] == "}":
            depth -= 1
            if depth == 0: return i
    return -1

def get_function_info(src: str):
    """Findet alle Funktionen und gibt ihre Bereiche (start, open_brace, end) zurück."""
    masked = strip_comments_and_strings(src)
    funcs = {}
    # Pattern für Funktionsnamen
    for m in re.finditer(r"\b([A-Za-z_]\w*)\s*\(", masked):
        name = m.group(1)
        if name in ("if","for","while","switch","return","sizeof"): continue
        
        idx = m.start()
        line_start = max(0, masked.rfind("\n", 0, idx) + 1)
        
        # Suche die öffnende Klammer des Körpers
        i = m.end()
        paren = 1
        while i < len(masked) and paren > 0:
            if masked[i] == "(": paren += 1
            elif masked[i] == ")": paren -= 1
            i += 1
        if paren != 0: continue
        
        brace_open = i
        while brace_open < len(masked) and masked[brace_open].isspace():
            brace_open += 1
        
        if brace_open < len(masked) and masked[brace_open] == "{":
            brace_close = find_matching_brace(masked, brace_open)
            if brace_close != -1:
                funcs[name] = (line_start, brace_open, brace_close)
    return funcs

def create_context_split(src: str, funcs_info: dict, target_names: list, is_unsafe_file: bool):
    """Erzeugt eine Version der Datei, in der nur target_names ihre Koerper behalten.

    Wichtig fuer Hybrid-Linking: jede Funktion, deren Body wir nach Rust ausgelagert
    haben, darf in der C-Datei nicht mehr `static` sein. Der Body kommt jetzt aus
    der Rust-Crate (libhybrid_project.a) und der Linker muss das Symbol von dort
    aufloesen koennen — `static` macht das Symbol TU-lokal und versteckt es
    sowohl vor dem Rust-Code als auch vor anderen C-TUs.

    Der bisherige Code stripped `static` nur fuer `is_unsafe_file=True`. Das war
    falsch: auch in safe_*.c werden Bodies entfernt (fuer die unsafe-Funktionen
    der Datei) und auch die muessen extern werden. Ohne das schlaegt das
    Hybrid-Linking spaeter fehl mit 'undefined reference to <fn>'.
    """
    # Sortiere von hinten nach vorne, um Indizes beim Ersetzen nicht zu korrumpieren
    sorted_items = sorted(funcs_info.items(), key=lambda x: x[1][0], reverse=True)

    current_src = src
    for name, (start, brace_open, end) in sorted_items:
        if name not in target_names:
            # Body removed -> Funktion ist jetzt in Rust definiert.
            # Strip alle Whitespace-Varianten von `static `: `static ` (Space),
            # `static\t` (Tab), Tabbed im Original-K&R-Stil.
            header = current_src[start:brace_open]
            header = re.sub(r'^\s*static\s+', '', header)
            # Innerhalb des headers (z.B. nach Attributen) ggf. weitere static
            # entfernen — selten, aber defensiv.
            header = re.sub(r'\bstatic\s+', '', header)

            current_src = current_src[:start] + header + ";" + current_src[end+1:]
    return current_src

# --- Management ---

def process_project(in_dir: Path, unsafe_cfg: dict, out_dir: Path):
    """Splits each `.c` into a safe-helpers and an unsafe variant. Preserves
    the project's subdirectory layout in `out_dir` so that projects with
    nested source trees (e.g. libtiff with `libtiff/`, `tools/`, `port/`,
    `contrib/`) work correctly.

    Output convention:
      <subdir>/<file>.c          -> <subdir>/safe_<file>.c   (safe helpers)
                                  -> <subdir>/<file>.c        (unsafe target)

    Critically: the `safe_` prefix is applied to the filename only, NOT to
    the directory part. Earlier versions naively prepended `safe_` to the
    full relative path, which produced paths like `safe_libtiff/tif_dir.c`
    where `safe_libtiff` is not an actual directory — those writes either
    silently land in the wrong place or fail with "no such file or
    directory". Manifests now store full relative paths (forward-slash
    separators) so the build script and downstream link step can resolve
    them.
    """
    c_files = list(in_dir.rglob("*.c"))
    safe_manifest, unsafe_manifest = [], []

    for c_file in c_files:
        rel_path = str(c_file.relative_to(in_dir)).replace("\\", "/")
        unsafe_funcs_list = unsafe_cfg.get(rel_path, [])
        src = c_file.read_text(encoding="utf-8", errors="ignore")

        funcs_info = get_function_info(src)

        if not funcs_info: continue

        f_unsafe = [n for n in funcs_info if n in unsafe_funcs_list]
        f_safe = [n for n in funcs_info if n not in unsafe_funcs_list]

        # Split rel_path into directory and filename so that the `safe_`
        # prefix lands on the basename, not the path.
        rel_dir, _, file_name = rel_path.rpartition("/")
        # `rel_dir` is empty for files at project root; rpartition leaves the
        # delimiter out for that case.

        if f_safe:
            safe_rel = f"{rel_dir}/safe_{file_name}" if rel_dir else f"safe_{file_name}"
            safe_path = out_dir / safe_rel
            safe_path.parent.mkdir(parents=True, exist_ok=True)
            content = create_context_split(src, funcs_info, f_safe, is_unsafe_file=False)
            safe_path.write_text(content, encoding="utf-8")
            safe_manifest.append(safe_rel)

        if f_unsafe:
            unsafe_path = out_dir / rel_path
            unsafe_path.parent.mkdir(parents=True, exist_ok=True)
            content = create_context_split(src, funcs_info, f_unsafe, is_unsafe_file=True)
            unsafe_path.write_text(content, encoding="utf-8")
            unsafe_manifest.append(unsafe_path)

    return safe_manifest, unsafe_manifest

def main(args_list=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--input-dir", required=True)
    ap.add_argument("--config", required=True)
    ap.add_argument("--out", default="workdir")
    args = ap.parse_args(args_list)

    in_dir, work = Path(args.input_dir).resolve(), Path(args.out).resolve()
    config_data = json.loads(Path(args.config).read_text())
    unsafe_cfg = config_data.get("unsafe", {})

    if work.exists(): shutil.rmtree(work)
    work.mkdir(parents=True)

    # 1. Header kopieren
    for h in in_dir.rglob("*.h"):
        target = work / h.relative_to(in_dir)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(h, target)
        
   

    # 2. Dateien splitten
    safe_files, unsafe_paths = process_project(in_dir, unsafe_cfg, work)


    # 3. Compile Commands — use relative paths so c2rust resolves files
    # from any subdirectory under `work`. Each translation unit gets its
    # own `-I` for its directory plus a project-root `-I.`, so headers in
    # both layouts (project-root and per-subdir) resolve.
    # Also collect the set of all directories that contain headers so we
    # can add them as include paths globally.
    header_dirs = sorted({
        str(h.parent.relative_to(in_dir)).replace("\\", "/")
        for h in in_dir.rglob("*.h")
    })
    extra_includes = " ".join(
        f'-I"{d}"' if d else '-I.'
        for d in header_dirs
    )

    cmds = []
    for p in unsafe_paths:
        rel = str(p.relative_to(work)).replace("\\", "/")
        own_dir = str(p.parent.relative_to(work)).replace("\\", "/")
        own_include = f'-I"{own_dir}"' if own_dir and own_dir != "." else ""
        cmds.append({
            "directory": str(work),
            "command": (
                f"clang -I. {own_include} {extra_includes} "
                f"-std=gnu89 -fcommon -Wno-everything -c {rel}"
            ).strip(),
            "file": rel,
        })
    (work / "compile_commands.json").write_text(json.dumps(cmds, indent=2))

    # 4. Transpilieren
    rust_out = work / "rust_out"
    rust_out.mkdir()
    print(">> Starte c2rust Transpilation mit Kontext-Erhalt...")
    subprocess.run(["c2rust", "transpile","--emit-build-files" ,"--output-dir", str(rust_out), "compile_commands.json"], cwd=work, check=True)

    # 5. Cargo Setup
    (rust_out / "src").mkdir(exist_ok=True)
    if safe_files:
        # `safe_files` carries forward-slash relative paths that include
        # subdirectory components. cc-rs accepts these as-is.
        file_calls = "".join([f'.file("{f}")' for f in safe_files])
        # Pass every header directory as an include path so the safe_*.c
        # files in any subdir can resolve project-wide headers.
        include_calls = "".join([
            f'.include("{d}")' if d else '.include(".")'
            for d in (header_dirs or [""])
        ])
        if not include_calls:
            include_calls = '.include(".")'
        build_script_content = f"""fn main() {{
        cc::Build::new()
            {file_calls}
            {include_calls}
            .flag("-std=c99")
            .flag("-fcommon")
            .warnings(false)
            .compile("c_parts");
    }}"""
    else:
        # An empty main function so the build script exists but does nothing
        build_script_content = "fn main() {}"

    (rust_out / "build.rs").write_text(build_script_content, encoding="utf-8")
    (rust_out / "Cargo.toml").write_text("""[package]\nname = "hybrid_project"\nversion = "0.1.0"\nedition = "2021"\nbuild = "build.rs"\n[build-dependencies]\ncc = "1"\n[lib]\nname="hybrid_project"\npath= "lib.rs"\ncrate-type = ["staticlib", "rlib"]\n """, encoding="utf-8")

    # Copy safe_*.c files into rust_out, preserving subdirectory layout.
    for f in safe_files:
        src = work / f
        dst = rust_out / f
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
    for h in in_dir.rglob("*.h"):
        target = rust_out / h.relative_to(in_dir)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(h, target)

    print(f"\nMigration erfolgreich! Projekt: {rust_out}")

if __name__ == "__main__":
    main()