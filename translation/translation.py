import json
import os
import subprocess
import ollama
import csv
import time
import re
import sys
import entities.FunctionObject as FunctionObject

# Importiere die Funktion aus deiner create_prompt.py
try:
    from .create_prompt import build_final_prompt, build_struct_prompt
except ImportError:
    from create_prompt import build_final_prompt, build_struct_prompt

# Mechanische Signature-Preservation (Thesis-Empirie: LLM ignoriert Rule 20).
try:
    from .signature_guard import (
        extract_signature, signatures_equivalent, is_placeholder_body,
        dedup_function_definitions, build_param_alias_prelude,
        build_safe_bridge_prelude, build_safe_bridge_prelude_v2,
        rewrite_raw_ptr_accesses, GUARD_STATS,
    )
    from .ffi_export import auto_export_rust_fns_for_c_callers
except ImportError:
    from signature_guard import (
        extract_signature, signatures_equivalent, is_placeholder_body,
        dedup_function_definitions, build_param_alias_prelude,
        build_safe_bridge_prelude, build_safe_bridge_prelude_v2,
        rewrite_raw_ptr_accesses, GUARD_STATS,
    )
    from ffi_export import auto_export_rust_fns_for_c_callers

# --- KONFIGURATION ---
MODEL = "deepseek-coder:33b"
#"deepseek-coder:33b"
C_SOURCE_BASE = "C-projects"
MAX_RETRIES = 3
LOG_FILE = "bachelor_evaluation_results.csv"

client = ollama.Client(host='http://127.0.0.1:11434')

# ---------------------------------------------------------------------------
# AST-basierte Sanitizer (tree-sitter)
# ---------------------------------------------------------------------------
# Regex kann verschachtelte Braces { { } } nicht zuverlaessig parsen. Fuer
# struct_item, foreign_mod_item, function_item etc. nutzen wir daher den
# tree-sitter-Rust-Parser und editieren den Quelltext ueber Byte-Ranges. Faellt
# tree-sitter aus (Import-Fehler, Parser-Crash), greift der bisherige
# Regex-Pfad als Fallback.
# ---------------------------------------------------------------------------
_TS_PARSER = None
_TS_AVAILABLE = False
try:
    import tree_sitter_rust  # noqa: F401
    from tree_sitter import Language, Parser  # noqa: F401
    _TS_AVAILABLE = True
except Exception as _ts_err:  # pragma: no cover
    print(f"  [!] tree-sitter nicht verfuegbar ({_ts_err}); Fallback auf Regex.")


def _ts_parser():
    """Lazy-init des Rust-Parsers."""
    global _TS_PARSER
    if _TS_PARSER is None and _TS_AVAILABLE:
        _TS_PARSER = Parser(Language(tree_sitter_rust.language()))
    return _TS_PARSER


def _ts_walk(node):
    """Tiefensuche ueber alle AST-Knoten."""
    yield node
    for child in node.children:
        yield from _ts_walk(child)


def _ts_node_start_with_attrs(node):
    """Start-Byte eines Items inkl. vorgelagerter #[...]-Attribute."""
    start = node.start_byte
    prev = node.prev_sibling
    while prev is not None and prev.type == "attribute_item":
        start = prev.start_byte
        prev = prev.prev_sibling
    return start


def _ts_delete_byte_ranges(source_bytes: bytes, ranges):
    if not ranges:
        return source_bytes
    ranges = sorted(ranges, key=lambda r: -r[0])
    buf = bytearray(source_bytes)
    for s, e in ranges:
        del buf[s:e]
    return bytes(buf)


def _ts_byte_to_char(source_bytes: bytes, byte_idx: int) -> int:
    """Byte-Offset -> Python-char-Offset (UTF-8 -> str) fuer kompatible Slices."""
    return len(source_bytes[:byte_idx].decode("utf-8", errors="replace"))


def ast_remove_struct_and_type_defs(code: str) -> str:
    """Entfernt struct/enum/union/type-Definitionen (auch mit verschachtelten Braces)."""
    if not _TS_AVAILABLE or not code or not code.strip():
        return code
    try:
        source_bytes = code.encode("utf-8")
        tree = _ts_parser().parse(source_bytes)
        targets = {"struct_item", "enum_item", "union_item", "type_item"}
        ranges = [
            (_ts_node_start_with_attrs(n), n.end_byte)
            for n in _ts_walk(tree.root_node)
            if n.type in targets
        ]
        return _ts_delete_byte_ranges(source_bytes, ranges).decode("utf-8", errors="replace")
    except Exception as e:
        print(f"  [!] AST-Struct-Entferner-Fehler ({e}); behalte Original.")
        return code


def ast_remove_extern_c_blocks(code: str) -> str:
    """Entfernt `extern "C" { ... }`-Bloecke komplett (tree-sitter: foreign_mod_item)."""
    if not _TS_AVAILABLE or not code or not code.strip():
        return code
    try:
        source_bytes = code.encode("utf-8")
        tree = _ts_parser().parse(source_bytes)
        ranges = [
            (_ts_node_start_with_attrs(n), n.end_byte)
            for n in _ts_walk(tree.root_node)
            if n.type == "foreign_mod_item"
        ]
        return _ts_delete_byte_ranges(source_bytes, ranges).decode("utf-8", errors="replace")
    except Exception as e:
        print(f"  [!] AST-ExternC-Entferner-Fehler ({e}); behalte Original.")
        return code


def ast_remove_duplicate_structs(code: str) -> str:
    """Behaelt nur die erste Definition jedes benannten Structs (gegen E0119)."""
    if not _TS_AVAILABLE or not code:
        return code
    try:
        source_bytes = code.encode("utf-8")
        tree = _ts_parser().parse(source_bytes)
        seen = set()
        ranges = []
        for n in _ts_walk(tree.root_node):
            if n.type != "struct_item":
                continue
            name_node = n.child_by_field_name("name")
            if name_node is None:
                continue
            name = name_node.text.decode("utf-8")
            if name in seen:
                ranges.append((_ts_node_start_with_attrs(n), n.end_byte))
            else:
                seen.add(name)
        return _ts_delete_byte_ranges(source_bytes, ranges).decode("utf-8", errors="replace")
    except Exception as e:
        print(f"  [!] AST-Duplicate-Entferner-Fehler ({e}); behalte Original.")
        return code


def ast_get_function_range(code: str, func_name: str):
    """Liefert (start_char, end_char) der ersten function_item mit passendem Namen."""
    if not _TS_AVAILABLE or not code:
        return None, None
    try:
        source_bytes = code.encode("utf-8")
        tree = _ts_parser().parse(source_bytes)
        for n in _ts_walk(tree.root_node):
            if n.type != "function_item":
                continue
            name_node = n.child_by_field_name("name")
            if name_node is None:
                continue
            if name_node.text.decode("utf-8") == func_name:
                start_b = _ts_node_start_with_attrs(n)
                return (
                    _ts_byte_to_char(source_bytes, start_b),
                    _ts_byte_to_char(source_bytes, n.end_byte),
                )
        return None, None
    except Exception as e:
        print(f"  [!] AST-Funktionssuche-Fehler ({e}).")
        return None, None


def ast_extract_struct_defs(code: str):
    """Gibt [(name, volltext), ...] fuer jede struct_item-Definition zurueck."""
    if not _TS_AVAILABLE or not code or not code.strip():
        return []
    try:
        source_bytes = code.encode("utf-8")
        tree = _ts_parser().parse(source_bytes)
        out = []
        for n in _ts_walk(tree.root_node):
            if n.type != "struct_item":
                continue
            name_node = n.child_by_field_name("name")
            if name_node is None:
                continue
            start = _ts_node_start_with_attrs(n)
            text = source_bytes[start:n.end_byte].decode("utf-8", errors="replace")
            out.append((name_node.text.decode("utf-8"), text))
        return out
    except Exception as e:
        print(f"  [!] AST-Struct-Extract-Fehler ({e}).")
        return []


