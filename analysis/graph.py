import subprocess
import sys
import os

def run_analysis(c_project_dir):
    project_path = os.path.abspath(c_project_dir)
    print(f"[*] Starte Rust-Analyzer für: {project_path}")
    
    try:
        # Wir leiten stderr DIREKT an sys.stderr weiter (Real-time Anzeige)
        # stdout wird abgefangen, um die STEP-Liste zu parsen
        process = subprocess.Popen(
            ["cargo", "run", "--manifest-path", "analysis/graph-builder/Cargo.toml", "--", project_path],
            stdout=subprocess.PIPE,
            stderr=sys.stderr, # <--- Schickt Rust-Diagnose direkt ins Terminal
            text=True
        )

        stdout, _ = process.communicate()

        if process.returncode != 0:
            print(f"\n[!] Cargo Prozess beendet mit Fehlercode {process.returncode}")
            return

        # 2. Extrahiere die Reihenfolge aus dem abgefangenen stdout
        if "ANALYSIS_START" in stdout:
            content = stdout.split("ANALYSIS_START")[1].split("ANALYSIS_END")[0]
            lines = content.strip().split("\n")
            
            print("\n" + "="*60)
            print("ÜBERSETZUNGSREIHENFOLGE (ERGEBNIS AUS RUST)")
            print("="*60)
            for line in lines:
                if line.strip():
                    print(line)
        else:
            print("[!] Keine Graphen-Daten im Output gefunden.")

    except Exception as e:
        print(f"[!] Fehler: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python main.py <c_project_dir>")
    else:
        run_analysis(sys.argv[1])