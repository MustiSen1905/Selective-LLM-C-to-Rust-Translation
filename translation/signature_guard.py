"""Mechanische Signature-Preservation und Duplicate-Removal-Passes.

Schluesselbeobachtung aus der empirischen Studie: deepseek-coder:33b ignoriert
"CRITICAL"-Prompt-Regeln zum Signatur-Erhalt in ~90 % der Faelle. Das fuehrt zu
E0308/E0061/E0277/E0428-Kaskaden, weil Caller in anderen Translation-Units
weiterhin die c2rust-Signatur erwarten.

Loesung: 3-Stufen-Verteidigung, die NICHT auf LLM-Compliance angewiesen ist.
  1. Signature-Guard:    Snapshot c2rust-Signatur vor LLM-Call; bei
                         Drift Revert auf Unsafe-Baseline.
  2. Placeholder-Reject: LLM-Output mit Stub-Body (`{ /* ... */ }`,
                         `unimplemented!()`) wird verworfen.
  3. Duplicate-Dedup:    Mehrere `fn NAME` im selben File -> nur die
                         laengste echte Definition behalten (E0428).
"""
from __future__ import annotations
import os
import re
from typing import Optional, Tuple


# ---------------------------------------------------------------------------
# 1. Signature-Extraktion + Vergleich
# ---------------------------------------------------------------------------

# Greift auch attr-prefixed (`#[no_mangle] pub unsafe extern "C" fn ...`).
_FN_HEAD_RE = re.compile(
    r'(?P<head>'
    r'(?:#\[[^\]]+\]\s*)*'
    r'(?:pub(?:\s*\([^)]*\))?\s+)?'
    r'(?:unsafe\s+)?'
    r'(?:extern\s+"[^"]*"\s+)?'
    r'fn\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)'
    r'\s*(?:<[^>]*>)?'      # generics
    r'\s*\('
    r')'
)


