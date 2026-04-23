import os
import subprocess
import json
import re
import sys

def analyze_ownership(project_dir):
    # Pfad normalisieren, um saubere Namen zu erhalten
    project_dir = os.path.normpath(project_dir)
    project_name = os.path.basename(project_dir)
    
    if not os.path.exists(project_dir):
        print(f"❌ Fehler: Verzeichnis '{project_dir}' existiert nicht.")
        return None

    data = {
        "analysis_target": project_name,
        "files": {}
    }

    c_files = [f for f in os.listdir(project_dir) if f.endswith(('.c', '.h'))]

    for filename in c_files:
        filepath = os.path.join(project_dir, filename)
        # 'replace' hilft bei alten Dateien mit ungewöhnlichem Encoding
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()

        file_info = {
            "functions": [],
            "allocations": [],
            "deallocations": [],
            "global_vars": [],
            "cppcheck_reports": []
        }

        # 1. Funktions-Signaturen extrahieren (Wichtig für Borrowing vs. Ownership)
        # Sucht nach: Typ Name(Args)
        func_matches = re.finditer(r"\b(int|char|void|unsigned|float|double)\s*\*?\s+(\w+)\s*\(([^)]*)\)\s*\{", content)
        for m in func_matches:
            args = m.group(3).strip()
            file_info["functions"].append({
                "name": m.group(2),
                "args": args,
                "has_pointers": "*" in args,
                "line": content.count('\n', 0, m.start()) + 1
            })

        # 2. Speicher-Allokationen
        alloc_matches = re.finditer(r"(\w+)\s*=\s*(malloc|calloc|realloc|strdup)\((.*)\);", content)
        for m in alloc_matches:
            file_info["allocations"].append({
                "target_var": m.group(1),
                "method": m.group(2),
                "line": content.count('\n', 0, m.start()) + 1
            })

        # 3. Speicher-Freigaben
        free_matches = re.finditer(r"free\(([^)]+)\);", content)
        for m in free_matches:
            file_info["deallocations"].append({
                "variable": m.group(1).strip(),
                "line": content.count('\n', 0, m.start()) + 1
            })

        # 4. Statische Analyse mit cppcheck
        try:
            # --template=text nutzt ein einfaches Format für die JSON
            cpp_cmd = ["cppcheck", "--enable=warning", "--quiet", "--inline-suppr", filepath]
            result = subprocess.run(cpp_cmd, capture_output=True, text=True)
            for line in result.stderr.split('\n'):
                if any(k in line.lower() for k in ["leak", "address", "nullPointer", "uninitVar"]):
                    file_info["cppcheck_reports"].append(line.strip())
        except FileNotFoundError:
            pass

        data["files"][filename] = file_info

    # Dynamischer Dateiname
    output_filename = f"ownership.json"
    with open(output_filename, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=4)
    
    return output_filename

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("💡 Nutzung: python3 extract_ownership.py <projekt_verzeichnis>")
        sys.exit(1)

    target = sys.argv[1]
    result_file = analyze_ownership(target)
    
    if result_file:
        print(f"---")
        print(f"✅ Analyse für '{target}' abgeschlossen!")
        print(f"📂 JSON erstellt: {result_file}")
        print(f"---")