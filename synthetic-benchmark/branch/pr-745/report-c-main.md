### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
| workload_hash | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**main vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · within-run n/σ/CV of `server_ms` · client-observed total p50) and `Δms` are informational, never part of the verdict. n = samples retained after severe-outlier removal (pooled across the C workers; `n (server m)` when only `m` carry a server time); σ = their **sample** standard deviation (n−1) of `server_ms` **within this run** — not run-to-run noise; CV = 100·σ/mean. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.241<br><sub>context: p90 1.289 · p95 1.310 · p99 1.335 · 750 op/s · n/σ/CV 196/0.038/3.0% · total p50 5.321</sub> | 0.602<br><sub>context: p90 0.626 · p95 0.634 · p99 0.644 · 1329 op/s · n/σ/CV 198/0.018/3.0% · total p50 2.998</sub> | -51.5% (-0.639) | 150% AND 2 ms | 🟢 |
| 8 | 1.316<br><sub>context: p90 1.681 · p95 1.929 · p99 2.087 · 4294 op/s · n/σ/CV 1419/0.204/14.7% · total p50 6.884</sub> | 0.615<br><sub>context: p90 0.668 · p95 0.691 · p99 0.733 · 9084 op/s · n/σ/CV 1554/0.034/5.5% · total p50 3.482</sub> | -53.3% (-0.701) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN avg(n.age) AS avg_age
```

</details>

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.930<br><sub>context: p90 2.001 · p95 2.023 · p99 2.047 · 487 op/s · n/σ/CV 192/0.042/2.1% · total p50 8.191</sub> | 0.629<br><sub>context: p90 0.650 · p95 0.660 · p99 0.675 · 1297 op/s · n/σ/CV 198/0.017/2.8% · total p50 3.064</sub> | -67.4% (-1.302) | 150% AND 2 ms | 🟢 |
| 8 | 2.040<br><sub>context: p90 2.458 · p95 2.675 · p99 3.021 · 3470 op/s · n/σ/CV 1524/0.255/12.0% · total p50 8.710</sub> | 0.653<br><sub>context: p90 0.700 · p95 0.723 · p99 0.771 · 8705 op/s · n/σ/CV 1570/0.034/5.2% · total p50 3.621</sub> | -68.0% (-1.387) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN count(DISTINCT n.age) AS distinct_ages
```

</details>

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.867<br><sub>context: p90 1.925 · p95 1.941 · p99 1.968 · 507 op/s · n/σ/CV 196/0.044/2.3% · total p50 7.864</sub> | 1.098<br><sub>context: p90 1.123 · p95 1.130 · p99 1.152 · 786 op/s · n/σ/CV 197/0.019/1.8% · total p50 5.071</sub> | -41.2% (-0.769) | 150% AND 2 ms | 🟢 |
| 8 | 2.030<br><sub>context: p90 2.692 · p95 2.934 · p99 3.298 · 3443 op/s · n/σ/CV 1555/0.357/16.6% · total p50 8.837</sub> | 1.740<br><sub>context: p90 2.436 · p95 2.616 · p99 2.810 · 3768 op/s · n/σ/CV 1592/0.458/25.4% · total p50 8.366</sub> | -14.3% (-0.290) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) WHERE n.age >= 18 RETURN avg(n.age) AS avg_age
```

</details>

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.489<br><sub>context: p90 2.549 · p95 2.590 · p99 2.636 · 386 op/s · n/σ/CV 197/0.053/2.1% · total p50 10.349</sub> | 1.194<br><sub>context: p90 1.220 · p95 1.225 · p99 1.239 · 724 op/s · n/σ/CV 196/0.020/1.7% · total p50 5.515</sub> | -52.0% (-1.295) | 150% AND 2 ms | 🟢 |
| 8 | 2.752<br><sub>context: p90 3.618 · p95 3.920 · p99 4.589 · 2576 op/s · n/σ/CV 1557/0.495/16.9% · total p50 11.912</sub> | 1.217<br><sub>context: p90 1.347 · p95 1.395 · p99 1.467 · 5105 op/s · n/σ/CV 1552/0.071/5.8% · total p50 6.074</sub> | -55.8% (-1.536) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN min(n.age) AS min_age, max(n.age) AS max_age, avg(n.age) AS avg_age
```

