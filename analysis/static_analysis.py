import sys
import json
#import analysis.declarations as declarations
import analysis.cppcheck as cppcheck

def main(source_file,xml_file,json_file) -> None:
    """Main entry point for static analysis."""
    print("Starting static analysis...")
    print(f"Analyzing source file: {source_file}")
    #declarations.main(source_file)
    cppcheck.main(source_path=source_file, xml_file=xml_file, json_file=json_file)
    print("Static analysis completed.")
    


if __name__ == "__main__":
    main(sys.argv[1])