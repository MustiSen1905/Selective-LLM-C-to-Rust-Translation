import sys
import json
import clang.cindex
from clang.cindex import CursorKind

index = clang.cindex.Index.create()

functions = {}
call_graph = {}

def is_in_main_file(cursor):
    return cursor.location.file and cursor.location.file.name == main_file

def visit(cursor, current_func=None):
    # Nur Code aus der Hauptdatei
    # if not is_in_main_file(cursor):
    #     print("Skipping code outside main file")
    #     return

    # Funktionsdefinition
    if cursor.kind == CursorKind.FUNCTION_DECL and cursor.is_definition():
        func_name = cursor.spelling
        current_func = cursor.get_usr()  # eindeutige ID!

        functions[current_func] = {
            "name": func_name,
            "return_type": cursor.result_type.spelling,
            "location": {
                "file": cursor.location.file.name,
                "start_line": cursor.extent.start.line,
                "end_line": cursor.extent.end.line,
            },
            "parameters": [
                {
                    "name": arg.spelling,
                    "type": arg.type.spelling,
                    "is_pointer": arg.type.kind.name.endswith("POINTER")
                }
                for arg in cursor.get_arguments()
            ]
        }

        call_graph[current_func] = set()

    # Funktionsaufrufe
    elif cursor.kind == CursorKind.CALL_EXPR and current_func:
        referenced = cursor.referenced
        if referenced and referenced.spelling:
            call_graph[current_func].add(referenced.get_usr())

    for child in cursor.get_children():
        visit(child, current_func)

def export_json(filename="analysis_output.json"):
    output = {
        "functions": functions,
        "call_graph": {k: list(v) for k, v in call_graph.items()}
    }
    with open(filename, "w") as f:
        json.dump(output, f, indent=2)
    print(f"Exported analysis to {filename}")

def main(source_file):
    print("Starting static analysis...")
    tu = index.parse(source_file, args=["-std=c11"])
    visit(tu.cursor)
    export_json()
    print("Done.")

if __name__ == "__main__":
    main_file = sys.argv[1]
    main(main_file)
