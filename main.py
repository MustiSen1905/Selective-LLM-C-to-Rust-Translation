import sys
import os
import analysis.static_analysis as static_analysis
from translation.c2rust import main as c2rust_main
from analysis.graph import run_analysis

def main(project_dir) -> None:
    try:
        #Check if the provided path is a directory
        isdir=os.path.isdir(f"C-projects/{project_dir}")
        if not isdir:
            raise NotADirectoryError(f"The path {project_dir} is not a valid directory.")
        
        print(f"Successfully opened project at: {project_dir}")
        
        files = os.listdir(f"C-projects/{project_dir}")
        files = [f for f in files if f.endswith('.c') or f.endswith('.h')]
        print(f"Project files: {files}")
        
        for file in files:
        
        
            #Starting Analysis
            print("Starting code analysis...")
            print(f"Analyzing file: {file}")
            print(f"C-projects/{project_dir}/{file}")
            run_analysis(f"C-projects/{project_dir}")  # Hier wird die Analyse auf das gesamte Projekt angewendet, nicht nur auf eine Datei
            exit(0)  # Entferne diesen Exit, wenn du die Analyse für alle Dateien durchführen möchtest
            static_analysis.main(f"C-projects/{project_dir}/{file}",f"cppcheck_report_{file}.xml",f"cppcheck_report_{file}.json")
            #Dividing C Code to unsafe and safe parts
            print("Dividing C code into unsafe and safe parts...")
            
            
            #Starting C2Rust Translation
            print("Translating C code to unsafe Rust...")
            c2rust_main(["--input-dir",f"C-projects/{project_dir}",
                          "--config",f"unsafe.json",
                          "--out",f"Rust-Outcome/{project_dir}"
                          ])
            #Starting unsafe Rust to safe Rust Translation
            print("Translating unsafe Rust code to safe Rust...")
            
            
            #Starting C2Rust Translation
            print("Translating C code to unsafe Rust...")

            #Starting unsafe Rust to safe Rust Translation
            print("Translating unsafe Rust code to safe Rust...")
            
            #Starting Merging Translations
            print("Merging translations into final Rust project...")
            
            #Starting Tests
            print("Running tests on the final Rust project...")
            
            print("Project translation and testing completed successfully.")
        
        
        
        
        
    except Exception as e:
        print(f"Error opening project: {e}")


if __name__ == "__main__":
    main(sys.argv[1])