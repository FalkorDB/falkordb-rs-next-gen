### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad |
| workload_hash | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` |
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

**pr vs main** — 🟢 no p50 regression beyond budget across 100 comparable cell(s)

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 → ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · within-run n/σ/CV of `server_ms` · client-observed total p50) and `Δms` are informational, never part of the verdict. n = samples retained after severe-outlier removal (pooled across the C workers; `n (server m)` when only `m` carry a server time); σ = their **sample** standard deviation (n−1) of `server_ms` **within this run** — not run-to-run noise; CV = 100·σ/mean. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.602<br><sub>context: p90 0.626 · p95 0.634 · p99 0.644 · 1329 op/s · n/σ/CV 198/0.018/3.0% · total p50 2.998</sub> | 0.605<br><sub>context: p90 0.632 · p95 0.639 · p99 0.673 · 1305 op/s · n/σ/CV 194/0.022/3.6% · total p50 3.034</sub> | +0.6% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.615<br><sub>context: p90 0.668 · p95 0.691 · p99 0.733 · 9084 op/s · n/σ/CV 1554/0.034/5.5% · total p50 3.482</sub> | 0.615<br><sub>context: p90 0.663 · p95 0.681 · p99 0.722 · 9058 op/s · n/σ/CV 1579/0.031/5.0% · total p50 3.455</sub> | +0.0% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN avg(n.age) AS avg_age
```

</details>

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.629<br><sub>context: p90 0.650 · p95 0.660 · p99 0.675 · 1297 op/s · n/σ/CV 198/0.017/2.8% · total p50 3.064</sub> | 0.643<br><sub>context: p90 0.668 · p95 0.680 · p99 0.708 · 1236 op/s · n/σ/CV 196/0.020/3.2% · total p50 3.229</sub> | +2.4% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.653<br><sub>context: p90 0.700 · p95 0.723 · p99 0.771 · 8705 op/s · n/σ/CV 1570/0.034/5.2% · total p50 3.621</sub> | 0.659<br><sub>context: p90 0.724 · p95 0.752 · p99 0.798 · 8339 op/s · n/σ/CV 1549/0.041/6.2% · total p50 3.750</sub> | +1.0% (+0.007) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN count(DISTINCT n.age) AS distinct_ages
```

</details>

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.098<br><sub>context: p90 1.123 · p95 1.130 · p99 1.152 · 786 op/s · n/σ/CV 197/0.019/1.8% · total p50 5.071</sub> | 1.112<br><sub>context: p90 1.141 · p95 1.158 · p99 1.187 · 755 op/s · n/σ/CV 194/0.023/2.0% · total p50 5.287</sub> | +1.2% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.740<br><sub>context: p90 2.436 · p95 2.616 · p99 2.810 · 3768 op/s · n/σ/CV 1592/0.458/25.4% · total p50 8.366</sub> | 1.698<br><sub>context: p90 2.391 · p95 2.612 · p99 2.896 · 3868 op/s · n/σ/CV 1597/0.471/26.8% · total p50 8.085</sub> | -2.4% (-0.042) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) WHERE n.age >= 18 RETURN avg(n.age) AS avg_age
```

