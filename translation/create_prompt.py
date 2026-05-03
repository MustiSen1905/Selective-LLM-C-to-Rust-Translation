import json
from posixpath import basename
from posixpath import basename

import clang.cindex
import os
from clang.cindex import Config

# Pfad zur libclang (angepasst an dein System LLVM 18)
CLANG_LIB_PATH = '/usr/lib/llvm-18/lib'
CLANG_LIB_PATH_WIN = r'C:\Program Files\LLVM\bin'
if not Config.library_path:
    if os.path.exists(CLANG_LIB_PATH):
        Config.set_library_path(CLANG_LIB_PATH)
    elif os.path.exists(CLANG_LIB_PATH_WIN):
        Config.set_library_path(CLANG_LIB_PATH_WIN)

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
    
def get_global_symbols(c_file):
    """
    Extrahiert Namen globaler Variablen aus der C-Datei und filtert 
    Standard-Systemsymbole sowie Duplikate.
    """
    # Liste von Symbolen, die wir NICHT filtern wollen (Standard-C/POSIX)
    standard_c_symbols = {
        'stderr', 'stdout', 'stdin', 'errno', 'environ', 
        'optarg', 'optind', 'opterr', 'optopt'
    }

    index = clang.cindex.Index.create()
    # Wir parsen ohne komplexe Includes, um nur die lokalen Statics zu finden
    tu = index.parse(c_file, args=['-fsyntax-only'])
    global_vars = set()

    for node in tu.cursor.get_children():
        # Nur Variablen-Deklarationen auf Modulebene (Global/Static)
        if node.kind == clang.cindex.CursorKind.VAR_DECL:
            name = node.spelling
            # Filter: Nur hinzufügen, wenn nicht in Standard-Liste
            if name and name not in standard_c_symbols:
                global_vars.add(name)
            
    return sorted(list(global_vars))

