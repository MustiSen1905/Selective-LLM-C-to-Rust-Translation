"""FFI-Auto-Export fuer Rust-Funktionen, die aus C-Code (safe_*.c) zurueck-
gerufen werden.

Hintergrund (Thesis-Empirie): wenn die selektive Rustification ein
zirkulaeres Call-Pattern erzeugt — Rust ruft C ruft Rust — muss die
Rust-Funktion mit C-Linkage exportiert sein. Der LLM emittiert aber
oft idiomatische Rust-Signaturen ohne `#[no_mangle]` oder `extern "C"`,
was zu `undefined reference` Linker-Fehlern fuehrt.

Diese Pipeline-Erweiterung:
  1. Parsed jede `safe_*.c` und sammelt Funktionen, die nur als Forward-
     Declaration vorkommen (Body wurde nach Rust verlagert).
  2. Findet die korrespondierende Rust-Funktion in `src/*.rs`.
  3. Prueft, ob `#[no_mangle]` UND eine C-kompatible Signatur vorliegen.
     - Wenn beides ja: `pub extern "C"` ggf. ergaenzen.
     - Wenn `#[no_mangle]` fehlt: einfuegen.
     - Wenn Signatur Rust-idiomatisch ist (`&str`, `&[u8]`, `Vec`, etc.):
       loggen + warnen — manueller Wrapper noetig.
"""
from __future__ import annotations
import os
import re
from typing import Optional


# Forward-Decl-Pattern: `<sig>;` ohne `{`-Body danach.
# Wir suchen Funktionen-mit-Argumenten-Liste, die mit `;` enden anstatt `{`.
_C_FORWARD_DECL_RE = re.compile(
    r'^\s*'
    r'(?:[A-Za-z_][A-Za-z_0-9]*[\s*]+)+'   # return type tokens (e.g. "void", "char *", "const char *")
    r'(?P<name>[A-Za-z_][A-Za-z_0-9]*)\s*'  # function name
    r'\([^)]*\)'                            # parameter list (one-line; multi-line decls handled below)
    r'\s*;',
    re.MULTILINE,
)

# Multi-line variant: `<head>(\n  args\n)\n  ;`
_C_FORWARD_DECL_MULTI_RE = re.compile(
    r'^[ \t]*'
    r'(?:[A-Za-z_][A-Za-z_0-9]*[\s*]+)+'
    r'(?P<name>[A-Za-z_][A-Za-z_0-9]*)\s*'
    r'\([^)]*\)'
    r'\s*\n\s*;',
    re.MULTILINE,
)

# Excluded names that match the regex but aren't really functions.
_C_KEYWORDS = {"if", "for", "while", "switch", "do", "return", "sizeof", "typedef"}

# `main` ist Rust-bin-Entry-Point; Rust's eigener Runtime uebernimmt das
# Symbol. Adding `#[no_mangle] pub extern "C"` zu `fn main` kollidiert mit
# der Rust-bin-Konvention. Daher hardgekodet ausschliessen.
_FFI_EXPORT_BLACKLIST = {"main"}


def collect_c_forward_decls(c_file_path: str) -> set[str]:
    """Liefert Namen aller Funktionen, die in der Datei nur als Forward-
    Declaration (ohne Body) vorkommen. Diese Funktionen MUESSEN von Rust
    bereitgestellt werden, sonst Linker-Fehler.
    """
    if not os.path.isfile(c_file_path):
        return set()
    with open(c_file_path, "r", encoding="utf-8", errors="ignore") as f:
        src = f.read()
    decls: set[str] = set()
    for m in _C_FORWARD_DECL_RE.finditer(src):
        name = m.group("name")
        if name not in _C_KEYWORDS:
            decls.add(name)
    for m in _C_FORWARD_DECL_MULTI_RE.finditer(src):
        name = m.group("name")
        if name not in _C_KEYWORDS:
            decls.add(name)
    # Filter: wenn der Name ANDERSWO mit `{`-Body in derselben Datei vorkommt,
    # ist es eine echte Definition, kein Forward-Decl.
    has_body = re.compile(
        r'\b(' + "|".join(re.escape(n) for n in decls) + r')\s*\([^)]*\)\s*\{',
    ) if decls else None
    if has_body:
        for m in has_body.finditer(src):
            decls.discard(m.group(1))
    return decls