</details>

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.194<br><sub>context: p90 1.220 · p95 1.225 · p99 1.239 · 724 op/s · n/σ/CV 196/0.020/1.7% · total p50 5.515</sub> | 1.194<br><sub>context: p90 1.220 · p95 1.229 · p99 1.250 · 719 op/s · n/σ/CV 196/0.020/1.6% · total p50 5.547</sub> | -0.0% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.217<br><sub>context: p90 1.347 · p95 1.395 · p99 1.467 · 5105 op/s · n/σ/CV 1552/0.071/5.8% · total p50 6.074</sub> | 1.219<br><sub>context: p90 1.321 · p95 1.363 · p99 1.451 · 4869 op/s · n/σ/CV 1543/0.063/5.1% · total p50 6.469</sub> | +0.2% (+0.002) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN min(n.age) AS min_age, max(n.age) AS max_age, avg(n.age) AS avg_age
```

</details>

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.017<br><sub>context: p90 0.026 · p95 0.029 · p99 0.038 · 10668 op/s · n/σ/CV 179/0.005/29.7% · total p50 0.327</sub> | 0.015<br><sub>context: p90 0.022 · p95 0.027 · p99 0.032 · 12605 op/s · n/σ/CV 187/0.005/28.6% · total p50 0.291</sub> | -10.1% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.016<br><sub>context: p90 0.023 · p95 0.025 · p99 0.032 · 53159 op/s · n/σ/CV 1431/0.005/26.3% · total p50 0.431</sub> | 0.016<br><sub>context: p90 0.024 · p95 0.026 · p99 0.033 · 53301 op/s · n/σ/CV 1508/0.005/25.6% · total p50 0.431</sub> | +2.7% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.meta.stats() YIELD nodeCount RETURN nodeCount AS cnt
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.074<br><sub>context: p90 0.103 · p95 0.110 · p99 0.143 · 3375 op/s · n/σ/CV 195/0.021/27.7% · total p50 1.122</sub> | 0.057<br><sub>context: p90 0.083 · p95 0.095 · p99 0.103 · 4807 op/s · n/σ/CV 197/0.018/29.9% · total p50 0.811</sub> | -23.0% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.056<br><sub>context: p90 0.091 · p95 0.099 · p99 0.124 · 32476 op/s · n/σ/CV 1570/0.021/34.3% · total p50 0.894</sub> | 0.056<br><sub>context: p90 0.091 · p95 0.097 · p99 0.121 · 32336 op/s · n/σ/CV 1560/0.021/34.4% · total p50 0.913</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 4275 MATCH (s:User {id: $id})-->(n:User) RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.065<br><sub>context: p90 0.094 · p95 0.098 · p99 0.109 · 4567 op/s · n/σ/CV 200/0.017/26.1% · total p50 0.859</sub> | 0.071<br><sub>context: p90 0.098 · p95 0.101 · p99 0.113 · 3724 op/s · n/σ/CV 197/0.018/25.5% · total p50 1.041</sub> | +9.5% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.059<br><sub>context: p90 0.092 · p95 0.098 · p99 0.143 · 32395 op/s · n/σ/CV 1584/0.021/32.7% · total p50 0.905</sub> | 0.070<br><sub>context: p90 0.106 · p95 0.121 · p99 0.168 · 21276 op/s · n/σ/CV 1537/0.026/34.7% · total p50 1.328</sub> | +19.9% (+0.012) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6967 MATCH (s:User {id: $id})-->(n:User)  WHERE n.age >= 18  RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.157 · p95 0.163 · p99 0.173 · 3006 op/s · n/σ/CV 200/0.020/15.7% · total p50 1.318</sub> | 0.134<br><sub>context: p90 0.162 · p95 0.172 · p99 0.196 · 2788 op/s · n/σ/CV 198/0.022/16.6% · total p50 1.407</sub> | +3.2% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.103<br><sub>context: p90 0.136 · p95 0.143 · p99 0.158 · 27313 op/s · n/σ/CV 1581/0.020/19.4% · total p50 1.069</sub> | 0.105<br><sub>context: p90 0.142 · p95 0.152 · p99 0.174 · 25363 op/s · n/σ/CV 1581/0.023/21.7% · total p50 1.173</sub> | +1.5% (+0.002) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 7714 MATCH (s:User {id: $id})-->()-->(n:User) RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.124<br><sub>context: p90 0.154 · p95 0.161 · p99 0.182 · 3167 op/s · n/σ/CV 199/0.024/19.4% · total p50 1.260</sub> | 0.120<br><sub>context: p90 0.152 · p95 0.158 · p99 0.169 · 3266 op/s · n/σ/CV 200/0.023/19.4% · total p50 1.230</sub> | -3.1% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.110<br><sub>context: p90 0.147 · p95 0.156 · p99 0.181 · 25053 op/s · n/σ/CV 1588/0.025/21.6% · total p50 1.173</sub> | 0.109<br><sub>context: p90 0.148 · p95 0.159 · p99 0.189 · 24218 op/s · n/σ/CV 1584/0.025/22.3% · total p50 1.202</sub> | -0.7% (-0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5665 MATCH (s:User {id: $id})-->()-->(n:User)  WHERE n.age >= 18  RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.139<br><sub>context: p90 0.179 · p95 0.189 · p99 0.212 · 3181 op/s · n/σ/CV 200/0.029/20.2% · total p50 1.252</sub> | 0.164<br><sub>context: p90 0.197 · p95 0.206 · p99 0.248 · 2626 op/s · n/σ/CV 199/0.028/16.6% · total p50 1.508</sub> | +18.5% (+0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.151<br><sub>context: p90 0.199 · p95 0.214 · p99 0.244 · 15376 op/s · n/σ/CV 1600/0.034/22.4% · total p50 1.961</sub> | 0.151<br><sub>context: p90 0.203 · p95 0.214 · p99 0.242 · 15215 op/s · n/σ/CV 1600/0.035/22.4% · total p50 1.972</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1612 MATCH (s:User {id: $id})-->()-->()-->(n:User) RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.164<br><sub>context: p90 0.204 · p95 0.214 · p99 0.235 · 2652 op/s · n/σ/CV 199/0.028/17.2% · total p50 1.503</sub> | 0.151<br><sub>context: p90 0.197 · p95 0.209 · p99 0.227 · 2861 op/s · n/σ/CV 199/0.030/19.5% · total p50 1.369</sub> | -7.5% (-0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.161<br><sub>context: p90 0.216 · p95 0.231 · p99 0.256 · 14297 op/s · n/σ/CV 1600/0.038/23.2% · total p50 2.138</sub> | 0.162<br><sub>context: p90 0.217 · p95 0.231 · p99 0.261 · 13590 op/s · n/σ/CV 1600/0.038/23.1% · total p50 2.239</sub> | +0.8% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 460 MATCH (s:User {id: $id})-->()-->()-->(n:User)  WHERE n.age >= 18  RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.287<br><sub>context: p90 0.377 · p95 0.392 · p99 0.427 · 1651 op/s · n/σ/CV 198/0.062/21.2% · total p50 2.217</sub> | 0.282<br><sub>context: p90 0.364 · p95 0.384 · p99 0.465 · 1775 op/s · n/σ/CV 200/0.065/22.8% · total p50 2.177</sub> | -1.9% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.281<br><sub>context: p90 0.376 · p95 0.410 · p99 0.461 · 3934 op/s · n/σ/CV 1600/0.068/23.7% · total p50 7.733</sub> | 0.281<br><sub>context: p90 0.379 · p95 0.415 · p99 0.462 · 3780 op/s · n/σ/CV 1600/0.069/23.9% · total p50 7.989</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 2820 MATCH (s:User {id: $id})-->()-->()-->()-->(n:User) RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.379 · p95 0.399 · p99 0.457 · 1737 op/s · n/σ/CV 200/0.062/20.6% · total p50 2.242</sub> | 0.300<br><sub>context: p90 0.366 · p95 0.383 · p99 0.428 · 1723 op/s · n/σ/CV 197/0.055/18.6% · total p50 2.238</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.298<br><sub>context: p90 0.390 · p95 0.420 · p99 0.463 · 3573 op/s · n/σ/CV 1600/0.066/21.8% · total p50 8.666</sub> | 0.300<br><sub>context: p90 0.391 · p95 0.424 · p99 0.474 · 3733 op/s · n/σ/CV 1600/0.066/21.8% · total p50 8.104</sub> | +0.7% (+0.002) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5473 MATCH (s:User {id: $id})-->()-->()-->()-->(n:User)  WHERE n.age >= 18 RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.334<br><sub>context: p90 0.516 · p95 0.553 · p99 0.630 · 1980 op/s · n/σ/CV 199/0.132/38.4% · total p50 1.993</sub> | 0.355<br><sub>context: p90 0.537 · p95 0.587 · p99 0.670 · 1735 op/s · n/σ/CV 200/0.140/38.0% · total p50 2.313</sub> | +6.1% (+0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.419<br><sub>context: p90 0.647 · p95 0.715 · p99 0.886 · 11694 op/s · n/σ/CV 1599/0.171/40.3% · total p50 2.659</sub> | 0.434<br><sub>context: p90 0.694 · p95 0.790 · p99 0.991 · 10226 op/s · n/σ/CV 1599/0.189/41.8% · total p50 3.006</sub> | +3.5% (+0.015) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 326 to = 140 MATCH (s:User {id: $from}), (t:User {id: $to}) WITH s, t MATCH p = allShortestPaths((s)-[:Friend*1..4]->(t)) RETURN length(p)
```

