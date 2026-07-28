#!/usr/bin/env python3
"""Benchmark harness: per-query instructions/cycles (+ optional PMU counters).

Starts a redis-server with the module, builds the benchmark graph, then for
each query measures redis-server-process instructions & cycles (macOS
proc_pid_rusage, no root needed) around `redis-benchmark -c 1 -n N`.
If ./pmc_tool exists and works (setuid root or run under sudo), also records
system-wide branches / branch-misses / L1D-misses, idle-adjusted.
If the server is jemalloc-built (stock redis is), also records per-query
allocated/deallocated bytes from `MEMORY MALLOC-STATS` merged-arena deltas.

Usage:
  python3 bench/run_bench.py [options] [query name ...]
    --module PATH   module to load (default target/release/libfalkordb.dylib)
    --port P        (default 6399)
    --out FILE      CSV output (default bench/results/current.csv); named
                    queries are merged into an existing CSV, so subset re-runs
                    patch only those rows
    --n N           requests per query (default 1000)
    --once          run each query exactly once via redis-cli, no measurement
                    (used by coverage.sh; server env gets LLVM_PROFILE_FILE
                    passthrough automatically)
    --keep-server   leave the server running after the run (for profiling)
    --reuse         assume a server is already on --port with the graph built
    --c-compat      benchmarking the C FalkorDB module: skip setup commands
                    known to crash it (composite unique constraint) and drop
                    queries whose warmup reply is an error instead of writing
                    artifact rows
"""
import argparse, csv, ctypes, os, shutil, signal, subprocess, sys, time

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from queries import QUERIES, SETUP, SETUP_COMMANDS, ERROR_QUERIES, IMPORT_DIR, CSV_FILES

libproc = ctypes.CDLL("/usr/lib/libproc.dylib")
RUSAGE_INFO_V4 = 4


def rusage(pid):
    buf = ctypes.create_string_buffer(1024)
    if libproc.proc_pid_rusage(ctypes.c_int(pid), ctypes.c_int(RUSAGE_INFO_V4), buf) != 0:
        raise OSError("proc_pid_rusage failed")
    u64 = (ctypes.c_uint64 * 40).from_buffer_copy(buf.raw[16:336])
    return u64[29], u64[30]  # ri_instructions, ri_cycles


def pmc_run(pmc, cmd):
    out = subprocess.run([pmc, "runcmd"] + cmd, capture_output=True, text=True)
    if "EVENT" not in out.stdout:
        return None, None
    ev, elapsed = {}, 0.0
    for line in out.stdout.splitlines():
        p = line.split()
        if not p:
            continue
        if p[0] == "ELAPSED":
            elapsed = float(p[1])
        elif p[0] == "EVENT":
            ev[p[1]] = int(p[2])
    return ev, elapsed


def cli(port, *args):
    return subprocess.run(["redis-cli", "-p", str(port)] + list(args),
                          capture_output=True, text=True).stdout


