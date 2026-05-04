#!/usr/bin/env python3
"""
MiBench Translation Pipeline.

For each (or a specified) MiBench program this script:
  1. Copies the relevant C source files into C-projects/mibench_<NAME>/
     (flat layout — required so main.py and translation.py resolve paths
     by project basename without modification)
  2. Runs Stage 1: unsafe-function heuristic classification
     → writes unsafe_functions.json
  3. Runs Stage 2: ownership analysis (KLEE — skipped if unavailable)
  4. Runs Stage 3: call-graph + topological sort  (graph-builder)
  5. Runs Stage 4: c2rust transpilation
     → writes Rust-Outcome/mibench_<NAME>/rust_out/
     → snapshots V-c2rust checkpoint
  6. Optionally runs Stage 5: LLM rewrite (deepseek-coder via Ollama)
     → requires `ollama serve` + deepseek-coder:33b pulled

Usage (from project root):
    python3 translation/mibench_pipeline.py basicmath [--stop-after-c2rust]
    python3 translation/mibench_pipeline.py --all     [--stop-after-c2rust]
    python3 translation/mibench_pipeline.py --list
"""
import os
import sys
import shutil
from pathlib import Path

# ---------------------------------------------------------------------------
# Source-file manifest for each project
# key  = short project name  (used as C-projects/mibench_<key> directory)
# value = (subdir_in_mibench, [files_to_copy])
#         files_to_copy may contain globs (* = all *.c / *.h in that subdir)
# ---------------------------------------------------------------------------
MIBENCH_ROOT = Path("C-projects/mibench-master")

PROJECTS = {
    "basicmath": (
        "automotive/basicmath",
        ["basicmath.c", "cubic.c", "isqrt.c", "rad2deg.c",
         "snipmath.h", "sniptype.h", "pi.h", "round.h"],
    ),
    "bitcount": (
        "automotive/bitcount",
        ["bitcnt_1.c", "bitcnt_2.c", "bitcnt_3.c", "bitcnt_4.c",
         "bitcnts.c", "bitfiles.c", "bitstrng.c", "bstr_i.c",
         "bitops.h", "conio.h", "extkword.h", "sniptype.h"],
    ),
    # Only qsort_small.c — qsort_large.c has its own main() and would
    # cause a duplicate-symbol error if both are included.
    "qsort_small": (
        "automotive/qsort",
        ["qsort_small.c"],
    ),
    "susan": (
        "automotive/susan",
        ["susan.c"],
    ),
    "dijkstra": (
        "network/dijkstra",
        ["dijkstra.c"],
    ),
    "patricia": (
        "network/patricia",
        ["patricia.c", "patricia_test.c", "patricia.h"],
    ),
    "stringsearch": (
        "office/stringsearch",
        ["bmhasrch.c", "bmhisrch.c", "bmhsrch.c", "pbmsrch.c", "search.h"],
    ),
    "blowfish": (
        "security/blowfish",
        ["bf.c", "bf_skey.c", "bf_ecb.c", "bf_enc.c",
         "bf_cbc.c", "bf_cfb64.c", "bf_ofb64.c",
         "blowfish.h", "bf_locl.h", "bf_pi.h"],
    ),
    "sha": (
        "security/sha",
        ["sha.c", "sha_driver.c", "sha.h"],
    ),
    "crc": (
        "telecomm/CRC32",
        ["crc_32.c", "crc.h", "sniptype.h"],
    ),
    "fft": (
        "telecomm/FFT",
        ["main.c", "fftmisc.c", "fourierf.c",
         "ddc.h", "fourier.h", "ddcmath.h"],
    ),
}


# ---------------------------------------------------------------------------
# Stage helpers
# ---------------------------------------------------------------------------

def stage_copy(name: str) -> Path:
    """Copy project source files into C-projects/mibench_<name>/."""
    subdir, files = PROJECTS[name]
    src_dir = MIBENCH_ROOT / subdir
    dst_dir = Path(f"C-projects/mibench_{name}")
    dst_dir.mkdir(parents=True, exist_ok=True)

    copied, missing = [], []
    for f in files:
        src = src_dir / f
        if src.exists():
            shutil.copy2(src, dst_dir / f)
            copied.append(f)
        else:
            missing.append(f)

    print(f"  Copied {len(copied)} files to {dst_dir}")
    if missing:
        print(f"  WARN: missing source files: {missing}")
    return dst_dir


