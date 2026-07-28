#!/usr/bin/env python3
"""Deterministic per-query instruction counts via callgrind.

Why this exists: no hosted CI runner exposes a PMU. Measured, not assumed —
GCE rejects `--performance-monitoring-unit` on the v1 API and on the beta API
alike, `perf` there reports `<not supported>`, macOS runners return 0 for
`proc_pid_rusage`'s ri_instructions, and kperf inside a macOS runner fails with
`kpep_db_create failed: 7`. Hardware counters are simply unavailable.

Callgrind counts instructions in *software*, so it needs no PMU and no
privileges, and its counts are **deterministic** — the same build and query
give the same number every run. For a regression gate that is strictly better
than sampled counters: there is no run-to-run noise to threshold against.

The cost is a ~50x slowdown, so this measures a curated subset rather than the
whole 317-query set. `bench/run_bench.py` remains the tool for full-set
allocation and wall-clock coverage.

## How the windowing works

Callgrind reports totals at process exit, not per-window, so a naive approach
would need one server per query. Instead the server starts with instrumentation
off, and for each query we turn it on, run the query, turn it off, and dump.
Each dump's `summary:` line is a running total, so consecutive differences give
per-query counts.

    valgrind --tool=callgrind --instr-atstart=no redis-server --loadmodule ...
    callgrind_control --instr=on   <pid>
    <query>
    callgrind_control --instr=off  <pid>
    callgrind_control --dump       <pid>

Usage:
    bench/cachegrind.py --module path/to/libfalkordb.so [--out out.csv]
                        [--iterations 5] [query name ...]
"""

import argparse
import csv
import glob
import os
import shutil
import subprocess
import sys
import time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from queries import QUERIES, SETUP, SETUP_COMMANDS  # noqa: E402

SHLIB_EXT = "dylib" if sys.platform == "darwin" else "so"

# The default subset: one representative per family that has shown movement in
# this work, kept small because each query costs ~50x its normal runtime.
DEFAULT_SUBSET = [
    "RETURN 1",              # fixed per-query floor; the control
    "arithmetic",            # scalar expression evaluation
    "CASE",                  # branchy expression + property access
    "list comprehension",    # scoped iteration
    "reduce",                # accumulator loop
    "create node",           # small write path
    "delete node",           # small delete path
    "two-hop",               # matrix traversal
]


# Names must exist in queries.py. Checked at import rather than after a
# 2-minute build and a container spin-up, which is how `traverse 2 hops` — a
# name I assumed rather than looked up — reached CI.
_UNKNOWN = sorted(set(DEFAULT_SUBSET) - {q[0] for q in QUERIES})
assert not _UNKNOWN, f"DEFAULT_SUBSET names not in queries.py: {_UNKNOWN}"


def parse_summary(path):
    """Total instruction count from a callgrind output file.

    Callgrind writes `summary: <Ir> [...]` (or `totals:`) with the first field
    being instruction reads. Returns None when the file has neither, which
    happens for a dump produced before any instrumented work.
    """
    try:
        with open(path, errors="replace") as f:
            for line in f:
                if line.startswith(("summary:", "totals:")):
                    parts = line.split(":", 1)[1].split()
                    if parts:
                        return int(parts[0])
    except OSError:
        return None
    return None


def latest_total(outdir):
    """Highest summary across all dumps written so far.

    Callgrind numbers dumps `callgrind.out.<pid>.<n>`; lexical order does not
    match numeric order past 9, and a dump may be mid-write, so take the max of
    what parses rather than trusting the newest filename.
    """
    totals = [
        t
        for t in (parse_summary(p) for p in glob.glob(os.path.join(outdir, "callgrind.out.*")))
        if t is not None
    ]
    return max(totals) if totals else None


def cli(port, *args, check=True):
    out = subprocess.run(
        ["redis-cli", "-p", str(port)] + [str(a) for a in args],
        capture_output=True,
        text=True,
    )
    if check and out.returncode != 0:
        raise RuntimeError(
            f"redis-cli {' '.join(str(a) for a in args)} failed "
            f"(exit {out.returncode}): {out.stderr.strip()[:200]}"
        )
    return out.stdout