def build_final_prompt(c_file, function_name, unsafe_rust_snippet, safe_struct_context=""):
    """Erstellt den Prompt basierend auf dem C-Kontext und dem Rust-Snippet."""
    extractor = CContextExtractor(c_file)
    ctx = extractor.get_full_context(function_name)
    globals_list = get_global_symbols(c_file)
    globals_formatted = ", ".join(globals_list) if globals_list else "None identified"
    
    basename = os.path.basename(c_file) # Ergibt "main.c"
    ownership_file = "ownership.json"
    file_info = None
    if os.path.exists(ownership_file):
        with open(ownership_file, 'r') as f:
            data = json.load(f)
            
        # Hier holst du dir die Infos für die spezifische Datei
        # Wir nutzen .get(), falls die Datei mal nicht im JSON existiert
        file_info = data.get("files", {}).get(basename)
        file_info = json.dumps(file_info, indent=2) if file_info else "No ownership info available for this file."
    else:
        file_info = "Ownership info file not found."
   
    
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

    #4. **IMPORTS**: Do NOT use `use` statements. Use absolute paths (e.g., `std::collections::HashMap`).
    # Der finale, hoch-präzise Prompt
    prompt = f"""### ROLE
You are an expert Rust Developer refactoring C2Rust-transpiled code into IDIOMATIC SAFE RUST while using SHADOW STRUCTS.

### TASK
Refactor the function `{function_name}`. 
- **SIGNATURE**: Change parameters to use Safe-types (e.g., `&SafeUser` instead of `*mut User`).
- **BRIDGE**: If the function receives a raw pointer from the old system, use the `SafeX::from_ptr(ptr)` bridge at the start.
- **LOGIC**: 100% Safe Rust. No `unsafe` keyword. Replace C-logic (strcpy, malloc) with idiomatic Rust.

### SAFE STRUCT DEFINITIONS (USE THESE)
{safe_struct_context}

### INPUT DATA
1. <c_context>: Original C code/types. Use ONLY for understanding logic.
2. <unsafe_rust_function>: The specific Rust function to refactor.
3. <existing_global_symbols>: {globals_formatted}

### STRICT GUIDING PRINCIPLES
1. **TOTAL SAFETY (CRITICAL)**: The output MUST NOT contain the `unsafe` keyword. Not in the function body, and NOT in the function header.
2. **SAFE SIGNATURES**: Change the function signature to use safe Rust types. 
   - Convert `*mut T` or `*const T` to `&mut T`, `&T`, or `Option<&T>`.
   - Convert C-style arrays/pointer pairs to Slices (`&[T]`) or `Vec<T>`.
   - Convert `*mut c_char` to `&str` or `String`.
3. **NO RE-DEFINITIONS**: DO NOT output `struct`, `type`, or `extern` blocks. Assume they exist or are handled by safe wrappers.
4. **IMPORTS**: Do NOT use `use` statements. Use absolute paths (e.g., `std::collections::HashMap`).
5. **CONST & STATICS**: If a global variable is used as a lookup table (like an S-Box), convert it to a `const` or an immutable `static`. DO NOT use `static mut`.
6. **ITERATOR PREFERENCE**: Instead of `for i in 0..len {{ slice[i] = ... }}`, prefer using `.iter_mut()`, `.chunks_exact()`, or `.zip()`. This is CRITICAL for performance as it eliminates bounds checks.
7. **FIXED SIZES**: If the C-code uses a fixed-size block (e.g., AES_BLOCKLEN), use fixed-size array references `&mut [u8; 16]` to assist the compiler with optimizations.
8. **NO SHADOWING**: Do NOT use names from <existing_global_symbols> for function parameters or local variables. If a name conflicts, append an underscore (e.g., `abc_file` -> `abc_file_`).
9. **PRINTF TO MACRO**: Replace `fprintf(stderr, ...)` or `printf` with `std::eprintln!` or `std::println!`. 
   - **CRITICAL**: Rust macros require a string literal. Do NOT pass a variable as the first argument. Use `println!("{{}}", var)` instead of `println!(var)`.
10. **ABSOLUTE PATHS FOR FFI**: You MUST use `std::ffi::CStr` and `std::ffi::c_char`. Never use just `CStr`.
11. **VARIADIC FUNCTIONS**: If the original function uses `...` (variadic arguments), do NOT use the unicode ellipsis character `…`. You must translate it to a valid Rust pattern (like a slice `&[&dyn std::any::Any]`, a macro, or keep the `...` if it's an extern "C" block, but ensure valid ASCII syntax).
12. **BRIDGE CASE SENSITIVITY**: Use the `SafeX::from_ptr(ptr)` bridge. Make sure to use the EXACT case-sensitive name of the Safe struct (e.g., if the original was `frac`, use `Safefrac`, not `SafeFrac`).
13. **NO ASSIGNMENTS IN CONDITIONS (CRITICAL)**: Rust forbids `if a = b {{ ... }}`. You MUST extract the assignment outside the condition: `a = b; if a != 0 {{ ... }}`.
14. **SYSTEM POINTERS EXCEPTION**: Do NOT use the `SafeX::from_ptr()` bridge for system types like `FILE`, `_IO_FILE`, `va_list`, or `__va_list_tag`. Leave these as raw pointers (`*mut T` or `*mut core::ffi::c_void`) or use them contextually. Do NOT invent a `SafeIOFILE`.
15. **C-STRING CASTING**: If you must pass a string literal to a legacy C function (like `strcpy`), always cast it correctly: `b"text\\0".as_ptr() as *const core::ffi::c_char`. Do not pass `u8` pointers where `i8` (`c_char`) is expected.
16. **C-STRING CASTING (CRITICAL)**: When converting byte string literals to raw pointers for C-functions or `CStr::from_ptr`, you MUST cast them to `c_char` like this: `b"text\\0".as_ptr() as *const core::ffi::c_char`. Do NOT just use `.as_bytes().as_ptr()`.
17. **STRING INDEXING**: In Rust, `String` cannot be indexed by `usize` (e.g., `s[i]`). If you need to access a character by index like in C, use `s.as_bytes()[i] as char` or `s.chars().nth(i).unwrap()`.
18. **C-INT CASTING FOR CHARACTERS (CRITICAL)**: Many legacy C functions (like `strchr`, `isalpha`, `toupper`) take characters as `int`. If you extract a `c_char` (which is `i8`) and pass it to such a function, you MUST cast it: `my_char as core::ffi::c_int`. Never pass an `i8` directly to a function expecting `i32` or `c_int`.
19. **PRESERVE FUNCTION NAME (CRITICAL)**: The Rust function name MUST be BYTE-IDENTICAL to `{function_name}`. Do NOT rename to snake_case (e.g., `MixColumns` stays `MixColumns`, NOT `mix_columns`; `XorWithIv` stays `XorWithIv`). Callers in other translation units still use the original name. Rust's `#[allow(non_snake_case)]` handles the convention warning.
20. **PRESERVE FUNCTION SIGNATURE (CRITICAL)**: The parameter types and return type MUST match the original c2rust-generated signature EXACTLY. Do NOT change `*mut u8` to `&mut [u8]`, `*const c_char` to `&str`, or `*mut T` to `Box<T>`. Legacy callers in other translation units still pass raw pointers — changing the signature breaks them with E0308. Keep `unsafe extern "C" fn` where present, keep raw-pointer parameters. Do all Safe-wrapping INTERNALLY by calling `SafeX::from_ptr(ptr)` in the FIRST LINE of the function body and operating on the Safe-Shadow from there. The signature is the public boundary; only the body changes.
21. **ONLY USE SAFE TYPES LISTED ABOVE (CRITICAL)**: You may ONLY use `SafeX::from_ptr()` for Safe types that appear in the `### SAFE STRUCT DEFINITIONS` section above. If a type like `pdf_t`, `xref_t`, or `kv_t` has NO listed Safe wrapper, do NOT invent `SafePdf`, `SafeXref`, `SafeKv` etc. Instead, access those types directly via raw pointer dereferencing: `(*ptr).field`. For C-style arrays (e.g., `xrefs: *mut xref_t`, count: `n_xrefs: c_int`), use index-based loops: `for i in 0..(*ptr).n_xrefs {{ let entry = &*(*ptr).xrefs.offset(i as isize); ... }}`.

### MEMORY & LOGIC RULES
- **Ownership**: Use `Box<T>` for heap allocation instead of raw pointers. Let Rust's RAII handle deallocation (no explicit `free`).
- **Error Handling**: Replace error codes (int) with `Result` or `Option` if appropriate for the logic, otherwise maintain the return type but ensure internal safety.
- **Strings**: Use `std::ffi::CStr` only if necessary to convert, but prefer passing `&str`. For printing, use `println!("{{}}", s)`.
- **Slices & Iterators**: Replace manual `while` loops and pointer increments with safe iterators (`.iter()`, `.chunks()`, etc.) or indexed access on slices.
- **Null-Safety**: Use `Option` to represent nullable pointers. Use `.as_ref()` or `.as_mut()` to safely access them.

### EXAMPLES OF THE BRIDGE PATTERN (CRITICAL)
Here is exactly how you must isolate unsafe boundaries using the Shadow Struct bridge function.

[BAD RUST - DO NOT DO THIS]
pub fn process_user(u: *mut User) {{
    unsafe {{
        if !u.is_null() {{
            println!("Admin: {{}}", (*u).isAdmin);
        }}
    }}
}}

[GOOD RUST - DO EXACTLY THIS]
pub fn process_user(u_ptr: *mut User) {{
    // 1. ISOLATE UNSAFE BOUNDARY: Convert immediately using the bridge
    let safe_u = unsafe {{ SafeUser::from_ptr(u_ptr) }};
    
    // 2. PURE SAFE LOGIC: No more raw pointers below this line!
    if safe_u.is_admin != 0 {{
        println!("Admin state active.");
    }}
}}

### EXAMPLE: RAW-POINTER STRUCT (NO Safe WRAPPER EXISTS)
When `pdf_t`, `xref_t` etc. have no Safe wrapper listed above, access their fields directly:

[BAD — SafePdf does not exist, will cause E0422]
pub unsafe extern "C" fn show_pdf(pdf: *const pdf_t) {{
    let safe = SafePdf::from_ptr(pdf);  // ERROR: SafePdf undefined
    for xref in safe.xrefs.iter() {{ ... }}  // ERROR: xrefs is *mut, not Vec
}}

[GOOD — use raw pointer dereferencing]
pub unsafe extern "C" fn show_pdf(pdf: *const pdf_t) {{
    if pdf.is_null() {{ return; }}
    let n = unsafe {{ (*pdf).n_xrefs }};
    for i in 0..n {{
        let xref = unsafe {{ &*(*pdf).xrefs.offset(i as isize) }};
        // use xref.start, xref.end, etc.
    }}
}}

### OUTPUT FORMAT
Return EXACTLY ONE Rust function. No markdown formatting (no ```rust), no explanations, no use statements, no structs. The function must be 100% safe Rust.

---
<c_context>
{c_info_block}
</c_context>

<unsafe_rust_function>
{unsafe_rust_snippet}
</unsafe_rust_function>"""

    return prompt