def stage_unsafe_functions(c_dir: str):
    """Stage 1 — heuristic unsafe-function classification."""
    from analysis.unsafe_functions import main as get_unsafe_functions
    get_unsafe_functions(c_dir + "/")


def stage_ownership(c_dir: str):
    """Stage 2 — KLEE ownership analysis (optional, silently skipped)."""
    try:
        from analysis.klee import analyze_ownership as collect_ownership_info
        collect_ownership_info(c_dir)
    except Exception as e:
        print(f"  [WARN] Ownership analysis skipped: {e}")


def stage_graph(c_dir: str):
    """Stage 3 — call-graph + topological order."""
    from analysis.graph import run_analysis
    return run_analysis(c_dir)


def stage_c2rust(c_dir: str, out_dir: str):
    """Stage 4 — split C files + c2rust transpilation + checkpoint."""
    from translation.c2rust import main as c2rust_main
    c2rust_main([
        "--input-dir", c_dir,
        "--config",    "unsafe_functions.json",
        "--out",       out_dir,
    ])
    # Snapshot V-c2rust checkpoint
    rust_out   = Path(out_dir) / "rust_out"
    checkpoint = Path(out_dir) / "rust_out_c2rust_checkpoint"
    if rust_out.is_dir():
        if checkpoint.exists():
            shutil.rmtree(checkpoint)
        shutil.copytree(rust_out, checkpoint)
        print(f"  V-c2rust checkpoint → {checkpoint}")
    else:
        print("  WARN: rust_out not found after c2rust step")


def stage_llm(out_dir: str, order):
    """Stage 5 — LLM rewrite (requires ollama + deepseek-coder:33b)."""
    from translation.translation import main as translate_unsafe_to_safe
    translate_unsafe_to_safe(out_dir, order)


# ---------------------------------------------------------------------------
# Per-project runner
# ---------------------------------------------------------------------------

def run_project(name: str, stop_after_c2rust: bool = False) -> bool:
    if name not in PROJECTS:
        print(f"Unknown project: '{name}'. Use --list to see available projects.")
        return False

    print(f"\n{'='*60}")
    print(f"  MiBench Pipeline: {name}")
    print(f"{'='*60}")

    # Stage 0 — copy source
    print("\n[0] Copying source files")
    dst_dir = stage_copy(name)
    c_dir   = str(dst_dir)
    out_dir = f"Rust-Outcome/mibench_{name}"

    # Stage 1
    print("\n[1] Unsafe-function classification")
    stage_unsafe_functions(c_dir)

    # Stage 2
    print("\n[2] Ownership analysis")
    stage_ownership(c_dir)

    # Stage 3
    print("\n[3] Call-graph analysis")
    order = stage_graph(c_dir)
    print(f"  Function order: {[f.name for f in (order or [])]}")

    # Stage 4
    print("\n[4] c2rust transpilation")
    stage_c2rust(c_dir, out_dir)

    if stop_after_c2rust:
        print(f"\n  Stopped after Stage 4 (--stop-after-c2rust).")
        print(f"  Output: {out_dir}")
        return True

    # Stage 5
    print("\n[5] LLM rewrite (deepseek-coder via Ollama)")
    try:
        stage_llm(out_dir, order)
    except Exception as e:
        print(f"  ERROR in LLM stage: {e}")
        print("  Tip: make sure 'ollama serve' is running and deepseek-coder:33b is pulled.")
        return False

    print(f"\n{'='*60}")
    print(f"  Done: {name}  →  {out_dir}")
    print(f"{'='*60}")
    return True


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main():
    args = sys.argv[1:]

    if not args or "--help" in args or "-h" in args:
        print(__doc__)
        sys.exit(0)

    if "--list" in args:
        print("Available MiBench projects:")
        for n, (sub, _) in PROJECTS.items():
            print(f"  {n:<18}  ({sub})")
        sys.exit(0)

    stop = "--stop-after-c2rust" in args
    args = [a for a in args if a != "--stop-after-c2rust"]

    # Ensure we run from the project root
    script_dir = Path(__file__).resolve().parent.parent
    os.chdir(script_dir)

    if args[0] == "--all":
        results = {}
        for name in PROJECTS:
            ok = run_project(name, stop)
            results[name] = "OK" if ok else "FAILED"
        print("\n=== Summary ===")
        for n, s in results.items():
            print(f"  {n:<18}  {s}")
    else:
        ok = run_project(args[0], stop)
        sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