def control(pid, *flags):
    subprocess.run(["callgrind_control", *flags, str(pid)], capture_output=True, text=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--module", default=os.path.join(ROOT, f"target/release/libfalkordb.{SHLIB_EXT}"))
    ap.add_argument("--port", default="6402")
    ap.add_argument("--out", default=os.path.join(HERE, "results/cachegrind.csv"))
    ap.add_argument(
        "--iterations",
        type=int,
        default=5,
        help="runs per query. Counts are deterministic, so this only averages "
        "away per-connection setup; it does not reduce measurement noise "
        "(there is none).",
    )
    ap.add_argument("names", nargs="*", help="queries to measure (default: a curated subset)")
    args = ap.parse_args()

    for tool in ("valgrind", "callgrind_control", "redis-server", "redis-cli"):
        if not shutil.which(tool):
            sys.exit(
                f"{tool} not found. This tool needs valgrind and redis on PATH. "
                "Note valgrind does not support macOS on Apple silicon, so this "
                "is Linux-only in practice."
            )

    wanted = args.names or DEFAULT_SUBSET
    queries = [q for q in QUERIES if q[0] in set(wanted)]
    missing = set(wanted) - {q[0] for q in queries}
    if missing:
        sys.exit(f"unknown queries: {sorted(missing)}")

    outdir = os.path.join(HERE, "results/callgrind")
    shutil.rmtree(outdir, ignore_errors=True)
    os.makedirs(outdir, exist_ok=True)

    server = subprocess.Popen(
        [
            "valgrind",
            "--tool=callgrind",
            "--instr-atstart=no",  # windowing: see the module docstring
            f"--callgrind-out-file={outdir}/callgrind.out.%p.%n",
            "redis-server",
            "--port", str(args.port),
            "--save", "",
            "--loadmodule", args.module,
        ],
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )

    try:
        # Under valgrind startup is ~50x slower, so allow far longer than the
        # native harness does.
        for _ in range(600):
            if cli(args.port, "ping", check=False).strip() == "PONG":
                break
            time.sleep(0.5)
        else:
            sys.exit("server did not start under callgrind")

        print("server up under callgrind, building graph (slow)...", flush=True)
        for stmt in SETUP:
            cli(args.port, "GRAPH.QUERY", "bench", stmt)
        for cmd in SETUP_COMMANDS:
            cli(args.port, *[a.replace("{graph}", "bench") for a in cmd], check=False)

        pid = int(cli(args.port, "info", "server").split("process_id:")[1].split()[0])

        # Baseline: everything up to here is excluded by --instr-atstart=no, but
        # dump once so the first query differences against a known total.
        control(pid, "--dump")
        time.sleep(1)
        prev = latest_total(outdir) or 0

        rows = []
        for name, _is_write, q, *_rest in queries:
            control(pid, "--instr=on")
            for _ in range(args.iterations):
                cli(args.port, "GRAPH.QUERY", "bench", q, check=False)
            control(pid, "--instr=off")
            control(pid, "--dump")

            # The dump is asynchronous; wait for the total to move.
            total = prev
            for _ in range(120):
                time.sleep(0.5)
                total = latest_total(outdir) or prev
                if total > prev:
                    break
            else:
                print(f"  {name}: no dump appeared, skipping", flush=True)
                continue

            per = (total - prev) / args.iterations
            prev = total
            rows.append({"query": name, "instr": f"{per:.0f}"})
            print(f"{name:<24}{per:>15,.0f} instr (deterministic)", flush=True)
    finally:
        server.terminate()
        try:
            server.wait(timeout=120)
        except subprocess.TimeoutExpired:
            server.kill()

    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    with open(args.out, "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=["query", "instr"])
        w.writeheader()
        w.writerows(rows)
    print(f"wrote {args.out} ({len(rows)} rows)", flush=True)


if __name__ == "__main__":
    main()
