#!/usr/bin/env python3
"""
Measurement harness analyzer — produces Chapter-5 tables from CSV results.

Usage:
    python3 benchmark/harness/analyze.py [results_dir]
    python3 benchmark/harness/analyze.py [results_dir] --latex

Output:
    - Console: formatted text tables (V-C / V-c2rust / V-Hybrid comparison)
    - --latex flag: also writes benchmark/results/tables.tex
"""
import csv
import os
import sys
import statistics
from collections import defaultdict
from pathlib import Path

RESULTS_DIR = Path(sys.argv[1]) if len(sys.argv) > 1 and not sys.argv[1].startswith("--") else \
              Path(__file__).parent.parent / "results"
LATEX = "--latex" in sys.argv

VARIANT_ORDER = ["c", "c2rust", "hybrid"]
VARIANT_LABELS = {"c": "V-C", "c2rust": "V-c2rust", "hybrid": "V-Hybrid"}


# ---------------------------------------------------------------------------
# Data loading
# ---------------------------------------------------------------------------

def load_perf(results_dir: Path) -> dict:
    """Returns {project: {variant: {workload: [mean_s, ...]}}}"""
    data = defaultdict(lambda: defaultdict(lambda: defaultdict(list)))
    for csv_file in sorted(results_dir.glob("*_perf.csv")):
        with open(csv_file) as f:
            for row in csv.DictReader(f):
                proj = row["project"]
                var  = row["variant"]
                wl   = row["workload"]
                data[proj][var][wl].append(float(row["mean_s"]))
    return data


def load_mem(results_dir: Path) -> dict:
    """Returns {project: {variant: {workload: rss_kb}}}"""
    data = defaultdict(lambda: defaultdict(dict))
    for csv_file in sorted(results_dir.glob("*_mem.csv")):
        with open(csv_file) as f:
            for row in csv.DictReader(f):
                proj = row["project"]
                var  = row["variant"]
                wl   = row["workload"]
                try:
                    data[proj][var][wl] = int(row["max_rss_kb"])
                except (ValueError, KeyError):
                    pass
    return data


def load_guard_stats(repo_root: Path) -> dict:
    """Returns {project: {accepted, reverted_sig, reverted_placeholder, total}}"""
    stats = {}
    for csv_file in sorted(repo_root.glob("Rust-Outcome/*/rust_out/guard_stats.csv")):
        project = csv_file.parts[-3]
        accepted = reverted_sig = reverted_ph = 0
        with open(csv_file) as f:
            for row in csv.DictReader(f):
                o = row.get("outcome", "")
                if o == "ACCEPTED":
                    accepted += 1
                elif o == "REVERTED_SIG_DRIFT":
                    reverted_sig += 1
                elif o == "REVERTED_PLACEHOLDER":
                    reverted_ph += 1
        total = accepted + reverted_sig + reverted_ph
        if total > 0:
            stats[project] = {
                "accepted": accepted,
                "reverted_sig": reverted_sig,
                "reverted_placeholder": reverted_ph,
                "total": total,
            }
    return stats


# ---------------------------------------------------------------------------
# Formatting helpers
# ---------------------------------------------------------------------------

def fmt_delta(val_new, val_ref, unit="%"):
    if val_ref == 0:
        return "n/a"
    delta = (val_new - val_ref) / val_ref * 100
    sign = "+" if delta >= 0 else ""
    return f"{sign}{delta:.1f}{unit}"


def col(s, w):
    return str(s)[:w].ljust(w)


def rcol(s, w):
    return str(s)[:w].rjust(w)


def separator(widths):
    return "+" + "+".join("-" * (w + 2) for w in widths) + "+"


def header_row(cells, widths):
    parts = []
    for c_text, w in zip(cells, widths):
        parts.append(f" {c_text[:w].center(w)} ")
    return "|" + "|".join(parts) + "|"


def data_row(cells, widths, right_align_from=2):
    parts = []
    for i, (c_text, w) in enumerate(zip(cells, widths)):
        if i >= right_align_from:
            parts.append(f" {str(c_text)[:w].rjust(w)} ")
        else:
            parts.append(f" {str(c_text)[:w].ljust(w)} ")
    return "|" + "|".join(parts) + "|"


# ---------------------------------------------------------------------------
# Table 1: GuardStats (RQ1 input)
# ---------------------------------------------------------------------------

def print_guard_stats_table(guard_stats: dict):
    if not guard_stats:
        print("  [No guard_stats.csv found]\n")
        return

    widths = [26, 8, 14, 20, 8, 10]
    hdrs = ["Project", "Total", "Accepted", "Reverted (sig)", "Reverted (ph)", "Drift %"]
    print(separator(widths))
    print(header_row(hdrs, widths))
    print(separator(widths))
    for proj, s in sorted(guard_stats.items()):
        drift_pct = s["reverted_sig"] / s["total"] * 100 if s["total"] > 0 else 0
        print(data_row([
            proj,
            s["total"],
            s["accepted"],
            s["reverted_sig"],
            s["reverted_placeholder"],
            f"{drift_pct:.1f}%",
        ], widths))
    print(separator(widths))
    print()


