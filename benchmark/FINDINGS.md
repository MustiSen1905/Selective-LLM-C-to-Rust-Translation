# Empirical findings from fresh end-to-end translations

This document records what we learned from running the full pipeline
(`python main.py <project>`) end-to-end after the signature-guard
(PR #1) and FFI-export (PR #2) merges.

## TL;DR

The mechanical signature-guard reduces **caller-cascading compile
errors** to near zero, but the LLM-generated function bodies still
contain **semantic errors** that the guard cannot catch. Two failure
modes were observed:

1. **Compile-clean but semantically wrong** (tiny-aes).
2. **Compile-broken from LLM hallucinations** (pdfresurrect).

This means a clean `cargo check --lib` is **necessary but not
sufficient** evidence of translation correctness; a runtime / property
test is needed for the strong claim.

## Setup

- Toolchain: `nightly-2022-08-08-x86_64-unknown-linux-gnu` (per
  `rust-toolchain.toml`).
- LLM: `deepseek-coder:33b` via local Ollama in WSL.
- Pipeline: `main.py <project>` running c2rust → analysis →
  signature-guarded LLM rewrite → ffi-export post-pass.

## Run 1: tiny-aes (clean E0-count, broken runtime)

| metric | value |
|---|---|
| GuardStats accepted | 3 / 55 attempts (5.5 %) |
| GuardStats reverted-sig (drift) | 52 / 55 (94.5 %) |
| GuardStats reverted-placeholder | 0 |
| `cargo check --lib` errors | **0** |
| Hybrid binary links | yes |
| Hybrid binary functional output | **AES decryption returns wrong plaintext** |

Hybrid `tiny_aes_hybrid` produces `CBC decrypt: FAILURE!` and
`ECB decrypt: FAILURE!`, while the C-only baseline produces
`SUCCESS!` for both. Encryption succeeds in both.

Root cause: in the LLM's translation of `InvShiftRows`, the
`*mut state_t` (= `*mut [[u8; 4]; 4]`) parameter is locally rebound to
a flat `&mut [u8; 16]`:

```rust
unsafe extern "C" fn InvShiftRows(mut state: *mut state_t) {
    let state: &mut [u8; 16] = unsafe { &mut *(state as *mut [u8; 16]) };
    let temp = state[3];          // ❌ should be state[3*4 + 1] = state[13]
    state[3] = state[2];          //    in 2D this is `state[3][1] = state[2][1]`
    state[2] = state[1];
    state[1] = state[0];
    state[0] = temp;
    // ... similar for rows 2 and 3
}
```

The signature is preserved (`*mut state_t`) so the function compiles
and the call sites in `InvCipher` work. But the body's index math
treats the AES state as a flat 16-byte array instead of a column-major
4×4 matrix. Decryption then garbles every block.

This is a translation-quality failure mode that the signature-guard
cannot detect — the function type-checks, the body's free-standing
arithmetic is internally consistent, only the **algorithmic
intent** is wrong. Differential or round-trip testing is required.

## Run 2: pdfresurrect (compile-broken)

| metric | value |
|---|---|
| GuardStats accepted | 14 / 49 attempts (28.6 %) |
| GuardStats reverted-sig (drift) | 34 (69.4 %) |
| GuardStats reverted-placeholder | 1 (2.0 %) |
| `cargo check --lib` errors | 156 |
| Hybrid binary links | no |

Two pipeline-quality bugs were discovered and fixed inline:

1. `is_placeholder_body` did not match the bare `{ ... }` token-stream
   ellipsis the LLM sometimes emits as a body, only the C-style
   `/* ... */` comment form. Extended the regex.
2. The auto-stub-types pass was creating `pub type Safe__IO_FILE = ();`
   stubs (double underscore from `Safe_` + `_IO_FILE` concatenation)
   while the LLM's actual struct is `Safe_IO_FILE`. The collision broke
   `impl ... { ... }` blocks (E0390 on the unit-type alias). Added a
   `normalize_safe_double_underscore` pre-pass that collapses
   `Safe__X` references to `Safe_X` and removes the redundant stub.

After both fixes, 152 compile errors remain. Sample classes:

  - `error[E0425]: cannot find function `null` in this scope`
  - `error[E0609]: no field `flags` on type `&main::_IO_FILE``
  - `error[E0599]: no function or associated item named `from_ptr`
    found for unit type `()``
  - `error[E0063]: missing fields `chain`, `codecvt`, ...`
  - `error[E0599]: no method named `to_str` found for enum `Result``
  - `error[E0599]: no method named `as_ptr` found for struct
    `main::_IO_FILE``
  - `error[E0599]: no method named `find` found for reference `&i8``

These are LLM hallucinations: methods that don't exist on the target
types, missing imports, and incomplete struct initializations. They
are not addressable by the existing mechanical post-passes.

## Implications for the thesis

1. **Compile-error reduction is the right primary metric for
   measuring the signature-guard's effect**, and it shows a clean
   −35.5 % on tiny-aes (PR #1).
2. **Compile correctness is necessary but not sufficient.** The
   tiny-aes run is the worked example: 0 cargo errors, but the AES
   algorithm is broken at the body level. Future work must add a
   functional-equivalence check (differential fuzzing, cryptographic
   test vectors, or property-based tests) as a second filter.
3. **Performance benchmarks require a hand-validated baseline.** The
   PR #1 `pdfresurrect` numbers (median +1.7 % runtime, +21.7 %
   memory) used a working Rust output that was the cumulative result
   of many LLM-translation sessions plus manual fixes; that exact
   state is not reproducible from the current pipeline alone.
   Re-establishing a clean benchmark baseline therefore needs either
   (a) the wrapper-generator for LLM-idiomatic signatures planned in
   PR #2's "next-iteration targets", or (b) a project where the LLM
   does not introduce semantic regressions (smaller / less stateful
   than tiny-aes).

## Pipeline-quality changes from this run

  - `signature_guard.py`: extended `is_placeholder_body` regex to
    match bare `{ ... }`.
  - `translation.py`: new `normalize_safe_double_underscore` pre-pass;
    `inject_safe_stub_types` now calls it first.

These are committed as small follow-up fixes, not as a "the pipeline
now works" claim. The semantic correctness gap above is the dominant
open issue.