# Rust-Funktions-Definition Pattern.
_RUST_FN_DEF_RE = re.compile(
    r'(?P<attrs>(?:#\[[^\]]+\]\s*)*)'
    r'(?P<vis>pub(?:\s*\([^)]*\))?\s+)?'
    r'(?P<unsafe>unsafe\s+)?'
    r'(?P<extern>extern\s+"[^"]*"\s+)?'
    r'fn\s+(?P<name>[A-Za-z_][A-Za-z_0-9]*)\s*'
    r'(?:<[^>]*>)?'
    r'\s*\(',
    re.MULTILINE,
)


def _signature_is_c_compatible(sig_text: str) -> bool:
    """Heuristik: enthaelt die Param-Liste oder Returntyp idiomatische Rust-
    Konstrukte (`&str`, `&[T]`, `Vec`, `String`, `Box`, `Option`, `Result`)?
    Wenn ja, ist die Signatur NICHT direkt C-kompatibel und ein Wrapper
    waere noetig.
    """
    forbidden = (
        "&str", "&[", "Vec<", "String", "Box<", "Option<", "Result<",
        "Rc<", "Arc<", "RefCell<", "Cell<",
    )
    return not any(tok in sig_text for tok in forbidden)


def _is_inside_extern_block(rust_src: str, pos: int) -> bool:
    """True wenn `pos` innerhalb eines `extern "C" { ... }` Blocks liegt.
    Solche Bloecke deklarieren nur Imports von C, NICHT Rust-Definitionen —
    der Patcher darf sie nicht anfassen.
    """
    # Naive: zaehle `{` und `}` rueckwaerts; suche das letzte unmatched `{`.
    depth = 0
    i = pos
    while i > 0:
        c = rust_src[i]
        if c == "}":
            depth += 1
        elif c == "{":
            if depth == 0:
                # Found unmatched `{`; check if preceded by `extern "..."`.
                prefix = rust_src[max(0, i - 40): i]
                if re.search(r'extern\s+"[^"]*"\s*$', prefix):
                    return True
                return False
            depth -= 1
        i -= 1
    return False


def _find_fn_def(rust_src: str, fn_name: str) -> Optional[tuple[int, int, dict]]:
    """Findet eine Rust-Funktionsdefinition (NICHT eine extern-Import-Decl).
    Liefert (start, end_of_attrs+head, {attrs, vis, unsafe, extern}) oder None.
    """
    for m in _RUST_FN_DEF_RE.finditer(rust_src):
        if m.group("name") != fn_name:
            continue
        # Skip wenn innerhalb eines `extern "C" { ... }` Blocks.
        if _is_inside_extern_block(rust_src, m.start()):
            continue
        # Pruefe ob nach der Param-Liste ein Body `{...}` oder nur `;` folgt.
        # Bei `;` ist es eine Declaration, kein Body -> auch ueberspringen
        # (kann passieren z.B. bei stripped traits / abstract decls).
        paren = m.end() - 1
        depth, i = 1, paren + 1
        while i < len(rust_src) and depth > 0:
            if rust_src[i] == "(": depth += 1
            elif rust_src[i] == ")": depth -= 1
            i += 1
        # Nach `)`: skip whitespace + return-type bis zu `{` oder `;`.
        j = i
        while j < len(rust_src) and rust_src[j] != "{" and rust_src[j] != ";":
            j += 1
        if j >= len(rust_src) or rust_src[j] == ";":
            continue  # Declaration, no body
        return m.start(), m.end(), {
            "attrs": m.group("attrs") or "",
            "vis": m.group("vis") or "",
            "unsafe": m.group("unsafe") or "",
            "extern": m.group("extern") or "",
        }
    return None