</details>

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.086<br><sub>context: p90 0.113 · p95 0.125 · p99 0.138 · 4614 op/s · n/σ/CV 200/0.021/24.3% · total p50 0.841</sub> | 0.091<br><sub>context: p90 0.121 · p95 0.133 · p99 0.144 · 4141 op/s · n/σ/CV 200/0.021/22.7% · total p50 0.929</sub> | +6.0% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.086<br><sub>context: p90 0.119 · p95 0.126 · p99 0.145 · 29403 op/s · n/σ/CV 1587/0.021/23.1% · total p50 1.041</sub> | 0.087<br><sub>context: p90 0.120 · p95 0.127 · p99 0.153 · 29358 op/s · n/σ/CV 1591/0.022/24.0% · total p50 1.034</sub> | +0.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1670 MATCH (a:User {id: $id}) CALL { WITH a MATCH (a)-->(b:User) RETURN b.id AS bid } RETURN bid
```

</details>

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 12681 op/s · n/σ/CV 190/0.000/19.5% · total p50 0.299</sub> | 0.003<br><sub>context: p90 0.003 · p95 0.004 · p99 0.004 · 10153 op/s · n/σ/CV 185/0.001/18.8% · total p50 0.360</sub> | +22.6% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 55349 op/s · n/σ/CV 1444/0.001/26.3% · total p50 0.431</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 53346 op/s · n/σ/CV 1440/0.001/28.0% · total p50 0.436</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH ()-[r:Friend]->() RETURN count(r) AS cnt
```

