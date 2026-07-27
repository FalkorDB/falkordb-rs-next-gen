### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
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

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.582<br><sub>context: p90 0.615 · p95 0.621 · p99 0.638 · 1457 op/s · total p50 2.727</sub> | 0.584<br><sub>context: p90 0.614 · p95 0.625 · p99 0.634 · 1456 op/s · total p50 2.739</sub> | +0.4% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.615<br><sub>context: p90 0.662 · p95 0.685 · p99 0.720 · 9003 op/s · total p50 3.507</sub> | 0.621<br><sub>context: p90 0.688 · p95 0.717 · p99 0.751 · 9268 op/s · total p50 3.262</sub> | +0.9% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.607<br><sub>context: p90 0.630 · p95 0.637 · p99 0.649 · 1410 op/s · total p50 2.804</sub> | 0.626<br><sub>context: p90 0.659 · p95 0.669 · p99 0.685 · 1334 op/s · total p50 2.983</sub> | +3.2% (+0.019) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.655<br><sub>context: p90 0.697 · p95 0.720 · p99 0.755 · 8706 op/s · total p50 3.615</sub> | 0.656<br><sub>context: p90 0.707 · p95 0.729 · p99 0.775 · 8696 op/s · total p50 3.581</sub> | +0.2% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.091<br><sub>context: p90 1.120 · p95 1.131 · p99 1.157 · 807 op/s · total p50 4.969</sub> | 1.091<br><sub>context: p90 1.122 · p95 1.133 · p99 1.156 · 805 op/s · total p50 4.912</sub> | +0.0% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.820<br><sub>context: p90 2.468 · p95 2.655 · p99 3.062 · 3857 op/s · total p50 8.084</sub> | 1.703<br><sub>context: p90 2.448 · p95 2.603 · p99 2.953 · 3973 op/s · total p50 7.853</sub> | -6.4% (-0.117) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.175<br><sub>context: p90 1.210 · p95 1.229 · p99 1.242 · 758 op/s · total p50 5.257</sub> | 1.175<br><sub>context: p90 1.207 · p95 1.218 · p99 1.236 · 758 op/s · total p50 5.241</sub> | +0.0% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.213<br><sub>context: p90 1.346 · p95 1.392 · p99 1.453 · 5148 op/s · total p50 5.908</sub> | 1.210<br><sub>context: p90 1.309 · p95 1.353 · p99 1.402 · 5147 op/s · total p50 5.832</sub> | -0.2% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.014<br><sub>context: p90 0.018 · p95 0.019 · p99 0.026 · 13251 op/s · total p50 0.292</sub> | 0.014<br><sub>context: p90 0.018 · p95 0.020 · p99 0.022 · 14290 op/s · total p50 0.263</sub> | +1.4% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.015<br><sub>context: p90 0.021 · p95 0.024 · p99 0.033 · 48635 op/s · total p50 0.402</sub> | 0.016<br><sub>context: p90 0.023 · p95 0.025 · p99 0.032 · 50648 op/s · total p50 0.431</sub> | +12.0% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.047<br><sub>context: p90 0.075 · p95 0.083 · p99 0.092 · 6969 op/s · total p50 0.562</sub> | 0.046<br><sub>context: p90 0.074 · p95 0.084 · p99 0.094 · 6763 op/s · total p50 0.580</sub> | -3.3% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.053<br><sub>context: p90 0.087 · p95 0.091 · p99 0.100 · 39016 op/s · total p50 0.766</sub> | 0.054<br><sub>context: p90 0.088 · p95 0.092 · p99 0.101 · 36197 op/s · total p50 0.829</sub> | +1.7% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.052<br><sub>context: p90 0.079 · p95 0.089 · p99 0.102 · 5294 op/s · total p50 0.747</sub> | 0.054<br><sub>context: p90 0.087 · p95 0.089 · p99 0.097 · 5608 op/s · total p50 0.680</sub> | +2.9% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.058<br><sub>context: p90 0.092 · p95 0.096 · p99 0.117 · 32645 op/s · total p50 0.922</sub> | 0.057<br><sub>context: p90 0.091 · p95 0.095 · p99 0.108 · 35085 op/s · total p50 0.868</sub> | -1.0% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.096<br><sub>context: p90 0.127 · p95 0.134 · p99 0.146 · 4227 op/s · total p50 0.929</sub> | 0.095<br><sub>context: p90 0.124 · p95 0.131 · p99 0.156 · 4451 op/s · total p50 0.880</sub> | -1.1% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.103<br><sub>context: p90 0.135 · p95 0.144 · p99 0.162 · 27805 op/s · total p50 1.057</sub> | 0.103<br><sub>context: p90 0.138 · p95 0.150 · p99 0.167 · 28149 op/s · total p50 1.069</sub> | +0.0% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.137 · p95 0.153 · p99 0.165 · 3841 op/s · total p50 1.013</sub> | 0.104<br><sub>context: p90 0.136 · p95 0.147 · p99 0.163 · 3836 op/s · total p50 1.051</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.112<br><sub>context: p90 0.149 · p95 0.160 · p99 0.185 · 24974 op/s · total p50 1.186</sub> | 0.112<br><sub>context: p90 0.150 · p95 0.160 · p99 0.178 · 23647 op/s · total p50 1.263</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.137<br><sub>context: p90 0.189 · p95 0.206 · p99 0.221 · 3140 op/s · total p50 1.252</sub> | 0.136<br><sub>context: p90 0.182 · p95 0.194 · p99 0.222 · 3271 op/s · total p50 1.200</sub> | -0.9% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.150<br><sub>context: p90 0.200 · p95 0.214 · p99 0.239 · 14951 op/s · total p50 1.975</sub> | 0.149<br><sub>context: p90 0.200 · p95 0.214 · p99 0.242 · 14983 op/s · total p50 2.035</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.151<br><sub>context: p90 0.199 · p95 0.205 · p99 0.217 · 2933 op/s · total p50 1.350</sub> | 0.149<br><sub>context: p90 0.184 · p95 0.202 · p99 0.219 · 2986 op/s · total p50 1.329</sub> | -0.7% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.160<br><sub>context: p90 0.211 · p95 0.227 · p99 0.253 · 13160 op/s · total p50 2.298</sub> | 0.159<br><sub>context: p90 0.218 · p95 0.233 · p99 0.260 · 13591 op/s · total p50 2.215</sub> | -0.5% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.343 · p95 0.360 · p99 0.405 · 1822 op/s · total p50 1.971</sub> | 0.241<br><sub>context: p90 0.323 · p95 0.338 · p99 0.385 · 1851 op/s · total p50 1.935</sub> | -7.7% (-0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.272<br><sub>context: p90 0.376 · p95 0.412 · p99 0.454 · 4096 op/s · total p50 7.514</sub> | 0.272<br><sub>context: p90 0.376 · p95 0.411 · p99 0.452 · 3971 op/s · total p50 7.548</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.267<br><sub>context: p90 0.349 · p95 0.369 · p99 0.414 · 1872 op/s · total p50 2.073</sub> | 0.276<br><sub>context: p90 0.361 · p95 0.382 · p99 0.442 · 1734 op/s · total p50 2.162</sub> | +3.5% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.291<br><sub>context: p90 0.387 · p95 0.415 · p99 0.461 · 3872 op/s · total p50 7.885</sub> | 0.300<br><sub>context: p90 0.390 · p95 0.420 · p99 0.470 · 3831 op/s · total p50 7.872</sub> | +3.1% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.510 · p95 0.540 · p99 0.625 · 1934 op/s · total p50 2.040</sub> | 0.325<br><sub>context: p90 0.519 · p95 0.538 · p99 0.629 · 1997 op/s · total p50 1.963</sub> | -1.5% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.418<br><sub>context: p90 0.639 · p95 0.712 · p99 0.868 · 11825 op/s · total p50 2.582</sub> | 0.414<br><sub>context: p90 0.640 · p95 0.715 · p99 0.843 · 11165 op/s · total p50 2.749</sub> | -0.9% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.088<br><sub>context: p90 0.119 · p95 0.133 · p99 0.143 · 4291 op/s · total p50 0.922</sub> | 0.078<br><sub>context: p90 0.107 · p95 0.112 · p99 0.127 · 4962 op/s · total p50 0.796</sub> | -11.5% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.089<br><sub>context: p90 0.122 · p95 0.130 · p99 0.155 · 28701 op/s · total p50 1.045</sub> | 0.085<br><sub>context: p90 0.119 · p95 0.125 · p99 0.146 · 30254 op/s · total p50 0.991</sub> | -4.1% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 13092 op/s · total p50 0.294</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 12331 op/s · total p50 0.309</sub> | +18.0% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 47494 op/s · total p50 0.438</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 54043 op/s · total p50 0.434</sub> | -5.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.002 · p95 0.003 · p99 0.003 · 15286 op/s · total p50 0.245</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 13709 op/s · total p50 0.274</sub> | +5.6% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 56338 op/s · total p50 0.407</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 57158 op/s · total p50 0.391</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.074<br><sub>context: p90 0.099 · p95 0.107 · p99 0.116 · 3945 op/s · total p50 0.985</sub> | 0.067<br><sub>context: p90 0.095 · p95 0.100 · p99 0.122 · 4267 op/s · total p50 0.905</sub> | -9.7% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.080<br><sub>context: p90 0.110 · p95 0.120 · p99 0.134 · 24992 op/s · total p50 1.178</sub> | 0.078<br><sub>context: p90 0.110 · p95 0.118 · p99 0.132 · 26036 op/s · total p50 1.141</sub> | -2.0% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.063<br><sub>context: p90 1.529 · p95 1.593 · p99 1.775 · 808 op/s · total p50 4.923</sub> | 1.091<br><sub>context: p90 1.530 · p95 1.597 · p99 1.683 · 807 op/s · total p50 4.949</sub> | +2.6% (+0.028) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.790<br><sub>context: p90 2.676 · p95 2.874 · p99 3.226 · 3598 op/s · total p50 8.356</sub> | 1.761<br><sub>context: p90 2.634 · p95 2.827 · p99 3.140 · 3694 op/s · total p50 8.098</sub> | -1.6% (-0.029) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.401<br><sub>context: p90 7.217 · p95 7.610 · p99 7.888 · 204 op/s · total p50 19.418</sub> | 4.505<br><sub>context: p90 7.279 · p95 7.619 · p99 7.922 · 203 op/s · total p50 19.545</sub> | +2.4% (+0.104) | 10% AND 0.5 ms | 🟢 |
| 8 | 7.393<br><sub>context: p90 11.576 · p95 12.239 · p99 13.483 · 987 op/s · total p50 32.073</sub> | 7.542<br><sub>context: p90 11.785 · p95 12.486 · p99 13.602 · 966 op/s · total p50 32.702</sub> | +2.0% (+0.149) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.017<br><sub>context: p90 0.021 · p95 0.023 · p99 0.026 · 9328 op/s · total p50 0.413</sub> | 0.014<br><sub>context: p90 0.019 · p95 0.020 · p99 0.024 · 10895 op/s · total p50 0.349</sub> | -19.0% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.019<br><sub>context: p90 0.024 · p95 0.026 · p99 0.030 · 44815 op/s · total p50 0.620</sub> | 0.019<br><sub>context: p90 0.023 · p95 0.026 · p99 0.031 · 44353 op/s · total p50 0.614</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.013<br><sub>context: p90 0.019 · p95 0.020 · p99 0.022 · 11636 op/s · total p50 0.330</sub> | 0.017<br><sub>context: p90 0.019 · p95 0.022 · p99 0.026 · 10626 op/s · total p50 0.362</sub> | +25.1% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.018<br><sub>context: p90 0.022 · p95 0.025 · p99 0.029 · 46112 op/s · total p50 0.551</sub> | 0.018<br><sub>context: p90 0.022 · p95 0.023 · p99 0.027 · 50417 op/s · total p50 0.467</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.017 · 8203 op/s · total p50 0.473</sub> | 0.010<br><sub>context: p90 0.012 · p95 0.013 · p99 0.017 · 7873 op/s · total p50 0.487</sub> | -4.4% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.011<br><sub>context: p90 0.014 · p95 0.016 · p99 0.020 · 18126 op/s · total p50 1.669</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.016 · p99 0.020 · 17983 op/s · total p50 1.689</sub> | -1.7% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.005<br><sub>context: p90 0.007 · p95 0.008 · p99 0.009 · 12157 op/s · total p50 0.307</sub> | 0.004<br><sub>context: p90 0.006 · p95 0.007 · p99 0.008 · 14829 op/s · total p50 0.260</sub> | -5.1% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.011 · 48582 op/s · total p50 0.447</sub> | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.012 · 56154 op/s · total p50 0.444</sub> | +1.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.139<br><sub>context: p90 0.176 · p95 0.182 · p99 0.205 · 3747 op/s · total p50 1.059</sub> | 0.143<br><sub>context: p90 0.177 · p95 0.188 · p99 0.210 · 3737 op/s · total p50 1.059</sub> | +2.7% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.152<br><sub>context: p90 0.195 · p95 0.210 · p99 0.236 · 26182 op/s · total p50 1.129</sub> | 0.153<br><sub>context: p90 0.196 · p95 0.211 · p99 0.240 · 25948 op/s · total p50 1.156</sub> | +1.1% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.068<br><sub>context: p90 0.095 · p95 0.102 · p99 0.111 · 5347 op/s · total p50 0.727</sub> | 0.065<br><sub>context: p90 0.093 · p95 0.098 · p99 0.104 · 6314 op/s · total p50 0.618</sub> | -4.4% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.076<br><sub>context: p90 0.110 · p95 0.126 · p99 0.162 · 36250 op/s · total p50 0.830</sub> | 0.075<br><sub>context: p90 0.109 · p95 0.121 · p99 0.150 · 35263 op/s · total p50 0.819</sub> | -1.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.086<br><sub>context: p90 0.121 · p95 0.126 · p99 0.151 · 4693 op/s · total p50 0.832</sub> | 0.098<br><sub>context: p90 0.128 · p95 0.139 · p99 0.152 · 4364 op/s · total p50 0.891</sub> | +13.5% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.104<br><sub>context: p90 0.140 · p95 0.149 · p99 0.167 · 25910 op/s · total p50 1.173</sub> | 0.104<br><sub>context: p90 0.140 · p95 0.149 · p99 0.170 · 26220 op/s · total p50 1.117</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.100<br><sub>context: p90 0.136 · p95 0.142 · p99 0.162 · 3618 op/s · total p50 1.079</sub> | 0.101<br><sub>context: p90 0.136 · p95 0.147 · p99 0.176 · 3567 op/s · total p50 1.116</sub> | +1.2% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.103<br><sub>context: p90 0.142 · p95 0.154 · p99 0.177 · 14955 op/s · total p50 2.027</sub> | 0.104<br><sub>context: p90 0.142 · p95 0.153 · p99 0.172 · 14358 op/s · total p50 2.078</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.101<br><sub>context: p90 0.130 · p95 0.141 · p99 0.156 · 3562 op/s · total p50 1.098</sub> | 0.109<br><sub>context: p90 0.134 · p95 0.152 · p99 0.162 · 3546 op/s · total p50 1.094</sub> | +7.5% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.109<br><sub>context: p90 0.146 · p95 0.158 · p99 0.177 · 16312 op/s · total p50 1.881</sub> | 0.111<br><sub>context: p90 0.149 · p95 0.160 · p99 0.182 · 16488 op/s · total p50 1.869</sub> | +1.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.093<br><sub>context: p90 0.123 · p95 0.130 · p99 0.155 · 4128 op/s · total p50 0.952</sub> | 0.088<br><sub>context: p90 0.126 · p95 0.137 · p99 0.157 · 4259 op/s · total p50 0.901</sub> | -4.8% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.106<br><sub>context: p90 0.142 · p95 0.152 · p99 0.173 · 24390 op/s · total p50 1.226</sub> | 0.105<br><sub>context: p90 0.140 · p95 0.150 · p99 0.166 · 26173 op/s · total p50 1.140</sub> | -1.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.085<br><sub>context: p90 0.118 · p95 0.125 · p99 0.140 · 4708 op/s · total p50 0.828</sub> | 0.074<br><sub>context: p90 0.104 · p95 0.108 · p99 0.133 · 5306 op/s · total p50 0.741</sub> | -12.8% (-0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.084<br><sub>context: p90 0.120 · p95 0.130 · p99 0.157 · 30584 op/s · total p50 0.966</sub> | 0.085<br><sub>context: p90 0.122 · p95 0.132 · p99 0.156 · 30667 op/s · total p50 0.977</sub> | +0.7% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.744<br><sub>context: p90 2.790 · p95 2.810 · p99 2.850 · 92 op/s · total p50 42.427</sub> | 2.753<br><sub>context: p90 2.798 · p95 2.819 · p99 2.838 · 93 op/s · total p50 42.695</sub> | +0.3% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.788<br><sub>context: p90 3.875 · p95 3.940 · p99 4.039 · 155 op/s · total p50 197.395</sub> | 2.758<br><sub>context: p90 2.831 · p95 2.859 · p99 2.920 · 158 op/s · total p50 197.354</sub> | -1.0% (-0.029) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.115<br><sub>context: p90 0.146 · p95 0.153 · p99 0.166 · 3355 op/s · total p50 1.189</sub> | 0.104<br><sub>context: p90 0.131 · p95 0.138 · p99 0.147 · 3870 op/s · total p50 1.014</sub> | -9.4% (-0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.111<br><sub>context: p90 0.146 · p95 0.154 · p99 0.171 · 24345 op/s · total p50 1.213</sub> | 0.109<br><sub>context: p90 0.144 · p95 0.153 · p99 0.170 · 24288 op/s · total p50 1.209</sub> | -1.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.244<br><sub>context: p90 0.344 · p95 0.366 · p99 0.421 · 1123 op/s · total p50 3.391</sub> | 0.249<br><sub>context: p90 0.327 · p95 0.352 · p99 0.408 · 1080 op/s · total p50 3.800</sub> | +2.0% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.262<br><sub>context: p90 0.352 · p95 0.382 · p99 0.450 · 2570 op/s · total p50 11.686</sub> | 0.259<br><sub>context: p90 0.351 · p95 0.382 · p99 0.436 · 2620 op/s · total p50 11.470</sub> | -1.2% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.101<br><sub>context: p90 0.134 · p95 0.149 · p99 0.169 · 4102 op/s · total p50 0.946</sub> | 0.103<br><sub>context: p90 0.137 · p95 0.145 · p99 0.157 · 4051 op/s · total p50 0.958</sub> | +1.1% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.111<br><sub>context: p90 0.148 · p95 0.158 · p99 0.180 · 24209 op/s · total p50 1.168</sub> | 0.111<br><sub>context: p90 0.148 · p95 0.157 · p99 0.185 · 25419 op/s · total p50 1.137</sub> | +0.7% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.113<br><sub>context: p90 0.152 · p95 0.162 · p99 0.222 · 4231 op/s · total p50 0.923</sub> | 0.106<br><sub>context: p90 0.149 · p95 0.157 · p99 0.202 · 4372 op/s · total p50 0.891</sub> | -6.2% (-0.007) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.123<br><sub>context: p90 0.173 · p95 0.194 · p99 0.247 · 26904 op/s · total p50 1.081</sub> | 0.126<br><sub>context: p90 0.175 · p95 0.197 · p99 0.240 · 24859 op/s · total p50 1.240</sub> | +2.4% (+0.003) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.108<br><sub>context: p90 0.156 · p95 0.177 · p99 0.198 · 3909 op/s · total p50 1.020</sub> | 0.109<br><sub>context: p90 0.154 · p95 0.173 · p99 0.190 · 4087 op/s · total p50 0.949</sub> | +1.0% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.123<br><sub>context: p90 0.173 · p95 0.190 · p99 0.217 · 25495 op/s · total p50 1.178</sub> | 0.121<br><sub>context: p90 0.171 · p95 0.191 · p99 0.215 · 25601 op/s · total p50 1.135</sub> | -1.3% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.034<br><sub>context: p90 0.063 · p95 0.070 · p99 0.076 · 8552 op/s · total p50 0.446</sub> | 0.035<br><sub>context: p90 0.070 · p95 0.073 · p99 0.077 · 8596 op/s · total p50 0.441</sub> | +2.8% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.039<br><sub>context: p90 0.072 · p95 0.075 · p99 0.083 · 43502 op/s · total p50 0.611</sub> | 0.038<br><sub>context: p90 0.072 · p95 0.074 · p99 0.084 · 46433 op/s · total p50 0.539</sub> | -2.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 10911 op/s · total p50 0.355</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.005 · p99 0.008 · 10472 op/s · total p50 0.374</sub> | -1.7% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 48104 op/s · total p50 0.481</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.007 · 48848 op/s · total p50 0.479</sub> | -0.9% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.047<br><sub>context: p90 0.078 · p95 0.082 · p99 0.088 · 6049 op/s · total p50 0.647</sub> | 0.045<br><sub>context: p90 0.072 · p95 0.077 · p99 0.085 · 6284 op/s · total p50 0.617</sub> | -3.9% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.050<br><sub>context: p90 0.083 · p95 0.086 · p99 0.097 · 33201 op/s · total p50 0.909</sub> | 0.050<br><sub>context: p90 0.082 · p95 0.086 · p99 0.095 · 36673 op/s · total p50 0.807</sub> | -1.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.069<br><sub>context: p90 0.129 · p95 0.135 · p99 0.142 · 4803 op/s · total p50 0.818</sub> | 0.066<br><sub>context: p90 0.122 · p95 0.127 · p99 0.135 · 5810 op/s · total p50 0.675</sub> | -4.6% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.076<br><sub>context: p90 0.143 · p95 0.147 · p99 0.157 · 32925 op/s · total p50 0.908</sub> | 0.076<br><sub>context: p90 0.143 · p95 0.147 · p99 0.159 · 33586 op/s · total p50 0.892</sub> | -0.2% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.036<br><sub>context: p90 0.066 · p95 0.069 · p99 0.074 · 6720 op/s · total p50 0.585</sub> | 0.040<br><sub>context: p90 0.070 · p95 0.071 · p99 0.084 · 7013 op/s · total p50 0.554</sub> | +9.8% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.042<br><sub>context: p90 0.076 · p95 0.079 · p99 0.089 · 39303 op/s · total p50 0.779</sub> | 0.042<br><sub>context: p90 0.075 · p95 0.079 · p99 0.090 · 39621 op/s · total p50 0.763</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.565<br><sub>context: p90 0.609 · p95 0.624 · p99 0.675 · 1335 op/s · total p50 2.979</sub> | 0.568<br><sub>context: p90 0.615 · p95 0.620 · p99 0.628 · 1352 op/s · total p50 2.949</sub> | +0.6% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.639<br><sub>context: p90 0.786 · p95 0.840 · p99 0.931 · 8143 op/s · total p50 3.773</sub> | 0.623<br><sub>context: p90 0.757 · p95 0.786 · p99 0.871 · 8439 op/s · total p50 3.640</sub> | -2.4% (-0.015) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.565<br><sub>context: p90 0.599 · p95 0.609 · p99 0.644 · 1351 op/s · total p50 2.909</sub> | 0.559<br><sub>context: p90 0.595 · p95 0.601 · p99 0.621 · 1385 op/s · total p50 2.857</sub> | -1.0% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.593<br><sub>context: p90 0.675 · p95 0.709 · p99 0.754 · 8619 op/s · total p50 3.609</sub> | 0.583<br><sub>context: p90 0.631 · p95 0.649 · p99 0.691 · 8927 op/s · total p50 3.470</sub> | -1.6% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.064<br><sub>context: p90 0.097 · p95 0.103 · p99 0.108 · 5418 op/s · total p50 0.721</sub> | 0.055<br><sub>context: p90 0.084 · p95 0.092 · p99 0.098 · 5913 op/s · total p50 0.661</sub> | -13.2% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.066<br><sub>context: p90 0.101 · p95 0.108 · p99 0.120 · 32019 op/s · total p50 0.907</sub> | 0.065<br><sub>context: p90 0.100 · p95 0.107 · p99 0.121 · 34148 op/s · total p50 0.883</sub> | -1.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.143<br><sub>context: p90 0.226 · p95 0.240 · p99 0.274 · 3184 op/s · total p50 1.243</sub> | 0.134<br><sub>context: p90 0.216 · p95 0.241 · p99 0.280 · 3462 op/s · total p50 1.118</sub> | -6.7% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.169<br><sub>context: p90 0.262 · p95 0.300 · p99 0.357 · 21031 op/s · total p50 1.403</sub> | 0.164<br><sub>context: p90 0.255 · p95 0.292 · p99 0.350 · 22019 op/s · total p50 1.353</sub> | -2.7% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.023<br><sub>context: p90 0.033 · p95 0.040 · p99 0.050 · 7774 op/s · total p50 0.492</sub> | 0.022<br><sub>context: p90 0.032 · p95 0.038 · p99 0.049 · 8400 op/s · total p50 0.451</sub> | -6.2% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.025<br><sub>context: p90 0.033 · p95 0.037 · p99 0.043 · 41350 op/s · total p50 0.670</sub> | 0.025<br><sub>context: p90 0.034 · p95 0.037 · p99 0.044 · 42155 op/s · total p50 0.659</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.036<br><sub>context: p90 0.067 · p95 0.069 · p99 0.073 · 8356 op/s · total p50 0.466</sub> | 0.037<br><sub>context: p90 0.066 · p95 0.071 · p99 0.075 · 7999 op/s · total p50 0.490</sub> | +3.0% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.037<br><sub>context: p90 0.046 · p95 0.061 · p99 0.069 · 45692 op/s · total p50 0.529</sub> | 0.037<br><sub>context: p90 0.052 · p95 0.069 · p99 0.071 · 45999 op/s · total p50 0.516</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.036<br><sub>context: p90 0.070 · p95 0.071 · p99 0.078 · 8039 op/s · total p50 0.486</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.071 · p99 0.074 · 8103 op/s · total p50 0.483</sub> | -6.2% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.038<br><sub>context: p90 0.071 · p95 0.073 · p99 0.079 · 47775 op/s · total p50 0.508</sub> | 0.038<br><sub>context: p90 0.071 · p95 0.074 · p99 0.082 · 47952 op/s · total p50 0.502</sub> | +0.0% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.201<br><sub>context: p90 0.225 · p95 0.230 · p99 0.241 · 3257 op/s · total p50 1.215</sub> | 0.201<br><sub>context: p90 0.227 · p95 0.233 · p99 0.244 · 3241 op/s · total p50 1.219</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.229<br><sub>context: p90 0.255 · p95 0.263 · p99 0.286 · 21149 op/s · total p50 1.366</sub> | 0.232<br><sub>context: p90 0.257 · p95 0.268 · p99 0.302 · 19142 op/s · total p50 1.606</sub> | +1.0% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>