def extract_signature(snippet: str) -> Optional[Tuple[str, str, str]]:
    """Liefert (head_until_open_paren, full_signature, body_with_braces).

    `full_signature` ist alles von `fn`/Attributen bis exklusive der oeffnenden
    `{` des Bodies. `body_with_braces` schliesst die `{ ... }` ein.
    Returns None wenn der Snippet keine Funktionsdefinition enthaelt.
    """
    m = _FN_HEAD_RE.search(snippet)
    if not m:
        return None
    sig_start = m.start()

    # Param-Liste: balanciere `(` `)`.
    paren = snippet.find("(", m.end("head") - 1)
    if paren < 0:
        return None
    depth = 0
    i = paren
    while i < len(snippet):
        c = snippet[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                i += 1
                break
        i += 1
    if depth != 0:
        return None
    after_params = i

    # Return-Type + where-Clause: alles bis zur naechsten balancierten `{`.
    j = after_params
    while j < len(snippet) and snippet[j] != "{":
        j += 1
    if j >= len(snippet):
        return None  # nur Deklaration, kein Body

    full_signature = snippet[sig_start:j].rstrip()

    # Body
    depth = 1
    k = j + 1
    while k < len(snippet) and depth > 0:
        c = snippet[k]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
        k += 1
    body = snippet[j:k]
    head = m.group("head").rstrip()
    return head, full_signature, body


# Strip-Regex fuer Vergleich: Whitespace, Attribute, `mut`-Bindings, Lifetimes.
_NORM_STRIP_ATTRS = re.compile(r'#\[[^\]]+\]')
_NORM_STRIP_MUT = re.compile(r'\bmut\s+')
_NORM_WS = re.compile(r'\s+')


def _normalize_param_list(sig: str) -> str:
    """Reduziert die Parameter-Liste auf vergleichbare Form.

    Entfernt:
      - Attribute (`#[unused]`)
      - `mut`-Bindings (`mut foo: T` -> `foo: T`)
      - Lifetime-Annotationen (`<'a>` -> ``)
      - Param-Namen (nur Typen vergleichen — c2rust generiert oft synthetische
        Namen wie `__arg0`).
    """
    paren = sig.find("(")
    if paren < 0:
        return ""
    # rsplit bis matching `)`.
    depth = 0
    end = paren
    for i in range(paren, len(sig)):
        if sig[i] == "(":
            depth += 1
        elif sig[i] == ")":
            depth -= 1
            if depth == 0:
                end = i
                break
    params = sig[paren + 1: end]
    rt = sig[end + 1:].strip()
    # split params on top-level commas.
    parts, cur, d = [], [], 0
    for ch in params:
        if ch in "([<":
            d += 1
        elif ch in ")]>":
            d -= 1
        if ch == "," and d == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
    if cur:
        parts.append("".join(cur))
    norm_parts = []
    for p in parts:
        p = _NORM_STRIP_ATTRS.sub("", p)
        p = _NORM_STRIP_MUT.sub("", p)
        # `name: Type` -> `Type` (split on FIRST `:` at top level)
        d2 = 0
        colon = -1
        for i, ch in enumerate(p):
            if ch in "([<":
                d2 += 1
            elif ch in ")]>":
                d2 -= 1
            elif ch == ":" and d2 == 0:
                colon = i
                break
        if colon >= 0:
            p = p[colon + 1:]
        p = _NORM_WS.sub(" ", p).strip()
        if p:
            norm_parts.append(p)
    rt_norm = _NORM_WS.sub(" ", _NORM_STRIP_ATTRS.sub("", rt)).strip()
    return ",".join(norm_parts) + "|" + rt_norm


def signatures_equivalent(orig_sig: str, llm_sig: str) -> bool:
    """True genau dann, wenn die normalisierten Param-Type-Listen + Returntyp
    identisch sind. ABI-Marker (`unsafe`, `extern "C"`, `pub`) werden absichtlich
    NICHT verglichen — der Body kann safe sein, solange die Caller-Schnittstelle
    bleibt. Param-Namen + `mut` werden ignoriert.
    """
    return _normalize_param_list(orig_sig) == _normalize_param_list(llm_sig)


def _split_params(sig: str) -> list[tuple[str, str]]:
    """Liefert [(name, type)] aus einer Signatur. Ueberspringt `self`-Receiver."""
    paren = sig.find("(")
    if paren < 0:
        return []
    depth, end = 0, paren
    for i in range(paren, len(sig)):
        if sig[i] == "(":
            depth += 1
        elif sig[i] == ")":
            depth -= 1
            if depth == 0:
                end = i
                break
    body = sig[paren + 1:end]
    parts, cur, d = [], [], 0
    for ch in body:
        if ch in "([<":
            d += 1
        elif ch in ")]>":
            d -= 1
        if ch == "," and d == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(ch)
    if cur:
        parts.append("".join(cur))
    out: list[tuple[str, str]] = []
    for p in parts:
        p = _NORM_STRIP_ATTRS.sub("", p).strip()
        if not p or p in ("self", "&self", "&mut self") or p.startswith("self:"):
            continue
        # Strip leading `mut `.
        if p.startswith("mut "):
            p = p[4:]
        # name : type (split at first top-level `:`)
        d2, colon = 0, -1
        for i, ch in enumerate(p):
            if ch in "([<":
                d2 += 1
            elif ch in ")]>":
                d2 -= 1
            elif ch == ":" and d2 == 0:
                colon = i
                break
        if colon < 0:
            continue
        name = p[:colon].strip()
        ty = p[colon + 1:].strip()
        if name and ty:
            out.append((name, ty))
    return out


def build_param_alias_prelude(original_sig: str, llm_sig: str) -> str:
    """Generiert `let llm_name = orig_name;`-Aliases, damit der LLM-Body
    weiter unter seinen eigenen Param-Namen kompiliert, waehrend die
    ABI-Signatur die c2rust-Originalnamen verwendet.

    Beispiel:
      original: `fn f(RoundKey: *mut u8, Key: *const u8)`
      llm     : `fn f(round_key: &mut [u8], key: &[u8])`
      prelude : `let mut round_key = RoundKey; let key = Key;`
    """
    orig_params = _split_params(original_sig)
    llm_params = _split_params(llm_sig)
    if not orig_params or not llm_params:
        return ""
    n = min(len(orig_params), len(llm_params))
    lines = []
    for i in range(n):
        orig_name, _ = orig_params[i]
        llm_name, _ = llm_params[i]
        if orig_name == llm_name:
            continue
        # `mut` damit Body sowohl Read als auch Write-Pattern abdeckt.
        # Pointer sind Copy; Aliasing ist unkritisch.
        lines.append(f"    let mut {llm_name} = {orig_name};")
    if not lines:
        return ""
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------
# Safe-Shadow-Auto-Bridge
# ---------------------------------------------------------------------------

# Erkennt einen `Safe<Name>`-Typ in der Form `&mut SafeFoo`, `& SafeFoo`,
# `Safe_Bar`, etc.
_SAFE_TYPE_RE = re.compile(
    r'(?P<refmut>&\s*(?:mut\s+)?)?(?P<sty>Safe[A-Za-z_][A-Za-z0-9_]*)'
)
# Raw-Pointer auf Struct: `*mut Name`, `*const Name`. T darf KEIN built-in
# Skalar (`u8`, `i32`, `c_char`, ...) sein, sonst entspricht das nicht der
# Safe-Shadow-Konvention.
_RAW_PTR_TO_STRUCT_RE = re.compile(
    r'^\*\s*(?P<mut>mut|const)\s+(?P<ty>[A-Za-z_][A-Za-z0-9_]*)$'
)
_PRIMITIVE_TYPES = {
    "u8", "u16", "u32", "u64", "u128", "usize",
    "i8", "i16", "i32", "i64", "i128", "isize",
    "f32", "f64", "bool", "char", "uint8_t", "uint16_t", "uint32_t",
    "uint64_t", "int8_t", "int16_t", "int32_t", "int64_t", "size_t",
    "c_char", "c_int", "c_uint", "c_void", "c_short", "c_long",
}
# Slice/Array-Refs: `&mut [u8; 16]`, `&[u8; 16]`, `&mut [T; N]`.
_REF_FIXED_ARRAY_RE = re.compile(
    r'^&\s*(?P<mut>mut\s+)?\[(?P<inner>[^;]+);\s*(?P<n>[A-Za-z0-9_]+)\s*\]$'
)
# Single-Element-Ref: `&mut T`, `& T`.
_REF_SINGLE_RE = re.compile(
    r'^&\s*(?P<mut>mut\s+)?(?P<ty>[A-Za-z_][A-Za-z0-9_:<>\s,]*?)$'
)


def _norm_type(t: str) -> str:
    return _NORM_WS.sub(" ", t).strip()


_FROM_PTR_FULL_RE = re.compile(
    r'impl\s+(?P<safe>Safe[A-Za-z_][A-Za-z0-9_]*)\s*\{[^}]*?'
    r'fn\s+from_ptr\s*\(\s*[A-Za-z_][A-Za-z0-9_]*\s*:\s*\*\s*(?:const|mut)\s+'
    r'(?P<inner>[A-Za-z_][A-Za-z0-9_]*)\s*\)\s*->\s*(?P<ret>Option<\s*Self\s*>|Self)',
    re.DOTALL,
)


def scan_from_ptr_returns(file_content: str) -> dict[str, bool]:
    """Liefert {SafeStructName: returns_option}."""
    out = {}
    for m in _FROM_PTR_FULL_RE.finditer(file_content):
        out[m.group("safe")] = m.group("ret").startswith("Option")
    return out


def scan_from_ptr_inner_type(file_content: str) -> dict[str, str]:
    """Liefert {SafeStructName: c2rust_inner_type}. Nutzbar zur Validierung,
    dass die Bridge `SafeX::from_ptr(orig_ptr)` nur emittiert wird, wenn
    `orig_ptr` tatsaechlich ein Pointer auf den erwarteten Inner-Type ist.
    """
    out = {}
    for m in _FROM_PTR_FULL_RE.finditer(file_content):
        out[m.group("safe")] = m.group("inner")
    return out


def _try_bridge(orig_type: str, llm_type: str,
                orig_name: str, llm_name: str,
                from_ptr_returns_option: Optional[dict[str, bool]] = None,
                from_ptr_inner_type: Optional[dict[str, str]] = None,
                ) -> Optional[list[str]]:
    """Liefert Bridge-Statements als Liste von Zeilen, oder None wenn die
    Typkombination nicht mechanisch ueberbrueckbar ist (Fallback: einfacher
    Copy-Alias).
    """
    orig = _norm_type(orig_type)
    llm = _norm_type(llm_type)
    if orig == llm:
        return None  # Caller fragt nur, wenn Drift -> identische Typen seltsam

    # Pattern A: `*mut|*const StructName`  ->  `&mut|& SafeStructName`
    m_orig = _RAW_PTR_TO_STRUCT_RE.match(orig)
    m_llm = _SAFE_TYPE_RE.fullmatch(llm)
    if m_orig and m_llm:
        struct_ty = m_orig.group("ty")
        safe_ty = m_llm.group("sty")
        # Validierung: `SafeS::from_ptr(_: *const X)` muss existieren
        # UND X muss zum orig-Inner-Type passen. Sonst keine Bridge —
        # Fallback auf Copy-Alias verhindert E0308 in der Bridge selbst.
        if struct_ty in _PRIMITIVE_TYPES:
            pass
        elif from_ptr_inner_type is not None:
            expected_inner = from_ptr_inner_type.get(safe_ty)
            if expected_inner is None or expected_inner != struct_ty:
                pass  # Keine passende from_ptr -> Pattern A skippen.
            else:
                mut_kw = "mut " if (m_orig.group("mut") == "mut" or
                                    (m_llm.group("refmut") and "mut" in m_llm.group("refmut"))) else ""
                tmp = f"__sg_{llm_name}"
                returns_option = bool(from_ptr_returns_option and
                                      from_ptr_returns_option.get(safe_ty, False))
                tail = ".unwrap_or_default()" if returns_option else ""
                return [
                    f"    let mut {tmp} = unsafe {{ {safe_ty}::from_ptr({orig_name}) }}{tail};",
                    f"    let {mut_kw}{llm_name} = &{mut_kw.strip()} {tmp};",
                ]
        else:
            # Kein Inner-Type-Probe -> alte Heuristik (best effort).
            mut_kw = "mut " if (m_orig.group("mut") == "mut" or
                                (m_llm.group("refmut") and "mut" in m_llm.group("refmut"))) else ""
            tmp = f"__sg_{llm_name}"
            returns_option = bool(from_ptr_returns_option and
                                  from_ptr_returns_option.get(safe_ty, False))
            tail = ".unwrap_or_default()" if returns_option else ""
            return [
                f"    let mut {tmp} = unsafe {{ {safe_ty}::from_ptr({orig_name}) }}{tail};",
                f"    let {mut_kw}{llm_name} = &{mut_kw.strip()} {tmp};",
            ]

    # Pattern B: `*mut|*const T`  ->  `&mut|& [T; N]` (Fixed-Size-Array-Ref)
    m_arr = _REF_FIXED_ARRAY_RE.match(llm)
    if m_arr and orig.startswith("*"):
        # `*mut u8` -> `&mut [u8; 16]`: cast to array pointer + dereference.
        is_mut = orig.startswith("*mut") or bool(m_arr.group("mut"))
        ref_kw = "&mut " if is_mut else "&"
        ptr_kw = "*mut" if is_mut else "*const"
        inner = m_arr.group("inner").strip()
        n = m_arr.group("n").strip()
        return [
            f"    let {llm_name}: {ref_kw}[{inner}; {n}] = unsafe {{ "
            f"{ref_kw}*({orig_name} as {ptr_kw} [{inner}; {n}]) }};",
        ]

    # Pattern C: `*mut|*const T`  ->  `&mut|& T` (Single-Element-Ref).
    m_single = _REF_SINGLE_RE.match(llm)
    if m_single and orig.startswith("*"):
        # Skip Patterns die schon von A/B abgedeckt sind.
        ll_ty = m_single.group("ty").strip()
        if ll_ty.startswith("[") or ll_ty.startswith("Safe"):
            pass
        else:
            is_mut = (orig.startswith("*mut") and
                      (m_single.group("mut") and "mut" in m_single.group("mut")))
            ref_kw = "&mut " if is_mut else "&"
            return [
                f"    let {llm_name} = unsafe {{ {ref_kw}*{orig_name} }};",
            ]

    # Pattern D: `*mut T`  ->  `&mut [T]` (unsized slice). Laenge unbekannt
    # -> fall through zu None (Caller faellt auf Copy-Alias zurueck).
    return None


def build_safe_bridge_prelude_v2(original_sig: str, llm_sig: str,
                                 file_content: Optional[str] = None
                                 ) -> tuple[str, set[str]]:
    """Wie `build_safe_bridge_prelude`, aber liefert zusaetzlich die Menge der
    Body-sichtbaren Namen, die NACH dem Prelude noch Raw-Pointer sind.
    Diese Namen sollten via `rewrite_raw_ptr_accesses` post-bearbeitet werden,
    damit `name[i]`/`name.field` zu `*name.add(i)`/`(*name).field` wird.
    """
    orig_params = _split_params(original_sig)
    llm_params = _split_params(llm_sig)
    raw_ptr_names: set[str] = set()
    if not orig_params or not llm_params:
        return "", raw_ptr_names
    fp_returns = scan_from_ptr_returns(file_content) if file_content else {}
    fp_inner = scan_from_ptr_inner_type(file_content) if file_content else {}
    n = min(len(orig_params), len(llm_params))
    lines: list[str] = []
    for i in range(n):
        orig_name, orig_ty = orig_params[i]
        llm_name, llm_ty = llm_params[i]
        orig_is_raw_ptr = _norm_type(orig_ty).startswith("*")
        if _norm_type(orig_ty) == _norm_type(llm_ty):
            if orig_name != llm_name:
                lines.append(f"    let mut {llm_name} = {orig_name};")
            if orig_is_raw_ptr:
                raw_ptr_names.add(llm_name)
            continue
        bridge = _try_bridge(orig_ty, llm_ty, orig_name, llm_name,
                             fp_returns, fp_inner)
        if bridge is not None:
            lines.extend(bridge)
        else:
            if orig_name != llm_name:
                lines.append(f"    let mut {llm_name} = {orig_name};")
            if orig_is_raw_ptr:
                raw_ptr_names.add(llm_name)
    prelude = ("\n".join(lines) + "\n") if lines else ""
    return prelude, raw_ptr_names


def build_safe_bridge_prelude(original_sig: str, llm_sig: str,
                              file_content: Optional[str] = None) -> str:
    """Type-aware Bridge zwischen Original-Signatur und LLM-Body.

    Fuer jeden Param, der in beiden Signaturen vorkommt, wird die beste
    mechanische Konversion erzeugt:
      *mut Foo  -> &mut SafeFoo:  via SafeFoo::from_ptr() (mit `.unwrap_or_default()`
                                  falls die im File definierte from_ptr `Option<Self>`
                                  zurueckgibt).
      *mut u8   -> &mut [u8; N]:  Pointer-Cast + Deref.
      *mut T    -> &mut T:        einfaches Deref.
    Sonst Fallback auf reinen Name-Alias (`let llm = orig;`).
    """
    orig_params = _split_params(original_sig)
    llm_params = _split_params(llm_sig)
    if not orig_params or not llm_params:
        return ""
    fp_map = scan_from_ptr_returns(file_content) if file_content else {}
    n = min(len(orig_params), len(llm_params))
    lines: list[str] = []
    for i in range(n):
        orig_name, orig_ty = orig_params[i]
        llm_name, llm_ty = llm_params[i]
        if _norm_type(orig_ty) == _norm_type(llm_ty):
            # Typen identisch; nur Name-Alias falls noetig.
            if orig_name != llm_name:
                lines.append(f"    let mut {llm_name} = {orig_name};")
            continue
        bridge = _try_bridge(orig_ty, llm_ty, orig_name, llm_name, fp_map)
        if bridge is not None:
            lines.extend(bridge)
        else:
            # Fallback: Copy-Alias (Pointer ist Copy). Body-Typfehler bleiben
            # lokal.
            if orig_name != llm_name:
                lines.append(f"    let mut {llm_name} = {orig_name};")
    if not lines:
        return ""
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------
# Body-Rewriter fuer raw-pointer Variablen
# ---------------------------------------------------------------------------

def _find_balanced_close(text: str, open_pos: int, open_ch: str, close_ch: str) -> int:
    """Index der zur oeffnenden `[`/`(` gehoerenden schliessenden Klammer
    (Berechnung mit Klammern-Zaehler). -1 wenn nicht gefunden.
    """
    depth = 0
    i = open_pos
    while i < len(text):
        c = text[i]
        if c == open_ch:
            depth += 1
        elif c == close_ch:
            depth -= 1
            if depth == 0:
                return i
        i += 1
    return -1


def _find_local_shadows(body: str, names: set[str]) -> dict[str, int]:
    """Sucht `let NAME = ...` Deklarationen, die die aeusseren Param-Namen
    ueberschatten. Liefert {name: position_after_shadow}. Nach dieser
    Position soll der Rewriter den Namen NICHT mehr anfassen.
    """
    out: dict[str, int] = {}
    if not names:
        return out
    # Match `let [mut] NAME` ohne Pfade/Patterns.
    pat = re.compile(r'\blet\s+(?:mut\s+)?(' +
                     "|".join(re.escape(n) for n in names) + r')\b')
    for m in pat.finditer(body):
        n = m.group(1)
        # Nimm die FRUEHESTE Shadow-Position (alles danach ueberschattet).
        if n not in out or m.start() < out[n]:
            out[n] = m.start()
    return out


def rewrite_raw_ptr_accesses(body: str, raw_ptr_names: set[str]) -> str:
    """Sucht im Body alle `name[expr]` und `name.field` fuer Namen aus
    `raw_ptr_names` und rewrited:
        name[expr]   ->  (*name.offset((expr) as isize))
        name.field   ->  (*name).field
    Innerhalb von String-Literals + Kommentaren wird NICHT rewrited.
    Nach lokalem `let NAME = ...` wird der Name nicht mehr beruehrt
    (Shadowing-Schutz).
    """
    if not raw_ptr_names:
        return body
    shadows = _find_local_shadows(body, raw_ptr_names)
    out = []
    i = 0
    L = len(body)
    name_re = re.compile(
        r'\b(' + "|".join(re.escape(n) for n in sorted(raw_ptr_names, key=len, reverse=True))
        + r')\b'
    )
    while i < L:
        c = body[i]
        # String-Literals
        if c == '"':
            j = i + 1
            while j < L and body[j] != '"':
                if body[j] == "\\":
                    j += 2
                else:
                    j += 1
            j = min(j + 1, L)
            out.append(body[i:j])
            i = j
            continue
        # Block-Kommentar
        if c == "/" and i + 1 < L and body[i + 1] == "*":
            j = body.find("*/", i + 2)
            if j < 0:
                j = L
            else:
                j += 2
            out.append(body[i:j])
            i = j
            continue
        # Line-Kommentar
        if c == "/" and i + 1 < L and body[i + 1] == "/":
            j = body.find("\n", i + 2)
            if j < 0:
                j = L
            else:
                j += 1
            out.append(body[i:j])
            i = j
            continue
        # Identifier-Match an Position i?
        m = name_re.match(body, i)
        if m:
            name = m.group(1)
            end = m.end()
            # Shadow-Schutz: nach `let NAME = ...` ist der Name lokal.
            shadow_pos = shadows.get(name)
            if shadow_pos is not None and i >= shadow_pos:
                out.append(name)
                i = end
                continue
            # Lookback fuer `let NAME =`, `fn NAME`, `mut NAME`, etc.
            look = body[max(0, i - 8): i]
            if re.search(r'\b(let|fn|mut|struct|enum)\s+$', look):
                out.append(name)
                i = end
                continue
            # Indexing `name[`?
            if end < L and body[end] == "[":
                close = _find_balanced_close(body, end, "[", "]")
                if close > end:
                    expr = body[end + 1: close]
                    # Parens essenziell wegen Operator-Precedence: `*p.offset(i)[j]`
                    # parsed `*(p.offset(i)[j])`, nicht `(*p.offset(i))[j]`.
                    out.append(f"(*{name}.offset(({expr}) as isize))")
                    i = close + 1
                    continue
            # Field/Method `name.<ident>`?
            if end + 1 < L and body[end] == "." and (body[end + 1].isalpha() or body[end + 1] == "_"):
                out.append(f"(*{name})")
                i = end
                continue
            # Sonst: Name unveraendert lassen.
            out.append(name)
            i = end
            continue
        # Default: Char durchreichen
        out.append(c)
        i += 1
    return "".join(out)


# ---------------------------------------------------------------------------
# 2. Placeholder-Body-Detector
# ---------------------------------------------------------------------------

_PLACEHOLDER_BODY_RE = re.compile(
    r'^\s*\{\s*'
    r'(?:'
    r'\.\.\.'                       # bare `...` (LLM ellipsis-as-placeholder)
    r'|/\*\s*\.\.\.\s*\*/'           # /* ... */
    r'|/\*\s*TODO[^*]*\*/'
    r'|//[^\n]*\n\s*'
    r'|unimplemented!\(\s*\)\s*;?'
    r'|todo!\(\s*\)\s*;?'
    r'|panic!\([^)]*\)\s*;?'
    r')\s*\}\s*$'
)


def is_placeholder_body(body: str) -> bool:
    """True wenn der Body inhaltsleer / Stub ist (E0428-Quelle bei Doppel-Defs)."""
    return bool(_PLACEHOLDER_BODY_RE.match(body))


# ---------------------------------------------------------------------------
# 3. Duplicate-Function-Dedup
# ---------------------------------------------------------------------------

def dedup_function_definitions(rust_src_dir: str) -> int:
    """Findet `fn NAME` mit >1 Definition pro Datei. Entfernt Stub-Definitionen
    (Placeholder-Body) bevorzugt; sonst die kuerzere. Liefert Anzahl entfernter
    Duplikate.
    """
    if not os.path.isdir(rust_src_dir):
        return 0
    removed = 0
    # Walk subdirectories so projects with nested source trees (e.g. libtiff
    # with `libtiff/`, `tools/`, `port/`) are fully covered. The callers
    # already pass the project's `src/` root.
    rs_files = []
    for root, _, files in os.walk(rust_src_dir):
        for f in sorted(files):
            if f.endswith(".rs"):
                rs_files.append((f, os.path.join(root, f)))
    for fname, fpath in rs_files:
        with open(fpath, "r", encoding="utf-8") as f:
            code = f.read()

        # Sammle alle (start, end, name, body) Tripel.
        defs = []
        idx = 0
        while True:
            m = _FN_HEAD_RE.search(code, idx)
            if not m:
                break
            sub = code[m.start():]
            ext = extract_signature(sub)
            if ext is None:
                idx = m.end()
                continue
            head, sig, body = ext
            full = sig + body
            start = m.start()
            end = start + len(full)
            defs.append((start, end, m.group("name"), body))
            idx = end

        # Gruppiere nach Name.
        by_name = {}
        for d in defs:
            by_name.setdefault(d[2], []).append(d)

        to_remove = []
        for name, group in by_name.items():
            if len(group) <= 1:
                continue
            # Bewerte: Stub > kuerzer ist schlechter.
            scored = [
                (is_placeholder_body(d[3]), len(d[3]), d) for d in group
            ]
            # Behalte den Eintrag mit (placeholder=False, max body length).
            scored.sort(key=lambda x: (x[0], -x[1]))  # placeholder LAST -> keep first non-placeholder
            keep = scored[0][2]
            for _, _, d in scored[1:]:
                to_remove.append(d)

        if not to_remove:
            continue

        # Entferne von hinten nach vorne (Indices stabil halten).
        to_remove.sort(key=lambda d: d[0], reverse=True)
        for start, end, name, body in to_remove:
            code = code[:start].rstrip() + "\n\n" + code[end:].lstrip()
            removed += 1
            print(f"  [+] Dedup: zweite Definition von `fn {name}` in {fname} entfernt"
                  f" ({'Placeholder' if is_placeholder_body(body) else 'kuerzere Variante'})")

        with open(fpath, "w", encoding="utf-8") as f:
            f.write(code)
    return removed


# ---------------------------------------------------------------------------
# 4. Sidecar-Stats fuer Thesis-Reporting
# ---------------------------------------------------------------------------

class GuardStats:
    """Akkumuliert Reverts/Akzeptanzen ueber den Pipeline-Lauf.

    Wird von `process_single_function` befuellt und kann am Ende fuer das
    Thesis-Reporting (Tabelle "Mechanical Guard Rejection Rate") als CSV
    geschrieben werden.
    """
    def __init__(self):
        self.accepted: list[tuple[str, str]] = []
        self.reverted_signature: list[tuple[str, str, str, str]] = []
        self.reverted_placeholder: list[tuple[str, str]] = []

    def log_accept(self, project: str, func: str):
        self.accepted.append((project, func))

    def log_revert_signature(self, project: str, func: str,
                             original_sig: str, llm_sig: str):
        self.reverted_signature.append((project, func, original_sig, llm_sig))

    def log_revert_placeholder(self, project: str, func: str):
        self.reverted_placeholder.append((project, func))

    def write_csv(self, path: str):
        import csv
        with open(path, "w", encoding="utf-8", newline="") as f:
            w = csv.writer(f)
            w.writerow(["project", "function", "outcome",
                        "original_signature", "llm_signature"])
            for p, fn in self.accepted:
                w.writerow([p, fn, "ACCEPTED", "", ""])
            for p, fn, o, l in self.reverted_signature:
                w.writerow([p, fn, "REVERTED_SIG_DRIFT", o, l])
            for p, fn in self.reverted_placeholder:
                w.writerow([p, fn, "REVERTED_PLACEHOLDER", "", ""])

    def summary(self) -> str:
        n_acc = len(self.accepted)
        n_sig = len(self.reverted_signature)
        n_ph = len(self.reverted_placeholder)
        total = n_acc + n_sig + n_ph
        if total == 0:
            return "GuardStats: no functions processed"
        pct = lambda x: f"{100 * x / total:.1f}%"
        return (f"GuardStats: {n_acc} accepted ({pct(n_acc)}), "
                f"{n_sig} reverted-sig ({pct(n_sig)}), "
                f"{n_ph} reverted-placeholder ({pct(n_ph)}), "
                f"total {total}")


# Globale Instanz, von translation.py befuellt.
GUARD_STATS = GuardStats()