</details>

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.069<br><sub>context: p90 0.101 · p95 0.118 · p99 0.125 · 8625 op/s · n/σ/CV 198/0.018/25.6% · total p50 0.442</sub> | 0.017<br><sub>context: p90 0.026 · p95 0.029 · p99 0.038 · 10668 op/s · n/σ/CV 179/0.005/29.7% · total p50 0.327</sub> | -75.4% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.167<br><sub>context: p90 0.286 · p95 0.342 · p99 0.441 · 35143 op/s · n/σ/CV 1588/0.080/44.9% · total p50 0.849</sub> | 0.016<br><sub>context: p90 0.023 · p95 0.025 · p99 0.032 · 53159 op/s · n/σ/CV 1431/0.005/26.3% · total p50 0.431</sub> | -90.5% (-0.151) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.meta.stats() YIELD nodeCount RETURN nodeCount AS cnt
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.214<br><sub>context: p90 0.270 · p95 0.278 · p99 0.292 · 3698 op/s · n/σ/CV 199/0.037/17.2% · total p50 1.078</sub> | 0.074<br><sub>context: p90 0.103 · p95 0.110 · p99 0.143 · 3375 op/s · n/σ/CV 195/0.021/27.7% · total p50 1.122</sub> | -65.6% (-0.141) | 150% AND 2 ms | 🟢 |
| 8 | 0.308<br><sub>context: p90 0.454 · p95 0.516 · p99 0.622 · 18639 op/s · n/σ/CV 1592/0.092/28.0% · total p50 1.651</sub> | 0.056<br><sub>context: p90 0.091 · p95 0.099 · p99 0.124 · 32476 op/s · n/σ/CV 1570/0.021/34.3% · total p50 0.894</sub> | -81.8% (-0.252) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 4275 MATCH (s:User {id: $id})-->(n:User) RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.251<br><sub>context: p90 0.295 · p95 0.302 · p99 0.315 · 3230 op/s · n/σ/CV 199/0.033/13.2% · total p50 1.223</sub> | 0.065<br><sub>context: p90 0.094 · p95 0.098 · p99 0.109 · 4567 op/s · n/σ/CV 200/0.017/26.1% · total p50 0.859</sub> | -74.2% (-0.187) | 150% AND 2 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.487 · p95 0.538 · p99 0.640 · 17292 op/s · n/σ/CV 1581/0.095/26.7% · total p50 1.762</sub> | 0.059<br><sub>context: p90 0.092 · p95 0.098 · p99 0.143 · 32395 op/s · n/σ/CV 1584/0.021/32.7% · total p50 0.905</sub> | -82.6% (-0.277) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6967 MATCH (s:User {id: $id})-->(n:User)  WHERE n.age >= 18  RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.311 · p95 0.323 · p99 0.342 · 3064 op/s · n/σ/CV 199/0.035/13.2% · total p50 1.292</sub> | 0.130<br><sub>context: p90 0.157 · p95 0.163 · p99 0.173 · 3006 op/s · n/σ/CV 200/0.020/15.7% · total p50 1.318</sub> | -50.6% (-0.133) | 150% AND 2 ms | 🟢 |
| 8 | 0.361<br><sub>context: p90 0.538 · p95 0.616 · p99 0.757 · 15979 op/s · n/σ/CV 1589/0.110/28.2% · total p50 1.931</sub> | 0.103<br><sub>context: p90 0.136 · p95 0.143 · p99 0.158 · 27313 op/s · n/σ/CV 1581/0.020/19.4% · total p50 1.069</sub> | -71.4% (-0.257) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 7714 MATCH (s:User {id: $id})-->()-->(n:User) RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.361 · p95 0.368 · p99 0.390 · 2567 op/s · n/σ/CV 199/0.039/12.6% · total p50 1.545</sub> | 0.124<br><sub>context: p90 0.154 · p95 0.161 · p99 0.182 · 3167 op/s · n/σ/CV 199/0.024/19.4% · total p50 1.260</sub> | -61.0% (-0.194) | 150% AND 2 ms | 🟢 |
| 8 | 0.407<br><sub>context: p90 0.616 · p95 0.703 · p99 0.870 · 14182 op/s · n/σ/CV 1577/0.126/28.8% · total p50 2.160</sub> | 0.110<br><sub>context: p90 0.147 · p95 0.156 · p99 0.181 · 25053 op/s · n/σ/CV 1588/0.025/21.6% · total p50 1.173</sub> | -72.9% (-0.297) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5665 MATCH (s:User {id: $id})-->()-->(n:User)  WHERE n.age >= 18  RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.345<br><sub>context: p90 0.412 · p95 0.425 · p99 0.449 · 2305 op/s · n/σ/CV 195/0.045/13.0% · total p50 1.672</sub> | 0.139<br><sub>context: p90 0.179 · p95 0.189 · p99 0.212 · 3181 op/s · n/σ/CV 200/0.029/20.2% · total p50 1.252</sub> | -59.8% (-0.206) | 150% AND 2 ms | 🟢 |
| 8 | 0.460<br><sub>context: p90 0.656 · p95 0.744 · p99 0.910 · 11649 op/s · n/σ/CV 1549/0.130/26.8% · total p50 2.542</sub> | 0.151<br><sub>context: p90 0.199 · p95 0.214 · p99 0.244 · 15376 op/s · n/σ/CV 1600/0.034/22.4% · total p50 1.961</sub> | -67.2% (-0.309) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1612 MATCH (s:User {id: $id})-->()-->()-->(n:User) RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.399<br><sub>context: p90 0.466 · p95 0.497 · p99 0.551 · 2044 op/s · n/σ/CV 199/0.054/13.3% · total p50 1.916</sub> | 0.164<br><sub>context: p90 0.204 · p95 0.214 · p99 0.235 · 2652 op/s · n/σ/CV 199/0.028/17.2% · total p50 1.503</sub> | -59.0% (-0.235) | 150% AND 2 ms | 🟢 |
| 8 | 0.528<br><sub>context: p90 0.744 · p95 0.835 · p99 1.020 · 10885 op/s · n/σ/CV 1579/0.144/26.0% · total p50 2.827</sub> | 0.161<br><sub>context: p90 0.216 · p95 0.231 · p99 0.256 · 14297 op/s · n/σ/CV 1600/0.038/23.2% · total p50 2.138</sub> | -69.5% (-0.367) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 460 MATCH (s:User {id: $id})-->()-->()-->(n:User)  WHERE n.age >= 18  RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.603<br><sub>context: p90 0.763 · p95 0.794 · p99 0.913 · 1367 op/s · n/σ/CV 200/0.118/19.7% · total p50 2.877</sub> | 0.287<br><sub>context: p90 0.377 · p95 0.392 · p99 0.427 · 1651 op/s · n/σ/CV 198/0.062/21.2% · total p50 2.217</sub> | -52.4% (-0.316) | 150% AND 2 ms | 🟢 |
| 8 | 0.645<br><sub>context: p90 0.858 · p95 0.926 · p99 1.117 · 3691 op/s · n/σ/CV 1597/0.154/23.4% · total p50 8.123</sub> | 0.281<br><sub>context: p90 0.376 · p95 0.410 · p99 0.461 · 3934 op/s · n/σ/CV 1600/0.068/23.7% · total p50 7.733</sub> | -56.5% (-0.364) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 2820 MATCH (s:User {id: $id})-->()-->()-->()-->(n:User) RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.620<br><sub>context: p90 0.768 · p95 0.834 · p99 0.899 · 1336 op/s · n/σ/CV 200/0.112/17.8% · total p50 2.968</sub> | 0.299<br><sub>context: p90 0.379 · p95 0.399 · p99 0.457 · 1737 op/s · n/σ/CV 200/0.062/20.6% · total p50 2.242</sub> | -51.8% (-0.321) | 150% AND 2 ms | 🟢 |
| 8 | 0.709<br><sub>context: p90 0.971 · p95 1.051 · p99 1.199 · 3649 op/s · n/σ/CV 1599/0.175/24.0% · total p50 8.499</sub> | 0.298<br><sub>context: p90 0.390 · p95 0.420 · p99 0.463 · 3573 op/s · n/σ/CV 1600/0.066/21.8% · total p50 8.666</sub> | -58.0% (-0.412) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5473 MATCH (s:User {id: $id})-->()-->()-->()-->(n:User)  WHERE n.age >= 18 RETURN DISTINCT n.id
```

</details>

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.498<br><sub>context: p90 0.585 · p95 0.626 · p99 0.691 · 1718 op/s · n/σ/CV 198/0.069/13.8% · total p50 2.324</sub> | 0.334<br><sub>context: p90 0.516 · p95 0.553 · p99 0.630 · 1980 op/s · n/σ/CV 199/0.132/38.4% · total p50 1.993</sub> | -32.9% (-0.164) | 150% AND 2 ms | 🟢 |
| 8 | 0.584<br><sub>context: p90 0.770 · p95 0.838 · p99 1.005 · 10385 op/s · n/σ/CV 1584/0.126/20.9% · total p50 3.007</sub> | 0.419<br><sub>context: p90 0.647 · p95 0.715 · p99 0.886 · 11694 op/s · n/σ/CV 1599/0.171/40.3% · total p50 2.659</sub> | -28.2% (-0.165) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 326 to = 140 MATCH (s:User {id: $from}), (t:User {id: $to}) WITH s, t MATCH p = allShortestPaths((s)-[:Friend*1..4]->(t)) RETURN length(p)
```

