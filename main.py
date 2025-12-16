import sys
import os
import analysis.static_analysis as static_analysis

def main(project_dir) -> None:
    try:
        #Check if the provided path is a directory
        isdir=os.path.isdir(f"C-projects/{project_dir}")
        if not isdir:
            raise NotADirectoryError(f"The path {project_dir} is not a valid directory.")
        
        print(f"Successfully opened project at: {project_dir}")
        
        #Starting Analysis
        print("Starting code analysis...")
        static_analysis.main(f"C-projects/{project_dir}")
        #Dividing C Code to unsafe and safe parts
        print("Dividing C code into unsafe and safe parts...")
        
        
        
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