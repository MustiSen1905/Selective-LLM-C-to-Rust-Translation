import clang.cindex
import os
from clang.cindex import Config

# Pfad zur libclang (angepasst an dein System LLVM 18)
CLANG_LIB_PATH = '/usr/lib/llvm-18/lib'
if not Config.library_path and os.path.exists(CLANG_LIB_PATH):
    Config.set_library_path(CLANG_LIB_PATH)

class CContextExtractor:
    def __init__(self, file_path, include_paths=None):
        self.index = clang.cindex.Index.create()
        args = ['-fsyntax-only']
        if include_paths:
            for p in include_paths:
                args.append(f'-I{p}')
        
        # Parse die gesamte C-Datei, um alle Definitionen zu kennen
        self.tu = self.index.parse(file_path, args=args)
        self.file_path = os.path.abspath(file_path)
        self.seen_types = set()

    def _extract_source(self, node):
        if not node.location.file: return ""
        
        # Sicherstellen, dass wir nicht aus Standard-System-Headern lesen
        filename = node.location.file.name
        if "/usr/include" in filename or "/lib/gcc" in filename:
            return ""

        try:
            with open(filename, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            start, end = node.extent.start, node.extent.end
            if start.line == end.line:
                return lines[start.line-1][start.column-1:end.column-1]
            res = lines[start.line-1][start.column-1:]
            res += "".join(lines[start.line:end.line-1])
            res += lines[end.line-1][:end.column-1]
            return res
        except:
            return f"/* Source for {node.spelling} could not be read */"

    def resolve_type_dependencies(self, cursor, collected_structs):
        """Sucht rekursiv nach allen Typen, die die Funktion oder deren Typen nutzen."""
        for child in cursor.walk_preorder():
            if child.kind in [clang.cindex.CursorKind.TYPE_REF, clang.cindex.CursorKind.TYPEDEF_DECL]:
                ref = child.referenced
                if ref and ref.spelling and ref.spelling not in self.seen_types:
                    definition = ref.get_definition()
                    if definition:
                        self.seen_types.add(ref.spelling)
                        source = self._extract_source(definition)
                        if source:
                            collected_structs[ref.spelling] = source
                            # Rekursion für verschachtelte Structs
                            self.resolve_type_dependencies(definition, collected_structs)

    def get_full_context(self, func_name):
        func_node = None
        # Debug: Alle gefundenen Funktionen auflisten
        found_names = []
        if func_name == "main_0":
            func_name = "main"
        
        for node in self.tu.cursor.walk_preorder():
            if node.kind == clang.cindex.CursorKind.FUNCTION_DECL:
                found_names.append(node.spelling)
                # Prüfe auf Übereinstimmung UND ob es eine Definition ist (Body hat)
                if node.spelling == func_name:
                    # Wir nehmen die Definition, wenn verfügbar
                    if node.is_definition():
                        func_node = node
                        break
                    else:
                        func_node = node # Fallback auf Deklaration

        if not func_node:
            print(f"  [!] Clang-Fehler: Funktion '{func_name}' in C-Datei nicht gefunden.")
            # print(f"  [i] Vorhandene Funktionen: {', '.join(found_names[:10])}...")
            return None

        context = {
            "source": self._extract_source(func_node),
            "types": {},
            "calls": []
        }

        self.resolve_type_dependencies(func_node, context["types"])
        
        for child in func_node.walk_preorder():
            if child.kind == clang.cindex.CursorKind.CALL_EXPR and child.referenced:
                context["calls"].append(child.referenced.spelling)
        
        context["calls"] = list(set(context["calls"]))
        return context

def build_final_prompt(c_file, function_name, unsafe_rust_snippet):
    """Erstellt den Prompt basierend auf dem C-Kontext und dem Rust-Snippet."""
    extractor = CContextExtractor(c_file)
    ctx = extractor.get_full_context(function_name)
    
    # Falls die C-Funktion nicht gefunden wird, nutzen wir nur das Rust-Snippet
    c_info_block = "Original C context not found for this function."
    if ctx:
        type_defs_str = "\n\n".join([f"// Definition of {name}:\n{src}" for name, src in ctx["types"].items()])
        c_info_block = f"""ORIGINAL C FUNCTION:
{ctx['source']}

RELEVANT TYPE DEFINITIONS (RECURSIVE):
{type_defs_str}

CALLED EXTERNAL FUNCTIONS:
{', '.join(ctx['calls'])}"""

    # Der finale, hoch-präzise Prompt
    prompt = f"""### ROLE
You are an expert Rust Developer refactoring C2Rust-transpiled code.

### INPUT DATA
1. <c_context>: Original C code/types. Use ONLY for understanding logic.
2. <unsafe_rust_function>: The specific Rust function to refactor.

### STRICT GUIDING PRINCIPLES
1. **NO RE-DEFINITIONS (CRITICAL)**: DO NOT output `struct`, `type`, or `extern "C" { ... }` blocks. You are writing ONLY the function body. Assume everything else is globally available.
2. **MAINTAIN SIGNATURES**: DO NOT change the outer function signature.
3. **IMPORTS**: Do NOT use `use std::...` statements. Use absolute paths directly in the code.

### FFI & MEMORY RULES (CRITICAL)
- **Allocation**: To dynamically allocate a struct and return a raw pointer, DO NOT use `std::alloc`. Use `Box::into_raw(Box::new(StructName {{ ... }}))`.
- **Deallocation**: To free a raw pointer, call the external `free(ptr as *mut _)` function.
- **String Types & ASCII**: C `char` is often `i8`. Rust byte literals are `[u8]`. Cast them like this: `b"text\\0".as_ptr() as *const i8`. Byte literals (`b"..."`) MUST ONLY contain ASCII characters. For special characters like 'ö', use hex escapes (e.g., `\\xF6`).
- **Printing C-Strings**: Use `.to_string_lossy()`. Example: `println!("{{}}", CStr::from_ptr(ptr).to_string_lossy())`.
- **Pointer Field Access**: To access a field of a raw pointer, you MUST dereference it first: `(*ptr).field_name`, NEVER `ptr.field_name`.
- **C-Arrays to Pointers**: To pass a local array `let mut buf = [0; 32];` to a function expecting `*mut c_char`, use `buf.as_mut_ptr() as *mut i8`.
- **Struct Initialization**: Use `std::ptr::null_mut()` for null pointers. Cast integer literals strictly (e.g., `0 as i32` or `0 as c_int`).

### OUTPUT FORMAT
Return EXACTLY ONE Rust function (the implementation). No markdown formatting like ```rust, no explanations, no use statements, no structs.

---
<c_context>
{c_info_block}
</c_context>

<unsafe_rust_function>
{unsafe_rust_snippet}
</unsafe_rust_function>"""

    return prompt