import os
import re
import json

# Diese Muster definieren, was wir als "speicherkritisch" betrachten
CRITICAL_PATTERNS = [
    # --- Die Klassiker ---
    r"\b(malloc|calloc|realloc|free)\b", 
    r"\b(strcpy|strcat|gets|sprintf|scanf)\b", # Unsichere Lib-Funktionen
    
    # --- Komplexität & Ownership ---
    r"->",                       # Pointer-Dereferenzierung (Ownership-Kette)
    r"\[.*\]",                   # Array-Zugriffe (potenzielle Out-of-bounds)
    r"\*",                       # Pointer-Deklarationen oder Dereferenzierung
    
    # --- Globaler Zustand ---
    r"\bextern\b|\bstatic\b",    # Zugriff auf geteilte Zustände
    
    # --- Input/Output (Parsing-Gefahr) ---
    r"\b(fopen|fread|recv|read)\b" 
]

def get_unsafe_functions(filepath):
    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()

    # Regex sucht nach Funktions-Signaturen: Rückgabetyp Name(Parameter) {
    # Erklärt: Findet Wörter am Zeilenanfang, gefolgt von Name und Klammern
    func_pattern = r"\n([\w\* ]+)\s+([\w\d_]+)\s*\([^)]*\)\s*\{"
    
    # Wir teilen die Datei grob in Funktionen auf, indem wir nach der öffnenden Klammer suchen
    # Da C-Parsing mit Regex komplex ist, nutzen wir eine Heuristik für den Funktions-Body
    findings = []
    
    # Suche alle Funktionsanfänge
    for match in re.finditer(func_pattern, content):
        func_name = match.group(2)
        start_pos = match.start()
        
        # Wir suchen das Ende der Funktion (vereinfacht: bis zur nächsten schließenden Klammer am Zeilenanfang)
        # Für eine Bachelorarbeit ist diese Heuristik meist präzise genug.
        end_match = re.search(r"\n\}", content[start_pos:])
        if end_match:
            end_pos = start_pos + end_match.end()
            func_body = content[start_pos:end_pos]
            
            # Prüfen, ob eines der kritischen Muster im Body vorkommt
            is_unsafe = False
            for pattern in CRITICAL_PATTERNS:
                if re.search(pattern, func_body):
                    is_unsafe = True
                    break
            
            if is_unsafe:
                findings.append(func_name)
                
    return findings

def main(project_path):
    output = {"unsafe": {}}

    for root, _, files in os.walk(project_path):
        for file in files:
            if file.endswith('.c'):
                full_path = os.path.join(root, file)
                # Wir speichern nur den Dateinamen (oder relativen Pfad)
                rel_path = os.path.relpath(full_path, project_path)
                
                unsafe_funcs = get_unsafe_functions(full_path)
                
                if unsafe_funcs:
                    output["unsafe"][rel_path] = unsafe_funcs

    with open('unsafe_functions.json', 'w', encoding='utf-8') as jf:
        json.dump(output, jf, indent=2)

    print(f"Erfolgreich erstellt: unsafe_functions.json mit {len(output['unsafe'])} Dateien.")

if __name__ == "__main__":
    # Pfad zum Ordner mit deinen .c Files ('.' für aktuelles Verzeichnis)
    main('.')