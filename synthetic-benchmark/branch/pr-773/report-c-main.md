### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.230<br><sub>context: p90 1.287 · p95 1.299 · p99 1.337 · 769 op/s · total p50 5.185</sub> | 0.582<br><sub>context: p90 0.615 · p95 0.621 · p99 0.638 · 1457 op/s · total p50 2.727</sub> | -52.7% (-0.648) | 150% AND 2 ms | 🟢 |
| 8 | 1.384<br><sub>context: p90 2.045 · p95 2.283 · p99 2.603 · 4874 op/s · total p50 6.220</sub> | 0.615<br><sub>context: p90 0.662 · p95 0.685 · p99 0.720 · 9003 op/s · total p50 3.507</sub> | -55.6% (-0.769) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.887<br><sub>context: p90 1.959 · p95 1.974 · p99 2.012 · 509 op/s · total p50 7.812</sub> | 0.607<br><sub>context: p90 0.630 · p95 0.637 · p99 0.649 · 1410 op/s · total p50 2.804</sub> | -67.8% (-1.280) | 150% AND 2 ms | 🟢 |
| 8 | 2.028<br><sub>context: p90 2.460 · p95 2.638 · p99 3.004 · 3548 op/s · total p50 8.622</sub> | 0.655<br><sub>context: p90 0.697 · p95 0.720 · p99 0.755 · 8706 op/s · total p50 3.615</sub> | -67.7% (-1.373) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.865<br><sub>context: p90 1.935 · p95 1.953 · p99 1.977 · 515 op/s · total p50 7.748</sub> | 1.091<br><sub>context: p90 1.120 · p95 1.131 · p99 1.157 · 807 op/s · total p50 4.969</sub> | -41.5% (-0.774) | 150% AND 2 ms | 🟢 |
| 8 | 2.056<br><sub>context: p90 2.652 · p95 2.930 · p99 3.275 · 3386 op/s · total p50 8.913</sub> | 1.820<br><sub>context: p90 2.468 · p95 2.655 · p99 3.062 · 3857 op/s · total p50 8.084</sub> | -11.4% (-0.235) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.453<br><sub>context: p90 2.534 · p95 2.554 · p99 2.574 · 395 op/s · total p50 10.134</sub> | 1.175<br><sub>context: p90 1.210 · p95 1.229 · p99 1.242 · 758 op/s · total p50 5.257</sub> | -52.1% (-1.279) | 150% AND 2 ms | 🟢 |
| 8 | 2.813<br><sub>context: p90 3.681 · p95 3.958 · p99 4.914 · 2476 op/s · total p50 12.087</sub> | 1.213<br><sub>context: p90 1.346 · p95 1.392 · p99 1.453 · 5148 op/s · total p50 5.908</sub> | -56.9% (-1.600) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.066<br><sub>context: p90 0.095 · p95 0.109 · p99 0.133 · 9391 op/s · total p50 0.412</sub> | 0.014<br><sub>context: p90 0.018 · p95 0.019 · p99 0.026 · 13251 op/s · total p50 0.292</sub> | -79.3% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.159<br><sub>context: p90 0.284 · p95 0.352 · p99 0.433 · 35635 op/s · total p50 0.850</sub> | 0.015<br><sub>context: p90 0.021 · p95 0.024 · p99 0.033 · 48635 op/s · total p50 0.402</sub> | -90.8% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.249 · p95 0.271 · p99 0.293 · 3963 op/s · total p50 0.996</sub> | 0.047<br><sub>context: p90 0.075 · p95 0.083 · p99 0.092 · 6969 op/s · total p50 0.562</sub> | -74.7% (-0.139) | 150% AND 2 ms | 🟢 |
| 8 | 0.307<br><sub>context: p90 0.455 · p95 0.516 · p99 0.631 · 18820 op/s · total p50 1.641</sub> | 0.053<br><sub>context: p90 0.087 · p95 0.091 · p99 0.100 · 39016 op/s · total p50 0.766</sub> | -82.6% (-0.253) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.236<br><sub>context: p90 0.279 · p95 0.301 · p99 0.314 · 3415 op/s · total p50 1.148</sub> | 0.052<br><sub>context: p90 0.079 · p95 0.089 · p99 0.102 · 5294 op/s · total p50 0.747</sub> | -77.9% (-0.183) | 150% AND 2 ms | 🟢 |
| 8 | 0.334<br><sub>context: p90 0.467 · p95 0.527 · p99 0.645 · 17519 op/s · total p50 1.759</sub> | 0.058<br><sub>context: p90 0.092 · p95 0.096 · p99 0.117 · 32645 op/s · total p50 0.922</sub> | -82.7% (-0.276) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.243<br><sub>context: p90 0.300 · p95 0.308 · p99 0.326 · 3312 op/s · total p50 1.195</sub> | 0.096<br><sub>context: p90 0.127 · p95 0.134 · p99 0.146 · 4227 op/s · total p50 0.929</sub> | -60.5% (-0.147) | 150% AND 2 ms | 🟢 |
| 8 | 0.365<br><sub>context: p90 0.538 · p95 0.584 · p99 0.738 · 15973 op/s · total p50 1.937</sub> | 0.103<br><sub>context: p90 0.135 · p95 0.144 · p99 0.162 · 27805 op/s · total p50 1.057</sub> | -71.7% (-0.261) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.322 · p95 0.337 · p99 0.352 · 2900 op/s · total p50 1.369</sub> | 0.104<br><sub>context: p90 0.137 · p95 0.153 · p99 0.165 · 3841 op/s · total p50 1.013</sub> | -62.9% (-0.177) | 150% AND 2 ms | 🟢 |
| 8 | 0.394<br><sub>context: p90 0.563 · p95 0.643 · p99 0.751 · 14922 op/s · total p50 2.066</sub> | 0.112<br><sub>context: p90 0.149 · p95 0.160 · p99 0.185 · 24974 op/s · total p50 1.186</sub> | -71.5% (-0.282) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.293<br><sub>context: p90 0.373 · p95 0.394 · p99 0.418 · 2742 op/s · total p50 1.451</sub> | 0.137<br><sub>context: p90 0.189 · p95 0.206 · p99 0.221 · 3140 op/s · total p50 1.252</sub> | -53.1% (-0.155) | 150% AND 2 ms | 🟢 |
| 8 | 0.451<br><sub>context: p90 0.616 · p95 0.689 · p99 0.840 · 12491 op/s · total p50 2.431</sub> | 0.150<br><sub>context: p90 0.200 · p95 0.214 · p99 0.239 · 14951 op/s · total p50 1.975</sub> | -66.8% (-0.301) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.352<br><sub>context: p90 0.426 · p95 0.447 · p99 0.490 · 2368 op/s · total p50 1.683</sub> | 0.151<br><sub>context: p90 0.199 · p95 0.205 · p99 0.217 · 2933 op/s · total p50 1.350</sub> | -57.2% (-0.201) | 150% AND 2 ms | 🟢 |
| 8 | 0.499<br><sub>context: p90 0.693 · p95 0.767 · p99 0.900 · 11307 op/s · total p50 2.745</sub> | 0.160<br><sub>context: p90 0.211 · p95 0.227 · p99 0.253 · 13160 op/s · total p50 2.298</sub> | -67.9% (-0.339) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.561<br><sub>context: p90 0.734 · p95 0.822 · p99 0.892 · 1448 op/s · total p50 2.754</sub> | 0.262<br><sub>context: p90 0.343 · p95 0.360 · p99 0.405 · 1822 op/s · total p50 1.971</sub> | -53.4% (-0.299) | 150% AND 2 ms | 🟢 |
| 8 | 0.618<br><sub>context: p90 0.847 · p95 0.913 · p99 1.040 · 3773 op/s · total p50 8.128</sub> | 0.272<br><sub>context: p90 0.376 · p95 0.412 · p99 0.454 · 4096 op/s · total p50 7.514</sub> | -55.9% (-0.346) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.608<br><sub>context: p90 0.759 · p95 0.812 · p99 0.897 · 1401 op/s · total p50 2.870</sub> | 0.267<br><sub>context: p90 0.349 · p95 0.369 · p99 0.414 · 1872 op/s · total p50 2.073</sub> | -56.1% (-0.341) | 150% AND 2 ms | 🟢 |
| 8 | 0.698<br><sub>context: p90 0.954 · p95 1.035 · p99 1.191 · 3447 op/s · total p50 8.825</sub> | 0.291<br><sub>context: p90 0.387 · p95 0.415 · p99 0.461 · 3872 op/s · total p50 7.885</sub> | -58.3% (-0.407) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.443<br><sub>context: p90 0.523 · p95 0.540 · p99 0.567 · 1981 op/s · total p50 2.011</sub> | 0.330<br><sub>context: p90 0.510 · p95 0.540 · p99 0.625 · 1934 op/s · total p50 2.040</sub> | -25.5% (-0.113) | 150% AND 2 ms | 🟢 |
| 8 | 0.579<br><sub>context: p90 0.751 · p95 0.826 · p99 0.999 · 10341 op/s · total p50 2.975</sub> | 0.418<br><sub>context: p90 0.639 · p95 0.712 · p99 0.868 · 11825 op/s · total p50 2.582</sub> | -27.9% (-0.161) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.332 · p95 0.346 · p99 0.362 · 3003 op/s · total p50 1.306</sub> | 0.088<br><sub>context: p90 0.119 · p95 0.133 · p99 0.143 · 4291 op/s · total p50 0.922</sub> | -67.8% (-0.185) | 150% AND 2 ms | 🟢 |
| 8 | 0.408<br><sub>context: p90 0.587 · p95 0.651 · p99 0.826 · 14784 op/s · total p50 2.078</sub> | 0.089<br><sub>context: p90 0.122 · p95 0.130 · p99 0.155 · 28701 op/s · total p50 1.045</sub> | -78.2% (-0.319) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.147 · p95 0.158 · p99 0.173 · 6241 op/s · total p50 0.625</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 13092 op/s · total p50 0.294</sub> | -98.5% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.235<br><sub>context: p90 0.376 · p95 0.425 · p99 0.519 · 25436 op/s · total p50 1.202</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 47494 op/s · total p50 0.438</sub> | -99.0% (-0.233) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.088<br><sub>context: p90 0.105 · p95 0.110 · p99 0.143 · 8299 op/s · total p50 0.480</sub> | 0.002<br><sub>context: p90 0.002 · p95 0.003 · p99 0.003 · 15286 op/s · total p50 0.245</sub> | -98.0% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.299 · p95 0.360 · p99 0.466 · 32482 op/s · total p50 0.941</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 56338 op/s · total p50 0.407</sub> | -98.8% (-0.187) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.388 · p95 0.399 · p99 0.426 · 2563 op/s · total p50 1.546</sub> | 0.074<br><sub>context: p90 0.099 · p95 0.107 · p99 0.116 · 3945 op/s · total p50 0.985</sub> | -77.1% (-0.249) | 150% AND 2 ms | 🟢 |
| 8 | 0.490<br><sub>context: p90 0.722 · p95 0.836 · p99 1.024 · 12433 op/s · total p50 2.487</sub> | 0.080<br><sub>context: p90 0.110 · p95 0.120 · p99 0.134 · 24992 op/s · total p50 1.178</sub> | -83.8% (-0.411) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.649<br><sub>context: p90 0.842 · p95 0.882 · p99 0.943 · 1404 op/s · total p50 2.869</sub> | 1.063<br><sub>context: p90 1.529 · p95 1.593 · p99 1.775 · 808 op/s · total p50 4.923</sub> | +63.7% (+0.414) | 150% AND 2 ms | 🟢 |
| 8 | 0.772<br><sub>context: p90 1.065 · p95 1.153 · p99 1.496 · 7571 op/s · total p50 4.003</sub> | 1.790<br><sub>context: p90 2.676 · p95 2.874 · p99 3.226 · 3598 op/s · total p50 8.356</sub> | +131.9% (+1.018) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.260<br><sub>context: p90 3.910 · p95 4.075 · p99 4.307 · 401 op/s · total p50 9.875</sub> | 4.401<br><sub>context: p90 7.217 · p95 7.610 · p99 7.888 · 204 op/s · total p50 19.418</sub> | +94.7% (+2.141) | 150% AND 2 ms | 🟢 |
| 8 | 2.386<br><sub>context: p90 4.030 · p95 4.556 · p99 5.867 · 2866 op/s · total p50 10.517</sub> | 7.393<br><sub>context: p90 11.576 · p95 12.239 · p99 13.483 · 987 op/s · total p50 32.073</sub> | +209.9% (+5.007) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.120<br><sub>context: p90 0.163 · p95 0.171 · p99 0.195 · 5886 op/s · total p50 0.670</sub> | 0.017<br><sub>context: p90 0.021 · p95 0.023 · p99 0.026 · 9328 op/s · total p50 0.413</sub> | -85.6% (-0.103) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.373 · p95 0.431 · p99 0.542 · 24170 op/s · total p50 1.271</sub> | 0.019<br><sub>context: p90 0.024 · p95 0.026 · p99 0.030 · 44815 op/s · total p50 0.620</sub> | -92.0% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.118<br><sub>context: p90 0.148 · p95 0.160 · p99 0.195 · 5838 op/s · total p50 0.656</sub> | 0.013<br><sub>context: p90 0.019 · p95 0.020 · p99 0.022 · 11636 op/s · total p50 0.330</sub> | -88.6% (-0.104) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.381 · p95 0.435 · p99 0.553 · 23729 op/s · total p50 1.287</sub> | 0.018<br><sub>context: p90 0.022 · p95 0.025 · p99 0.029 · 46112 op/s · total p50 0.551</sub> | -92.4% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.129<br><sub>context: p90 0.181 · p95 0.207 · p99 0.216 · 5411 op/s · total p50 0.724</sub> | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.017 · 8203 op/s · total p50 0.473</sub> | -92.2% (-0.119) | 150% AND 2 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.254 · p95 0.277 · p99 0.341 · 17197 op/s · total p50 1.778</sub> | 0.011<br><sub>context: p90 0.014 · p95 0.016 · p99 0.020 · 18126 op/s · total p50 1.669</sub> | -94.4% (-0.179) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.097<br><sub>context: p90 0.129 · p95 0.149 · p99 0.162 · 7014 op/s · total p50 0.556</sub> | 0.005<br><sub>context: p90 0.007 · p95 0.008 · p99 0.009 · 12157 op/s · total p50 0.307</sub> | -95.3% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.197<br><sub>context: p90 0.323 · p95 0.374 · p99 0.486 · 29947 op/s · total p50 1.029</sub> | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.011 · 48582 op/s · total p50 0.447</sub> | -97.1% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.218<br><sub>context: p90 0.271 · p95 0.286 · p99 0.312 · 3575 op/s · total p50 1.101</sub> | 0.139<br><sub>context: p90 0.176 · p95 0.182 · p99 0.205 · 3747 op/s · total p50 1.059</sub> | -36.4% (-0.080) | 150% AND 2 ms | 🟢 |
| 8 | 0.313<br><sub>context: p90 0.447 · p95 0.505 · p99 0.589 · 17788 op/s · total p50 1.717</sub> | 0.152<br><sub>context: p90 0.195 · p95 0.210 · p99 0.236 · 26182 op/s · total p50 1.129</sub> | -51.6% (-0.162) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.158<br><sub>context: p90 0.200 · p95 0.210 · p99 0.227 · 4827 op/s · total p50 0.816</sub> | 0.068<br><sub>context: p90 0.095 · p95 0.102 · p99 0.111 · 5347 op/s · total p50 0.727</sub> | -57.1% (-0.090) | 150% AND 2 ms | 🟢 |
| 8 | 0.258<br><sub>context: p90 0.398 · p95 0.449 · p99 0.536 · 21993 op/s · total p50 1.397</sub> | 0.076<br><sub>context: p90 0.110 · p95 0.126 · p99 0.162 · 36250 op/s · total p50 0.830</sub> | -70.6% (-0.182) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.245<br><sub>context: p90 0.290 · p95 0.300 · p99 0.314 · 3364 op/s · total p50 1.174</sub> | 0.086<br><sub>context: p90 0.121 · p95 0.126 · p99 0.151 · 4693 op/s · total p50 0.832</sub> | -64.8% (-0.159) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.478 · p95 0.528 · p99 0.639 · 17075 op/s · total p50 1.806</sub> | 0.104<br><sub>context: p90 0.140 · p95 0.149 · p99 0.167 · 25910 op/s · total p50 1.173</sub> | -69.2% (-0.234) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.205<br><sub>context: p90 0.291 · p95 0.313 · p99 0.336 · 3481 op/s · total p50 1.131</sub> | 0.100<br><sub>context: p90 0.136 · p95 0.142 · p99 0.162 · 3618 op/s · total p50 1.079</sub> | -51.2% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 0.314<br><sub>context: p90 0.413 · p95 0.469 · p99 0.580 · 14030 op/s · total p50 2.108</sub> | 0.103<br><sub>context: p90 0.142 · p95 0.154 · p99 0.177 · 14955 op/s · total p50 2.027</sub> | -67.1% (-0.210) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.249<br><sub>context: p90 0.310 · p95 0.322 · p99 0.354 · 3046 op/s · total p50 1.310</sub> | 0.101<br><sub>context: p90 0.130 · p95 0.141 · p99 0.156 · 3562 op/s · total p50 1.098</sub> | -59.4% (-0.148) | 150% AND 2 ms | 🟢 |
| 8 | 0.374<br><sub>context: p90 0.512 · p95 0.570 · p99 0.700 · 13838 op/s · total p50 2.121</sub> | 0.109<br><sub>context: p90 0.146 · p95 0.158 · p99 0.177 · 16312 op/s · total p50 1.881</sub> | -70.8% (-0.265) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.244<br><sub>context: p90 0.301 · p95 0.312 · p99 0.349 · 3302 op/s · total p50 1.192</sub> | 0.093<br><sub>context: p90 0.123 · p95 0.130 · p99 0.155 · 4128 op/s · total p50 0.952</sub> | -61.9% (-0.151) | 150% AND 2 ms | 🟢 |
| 8 | 0.376<br><sub>context: p90 0.549 · p95 0.614 · p99 0.723 · 15677 op/s · total p50 1.967</sub> | 0.106<br><sub>context: p90 0.142 · p95 0.152 · p99 0.173 · 24390 op/s · total p50 1.226</sub> | -71.8% (-0.270) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.329 · p95 0.342 · p99 0.369 · 2995 op/s · total p50 1.334</sub> | 0.085<br><sub>context: p90 0.118 · p95 0.125 · p99 0.140 · 4708 op/s · total p50 0.828</sub> | -69.9% (-0.198) | 150% AND 2 ms | 🟢 |
| 8 | 0.389<br><sub>context: p90 0.555 · p95 0.622 · p99 0.765 · 15143 op/s · total p50 2.026</sub> | 0.084<br><sub>context: p90 0.120 · p95 0.130 · p99 0.157 · 30584 op/s · total p50 0.966</sub> | -78.4% (-0.305) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.632<br><sub>context: p90 5.785 · p95 5.867 · p99 6.058 · 72 op/s · total p50 53.929</sub> | 2.744<br><sub>context: p90 2.790 · p95 2.810 · p99 2.850 · 92 op/s · total p50 42.427</sub> | -51.3% (-2.888) | 150% AND 2 ms | 🟢 |
| 8 | 6.081<br><sub>context: p90 8.042 · p95 8.332 · p99 8.644 · 154 op/s · total p50 202.674</sub> | 2.788<br><sub>context: p90 3.875 · p95 3.940 · p99 4.039 · 155 op/s · total p50 197.395</sub> | -54.2% (-3.293) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.347<br><sub>context: p90 0.410 · p95 0.419 · p99 0.455 · 2461 op/s · total p50 1.619</sub> | 0.115<br><sub>context: p90 0.146 · p95 0.153 · p99 0.166 · 3355 op/s · total p50 1.189</sub> | -66.9% (-0.232) | 150% AND 2 ms | 🟢 |
| 8 | 0.489<br><sub>context: p90 0.675 · p95 0.763 · p99 0.916 · 12179 op/s · total p50 2.506</sub> | 0.111<br><sub>context: p90 0.146 · p95 0.154 · p99 0.171 · 24345 op/s · total p50 1.213</sub> | -77.4% (-0.378) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.533<br><sub>context: p90 0.657 · p95 0.698 · p99 0.772 · 1292 op/s · total p50 2.752</sub> | 0.244<br><sub>context: p90 0.344 · p95 0.366 · p99 0.421 · 1123 op/s · total p50 3.391</sub> | -54.1% (-0.288) | 150% AND 2 ms | 🟢 |
| 8 | 0.578<br><sub>context: p90 0.749 · p95 0.795 · p99 0.899 · 2530 op/s · total p50 11.790</sub> | 0.262<br><sub>context: p90 0.352 · p95 0.382 · p99 0.450 · 2570 op/s · total p50 11.686</sub> | -54.6% (-0.315) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.260<br><sub>context: p90 0.308 · p95 0.315 · p99 0.342 · 3145 op/s · total p50 1.255</sub> | 0.101<br><sub>context: p90 0.134 · p95 0.149 · p99 0.169 · 4102 op/s · total p50 0.946</sub> | -61.0% (-0.158) | 150% AND 2 ms | 🟢 |
| 8 | 0.382<br><sub>context: p90 0.538 · p95 0.590 · p99 0.727 · 15425 op/s · total p50 2.020</sub> | 0.111<br><sub>context: p90 0.148 · p95 0.158 · p99 0.180 · 24209 op/s · total p50 1.168</sub> | -71.0% (-0.271) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.041<br><sub>context: p90 2.602 · p95 2.637 · p99 2.706 · 486 op/s · total p50 8.411</sub> | 0.113<br><sub>context: p90 0.152 · p95 0.162 · p99 0.222 · 4231 op/s · total p50 0.923</sub> | -94.5% (-1.928) | 150% AND 2 ms | 🟢 |
| 8 | 2.224<br><sub>context: p90 2.801 · p95 2.911 · p99 3.796 · 3391 op/s · total p50 9.172</sub> | 0.123<br><sub>context: p90 0.173 · p95 0.194 · p99 0.247 · 26904 op/s · total p50 1.081</sub> | -94.5% (-2.101) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.965<br><sub>context: p90 2.575 · p95 2.636 · p99 2.748 · 494 op/s · total p50 8.022</sub> | 0.108<br><sub>context: p90 0.156 · p95 0.177 · p99 0.198 · 3909 op/s · total p50 1.020</sub> | -94.5% (-1.857) | 150% AND 2 ms | 🟢 |
| 8 | 2.156<br><sub>context: p90 2.797 · p95 2.942 · p99 3.913 · 3538 op/s · total p50 8.671</sub> | 0.123<br><sub>context: p90 0.173 · p95 0.190 · p99 0.217 · 25495 op/s · total p50 1.178</sub> | -94.3% (-2.033) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.120<br><sub>context: p90 0.145 · p95 0.156 · p99 0.178 · 6291 op/s · total p50 0.630</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.070 · p99 0.076 · 8552 op/s · total p50 0.446</sub> | -72.0% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.311 · p95 0.356 · p99 0.461 · 29871 op/s · total p50 1.016</sub> | 0.039<br><sub>context: p90 0.072 · p95 0.075 · p99 0.083 · 43502 op/s · total p50 0.611</sub> | -79.2% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.189<br><sub>context: p90 0.252 · p95 0.260 · p99 0.294 · 4118 op/s · total p50 0.962</sub> | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 10911 op/s · total p50 0.355</sub> | -98.4% (-0.186) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.283<br><sub>context: p90 0.411 · p95 0.483 · p99 0.557 · 19635 op/s · total p50 1.588</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 48104 op/s · total p50 0.481</sub> | -98.6% (-0.279) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.191<br><sub>context: p90 0.235 · p95 0.245 · p99 0.258 · 4268 op/s · total p50 0.923</sub> | 0.047<br><sub>context: p90 0.078 · p95 0.082 · p99 0.088 · 6049 op/s · total p50 0.647</sub> | -75.6% (-0.144) | 150% AND 2 ms | 🟢 |
| 8 | 0.328<br><sub>context: p90 0.496 · p95 0.566 · p99 0.713 · 18874 op/s · total p50 1.627</sub> | 0.050<br><sub>context: p90 0.083 · p95 0.086 · p99 0.097 · 33201 op/s · total p50 0.909</sub> | -84.6% (-0.277) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.200<br><sub>context: p90 0.239 · p95 0.251 · p99 0.267 · 4152 op/s · total p50 0.955</sub> | 0.069<br><sub>context: p90 0.129 · p95 0.135 · p99 0.142 · 4803 op/s · total p50 0.818</sub> | -65.5% (-0.131) | 150% AND 2 ms | 🟢 |
| 8 | 0.336<br><sub>context: p90 0.492 · p95 0.555 · p99 0.704 · 18417 op/s · total p50 1.669</sub> | 0.076<br><sub>context: p90 0.143 · p95 0.147 · p99 0.157 · 32925 op/s · total p50 0.908</sub> | -77.4% (-0.260) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.235 · p95 0.250 · p99 0.264 · 4356 op/s · total p50 0.904</sub> | 0.036<br><sub>context: p90 0.066 · p95 0.069 · p99 0.074 · 6720 op/s · total p50 0.585</sub> | -79.8% (-0.142) | 150% AND 2 ms | 🟢 |
| 8 | 0.274<br><sub>context: p90 0.405 · p95 0.464 · p99 0.574 · 21109 op/s · total p50 1.455</sub> | 0.042<br><sub>context: p90 0.076 · p95 0.079 · p99 0.089 · 39303 op/s · total p50 0.779</sub> | -84.5% (-0.232) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.089<br><sub>context: p90 1.168 · p95 1.187 · p99 1.214 · 850 op/s · total p50 4.678</sub> | 0.565<br><sub>context: p90 0.609 · p95 0.624 · p99 0.675 · 1335 op/s · total p50 2.979</sub> | -48.2% (-0.525) | 150% AND 2 ms | 🟢 |
| 8 | 1.275<br><sub>context: p90 1.567 · p95 1.738 · p99 2.056 · 5389 op/s · total p50 5.555</sub> | 0.639<br><sub>context: p90 0.786 · p95 0.840 · p99 0.931 · 8143 op/s · total p50 3.773</sub> | -49.9% (-0.636) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.090<br><sub>context: p90 1.180 · p95 1.189 · p99 1.223 · 870 op/s · total p50 4.604</sub> | 0.565<br><sub>context: p90 0.599 · p95 0.609 · p99 0.644 · 1351 op/s · total p50 2.909</sub> | -48.1% (-0.525) | 150% AND 2 ms | 🟢 |
| 8 | 1.211<br><sub>context: p90 1.404 · p95 1.515 · p99 1.736 · 4982 op/s · total p50 6.130</sub> | 0.593<br><sub>context: p90 0.675 · p95 0.709 · p99 0.754 · 8619 op/s · total p50 3.609</sub> | -51.1% (-0.618) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.222 · p95 0.234 · p99 0.267 · 4184 op/s · total p50 0.941</sub> | 0.064<br><sub>context: p90 0.097 · p95 0.103 · p99 0.108 · 5418 op/s · total p50 0.721</sub> | -65.8% (-0.123) | 150% AND 2 ms | 🟢 |
| 8 | 0.307<br><sub>context: p90 0.448 · p95 0.503 · p99 0.624 · 18958 op/s · total p50 1.639</sub> | 0.066<br><sub>context: p90 0.101 · p95 0.108 · p99 0.120 · 32019 op/s · total p50 0.907</sub> | -78.4% (-0.240) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.309<br><sub>context: p90 0.382 · p95 0.406 · p99 0.457 · 2669 op/s · total p50 1.477</sub> | 0.143<br><sub>context: p90 0.226 · p95 0.240 · p99 0.274 · 3184 op/s · total p50 1.243</sub> | -53.6% (-0.166) | 150% AND 2 ms | 🟢 |
| 8 | 0.413<br><sub>context: p90 0.581 · p95 0.644 · p99 0.797 · 14111 op/s · total p50 2.178</sub> | 0.169<br><sub>context: p90 0.262 · p95 0.300 · p99 0.357 · 21031 op/s · total p50 1.403</sub> | -59.2% (-0.245) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.174<br><sub>context: p90 0.216 · p95 0.225 · p99 0.238 · 4632 op/s · total p50 0.853</sub> | 0.023<br><sub>context: p90 0.033 · p95 0.040 · p99 0.050 · 7774 op/s · total p50 0.492</sub> | -86.6% (-0.150) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.417 · p95 0.480 · p99 0.584 · 20660 op/s · total p50 1.502</sub> | 0.025<br><sub>context: p90 0.033 · p95 0.037 · p99 0.043 · 41350 op/s · total p50 0.670</sub> | -90.6% (-0.241) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.159 · p95 0.170 · p99 0.189 · 5904 op/s · total p50 0.664</sub> | 0.036<br><sub>context: p90 0.067 · p95 0.069 · p99 0.073 · 8356 op/s · total p50 0.466</sub> | -70.7% (-0.087) | 150% AND 2 ms | 🟢 |
| 8 | 0.183<br><sub>context: p90 0.293 · p95 0.337 · p99 0.446 · 29729 op/s · total p50 1.027</sub> | 0.037<br><sub>context: p90 0.046 · p95 0.061 · p99 0.069 · 45692 op/s · total p50 0.529</sub> | -79.9% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.121<br><sub>context: p90 0.147 · p95 0.162 · p99 0.185 · 6272 op/s · total p50 0.624</sub> | 0.036<br><sub>context: p90 0.070 · p95 0.071 · p99 0.078 · 8039 op/s · total p50 0.486</sub> | -69.8% (-0.084) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.309 · p95 0.350 · p99 0.446 · 29021 op/s · total p50 1.050</sub> | 0.038<br><sub>context: p90 0.071 · p95 0.073 · p99 0.079 · 47775 op/s · total p50 0.508</sub> | -79.8% (-0.149) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.833<br><sub>context: p90 0.863 · p95 0.876 · p99 0.898 · 1111 op/s · total p50 3.584</sub> | 0.201<br><sub>context: p90 0.225 · p95 0.230 · p99 0.241 · 3257 op/s · total p50 1.215</sub> | -75.9% (-0.632) | 150% AND 2 ms | 🟢 |
| 8 | 0.930<br><sub>context: p90 1.565 · p95 1.706 · p99 1.962 · 6093 op/s · total p50 4.938</sub> | 0.229<br><sub>context: p90 0.255 · p95 0.263 · p99 0.286 · 21149 op/s · total p50 1.366</sub> | -75.4% (-0.701) | 150% AND 2 ms | 🟢 |

</details>
