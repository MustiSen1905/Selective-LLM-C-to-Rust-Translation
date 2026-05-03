import subprocess
import sys

def run_multiple_projects():
    # Liste deiner Projekte / Argumente
    projekte = ["abc2mtex","pdfresurrect-master"]
    log_datei = "projekte_ergebnisse.txt"
    
    # Datei öffnen ("w" zum Überschreiben am Anfang)
    with open(log_datei, "w", encoding="utf-8") as f:
        f.write("=== LOG-DATEI FÜR MEHRERE PROJEKTE ===\n\n")

    for projekt in projekte:
        print(f"Verarbeite: {projekt}...")
        
        # Befehl zusammenbauen
        command = [sys.executable, "main.py", projekt]
        
        try:
            # Ausführung
            result = subprocess.run(command, check=True, text=True, capture_output=True)
            output = result.stdout
            status = "ERFOLGREICH"
        except subprocess.CalledProcessError as e:
            output = e.stderr
            status = "FEHLER"
        except Exception as e:
            output = str(e)
            status = "SYSTEMFEHLER"

        # Ergebnisse an die Datei anhängen ("a" für append)
        with open(log_datei, "a", encoding="utf-8") as f:
            f.write(f"--- Projekt: {projekt} | Status: {status} ---\n")
            f.write(output)
            f.write("\n" + "="*40 + "\n\n")

    print(f"\nFertig! Alle Ergebnisse wurden in '{log_datei}' gespeichert.")

if __name__ == "__main__":
    run_multiple_projects()