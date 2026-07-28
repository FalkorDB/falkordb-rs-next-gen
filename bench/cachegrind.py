#!/usr/bin/env python3
"""Deterministic per-query instruction counts via callgrind.

Why this exists: no hosted CI runner exposes a PMU. Measured, not assumed —
GCE rejects `--performance-monitoring-unit` on the v1 API and on the beta API
alike, `perf` there reports `<not supported>`, macOS runners return 0 for
`proc_pid_rusage`'s ri_instructions, and kperf inside a macOS runner fails with
`kpep_db_create failed: 7`. Hardware counters are simply unavailable.

Callgrind counts instructions in *software*, so it needs no PMU and no
privileges, and its counts are near-deterministic: far steadier than sampled
counters, though not bit-exact (see "Measured precision").

The cost is a large slowdown, so this measures a curated subset on a smaller
graph than `bench/run_bench.py`. See "Not comparable to run_bench.py" below.

## How a query is isolated: differencing, not windowing

Callgrind reports one total when the process exits. The obvious approach is to
window with `callgrind_control --instr=on/off --dump` around each query, and
that is what the first version of this script did. **It does not work in a
container.** `callgrind_control` reaches the process through vgdb FIFOs in
/tmp, and reproduced locally in `debian:trixie-slim` (valgrind 3.24.0, the CI
version):

    ==236== open fifo /tmp/vgdb-pipe-from-vgdb-to-236-by-???-on-???
    ==236== valgrind: fatal error: vgdb FIFO cannot be opened.

The server dies on the first control command, so every dump silently never
arrives and every query reports nothing. Setting USER/LOGNAME does not help;
`--vgdb-prefix` makes `callgrind_control` hang instead. In CI this produced
two empty CSVs after 16 minutes.

So instead each query is measured by **differencing two complete runs** of the
same query at different repeat counts:

    T(n2) = startup + setup + compile + n2 * exec
    T(n1) = startup + setup + compile + n1 * exec
    exec  = (T(n2) - T(n1)) / (n2 - n1)

Startup, graph setup and one-time plan compilation appear identically in both
runs and cancel *exactly* — not approximately — because the counts are
deterministic. Nothing but the query's steady-state execution survives the
subtraction. No vgdb, no dumps, no timing races, and each run reads a single
number from a single file at exit.

The price is two valgrind runs per query, each paying setup, which is why
`CG_SETUP` below is deliberately small.

## Measured precision

Measured on arm64 against a bare redis-server (see GRAPH_CMD for why the module
is not used there), with `--hz 1` and one connection per run:

  - three identical runs: totals within 3,450 instr, 0.015% of the total
  - two independent (n1, n2) pairs on the same command agreed to 0.45%

The residual comes from work whose amount depends on how long the process
lives, so it appears as a roughly fixed absolute drift per run rather than a
per-execution error: ~3.5k instr spread over a span of `n2 - n1` executions is
~18-35 instr/exec. That is 0.5% of a 4.9k-instruction PING and under 0.01% of a
query costing 500k, so the useful precision is much better for real queries
than the PING figure suggests. Treat sub-1% differences as noise.

An earlier version of this file claimed the counts were exactly reproducible
with "no run-to-run noise to threshold against". That was wrong, and the
validation above is what caught it: at the default hz=10 and with a fresh
connection per execution, two (n1, n2) pairs disagreed by 44%.

## Not comparable to run_bench.py

`CG_SETUP` builds a 1,000-node graph, not the 10,000-node one in queries.py,
and skips the vector/fulltext indexes, constraints, UDFs and DEBUG RELOAD that
the full harness sets up. Absolute numbers here are therefore *not* comparable
to run_bench.py rows. They are only meaningful as a main-vs-PR ratio, where
both sides run this identical setup.

Usage:
    bench/cachegrind.py --module path/to/libfalkordb.so [--out out.csv]
                        [--n1 3] [--n2 13] [query name ...]
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
from queries import QUERIES  # noqa: E402

SHLIB_EXT = "dylib" if sys.platform == "darwin" else "so"

# A small graph that supports the subset below. Deliberately not queries.py's
# SETUP: that builds 10k nodes, 10k edges, vector and fulltext indexes,
# constraints, UDFs and a DEBUG RELOAD, and every one of those instructions
# would be paid twice per measured query under instrumentation.
#
# The Person index is created before the ring so the ring build is index-driven
# rather than a 1000x1000 nested scan.
CG_SETUP = [
    "UNWIND range(0, 999) AS i "
    "CREATE (:Person {id: i, name: 'p' + toString(i), age: i % 80, score: i * 1.5})",
    "CREATE INDEX FOR (p:Person) ON (p.id)",
    "UNWIND range(0, 999) AS i "
    "MATCH (a:Person {id: i}) MATCH (b:Person {id: (i + 1) % 1000}) "
    "CREATE (a)-[:KNOWS]->(b)",
    # `delete node` deletes one :Tmp per execution, so there must be more of
    # them than the highest repeat count.
    "UNWIND range(0, 999) AS i CREATE (:Tmp {x: i})",
]

# The command each statement/query is sent as. A seam: valgrind on arm64 cannot
# run this module at all (`unhandled instruction 0xB8BFC108` — an ARMv8.1 LSE
# atomic in RediSearch's slots_tracker, valgrind's limitation rather than a
# module bug), so the differencing mechanism is validated on arm64 against a
# plain redis-server by patching this to () and CG_SETUP to [].
GRAPH_CMD = ("GRAPH.QUERY", "bench")

# One representative per family that has shown movement in this work.
#
# Deliberately short: each query costs two instrumented server lifecycles, and
# every lifecycle pays CG_SETUP again. Instrumented setup cost has not been
# measured yet (valgrind cannot run the module on arm64, so this could not be
# timed locally), so the subset starts small and the script prints per-query
# wall time — widen it once CI shows what a run actually costs. Extra queries
# can be passed as positional arguments without editing this list.
DEFAULT_SUBSET = [
    "RETURN 1",              # fixed per-query floor; the control
    "arithmetic",            # scalar expression evaluation
    "CASE",                  # branchy expression + property access
    "reduce",                # accumulator loop
    "two-hop",               # matrix traversal
]


# Names must exist in queries.py. Checked at import rather than after a
# 2-minute build and a container spin-up, which is how `traverse 2 hops` — a
# name I assumed rather than looked up — reached CI.
_UNKNOWN = sorted(set(DEFAULT_SUBSET) - {q[0] for q in QUERIES})
assert not _UNKNOWN, f"DEFAULT_SUBSET names not in queries.py: {_UNKNOWN}"


def parse_total(path):
    """Instruction count from a callgrind output file.

    Callgrind writes `totals:` (and `summary:`) with the first field being
    instruction reads. Returns None when neither is present, which happens for
    a file still being written.
    """
    try:
        with open(path, errors="replace") as f:
            for line in f:
                if line.startswith(("totals:", "summary:")):
                    parts = line.split(":", 1)[1].split()
                    if parts:
                        return int(parts[0])
    except OSError:
        return None
    return None


def run_total(module, port, outdir, query, reps):
    """Run one instrumented server lifecycle; return its total instruction count.

    The server runs `query` `reps` times after building CG_SETUP, then exits so
    callgrind writes its total.
    """
    shutil.rmtree(outdir, ignore_errors=True)
    os.makedirs(outdir, exist_ok=True)

    cmd = [
        "valgrind",
        "--tool=callgrind",
        f"--callgrind-out-file={outdir}/callgrind.out.%p",
        "redis-server",
        "--port", str(port),
        "--save", "",
        # serverCron does work proportional to how long the process lives, and
        # under instrumentation the two runs being differenced live for
        # different durations — so cron lands in the subtraction as drift.
        # Measured at the default hz=10 it is ~240k instr per second of life,
        # which swamped a PING (~20k) and made two (n1,n2) pairs disagree by
        # 44%. hz=1 is the lowest redis accepts and cuts it 10x.
        "--hz", "1",
    ]
    # module=None runs a bare redis-server. Used only by the arm64 validation
    # described at GRAPH_CMD, where the module cannot run under valgrind.
    if module:
        cmd += ["--loadmodule", module]

    server = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    try:
        for _ in range(1200):  # instrumented startup is far slower than native
            if server.poll() is not None:
                raise RuntimeError("server exited during startup under callgrind")
            if cli(port, "ping", check=False).strip() == "PONG":
                break
            time.sleep(0.5)
        else:
            raise RuntimeError("server did not answer PING under callgrind")

        for stmt in CG_SETUP:
            cli(port, *GRAPH_CMD, stmt)

        # One redis-cli with -r, not `reps` of them: a fresh connection per
        # execution would put accept/handshake/teardown into the measurement,
        # and the extra wall time feeds the cron drift described above.
        if reps:
            cli(port, "-r", reps, *GRAPH_CMD, query)

        cli(port, "shutdown", "nosave", check=False)
        server.wait(timeout=600)
    finally:
        if server.poll() is None:
            server.terminate()
            try:
                server.wait(timeout=120)
            except subprocess.TimeoutExpired:
                server.kill()

    # Redis forks, and valgrind profiles the child too; the child's total is
    # tiny, so the server's own run is the maximum.
    totals = [
        t
        for t in (parse_total(p) for p in glob.glob(os.path.join(outdir, "callgrind.out.*")))
        if t is not None
    ]
    if not totals:
        raise RuntimeError(f"no parseable callgrind output in {outdir}")
    return max(totals)


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


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--module", default=os.path.join(ROOT, f"target/release/libfalkordb.{SHLIB_EXT}"))
    ap.add_argument("--port", default="6402")
    ap.add_argument("--out", default=os.path.join(HERE, "results/cachegrind.csv"))
    ap.add_argument("--n1", type=int, default=20, help="low repeat count")
    ap.add_argument(
        "--n2",
        type=int,
        default=120,
        help="high repeat count. exec cost = (T(n2) - T(n1)) / (n2 - n1). The "
        "span divides the per-run drift, so a wider span is more precise: at "
        "span 100 the ~3.5k drift is ~35 instr/exec.",
    )
    ap.add_argument("names", nargs="*", help="queries to measure (default: a curated subset)")
    args = ap.parse_args()

    if args.n2 <= args.n1:
        sys.exit(f"--n2 ({args.n2}) must exceed --n1 ({args.n1})")

    for tool in ("valgrind", "redis-server", "redis-cli"):
        if not shutil.which(tool):
            sys.exit(
                f"{tool} not found. This tool needs valgrind and redis on PATH. "
                "Note valgrind does not support macOS on Apple silicon, so this "
                "is Linux-only in practice."
            )
    if not os.path.exists(args.module):
        sys.exit(f"module not found: {args.module}")

    wanted = args.names or DEFAULT_SUBSET
    queries = [q for q in QUERIES if q[0] in set(wanted)]
    missing = set(wanted) - {q[0] for q in queries}
    if missing:
        sys.exit(f"unknown queries: {sorted(missing)}")

    outdir = os.path.join(HERE, "results/callgrind")
    span = args.n2 - args.n1
    rows = []

    for name, _is_write, q, *_rest in queries:
        t0 = time.time()
        try:
            lo = run_total(args.module, args.port, outdir, q, args.n1)
            hi = run_total(args.module, args.port, outdir, q, args.n2)
        except (RuntimeError, subprocess.TimeoutExpired) as e:
            print(f"{name:<24} FAILED: {e}", flush=True)
            continue

        if hi <= lo:
            # Deterministic counts cannot go down when work is added; if this
            # trips, the two runs did not do what the model above assumes.
            print(
                f"{name:<24} SKIPPED: T(n2)={hi:,} <= T(n1)={lo:,} — "
                "the runs are not differing only by repeat count",
                flush=True,
            )
            continue

        per = (hi - lo) / span
        rows.append({"query": name, "instr": f"{per:.0f}"})
        print(
            f"{name:<24}{per:>15,.0f} instr/exec   "
            f"(T{args.n1}={lo:,} T{args.n2}={hi:,}, {time.time() - t0:.0f}s)",
            flush=True,
        )

    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    with open(args.out, "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=["query", "instr"])
        w.writeheader()
        w.writerows(rows)
    print(f"wrote {args.out} ({len(rows)} rows)", flush=True)
    if not rows:
        sys.exit("no query produced a count")


if __name__ == "__main__":
    main()
