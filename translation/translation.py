import os
import subprocess
import ollama
import csv
import time
import re
import sys

# Importiere die Funktion aus deiner create_prompt.py
try:
    from .create_prompt import build_final_prompt
except ImportError:
    from create_prompt import build_final_prompt

# --- KONFIGURATION ---
MODEL = "deepseek-coder:33b"
C_SOURCE_BASE = "C-projects" 
MAX_RETRIES = 5
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

def map_functions_to_files(rust_src_path):
    """Scannt das Projekt und erstellt ein Mapping: {funktion_name: datei_pfad}"""
    func_map = {}
    for root, _, files in os.walk(rust_src_path):
        for file in files:
            if file.endswith(".rs") and file != "lib.rs":
                path = os.path.join(root, file)
                with open(path, 'r', encoding='utf-8') as f:
                    content = f.read()
                # Findet Funktionsnamen (auch mit pub/unsafe/extern Attributen)
                names = re.findall(r'fn\s+([a-zA-Z0-9_]+)\s*\(', content)
                for name in names:
                    func_map[name] = path
    return func_map

def get_function_range(content, func_name):
    """Findet Start- und End-Index einer Funktion im Rust-Code."""
    # Finde den Start der Funktion
    pattern = r'(?P<start>(?:#\[[^\]]+\]\s*)*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+' + re.escape(func_name) + r'\s*\()'
    match = re.search(pattern, content)
    if not match:
        return None, None

    start_idx = match.start()
    
    # Suche das Ende der Signatur (entweder ein '{' oder ein ';')
    signature_end = start_idx
    while signature_end < len(content):
        if content[signature_end] == '{':
            break
        if content[signature_end] == ';':
            # Es ist nur eine Deklaration (extern fn foo();), überspringen!
            return None, None
        signature_end += 1

    if signature_end >= len(content):
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

def run_cargo_check(project_root):
    try:
        result = subprocess.run(["cargo", "check"], cwd=project_root, capture_output=True, text=True, timeout=300)
        return result.returncode == 0, result.stderr
    except Exception as e:
        return False, f"Error: {str(e)}"

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

def process_single_function(func_name, file_path, project_name, cargo_root):
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
    print(c_file)
    
    # Prompt erstellen (erhält nur das Snippet!)
    base_prompt = build_final_prompt(c_file, func_name, unsafe_snippet)

    last_err = ""
    last_sent_prompt = ""
    func_identifier = f"{os.path.basename(file_path)}:{func_name}"
    
    for attempt in range(1, MAX_RETRIES + 1):
        current_prompt = base_prompt + (f"\n\n### COMPILER ERROR:\n{last_err}" if last_err else "")
        last_sent_prompt = current_prompt
        
        safe_snippet = ask_llm(current_prompt)
        if not safe_snippet: continue

        # Nur den Funktions-Teil in der Datei ersetzen
        updated_content = file_content[:start_idx] + safe_snippet + file_content[end_idx:]
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(updated_content)

        is_ok, err = run_cargo_check(cargo_root)
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

    if not os.path.exists(rust_src):
        print(f"Fehler: {rust_src} nicht gefunden.")
        return

    # HIER war der Fehler: Die Funktion muss definiert sein!
    func_to_file = map_functions_to_files(rust_src)
    
    # Reihenfolge festlegen
    order = function_order if function_order else list(func_to_file.keys())

    print(f"--- Starte Übersetzung von {len(order)} Funktionen ---")

    for func_name in order:
        if func_name in func_to_file:
            if func_name == "main":
                print("  [*] Spezieller Fall: Main ersetzen mit main_0, um die ursprüngliche main-Funktion zu erhalten.")
                process_single_function("main_0", func_to_file[func_name], project_name, cargo_root)
            else:
                process_single_function(func_name, func_to_file[func_name], project_name, cargo_root)
        else:
            print(f"  [!] Warnung: '{func_name}' nicht im Projekt gefunden.")