</details>

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.321<br><sub>context: p90 0.361 · p95 0.372 · p99 0.393 · 2624 op/s · n/σ/CV 199/0.043/13.7% · total p50 1.539</sub> | 0.086<br><sub>context: p90 0.113 · p95 0.125 · p99 0.138 · 4614 op/s · n/σ/CV 200/0.021/24.3% · total p50 0.841</sub> | -73.2% (-0.235) | 150% AND 2 ms | 🟢 |
| 8 | 0.413<br><sub>context: p90 0.610 · p95 0.686 · p99 0.847 · 14522 op/s · n/σ/CV 1575/0.122/27.7% · total p50 2.102</sub> | 0.086<br><sub>context: p90 0.119 · p95 0.126 · p99 0.145 · 29403 op/s · n/σ/CV 1587/0.021/23.1% · total p50 1.041</sub> | -79.1% (-0.326) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1670 MATCH (a:User {id: $id}) CALL { WITH a MATCH (a)-->(b:User) RETURN b.id AS bid } RETURN bid
```

</details>

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.164<br><sub>context: p90 0.200 · p95 0.204 · p99 0.222 · 4625 op/s · n/σ/CV 200/0.031/19.2% · total p50 0.846</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 12681 op/s · n/σ/CV 190/0.000/19.5% · total p50 0.299</sub> | -98.7% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.242<br><sub>context: p90 0.399 · p95 0.479 · p99 0.596 · 24786 op/s · n/σ/CV 1588/0.101/37.9% · total p50 1.220</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 55349 op/s · n/σ/CV 1444/0.001/26.3% · total p50 0.431</sub> | -99.1% (-0.240) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH ()-[r:Friend]->() RETURN count(r) AS cnt
```

</details>

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.099<br><sub>context: p90 0.141 · p95 0.151 · p99 0.172 · 6663 op/s · n/σ/CV 200/0.026/24.6% · total p50 0.574</sub> | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 17157 op/s · n/σ/CV 185/0.000/21.1% · total p50 0.221</sub> | -98.4% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.193<br><sub>context: p90 0.307 · p95 0.371 · p99 0.452 · 31294 op/s · n/σ/CV 1590/0.079/38.7% · total p50 0.952</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 52547 op/s · n/σ/CV 1363/0.001/28.1% · total p50 0.401</sub> | -98.8% (-0.190) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (u:User) RETURN count(u) AS cnt
```

</details>

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.397<br><sub>context: p90 0.448 · p95 0.458 · p99 0.474 · 2092 op/s · n/σ/CV 199/0.039/10.0% · total p50 1.882</sub> | 0.072<br><sub>context: p90 0.100 · p95 0.113 · p99 0.121 · 3821 op/s · n/σ/CV 200/0.019/24.6% · total p50 1.022</sub> | -81.9% (-0.325) | 150% AND 2 ms | 🟢 |
| 8 | 0.500<br><sub>context: p90 0.769 · p95 0.907 · p99 1.088 · 12054 op/s · n/σ/CV 1582/0.164/30.0% · total p50 2.547</sub> | 0.078<br><sub>context: p90 0.108 · p95 0.117 · p99 0.129 · 24679 op/s · n/σ/CV 1595/0.019/24.0% · total p50 1.193</sub> | -84.4% (-0.422) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 643 MATCH p=(a:User {id: $id})-[r:Friend]->(b:User) RETURN labels(a), type(r), properties(a), nodes(p), relationships(p), length(p) LIMIT 1
```

