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
    """Erzeugt eine Version der Datei, in der nur target_names ihre Körper behalten."""
    # Sortiere von hinten nach vorne, um Indizes beim Ersetzen nicht zu korrumpieren
    sorted_items = sorted(funcs_info.items(), key=lambda x: x[1][0], reverse=True)
    
    current_src = src
    for name, (start, brace_open, end) in sorted_items:
        if name not in target_names:
            # Ersetze Körper { ... } durch ;
            # Wenn es eine statische Funktion ist, machen wir sie 'extern', damit sie im Linker sichtbar bleibt
            header = current_src[start:brace_open]
            if is_unsafe_file:
                header = header.replace("static ", "extern ")
            
            current_src = current_src[:start] + header + ";" + current_src[end+1:]
    return current_src

# --- Management ---

def process_project(in_dir: Path, unsafe_cfg: dict, out_dir: Path):
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
        
        base_id = rel_path.replace("/", "_").replace(".", "_")

        if f_safe:
            path = out_dir / f"{base_id}_safe.c"
            content = create_context_split(src, funcs_info, f_safe, is_unsafe_file=False)
            path.write_text(content, encoding="utf-8")
            safe_manifest.append(path.name)

        if f_unsafe:
            path = out_dir / f"{base_id}_unsafe.c"
            content = create_context_split(src, funcs_info, f_unsafe, is_unsafe_file=True)
            path.write_text(content, encoding="utf-8")
            unsafe_manifest.append(path)

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

    # 3. Compile Commands
    cmds = []
    for p in unsafe_paths:
        cmds.append({
            "directory": str(work),
            "command": f"clang -I. -std=gnu89 -fcommon -Wno-everything -c {p.name}",
            "file": p.name
        })
    (work / "compile_commands.json").write_text(json.dumps(cmds, indent=2))

    # 4. Transpilieren
    rust_out = work / "rust_out"
    rust_out.mkdir()
    print(">> Starte c2rust Transpilation mit Kontext-Erhalt...")
    subprocess.run(["c2rust", "transpile","--emit-build-files" ,"--output-dir", str(rust_out), "compile_commands.json"], cwd=work, check=True)

    # 5. Cargo Setup
    (rust_out / "src").mkdir(exist_ok=True)
    f_list = ", ".join([f'"{f}"' for f in safe_files])
    (rust_out / "build.rs").write_text(f"""fn main() {{
    cc::Build::new().files([{f_list}]).include(".").flag("-std=gnu89").flag("-fcommon").warnings(false).compile("c_parts");
}}""", encoding="utf-8")
    (rust_out / "Cargo.toml").write_text("""[package]\nname = "hybrid_project"\nversion = "0.1.0"\nedition = "2021"\nbuild = "build.rs"\n[build-dependencies]\ncc = "1"\n""", encoding="utf-8")
    
    for f in safe_files: shutil.copy2(work / f, rust_out / f)
    for h in in_dir.rglob("*.h"):
        target = rust_out / h.relative_to(in_dir)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(h, target)

    print(f"\nMigration erfolgreich! Projekt: {rust_out}")

if __name__ == "__main__":
    main()