def jemalloc_totals(port):
    """Cumulative allocated/deallocated bytes from jemalloc's merged-arenas
    stats: sum size*nmalloc / size*ndalloc over the bins: and large: size-class
    tables. Returns (None, None) if the server isn't jemalloc-built."""
    out = cli(port, "memory", "malloc-stats")
    if "Merged arenas stats:" not in out:
        return None, None
    alloc = dealloc = 0
    in_merged = in_table = False
    for line in out.splitlines():
        if line.startswith("Merged arenas stats:"):
            in_merged = True
        elif line.startswith("arenas["):
            break
        elif in_merged:
            s = line.split()
            if not s:
                continue
            if s[0] in ("bins:", "large:") and "size" in s:
                in_table = True
            elif s[0] == "extents:":
                in_table = False
            elif in_table and s[0].isdigit():
                size = int(s[0])
                alloc += size * int(s[3])
                dealloc += size * int(s[5])
    return alloc, dealloc


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--module", default=os.path.join(ROOT, "target/release/libfalkordb.dylib"))
    ap.add_argument("--port", default="6399")
    ap.add_argument("--out", default=os.path.join(HERE, "results/current.csv"))
    ap.add_argument("--n", type=int, default=1000)
    ap.add_argument("--once", action="store_true")
    ap.add_argument("--keep-server", action="store_true")
    ap.add_argument("--reuse", action="store_true")
    ap.add_argument("--c-compat", action="store_true")
    ap.add_argument("names", nargs="*")
    args = ap.parse_args()

    queries = QUERIES
    if args.names:
        only = set(args.names)
        queries = [q for q in QUERIES if q[0] in only]
        missing = only - {q[0] for q in queries}
        if missing:
            sys.exit(f"unknown queries: {missing}")

    server = None
    if not args.reuse:
        if cli(args.port, "ping").strip() == "PONG":
            sys.exit(f"port {args.port} already in use; use --reuse or another --port")
        os.makedirs(IMPORT_DIR, exist_ok=True)
        for fname, content in CSV_FILES.items():
            with open(os.path.join(IMPORT_DIR, fname), "w") as f:
                f.write(content)
        # Isolated, clean data dir: DEBUG RELOAD in setup writes a dump.rdb
        # into the server dir, and a later server would reload it (making
        # setup fail with "already indexed").
        work_dir = os.path.join(HERE, "results", "server_dir")
        shutil.rmtree(work_dir, ignore_errors=True)
        os.makedirs(work_dir)
        server_args = ["redis-server", "--port", str(args.port), "--save", "",
                       "--enable-debug-command", "local", "--dir", work_dir,
                       # trailing slash: the C module concatenates IMPORT_FOLDER
                       # + filename directly, without inserting a separator
                       "--loadmodule", args.module, "IMPORT_FOLDER", IMPORT_DIR + "/"]
        if args.once:
            # AOF makes the module emit replication effects on every write
            # (Pending::build_effects_buffer), covering those paths.
            server_args += ["--appendonly", "yes", "--appendfsync", "no"]
        server = subprocess.Popen(
            server_args,
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        for _ in range(100):
            if cli(args.port, "ping").strip() == "PONG":
                break
            time.sleep(0.1)
        else:
            sys.exit("server did not start")
        print(f"server up on :{args.port}, building graph...", flush=True)
        for stmt in SETUP:
            out = cli(args.port, "GRAPH.QUERY", "bench", stmt)
            # Successful GRAPH.QUERY replies always end with the execution-time
            # stat; anything else (error message, empty reply) is a failure.
            if "execution time" not in out:
                sys.exit(f"setup failed: {out[:200]}")
        for cmd in SETUP_COMMANDS:
            # The C server's async validation of a composite (2-property)
            # unique constraint crashes it; everything else in setup works.
            if args.c_compat and cmd[:2] == ["GRAPH.CONSTRAINT", "CREATE"] and "2" in cmd:
                continue
            # The C server's RDB round-trip drops numeric 0 from the Person
            # range index: after DEBUG RELOAD, MATCH (:Person {id: 0}) finds
            # nothing, turning every id-0 row into a no-op (e.g. algo.BFS).
            if args.c_compat and cmd == ["DEBUG", "RELOAD"]:
                continue
            out = cli(args.port, *[a.replace("{graph}", "bench") for a in cmd])
            if "error" in out.lower() or out.startswith("ERR"):
                sys.exit(f"setup command failed: {cmd[0]} {cmd[1]}: {out[:200]}")

    try:
        if args.once:
            fails = 0
            for name, is_write, q, *_ in queries:
                cmd = "GRAPH.QUERY" if is_write else "GRAPH.RO_QUERY"
                out = cli(args.port, cmd, "bench", q)
                if "execution time" not in out:
                    print(f"FAIL {name}: {out.strip().splitlines()[0][:120] if out.strip() else '(empty reply)'}")
                    fails += 1
            # Expected-error queries: cover parse/bind/eval error paths and
            # constraint rollback. Pass iff the reply is a non-empty error.
            if not args.names:
                for name, cmd, q in ERROR_QUERIES:
                    out = cli(args.port, cmd, "bench", q)
                    if not out.strip() or "execution time" in out:
                        print(f"FAIL (expected error) {name}: {out.strip().splitlines()[0][:120] if out.strip() else '(empty reply)'}")
                        fails += 1
            print(f"once-mode done, {fails} failures")
            if server and not args.keep_server:
                cli(args.port, "shutdown", "nosave")  # graceful: flushes .profraw
                server.wait()
                server = None
            if fails:
                sys.exit(1)
            return

        pid = int(cli(args.port, "info", "server").split("process_id:")[1].split()[0])
        pmc = os.path.join(HERE, "pmc_tool")
        pmc_ok = os.path.exists(pmc) and pmc_run(pmc, ["true"])[0] is not None
        if not pmc_ok:
            print("pmc_tool unavailable — branches/L1D columns will be empty "
                  "(see bench/README.md to build+setuid)", flush=True)

        # idle baseline
        i0, c0 = rusage(pid)
        if pmc_ok:
            idle_ev, idle_dt = pmc_run(pmc, ["sleep", "3"])
        else:
            time.sleep(3)
            idle_ev, idle_dt = {}, 3.0
        i1, c1 = rusage(pid)
        idle_ips, idle_cps = (i1 - i0) / idle_dt, (c1 - c0) / idle_dt
        idle_rate = {k: v / idle_dt for k, v in idle_ev.items()}
        print(f"pid {pid}, idle process rate {idle_ips/1e6:.1f}M instr/s", flush=True)

        rows = []
        for name, is_write, q, *rest in queries:
            n = rest[0] if rest else args.n
            cmd = ["GRAPH.QUERY" if is_write else "GRAPH.RO_QUERY", "bench", q]
            warm = cli(args.port, *cmd)  # warmup / plan cache
            if args.c_compat and "execution time" not in warm:
                first = warm.strip().splitlines()[0][:100] if warm.strip() else "(empty reply)"
                print(f"{name:<20} SKIPPED (C error: {first})", flush=True)
                continue
            bench = ["redis-benchmark", "-p", str(args.port), "-c", "1",
                     "-n", str(n)] + cmd
            # memory snapshots outside the i0..i1 window so the MALLOC-STATS
            # call's own work doesn't pollute the instruction counts
            m0a, m0d = jemalloc_totals(args.port)
            i0, c0 = rusage(pid)
            if pmc_ok:
                ev, dt = pmc_run(pmc, bench)
            else:
                t0 = time.time()
                subprocess.run(bench, capture_output=True)
                ev, dt = {}, time.time() - t0
            i1, c1 = rusage(pid)
            m1a, m1d = jemalloc_totals(args.port)
            row = {
                "query": name,
                "instr": (i1 - i0 - idle_ips * dt) / n,
                "cycles": (c1 - c0 - idle_cps * dt) / n,
                "branches": "", "br_miss": "", "l1d_miss": "",
                "alloc_bytes": (m1a - m0a) / n if m0a is not None else "",
                "dealloc_bytes": (m1d - m0d) / n if m0d is not None else "",
                "ms": dt / n * 1000,
            }
            if pmc_ok:
                adj = {k: (v - idle_rate[k] * dt) / n for k, v in ev.items()}
                row["branches"] = adj["INST_BRANCH"]
                row["br_miss"] = adj["BRANCH_MISPRED_NONSPEC"]
                row["l1d_miss"] = adj["L1D_CACHE_MISS_LD"] + adj["L1D_CACHE_MISS_ST"]
            rows.append(row)
            print(f"{name:<20} {row['instr']:>13,.0f} instr {row['cycles']:>12,.0f} cyc "
                  f"{row['ms']:>8.3f} ms"
                  + (f" {row['alloc_bytes']:>12,.0f} B alloc" if row['alloc_bytes'] != "" else ""),
                  flush=True)

        os.makedirs(os.path.dirname(args.out), exist_ok=True)
        merged = {}
        if os.path.exists(args.out):
            merged = {r["query"]: r for r in csv.DictReader(open(args.out))}
        for r in rows:
            merged[r["query"]] = {k: str(v) for k, v in r.items()}
        fields = ["query", "instr", "cycles", "branches", "br_miss", "l1d_miss",
                  "alloc_bytes", "dealloc_bytes", "ms"]
        with open(args.out, "w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=fields)
            w.writeheader()
            w.writerows({k: r.get(k, "") for k in fields} for r in merged.values())
        print(f"wrote {args.out}")
    finally:
        if server and not args.keep_server:
            server.send_signal(signal.SIGTERM)
            server.wait()
        elif server:
            print(f"server left running on :{args.port} (pid {server.pid})")


if __name__ == "__main__":
    main()