</details>

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.694<br><sub>context: p90 0.890 · p95 0.936 · p99 0.972 · 1269 op/s · n/σ/CV 199/0.144/20.8% · total p50 3.155</sub> | 1.118<br><sub>context: p90 1.540 · p95 1.628 · p99 1.799 · 763 op/s · n/σ/CV 200/0.329/29.8% · total p50 5.217</sub> | +61.2% (+0.425) | 150% AND 2 ms | 🟢 |
| 8 | 0.780<br><sub>context: p90 1.080 · p95 1.195 · p99 1.495 · 7249 op/s · n/σ/CV 1593/0.221/27.5% · total p50 4.087</sub> | 1.829<br><sub>context: p90 2.752 · p95 2.971 · p99 3.434 · 3594 op/s · n/σ/CV 1600/0.655/35.2% · total p50 8.349</sub> | +134.4% (+1.049) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 7839 MATCH (s:User {id: $id})-[:Friend*5..5]->(t:User) RETURN count(t) AS cnt
```

</details>

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.285<br><sub>context: p90 3.826 · p95 4.152 · p99 4.331 · 388 op/s · n/σ/CV 200/0.969/39.2% · total p50 10.017</sub> | 4.551<br><sub>context: p90 7.387 · p95 7.845 · p99 8.034 · 197 op/s · n/σ/CV 200/1.834/38.0% · total p50 20.118</sub> | +99.2% (+2.266) | 150% AND 2 ms | 🟢 |
| 8 | 2.469<br><sub>context: p90 4.094 · p95 4.534 · p99 5.923 · 2835 op/s · n/σ/CV 1598/1.104/41.6% · total p50 10.742</sub> | 7.791<br><sub>context: p90 12.282 · p95 13.015 · p99 14.035 · 932 op/s · n/σ/CV 1600/2.858/35.0% · total p50 33.688</sub> | +215.5% (+5.322) | 150% AND 2 ms | 🔴 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1982 MATCH (s:User {id: $id})-[:Friend*6..6]->(t:User) RETURN count(t) AS cnt
```

</details>

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.220 · p95 0.229 · p99 0.263 · 4209 op/s · n/σ/CV 200/0.039/23.0% · total p50 0.944</sub> | 0.018<br><sub>context: p90 0.036 · p95 0.038 · p99 0.042 · 8891 op/s · n/σ/CV 199/0.008/36.4% · total p50 0.395</sub> | -89.6% (-0.157) | 150% AND 2 ms | 🟢 |
| 8 | 0.247<br><sub>context: p90 0.388 · p95 0.463 · p99 0.569 · 22081 op/s · n/σ/CV 1566/0.093/34.7% · total p50 1.352</sub> | 0.019<br><sub>context: p90 0.024 · p95 0.027 · p99 0.033 · 41962 op/s · n/σ/CV 1530/0.004/21.0% · total p50 0.614</sub> | -92.4% (-0.228) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.idx.fulltext.queryNodes('User', 'fixture_alice') YIELD node, score RETURN id(node), score LIMIT 10
```

</details>

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.165<br><sub>context: p90 0.202 · p95 0.218 · p99 0.235 · 4476 op/s · n/σ/CV 200/0.032/19.5% · total p50 0.874</sub> | 0.033<br><sub>context: p90 0.042 · p95 0.043 · p99 0.048 · 5912 op/s · n/σ/CV 198/0.009/27.6% · total p50 0.666</sub> | -80.0% (-0.132) | 150% AND 2 ms | 🟢 |
| 8 | 0.242<br><sub>context: p90 0.405 · p95 0.478 · p99 0.613 · 22846 op/s · n/σ/CV 1594/0.101/37.6% · total p50 1.348</sub> | 0.018<br><sub>context: p90 0.024 · p95 0.026 · p99 0.031 · 44822 op/s · n/σ/CV 1568/0.004/20.6% · total p50 0.590</sub> | -92.5% (-0.224) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.idx.fulltext.queryRelationships('Friend', 'fixture_blue') YIELD relationship, score RETURN id(relationship), score LIMIT 10
```