def ast_extract_safe_type_names(code: str):
    """Sammelt alle Namen von `struct_item`/`type_item`/`enum_item`/`union_item`,
    die mit `Safe` beginnen. Wird fuer Cross-Modul-Imports verwendet."""
    if not _TS_AVAILABLE or not code or not code.strip():
        # Regex-Fallback
        names = set()
        for m in re.finditer(r'\bpub\s+(?:struct|enum|union|type)\s+(Safe\w+)', code or ""):
            names.add(m.group(1))
        return sorted(names)
    try:
        source_bytes = code.encode("utf-8")
        tree = _ts_parser().parse(source_bytes)
        targets = {"struct_item", "enum_item", "union_item", "type_item"}
        names = set()
        for n in _ts_walk(tree.root_node):
            if n.type not in targets:
                continue
            name_node = n.child_by_field_name("name")
            if name_node is None:
                continue
            name = name_node.text.decode("utf-8")
            if name.startswith("Safe"):
                names.add(name)
        return sorted(names)
    except Exception as e:
        print(f"  [!] AST-Safe-Name-Extract-Fehler ({e}).")
        return []


def normalize_safe_double_underscore(rust_src_dir: str):
    """Pre-Pass vor `inject_safe_stub_types`: collapse `Safe__X` -> `Safe_X`
    references, wenn `Safe_X` als struct/enum/type im File definiert ist.
    Entfernt zusaetzlich den jetzt redundanten Stub `pub type Safe__X = ();`
    aus dem auto-generated stub block (sonst E0428 multiple definitions).

    Hintergrund: der LLM emittiert manchmal `Safe__IO_FILE` (doppeltes
    Underscore aus `Safe_` + `_IO_FILE` Zusammensetzung) als Field-Type, waehrend
    die echte struct als `Safe_IO_FILE` (single underscore, korrekt) definiert
    ist. Ohne diesen Pass injiziert `inject_safe_stub_types` einen
    `pub type Safe__IO_FILE = ();` stub, dessen `impl ... {{}}`-Block dann
    E0390 wirft (`cannot define inherent impl for primitive types`).
    """
    if not os.path.isdir(rust_src_dir):
        return
    def_re = re.compile(r'\bpub\s+(?:struct|enum|union|type)\s+(Safe\w+)')
    for fname in sorted(os.listdir(rust_src_dir)):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        with open(fpath, "r", encoding="utf-8") as f:
            code = f.read()
        defined = set(def_re.findall(code))
        renames = []
        for d in defined:
            if not d.startswith("Safe_") or d.startswith("Safe__"):
                continue
            wrong = "Safe__" + d[len("Safe_"):]
            if not re.search(r'\b' + re.escape(wrong) + r'\b', code):
                continue
            # 1. Drop redundant stub `pub type Safe__X = ();` (collapsed form
            #    would otherwise duplicate the actual `pub struct/type Safe_X`).
            stub_pat = re.compile(
                r'^pub\s+type\s+' + re.escape(wrong) + r'\s*=\s*\(\s*\)\s*;\s*\n',
                re.MULTILINE,
            )
            new_code = stub_pat.sub('', code)
            # 2. Replace remaining references to `Safe__X` with `Safe_X`.
            new_code = re.sub(r'\b' + re.escape(wrong) + r'\b', d, new_code)
            if new_code != code:
                renames.append((wrong, d))
                code = new_code
        if renames:
            with open(fpath, "w", encoding="utf-8") as f:
                f.write(code)
            for w, r in renames:
                print(f"  [+] Safe-Naming: collapsed {w} -> {r} in {fname}")


def inject_safe_stub_types(rust_src_dir: str):
    """Fuegt `pub type SafeX = ();` Stubs fuer referenzierte aber nicht definierte
    Safe*-Typen am Dateianfang ein. Fixt LLM-Halluzinationen wie
    `Box<Safe_IO_marker>`, wenn `Safe_IO_marker` nirgends definiert ist."""
    if not os.path.isdir(rust_src_dir):
        return

    # First normalize `Safe__X` references to `Safe_X` where the latter is
    # defined; otherwise the stub injector creates a useless unit-type stub.
    normalize_safe_double_underscore(rust_src_dir)

    def_re = re.compile(r'\bpub\s+(?:struct|enum|union|type)\s+(Safe\w+)')
    ref_re = re.compile(r'\b(Safe\w+)\b')
    marker = "// --- auto-generated Safe stub types ---"
    end_marker = "// --- end stubs ---"

    for fname in sorted(os.listdir(rust_src_dir)):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        with open(fpath, "r", encoding="utf-8") as f:
            code = f.read()

        defined = set(def_re.findall(code))
        refs = set(ref_re.findall(code))
        missing = sorted(refs - defined)

        if not missing:
            continue

        if marker in code:
            existing_block = code.split(marker, 1)[1].split(end_marker, 1)[0]
            existing = set(re.findall(r'\bpub\s+type\s+(Safe\w+)', existing_block))
            missing = [m for m in missing if m not in existing]
            if not missing:
                continue

        stub_lines = [f"pub type {n} = ();" for n in missing]
        header = marker + "\n" + "\n".join(stub_lines) + "\n" + end_marker + "\n\n"

        # Nach existierendem cross-module-import-Block einsortieren.
        if "// --- auto-generated cross-module Safe imports ---" in code:
            parts = code.split("\n\n", 1)
            code = parts[0] + "\n\n" + header + (parts[1] if len(parts) > 1 else "")
        else:
            code = header + code

        with open(fpath, "w", encoding="utf-8") as f:
            f.write(code)
        print(f"  [+] Stub-Types in {fname}: {', '.join(missing)}")


def _ast_struct_field_count(code: str, struct_name: str):
    """Zaehlt die Felder einer struct_item-Definition ueber tree-sitter."""
    if not _TS_AVAILABLE:
        return None
    try:
        source_bytes = code.encode("utf-8")
        tree = _ts_parser().parse(source_bytes)
        for n in _ts_walk(tree.root_node):
            if n.type != "struct_item":
                continue
            name_node = n.child_by_field_name("name")
            if name_node is None or name_node.text.decode("utf-8") != struct_name:
                continue
            body = n.child_by_field_name("body")
            if body is None:
                return 0
            return sum(1 for c in body.children if c.type == "field_declaration")
        return None
    except Exception:
        return None


