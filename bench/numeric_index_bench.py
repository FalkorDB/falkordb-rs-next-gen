#!/usr/bin/env python3
"""Simple numeric-index benchmark: native banded index vs RediSearch.

Builds a graph of N nodes (`:N {val}`) plus ~`edges_per_node` edges
(`:R {w}`), creates RANGE indexes on the numeric props, and times the
index build (backfill) and representative scans. Measures the server-side
`run_time_ms` reported by GRAPH.QUERY (not client/network time) and uses
`RETURN count(...)` so we time the index scan, not result transfer.

Run it against a redis-server with the falkordb module loaded — once per
`.so` build (native feature-on vs feature-off) — and compare the tables.

    python3 bench/numeric_index_bench.py --port 6379 --nodes 1000000 \
        --edges-per-node 3

Build the dataset+indexes once, then only re-run the scans with --no-build
(useful when re-pointing at an already-populated graph).
"""

import argparse
import json
import random
import statistics
import sys
import time

import redis
from falkordb import FalkorDB


def capture_memory(conn, graph_name):
    """Process RSS (OS resident memory) + redis used_memory + the graph's own
    GRAPH.MEMORY accounting. RSS is the true footprint; GRAPH.MEMORY is the
    graph's accounted bytes (which may under-count the native index matrices)."""
    out = {}
    info = conn.info("memory")
    out["rss_mb"] = round(info.get("used_memory_rss", 0) / 1048576, 1)
    out["used_memory_mb"] = round(info.get("used_memory", 0) / 1048576, 1)
    try:
        raw = conn.execute_command("GRAPH.MEMORY", "USAGE", graph_name)
        if isinstance(raw, (list, tuple)) and len(raw) % 2 == 0:
            out["graph_memory"] = {
                (raw[i].decode() if isinstance(raw[i], bytes) else raw[i]): raw[i + 1]
                for i in range(0, len(raw), 2)
            }
        else:
            out["graph_memory"] = raw
    except Exception as e:  # noqa: BLE001 - record, don't abort the bench
        out["graph_memory_error"] = str(e)
    return out


def q(graph, cypher, params=None):
    """Run a query, return (rows, server_run_time_ms)."""
    res = graph.query(cypher, params or {})
    return res.result_set, float(res.run_time_ms)


def timed_ms(fn, iters):
    """Median/min/max of `iters` calls to fn() returning server ms."""
    samples = [fn() for _ in range(iters)]
    return {
        "median_ms": round(statistics.median(samples), 3),
        "min_ms": round(min(samples), 3),
        "max_ms": round(max(samples), 3),
    }


def build_nodes(graph, n, vmax, batch):
    t0 = time.perf_counter()
    for s in range(0, n, batch):
        e = min(s + batch, n)
        graph.query(
            # Deterministic vals (i % vmax) so native and RediSearch index the
            # *same* data — makes the count(...) results comparable.
            "UNWIND range($s, $e - 1) AS i CREATE (:N {val: i % $vmax})",
            {"s": s, "e": e, "vmax": vmax},
        )
    return time.perf_counter() - t0


def build_edges(graph, n, per_node, wmax, batch):
    # Endpoints by internal id (sequential, no deletes here), so the MATCH is an
    # id-seek rather than a property scan.
    total = n * per_node
    t0 = time.perf_counter()
    rows = []
    for _ in range(total):
        rows.append([random.randrange(n), random.randrange(n), random.randrange(wmax)])
        if len(rows) >= batch:
            _flush_edges(graph, rows)
            rows = []
    if rows:
        _flush_edges(graph, rows)
    return time.perf_counter() - t0


def _flush_edges(graph, rows):
    graph.query(
        "UNWIND $rows AS r "
        "MATCH (a:N) WHERE id(a) = r[0] "
        "MATCH (b:N) WHERE id(b) = r[1] "
        "CREATE (a)-[:R {w: r[2]}]->(b)",
        {"rows": rows},
    )