</details>

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.184<br><sub>context: p90 0.214 · p95 0.221 · p99 0.237 · 3959 op/s · n/σ/CV 199/0.029/16.3% · total p50 0.995</sub> | 0.011<br><sub>context: p90 0.016 · p95 0.017 · p99 0.019 · 7947 op/s · n/σ/CV 195/0.003/24.7% · total p50 0.477</sub> | -94.0% (-0.173) | 150% AND 2 ms | 🟢 |
| 8 | 0.198<br><sub>context: p90 0.260 · p95 0.285 · p99 0.359 · 16628 op/s · n/σ/CV 1589/0.048/24.1% · total p50 1.834</sub> | 0.011<br><sub>context: p90 0.014 · p95 0.017 · p99 0.021 · 17812 op/s · n/σ/CV 1596/0.003/27.0% · total p50 1.696</sub> | -94.6% (-0.187) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER end = 2049 start = 1949 MATCH (n) WHERE id(n) >= $start AND id(n) < $end RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.111<br><sub>context: p90 0.145 · p95 0.161 · p99 0.171 · 6609 op/s · n/σ/CV 199/0.025/22.5% · total p50 0.585</sub> | 0.006<br><sub>context: p90 0.012 · p95 0.013 · p99 0.014 · 9861 op/s · n/σ/CV 196/0.003/36.6% · total p50 0.389</sub> | -94.5% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 0.204<br><sub>context: p90 0.356 · p95 0.426 · p99 0.537 · 27472 op/s · n/σ/CV 1587/0.092/40.3% · total p50 1.094</sub> | 0.005<br><sub>context: p90 0.008 · p95 0.008 · p99 0.011 · 61569 op/s · n/σ/CV 1484/0.002/28.4% · total p50 0.418</sub> | -97.3% (-0.198) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 2122 MATCH (n) WHERE id(n) = $id RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.268<br><sub>context: p90 0.309 · p95 0.322 · p99 0.353 · 3097 op/s · n/σ/CV 199/0.038/14.3% · total p50 1.278</sub> | 0.141<br><sub>context: p90 0.180 · p95 0.187 · p99 0.216 · 3700 op/s · n/σ/CV 199/0.030/21.1% · total p50 1.040</sub> | -47.5% (-0.127) | 150% AND 2 ms | 🟢 |
| 8 | 0.323<br><sub>context: p90 0.478 · p95 0.542 · p99 0.672 · 17085 op/s · n/σ/CV 1557/0.094/27.2% · total p50 1.751</sub> | 0.150<br><sub>context: p90 0.195 · p95 0.205 · p99 0.234 · 26753 op/s · n/σ/CV 1587/0.034/22.9% · total p50 1.140</sub> | -53.7% (-0.173) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id1 = 5885 id2 = 9218 id3 = 9441 id4 = 2621 MATCH (u:User) WHERE u.id IN [$id1, $id2, $id3, $id4] RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.204<br><sub>context: p90 0.236 · p95 0.243 · p99 0.251 · 3904 op/s · n/σ/CV 199/0.031/15.6% · total p50 1.006</sub> | 0.069<br><sub>context: p90 0.096 · p95 0.100 · p99 0.108 · 5751 op/s · n/σ/CV 199/0.019/25.9% · total p50 0.693</sub> | -66.2% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.259<br><sub>context: p90 0.398 · p95 0.448 · p99 0.572 · 21757 op/s · n/σ/CV 1591/0.086/30.6% · total p50 1.391</sub> | 0.074<br><sub>context: p90 0.108 · p95 0.117 · p99 0.150 · 36719 op/s · n/σ/CV 1595/0.025/31.7% · total p50 0.819</sub> | -71.4% (-0.185) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id1 = 5236 id2 = 3248 MATCH (u:User) WHERE u.id = $id1 OR u.id = $id2 RETURN u.id
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.276<br><sub>context: p90 0.308 · p95 0.327 · p99 0.335 · 3001 op/s · n/σ/CV 198/0.030/11.1% · total p50 1.330</sub> | 0.126<br><sub>context: p90 0.162 · p95 0.169 · p99 0.189 · 2871 op/s · n/σ/CV 197/0.024/18.5% · total p50 1.365</sub> | -54.5% (-0.151) | 150% AND 2 ms | 🟢 |
| 8 | 0.360<br><sub>context: p90 0.539 · p95 0.611 · p99 0.731 · 15768 op/s · n/σ/CV 1575/0.107/27.7% · total p50 1.922</sub> | 0.102<br><sub>context: p90 0.136 · p95 0.145 · p99 0.167 · 26295 op/s · n/σ/CV 1583/0.022/21.2% · total p50 1.130</sub> | -71.7% (-0.258) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 692 MATCH (s:User {id: $id})-->()-->(n:User) RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.303 · p95 0.318 · p99 0.337 · 3043 op/s · n/σ/CV 194/0.043/17.7% · total p50 1.267</sub> | 0.107<br><sub>context: p90 0.144 · p95 0.153 · p99 0.168 · 3273 op/s · n/σ/CV 200/0.024/22.4% · total p50 1.201</sub> | -55.4% (-0.133) | 150% AND 2 ms | 🟢 |
| 8 | 0.364<br><sub>context: p90 0.537 · p95 0.606 · p99 0.719 · 14119 op/s · n/σ/CV 1576/0.107/27.5% · total p50 2.170</sub> | 0.104<br><sub>context: p90 0.144 · p95 0.155 · p99 0.175 · 14896 op/s · n/σ/CV 1599/0.026/24.8% · total p50 2.030</sub> | -71.5% (-0.260) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 6334 MATCH (s:User {id: $id})-->()-->(n:User) RETURN n
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.268<br><sub>context: p90 0.322 · p95 0.339 · p99 0.367 · 2941 op/s · n/σ/CV 199/0.044/16.4% · total p50 1.355</sub> | 0.117<br><sub>context: p90 0.150 · p95 0.162 · p99 0.169 · 3102 op/s · n/σ/CV 199/0.023/19.8% · total p50 1.274</sub> | -56.4% (-0.151) | 150% AND 2 ms | 🟢 |
| 8 | 0.378<br><sub>context: p90 0.519 · p95 0.585 · p99 0.680 · 14680 op/s · n/σ/CV 1574/0.094/23.8% · total p50 2.089</sub> | 0.110<br><sub>context: p90 0.149 · p95 0.157 · p99 0.175 · 16929 op/s · n/σ/CV 1600/0.026/23.4% · total p50 1.801</sub> | -71.0% (-0.268) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 9102 MATCH (s:User {id: $id})-->()-->(n:User) WHERE n.age >= 18 RETURN n
```

</details>

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.288<br><sub>context: p90 0.330 · p95 0.344 · p99 0.357 · 2795 op/s · n/σ/CV 198/0.031/10.7% · total p50 1.402</sub> | 0.104<br><sub>context: p90 0.137 · p95 0.142 · p99 0.168 · 3609 op/s · n/σ/CV 200/0.023/21.3% · total p50 1.080</sub> | -63.9% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.371<br><sub>context: p90 0.517 · p95 0.585 · p99 0.713 · 15877 op/s · n/σ/CV 1569/0.095/24.3% · total p50 1.937</sub> | 0.103<br><sub>context: p90 0.140 · p95 0.150 · p99 0.166 · 26027 op/s · n/σ/CV 1573/0.023/21.7% · total p50 1.127</sub> | -72.2% (-0.268) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 3235 MATCH (s:User {id: $id})-->()-->(n:User) WHERE n.age >= 18 RETURN n.id
```

</details>

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.327 · p95 0.341 · p99 0.372 · 2955 op/s · n/σ/CV 199/0.041/14.6% · total p50 1.331</sub> | 0.085<br><sub>context: p90 0.113 · p95 0.123 · p99 0.149 · 4659 op/s · n/σ/CV 200/0.023/26.7% · total p50 0.848</sub> | -69.7% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 0.394<br><sub>context: p90 0.560 · p95 0.619 · p99 0.759 · 15216 op/s · n/σ/CV 1586/0.106/25.4% · total p50 2.021</sub> | 0.081<br><sub>context: p90 0.116 · p95 0.124 · p99 0.148 · 32744 op/s · n/σ/CV 1588/0.023/26.4% · total p50 0.910</sub> | -79.5% (-0.313) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 8719 MATCH (a:User {id: $id}) OPTIONAL MATCH (a)-->(b:User) RETURN a.id, b.id
```

</details>

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 6.134<br><sub>context: p90 6.217 · p95 6.246 · p99 6.304 · 90 op/s · n/σ/CV 167/0.060/1.0% · total p50 42.901</sub> | 2.848<br><sub>context: p90 2.886 · p95 2.902 · p99 2.982 · 92 op/s · n/σ/CV 169/0.031/1.1% · total p50 42.356</sub> | -53.6% (-3.286) | 150% AND 2 ms | 🟢 |
| 8 | 6.597<br><sub>context: p90 8.526 · p95 8.912 · p99 9.348 · 149 op/s · n/σ/CV 1539/0.848/12.3% · total p50 206.667</sub> | 2.872<br><sub>context: p90 3.918 · p95 3.996 · p99 4.113 · 154 op/s · n/σ/CV 1600/0.444/14.3% · total p50 199.429</sub> | -56.5% (-3.726) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  MATCH (n:User) RETURN n.id, n.age ORDER BY n.age, n.id
```

