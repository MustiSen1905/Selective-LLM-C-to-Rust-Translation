import json
import os
import subprocess
import ollama
import csv
import time
import re
import sys
import entities.FunctionObject as FunctionObject

# Importiere die Funktion aus deiner create_prompt.py
try:
    from .create_prompt import build_final_prompt, build_struct_prompt
except ImportError:
    from create_prompt import build_final_prompt, build_struct_prompt

# --- KONFIGURATION ---
MODEL = "deepseek-coder:33b"
#"deepseek-coder:33b"
C_SOURCE_BASE = "C-projects" 
MAX_RETRIES = 3
LOG_FILE = "bachelor_evaluation_results.csv"

client = ollama.Client(host='http://localhost:11434')

def log_result(project, func_info, status, attempts, prompt, error=""):
    """Schreibt die Ergebnisse inklusive des verwendeten Prompts in eine CSV-Datei."""
    file_exists = os.path.isfile(LOG_FILE)
    with open(LOG_FILE, 'a', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        if not file_exists:
            writer.writerow(["Timestamp", "Project", "Function_Info", "Status", "Attempts", "Prompt", "ErrorMessage"])
        
        clean_error = error.replace('\n', ' ').replace('\r', '')[:500]
        clean_prompt = prompt.replace('\r', '') 
        
        writer.writerow([
            time.strftime("%Y-%m-%d %H:%M:%S"), 
            project, 
            func_info, 
            status, 
            attempts, 
            clean_prompt,
            clean_error
        ])

def map_c_functions_to_rust_files(function_objects, rust_src_path):
    """
    Erstellt eine Map {funktion_name: rust_datei_pfad}.
    Nutzt die Liste der FunctionObjects, um gezielt nach den .rs-Pendanten zu suchen.
    """
    func_map = {}
    # 1. Wir erstellen zuerst einen Index aller vorhandenen Rust-Dateien im Projekt
    # Key: "main", Value: "/pfad/zu/main.rs"
    rust_file_index = {}
    for root, _, files in os.walk(rust_src_path):
        for file in files:
            if file.endswith(".rs"):
                base_name = os.path.splitext(file)[0]
                rust_file_index[base_name] = os.path.join(root, file)

    # 2. Wir gehen die Liste deiner FunctionObjects durch
    print("Test")
    
    for obj in function_objects:
        # Extrahiere Basisnamen der C-Datei (z.B. "logic" aus "logic.c")
        c_base_name = os.path.splitext(obj.file)[0]
        
        
        # Prüfen, ob wir eine passende Rust-Datei im Index haben
        if c_base_name in rust_file_index:
            rust_path = rust_file_index[c_base_name]
            # Mapping: Funktionsname -> Pfad zur gefundenen .rs Datei
            func_map[obj.name] = rust_path
        else:
            # Optional: Falls keine passende .rs Datei gefunden wurde
            print(f"Warnung: Keine Rust-Datei für {obj.file} (Funktion: {obj.name}) gefunden.")

    print(func_map)
    return func_map

def get_function_range(content, func_name):
    """Findet Start- und End-Index einer Funktion im Rust-Code."""
    # Finde den Start der Funktion
    pattern = r'(?P<start>(?:#\[[^\]]+\]\s*)*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+' + re.escape(func_name) + r'\s*\()'
    match = re.search(pattern, content)
    if not match:
        print("True1")
        return None, None

    start_idx = match.start()
    
    # Suche das Ende der Signatur (entweder ein '{' oder ein ';')
    signature_end = start_idx
    while signature_end < len(content):
        if content[signature_end] == '{':
            break
        if content[signature_end] == ';':
            # Es ist nur eine Deklaration (extern fn foo();), überspringen!
            print("True2")
            return None, None
        signature_end += 1

    if signature_end >= len(content):
        print("True3")
        return None, None
    
    # Jetzt zählen wir die Klammern, um das Ende der Funktion zu finden
    brace_count = 1
    idx = signature_end + 1
    while brace_count > 0 and idx < len(content):
        if content[idx] == '{':
            brace_count += 1
        elif content[idx] == '}':
            brace_count -= 1
        idx += 1
    
    return start_idx, idx

def run_cargo_check_for_function(cargo_root, target_file, start_line, end_line):
    """
    Führt cargo check aus und filtert ALLE Fehler heraus, 
    die NICHT in der aktuell bearbeiteten Funktion (Zeile X bis Y) liegen.
    """
    result = subprocess.run(
        ["cargo", "check", "--message-format=json"],
        cwd=cargo_root,
        capture_output=True,
        text=True
    )

    relevant_errors = []
    
    for line in result.stdout.splitlines():
        if not line.startswith('{'):
            continue
            
        try:
            msg = json.loads(line)
            if msg.get("reason") == "compiler-message":
                error_data = msg["message"]
                
                if error_data.get("level") not in ["error", "fatal"]:
                    continue
                
                is_in_target_function = False
                for span in error_data.get("spans", []):
                    # 1. Check: Ist es die richtige Datei?
                    if span.get("is_primary") and target_file in span.get("file_name", ""):
                        error_line = span.get("line_start", 0)
                        
                        # 2. Check: Ist es die richtige Funktion? 
                        # Wir geben einen Puffer von +/- 2 Zeilen (z.B. für #[no_mangle] Attribute)
                        if (start_line - 2) <= error_line <= (end_line + 2):
                            is_in_target_function = True
                            break
                
                if is_in_target_function:
                    relevant_errors.append(error_data.get("rendered", ""))
                    
        except json.JSONDecodeError:
            continue

    if not relevant_errors:
        return True, ""
    else:
        # Gebe maximal 3 Fehler zurück, um die KI nicht mit Folgefehlern zu überlasten
        return False, "\n".join(relevant_errors[:3])

def ask_llm(prompt):
    try:
        response = client.generate(model=MODEL, prompt=prompt)
        content = response['response']
        if "```rust" in content:
            content = content.split("```rust")[1].split("```")[0].strip()
        elif "```" in content:
            content = content.split("```")[1].split("```")[0].strip()
        return content
    except Exception as e:
        print(f"  !! LLM Error: {e}")
        return None
    
def refactor_structs_in_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # WICHTIGER SCHUTZ: Wenn die Safe-Structs schon in der Datei sind, 
    # überspringen wir das Generieren, damit sie nicht doppelt vorkommen!
    if "pub struct Safe" in content:
        print(f"  [*] Überspringe Struct-Refactoring für {file_path} (Safe-Structs existieren bereits).")
        return content

    # Extrahiere alle Struct-Definitionen (vereinfacht)
    struct_pattern = r'(#\[repr\(C\)\]\s+)?pub struct (\w+) \{[^}]+\}'
    matches = list(re.finditer(struct_pattern, content))
    if not matches: return content

    unsafe_snippet = "\n\n".join([m.group(0) for m in matches])
    
    # Generiere Schatten-Strukturen
    raw_safe_structs = ask_llm(build_struct_prompt("Context", unsafe_snippet))
    
    # Bereinige den generierten Code
    safe_structs = sanitize_rust_code(raw_safe_structs)

    # Füge die Safe-Structs OBEN ein
    new_content = safe_structs + "\n\n" + content
    return new_content

import re

def sanitize_rust_code(code):
    """Bereinigt deterministisch Halluzinationen und Syntax-Fehler der KI."""
    if not code:
        return ""

    # 1. Entferne Markdown-Blöcke
    code = code.replace("```rust", "").replace("```", "").strip()

    # 2. Entferne Konversations-Geplapper am Anfang/Ende
    clean_lines = []
    for line in code.split('\n'):
        if re.match(r'^(The|Here|Note|This|I |You |It |If you|In order)', line):
            continue
        clean_lines.append(line)
    code = "\n".join(clean_lines)

    # 3. FIX E0774: Entferne derive Attribute über Typ-Aliasen
    code = re.sub(r'#\[derive\([^)]+\)\]\s+(?=pub type|type|pub const|const)', '', code)

    # 4. FIX Unicode: Ersetze '…' durch '...'
    code = code.replace("…", "...")

    # 5. FIX E0433 (CStr): Erzwinge absolute Pfade für CStr
    code = re.sub(r'(?<!std::ffi::)CStr::', 'std::ffi::CStr::', code)

    # 6. FIX (c_char/c_int): Erzwinge absolute Pfade
    code = re.sub(r'(?<!core::ffi::)(?<!std::os::raw::)\bc_char\b', 'core::ffi::c_char', code)
    code = re.sub(r'(?<!core::ffi::)(?<!std::os::raw::)\bc_int\b', 'core::ffi::c_int', code)

    # 7. FIX E0433 (ptr): Erzwinge absolute Pfade für ptr::null()
    code = re.sub(r'(?<!std::)\bptr::null', 'std::ptr::null', code)

    # 8. FIX: Aggressiver Use-Remover (löscht alles, was mit "use " beginnt)
    code_lines = []
    in_use_block = False
    for line in code.split('\n'):
        line_stripped = line.strip()
        if line_stripped.startswith("use ") and not line_stripped.endswith("!"):
            if not line_stripped.endswith(";"):
                in_use_block = True
            continue
        if in_use_block:
            if line_stripped.endswith(";"):
                in_use_block = False
            continue
        code_lines.append(line)
    code = "\n".join(code_lines)

    # 9. FIX: 'ustrchr' Halluzination zu 'strchr' korrigieren
    code = re.sub(r'\bustrchr\b', 'strchr', code)

    # 10. FIX: Entferne halluzinierte "extern C" Blöcke (die KI soll keine C-Funktionen neu deklarieren)
    # Entfernt #[link(name = "c")] extern "C" { ... }
    code = re.sub(r'(#\[link\(name\s*=\s*"c"\)\]\s*)?extern\s+"C"\s*\{[^}]+\}', '', code)

    # 11. FIX: Entferne halluzinierte Struct/Type-Definitionen (die KI darf NUR die Funktion zurückgeben!)
    # Sucht nach "pub struct X { ... }" oder "pub type X = Y;" und löscht sie aus dem Output
    code = re.sub(r'(#\[repr\(C\)\]\s*)?(#\[derive\([^)]+\)\]\s*)?pub struct \w+\s*\{[^}]+\}', '', code)
    code = re.sub(r'pub type \w+\s*=[^;]+;', '', code)

    # NEUER SCHUTZ-MECHANISMUS: Hat die KI überhaupt eine Funktion geschrieben?
    if "fn " not in code:
        print("  [!] KI hat keine gültige Funktion generiert (möglicherweise ein Refusal). Verwerfe Output.")
        return ""
    
    last_brace = code.rfind('}')
    if last_brace != -1:
        suffix = code[last_brace + 1 :].strip()
        # Wenn nach der letzten Klammer nur noch Fragmente wie '1];' oder '...' stehen
        if suffix and not any(suffix in su for su in ["fn ", "pub ", "impl "]):
            code = code[: last_brace + 1]

    if code.startswith("I'm sorry") or code.startswith("Apologies"):
        print("  [!] KI hat sich entschuldigt. Verwerfe Output.")
        return ""

    return code.strip()

def remove_duplicate_structs(rust_code):
    """
    Verhindert E0119, indem es doppelte Definitionen von C2RustUnnamed entfernt.
    """
    struct_pattern = r"pub struct (C2RustUnnamed\d*|__va_list_tag) \{.*?\}"
    seen_structs = set()
    clean_lines = []
    
    # Sehr simpler Parser, der nur die erste Definition eines Namens behält
    lines = rust_code.split('\n')
    skip_mode = False
    
    for line in lines:
        match = re.search(r"pub struct (C2RustUnnamed\w*)", line)
        if match:
            name = match.group(1)
            if name in seen_structs:
                skip_mode = True # Wir überspringen diese Definition
                continue
            seen_structs.add(name)
        
        if skip_mode and line.strip() == "}":
            skip_mode = False
            continue
            
        if not skip_mode:
            clean_lines.append(line)
            
    return "\n".join(clean_lines)

def process_single_function(func_name, file_path, project_name, cargo_root,safe_struct_context=""):
    """Extrahiert NUR die eine Funktion, lässt sie übersetzen und re-integriert sie."""
    print(f"[*] Analyse: {func_name} in {os.path.basename(file_path)}")
    
    with open(file_path, 'r', encoding='utf-8') as f:
        file_content = f.read()

    start_idx, end_idx = get_function_range(file_content, func_name)
    if start_idx is None:
        print(f"  [!] Überspringe: Funktion {func_name} im Rust-Code nicht gefunden.")
        return False

    unsafe_snippet = file_content[start_idx:end_idx]
    base_file_name = os.path.splitext(os.path.basename(file_path))[0]
    c_file = os.path.join(C_SOURCE_BASE, project_name, f"{base_file_name}.c")
    
    
    # Prompt erstellen (erhält nur das Snippet!)
    base_prompt = build_final_prompt(c_file, func_name, unsafe_snippet, safe_struct_context)

    
    
    last_err = ""
    last_sent_prompt = ""
    func_identifier = f"{os.path.basename(file_path)}:{func_name}"
    
    for attempt in range(1, MAX_RETRIES + 1):
        current_prompt = base_prompt + (f"\n\n### COMPILER ERROR:\n{last_err}" if last_err else "")
        last_sent_prompt = current_prompt
        
        raw_snippet = ask_llm(current_prompt)
        raw_snippet = remove_duplicate_structs(raw_snippet)
        safe_snippet = sanitize_rust_code(raw_snippet)
        if not safe_snippet: continue

        # Nur den Funktions-Teil in der Datei ersetzen
        updated_content = file_content[:start_idx] + safe_snippet + file_content[end_idx:]
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(updated_content)

        start_line = updated_content[:start_idx].count('\n') + 1
        # Die neue Endzeile ist die Startzeile + die Anzahl der Zeilen im neuen Snippet
        end_line = start_line + safe_snippet.count('\n')

        target_filename = os.path.basename(file_path)
        
        # Übergib start_line und end_line an die Check-Funktion
        is_ok, err = run_cargo_check_for_function(cargo_root, target_filename, start_line, end_line)
        
        if is_ok:
            print(f"  [+] Erfolg: {func_name} (Versuch {attempt})")
            log_result(project_name, func_identifier, "SUCCESS", attempt, last_sent_prompt)
            return True
        else:
            last_err = err
            print(f"  [-] Fehler in Versuch {attempt}.")
    
    # Reset bei Fehlschlag
    log_result(project_name, func_identifier, "FAILED", MAX_RETRIES, last_sent_prompt, last_err)
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(file_content)
    return False

def main(project_dir, function_order=None):
    target_path = os.path.abspath(project_dir)
    project_name = os.path.basename(target_path.rstrip(os.sep))
    rust_src = os.path.join(target_path, "rust_out", "src")
    cargo_root = os.path.join(target_path, "rust_out")
    
    processed_files = set()
    file_struct_registry = {}

    if not os.path.exists(rust_src):
        print(f"Fehler: {rust_src} nicht gefunden.")
        return

    # HIER war der Fehler: Die Funktion muss definiert sein!
    func_to_file = map_c_functions_to_rust_files(function_order, rust_src)
    
    for func in function_order:
        file_path = func_to_file.get(func.name)
        if file_path and file_path not in processed_files:
            new_content = refactor_structs_in_file(file_path)
            new_content = remove_duplicate_structs(new_content) # Doppelte Structs entfernen, um E0119 zu verhindern
            new_content = sanitize_rust_code(new_content) # Bereinige den Code von Erklärungen und Markdown-Überbleibseln
            # Extrahiere die neuen Structs für den Prompt-Kontext
            # (In einer echten Pipeline könntest du sie hier separat speichern)
            struct_defs = new_content.split("\n\n")[0] 
            file_struct_registry[file_path] = struct_defs
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            processed_files.add(file_path)
    
    # Reihenfolge festlegen
    order = function_order if function_order else list(func_to_file.keys())

    print(f"--- Starte Übersetzung von {len(order)} Funktionen ---")

    for func in order:
        if func.name in func_to_file:
            path = func_to_file[func.name]
            if func.name == "main":
                print("  [*] Spezieller Fall: Main ersetzen mit main_0, um die ursprüngliche main-Funktion zu erhalten.")
                process_single_function("main_0", func_to_file[func.name], project_name, cargo_root,safe_struct_context=file_struct_registry.get(path, ""))
            else:
                process_single_function(func.name, func_to_file[func.name], project_name, cargo_root,safe_struct_context=file_struct_registry.get(path, ""))
        else:
            print(f"  [!] Warnung: '{func.name}' nicht im Projekt gefunden.")