# ---------------------------------------------------------------------------
# Table 2: Runtime comparison per project (RQ2)
# ---------------------------------------------------------------------------

def print_runtime_table(perf: dict):
    if not perf:
        print("  [No *_perf.csv found]\n")
        return

    for project in sorted(perf):
        vdata = perf[project]
        all_workloads = sorted({wl for var in vdata.values() for wl in var})
        if not all_workloads:
            continue

        print(f"  Project: {project}")
        widths = [36, 10, 10, 10, 9, 9, 9]
        hdrs = ["Workload", "V-C ms", "Vc2r ms", "Vhyb ms", "Δc2r", "Δhyb", "Δhyb/c2r"]
        print(separator(widths))
        print(header_row(hdrs, widths))
        print(separator(widths))

        c_all = []; hyb_all = []; c2r_all = []
        for wl in all_workloads:
            # Use median of reported mean_s values per workload/variant
            def get_ms(variant):
                vals = vdata.get(variant, {}).get(wl, [])
                if not vals:
                    return None
                return statistics.median(vals) * 1000

            c_ms   = get_ms("c")
            c2r_ms = get_ms("c2rust")
            hyb_ms = get_ms("hybrid")

            wl_short = os.path.basename(wl) if "/" in wl else wl
            row = [
                wl_short,
                f"{c_ms:.1f}" if c_ms is not None else "—",
                f"{c2r_ms:.1f}" if c2r_ms is not None else "—",
                f"{hyb_ms:.1f}" if hyb_ms is not None else "—",
                fmt_delta(c2r_ms, c_ms) if (c2r_ms and c_ms) else "—",
                fmt_delta(hyb_ms, c_ms) if (hyb_ms and c_ms) else "—",
                fmt_delta(hyb_ms, c2r_ms) if (hyb_ms and c2r_ms) else "—",
            ]
            print(data_row(row, widths))
            if c_ms:   c_all.append(c_ms)
            if hyb_ms: hyb_all.append(hyb_ms)
            if c2r_ms: c2r_all.append(c2r_ms)

        print(separator(widths))

        # Median summary row
        c_med   = statistics.median(c_all)   if c_all   else None
        c2r_med = statistics.median(c2r_all) if c2r_all else None
        hyb_med = statistics.median(hyb_all) if hyb_all else None
        row = [
            "MEDIAN",
            f"{c_med:.1f}" if c_med else "—",
            f"{c2r_med:.1f}" if c2r_med else "—",
            f"{hyb_med:.1f}" if hyb_med else "—",
            fmt_delta(c2r_med, c_med) if (c2r_med and c_med) else "—",
            fmt_delta(hyb_med, c_med) if (hyb_med and c_med) else "—",
            fmt_delta(hyb_med, c2r_med) if (hyb_med and c2r_med) else "—",
        ]
        print(data_row(row, widths))
        print(separator(widths))
        print()


# ---------------------------------------------------------------------------
# Table 3: Memory comparison per project (RQ3)
# ---------------------------------------------------------------------------

def print_memory_table(mem: dict):
    if not mem:
        print("  [No *_mem.csv found]\n")
        return

    for project in sorted(mem):
        vdata = mem[project]
        all_workloads = sorted({wl for var in vdata.values() for wl in var})
        if not all_workloads:
            continue

        print(f"  Project: {project}")
        widths = [36, 9, 9, 9, 8, 8, 8]
        hdrs = ["Workload", "V-C MB", "Vc2r MB", "Vhyb MB", "Δc2r", "Δhyb", "Δhyb/c2r"]
        print(separator(widths))
        print(header_row(hdrs, widths))
        print(separator(widths))

        c_all = []; hyb_all = []; c2r_all = []
        for wl in all_workloads:
            def get_mb(variant):
                rss_kb = vdata.get(variant, {}).get(wl)
                return rss_kb / 1024 if rss_kb else None

            c_mb   = get_mb("c")
            c2r_mb = get_mb("c2rust")
            hyb_mb = get_mb("hybrid")

            wl_short = os.path.basename(wl) if "/" in wl else wl
            row = [
                wl_short,
                f"{c_mb:.1f}" if c_mb else "—",
                f"{c2r_mb:.1f}" if c2r_mb else "—",
                f"{hyb_mb:.1f}" if hyb_mb else "—",
                fmt_delta(c2r_mb, c_mb) if (c2r_mb and c_mb) else "—",
                fmt_delta(hyb_mb, c_mb) if (hyb_mb and c_mb) else "—",
                fmt_delta(hyb_mb, c2r_mb) if (hyb_mb and c2r_mb) else "—",
            ]
            print(data_row(row, widths))
            if c_mb:   c_all.append(c_mb)
            if hyb_mb: hyb_all.append(hyb_mb)
            if c2r_mb: c2r_all.append(c2r_mb)

        print(separator(widths))

        c_med   = statistics.median(c_all)   if c_all   else None
        c2r_med = statistics.median(c2r_all) if c2r_all else None
        hyb_med = statistics.median(hyb_all) if hyb_all else None
        row = [
            "MEDIAN",
            f"{c_med:.1f}" if c_med else "—",
            f"{c2r_med:.1f}" if c2r_med else "—",
            f"{hyb_med:.1f}" if hyb_med else "—",
            fmt_delta(c2r_med, c_med) if (c2r_med and c_med) else "—",
            fmt_delta(hyb_med, c_med) if (hyb_med and c_med) else "—",
            fmt_delta(hyb_med, c2r_med) if (hyb_med and c2r_med) else "—",
        ]
        print(data_row(row, widths))
        print(separator(widths))
        print()