</details>

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.391<br><sub>context: p90 0.441 · p95 0.449 · p99 0.475 · 2138 op/s · n/σ/CV 198/0.041/10.6% · total p50 1.863</sub> | 0.114<br><sub>context: p90 0.152 · p95 0.159 · p99 0.173 · 3355 op/s · n/σ/CV 200/0.027/23.4% · total p50 1.196</sub> | -70.8% (-0.277) | 150% AND 2 ms | 🟢 |
| 8 | 0.487<br><sub>context: p90 0.687 · p95 0.757 · p99 0.922 · 12355 op/s · n/σ/CV 1572/0.125/24.2% · total p50 2.478</sub> | 0.107<br><sub>context: p90 0.143 · p95 0.150 · p99 0.168 · 24554 op/s · n/σ/CV 1586/0.025/22.8% · total p50 1.183</sub> | -78.1% (-0.380) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 9234 MATCH (a:User {id: $id})-->(b:User)-->(c:User)-->(a) RETURN a.id, b.id, c.id
```

</details>

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.556<br><sub>context: p90 0.672 · p95 0.714 · p99 0.835 · 1261 op/s · n/σ/CV 195/0.093/16.5% · total p50 2.824</sub> | 0.261<br><sub>context: p90 0.341 · p95 0.359 · p99 0.413 · 1085 op/s · n/σ/CV 200/0.054/20.4% · total p50 3.476</sub> | -53.0% (-0.295) | 150% AND 2 ms | 🟢 |
| 8 | 0.575<br><sub>context: p90 0.741 · p95 0.805 · p99 0.917 · 2480 op/s · n/σ/CV 1598/0.123/21.0% · total p50 12.334</sub> | 0.271<br><sub>context: p90 0.358 · p95 0.396 · p99 0.479 · 2552 op/s · n/σ/CV 1596/0.066/24.1% · total p50 11.618</sub> | -52.9% (-0.304) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 3234 MATCH (a:User {id: $id})-->()-->()-->()-->(b:User) RETURN a.id, b.id
```

</details>

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.282<br><sub>context: p90 0.325 · p95 0.332 · p99 0.356 · 2926 op/s · n/σ/CV 197/0.043/15.5% · total p50 1.344</sub> | 0.143<br><sub>context: p90 0.174 · p95 0.185 · p99 0.215 · 2470 op/s · n/σ/CV 199/0.024/16.2% · total p50 1.600</sub> | -49.3% (-0.139) | 150% AND 2 ms | 🟢 |
| 8 | 0.378<br><sub>context: p90 0.553 · p95 0.625 · p99 0.740 · 15636 op/s · n/σ/CV 1587/0.109/27.0% · total p50 1.952</sub> | 0.113<br><sub>context: p90 0.152 · p95 0.164 · p99 0.195 · 23360 op/s · n/σ/CV 1585/0.028/23.9% · total p50 1.245</sub> | -70.0% (-0.264) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5820 MATCH (a:User {id: $id})-->()-->(b:User) RETURN a.id, b.id
```

</details>

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.148<br><sub>context: p90 2.706 · p95 2.770 · p99 2.889 · 458 op/s · n/σ/CV 199/0.505/24.3% · total p50 8.835</sub> | 0.118<br><sub>context: p90 0.164 · p95 0.175 · p99 0.222 · 3861 op/s · n/σ/CV 200/0.033/26.6% · total p50 1.030</sub> | -94.5% (-2.030) | 150% AND 2 ms | 🟢 |
| 8 | 2.211<br><sub>context: p90 2.790 · p95 2.943 · p99 3.931 · 3404 op/s · n/σ/CV 1581/0.549/25.0% · total p50 9.144</sub> | 0.125<br><sub>context: p90 0.173 · p95 0.190 · p99 0.239 · 25823 op/s · n/σ/CV 1594/0.035/26.8% · total p50 1.180</sub> | -94.4% (-2.087) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 1990 to = 6799 MATCH (s:User {id: $from}), (t:User {id: $to}) WITH shortestPath((s)-[*]->(t)) AS p RETURN length(p)
```

</details>

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.998<br><sub>context: p90 2.614 · p95 2.688 · p99 2.799 · 483 op/s · n/σ/CV 199/0.479/24.3% · total p50 8.222</sub> | 0.116<br><sub>context: p90 0.170 · p95 0.186 · p99 0.213 · 3737 op/s · n/σ/CV 199/0.031/25.7% · total p50 1.060</sub> | -94.2% (-1.882) | 150% AND 2 ms | 🟢 |
| 8 | 2.158<br><sub>context: p90 2.807 · p95 3.034 · p99 4.389 · 3521 op/s · n/σ/CV 1596/0.632/29.5% · total p50 8.692</sub> | 0.121<br><sub>context: p90 0.172 · p95 0.188 · p99 0.213 · 25761 op/s · n/σ/CV 1595/0.032/25.5% · total p50 1.149</sub> | -94.4% (-2.037) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER from = 6363 to = 141 MATCH (s:User {id: $from}), (t:User {id: $to}) WITH shortestPath((s)-[*]->(t)) AS p WHERE length(p) > 0 RETURN length(p)
```

</details>

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.159<br><sub>context: p90 0.193 · p95 0.203 · p99 0.225 · 4573 op/s · n/σ/CV 198/0.031/20.0% · total p50 0.871</sub> | 0.039<br><sub>context: p90 0.071 · p95 0.074 · p99 0.079 · 7273 op/s · n/σ/CV 200/0.018/40.4% · total p50 0.525</sub> | -75.2% (-0.119) | 150% AND 2 ms | 🟢 |
| 8 | 0.189<br><sub>context: p90 0.312 · p95 0.360 · p99 0.459 · 30435 op/s · n/σ/CV 1585/0.074/35.7% · total p50 1.013</sub> | 0.039<br><sub>context: p90 0.072 · p95 0.077 · p99 0.093 · 41198 op/s · n/σ/CV 1583/0.020/44.1% · total p50 0.646</sub> | -79.1% (-0.149) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 988 MATCH (n:User {id : $id}) RETURN n
```

