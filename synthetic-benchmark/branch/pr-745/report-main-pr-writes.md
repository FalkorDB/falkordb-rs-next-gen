### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 10% | 0.5 ms |
| `expand_hops_5` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `match_by_index` | 15% | 0.5 ms |
| `property_projection` | 15% | 0.5 ms |
| `return_const` | 15% | 0.5 ms |
| `shortest_path` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `single_edge_update` | 25% | 0.5 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**pr vs main** — 🟢 no p50 regression beyond budget across 10 comparable cell(s)

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 → ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · within-run n/σ/CV of `server_ms` · client-observed total p50) and `Δms` are informational, never part of the verdict. n = samples retained after severe-outlier removal (pooled across the C workers; `n (server m)` when only `m` carry a server time); σ = their **sample** standard deviation (n−1) of `server_ms` **within this run** — not run-to-run noise; CV = 100·σ/mean. Non-blocking.

<details><summary>🟢 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.690<br><sub>context: p90 4.814 · p95 4.881 · p99 4.954 · 188 op/s · n/σ/CV 99/0.099/2.1% · total p50 5.273</sub> | 4.662<br><sub>context: p90 4.786 · p95 4.860 · p99 4.916 · 191 op/s · n/σ/CV 100/0.100/2.1% · total p50 5.234</sub> | -0.6% (-0.029) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 2696 MATCH (u:User {id: $id}) DETACH DELETE u
```

</details>

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.069<br><sub>context: p90 0.091 · p95 0.102 · p99 0.111 · 1932 op/s · n/σ/CV 97/0.014/20.5% · total p50 0.472</sub> | 0.065<br><sub>context: p90 0.087 · p95 0.093 · p99 0.111 · 2423 op/s · n/σ/CV 97/0.016/24.2% · total p50 0.370</sub> | -6.9% (-0.005) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1984 MATCH (u:User {id: $id}) FOREACH (x IN [1,2,3] | SET u.loop_counter = x) RETURN u.loop_counter
```

</details>

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.706<br><sub>context: p90 0.766 · p95 0.787 · p99 0.814 · 762 op/s · n/σ/CV 99/0.040/5.7% · total p50 1.286</sub> | 0.708<br><sub>context: p90 0.771 · p95 0.783 · p99 0.799 · 782 op/s · n/σ/CV 100/0.038/5.3% · total p50 1.239</sub> | +0.2% (+0.002) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 5830 to = 4099 MATCH (a:User {id: $from}), (b:User {id: $to}) MERGE (a)-[r:Friend]->(b) ON CREATE SET r.since = date(), r.bench_capacity = 1 + ((a.id * 31 + b.id * 17) % 20) ON MATCH SET r.touch = date(), r.bench_capacity = coalesce(r.bench_capacity, 1 + ((a.id * 31 + b.id * 17) % 20)) RETURN id(r)
```

</details>

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.162<br><sub>context: p90 0.179 · p95 0.189 · p99 0.199 · 1734 op/s · n/σ/CV 94/0.020/12.7% · total p50 0.523</sub> | 0.154<br><sub>context: p90 0.175 · p95 0.179 · p99 0.189 · 1799 op/s · n/σ/CV 91/0.021/13.7% · total p50 0.493</sub> | -5.2% (-0.008) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER age = 3943 id = 16814 MERGE (u:User {id: $id}) ON CREATE SET u.created_at = timestamp(), u.age = $age RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.093<br><sub>context: p90 0.125 · p95 0.129 · p99 0.146 · 1866 op/s · n/σ/CV 96/0.020/21.4% · total p50 0.493</sub> | 0.097<br><sub>context: p90 0.130 · p95 0.134 · p99 0.143 · 1738 op/s · n/σ/CV 99/0.019/18.4% · total p50 0.536</sub> | +4.2% (+0.004) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER age = 1142 id = 980 MERGE (u:User {id: $id}) ON CREATE SET u.created_at = timestamp() ON MATCH SET u.age = $age, u.last_seen = timestamp() RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.181<br><sub>context: p90 0.216 · p95 0.223 · p99 0.227 · 1639 op/s · n/σ/CV 97/0.019/10.2% · total p50 0.570</sub> | 0.181<br><sub>context: p90 0.214 · p95 0.217 · p99 0.243 · 1790 op/s · n/σ/CV 89/0.021/11.4% · total p50 0.511</sub> | -0.2% (-0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 4071 MATCH (u:User {id: $id}) REMOVE u.rpc_social_credit, u:TemporaryLabel RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 14.496<br><sub>context: p90 14.598 · p95 14.610 · p99 14.644 · 65 op/s · n/σ/CV 87/0.075/0.5% · total p50 15.194</sub> | 14.610<br><sub>context: p90 14.804 · p95 15.052 · p99 15.181 · 65 op/s · n/σ/CV 92/0.170/1.2% · total p50 15.385</sub> | +0.8% (+0.114) | 25% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER color = 3352 MATCH (n:User)-[e:Friend]->(m:User) WITH n, m, e ORDER BY rand() LIMIT 1 SET e.color = $color, e.bench_capacity = coalesce(e.bench_capacity, 1 + ((n.id * 31 + m.id * 17) % 20)) RETURN e
```

</details>

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.689<br><sub>context: p90 0.752 · p95 0.764 · p99 0.823 · 459 op/s · n/σ/CV 96/0.040/5.8% · total p50 2.133</sub> | 0.710<br><sub>context: p90 0.768 · p95 0.783 · p99 0.823 · 442 op/s · n/σ/CV 98/0.037/5.2% · total p50 2.239</sub> | +3.1% (+0.021) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 6709 to = 6984 MATCH (n:User {id: $from}), (m:User {id: $to}) MERGE (n)-[e:Friend]->(m) ON CREATE SET e.bench_capacity = 1 + ((n.id * 31 + m.id * 17) % 20) ON MATCH SET e.bench_capacity = coalesce(e.bench_capacity, 1 + ((n.id * 31 + m.id * 17) % 20)), e.touch = date() RETURN e
```

</details>

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.063<br><sub>context: p90 0.092 · p95 0.098 · p99 0.119 · 2036 op/s · n/σ/CV 99/0.018/27.4% · total p50 0.451</sub> | 0.063<br><sub>context: p90 0.091 · p95 0.098 · p99 0.106 · 2342 op/s · n/σ/CV 90/0.018/27.5% · total p50 0.384</sub> | -0.8% (-0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6307 rpc_social_credit = 9796 MATCH (n:User {id: $id}) SET n.rpc_social_credit = $rpc_social_credit RETURN n
```

</details>

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.078<br><sub>context: p90 0.101 · p95 0.104 · p99 0.115 · 1409 op/s · n/σ/CV 99/0.014/18.3% · total p50 0.682</sub> | 0.081<br><sub>context: p90 0.092 · p95 0.095 · p99 0.115 · 1569 op/s · n/σ/CV 92/0.010/12.2% · total p50 0.590</sub> | +3.6% (+0.003) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6280 CREATE (n:User {id : $id}) RETURN n
```

</details>

</details>