# ---------------------------------------------------------------------------
# Optional: LaTeX table export
# ---------------------------------------------------------------------------

def write_latex_tables(perf: dict, mem: dict, guard_stats: dict, out_path: Path):
    lines = []
    lines.append(r"% Auto-generated by benchmark/harness/analyze.py")
    lines.append(r"% Copy-paste into Chapter 5 tables")
    lines.append("")

    # Guard stats table
    lines.append(r"\begin{table}[htbp]")
    lines.append(r"\centering")
    lines.append(r"\caption{Signature-guard decisions per project (RQ1).}")
    lines.append(r"\label{tab:guard-stats}")
    lines.append(r"\begin{tabular}{lrrrr}")
    lines.append(r"\toprule")
    lines.append(r"Project & Total & Accepted & Reverted (sig) & Drift \% \\")
    lines.append(r"\midrule")
    for proj, s in sorted(guard_stats.items()):
        drift = s["reverted_sig"] / s["total"] * 100 if s["total"] > 0 else 0
        lines.append(
            rf"\texttt{{{proj.replace('-', r'\mbox{-}')}}} & "
            rf"{s['total']} & {s['accepted']} & {s['reverted_sig']} & "
            rf"{drift:.1f}\% \\"
        )
    lines.append(r"\bottomrule")
    lines.append(r"\end{tabular}")
    lines.append(r"\end{table}")
    lines.append("")

    # Runtime table per project
    for project in sorted(perf):
        vdata = perf[project]
        all_workloads = sorted({wl for var in vdata.values() for wl in var})
        if not all_workloads:
            continue
        lines.append(r"\begin{table}[htbp]")
        lines.append(r"\centering")
        lines.append(
            rf"\caption{{Runtime comparison for \texttt{{{project}}} (RQ2). "
            r"V-C = C baseline, V-c2rust = raw c2rust, V-Hybrid = selective pipeline.}"
        )
        lines.append(rf"\label{{tab:runtime-{project}}}")
        lines.append(r"\begin{tabular}{lrrrrrrr}")
        lines.append(r"\toprule")
        lines.append(r"Workload & V-C (ms) & V-c2rust (ms) & V-Hybrid (ms) & $\Delta$c2r & $\Delta$hyb & $\Delta$hyb/c2r \\")
        lines.append(r"\midrule")
        for wl in all_workloads:
            def ms(var):
                vals = vdata.get(var, {}).get(wl, [])
                return statistics.median(vals) * 1000 if vals else None
            c, c2r, hyb = ms("c"), ms("c2rust"), ms("hybrid")
            wl_short = os.path.basename(wl) if "/" in wl else wl
            row_parts = [
                rf"\texttt{{{wl_short[:28]}}}",
                f"{c:.1f}" if c else "—",
                f"{c2r:.1f}" if c2r else "—",
                f"{hyb:.1f}" if hyb else "—",
                fmt_delta(c2r, c) if (c2r and c) else "—",
                fmt_delta(hyb, c) if (hyb and c) else "—",
                fmt_delta(hyb, c2r) if (hyb and c2r) else "—",
            ]
            lines.append(" & ".join(row_parts) + r" \\")
        lines.append(r"\bottomrule")
        lines.append(r"\end{tabular}")
        lines.append(r"\end{table}")
        lines.append("")

    out_path.write_text("\n".join(lines))
    print(f"LaTeX tables written to {out_path}")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    if not RESULTS_DIR.is_dir():
        print(f"Results directory not found: {RESULTS_DIR}", file=sys.stderr)
        sys.exit(1)

    repo_root = RESULTS_DIR.parent.parent
    perf = load_perf(RESULTS_DIR)
    mem  = load_mem(RESULTS_DIR)
    guard_stats = load_guard_stats(repo_root)

    print("=" * 70)
    print("TABLE 1  — Signature-guard decisions (RQ1)")
    print("=" * 70)
    print_guard_stats_table(guard_stats)

    print("=" * 70)
    print("TABLE 2  — Runtime (wall-clock) comparison (RQ2)")
    print("=" * 70)
    print_runtime_table(perf)

    print("=" * 70)
    print("TABLE 3  — Peak RSS comparison (RQ3)")
    print("=" * 70)
    print_memory_table(mem)

    if LATEX:
        out = RESULTS_DIR / "tables.tex"
        write_latex_tables(perf, mem, guard_stats, out)


if __name__ == "__main__":
    main()
