# Selective-LLM-C-to-Rust-Translation

A research pipeline that automatically translates the **unsafe parts** of a C
project to safe Rust, while leaving the rest of the project as C and producing
a hybrid binary. Built around three layers:

1. **Static + ownership analysis** to identify which C functions are unsafe
   and to derive a topological translation order.
2. **c2rust** transpilation of the unsafe subset to bit-equivalent unsafe Rust.
3. **LLM-driven rewrite** (DeepSeek-Coder 33B via Ollama) of the unsafe Rust
   into idiomatic, safer Rust, guarded by a **mechanical
   `signature_guard`** post-processor that snapshots the c2rust signature
   before each LLM call and splices it back if the LLM drifts.

The repository is the empirical part of the Bachelor's thesis
*"Selective Rustification: An Empirical Study of LLMs for Automated Code
Migration"*.

---

## Table of contents

- [Repository layout](#repository-layout)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the pipeline](#running-the-pipeline)
- [Reproducing the experiment](#reproducing-the-experiment)
- [Pipeline internals (the `signature_guard`)](#pipeline-internals-the-signature_guard)
- [Known limitations / findings](#known-limitations--findings)
- [Citation](#citation)

---

## Repository layout

```
.
├── main.py                       # Pipeline entry point: python main.py <project>
├── analysis/
│   ├── unsafe_functions.py       # cppcheck-driven unsafe-fn classifier
│   ├── klee.py                   # ownership analysis (despite the name, no KLEE binary)
│   ├── graph.py + graph-builder/ # call-graph (Rust binary) → topological order
│   ├── cppcheck.py               # static-analysis wrapper
│   └── static_analysis.py
├── translation/
│   ├── c2rust.py                 # invokes c2rust, produces unsafe Rust + safe_*.c stubs
│   ├── create_prompt.py          # prompt builder for the LLM (uses libclang for C ctx)
│   ├── translation.py            # runs the LLM per function, applies post-passes
│   ├── signature_guard.py        # mechanical sig-preservation + body rewriter (PR #1)
│   └── ffi_export.py             # auto-export Rust fns called from C (PR #2)
├── entities/FunctionObject.py    # call-graph node datatype
├── C-projects/                   # input C projects (tiny-aes, abc2mtex, pdfresurrect-master, …)
├── Rust-Outcome/                 # generated hybrid Rust output per project
├── benchmark/
│   ├── build_pdfresurrect.sh     # builds C-only + hybrid pdfresurrect binaries
│   ├── run_pdfresurrect_bench.sh # wall-clock + max-RSS benchmark harness
│   ├── results/*.csv             # raw benchmark data
│   └── FINDINGS.md               # writeup of fresh-translation findings
└── README.md
```

---

## Prerequisites

The pipeline was developed and tested under **WSL2 / Ubuntu 24.04** with the
listed versions; older equivalents may work but are untested.

| Component | Version used | Why |
|---|---|---|
| Linux (WSL2) | Ubuntu 24.04 | gcc, c2rust, cargo all live here |
| Python | 3.12 | Pipeline glue + LLM client |
| Rust toolchain | `nightly-2022-08-08` (pinned in `rust-toolchain.toml`) | c2rust output requires `extern_types`, `c_variadic` |
| `c2rust` | 0.20+ (via `cargo install c2rust`) | C → unsafe Rust transpiler |
| gcc | 13.x | Needed by c2rust + by hybrid binary linking |
| cppcheck | 2.13+ | Static analysis for unsafe-fn classification |
| libclang | LLVM 17 or 18 | C-context extraction during prompt building |
| Ollama daemon | 0.18+ | Local LLM runtime |
| `deepseek-coder:33b` | 18.8 GB model | The translator LLM (only model evaluated) |

GPU recommended for the LLM (CPU inference is ~10× slower per function).

---

## Installation

These steps assume a fresh WSL2 Ubuntu 24.04. Adjust the package names if you
are on a different distribution.

### 1. System packages

```bash
sudo apt update
sudo apt install -y gcc make cppcheck llvm-17-dev clang-17 \
                    python3 python3-venv python3-pip git curl
```

### 2. Rust toolchain + c2rust

```bash
# Install rustup if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Pinned nightly required by c2rust output
rustup toolchain install nightly-2022-08-08
rustup default nightly-2022-08-08

# c2rust
cargo install c2rust
```

### 3. Ollama + LLM model

```bash
# Install Ollama (one-line installer)
curl -fsSL https://ollama.com/install.sh | sh

# Pull the model (≈ 19 GB download)
ollama pull deepseek-coder:33b

# Verify it answers
curl http://127.0.0.1:11434/api/tags | python3 -m json.tool
```

### 4. Project + Python deps

```bash
git clone https://github.com/MustiSen1905/Selective-LLM-C-to-Rust-Translation.git
cd Selective-LLM-C-to-Rust-Translation

python3 -m venv .venv
source .venv/bin/activate
pip install --upgrade pip
pip install ollama tree-sitter tree-sitter-rust clang \
            httpx pydantic certifi
```

### 5. Build the call-graph helper

```bash
cd analysis/graph-builder
cargo build --release
cd ../..
```

---

## Running the pipeline

### Single project

```bash
source .venv/bin/activate
python main.py tiny-AES-c-master
```

`main.py <project>` expects:
- `C-projects/<project>/` to contain the input C source.
- An `unsafe_functions.json` at the repo root that lists which functions per
  file are considered unsafe (the pipeline regenerates this with cppcheck on
  every run, but you can hand-edit it for ablation studies).

It will:
1. Run cppcheck + ownership analysis → fill `unsafe_functions.json`,
   `ownership.json`.
2. Run the call-graph builder → produce a topological function order.
3. Run `c2rust transpile` → write `Rust-Outcome/<project>/rust_out/` with
   `lib.rs`, `src/*.rs`, `safe_*.c` (stripped C kept-helpers), `Cargo.toml`,
   `build.rs`.
4. Run the LLM rewrite per function in topological order, with the
   `signature_guard` post-processor applied to each candidate before it is
   accepted.
5. Run the FFI auto-export pass over the final Rust source.
6. Write `Rust-Outcome/<project>/rust_out/guard_stats.csv` with one row per
   function attempt: `accepted | reverted-sig-drift | reverted-placeholder`.

Total runtime depends mostly on the number of functions and LLM speed:
- tiny-aes (≈ 20 fns) on a 4090: ~5 minutes.
- pdfresurrect (≈ 35 fns) on a 4090: ~10 minutes.
- abc2mtex (≈ 80 fns) on a 4090: ~25 minutes.

### Cargo-checking the result

```bash
cd Rust-Outcome/<project>/rust_out
cargo check --lib
```

`--lib` skips the (sometimes broken) auto-discovered `bin` target. The
hybrid project's `[lib]` is what matters for the empirical part.

---

## Reproducing the experiment

### A — Compile-error reduction (PR #1 main result)

This is the primary metric: how many `cargo check` errors does the
signature-guarded pipeline produce vs. an "always-accept LLM output" baseline?

The PR #1 measurement on tiny-aes gave **104 → 67 errors (−35.5 %)**, and
**100 % of caller-cascading error classes (E0428 / E0061 / E0608)
eliminated**. To reproduce:

```bash
# 1. Run with the guard ENABLED (current pipeline)
python main.py tiny-AES-c-master
cd Rust-Outcome/tiny-AES-c-master/rust_out
rustc --edition=2021 --crate-type=lib --emit=metadata -o /dev/null lib.rs \
    --error-format=short 2>&1 | grep -oE "E[0-9]+" | sort | uniq -c | sort -rn
cat guard_stats.csv | wc -l    # number of guard-decision events
```

For the **without-guard** baseline you would temporarily comment out the
`signature_guard` hook in `translation/translation.py` (around line 1209,
the block beginning `if original_signature is not None:`). Re-run; the
resulting Rust will have far more E0308 / E0428 / E0061 errors that
cascade through the call graph. (We did this once for the empirical
table; we did not check it in as a flag because the wiring is small
enough to do by hand and it keeps the production code-path clean.)

### B — Performance benchmark (pdfresurrect)

The benchmark compares two binaries:

- `pdfresurrect_c`     = original C compiled with `gcc -O2`.
- `pdfresurrect_hybrid` = original `main.c` linked against the
                         Rust staticlib `libhybrid_project.a` (which contains
                         both the LLM-translated functions and the kept-C
                         helpers compiled by `build.rs`).

```bash
# Build both binaries
bash benchmark/build_pdfresurrect.sh

# Run the wall-clock + peak-RSS benchmark (N=15 iterations, 2 warmups)
bash benchmark/run_pdfresurrect_bench.sh

# Output:
#   benchmark/results/pdfresurrect_perf.csv  ← wall-clock per run
#   benchmark/results/pdfresurrect_mem.csv   ← peak RSS
#   stdout: aggregated table (mean + stddev + Δ%)
```

Important: `build_pdfresurrect.sh` strips `static` from `safe_pdf.c`
inside the build (the merged pipeline does this automatically going
forward, see PR #2; the script keeps the strip for reproducibility from
older translation outputs).

The PR #1 numbers (6 PDFs, 230 KB to 89 MB):

| Input | Size MB | C ms | Hybrid ms | Δ time | C MB | Hyb MB | Δ mem |
|---|---|---|---|---|---|---|---|
| bomblab.pdf | 0.23 | 111.05 | 124.76 | +12.3 % | 1.7 | 2.3 | +30.9 % |
| cachelab.pdf | 0.32 | 895.41 | 842.48 | −5.9 % | 2.0 | 2.3 | +15.6 % |
| machines | 6.05 | 1084 | 1111 | +2.5 % | 2.3 | 2.8 | +22.3 % |
| 978-3-031 | 15.42 | 8288 | 7910 | −4.6 % | 2.2 | 2.7 | +21.2 % |
| Analysis | 20.17 | 1868 | 1892 | +1.3 % | 3.4 | 3.7 | +8.8 % |
| Tradingpsy | 88.74 | 64933 | 66062 | +1.7 % | 1.7 | 2.1 | +28.9 % |

Median runtime delta: **+1.7 %**. Median memory delta: **+21.7 %**.
Output is byte-identical to C on all six PDFs.

To reproduce on different PDFs, edit the `PDFS=( ... )` array at the top of
`benchmark/run_pdfresurrect_bench.sh`.

### C — Functional-correctness check (planned future work)

The fresh end-to-end run on tiny-aes recorded in
`benchmark/FINDINGS.md` showed that the hybrid binary can compile cleanly
yet produce the **wrong cryptographic output** (the LLM rewrote
`InvShiftRows` with a flat `[u8; 16]` rebind and 1-D index math where
the original C uses 2-D `state[i][j]`). A planned future-work item is a
small differential-fuzzer that runs both binaries on AES test vectors
and reports any mismatch as a translation defect.

---

## Pipeline internals (the `signature_guard`)

The interesting empirical contribution lives in
`translation/signature_guard.py`. The module sits between the LLM and the
file system: every LLM-emitted function is intercepted and either
accepted, rejected (placeholder body, retry), or **spliced** (signature
drift). The splice produces a synthetic body of the form

```rust
<original c2rust signature> {
    <auto-generated bridge prelude>
    <LLM-emitted body, with raw-pointer accesses rewritten>
}
```

The bridge prelude converts raw-pointer parameters to whatever safe
shape the LLM expected (`*mut Foo` to `&mut SafeFoo` via
`SafeFoo::from_ptr`, `*mut u8` to `&mut [u8; N]` via cast+deref, etc.),
or falls back to a copy alias and rewrites `name[i]` /
`name.field` to `(*name.offset((i) as isize))` /
`(*name).field` so the body still type-checks against the raw pointer.
A `GuardStats` accumulator emits a per-function CSV used in the
empirical chapter.

`translation/ffi_export.py` is the symmetric pass for the back-translation
case: it scans `safe_*.c` for forward-declared functions whose bodies have
been moved to Rust, and ensures the corresponding Rust functions carry
`#[no_mangle] pub extern "C"` so that the C side can find them at link
time. It deliberately skips identifiers inside `extern "C" { ... }`
import blocks and blacklists `main` (Rust binary entry point convention).

---

## Known limitations / findings

`benchmark/FINDINGS.md` contains the full writeup. In short:

1. **Compile-clean is not semantic-clean.** The signature-guard
   guarantees the public API is preserved, but the LLM can still rewrite
   the body of a function with subtly wrong arithmetic (the tiny-aes
   `InvShiftRows` flat-array bug is the worked example). `cargo check`
   alone is therefore a necessary but not sufficient test.
2. **Idiomatic Rust signatures need a wrapper generator.** When the LLM
   emits `pub fn output_transline(s: &str)` while the C side declares
   `void output_transline(char *)`, the FFI auto-export reports
   `IDIOMATIC_SIG (manual wrapper needed)`; generating the
   `&str` ↔ `*const c_char` adapter automatically is future work.
3. **Correctness across translation sessions is unstable.** The LLM is
   non-deterministic; consecutive runs of the same project produce
   different drift rates and different body content. The PR #1
   pdfresurrect performance numbers used a translation snapshot that
   was hand-validated; a fresh end-to-end run today will likely produce
   a hybrid that needs additional manual fixing before it is
   functionally equivalent.

---

## Citation

If you build on this work, please cite the underlying Bachelor's thesis:

```
Mustafa Ali Sen, "Selective Rustification: An Empirical Study of LLMs for
Automated Code Migration", Bachelor's thesis, University of Duisburg-Essen,
2026.
```