def inject_safe_default_derive(rust_src_dir: str):
    """Faegt `#[derive(Default)]` an Safe-Structs und `..Default::default()` an
    truncated struct-Initializer an. Fixt E0063 bei LLM-Output-Truncation
    im `impl SafeX { pub fn from_ptr(...) -> Self { SafeX { f1: ..., } } }`.
    """
    if not os.path.isdir(rust_src_dir):
        return

    for fname in sorted(os.listdir(rust_src_dir)):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        with open(fpath, "r", encoding="utf-8") as f:
            code = f.read()
        original = code

        # 1. derive(Default) an alle `pub struct SafeX {`, AUSSER die Struct hat
        #    Raw-Pointer/Arrays als Felder (Default ist dafuer nicht implementiert).
        #    Fuer solche Structs generieren wir am Dateiende einen manuellen
        #    `impl Default` via `std::mem::zeroed()` (POD-sicher fuer c2rust-Structs).
        manual_default_structs = []

        def _add_derive(match):
            head = match.group(0)
            sname_m = re.search(r'struct\s+(Safe\w+)', head)
            sname = sname_m.group(1) if sname_m else None
            # Body der Struct holen, um Raw-Pointer-Felder zu pruefen.
            body_start = match.end()
            # Naive brace-matching (Safe-Structs haben keine verschachtelten Braces).
            depth = 1
            i = body_start
            while i < len(code) and depth > 0:
                if code[i] == '{':
                    depth += 1
                elif code[i] == '}':
                    depth -= 1
                i += 1
            body = code[body_start:i - 1]
            has_raw_ptr = bool(re.search(r':\s*\*(mut|const)\s', body))
            big_arr = re.search(r':\s*\[[^;\]]+;\s*(\d+)\s*\]', body)
            has_big_arr = bool(big_arr and int(big_arr.group(1)) > 32)

            prefix = code[: match.start()]
            tail_prefix = prefix.rstrip()
            recent = tail_prefix.splitlines()[-3:]
            already_has_derive = any("derive(Default" in ln for ln in recent)

            # Blocker-Fall: manueller impl Default statt derive.
            if has_raw_ptr or has_big_arr:
                if sname and sname not in manual_default_structs and not already_has_derive:
                    manual_default_structs.append(sname)
                return head

            if already_has_derive:
                return head
            return "#[derive(Default)]\n" + head

        code = re.sub(
            r'^pub\s+struct\s+Safe\w+\s*(?:<[^>]+>)?\s*\{',
            _add_derive,
            code,
            flags=re.MULTILINE,
        )

        # 1b. Manuellen `impl Default` anhaengen fuer Structs mit Raw-Pointer /
        #     grossen Arrays. `std::mem::zeroed()` ist fuer c2rust-POD-Structs
        #     semantisch korrekt (Pointer -> null, Primitives -> 0).
        if manual_default_structs:
            impl_lines = ["\n// --- auto-generated manual Default impls (raw-pointer structs) ---"]
            for sname in manual_default_structs:
                # Schon vorhanden? Dann ueberspringen.
                if re.search(r'impl\s+Default\s+for\s+' + re.escape(sname) + r'\b', code):
                    continue
                impl_lines.append(
                    f"impl Default for {sname} {{\n"
                    f"    fn default() -> Self {{\n"
                    f"        unsafe {{ std::mem::zeroed() }}\n"
                    f"    }}\n"
                    f"}}"
                )
            if len(impl_lines) > 1:
                code = code.rstrip() + "\n" + "\n".join(impl_lines) + "\n"

        # 2. `..Default::default()` in jedem Safe-Struct-Literal injizieren,
        #    wenn es weniger Felder hat als die Struct-Definition.
        #    WICHTIG: Struct-Definitionen (`pub struct SafeX { ... }`) aus-
        #    schliessen, sonst wird die Definition korrumpiert!
        current_code = code
        def _inject_rest(match):
            sname = match.group(1)
            body = match.group(2)
            # Ausschluss: Vorangehendes `struct`/`enum`/`union`/`impl` Keyword
            # -> ist Definition/Impl, kein Literal.
            pre = current_code[max(0, match.start() - 40):match.start()]
            if re.search(r'\b(struct|enum|union|impl|trait)\s+$', pre):
                return match.group(0)
            # Kommentare vor der ".."-Erkennung entfernen (Truncation-Platzhalter
            # wie `// ... repeat for all other fields` enthalten Punkte, die sonst
            # faelschlich als rest-syntax interpretiert werden).
            body_no_comments = re.sub(r'//[^\n]*', '', body)
            if ".." in body_no_comments:
                return match.group(0)
            filled = len(re.findall(r'^\s*\w+\s*:', body_no_comments, flags=re.MULTILINE))
            declared = _ast_struct_field_count(original, sname)
            if declared is None or filled >= declared:
                return match.group(0)
            # Truncated -> `..Default::default()` anhaengen.
            trimmed = body.rstrip().rstrip(",")
            new_body = trimmed + ",\n            ..Default::default()\n        "
            return f"{sname} {{{new_body}}}"

        code = re.sub(
            r'\b(Safe\w+)\s*\{([^{}]*)\}',
            _inject_rest,
            code,
        )

        if code != original:
            with open(fpath, "w", encoding="utf-8") as f:
                f.write(code)
            print(f"  [+] Default-Derive/rest-syntax in {fname}")