</details>

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.210<br><sub>context: p90 0.249 · p95 0.269 · p99 0.289 · 3790 op/s · n/σ/CV 199/0.035/16.5% · total p50 1.046</sub> | 0.004<br><sub>context: p90 0.006 · p95 0.008 · p99 0.009 · 8902 op/s · n/σ/CV 187/0.001/33.0% · total p50 0.418</sub> | -98.2% (-0.207) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.277<br><sub>context: p90 0.400 · p95 0.451 · p99 0.548 · 19631 op/s · n/σ/CV 1551/0.075/25.8% · total p50 1.560</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 49130 op/s · n/σ/CV 1541/0.001/21.7% · total p50 0.492</sub> | -98.7% (-0.273) | 150% AND 2 ms | ⚠ N/A |

<details><summary>example query</summary>

```cypher
CYPHER  RETURN date('2024-01-01') AS d, localtime('12:30:00') AS t, duration('P2DT3H') AS dur, distance( point({latitude: 32.1, longitude: 34.8}), point({latitude: 32.2, longitude: 34.9}) ) AS dist
```

</details>

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.209<br><sub>context: p90 0.265 · p95 0.276 · p99 0.299 · 3652 op/s · n/σ/CV 199/0.037/17.6% · total p50 1.085</sub> | 0.048<br><sub>context: p90 0.077 · p95 0.084 · p99 0.095 · 5943 op/s · n/σ/CV 200/0.017/31.4% · total p50 0.658</sub> | -77.2% (-0.161) | 150% AND 2 ms | 🟢 |
| 8 | 0.326<br><sub>context: p90 0.493 · p95 0.560 · p99 0.693 · 18569 op/s · n/σ/CV 1572/0.106/30.6% · total p50 1.626</sub> | 0.050<br><sub>context: p90 0.082 · p95 0.084 · p99 0.093 · 36863 op/s · n/σ/CV 1585/0.016/30.3% · total p50 0.830</sub> | -84.8% (-0.276) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 8021 MATCH (u:User {id: $id}) RETURN u.id AS uid UNION ALL MATCH (v:User) WHERE v.id < 10 RETURN v.id AS uid
```

</details>

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.207<br><sub>context: p90 0.277 · p95 0.290 · p99 0.298 · 3663 op/s · n/σ/CV 199/0.041/18.7% · total p50 1.074</sub> | 0.069<br><sub>context: p90 0.130 · p95 0.138 · p99 0.157 · 4958 op/s · n/σ/CV 200/0.032/41.1% · total p50 0.791</sub> | -66.5% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.350<br><sub>context: p90 0.546 · p95 0.615 · p99 0.779 · 17237 op/s · n/σ/CV 1568/0.122/32.4% · total p50 1.743</sub> | 0.076<br><sub>context: p90 0.142 · p95 0.146 · p99 0.155 · 33510 op/s · n/σ/CV 1587/0.033/39.4% · total p50 0.876</sub> | -78.3% (-0.274) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1322 MATCH (u:User {id: $id}) RETURN u.id AS uid UNION MATCH (v:User {id: $id}) RETURN v.id AS uid
```

</details>

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.205<br><sub>context: p90 0.254 · p95 0.262 · p99 0.291 · 3753 op/s · n/σ/CV 199/0.039/18.9% · total p50 1.047</sub> | 0.039<br><sub>context: p90 0.073 · p95 0.079 · p99 0.087 · 6017 op/s · n/σ/CV 193/0.018/39.1% · total p50 0.633</sub> | -80.9% (-0.166) | 150% AND 2 ms | 🟢 |
| 8 | 0.286<br><sub>context: p90 0.450 · p95 0.517 · p99 0.640 · 20497 op/s · n/σ/CV 1577/0.101/32.3% · total p50 1.491</sub> | 0.042<br><sub>context: p90 0.075 · p95 0.079 · p99 0.093 · 38977 op/s · n/σ/CV 1597/0.021/42.4% · total p50 0.789</sub> | -85.3% (-0.244) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 4016 MATCH (n:User {id: $id}) UNWIND [n.id, n.id + 1, n.id + 2] AS x RETURN x
```

</details>

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.115<br><sub>context: p90 1.179 · p95 1.193 · p99 1.230 · 828 op/s · n/σ/CV 197/0.053/4.8% · total p50 4.812</sub> | 0.578<br><sub>context: p90 0.620 · p95 0.629 · p99 0.653 · 1265 op/s · n/σ/CV 198/0.030/5.2% · total p50 3.141</sub> | -48.2% (-0.537) | 150% AND 2 ms | 🟢 |
| 8 | 1.261<br><sub>context: p90 1.583 · p95 1.822 · p99 2.097 · 4569 op/s · n/σ/CV 1552/0.224/17.0% · total p50 6.680</sub> | 0.618<br><sub>context: p90 0.755 · p95 0.784 · p99 0.841 · 8001 op/s · n/σ/CV 1584/0.078/12.3% · total p50 3.864</sub> | -51.0% (-0.643) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5610 MATCH (a:User {id: $id}), (b:User) WHERE a.age = b.age RETURN b.id
```