def create_indexes(graph):
    """Create RANGE indexes on a populated graph (times the backfill)."""
    out = {}
    t0 = time.perf_counter()
    graph.query("CREATE INDEX FOR (n:N) ON (n.val)")
    out["node_index_ms"] = round((time.perf_counter() - t0) * 1000, 1)
    t0 = time.perf_counter()
    graph.query("CREATE INDEX FOR ()-[e:R]->() ON (e.w)")
    out["edge_index_ms"] = round((time.perf_counter() - t0) * 1000, 1)
    return out


def bench_one(graph, cypher, params, iters):
    """Time a count(...) query and also record the count (for a native-vs-RS
    correctness check)."""
    rows, _ = q(graph, cypher, params)
    count = rows[0][0] if rows and rows[0] else None
    stats = timed_ms(lambda: q(graph, cypher, params)[1], iters)
    stats["count"] = count
    return stats


def bench_scans(graph, vmax, iters):
    out = {}

    # Node equality (point lookup).
    out["node_eq"] = bench_one(
        graph, "MATCH (n:N) WHERE n.val = $v RETURN count(n)", {"v": vmax // 2}, iters
    )

    # Node range scans at increasing selectivity (fraction of vmax).
    for frac in (0.01, 0.1, 0.5, 1.0):
        hi = int(vmax * frac)
        out[f"node_range_{frac:g}"] = bench_one(
            graph,
            "MATCH (n:N) WHERE n.val >= 0 AND n.val <= $hi RETURN count(n)",
            {"hi": hi},
            iters,
        )

    # Edge range scan (10% selectivity).
    out["edge_range_0.1"] = bench_one(
        graph,
        "MATCH ()-[e:R]->() WHERE e.w >= 0 AND e.w <= $hi RETURN count(e)",
        {"hi": int(vmax * 0.1)},
        iters,
    )
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=6379)
    ap.add_argument("--graph", default="bench")
    ap.add_argument("--nodes", type=int, default=1_000_000)
    ap.add_argument("--edges-per-node", type=int, default=3)
    ap.add_argument("--vmax", type=int, default=1_000_000)
    ap.add_argument("--batch", type=int, default=10_000)
    ap.add_argument("--iters", type=int, default=15)
    ap.add_argument("--no-index", action="store_true", help="skip index creation (raw-ingest baseline)")
    ap.add_argument("--label", default="", help="tag printed in the result JSON")
    ap.add_argument("--seed", type=int, default=42, help="RNG seed for edge generation")
    args = ap.parse_args()

    random.seed(args.seed)  # identical edge set across native/RediSearch runs
    db = FalkorDB(host=args.host, port=args.port)
    graph = db.select_graph(args.graph)

    result = {
        "label": args.label,
        "nodes": args.nodes,
        "edges_per_node": args.edges_per_node,
        "indexed": not args.no_index,
    }

    # Index-first methodology: create the index on the EMPTY graph (instant — no
    # backfill), then ingest. The ingest time then reflects *incremental* index
    # maintenance, identically for native and RediSearch (both index per-commit)
    # — no async-backfill confound. With --no-index it's the raw-ingest baseline,
    # so indexing overhead = ingest_with_index - ingest_no_index.
    if not args.no_index:
        result["index_create_ms"] = create_indexes(graph)  # ~0 on an empty graph
    result["ingest_nodes_s"] = round(build_nodes(graph, args.nodes, args.vmax, args.batch), 1)
    result["ingest_edges_s"] = round(
        build_edges(graph, args.nodes, args.edges_per_node, args.vmax, args.batch), 1
    )

    # Steady-state footprint after the dataset + index are built.
    conn = redis.Redis(host=args.host, port=args.port, decode_responses=True)
    result["memory"] = capture_memory(conn, args.graph)

    result["scans"] = bench_scans(graph, args.vmax, args.iters)

    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    sys.exit(main())