def inject_cross_module_safe_imports(rust_src_dir: str):
    """Injiziert `use crate::src::<other>::{Safe...};`-Statements fuer
    Cross-Modul-Referenzen auf Safe-Shadow-Typen.

    Strategie:
        1. Pro `.rs`-Datei alle `Safe*`-Typ-Namen per AST einsammeln.
        2. Fuer jede Datei ein `use`-Block mit den Safe-Namen der *anderen*
           Module vorne einfuegen (nur Safe-Typen -> keine Konflikte mit den
           c2rust-generierten Originalen, die pro Modul dupliziert sind).
    """
    if not os.path.isdir(rust_src_dir):
        return

    file_safe_map = {}
    for fname in os.listdir(rust_src_dir):
        if not fname.endswith(".rs"):
            continue
        mod = os.path.splitext(fname)[0]
        if mod in ("lib", "main"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        try:
            with open(fpath, "r", encoding="utf-8") as f:
                code = f.read()
        except OSError:
            continue
        file_safe_map[mod] = {
            "path": fpath,
            "code": code,
            "safe": set(ast_extract_safe_type_names(code)),
        }

    # Globaler Index: name -> Menge aller Module, die ihn definieren.
    # Canonical source = alphabetisch erstes Modul (deterministisch).
    name_to_mods = {}
    for mod, info in file_safe_map.items():
        for name in info["safe"]:
            name_to_mods.setdefault(name, set()).add(mod)
    canonical = {name: sorted(mods)[0] for name, mods in name_to_mods.items()}

    ident_re = re.compile(r'\b[A-Za-z_][A-Za-z0-9_]*\b')

    for mod, info in file_safe_map.items():
        if "auto-generated cross-module Safe imports" in info["code"]:
            continue  # bereits injiziert

        # Identifier im Code fuer referenced-Detection (vermeidet unused-Warnings).
        used_idents = set(ident_re.findall(info["code"]))

        # Gruppiere nach canonical-Modul.
        by_mod = {}
        for name, source in canonical.items():
            if source == mod:
                continue                    # selbst kanonisch -> Definition vorhanden
            if name in info["safe"]:
                continue                    # lokal auch definiert -> E0252 vermeiden
            if name not in used_idents:
                continue                    # nicht referenziert -> kein Import
            by_mod.setdefault(source, []).append(name)

        if not by_mod:
            continue

        import_lines = []
        total = 0
        for source in sorted(by_mod):
            names = sorted(set(by_mod[source]))
            total += len(names)
            import_lines.append(
                f"use crate::src::{source}::{{{', '.join(names)}}};"
            )

        header = (
            "// --- auto-generated cross-module Safe imports ---\n"
            + "\n".join(import_lines)
            + "\n\n"
        )
        new_code = header + info["code"]
        with open(info["path"], "w", encoding="utf-8") as f:
            f.write(new_code)
        print(f"  [+] Cross-Modul-Imports in {os.path.basename(info['path'])}: {total} Safe-Typen verlinkt.")


def _pascal_to_snake(name: str) -> str:
    """`MixColumns` -> `mix_columns`, `AESInit` -> `aes_init`, `XorWithIv` -> `xor_with_iv`."""
    s1 = re.sub(r'(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def fix_wrong_from_ptr_args(rust_src_dir: str):
    """Fixt LLM-Bug in Safe-Shadow Null-Branches: innerhalb
    `impl SafeA::from_ptr(outer: *const A)` wird `SafeB::from_ptr(outer)` aufgerufen,
    obwohl `outer` den falschen Pointer-Typ hat. Ersatz: `std::ptr::null()`.

    Pattern (typisch in Vec-Fallback-Branches):
        if orig.inner.is_null() {
            result.push(SafeInner::from_ptr(ptr));  // ptr ist *const Outer, BUG
        }

    Heuristik:
        * Finde `impl Safe<Outer> {` -> body via Brace-Matching.
        * Finde darin `fn from_ptr(<param>: *const ...)` -> merke `<param>` und `<Outer>`.
        * Ersetze innerhalb desselben impl-Bodies `Safe<Inner>::from_ptr(<param>)`
          mit `Safe<Inner>::from_ptr(std::ptr::null())`, wenn Inner != Outer.
    """
    if not os.path.isdir(rust_src_dir):
        return

    impl_head_re = re.compile(r'impl\s+(Safe\w+)\s*\{', re.MULTILINE)
    fn_sig_re = re.compile(
        r'\bfn\s+from_ptr\s*\(\s*([A-Za-z_]\w*)\s*:\s*\*(?:const|mut)\s+\w+',
    )

    total_files = 0
    total_fixes = 0

    for fname in sorted(os.listdir(rust_src_dir)):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        with open(fpath, "r", encoding="utf-8") as f:
            code = f.read()
        original = code

        # Alle impl-Bloecke finden (Brace-Matching ueber Byte-Index).
        edits = []  # (start, end, replacement)
        pos = 0
        while True:
            head = impl_head_re.search(code, pos)
            if not head:
                break
            outer_struct = head.group(1)
            body_start = head.end()
            # Brace-Matching.
            depth = 1
            i = body_start
            while i < len(code) and depth > 0:
                if code[i] == '{':
                    depth += 1
                elif code[i] == '}':
                    depth -= 1
                i += 1
            body_end = i - 1
            pos = i

            body = code[body_start:body_end]
            sig = fn_sig_re.search(body)
            if not sig:
                continue
            outer_param = sig.group(1)

            # Innerhalb body: Safe<Inner>::from_ptr(<outer_param>)
            call_re = re.compile(
                r'\b(Safe\w+)::from_ptr\s*\(\s*' + re.escape(outer_param) + r'\s*\)'
            )
            for m in call_re.finditer(body):
                inner_struct = m.group(1)
                if inner_struct == outer_struct:
                    continue  # Rekursion ist OK.
                abs_start = body_start + m.start()
                abs_end = body_start + m.end()
                repl = f"{inner_struct}::from_ptr(std::ptr::null())"
                edits.append((abs_start, abs_end, repl))

        if edits:
            # Von hinten nach vorn anwenden.
            for s, e, r in sorted(edits, key=lambda x: -x[0]):
                code = code[:s] + r + code[e:]
            with open(fpath, "w", encoding="utf-8") as f:
                f.write(code)
            total_files += 1
            total_fixes += len(edits)
            print(f"  [+] from_ptr-Arg-Fix in {fname}: {len(edits)} Null-Branches korrigiert.")

    if total_fixes == 0:
        print("  [i] from_ptr-Arg-Fix: keine Treffer.")


def fix_safe_char_fields(rust_src_dir: str):
    """Ersetzt `<field>: char` in Safe-Struct-Definitionen durch `i8`, weil C's
    `c_char` (i8) nicht mit Rust's 32-bit Unicode-`char` kompatibel ist.
    Fixt E0308 `expected char, found i8` bei `field: orig.field` Zuweisungen
    innerhalb der generierten `from_ptr`-Bruecken.

    Zusaetzlich werden `<field>: '\\0'`-Defaults in derselben Datei zu `<field>: 0`.
    """
    if not os.path.isdir(rust_src_dir):
        return

    char_field_re = re.compile(
        r'^(\s*pub\s+)([A-Za-z_][A-Za-z0-9_]*)(\s*:\s*)char(\s*,)',
        re.MULTILINE,
    )
    safe_struct_re = re.compile(r'pub\s+struct\s+Safe\w+\s*\{([^{}]*)\}', re.MULTILINE)

    patched_files = 0
    for fname in sorted(os.listdir(rust_src_dir)):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        with open(fpath, "r", encoding="utf-8") as f:
            code = f.read()
        original = code

        # Nur innerhalb von `pub struct Safe... { ... }` ersetzen.
        changed_fields = set()

        def _struct_sub(m):
            body = m.group(1)
            def _field_sub(fm):
                changed_fields.add(fm.group(2))
                return f"{fm.group(1)}{fm.group(2)}{fm.group(3)}i8{fm.group(4)}"
            new_body = char_field_re.sub(_field_sub, body)
            return m.group(0).replace(body, new_body)

        code = safe_struct_re.sub(_struct_sub, code)

        # Default-Initializer `field: '\0'` -> `field: 0` fuer betroffene Felder.
        for field in changed_fields:
            code = re.sub(
                r"(\b" + re.escape(field) + r"\s*:\s*)'\\0'(\s*[,}])",
                r"\g<1>0\g<2>",
                code,
            )

        if code != original:
            with open(fpath, "w", encoding="utf-8") as f:
                f.write(code)
            patched_files += 1
            print(f"  [+] Safe-char-Fix in {fname}: Felder {sorted(changed_fields)} -> i8")
    if patched_files == 0:
        print("  [i] Safe-char-Fix: keine Aenderungen noetig.")


def sanitize_legacy_files(rust_src_dir: str):
    """Wendet idempotente Sanitizer-Regeln projektweit auf ALLE `.rs`-Dateien an
    (auch c2rust-Legacy-Dateien, die nie durch die LLM-Pipeline liefen).

    Derzeit:
        * 7a: `::core::std::...` / `core::std::...`  ->  `std::...`
          Fixt E0433 in unberuehrtem c2rust-Output.
        * 7d: `.offset(i)` / `.add(i)` / `.sub(i)` mit nacktem Identifier ->
          `.offset(i as isize)` etc. Idempotent via Regex-Form.
    """
    if not os.path.isdir(rust_src_dir):
        return
    total = 0

    def _offset_sub(match):
        method = match.group(1)
        ident  = match.group(2)
        target = "usize" if method in ("add", "sub", "wrapping_add", "wrapping_sub") else "isize"
        return f".{method}({ident} as {target})"

    for fname in sorted(os.listdir(rust_src_dir)):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        try:
            with open(fpath, "r", encoding="utf-8") as f:
                code = f.read()
        except OSError:
            continue
        original = code

        # 7a: core::std::... -> std::...
        code = re.sub(r'(?:::)?core::std::', 'std::', code)

        # 7d: .offset/.add/.sub mit nackten Identifier -> Cast hinzufuegen.
        code = re.sub(
            r'\.(offset|add|sub|wrapping_offset|wrapping_add|wrapping_sub)\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)',
            _offset_sub,
            code,
        )

        if code != original:
            with open(fpath, "w", encoding="utf-8") as f:
                f.write(code)
            total += 1
    if total:
        print(f"  [+] Legacy-Sanitizer: {total} Dateien bereinigt (core::std, .offset-Cast).")


def fix_function_name_renames(rust_src_dir: str):
    """Repariert LLM-Rename-Inkonsistenz: LLM definiert z.B. `fn mix_columns(...)`,
    Caller in anderen (nicht-retranslated) Funktionen rufen aber weiter `MixColumns(...)`.

    Strategie:
        1. Sammle alle `fn <name>` Definitionen projektweit.
        2. Sammle alle Call-Sites `<Name>(` (PascalCase, mind. ein Großbuchstabe).
        3. Fuer jeden PascalCase-Aufruf, der nicht als Funktion existiert:
           - Pruefe, ob `pascal_to_snake(Name)` als Definition existiert.
           - Wenn ja: benenne Definition zurueck nach PascalCase und ersetze
             snake_case-Aufrufe (`\\bsnake\\(`) durch PascalCase. Fuegt
             `#[allow(non_snake_case)]` zur Definition hinzu, falls nicht vorhanden.

    Verhindert, dass Legacy-Caller mit E0425 "cannot find function" brechen,
    wenn nur einzelne Funktionen re-translated wurden.
    """
    if not os.path.isdir(rust_src_dir):
        return

    files = {}
    for fname in os.listdir(rust_src_dir):
        if not fname.endswith(".rs"):
            continue
        fpath = os.path.join(rust_src_dir, fname)
        try:
            with open(fpath, "r", encoding="utf-8") as f:
                files[fpath] = f.read()
        except OSError:
            continue

    fn_def_re = re.compile(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>]*>)?\s*\(')
    call_re   = re.compile(r'(?<![A-Za-z0-9_:.])([A-Za-z_][A-Za-z0-9_]*)\s*\(')

    defined_names = set()
    def_locations = {}  # name -> [(fpath, pos)]
    for fpath, code in files.items():
        for m in fn_def_re.finditer(code):
            n = m.group(1)
            defined_names.add(n)
            def_locations.setdefault(n, []).append((fpath, m.start(1), m.end(1)))

    called_pascal = set()
    for code in files.values():
        for m in call_re.finditer(code):
            n = m.group(1)
            if re.search(r'[A-Z]', n) and '_' not in n and n[0].isupper():
                called_pascal.add(n)

    rust_keywords = {"Self", "Some", "None", "Ok", "Err", "Box", "Vec", "String", "Option", "Result"}
    renames = {}  # snake -> Pascal
    for pascal in called_pascal:
        if pascal in defined_names or pascal in rust_keywords:
            continue
        snake = _pascal_to_snake(pascal)
        if snake == pascal:
            continue
        if snake in defined_names and snake not in renames:
            renames[snake] = pascal

    if not renames:
        return

    total_defs = 0
    total_calls = 0
    for fpath, code in files.items():
        new_code = code
        for snake, pascal in renames.items():
            # Definition: fn snake(  ->  fn Pascal(   (+ #[allow(non_snake_case)])
            def_pat = re.compile(r'(\bfn\s+)' + re.escape(snake) + r'(\s*(?:<[^>]*>)?\s*\()')
            def _def_sub(m):
                nonlocal total_defs
                total_defs += 1
                return m.group(1) + pascal + m.group(2)
            new_code2 = def_pat.sub(_def_sub, new_code)

            # #[allow(non_snake_case)] vor fn Pascal( einfuegen, falls nicht vorhanden
            allow_pat = re.compile(
                r'(^|\n)([ \t]*)(#\[allow\([^)]*\)\]\s*\n\2)*(fn\s+' + re.escape(pascal) + r'\s*(?:<[^>]*>)?\s*\()'
            )
            def _allow_sub(m):
                prefix, indent, existing_attrs, fn_part = m.group(1), m.group(2), m.group(3) or "", m.group(4)
                if "non_snake_case" in existing_attrs:
                    return m.group(0)
                return f"{prefix}{indent}#[allow(non_snake_case)]\n{indent}{existing_attrs}{fn_part}"
            new_code2 = allow_pat.sub(_allow_sub, new_code2)

            # Call-Sites: snake(  ->  Pascal(   (aber nicht als Feldzugriff .snake()
            call_pat = re.compile(r'(?<![A-Za-z0-9_:.])' + re.escape(snake) + r'(\s*\()')
            def _call_sub(m):
                nonlocal total_calls
                total_calls += 1
                return pascal + m.group(1)
            new_code2 = call_pat.sub(_call_sub, new_code2)

            new_code = new_code2

        if new_code != code:
            with open(fpath, "w", encoding="utf-8") as f:
                f.write(new_code)

    if renames:
        print(f"  [+] Function-Rename-Fixup: {len(renames)} Funktionen zurueckbenannt "
              f"({total_defs} Definitionen, {total_calls} Call-Sites).")
        for snake, pascal in sorted(renames.items()):
            print(f"      fn {snake}  ->  fn {pascal}")


def log_result(project, func_info, status, attempts, prompt, error=""):
    """Schreibt die Ergebnisse inklusive des verwendeten Prompts in eine CSV-Datei."""
    file_exists = os.path.isfile(LOG_FILE)
    with open(LOG_FILE, 'a', newline='', encoding='utf-8') as f:
        writer = csv.writer(f)
        if not file_exists:
            writer.writerow(["Timestamp", "Project", "Function_Info", "Status", "Attempts", "Prompt", "ErrorMessage"])
        
        clean_error = error.replace('\n', ' ').replace('\r', '')[:500]
        clean_prompt = prompt.replace('\r', '') 
        
        writer.writerow([
            time.strftime("%Y-%m-%d %H:%M:%S"), 
            project, 
            func_info, 
            status, 
            attempts, 
            clean_prompt,
            clean_error
        ])

def map_c_functions_to_rust_files(function_objects, rust_src_path):
    """
    Erstellt eine Map {funktion_name: rust_datei_pfad}.
    Nutzt die Liste der FunctionObjects, um gezielt nach den .rs-Pendanten zu suchen.
    """
    func_map = {}
    # 1. Wir erstellen zuerst einen Index aller vorhandenen Rust-Dateien im Projekt
    # Key: "main", Value: "/pfad/zu/main.rs"
    rust_file_index = {}
    for root, _, files in os.walk(rust_src_path):
        for file in files:
            if file.endswith(".rs"):
                base_name = os.path.splitext(file)[0]
                rust_file_index[base_name] = os.path.join(root, file)

    # 2. Wir gehen die Liste deiner FunctionObjects durch
    print("Test")
    
    for obj in function_objects:
        # Extrahiere Basisnamen der C-Datei (z.B. "logic" aus "logic.c")
        c_base_name = os.path.splitext(obj.file)[0]
        
        
        # Prüfen, ob wir eine passende Rust-Datei im Index haben
        if c_base_name in rust_file_index:
            rust_path = rust_file_index[c_base_name]
            # Mapping: Funktionsname -> Pfad zur gefundenen .rs Datei
            func_map[obj.name] = rust_path
        else:
            # Optional: Falls keine passende .rs Datei gefunden wurde
            print(f"Warnung: Keine Rust-Datei für {obj.file} (Funktion: {obj.name}) gefunden.")

    print(func_map)
    return func_map

def get_function_range(content, func_name):
    """Findet Start- und End-Index einer Funktion im Rust-Code.

    AST-first via tree-sitter (handhabt verschachtelte Braces, String-Literals
    mit `{`, Macro-Bodies korrekt). Faellt auf die bisherige Regex+Brace-Zaehlung
    zurueck, wenn tree-sitter nicht verfuegbar ist oder der Name im AST fehlt.
    """
    if _TS_AVAILABLE:
        s, e = ast_get_function_range(content, func_name)
        if s is not None:
            return s, e

    # --- Regex-Fallback (urspruengliche Logik) ---
    pattern = r'(?P<start>(?:#\[[^\]]+\]\s*)*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+' + re.escape(func_name) + r'\s*\()'
    match = re.search(pattern, content)
    if not match:
        print("True1")
        return None, None

    start_idx = match.start()
    
    # Suche das Ende der Signatur (entweder ein '{' oder ein ';')
    signature_end = start_idx
    while signature_end < len(content):
        if content[signature_end] == '{':
            break
        if content[signature_end] == ';':
            # Es ist nur eine Deklaration (extern fn foo();), überspringen!
            print("True2")
            return None, None
        signature_end += 1

    if signature_end >= len(content):
        print("True3")
        return None, None
    
    # Jetzt zählen wir die Klammern, um das Ende der Funktion zu finden
    brace_count = 1
    idx = signature_end + 1
    while brace_count > 0 and idx < len(content):
        if content[idx] == '{':
            brace_count += 1
        elif content[idx] == '}':
            brace_count -= 1
        idx += 1
    
    return start_idx, idx

def run_cargo_check_for_function(cargo_root, target_file, start_line, end_line):
    """
    Führt cargo check aus und filtert ALLE Fehler heraus, 
    die NICHT in der aktuell bearbeiteten Funktion (Zeile X bis Y) liegen.
    """
    result = subprocess.run(
        ["cargo", "check", "--message-format=json"],
        cwd=cargo_root,
        capture_output=True,
        text=True
    )

    relevant_errors = []
    
    for line in result.stdout.splitlines():
        if not line.startswith('{'):
            continue
            
        try:
            msg = json.loads(line)
            if msg.get("reason") == "compiler-message":
                error_data = msg["message"]
                
                if error_data.get("level") not in ["error", "fatal"]:
                    continue
                
                is_in_target_function = False
                for span in error_data.get("spans", []):
                    # 1. Check: Ist es die richtige Datei?
                    if span.get("is_primary") and target_file in span.get("file_name", ""):
                        error_line = span.get("line_start", 0)
                        
                        # 2. Check: Ist es die richtige Funktion? 
                        # Wir geben einen Puffer von +/- 2 Zeilen (z.B. für #[no_mangle] Attribute)
                        if (start_line - 2) <= error_line <= (end_line + 2):
                            is_in_target_function = True
                            break
                
                if is_in_target_function:
                    relevant_errors.append(error_data.get("rendered", ""))
                    
        except json.JSONDecodeError:
            continue

    if not relevant_errors:
        return True, ""
    else:
        # Gebe maximal 3 Fehler zurück, um die KI nicht mit Folgefehlern zu überlasten
        return False, "\n".join(relevant_errors[:3])

def ask_llm(prompt):
    try:
        response = client.generate(model=MODEL, prompt=prompt)
        content = response['response']
        if "```rust" in content:
            content = content.split("```rust")[1].split("```")[0].strip()
        elif "```" in content:
            content = content.split("```")[1].split("```")[0].strip()
        return content
    except Exception as e:
        print(f"  !! LLM Error: {e}")
        return None
    
def refactor_structs_in_file(file_path):
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # WICHTIGER SCHUTZ: Wenn die Safe-Structs schon in der Datei sind,
    # überspringen wir das Generieren, damit sie nicht doppelt vorkommen!
    if "pub struct Safe" in content:
        print(f"  [*] Überspringe Struct-Refactoring für {file_path} (Safe-Structs existieren bereits).")
        return content

    # Extrahiere alle Struct-Definitionen: AST-first, Regex-Fallback.
    # Regex scheitert an verschachtelten Braces in Feldtypen wie `Option<Bar<[u8; 4]>>`
    # oder `fn(i32) -> Result<(), Err>`; tree-sitter trifft die Grenzen praezise.
    struct_snippets = [text for _, text in ast_extract_struct_defs(content)]
    if not struct_snippets:
        struct_pattern = r'(#\[repr\(C\)\]\s+)?pub struct (\w+) \{[^}]+\}'
        struct_snippets = [m.group(0) for m in re.finditer(struct_pattern, content)]
    if not struct_snippets:
        return content

    unsafe_snippet = "\n\n".join(struct_snippets)
    
    # Generiere Schatten-Strukturen
    raw_safe_structs = ask_llm(build_struct_prompt("Context", unsafe_snippet))

    # Bereinige den generierten Code -- aber Structs/Impls behalten!
    safe_structs = sanitize_rust_code(raw_safe_structs, strip_items=False)

    # Füge die Safe-Structs OBEN ein
    new_content = safe_structs + "\n\n" + content
    return new_content

import re

def sanitize_rust_code(code, strip_items=True):
    """Bereinigt deterministisch Halluzinationen und Syntax-Fehler der KI.

    `strip_items`:
        True  (Default) -> fuer Funktions-Snippets: entfernt Struct/Enum/Union/
                           Type-Aliase und `extern "C" { ... }` (die KI soll nur
                           die Funktion zurueckgeben).
        False            -> fuer ganze Dateien oder den Output des Struct-
                           Refactor-Prompts (hier sollen die Safe-Structs und die
                           c2rust-Original-Structs erhalten bleiben).
    """
    if not code:
        return ""

    # 1. Entferne Markdown-Blöcke
    code = code.replace("```rust", "").replace("```", "").strip()

    # 2. Entferne Konversations-Geplapper am Anfang/Ende
    clean_lines = []
    for line in code.split('\n'):
        if re.match(r'^(The|Here|Note|This|I |You |It |If you|In order)', line):
            continue
        clean_lines.append(line)
    code = "\n".join(clean_lines)

    # 3. FIX E0774: Entferne derive Attribute über Typ-Aliasen
    code = re.sub(r'#\[derive\([^)]+\)\]\s+(?=pub type|type|pub const|const)', '', code)

    # 4. FIX Unicode: Ersetze '…' durch '...'
    code = code.replace("…", "...")

    # 5. FIX E0433 (CStr): Erzwinge absolute Pfade für CStr
    code = re.sub(r'(?<!std::ffi::)CStr::', 'std::ffi::CStr::', code)

    # 6. FIX (c_char/c_int): Erzwinge absolute Pfade
    code = re.sub(r'(?<!core::ffi::)(?<!std::os::raw::)\bc_char\b', 'core::ffi::c_char', code)
    code = re.sub(r'(?<!core::ffi::)(?<!std::os::raw::)\bc_int\b', 'core::ffi::c_int', code)

    # 7. FIX E0433 (ptr): Erzwinge absolute Pfade für ptr::null()
    #   a) Zuerst `core::std::` / `::core::std::` (Doppel-Prefix) bereinigen.
    code = re.sub(r'(?:::)?core::std::', 'std::', code)
    code = re.sub(r'(?<!std::)\bptr::null', 'std::ptr::null', code)

    # 7b. FIX: `public` ist kein Rust-Keyword -> zu `pub` korrigieren (nur Struct-Felder)
    code = re.sub(r'^\s*public\s+(?=\w+\s*:)', lambda m: m.group(0).replace('public', 'pub'), code, flags=re.MULTILINE)

    # 7c. FIX: Truncated LLM-Output (Platzhalter-Kommentare wie
    #   `// ... continue for all other fields`, `// ... repeat for ...`,
    #   `// Similar struct and impl blocks ... should follow`).
    #   Wenn solche Platzhalter auftauchen, ist das Snippet unvollstaendig;
    #   wir entfernen sie, damit sie keine Folgefehler produzieren.
    code = re.sub(
        r'^\s*//\s*\.\.\.\s*(continue|repeat|more|additional|rest|etc).*$',
        '',
        code,
        flags=re.MULTILINE | re.IGNORECASE,
    )
    code = re.sub(
        r'^\s*//\s*(repeat|continue)\s+for\s+all.*$',
        '',
        code,
        flags=re.MULTILINE | re.IGNORECASE,
    )
    code = re.sub(
        r'^\s*//\s*Similar\b.*(should|follow|be).*$',
        '',
        code,
        flags=re.MULTILINE | re.IGNORECASE,
    )

    # 7d. FIX E0308 (isize): `.offset(i)`, `.add(i)`, `.sub(i)`, `.wrapping_offset(i)`,
    #   `.wrapping_add(i)`, `.wrapping_sub(i)`, `.offset_from(...)` brauchen isize/usize.
    #   LLM nutzt oft eine i32-Loop-Variable `i` direkt -> E0308. Wir fuegen
    #   `as isize` bzw. `as usize` defensiv hinzu, falls der Parameter nur ein
    #   Identifier ist und noch keinen Cast enthaelt. Idempotent: doppelten
    #   Cast vermeiden wir via negative Lookahead auf `as `.
    def _add_offset_cast(match):
        method = match.group(1)
        ident  = match.group(2)
        # Schon gecastet? (sollte der Regex eh nicht treffen, Safety-Net)
        if " as " in ident:
            return match.group(0)
        target = "usize" if method in ("add", "sub", "wrapping_add", "wrapping_sub") else "isize"
        return f".{method}({ident} as {target})"

    code = re.sub(
        r'\.(offset|add|sub|wrapping_offset|wrapping_add|wrapping_sub)\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)',
        _add_offset_cast,
        code,
    )

    # 8. FIX: Aggressiver Use-Remover (löscht alles, was mit "use " beginnt)
    code_lines = []
    in_use_block = False
    for line in code.split('\n'):
        line_stripped = line.strip()
        if line_stripped.startswith("use ") and not line_stripped.endswith("!"):
            if not line_stripped.endswith(";"):
                in_use_block = True
            continue
        if in_use_block:
            if line_stripped.endswith(";"):
                in_use_block = False
            continue
        code_lines.append(line)
    code = "\n".join(code_lines)

    # 9. FIX: 'ustrchr' Halluzination zu 'strchr' korrigieren
    code = re.sub(r'\bustrchr\b', 'strchr', code)

    # 10./11. FIX (AST-first): extern "C"-Bloecke + Struct/Enum/Union/Type-Defs entfernen
    # (nur im Funktions-Snippet-Modus, sonst loeschen wir die c2rust-Originalstructs).
    if strip_items:
        if _TS_AVAILABLE:
            code = ast_remove_extern_c_blocks(code)
            code = ast_remove_struct_and_type_defs(code)
        else:
            code = re.sub(r'(#\[link\(name\s*=\s*"c"\)\]\s*)?extern\s+"C"\s*\{[^}]+\}', '', code)
            code = re.sub(r'(#\[repr\(C\)\]\s*)?(#\[derive\([^)]+\)\]\s*)?pub struct \w+\s*\{[^}]+\}', '', code)
            code = re.sub(r'pub type \w+\s*=[^;]+;', '', code)

    # NEUER SCHUTZ-MECHANISMUS: Hat die KI ueberhaupt eine Funktion geschrieben?
    # Nur im Snippet-Modus pruefen -- im Whole-File-Modus koennen Dateien auch
    # reine Struct-/Impl-Module sein.
    if strip_items and "fn " not in code:
        print("  [!] KI hat keine gültige Funktion generiert (möglicherweise ein Refusal). Verwerfe Output.")
        return ""
    
    last_brace = code.rfind('}')
    if last_brace != -1:
        suffix = code[last_brace + 1 :].strip()
        # Wenn nach der letzten Klammer nur noch Fragmente wie '1];' oder '...' stehen
        if suffix and not any(suffix in su for su in ["fn ", "pub ", "impl "]):
            code = code[: last_brace + 1]

    if code.startswith("I'm sorry") or code.startswith("Apologies"):
        print("  [!] KI hat sich entschuldigt. Verwerfe Output.")
        return ""

    return code.strip()

def remove_duplicate_structs(rust_code):
    """Verhindert E0119 durch doppelte Struct-Definitionen.

    AST-first: tree-sitter findet jede `struct_item` unabhaengig von Einrueckung
    oder verschachtelten Feldtypen und behaelt nur das erste Vorkommen je Name.
    Der alte line-basierte Fallback deckt nur `C2RustUnnamed*` / `__va_list_tag`
    ab und bleibt nur aktiv, wenn tree-sitter nicht verfuegbar ist.
    """
    if rust_code is None:
        return rust_code
    if _TS_AVAILABLE:
        return ast_remove_duplicate_structs(rust_code)

    # --- Fallback: urspruenglicher Zeilen-Parser ---
    seen_structs = set()
    clean_lines = []
    skip_mode = False
    for line in rust_code.split('\n'):
        match = re.search(r"pub struct (C2RustUnnamed\w*)", line)
        if match:
            name = match.group(1)
            if name in seen_structs:
                skip_mode = True
                continue
            seen_structs.add(name)
        if skip_mode and line.strip() == "}":
            skip_mode = False
            continue
        if not skip_mode:
            clean_lines.append(line)
    return "\n".join(clean_lines)

def process_single_function(func_name, file_path, project_name, cargo_root,safe_struct_context=""):
    """Extrahiert NUR die eine Funktion, lässt sie übersetzen und re-integriert sie."""
    print(f"[*] Analyse: {func_name} in {os.path.basename(file_path)}")
    
    with open(file_path, 'r', encoding='utf-8') as f:
        file_content = f.read()

    start_idx, end_idx = get_function_range(file_content, func_name)
    if start_idx is None:
        print(f"  [!] Überspringe: Funktion {func_name} im Rust-Code nicht gefunden.")
        return False

    unsafe_snippet = file_content[start_idx:end_idx]
    base_file_name = os.path.splitext(os.path.basename(file_path))[0]
    c_file = os.path.join(C_SOURCE_BASE, project_name, f"{base_file_name}.c")

    # === Signature-Guard: Snapshot der c2rust-Originalsignatur ===
    # Wird nach jedem LLM-Versuch verglichen; bei Drift -> Revert auf Baseline.
    _orig_extract = extract_signature(unsafe_snippet)
    original_signature = _orig_extract[1] if _orig_extract else None

    # Prompt erstellen (erhält nur das Snippet!)
    base_prompt = build_final_prompt(c_file, func_name, unsafe_snippet, safe_struct_context)

    
    
    last_err = ""
    last_sent_prompt = ""
    func_identifier = f"{os.path.basename(file_path)}:{func_name}"
    
    for attempt in range(1, MAX_RETRIES + 1):
        current_prompt = base_prompt + (f"\n\n### COMPILER ERROR:\n{last_err}" if last_err else "")
        last_sent_prompt = current_prompt
        
        raw_snippet = ask_llm(current_prompt)
        raw_snippet = remove_duplicate_structs(raw_snippet)
        safe_snippet = sanitize_rust_code(raw_snippet)
        if not safe_snippet: continue

        # === Signature-Guard ===
        # 1. Placeholder-Body? -> Verwerfen, naechster Versuch.
        # 2. Signatur-Drift?   -> Verwerfen, Original-Signatur erzwingen.
        if original_signature is not None:
            llm_extract = extract_signature(safe_snippet)
            if llm_extract is not None:
                _, llm_signature, llm_body = llm_extract
                if is_placeholder_body(llm_body):
                    print(f"  [-] Placeholder-Body von LLM (Versuch {attempt}). Verwerfe.")
                    GUARD_STATS.log_revert_placeholder(project_name, func_name)
                    last_err = ("Your previous output was a placeholder body "
                                "(`{ /* ... */ }` or `unimplemented!()`). "
                                "Emit a complete function body.")
                    continue
                if not signatures_equivalent(original_signature, llm_signature):
                    print(f"  [!] Signature-Drift: LLM hat Signatur geaendert.")
                    print(f"      Original : {' '.join(original_signature.split())}")
                    print(f"      LLM      : {' '.join(llm_signature.split())}")
                    GUARD_STATS.log_revert_signature(
                        project_name, func_name, original_signature, llm_signature)
                    # Mechanischer Splice: Original-Signatur + type-aware
                    # Bridge + Body-Rewriter + LLM-Body. Die Bridge konvertiert
                    # Raw-Pointer zu Safe-Shadow-Refs / Slice-Refs / Single-Refs,
                    # wo moeglich. Fuer Params, die Raw-Pointer bleiben (z.B.
                    # `*mut u8 -> &mut [u8]` ohne bekannte Laenge), wird der
                    # Body via `rewrite_raw_ptr_accesses` post-bearbeitet:
                    #   name[i]   ->  *name.offset((i) as isize)
                    #   name.f    ->  (*name).f
                    # Caller bleiben kompatibel; Body-Errors bleiben lokal.
                    bridge_prelude, raw_ptr_names = build_safe_bridge_prelude_v2(
                        original_signature, llm_signature, file_content)
                    body_inner = llm_body[1:-1] if llm_body.startswith("{") else llm_body
                    if raw_ptr_names:
                        body_inner = rewrite_raw_ptr_accesses(body_inner, raw_ptr_names)
                    if bridge_prelude or raw_ptr_names:
                        new_body = "{\n" + bridge_prelude + body_inner + "}"
                        tag = []
                        if bridge_prelude:
                            tag.append("Safe-Bridge")
                        if raw_ptr_names:
                            tag.append(f"RawPtr-Rewrite({','.join(sorted(raw_ptr_names))})")
                        print(f"      -> Splice: Original-Signatur + {'+'.join(tag)} + LLM-Body.")
                    else:
                        new_body = llm_body
                        print(f"      -> Splice: Original-Signatur + LLM-Body (keine Bridge noetig).")
                    safe_snippet = original_signature + " " + new_body

        # Nur den Funktions-Teil in der Datei ersetzen
        updated_content = file_content[:start_idx] + safe_snippet + file_content[end_idx:]
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(updated_content)

        start_line = updated_content[:start_idx].count('\n') + 1
        # Die neue Endzeile ist die Startzeile + die Anzahl der Zeilen im neuen Snippet
        end_line = start_line + safe_snippet.count('\n')

        target_filename = os.path.basename(file_path)
        
        # Übergib start_line und end_line an die Check-Funktion
        is_ok, err = run_cargo_check_for_function(cargo_root, target_filename, start_line, end_line)
        
        if is_ok:
            print(f"  [+] Erfolg: {func_name} (Versuch {attempt})")
            log_result(project_name, func_identifier, "SUCCESS", attempt, last_sent_prompt)
            GUARD_STATS.log_accept(project_name, func_name)
            return True
        else:
            last_err = err
            print(f"  [-] Fehler in Versuch {attempt}.")
    
    # Reset bei Fehlschlag
    log_result(project_name, func_identifier, "FAILED", MAX_RETRIES, last_sent_prompt, last_err)
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(file_content)
    return False

def main(project_dir, function_order=None):
    target_path = os.path.abspath(project_dir)
    project_name = os.path.basename(target_path.rstrip(os.sep))
    rust_src = os.path.join(target_path, "rust_out", "src")
    cargo_root = os.path.join(target_path, "rust_out")
    
    processed_files = set()
    file_struct_registry = {}

    if not os.path.exists(rust_src):
        print(f"Fehler: {rust_src} nicht gefunden.")
        return

    # HIER war der Fehler: Die Funktion muss definiert sein!
    func_to_file = map_c_functions_to_rust_files(function_order, rust_src)
    
    for func in function_order:
        file_path = func_to_file.get(func.name)
        if file_path and file_path not in processed_files:
            new_content = refactor_structs_in_file(file_path)
            new_content = remove_duplicate_structs(new_content) # Doppelte Structs entfernen, um E0119 zu verhindern
            # Whole-File-Sanitize: NUR Markdown/Geplapper raeumen, Structs/Types erhalten!
            new_content = sanitize_rust_code(new_content, strip_items=False)
            # Extrahiere die neuen Structs für den Prompt-Kontext
            # (In einer echten Pipeline könntest du sie hier separat speichern)
            struct_defs = new_content.split("\n\n")[0] 
            file_struct_registry[file_path] = struct_defs
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            processed_files.add(file_path)

    # Legacy-Dateien (nicht-retranslated c2rust-Output) idempotent bereinigen:
    # `core::std::` -> `std::`, `.offset(i)` -> `.offset(i as isize)`.
    print("--- Sanitize Legacy-Dateien ---")
    sanitize_legacy_files(rust_src)

    # Cross-Modul-Imports fuer Safe-Shadow-Typen injizieren, damit z.B.
    # `tex.rs` auf `SafeFrac` aus `abc.rs` zugreifen kann (fixt E0412/E0277
    # bei Cross-File-Referenzen auf Safe-Typen).
    print("--- Injiziere Cross-Modul-Safe-Imports ---")
    inject_cross_module_safe_imports(rust_src)

    # Stub-Types fuer LLM-Halluzinationen (referenzierte, aber nicht definierte
    # Safe*-Typen wie `Safe_IO_marker`, `SafeVoid`) -> `pub type Safe_X = ();`.
    print("--- Injiziere Safe-Stub-Types ---")
    inject_safe_stub_types(rust_src)

    # Safe-char-Felder zu i8 korrigieren (E0308 expected char, found i8).
    print("--- Korrigiere Safe-char-Felder zu i8 ---")
    fix_safe_char_fields(rust_src)

    # Falsche ptr-Args in Safe::from_ptr Null-Branches korrigieren (E0308).
    print("--- Korrigiere from_ptr Null-Branch Args ---")
    fix_wrong_from_ptr_args(rust_src)

    # #[derive(Default)] + ..Default::default() fuer truncated LLM-Output
    # in struct-Initializern (E0063 missing fields).
    print("--- Injiziere Default-Derive fuer Safe-Structs ---")
    inject_safe_default_derive(rust_src)

    # Reihenfolge festlegen
    order = function_order if function_order else list(func_to_file.keys())

    print(f"--- Starte Übersetzung von {len(order)} Funktionen ---")

    for func in order:
        if func.name in func_to_file:
            path = func_to_file[func.name]
            if func.name == "main":
                print("  [*] Spezieller Fall: Main ersetzen mit main_0, um die ursprüngliche main-Funktion zu erhalten.")
                process_single_function("main_0", func_to_file[func.name], project_name, cargo_root,safe_struct_context=file_struct_registry.get(path, ""))
            else:
                process_single_function(func.name, func_to_file[func.name], project_name, cargo_root,safe_struct_context=file_struct_registry.get(path, ""))
        else:
            print(f"  [!] Warnung: '{func.name}' nicht im Projekt gefunden.")

    # Post-Processing: repariert LLM-Rename-Inkonsistenz (z.B. LLM definiert
    # `fn mix_columns(...)`, Legacy-Caller rufen aber `MixColumns(...)`).
    # Benennt die Definition zurueck nach PascalCase + `#[allow(non_snake_case)]`.
    print("--- Repariere Funktionsnamen-Inkonsistenzen ---")
    fix_function_name_renames(rust_src)

    # Mechanischer Dedup: doppelte `fn NAME` (typ. LLM-Stub + echte Definition)
    # entfernen. Fixt E0428.
    print("--- Dedup doppelter Funktions-Definitionen ---")
    n_dedup = dedup_function_definitions(rust_src)
    if n_dedup == 0:
        print("  [i] Dedup: keine Duplikate gefunden.")

    # FFI-Auto-Export: Rust-Funktionen, die aus zurueck-gestrippten C-Helpern
    # (safe_*.c forward decls ohne Body) gerufen werden, brauchen
    # `#[no_mangle] pub extern "C"`, sonst E0xxx beim Hybrid-Linking.
    print("--- FFI Auto-Export fuer C-Caller ---")
    rust_out = os.path.dirname(rust_src.rstrip(os.sep))
    ffi_outcomes = auto_export_rust_fns_for_c_callers(rust_out)
    patched = [n for n, s in ffi_outcomes.items() if s.startswith(("PATCHED", "ALREADY"))]
    idiomatic = [n for n, s in ffi_outcomes.items() if "IDIOMATIC" in s]
    print(f"  [+] FFI: {len(patched)} Funktionen exportiert, "
          f"{len(idiomatic)} mit idiomatischer Signatur (manueller Wrapper noetig).")
    for n in idiomatic[:5]:
        print(f"      [warn] {n}: {ffi_outcomes[n]}")

    # Guard-Stats fuer Thesis-Reporting + stdout-Summary.
    stats_path = os.path.join(cargo_root, "guard_stats.csv")
    GUARD_STATS.write_csv(stats_path)
    print(f"--- {GUARD_STATS.summary()} (CSV: {stats_path}) ---")