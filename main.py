import sys
import os
import shutil
import analysis.static_analysis as static_analysis
from translation.c2rust import main as c2rust_main
from analysis.graph import run_analysis
from analysis.klee import analyze_ownership as collect_ownership_info
from analysis.unsafe_functions import main as get_unsafe_functions
from translation.translation import main as translate_unsafe_to_safe
import entities.FunctionObject as FunctionObject

def main(project_dir, stop_after_c2rust: bool = False) -> None:
    try:
        isdir = os.path.isdir(f"C-projects/{project_dir}")
        if not isdir:
            raise NotADirectoryError(f"The path {project_dir} is not a valid directory.")

        print(f"Successfully opened project at: {project_dir}")

        files = os.listdir(f"C-projects/{project_dir}")
        files = [f for f in files if f.endswith('.c') or f.endswith('.h')]
        print(f"Project files: {files}")

        # Stage 1 — unsafe function classification
        get_unsafe_functions(f"C-projects/{project_dir}/")

        # Stage 2 — ownership analysis
        print("Starting code analysis...")
        collect_ownership_info(f"C-projects/{project_dir}")

        # Stage 3 — call-graph + topological order
        order = run_analysis(f"C-projects/{project_dir}")

        # Stage 4 — c2rust transpilation
        print("Translating C code to unsafe Rust...")
        c2rust_main(["--input-dir", f"C-projects/{project_dir}",
                     "--config", "unsafe_functions.json",
                     "--out", f"Rust-Outcome/{project_dir}"])

        # Save V-c2rust checkpoint (raw c2rust output, before any LLM rewrite).
        # This snapshot is the V-c2rust variant used by the measurement harness.
        rust_out = f"Rust-Outcome/{project_dir}/rust_out"
        checkpoint = f"Rust-Outcome/{project_dir}/rust_out_c2rust_checkpoint"
        if os.path.isdir(rust_out):
            if os.path.exists(checkpoint):
                shutil.rmtree(checkpoint)
            shutil.copytree(rust_out, checkpoint)
            print(f"V-c2rust checkpoint saved → {checkpoint}")

        if stop_after_c2rust:
            print("Stopped after Stage 4 (--stop-after-c2rust).")
            return

        # Stage 5 — LLM-assisted safe-Rust transformation
        print("Translating unsafe Rust code to safe Rust...")
        translate_unsafe_to_safe(f"Rust-Outcome/{project_dir}", order)

        print("Merging translations into final Rust project...")
        print("Running tests on the final Rust project...")
        print("Project translation and testing completed successfully.")

    except Exception as e:
        print(f"Error opening project: {e}")


if __name__ == "__main__":
    args = sys.argv[1:]
    stop = "--stop-after-c2rust" in args
    args = [a for a in args if a != "--stop-after-c2rust"]
    main(args[0], stop_after_c2rust=stop)