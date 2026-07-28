### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**main vs c-engine** — 🔴 1 of 10 comparable cell(s) over budget

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · within-run n/σ/CV of `server_ms` · client-observed total p50) and `Δms` are informational, never part of the verdict. n = samples retained after severe-outlier removal (pooled across the C workers; `n (server m)` when only `m` carry a server time); σ = their **sample** standard deviation (n−1) of `server_ms` **within this run** — not run-to-run noise; CV = 100·σ/mean. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.952<br><sub>context: p90 1.016 · p95 1.024 · p99 1.050 · 768 op/s · n/σ/CV 88/0.044/4.6% · total p50 1.189</sub> | 4.690<br><sub>context: p90 4.814 · p95 4.881 · p99 4.954 · 188 op/s · n/σ/CV 99/0.099/2.1% · total p50 5.273</sub> | +392.7% (+3.738) | 150% AND 2 ms | 🔴 |

<details><summary>example query</summary>

```cypher
CYPHER id = 2696 MATCH (u:User {id: $id}) DETACH DELETE u
```

</details>

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.213<br><sub>context: p90 0.290 · p95 0.296 · p99 0.326 · 1937 op/s · n/σ/CV 88/0.049/21.7% · total p50 0.436</sub> | 0.069<br><sub>context: p90 0.091 · p95 0.102 · p99 0.111 · 1932 op/s · n/σ/CV 97/0.014/20.5% · total p50 0.472</sub> | -67.4% (-0.143) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1984 MATCH (u:User {id: $id}) FOREACH (x IN [1,2,3] | SET u.loop_counter = x) RETURN u.loop_counter
```

</details>

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.553<br><sub>context: p90 0.621 · p95 0.630 · p99 0.654 · 1196 op/s · n/σ/CV 88/0.042/7.4% · total p50 0.769</sub> | 0.706<br><sub>context: p90 0.766 · p95 0.787 · p99 0.814 · 762 op/s · n/σ/CV 99/0.040/5.7% · total p50 1.286</sub> | +27.8% (+0.154) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 5830 to = 4099 MATCH (a:User {id: $from}), (b:User {id: $to}) MERGE (a)-[r:Friend]->(b) ON CREATE SET r.since = date(), r.bench_capacity = 1 + ((a.id * 31 + b.id * 17) % 20) ON MATCH SET r.touch = date(), r.bench_capacity = coalesce(r.bench_capacity, 1 + ((a.id * 31 + b.id * 17) % 20)) RETURN id(r)
```

</details>

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.469<br><sub>context: p90 0.518 · p95 0.525 · p99 0.573 · 1343 op/s · n/σ/CV 86/0.039/8.3% · total p50 0.674</sub> | 0.162<br><sub>context: p90 0.179 · p95 0.189 · p99 0.199 · 1734 op/s · n/σ/CV 94/0.020/12.7% · total p50 0.523</sub> | -65.4% (-0.307) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER age = 3943 id = 16814 MERGE (u:User {id: $id}) ON CREATE SET u.created_at = timestamp(), u.age = $age RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.249<br><sub>context: p90 0.312 · p95 0.326 · p99 0.338 · 1831 op/s · n/σ/CV 87/0.040/15.5% · total p50 0.459</sub> | 0.093<br><sub>context: p90 0.125 · p95 0.129 · p99 0.146 · 1866 op/s · n/σ/CV 96/0.020/21.4% · total p50 0.493</sub> | -62.6% (-0.156) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER age = 1142 id = 980 MERGE (u:User {id: $id}) ON CREATE SET u.created_at = timestamp() ON MATCH SET u.age = $age, u.last_seen = timestamp() RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.351<br><sub>context: p90 0.399 · p95 0.408 · p99 0.426 · 1450 op/s · n/σ/CV 88/0.035/9.9% · total p50 0.613</sub> | 0.181<br><sub>context: p90 0.216 · p95 0.223 · p99 0.227 · 1639 op/s · n/σ/CV 97/0.019/10.2% · total p50 0.570</sub> | -48.4% (-0.170) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 4071 MATCH (u:User {id: $id}) REMOVE u.rpc_social_credit, u:TemporaryLabel RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 26.528<br><sub>context: p90 27.297 · p95 27.402 · p99 27.598 · 37 op/s · n/σ/CV 100/0.395/1.5% · total p50 27.077</sub> | 14.496<br><sub>context: p90 14.598 · p95 14.610 · p99 14.644 · 65 op/s · n/σ/CV 87/0.075/0.5% · total p50 15.194</sub> | -45.4% (-12.033) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER color = 3352 MATCH (n:User)-[e:Friend]->(m:User) WITH n, m, e ORDER BY rand() LIMIT 1 SET e.color = $color, e.bench_capacity = coalesce(e.bench_capacity, 1 + ((n.id * 31 + m.id * 17) % 20)) RETURN e
```

</details>

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.504<br><sub>context: p90 0.568 · p95 0.586 · p99 0.676 · 1208 op/s · n/σ/CV 88/0.050/9.7% · total p50 0.770</sub> | 0.689<br><sub>context: p90 0.752 · p95 0.764 · p99 0.823 · 459 op/s · n/σ/CV 96/0.040/5.8% · total p50 2.133</sub> | +36.7% (+0.185) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 6709 to = 6984 MATCH (n:User {id: $from}), (m:User {id: $to}) MERGE (n)-[e:Friend]->(m) ON CREATE SET e.bench_capacity = 1 + ((n.id * 31 + m.id * 17) % 20) ON MATCH SET e.bench_capacity = coalesce(e.bench_capacity, 1 + ((n.id * 31 + m.id * 17) % 20)), e.touch = date() RETURN e
```

</details>

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.154<br><sub>context: p90 0.236 · p95 0.249 · p99 0.283 · 2443 op/s · n/σ/CV 89/0.047/27.9% · total p50 0.323</sub> | 0.063<br><sub>context: p90 0.092 · p95 0.098 · p99 0.119 · 2036 op/s · n/σ/CV 99/0.018/27.4% · total p50 0.451</sub> | -59.0% (-0.091) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6307 rpc_social_credit = 9796 MATCH (n:User {id: $id}) SET n.rpc_social_credit = $rpc_social_credit RETURN n
```

</details>

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.089<br><sub>context: p90 0.153 · p95 0.165 · p99 0.172 · 2783 op/s · n/σ/CV 90/0.034/34.0% · total p50 0.260</sub> | 0.078<br><sub>context: p90 0.101 · p95 0.104 · p99 0.115 · 1409 op/s · n/σ/CV 99/0.014/18.3% · total p50 0.682</sub> | -12.3% (-0.011) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6280 CREATE (n:User {id : $id}) RETURN n
```

</details>

</details>