</details>

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 17157 op/s · n/σ/CV 185/0.000/21.1% · total p50 0.221</sub> | 0.003<br><sub>context: p90 0.006 · p95 0.006 · p99 0.008 · 11323 op/s · n/σ/CV 197/0.002/47.9% · total p50 0.323</sub> | +65.9% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 52547 op/s · n/σ/CV 1363/0.001/28.1% · total p50 0.401</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 48498 op/s · n/σ/CV 1358/0.001/32.6% · total p50 0.443</sub> | -1.7% (-0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (u:User) RETURN count(u) AS cnt
```

</details>

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.072<br><sub>context: p90 0.100 · p95 0.113 · p99 0.121 · 3821 op/s · n/σ/CV 200/0.019/24.6% · total p50 1.022</sub> | 0.079<br><sub>context: p90 0.105 · p95 0.114 · p99 0.127 · 3511 op/s · n/σ/CV 199/0.018/22.2% · total p50 1.135</sub> | +9.0% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.078<br><sub>context: p90 0.108 · p95 0.117 · p99 0.129 · 24679 op/s · n/σ/CV 1595/0.019/24.0% · total p50 1.193</sub> | 0.078<br><sub>context: p90 0.110 · p95 0.117 · p99 0.134 · 25655 op/s · n/σ/CV 1594/0.020/24.7% · total p50 1.192</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 643 MATCH p=(a:User {id: $id})-[r:Friend]->(b:User) RETURN labels(a), type(r), properties(a), nodes(p), relationships(p), length(p) LIMIT 1
```

</details>

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.118<br><sub>context: p90 1.540 · p95 1.628 · p99 1.799 · 763 op/s · n/σ/CV 200/0.329/29.8% · total p50 5.217</sub> | 1.110<br><sub>context: p90 1.553 · p95 1.646 · p99 1.769 · 772 op/s · n/σ/CV 200/0.334/30.6% · total p50 5.170</sub> | -0.7% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.829<br><sub>context: p90 2.752 · p95 2.971 · p99 3.434 · 3594 op/s · n/σ/CV 1600/0.655/35.2% · total p50 8.349</sub> | 1.780<br><sub>context: p90 2.603 · p95 2.803 · p99 3.292 · 3514 op/s · n/σ/CV 1586/0.609/33.7% · total p50 8.381</sub> | -2.6% (-0.048) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 7839 MATCH (s:User {id: $id})-[:Friend*5..5]->(t:User) RETURN count(t) AS cnt
```

</details>

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.551<br><sub>context: p90 7.387 · p95 7.845 · p99 8.034 · 197 op/s · n/σ/CV 200/1.834/38.0% · total p50 20.118</sub> | 4.768<br><sub>context: p90 7.640 · p95 7.952 · p99 8.293 · 190 op/s · n/σ/CV 200/1.872/37.6% · total p50 20.728</sub> | +4.8% (+0.218) | 10% AND 0.5 ms | 🟢 |
| 8 | 7.791<br><sub>context: p90 12.282 · p95 13.015 · p99 14.035 · 932 op/s · n/σ/CV 1600/2.858/35.0% · total p50 33.688</sub> | 7.780<br><sub>context: p90 12.226 · p95 12.978 · p99 14.134 · 926 op/s · n/σ/CV 1600/2.855/35.0% · total p50 33.600</sub> | -0.1% (-0.011) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1982 MATCH (s:User {id: $id})-[:Friend*6..6]->(t:User) RETURN count(t) AS cnt
```

</details>

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.018<br><sub>context: p90 0.036 · p95 0.038 · p99 0.042 · 8891 op/s · n/σ/CV 199/0.008/36.4% · total p50 0.395</sub> | 0.019<br><sub>context: p90 0.031 · p95 0.036 · p99 0.040 · 9249 op/s · n/σ/CV 198/0.007/33.5% · total p50 0.421</sub> | +2.9% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.019<br><sub>context: p90 0.024 · p95 0.027 · p99 0.033 · 41962 op/s · n/σ/CV 1530/0.004/21.0% · total p50 0.614</sub> | 0.018<br><sub>context: p90 0.023 · p95 0.026 · p99 0.030 · 44943 op/s · n/σ/CV 1516/0.004/19.7% · total p50 0.540</sub> | -2.9% (-0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.idx.fulltext.queryNodes('User', 'fixture_alice') YIELD node, score RETURN id(node), score LIMIT 10
```

</details>

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.033<br><sub>context: p90 0.042 · p95 0.043 · p99 0.048 · 5912 op/s · n/σ/CV 198/0.009/27.6% · total p50 0.666</sub> | 0.019<br><sub>context: p90 0.034 · p95 0.036 · p99 0.038 · 8782 op/s · n/σ/CV 200/0.007/37.2% · total p50 0.443</sub> | -43.2% (-0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.018<br><sub>context: p90 0.024 · p95 0.026 · p99 0.031 · 44822 op/s · n/σ/CV 1568/0.004/20.6% · total p50 0.590</sub> | 0.018<br><sub>context: p90 0.023 · p95 0.025 · p99 0.031 · 45402 op/s · n/σ/CV 1564/0.004/20.1% · total p50 0.591</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.idx.fulltext.queryRelationships('Friend', 'fixture_blue') YIELD relationship, score RETURN id(relationship), score LIMIT 10
```

</details>

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.011<br><sub>context: p90 0.016 · p95 0.017 · p99 0.019 · 7947 op/s · n/σ/CV 195/0.003/24.7% · total p50 0.477</sub> | 0.010<br><sub>context: p90 0.016 · p95 0.017 · p99 0.019 · 8229 op/s · n/σ/CV 196/0.003/27.7% · total p50 0.460</sub> | -8.9% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.011<br><sub>context: p90 0.014 · p95 0.017 · p99 0.021 · 17812 op/s · n/σ/CV 1596/0.003/27.0% · total p50 1.696</sub> | 0.011<br><sub>context: p90 0.017 · p95 0.020 · p99 0.022 · 17771 op/s · n/σ/CV 1595/0.004/30.4% · total p50 1.724</sub> | +6.7% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER end = 2049 start = 1949 MATCH (n) WHERE id(n) >= $start AND id(n) < $end RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.006<br><sub>context: p90 0.012 · p95 0.013 · p99 0.014 · 9861 op/s · n/σ/CV 196/0.003/36.6% · total p50 0.389</sub> | 0.006<br><sub>context: p90 0.007 · p95 0.007 · p99 0.008 · 12175 op/s · n/σ/CV 195/0.001/17.6% · total p50 0.318</sub> | -6.8% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.005<br><sub>context: p90 0.008 · p95 0.008 · p99 0.011 · 61569 op/s · n/σ/CV 1484/0.002/28.4% · total p50 0.418</sub> | 0.005<br><sub>context: p90 0.008 · p95 0.008 · p99 0.010 · 51420 op/s · n/σ/CV 1421/0.002/28.2% · total p50 0.423</sub> | -0.2% (-0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 2122 MATCH (n) WHERE id(n) = $id RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.141<br><sub>context: p90 0.180 · p95 0.187 · p99 0.216 · 3700 op/s · n/σ/CV 199/0.030/21.1% · total p50 1.040</sub> | 0.148<br><sub>context: p90 0.187 · p95 0.199 · p99 0.215 · 3456 op/s · n/σ/CV 199/0.028/19.2% · total p50 1.150</sub> | +5.1% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.150<br><sub>context: p90 0.195 · p95 0.205 · p99 0.234 · 26753 op/s · n/σ/CV 1587/0.034/22.9% · total p50 1.140</sub> | 0.150<br><sub>context: p90 0.193 · p95 0.208 · p99 0.233 · 26078 op/s · n/σ/CV 1583/0.034/22.8% · total p50 1.113</sub> | +0.4% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id1 = 5885 id2 = 9218 id3 = 9441 id4 = 2621 MATCH (u:User) WHERE u.id IN [$id1, $id2, $id3, $id4] RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.069<br><sub>context: p90 0.096 · p95 0.100 · p99 0.108 · 5751 op/s · n/σ/CV 199/0.019/25.9% · total p50 0.693</sub> | 0.071<br><sub>context: p90 0.097 · p95 0.105 · p99 0.115 · 4980 op/s · n/σ/CV 200/0.019/25.0% · total p50 0.783</sub> | +3.3% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.074<br><sub>context: p90 0.108 · p95 0.117 · p99 0.150 · 36719 op/s · n/σ/CV 1595/0.025/31.7% · total p50 0.819</sub> | 0.074<br><sub>context: p90 0.109 · p95 0.119 · p99 0.156 · 35778 op/s · n/σ/CV 1590/0.027/33.0% · total p50 0.831</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id1 = 5236 id2 = 3248 MATCH (u:User) WHERE u.id = $id1 OR u.id = $id2 RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.126<br><sub>context: p90 0.162 · p95 0.169 · p99 0.189 · 2871 op/s · n/σ/CV 197/0.024/18.5% · total p50 1.365</sub> | 0.103<br><sub>context: p90 0.137 · p95 0.149 · p99 0.164 · 3855 op/s · n/σ/CV 199/0.023/21.4% · total p50 1.026</sub> | -18.4% (-0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.102<br><sub>context: p90 0.136 · p95 0.145 · p99 0.167 · 26295 op/s · n/σ/CV 1583/0.022/21.2% · total p50 1.130</sub> | 0.102<br><sub>context: p90 0.138 · p95 0.146 · p99 0.165 · 26768 op/s · n/σ/CV 1588/0.022/20.8% · total p50 1.104</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 692 MATCH (s:User {id: $id})-->()-->(n:User) RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.107<br><sub>context: p90 0.144 · p95 0.153 · p99 0.168 · 3273 op/s · n/σ/CV 200/0.024/22.4% · total p50 1.201</sub> | 0.110<br><sub>context: p90 0.147 · p95 0.155 · p99 0.171 · 3302 op/s · n/σ/CV 200/0.027/24.9% · total p50 1.211</sub> | +2.9% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.104<br><sub>context: p90 0.144 · p95 0.155 · p99 0.175 · 14896 op/s · n/σ/CV 1599/0.026/24.8% · total p50 2.030</sub> | 0.105<br><sub>context: p90 0.144 · p95 0.155 · p99 0.174 · 14420 op/s · n/σ/CV 1600/0.026/24.4% · total p50 2.129</sub> | +1.1% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6334 MATCH (s:User {id: $id})-->()-->(n:User) RETURN n
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.117<br><sub>context: p90 0.150 · p95 0.162 · p99 0.169 · 3102 op/s · n/σ/CV 199/0.023/19.8% · total p50 1.274</sub> | 0.124<br><sub>context: p90 0.156 · p95 0.166 · p99 0.177 · 2889 op/s · n/σ/CV 200/0.025/20.0% · total p50 1.370</sub> | +6.5% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.110<br><sub>context: p90 0.149 · p95 0.157 · p99 0.175 · 16929 op/s · n/σ/CV 1600/0.026/23.4% · total p50 1.801</sub> | 0.111<br><sub>context: p90 0.148 · p95 0.160 · p99 0.184 · 16382 op/s · n/σ/CV 1600/0.027/23.4% · total p50 1.849</sub> | +1.0% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 9102 MATCH (s:User {id: $id})-->()-->(n:User) WHERE n.age >= 18 RETURN n
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.137 · p95 0.142 · p99 0.168 · 3609 op/s · n/σ/CV 200/0.023/21.3% · total p50 1.080</sub> | 0.111<br><sub>context: p90 0.139 · p95 0.146 · p99 0.161 · 3436 op/s · n/σ/CV 199/0.021/18.6% · total p50 1.167</sub> | +6.8% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.103<br><sub>context: p90 0.140 · p95 0.150 · p99 0.166 · 26027 op/s · n/σ/CV 1573/0.023/21.7% · total p50 1.127</sub> | 0.108<br><sub>context: p90 0.149 · p95 0.161 · p99 0.197 · 23117 op/s · n/σ/CV 1573/0.027/23.9% · total p50 1.248</sub> | +4.6% (+0.005) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 3235 MATCH (s:User {id: $id})-->()-->(n:User) WHERE n.age >= 18 RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.085<br><sub>context: p90 0.113 · p95 0.123 · p99 0.149 · 4659 op/s · n/σ/CV 200/0.023/26.7% · total p50 0.848</sub> | 0.108<br><sub>context: p90 0.141 · p95 0.150 · p99 0.195 · 3465 op/s · n/σ/CV 200/0.025/22.6% · total p50 1.136</sub> | +27.1% (+0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.081<br><sub>context: p90 0.116 · p95 0.124 · p99 0.148 · 32744 op/s · n/σ/CV 1588/0.023/26.4% · total p50 0.910</sub> | 0.082<br><sub>context: p90 0.119 · p95 0.128 · p99 0.164 · 32123 op/s · n/σ/CV 1582/0.024/27.6% · total p50 0.930</sub> | +0.7% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 8719 MATCH (a:User {id: $id}) OPTIONAL MATCH (a)-->(b:User) RETURN a.id, b.id
```

</details>

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.848<br><sub>context: p90 2.886 · p95 2.902 · p99 2.982 · 92 op/s · n/σ/CV 169/0.031/1.1% · total p50 42.356</sub> | 2.886<br><sub>context: p90 2.925 · p95 2.938 · p99 2.967 · 93 op/s · n/σ/CV 178/0.028/1.0% · total p50 42.789</sub> | +1.3% (+0.038) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.872<br><sub>context: p90 3.918 · p95 3.996 · p99 4.113 · 154 op/s · n/σ/CV 1600/0.444/14.3% · total p50 199.429</sub> | 2.904<br><sub>context: p90 3.141 · p95 3.631 · p99 3.720 · 153 op/s · n/σ/CV 1363/0.201/6.8% · total p50 206.202</sub> | +1.1% (+0.032) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN n.id, n.age ORDER BY n.age, n.id
```

</details>

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.114<br><sub>context: p90 0.152 · p95 0.159 · p99 0.173 · 3355 op/s · n/σ/CV 200/0.027/23.4% · total p50 1.196</sub> | 0.108<br><sub>context: p90 0.147 · p95 0.154 · p99 0.172 · 3548 op/s · n/σ/CV 198/0.027/24.4% · total p50 1.110</sub> | -5.8% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.107<br><sub>context: p90 0.143 · p95 0.150 · p99 0.168 · 24554 op/s · n/σ/CV 1586/0.025/22.8% · total p50 1.183</sub> | 0.111<br><sub>context: p90 0.147 · p95 0.157 · p99 0.174 · 23357 op/s · n/σ/CV 1597/0.025/22.4% · total p50 1.319</sub> | +3.8% (+0.004) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 9234 MATCH (a:User {id: $id})-->(b:User)-->(c:User)-->(a) RETURN a.id, b.id, c.id
```

</details>

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.341 · p95 0.359 · p99 0.413 · 1085 op/s · n/σ/CV 200/0.054/20.4% · total p50 3.476</sub> | 0.261<br><sub>context: p90 0.338 · p95 0.356 · p99 0.389 · 1042 op/s · n/σ/CV 200/0.055/20.9% · total p50 4.141</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.271<br><sub>context: p90 0.358 · p95 0.396 · p99 0.479 · 2552 op/s · n/σ/CV 1596/0.066/24.1% · total p50 11.618</sub> | 0.280<br><sub>context: p90 0.376 · p95 0.403 · p99 0.469 · 2450 op/s · n/σ/CV 1593/0.066/23.2% · total p50 12.329</sub> | +3.6% (+0.010) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 3234 MATCH (a:User {id: $id})-->()-->()-->()-->(b:User) RETURN a.id, b.id
```

</details>

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.143<br><sub>context: p90 0.174 · p95 0.185 · p99 0.215 · 2470 op/s · n/σ/CV 199/0.024/16.2% · total p50 1.600</sub> | 0.126<br><sub>context: p90 0.156 · p95 0.160 · p99 0.169 · 3208 op/s · n/σ/CV 199/0.023/18.2% · total p50 1.241</sub> | -11.7% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.113<br><sub>context: p90 0.152 · p95 0.164 · p99 0.195 · 23360 op/s · n/σ/CV 1585/0.028/23.9% · total p50 1.245</sub> | 0.112<br><sub>context: p90 0.152 · p95 0.166 · p99 0.209 · 25027 op/s · n/σ/CV 1579/0.029/24.8% · total p50 1.194</sub> | -1.5% (-0.002) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5820 MATCH (a:User {id: $id})-->()-->(b:User) RETURN a.id, b.id
```

</details>

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.118<br><sub>context: p90 0.164 · p95 0.175 · p99 0.222 · 3861 op/s · n/σ/CV 200/0.033/26.6% · total p50 1.030</sub> | 0.139<br><sub>context: p90 0.191 · p95 0.212 · p99 0.275 · 2894 op/s · n/σ/CV 200/0.038/26.5% · total p50 1.351</sub> | +18.1% (+0.021) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.125<br><sub>context: p90 0.173 · p95 0.190 · p99 0.239 · 25823 op/s · n/σ/CV 1594/0.035/26.8% · total p50 1.180</sub> | 0.129<br><sub>context: p90 0.189 · p95 0.215 · p99 0.261 · 22491 op/s · n/σ/CV 1579/0.040/29.4% · total p50 1.262</sub> | +3.6% (+0.005) | 12% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 1990 to = 6799 MATCH (s:User {id: $from}), (t:User {id: $to}) WITH shortestPath((s)-[*]->(t)) AS p RETURN length(p)
```

</details>

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.116<br><sub>context: p90 0.170 · p95 0.186 · p99 0.213 · 3737 op/s · n/σ/CV 199/0.031/25.7% · total p50 1.060</sub> | 0.122<br><sub>context: p90 0.165 · p95 0.191 · p99 0.211 · 3326 op/s · n/σ/CV 198/0.029/23.1% · total p50 1.184</sub> | +5.3% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.121<br><sub>context: p90 0.172 · p95 0.188 · p99 0.213 · 25761 op/s · n/σ/CV 1595/0.032/25.5% · total p50 1.149</sub> | 0.121<br><sub>context: p90 0.169 · p95 0.190 · p99 0.209 · 25595 op/s · n/σ/CV 1591/0.032/25.7% · total p50 1.128</sub> | -0.6% (-0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 6363 to = 141 MATCH (s:User {id: $from}), (t:User {id: $to}) WITH shortestPath((s)-[*]->(t)) AS p WHERE length(p) > 0 RETURN length(p)
```

</details>

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.039<br><sub>context: p90 0.071 · p95 0.074 · p99 0.079 · 7273 op/s · n/σ/CV 200/0.018/40.4% · total p50 0.525</sub> | 0.039<br><sub>context: p90 0.065 · p95 0.074 · p99 0.080 · 6787 op/s · n/σ/CV 200/0.017/38.6% · total p50 0.583</sub> | -1.0% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.039<br><sub>context: p90 0.072 · p95 0.077 · p99 0.093 · 41198 op/s · n/σ/CV 1583/0.020/44.1% · total p50 0.646</sub> | 0.040<br><sub>context: p90 0.074 · p95 0.080 · p99 0.100 · 37820 op/s · n/σ/CV 1588/0.021/45.2% · total p50 0.752</sub> | +1.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 988 MATCH (n:User {id : $id}) RETURN n
```

</details>

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.004<br><sub>context: p90 0.006 · p95 0.008 · p99 0.009 · 8902 op/s · n/σ/CV 187/0.001/33.0% · total p50 0.418</sub> | 0.005<br><sub>context: p90 0.009 · p95 0.010 · p99 0.012 · 7254 op/s · n/σ/CV 200/0.003/41.9% · total p50 0.536</sub> | +32.5% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 49130 op/s · n/σ/CV 1541/0.001/21.7% · total p50 0.492</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 47679 op/s · n/σ/CV 1559/0.001/24.6% · total p50 0.510</sub> | +3.8% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  RETURN date('2024-01-01') AS d, localtime('12:30:00') AS t, duration('P2DT3H') AS dur, distance( point({latitude: 32.1, longitude: 34.8}), point({latitude: 32.2, longitude: 34.9}) ) AS dist
```

</details>

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.048<br><sub>context: p90 0.077 · p95 0.084 · p99 0.095 · 5943 op/s · n/σ/CV 200/0.017/31.4% · total p50 0.658</sub> | 0.058<br><sub>context: p90 0.087 · p95 0.090 · p99 0.098 · 4359 op/s · n/σ/CV 198/0.017/29.0% · total p50 0.904</sub> | +22.3% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.050<br><sub>context: p90 0.082 · p95 0.084 · p99 0.093 · 36863 op/s · n/σ/CV 1585/0.016/30.3% · total p50 0.830</sub> | 0.051<br><sub>context: p90 0.083 · p95 0.088 · p99 0.100 · 31099 op/s · n/σ/CV 1559/0.017/30.9% · total p50 0.894</sub> | +2.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 8021 MATCH (u:User {id: $id}) RETURN u.id AS uid UNION ALL MATCH (v:User) WHERE v.id < 10 RETURN v.id AS uid
```

</details>

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.069<br><sub>context: p90 0.130 · p95 0.138 · p99 0.157 · 4958 op/s · n/σ/CV 200/0.032/41.1% · total p50 0.791</sub> | 0.080<br><sub>context: p90 0.136 · p95 0.139 · p99 0.154 · 4206 op/s · n/σ/CV 200/0.031/36.9% · total p50 0.930</sub> | +15.4% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.076<br><sub>context: p90 0.142 · p95 0.146 · p99 0.155 · 33510 op/s · n/σ/CV 1587/0.033/39.4% · total p50 0.876</sub> | 0.079<br><sub>context: p90 0.145 · p95 0.153 · p99 0.178 · 27373 op/s · n/σ/CV 1574/0.035/39.2% · total p50 1.089</sub> | +3.7% (+0.003) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1322 MATCH (u:User {id: $id}) RETURN u.id AS uid UNION MATCH (v:User {id: $id}) RETURN v.id AS uid
```

</details>

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.039<br><sub>context: p90 0.073 · p95 0.079 · p99 0.087 · 6017 op/s · n/σ/CV 193/0.018/39.1% · total p50 0.633</sub> | 0.047<br><sub>context: p90 0.079 · p95 0.085 · p99 0.093 · 4729 op/s · n/σ/CV 199/0.017/32.5% · total p50 0.826</sub> | +20.9% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.042<br><sub>context: p90 0.075 · p95 0.079 · p99 0.093 · 38977 op/s · n/σ/CV 1597/0.021/42.4% · total p50 0.789</sub> | 0.043<br><sub>context: p90 0.076 · p95 0.081 · p99 0.098 · 34835 op/s · n/σ/CV 1591/0.021/41.9% · total p50 0.853</sub> | +2.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 4016 MATCH (n:User {id: $id}) UNWIND [n.id, n.id + 1, n.id + 2] AS x RETURN x
```

</details>

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.578<br><sub>context: p90 0.620 · p95 0.629 · p99 0.653 · 1265 op/s · n/σ/CV 198/0.030/5.2% · total p50 3.141</sub> | 0.586<br><sub>context: p90 0.637 · p95 0.656 · p99 0.676 · 1246 op/s · n/σ/CV 198/0.033/5.5% · total p50 3.196</sub> | +1.4% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.618<br><sub>context: p90 0.755 · p95 0.784 · p99 0.841 · 8001 op/s · n/σ/CV 1584/0.078/12.3% · total p50 3.864</sub> | 0.639<br><sub>context: p90 0.775 · p95 0.805 · p99 0.926 · 7897 op/s · n/σ/CV 1593/0.087/13.3% · total p50 3.895</sub> | +3.5% (+0.022) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5610 MATCH (a:User {id: $id}), (b:User) WHERE a.age = b.age RETURN b.id
```

</details>

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.591<br><sub>context: p90 0.635 · p95 0.649 · p99 0.695 · 1220 op/s · n/σ/CV 194/0.034/5.8% · total p50 3.226</sub> | 0.587<br><sub>context: p90 0.644 · p95 0.665 · p99 0.684 · 1265 op/s · n/σ/CV 198/0.036/6.0% · total p50 3.168</sub> | -0.7% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.603<br><sub>context: p90 0.702 · p95 0.736 · p99 0.818 · 8309 op/s · n/σ/CV 1587/0.061/9.8% · total p50 3.678</sub> | 0.624<br><sub>context: p90 0.768 · p95 0.809 · p99 0.867 · 7973 op/s · n/σ/CV 1583/0.082/12.8% · total p50 3.731</sub> | +3.4% (+0.021) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 8594 MATCH (a:User {id: $id}), (b:User) WHERE a.age = b.age RETURN count(b)
```

</details>

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.062<br><sub>context: p90 0.093 · p95 0.102 · p99 0.116 · 5095 op/s · n/σ/CV 197/0.018/27.4% · total p50 0.762</sub> | 0.074<br><sub>context: p90 0.102 · p95 0.111 · p99 0.123 · 4270 op/s · n/σ/CV 200/0.019/25.6% · total p50 0.913</sub> | +18.8% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.065<br><sub>context: p90 0.099 · p95 0.105 · p99 0.116 · 33349 op/s · n/σ/CV 1584/0.018/26.1% · total p50 0.858</sub> | 0.065<br><sub>context: p90 0.101 · p95 0.108 · p99 0.122 · 32638 op/s · n/σ/CV 1580/0.019/27.4% · total p50 0.915</sub> | +1.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 9631 MATCH (a:User {id: $id})-[*1..2]->(b:User) RETURN b.id
```

</details>

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.151<br><sub>context: p90 0.227 · p95 0.251 · p99 0.281 · 3114 op/s · n/σ/CV 200/0.049/31.2% · total p50 1.267</sub> | 0.168<br><sub>context: p90 0.262 · p95 0.304 · p99 0.324 · 2479 op/s · n/σ/CV 199/0.057/31.5% · total p50 1.565</sub> | +11.0% (+0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.162<br><sub>context: p90 0.255 · p95 0.285 · p99 0.340 · 21843 op/s · n/σ/CV 1597/0.061/36.2% · total p50 1.337</sub> | 0.166<br><sub>context: p90 0.263 · p95 0.306 · p99 0.361 · 20968 op/s · n/σ/CV 1593/0.066/37.4% · total p50 1.407</sub> | +2.3% (+0.004) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5948 min_capacity = 1 MATCH (s:User {id: $id})-[r:Friend*1..3]->(t:User) WHERE r.bench_capacity >= $min_capacity RETURN count(t)
```

</details>

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.022<br><sub>context: p90 0.027 · p95 0.029 · p99 0.037 · 8924 op/s · n/σ/CV 197/0.005/22.7% · total p50 0.441</sub> | 0.027<br><sub>context: p90 0.049 · p95 0.054 · p99 0.061 · 6332 op/s · n/σ/CV 200/0.012/37.4% · total p50 0.618</sub> | +23.7% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.025<br><sub>context: p90 0.034 · p95 0.038 · p99 0.045 · 40332 op/s · n/σ/CV 1516/0.006/21.4% · total p50 0.647</sub> | 0.025<br><sub>context: p90 0.033 · p95 0.037 · p99 0.046 · 39370 op/s · n/σ/CV 1547/0.006/22.8% · total p50 0.709</sub> | -1.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.idx.vector.queryNodes('User', 'embedding', 10, vecf32([0.1, 0.2, 0.3])) YIELD node, score RETURN id(node), score LIMIT 10
```

</details>

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.042<br><sub>context: p90 0.071 · p95 0.075 · p99 0.084 · 5835 op/s · n/σ/CV 200/0.015/33.3% · total p50 0.642</sub> | 0.037<br><sub>context: p90 0.063 · p95 0.072 · p99 0.076 · 7226 op/s · n/σ/CV 200/0.014/35.5% · total p50 0.541</sub> | -12.3% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.037<br><sub>context: p90 0.058 · p95 0.070 · p99 0.072 · 44906 op/s · n/σ/CV 1520/0.013/32.8% · total p50 0.563</sub> | 0.036<br><sub>context: p90 0.042 · p95 0.046 · p99 0.068 · 48666 op/s · n/σ/CV 1282/0.009/25.6% · total p50 0.491</sub> | -2.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1830 MATCH (n:User {id: $id}) RETURN n
```

</details>

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.041<br><sub>context: p90 0.073 · p95 0.075 · p99 0.084 · 6787 op/s · n/σ/CV 200/0.018/39.0% · total p50 0.576</sub> | 0.037<br><sub>context: p90 0.069 · p95 0.073 · p99 0.082 · 7381 op/s · n/σ/CV 200/0.017/39.7% · total p50 0.532</sub> | -8.5% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.038<br><sub>context: p90 0.072 · p95 0.074 · p99 0.081 · 44977 op/s · n/σ/CV 1596/0.018/41.9% · total p50 0.577</sub> | 0.038<br><sub>context: p90 0.072 · p95 0.075 · p99 0.085 · 44056 op/s · n/σ/CV 1577/0.018/42.5% · total p50 0.584</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 7516 MATCH (n:User {id: $id}) RETURN n
```

</details>

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.211<br><sub>context: p90 0.239 · p95 0.244 · p99 0.248 · 2964 op/s · n/σ/CV 199/0.017/7.7% · total p50 1.339</sub> | 0.212<br><sub>context: p90 0.237 · p95 0.243 · p99 0.276 · 2694 op/s · n/σ/CV 194/0.018/8.3% · total p50 1.468</sub> | +0.8% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.232<br><sub>context: p90 0.259 · p95 0.270 · p99 0.318 · 21015 op/s · n/σ/CV 1555/0.025/10.9% · total p50 1.420</sub> | 0.231<br><sub>context: p90 0.258 · p95 0.267 · p99 0.320 · 20749 op/s · n/σ/CV 1552/0.025/11.0% · total p50 1.423</sub> | -0.5% (-0.001) | 10% AND 0.5 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1473 MATCH (n {id: $id}) RETURN n
```

</details>

</details>
