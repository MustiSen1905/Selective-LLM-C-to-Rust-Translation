import subprocess
import sys
import os
import re
import json

def load_unsafe_functions(json_path):
    """Lädt die Funktionsnamen aus der JSON-Datei in ein Set."""
    try:
        with open(json_path, 'r') as f:
            data = json.load(f)
            
        # Wir extrahieren alle Funktionsnamen aus allen Dateilisten unter "unsafe"
        unsafe_set = set()
        for functions in data.get("unsafe", {}).values():
            for func in functions:
                unsafe_set.add(func)
        return unsafe_set
    except (FileNotFoundError, json.JSONDecodeError) as e:
        print(f"[!] Fehler beim Laden der JSON: {e}")
        return set()

def run_analysis(c_project_dir, json_path="unsafe_functions.json"):
    project_path = os.path.abspath(c_project_dir)
    # 1. Lade die Whitelist aus der JSON
    allowed_functions = load_unsafe_functions(json_path)
    
    function_order = []
    
    try:
        process = subprocess.Popen(
            ["cargo", "run", "--manifest-path", "analysis/graph-builder/Cargo.toml", "--", project_path],
            stdout=subprocess.PIPE,
            stderr=sys.stderr, 
            text=True
        )

        stdout, _ = process.communicate()

        if process.returncode != 0:
            return []

        if "ANALYSIS_START" in stdout:
            content = stdout.split("ANALYSIS_START")[1].split("ANALYSIS_END")[0]
            lines = content.strip().split("\n")
            
            for line in lines:
                # Extrahiert den Namen nach 'FUNC:'
                match = re.search(r"FUNC:\s*([\w\d_]+)", line)
                if match:
                    func_name = match.group(1)
                    
                    # FILTER: Nur übernehmen, wenn in der JSON-Liste enthalten
                    if func_name in allowed_functions:
                        function_order.append(func_name)
            
    except Exception as e:
        print(f"[!] Fehler: {e}")
        return []

    return function_order

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python main.py <c_project_dir>")
    else:
        # Falls die JSON woanders liegt, hier den Pfad anpassen
        functions = run_analysis(sys.argv[1], "unsafe_functions.json")
        print(functions)