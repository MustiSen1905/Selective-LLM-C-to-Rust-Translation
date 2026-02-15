import sys
import json
import clang.cindex
from clang.cindex import CursorKind, TypeKind
import os

index = clang.cindex.Index.create()

functions = {}
call_graph = {}

def is_in_main_file(cursor, main_file):
    if cursor.location.file is None:
        return True
    return os.path.basename(cursor.location.file.name) == os.path.basename(main_file)

def flatten_rvalue(rvalue):
    """Hilfsfunktion: rvalue in String umwandeln, inkl. &x oder malloc(...)"""
    if rvalue.kind == CursorKind.UNARY_OPERATOR:
        tokens = list(rvalue.get_tokens())
        return ''.join(tok.spelling for tok in tokens)
    elif rvalue.kind == CursorKind.CALL_EXPR:
        return rvalue.spelling
    elif rvalue.kind == CursorKind.DECL_REF_EXPR:
        return rvalue.spelling
    else:
        return str(rvalue.kind)

def track_assignments(cursor, current_func):
    """Rekursiv Pointer-Zuweisungen tracken"""
    if cursor.kind == CursorKind.BINARY_OPERATOR:
        children = list(cursor.get_children())
        if len(children) == 2:
            lvalue, rvalue = children
            if lvalue.kind == CursorKind.DECL_REF_EXPR and lvalue.spelling in functions[current_func]["pointers"]:
                assigned_from = flatten_rvalue(rvalue)
                functions[current_func]["pointers"][lvalue.spelling]["assignments"].append({
                    "assigned_from": assigned_from,
                    "location": {
                        "line": cursor.location.line,
                        "file": cursor.location.file.name if cursor.location.file else None
                    }
                })
    for child in cursor.get_children():
        track_assignments(child, current_func)

def track_dereferences(cursor, current_func):
    """Rekursiv Dereferenzierungen tracken (*ptr)"""
    if cursor.kind == CursorKind.UNARY_OPERATOR:
        tokens = list(cursor.get_tokens())
        if any(tok.spelling == '*' for tok in tokens):
            child = next(cursor.get_children(), None)
            if child and child.kind == CursorKind.DECL_REF_EXPR:
                ptr_name = child.spelling
                if ptr_name in functions[current_func]["pointers"]:
                    functions[current_func]["pointers"][ptr_name]["dereferences"].append({
                        "location": {
                            "line": cursor.location.line,
                            "file": cursor.location.file.name if cursor.location.file else None
                        }
                    })
    for child in cursor.get_children():
        track_dereferences(child, current_func)

def track_struct_fields(cursor, current_func):
    """Rekursiv Struct-Felder erkennen (ptr->field)"""
    if cursor.kind == CursorKind.MEMBER_REF_EXPR:
        for child in cursor.get_children():
            if child.kind == CursorKind.DECL_REF_EXPR and child.spelling in functions[current_func]["pointers"]:
                functions[current_func]["pointers"][child.spelling]["struct_fields"].append(cursor.spelling)
    for child in cursor.get_children():
        track_struct_fields(child, current_func)

def visit(cursor, main_file, current_func=None):
    if not is_in_main_file(cursor, main_file):
        return

    # Funktionsdefinition
    if cursor.kind == CursorKind.FUNCTION_DECL and cursor.is_definition():
        current_func = cursor.get_usr()
        functions[current_func] = {
            "name": cursor.spelling,
            "return_type": cursor.result_type.spelling,
            "location": {
                "file": cursor.location.file.name if cursor.location.file else None,
                "start_line": cursor.extent.start.line,
                "end_line": cursor.extent.end.line
            },
            "parameters": [
                {
                    "name": arg.spelling,
                    "type": arg.type.spelling,
                    "is_pointer": arg.type.kind == TypeKind.POINTER
                } for arg in cursor.get_arguments()
            ],
            "pointers": {}
        }
        call_graph[current_func] = set()

    elif current_func:
        # Lokale Pointer erkennen
        if cursor.kind == CursorKind.VAR_DECL and cursor.type.kind == TypeKind.POINTER:
            initialized = len(list(cursor.get_children())) > 0
            functions[current_func]["pointers"][cursor.spelling] = {
                "type": cursor.type.spelling,
                "initialized": initialized,
                "assignments": [],
                "dereferences": [],
                "struct_fields": []
            }

        # Funktionsaufrufe tracken
        if cursor.kind == CursorKind.CALL_EXPR:
            referenced = cursor.referenced
            if referenced and referenced.spelling:
                call_graph[current_func].add(referenced.get_usr())

        # Rekursives Pointer-Tracking
        track_assignments(cursor, current_func)
        track_dereferences(cursor, current_func)
        track_struct_fields(cursor, current_func)

    for child in cursor.get_children():
        visit(child, main_file, current_func)

def export_json(filename="declarations.json"):
    output = {
        "functions": functions,
        "call_graph": {k: list(v) for k, v in call_graph.items()}
    }
    with open(filename, "w") as f:
        json.dump(output, f, indent=2)
    print(f"Exported analysis to {filename}")

def main(source_file):
    print("Starting static pointer analysis...")
    tu = index.parse(
        source_file,
        args=["-std=c11"],
        options=clang.cindex.TranslationUnit.PARSE_DETAILED_PROCESSING_RECORD
    )
    visit(tu.cursor, source_file)
    export_json()
    print("Done.")

if __name__ == "__main__":
    print("TEST: ", sys.argv[1])
    main_file = sys.argv[1]
    main(main_file)
