import sys
import clang.cindex
from clang.cindex import CursorKind
import json


index = clang.cindex.Index.create()

functions = {}
call_graph = {}

def visit(cursor,parent=None):
    """Visit nodes in the AST and perform analysis."""
    if cursor.kind == CursorKind.FUNCTION_DECL and cursor.is_definition():
        parent=cursor.spelling
        functions[parent] = {
            "return_type": cursor.result_type.spelling,
            "start_line": cursor.extent.start.line,
            "end_line": cursor.extent.end.line,
            "parameters": [(arg.spelling, arg.type.spelling) for arg in cursor.get_arguments()]
        }
        call_graph[parent] = set()
        
    elif cursor.kind==CursorKind.CALL_EXPR and parent:
        called_function = cursor.spelling
        if called_function:
            call_graph[parent].add(called_function)
                
    for child in cursor.get_children():
        visit(child, parent)
        

        

def export_json(filename="analysis_output.json"):
    """Export analysis results to a JSON file."""
    output = {
        "functions": functions,
        "call_graph": {k: list(v) for k, v in call_graph.items()}
    }
    with open(filename, 'w') as f:
        json.dump(output, f, indent=4)
    print(f"Analysis results exported to {filename}")

    

def main(translation_unit):
    """Main entry point for static analysis."""
    print("Starting static analysis...")
    
    visit(translation_unit.cursor)
    
    export_json()
    
    print("Static analysis completed.")

if __name__ == "__main__":
    translation_unit=index.parse(f'{sys.argv[1]}',args=["-std=c11"])  # Replace with actual file
    main(translation_unit)