def build_struct_prompt(c_file, unsafe_structs_snippet):
    """Erstellt Safe-Shadow-Structs inklusive Brücken-Funktionen (from_unsafe)."""
    return f"""### ROLE
You are a Rust Refactoring Expert. Create SAFE SHADOW STRUCTS and BRIDGE FUNCTIONS.

### TASK
For each C-style struct, create a 100% safe Rust version with the prefix 'Safe'.
Additionally, implement a bridge function `from_unsafe` to convert the raw C-pointer version into the safe version.

### RULES
1. **NAMING**: `User` -> `SafeUser`.
1. **NAMING**: Prefix the EXACT original struct name with 'Safe'. Do NOT change the capitalization of the original name. (e.g., `frac` -> `Safefrac`, `User` -> `SafeUser`).
2. **FIELDS**: Replace `*mut c_char` with `String`, `*mut T` with `Box<SafeT>` or `Vec<SafeT>`.
3. **BRIDGE FUNCTION**: For each struct, implement:
   `impl SafeUser {{ pub unsafe fn from_ptr(ptr: *const User) -> Self {{ ... }} }}`
   Inside this function:
   - Handle null pointers (provide defaults or use Option).
   - Convert C-strings to Rust Strings using `CStr::from_ptr(...).to_string_lossy().into_owned()`.
   - Deep-copy any nested pointers into their 'Safe' versions.
4. **NO DERIVE ON ALIASES**: Do not put #[derive] on 'type' definitions.
5. **SMART CLONE**: Use `#[derive(Debug, Clone)]`. Do NOT use `Copy` for structs with heap data.
6. **SYSTEM & OPAQUE TYPES**: Do NOT create safe versions or bridge functions for system structs like `_IO_FILE`, `FILE`, `va_list`, or `__va_list_tag`. If a struct contains a pointer to a system type, leave it as a raw pointer `*mut T` or use an opaque wrapper. Deep-copying file handles or va_lists is invalid.
7. **ABSOLUTE PATHS ONLY**: When using `CStr`, `String`, or `Vec` inside the bridge function, use fully qualified absolute paths (e.g., `std::ffi::CStr`). No `use` statements are allowed.
8. **NEVER USE `char` FOR C BYTE FIELDS (CRITICAL)**: Rust's `char` is a 32-bit Unicode scalar and is NOT compatible with C's `c_char`/`i8`/`u8` byte fields. If a C struct field is `c_char` (byte tag, flag character, etc.), the Safe field MUST be `i8` (or `u8` for unsigned). Example: C `char f_or_n;` -> Safe `pub f_or_n: i8,` (NOT `char`). Reason: `orig.f_or_n` is `i8`; assigning it to a `char` field causes E0308 "expected `char`, found `i8`".
9. **NULL-BRANCH ISOLATION IN from_ptr (CRITICAL)**: Inside `impl SafeA {{ pub unsafe fn from_ptr(ptr: *const A) -> Self }}`, if you need a fallback `SafeB` value (e.g. for a nullable nested field), NEVER pass the outer `ptr` to `SafeB::from_ptr(ptr)` -- `ptr` has type `*const A`, not `*const B`, and this causes E0308. Always pass a NULL pointer of the correct type: `SafeB::from_ptr(std::ptr::null::<B>())` or equivalently `SafeB::from_ptr(core::ptr::null())`. The inner `from_ptr` handles null internally.

### INPUT
{unsafe_structs_snippet}

### OUTPUT FORMAT
Return ONLY the new Safe-Structs and their impl blocks.
"""