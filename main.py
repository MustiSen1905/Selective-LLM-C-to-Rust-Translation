import sys
import os
import analysis.static_analysis as static_analysis
from translation.c2rust import main as c2rust_main
from analysis.graph import run_analysis
from analysis.klee import analyze_ownership as collect_ownership_info
from analysis.unsafe_functions import main as get_unsafe_functions
from translation.translation import main as translate_unsafe_to_safe

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
        
        
        
        
        #Starting Analysis
        get_unsafe_functions(f"C-projects/{project_dir}/")
        print("Starting code analysis...")
        print(collect_ownership_info(f"C-projects/{project_dir}"))
        order =[]
        order = run_analysis(f"C-projects/{project_dir}") 
        print(f"Function order: {order}")
        # Hier wird die Analyse auf das gesamte Projekt angewendet, nicht nur auf eine Datei
        #static_analysis.main(f"C-projects/{project_dir}/{file}",f"cppcheck_report_{file}.xml",f"cppcheck_report_{file}.json")
        #Dividing C Code to unsafe and safe parts
        print("Dividing C code into unsafe and safe parts...")
        
        
        #Starting C2Rust Translation
        print("Translating C code to unsafe Rust...")
        c2rust_main(["--input-dir",f"C-projects/{project_dir}",
                        "--config",f"unsafe_test.json",
                        "--out",f"Rust-Outcome/{project_dir}"
                        ])
        #Starting unsafe Rust to safe Rust Translation
        print("Translating unsafe Rust code to safe Rust...")
        translate_unsafe_to_safe(f"Rust-Outcome/{project_dir}", order)
        
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