</details>

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.117<br><sub>context: p90 1.183 · p95 1.203 · p99 1.216 · 840 op/s · n/σ/CV 197/0.058/5.2% · total p50 4.753</sub> | 0.591<br><sub>context: p90 0.635 · p95 0.649 · p99 0.695 · 1220 op/s · n/σ/CV 194/0.034/5.8% · total p50 3.226</sub> | -47.0% (-0.525) | 150% AND 2 ms | 🟢 |
| 8 | 1.250<br><sub>context: p90 1.619 · p95 1.850 · p99 2.120 · 5525 op/s · n/σ/CV 1543/0.236/18.0% · total p50 5.496</sub> | 0.603<br><sub>context: p90 0.702 · p95 0.736 · p99 0.818 · 8309 op/s · n/σ/CV 1587/0.061/9.8% · total p50 3.678</sub> | -51.7% (-0.647) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 8594 MATCH (a:User {id: $id}), (b:User) WHERE a.age = b.age RETURN count(b)
```

</details>

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.214<br><sub>context: p90 0.261 · p95 0.273 · p99 0.284 · 3706 op/s · n/σ/CV 199/0.034/16.0% · total p50 1.077</sub> | 0.062<br><sub>context: p90 0.093 · p95 0.102 · p99 0.116 · 5095 op/s · n/σ/CV 197/0.018/27.4% · total p50 0.762</sub> | -71.0% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.303<br><sub>context: p90 0.466 · p95 0.536 · p99 0.620 · 18822 op/s · n/σ/CV 1587/0.093/28.5% · total p50 1.629</sub> | 0.065<br><sub>context: p90 0.099 · p95 0.105 · p99 0.116 · 33349 op/s · n/σ/CV 1584/0.018/26.1% · total p50 0.858</sub> | -78.7% (-0.238) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 9631 MATCH (a:User {id: $id})-[*1..2]->(b:User) RETURN b.id
```

</details>

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.327<br><sub>context: p90 0.401 · p95 0.420 · p99 0.444 · 2592 op/s · n/σ/CV 199/0.052/16.0% · total p50 1.506</sub> | 0.151<br><sub>context: p90 0.227 · p95 0.251 · p99 0.281 · 3114 op/s · n/σ/CV 200/0.049/31.2% · total p50 1.267</sub> | -53.7% (-0.175) | 150% AND 2 ms | 🟢 |
| 8 | 0.426<br><sub>context: p90 0.603 · p95 0.666 · p99 0.791 · 13633 op/s · n/σ/CV 1579/0.113/25.4% · total p50 2.253</sub> | 0.162<br><sub>context: p90 0.255 · p95 0.285 · p99 0.340 · 21843 op/s · n/σ/CV 1597/0.061/36.2% · total p50 1.337</sub> | -62.0% (-0.264) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 5948 min_capacity = 1 MATCH (s:User {id: $id})-[r:Friend*1..3]->(t:User) WHERE r.bench_capacity >= $min_capacity RETURN count(t)
```

</details>

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.238 · p95 0.250 · p99 0.270 · 3949 op/s · n/σ/CV 198/0.037/19.5% · total p50 1.009</sub> | 0.022<br><sub>context: p90 0.027 · p95 0.029 · p99 0.037 · 8924 op/s · n/σ/CV 197/0.005/22.7% · total p50 0.441</sub> | -88.6% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 0.280<br><sub>context: p90 0.440 · p95 0.513 · p99 0.621 · 19829 op/s · n/σ/CV 1585/0.098/32.4% · total p50 1.513</sub> | 0.025<br><sub>context: p90 0.034 · p95 0.038 · p99 0.045 · 40332 op/s · n/σ/CV 1516/0.006/21.4% · total p50 0.647</sub> | -91.0% (-0.254) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER  CALL db.idx.vector.queryNodes('User', 'embedding', 10, vecf32([0.1, 0.2, 0.3])) YIELD node, score RETURN id(node), score LIMIT 10
```

</details>

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.120<br><sub>context: p90 0.147 · p95 0.167 · p99 0.180 · 6147 op/s · n/σ/CV 200/0.023/19.6% · total p50 0.633</sub> | 0.042<br><sub>context: p90 0.071 · p95 0.075 · p99 0.084 · 5835 op/s · n/σ/CV 200/0.015/33.3% · total p50 0.642</sub> | -64.7% (-0.077) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.301 · p95 0.346 · p99 0.432 · 29579 op/s · n/σ/CV 1582/0.071/34.5% · total p50 1.029</sub> | 0.037<br><sub>context: p90 0.058 · p95 0.070 · p99 0.072 · 44906 op/s · n/σ/CV 1520/0.013/32.8% · total p50 0.563</sub> | -80.1% (-0.149) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1830 MATCH (n:User {id: $id}) RETURN n
```

</details>

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.165 · p95 0.170 · p99 0.188 · 5517 op/s · n/σ/CV 199/0.026/20.6% · total p50 0.717</sub> | 0.041<br><sub>context: p90 0.073 · p95 0.075 · p99 0.084 · 6787 op/s · n/σ/CV 200/0.018/39.0% · total p50 0.576</sub> | -66.5% (-0.081) | 150% AND 2 ms | 🟢 |
| 8 | 0.180<br><sub>context: p90 0.304 · p95 0.354 · p99 0.431 · 30015 op/s · n/σ/CV 1581/0.072/35.9% · total p50 1.011</sub> | 0.038<br><sub>context: p90 0.072 · p95 0.074 · p99 0.081 · 44977 op/s · n/σ/CV 1596/0.018/41.9% · total p50 0.577</sub> | -79.0% (-0.142) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 7516 MATCH (n:User {id: $id}) RETURN n
```

</details>

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.844<br><sub>context: p90 0.883 · p95 0.893 · p99 0.910 · 1079 op/s · n/σ/CV 195/0.028/3.3% · total p50 3.697</sub> | 0.211<br><sub>context: p90 0.239 · p95 0.244 · p99 0.248 · 2964 op/s · n/σ/CV 199/0.017/7.7% · total p50 1.339</sub> | -75.0% (-0.633) | 150% AND 2 ms | 🟢 |
| 8 | 0.900<br><sub>context: p90 1.310 · p95 1.527 · p99 1.652 · 6031 op/s · n/σ/CV 1460/0.207/21.3% · total p50 4.992</sub> | 0.232<br><sub>context: p90 0.259 · p95 0.270 · p99 0.318 · 21015 op/s · n/σ/CV 1555/0.025/10.9% · total p50 1.420</sub> | -74.2% (-0.667) | 150% AND 2 ms | 🟢 |

<details><summary>example query</summary>

```cypher
CYPHER id = 1473 MATCH (n {id: $id}) RETURN n
```

</details>

</details>
