# Bench loop

Repeatable per-query performance + regression + coverage loop. All scripts
live in `bench/` (see `bench/README.md` for findings and details).

## Full loop (run in this order)

```bash
# 1. build the release module
cargo build --release

# 2. measure all 317 queries -> bench/results/current.csv (~10 min)
python3 bench/run_bench.py

# 3. regression gate vs baseline — every metric (cycles/instr/branches/alloc_bytes/
#    dealloc_bytes at +10%, br_miss/l1d_miss/ms at +25%); exit 1 on any breach.
#    --threshold X overrides all metrics, --metrics cycles restricts the gate.
python3 bench/compare.py

# 4. optional: compare against the legacy C engine baseline
python3 bench/compare.py bench/results/current.csv bench/baseline/c.csv

# 5. coverage check: query set should stay ~70% of graph-crate lines
bash bench/coverage.sh
```

After a confirmed improvement, promote the new numbers:

```bash
cp bench/results/current.csv bench/baseline/rust.csv   # local only
```

## Drilling into one query

```bash
# re-measure a subset (rows merge into existing CSV by query name)
python3 bench/run_bench.py "CASE" "WITH pipeline"

# keep the server + graph up, then sample-profile a query
python3 bench/run_bench.py --keep-server "RETURN 1"
bash bench/profile.sh case "MATCH (p:Person) RETURN sum(CASE WHEN p.id % 3 = 0 THEN 1 ELSE 3 END)"
```

`profile.sh` args: `<out-name> "<cypher>" [GRAPH.QUERY|GRAPH.RO_QUERY] [port]`.
Output lands in `bench/results/sample_<name>.txt`; the "Sort by top of stack"
section lists the hot leaves.

## Operational notes

- `run_bench.py` starts its own redis-server on :6399 and builds the graph
  (10k Person ring + KNOWS edges, index on id). If a server is already up
  with the graph, pass `--reuse --port <p>`. It refuses to start if the port
  is busy.
- Instructions/cycles come from `proc_pid_rusage` (per-process, no root) —
  these are the regression-gate columns. Branch/L1D columns need
  `bench/pmc_tool` (setuid root); without it they're left empty and that's
  fine. Rebuild it with:
  ```bash
  clang -O2 -o bench/pmc_tool bench/pmc_tool.c \
    -F /System/Library/PrivateFrameworks -framework kperf -framework kperfdata
  sudo chown root:wheel bench/pmc_tool && sudo chmod u+s bench/pmc_tool
  ```
- PMU numbers are system-wide and include the redis-benchmark client; use
  the RETURN 1 row as the client floor, treat <1K/query as noise.
- `coverage.sh` uses port 6401 and an instrumented debug build; it exits
  non-zero if any query errors, so it doubles as query validation.
- Queries and graph setup are canonical in `bench/queries.py`; add new
  queries there and they flow to benchmark, compare, and coverage. The
  sized "write N" queries must stay LAST in the list — they inflate node
  capacity / matrix dimension to max(N) and would slow every full-graph
  query measured after them (algo.pageRank went 150x when they ran first).
- Run-to-run noise is ~1-2% on cycles but micro-queries (<200k cycles) can
  flag ±15% on cycles; the instruction ratio is the stable signal — trust
  it over a cycles-only flag. Adjust the gate with `--threshold`.

## Current improvement targets

Baselines are local-only (`bench/baseline/` is git-ignored) — a committed one
goes stale on the next merge and then reports phantom regressions. Build your
own from a `main` checkout before comparing, and re-promote it after anything
lands. CI does not use one: `benchmark-cov.yml` measures main, the PR and the C
engine in the same run.

**Read this before ranking anything.** The baseline is `main`, which does
*not* yet include #767 (adaptive fold policy) or #768 (small-delete fast
paths). Those two own the whole top of the Rust/C table — delete pending
edge 23.5x, delete returning 22.3x, delete path 19.2x, DETACH DELETE 16.5x,
write 1 5.6x, create 10k 3.0x. Do not start work on a delete or small-write
row without checking those PRs first; re-measure after they land.

Genuinely open targets, delete/write rows excluded:

| row | Rust/C cyc |
|---|---|
| UDF traverse | 2.78x |
| reversed chain mid filter | 2.48x |
| split+trim+replace | 2.46x |
| order by mixed types | 2.33x |
| var-length 1..50 | 2.29x |
| label predicate | 2.22x |
| correlated hash join | 2.17x |
| percentileDisc / percentileCont | 1.95x / 1.93x |
| reversed 2hop chain | 1.82x |

The two reversed-chain rows are the chain-reversal ordering bug in
`planner/optimizer/select_scan_node.rs` — a reversed plan runs the inner
CondTraverse with neither endpoint bound. #777 did *not* fix it (verified:
1.00x instr before/after); it is independent.

**Already past C** and worth keeping as regression canaries: `arithmetic`
0.73x and `CASE` 0.69x (via #781 + #782), stDev/stDevP ~0.66x/0.69x.
`cross product filter` and `untyped shortestPath` are extreme — C burns
1.8B and 3.8B instructions there against ~400K in Rust — so a regression in
those rows would be invisible as a ratio but very visible in absolute terms.

### Baseline caveats

- Rows where the C column is ~500-2500 instr (regex, week/ordinal dates,
  LOAD CSV*, toJSON scalars) are rows the **C engine errors on** — ignore
  the ratio.
- ~34 id-0 rows in `c.csv` are suspect: C's `DEBUG RELOAD` drops id 0 from
  the range index. The BFS rows were patched; the rest were not. Re-measure
  on a fresh C server before trusting an id-0 row.
- Promote a new rust baseline only from a `main` build, never from a
  feature branch or a combined working tree — a baseline carrying unmerged
  work makes `compare.py` report phantom regressions for everyone else.