def add_no_mangle_to_fn(rust_src: str, fn_name: str) -> tuple[str, str]:
    """Patcht eine Rust-Funktionsdefinition so, dass sie von C aufrufbar ist.

    Aenderungen (idempotent):
      - `#[no_mangle]` als Attribute ergaenzen, falls fehlt.
      - `pub` ergaenzen, falls fehlt.
      - `extern "C"` ergaenzen, falls fehlt.

    Wenn die Signatur Rust-idiomatische Typen enthaelt, wird die Aenderung
    NICHT angewandt (Compilation wuerde sonst fail mit
    "improper_ctypes_definitions"); stattdessen wird ein Hinweis in der
    Status-Meldung zurueckgegeben.

    Liefert (modified_src, status_message).
    """
    found = _find_fn_def(rust_src, fn_name)
    if found is None:
        return rust_src, f"NOT_FOUND"
    start, end, parts = found
    # Param-Liste + Returntyp extrahieren.
    paren = end - 1  # `(`
    depth, i = 0, paren
    while i < len(rust_src):
        c = rust_src[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                i += 1
                break
        i += 1
    after_params = i
    # Returntyp + where-clause bis zur `{`.
    j = after_params
    while j < len(rust_src) and rust_src[j] != "{":
        j += 1
    sig_text = rust_src[paren:j]
    if not _signature_is_c_compatible(sig_text):
        return rust_src, f"IDIOMATIC_SIG (manual wrapper needed)"

    attrs = parts["attrs"]
    needs_no_mangle = "#[no_mangle]" not in attrs
    needs_pub = not parts["vis"].strip().startswith("pub")
    needs_extern = "extern" not in parts["extern"]
    if not (needs_no_mangle or needs_pub or needs_extern):
        return rust_src, "ALREADY_EXPORTED"

    # Build new prefix: keep existing attrs + add #[no_mangle], then "pub
    # extern \"C\" " + existing unsafe + "fn ..."
    new_attrs = attrs
    if needs_no_mangle:
        # Append #[no_mangle] to attrs (preserve existing attrs/whitespace).
        if new_attrs.strip():
            new_attrs = new_attrs.rstrip() + "\n#[no_mangle]\n"
        else:
            new_attrs = "#[no_mangle]\n"
    new_vis = parts["vis"] if parts["vis"] else "pub "
    new_unsafe = parts["unsafe"]
    new_extern = parts["extern"] if parts["extern"] else 'extern "C" '

    # Find the position of `fn` in the original match — it's right after vis+unsafe+extern.
    head_start = start + len(attrs)
    # Recompute fn-keyword position by re-matching head fragment.
    fn_kw_pos = rust_src.find("fn ", head_start, end)
    if fn_kw_pos < 0:
        return rust_src, "FN_KW_NOT_FOUND"

    new_head = (new_attrs + new_vis + new_unsafe + new_extern + "fn " +
                rust_src[fn_kw_pos + 3:end])
    modified = rust_src[:start] + new_head + rust_src[end:]
    parts_changed = []
    if needs_no_mangle: parts_changed.append("no_mangle")
    if needs_pub: parts_changed.append("pub")
    if needs_extern: parts_changed.append('extern "C"')
    return modified, "PATCHED: " + ", ".join(parts_changed)


def auto_export_rust_fns_for_c_callers(rust_out_dir: str) -> dict[str, str]:
    """Top-level Pipeline-Pass.

    Scannt alle `safe_*.c` Dateien im Projekt-rust_out, sammelt die Forward-
    Declarations, und patcht die korrespondierenden Rust-Funktionen in
    `src/*.rs` so, dass sie C-aufrufbar sind.

    Liefert {fn_name: outcome} fuer Reporting.
    """
    rust_src_dir = os.path.join(rust_out_dir, "src")
    if not os.path.isdir(rust_out_dir) or not os.path.isdir(rust_src_dir):
        return {}

    # 1. C-side forward decls sammeln. Walk subdirectories so multi-level
    # source trees (libtiff/tools/port/contrib/...) work end-to-end.
    needed_exports: set[str] = set()
    for root, _, files in os.walk(rust_out_dir):
        # Skip the auto-generated rust_out/src/ tree; safe_*.c only live
        # alongside the C subdir layout in rust_out's top-level mirror.
        if os.path.commonpath([root, rust_src_dir]) == rust_src_dir:
            continue
        for fname in files:
            if not (fname.startswith("safe_") and fname.endswith(".c")):
                continue
            decls = collect_c_forward_decls(os.path.join(root, fname))
            needed_exports.update(decls)

    # Blacklist: spezielle Symbole wie `main` nie patchen.
    needed_exports -= _FFI_EXPORT_BLACKLIST

    if not needed_exports:
        return {}

    # 2. Pro Rust-Datei patchen — recurse so nested src/<dir>/<file>.rs is
    # also covered.
    outcomes: dict[str, str] = {}
    rs_paths = []
    for root, _, files in os.walk(rust_src_dir):
        for f in sorted(files):
            if f.endswith(".rs"):
                rs_paths.append((f, os.path.join(root, f)))
    for fname, fpath in rs_paths:
        with open(fpath, "r", encoding="utf-8") as f:
            src = f.read()
        modified = src
        for fn_name in sorted(needed_exports):
            new_src, status = add_no_mangle_to_fn(modified, fn_name)
            if status not in ("NOT_FOUND",):
                outcomes[fn_name] = f"{fname}: {status}"
            if new_src != modified:
                modified = new_src
        if modified != src:
            with open(fpath, "w", encoding="utf-8") as f:
                f.write(modified)
    return outcomes
