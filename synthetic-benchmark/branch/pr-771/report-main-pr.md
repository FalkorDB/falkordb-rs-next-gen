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
| 1 | 0.599<br><sub>context: p90 0.621 · p95 0.631 · p99 0.643 · 1080 op/s · total p50 0.919</sub> | 0.612<br><sub>context: p90 0.637 · p95 0.648 · p99 0.670 · 960 op/s · total p50 1.023</sub> | +2.2% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.607<br><sub>context: p90 0.680 · p95 0.707 · p99 0.746 · 8091 op/s · total p50 0.899</sub> | 0.624<br><sub>context: p90 0.711 · p95 0.743 · p99 0.808 · 6787 op/s · total p50 1.089</sub> | +2.8% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.631<br><sub>context: p90 0.651 · p95 0.658 · p99 0.667 · 1061 op/s · total p50 0.944</sub> | 0.646<br><sub>context: p90 0.672 · p95 0.681 · p99 0.695 · 903 op/s · total p50 1.102</sub> | +2.5% (+0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.637<br><sub>context: p90 0.726 · p95 0.757 · p99 0.827 · 7722 op/s · total p50 0.939</sub> | 0.668<br><sub>context: p90 0.769 · p95 0.818 · p99 0.906 · 6373 op/s · total p50 1.154</sub> | +4.8% (+0.031) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.111<br><sub>context: p90 1.134 · p95 1.143 · p99 1.155 · 657 op/s · total p50 1.518</sub> | 1.115<br><sub>context: p90 1.144 · p95 1.152 · p99 1.174 · 626 op/s · total p50 1.592</sub> | +0.4% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.611<br><sub>context: p90 2.347 · p95 2.489 · p99 2.880 · 3785 op/s · total p50 1.992</sub> | 1.793<br><sub>context: p90 2.510 · p95 2.736 · p99 2.997 · 3338 op/s · total p50 2.284</sub> | +11.3% (+0.183) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.200<br><sub>context: p90 1.228 · p95 1.235 · p99 1.248 · 614 op/s · total p50 1.620</sub> | 1.209<br><sub>context: p90 1.240 · p95 1.248 · p99 1.275 · 596 op/s · total p50 1.683</sub> | +0.8% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.205<br><sub>context: p90 1.340 · p95 1.385 · p99 1.446 · 4730 op/s · total p50 1.556</sub> | 1.227<br><sub>context: p90 1.370 · p95 1.428 · p99 1.526 · 4350 op/s · total p50 1.672</sub> | +1.8% (+0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.023<br><sub>context: p90 0.040 · p95 0.041 · p99 0.050 · 5652 op/s · total p50 0.159</sub> | 0.038<br><sub>context: p90 0.044 · p95 0.047 · p99 0.053 · 2871 op/s · total p50 0.324</sub> | +65.7% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.017<br><sub>context: p90 0.024 · p95 0.026 · p99 0.032 · 35517 op/s · total p50 0.218</sub> | 0.021<br><sub>context: p90 0.042 · p95 0.047 · p99 0.058 · 26228 op/s · total p50 0.279</sub> | +25.3% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.066<br><sub>context: p90 0.095 · p95 0.100 · p99 0.107 · 2617 op/s · total p50 0.373</sub> | 0.069<br><sub>context: p90 0.098 · p95 0.103 · p99 0.119 · 2511 op/s · total p50 0.374</sub> | +4.4% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.054<br><sub>context: p90 0.083 · p95 0.090 · p99 0.103 · 22479 op/s · total p50 0.335</sub> | 0.058<br><sub>context: p90 0.089 · p95 0.098 · p99 0.115 · 19309 op/s · total p50 0.371</sub> | +8.3% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.068<br><sub>context: p90 0.098 · p95 0.102 · p99 0.106 · 2419 op/s · total p50 0.402</sub> | 0.071<br><sub>context: p90 0.098 · p95 0.102 · p99 0.109 · 2117 op/s · total p50 0.462</sub> | +4.1% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.055<br><sub>context: p90 0.086 · p95 0.093 · p99 0.103 · 22311 op/s · total p50 0.345</sub> | 0.064<br><sub>context: p90 0.094 · p95 0.101 · p99 0.121 · 16307 op/s · total p50 0.449</sub> | +15.6% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.116<br><sub>context: p90 0.149 · p95 0.161 · p99 0.172 · 2302 op/s · total p50 0.430</sub> | 0.137<br><sub>context: p90 0.165 · p95 0.176 · p99 0.184 · 1704 op/s · total p50 0.583</sub> | +17.7% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.109<br><sub>context: p90 0.148 · p95 0.160 · p99 0.186 · 16721 op/s · total p50 0.460</sub> | 0.120<br><sub>context: p90 0.161 · p95 0.173 · p99 0.201 · 14146 op/s · total p50 0.545</sub> | +10.1% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.160 · p95 0.165 · p99 0.176 · 1922 op/s · total p50 0.516</sub> | 0.140<br><sub>context: p90 0.168 · p95 0.176 · p99 0.184 · 1527 op/s · total p50 0.649</sub> | +7.7% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.114<br><sub>context: p90 0.151 · p95 0.162 · p99 0.181 · 16350 op/s · total p50 0.471</sub> | 0.134<br><sub>context: p90 0.173 · p95 0.187 · p99 0.239 · 11027 op/s · total p50 0.685</sub> | +17.6% (+0.020) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.162<br><sub>context: p90 0.198 · p95 0.208 · p99 0.229 · 1525 op/s · total p50 0.648</sub> | 0.177<br><sub>context: p90 0.213 · p95 0.226 · p99 0.249 · 1299 op/s · total p50 0.768</sub> | +9.1% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.162<br><sub>context: p90 0.210 · p95 0.222 · p99 0.248 · 11357 op/s · total p50 0.680</sub> | 0.166<br><sub>context: p90 0.217 · p95 0.235 · p99 0.267 · 10475 op/s · total p50 0.721</sub> | +3.0% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.170<br><sub>context: p90 0.218 · p95 0.228 · p99 0.257 · 1443 op/s · total p50 0.673</sub> | 0.183<br><sub>context: p90 0.230 · p95 0.240 · p99 0.254 · 1266 op/s · total p50 0.779</sub> | +7.2% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.161<br><sub>context: p90 0.207 · p95 0.221 · p99 0.254 · 11370 op/s · total p50 0.683</sub> | 0.181<br><sub>context: p90 0.241 · p95 0.267 · p99 0.320 · 9141 op/s · total p50 0.829</sub> | +12.9% (+0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.304<br><sub>context: p90 0.375 · p95 0.401 · p99 0.420 · 814 op/s · total p50 1.222</sub> | 0.316<br><sub>context: p90 0.387 · p95 0.398 · p99 0.443 · 764 op/s · total p50 1.314</sub> | +3.9% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.310<br><sub>context: p90 0.407 · p95 0.436 · p99 0.481 · 5751 op/s · total p50 1.321</sub> | 0.326<br><sub>context: p90 0.433 · p95 0.470 · p99 0.550 · 5218 op/s · total p50 1.475</sub> | +5.2% (+0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.322<br><sub>context: p90 0.400 · p95 0.422 · p99 0.476 · 765 op/s · total p50 1.262</sub> | 0.331<br><sub>context: p90 0.413 · p95 0.432 · p99 0.472 · 697 op/s · total p50 1.410</sub> | +2.7% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.318<br><sub>context: p90 0.424 · p95 0.457 · p99 0.522 · 5733 op/s · total p50 1.317</sub> | 0.341<br><sub>context: p90 0.465 · p95 0.511 · p99 0.604 · 4938 op/s · total p50 1.545</sub> | +7.3% (+0.023) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.538 · p95 0.572 · p99 0.663 · 1338 op/s · total p50 0.738</sub> | 0.361<br><sub>context: p90 0.544 · p95 0.586 · p99 0.667 · 1270 op/s · total p50 0.780</sub> | -2.5% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.428<br><sub>context: p90 0.651 · p95 0.700 · p99 0.802 · 9029 op/s · total p50 0.825</sub> | 0.421<br><sub>context: p90 0.636 · p95 0.693 · p99 0.784 · 8994 op/s · total p50 0.844</sub> | -1.8% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.102<br><sub>context: p90 0.139 · p95 0.144 · p99 0.153 · 2500 op/s · total p50 0.390</sub> | 0.103<br><sub>context: p90 0.132 · p95 0.141 · p99 0.151 · 2327 op/s · total p50 0.426</sub> | +0.9% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.088<br><sub>context: p90 0.120 · p95 0.129 · p99 0.147 · 19126 op/s · total p50 0.402</sub> | 0.088<br><sub>context: p90 0.121 · p95 0.131 · p99 0.151 · 19289 op/s · total p50 0.395</sub> | -0.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 5148 op/s · total p50 0.178</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 5254 op/s · total p50 0.176</sub> | -3.2% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 34292 op/s · total p50 0.222</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 34156 op/s · total p50 0.223</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 6204 op/s · total p50 0.147</sub> | 0.002<br><sub>context: p90 0.006 · p95 0.006 · p99 0.006 · 6559 op/s · total p50 0.143</sub> | -20.8% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.004 · 37789 op/s · total p50 0.202</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 37852 op/s · total p50 0.203</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.085<br><sub>context: p90 0.114 · p95 0.124 · p99 0.135 · 2067 op/s · total p50 0.473</sub> | 0.086<br><sub>context: p90 0.114 · p95 0.122 · p99 0.132 · 2066 op/s · total p50 0.467</sub> | +2.1% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.077<br><sub>context: p90 0.109 · p95 0.117 · p99 0.132 · 16827 op/s · total p50 0.454</sub> | 0.076<br><sub>context: p90 0.108 · p95 0.116 · p99 0.131 · 17481 op/s · total p50 0.439</sub> | -0.9% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.192<br><sub>context: p90 1.907 · p95 2.105 · p99 2.413 · 599 op/s · total p50 1.614</sub> | 1.183<br><sub>context: p90 1.895 · p95 2.127 · p99 2.452 · 605 op/s · total p50 1.621</sub> | -0.8% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.762<br><sub>context: p90 2.949 · p95 3.288 · p99 3.871 · 3405 op/s · total p50 2.219</sub> | 1.721<br><sub>context: p90 2.851 · p95 3.204 · p99 3.687 · 3542 op/s · total p50 2.142</sub> | -2.3% (-0.041) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.654<br><sub>context: p90 7.208 · p95 7.992 · p99 8.818 · 185 op/s · total p50 5.198</sub> | 4.630<br><sub>context: p90 7.114 · p95 7.912 · p99 8.482 · 186 op/s · total p50 5.211</sub> | -0.5% (-0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 7.908<br><sub>context: p90 12.150 · p95 13.412 · p99 14.840 · 929 op/s · total p50 8.369</sub> | 7.488<br><sub>context: p90 11.537 · p95 12.550 · p99 13.708 · 984 op/s · total p50 7.899</sub> | -5.3% (-0.420) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.031<br><sub>context: p90 0.038 · p95 0.040 · p99 0.047 · 3666 op/s · total p50 0.255</sub> | 0.029<br><sub>context: p90 0.038 · p95 0.041 · p99 0.051 · 3862 op/s · total p50 0.252</sub> | -3.7% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.020<br><sub>context: p90 0.027 · p95 0.032 · p99 0.038 · 26105 op/s · total p50 0.285</sub> | 0.020<br><sub>context: p90 0.027 · p95 0.032 · p99 0.040 · 26897 op/s · total p50 0.282</sub> | -1.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.027<br><sub>context: p90 0.037 · p95 0.040 · p99 0.043 · 4194 op/s · total p50 0.242</sub> | 0.026<br><sub>context: p90 0.036 · p95 0.038 · p99 0.045 · 3892 op/s · total p50 0.239</sub> | -2.6% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.036 · 27617 op/s · total p50 0.270</sub> | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.035 · 28128 op/s · total p50 0.265</sub> | -1.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.013<br><sub>context: p90 0.018 · p95 0.019 · p99 0.025 · 3056 op/s · total p50 0.309</sub> | 0.017<br><sub>context: p90 0.019 · p95 0.020 · p99 0.023 · 2832 op/s · total p50 0.336</sub> | +27.3% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.012<br><sub>context: p90 0.016 · p95 0.017 · p99 0.021 · 20566 op/s · total p50 0.365</sub> | 0.012<br><sub>context: p90 0.017 · p95 0.019 · p99 0.022 · 17452 op/s · total p50 0.397</sub> | +4.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.006<br><sub>context: p90 0.013 · p95 0.014 · p99 0.015 · 6019 op/s · total p50 0.144</sub> | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.016 · 4863 op/s · total p50 0.192</sub> | +59.2% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.011 · 34942 op/s · total p50 0.218</sub> | 0.007<br><sub>context: p90 0.013 · p95 0.015 · p99 0.017 · 28226 op/s · total p50 0.250</sub> | +11.5% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.146<br><sub>context: p90 0.187 · p95 0.196 · p99 0.228 · 2078 op/s · total p50 0.466</sub> | 0.142<br><sub>context: p90 0.185 · p95 0.192 · p99 0.215 · 2243 op/s · total p50 0.437</sub> | -2.5% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.139<br><sub>context: p90 0.184 · p95 0.197 · p99 0.228 · 20123 op/s · total p50 0.380</sub> | 0.139<br><sub>context: p90 0.183 · p95 0.195 · p99 0.221 · 19627 op/s · total p50 0.388</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.078<br><sub>context: p90 0.108 · p95 0.117 · p99 0.133 · 2588 op/s · total p50 0.375</sub> | 0.076<br><sub>context: p90 0.105 · p95 0.111 · p99 0.134 · 3180 op/s · total p50 0.294</sub> | -3.5% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.072<br><sub>context: p90 0.106 · p95 0.113 · p99 0.136 · 24176 op/s · total p50 0.318</sub> | 0.072<br><sub>context: p90 0.104 · p95 0.114 · p99 0.137 · 23943 op/s · total p50 0.318</sub> | -0.7% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.154 · p95 0.165 · p99 0.183 · 1972 op/s · total p50 0.497</sub> | 0.129<br><sub>context: p90 0.157 · p95 0.164 · p99 0.170 · 1751 op/s · total p50 0.558</sub> | +5.7% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.108<br><sub>context: p90 0.147 · p95 0.156 · p99 0.174 · 16144 op/s · total p50 0.474</sub> | 0.115<br><sub>context: p90 0.152 · p95 0.164 · p99 0.190 · 14140 op/s · total p50 0.534</sub> | +6.5% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.120<br><sub>context: p90 0.154 · p95 0.157 · p99 0.166 · 1805 op/s · total p50 0.548</sub> | 0.131<br><sub>context: p90 0.164 · p95 0.172 · p99 0.181 · 1256 op/s · total p50 0.792</sub> | +8.6% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.112<br><sub>context: p90 0.150 · p95 0.160 · p99 0.180 · 12673 op/s · total p50 0.600</sub> | 0.126<br><sub>context: p90 0.164 · p95 0.178 · p99 0.211 · 10538 op/s · total p50 0.732</sub> | +12.7% (+0.014) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.131<br><sub>context: p90 0.161 · p95 0.168 · p99 0.177 · 1527 op/s · total p50 0.653</sub> | 0.134<br><sub>context: p90 0.162 · p95 0.173 · p99 0.187 · 1409 op/s · total p50 0.695</sub> | +2.6% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.122<br><sub>context: p90 0.162 · p95 0.174 · p99 0.197 · 11605 op/s · total p50 0.656</sub> | 0.130<br><sub>context: p90 0.169 · p95 0.183 · p99 0.219 · 10129 op/s · total p50 0.741</sub> | +6.4% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.126<br><sub>context: p90 0.157 · p95 0.164 · p99 0.174 · 1880 op/s · total p50 0.526</sub> | 0.134<br><sub>context: p90 0.159 · p95 0.165 · p99 0.185 · 1525 op/s · total p50 0.650</sub> | +6.4% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.106<br><sub>context: p90 0.141 · p95 0.154 · p99 0.172 · 16397 op/s · total p50 0.470</sub> | 0.128<br><sub>context: p90 0.168 · p95 0.185 · p99 0.229 · 11236 op/s · total p50 0.680</sub> | +20.4% (+0.022) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.138 · p95 0.141 · p99 0.154 · 2274 op/s · total p50 0.428</sub> | 0.091<br><sub>context: p90 0.124 · p95 0.134 · p99 0.142 · 2693 op/s · total p50 0.355</sub> | -12.7% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.089<br><sub>context: p90 0.128 · p95 0.141 · p99 0.173 · 18145 op/s · total p50 0.406</sub> | 0.084<br><sub>context: p90 0.117 · p95 0.127 · p99 0.150 · 19564 op/s · total p50 0.388</sub> | -5.7% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.937<br><sub>context: p90 2.978 · p95 2.996 · p99 3.050 · 64 op/s · total p50 15.687</sub> | 2.936<br><sub>context: p90 2.984 · p95 3.002 · p99 3.082 · 64 op/s · total p50 15.590</sub> | -0.1% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 3.725<br><sub>context: p90 4.050 · p95 4.078 · p99 4.171 · 385 op/s · total p50 18.581</sub> | 3.675<br><sub>context: p90 4.055 · p95 4.096 · p99 4.290 · 377 op/s · total p50 18.649</sub> | -1.3% (-0.049) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.133<br><sub>context: p90 0.164 · p95 0.171 · p99 0.185 · 1973 op/s · total p50 0.509</sub> | 0.137<br><sub>context: p90 0.168 · p95 0.173 · p99 0.187 · 1681 op/s · total p50 0.587</sub> | +2.9% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.109<br><sub>context: p90 0.145 · p95 0.154 · p99 0.175 · 17934 op/s · total p50 0.430</sub> | 0.126<br><sub>context: p90 0.167 · p95 0.179 · p99 0.220 · 13212 op/s · total p50 0.567</sub> | +16.5% (+0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.348 · p95 0.354 · p99 0.380 · 670 op/s · total p50 1.466</sub> | 0.301<br><sub>context: p90 0.364 · p95 0.388 · p99 0.413 · 605 op/s · total p50 1.652</sub> | +6.4% (+0.018) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.294<br><sub>context: p90 0.388 · p95 0.416 · p99 0.460 · 4794 op/s · total p50 1.619</sub> | 0.305<br><sub>context: p90 0.413 · p95 0.444 · p99 0.520 · 4415 op/s · total p50 1.744</sub> | +3.8% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.156 · p95 0.163 · p99 0.177 · 2075 op/s · total p50 0.477</sub> | 0.132<br><sub>context: p90 0.165 · p95 0.170 · p99 0.181 · 1495 op/s · total p50 0.656</sub> | +7.2% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.112<br><sub>context: p90 0.145 · p95 0.157 · p99 0.184 · 15667 op/s · total p50 0.491</sub> | 0.128<br><sub>context: p90 0.170 · p95 0.184 · p99 0.236 · 12165 op/s · total p50 0.627</sub> | +14.4% (+0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.138<br><sub>context: p90 0.188 · p95 0.200 · p99 0.229 · 2089 op/s · total p50 0.473</sub> | 0.142<br><sub>context: p90 0.199 · p95 0.208 · p99 0.235 · 1894 op/s · total p50 0.517</sub> | +2.7% (+0.004) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.124<br><sub>context: p90 0.175 · p95 0.193 · p99 0.229 · 17215 op/s · total p50 0.438</sub> | 0.131<br><sub>context: p90 0.191 · p95 0.211 · p99 0.258 · 15511 op/s · total p50 0.480</sub> | +5.3% (+0.007) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.140<br><sub>context: p90 0.201 · p95 0.215 · p99 0.242 · 1940 op/s · total p50 0.510</sub> | 0.148<br><sub>context: p90 0.206 · p95 0.231 · p99 0.259 · 1731 op/s · total p50 0.569</sub> | +5.3% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.131<br><sub>context: p90 0.191 · p95 0.211 · p99 0.244 · 15536 op/s · total p50 0.472</sub> | 0.132<br><sub>context: p90 0.196 · p95 0.216 · p99 0.254 · 15402 op/s · total p50 0.481</sub> | +1.2% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.042<br><sub>context: p90 0.073 · p95 0.075 · p99 0.081 · 3471 op/s · total p50 0.271</sub> | 0.048<br><sub>context: p90 0.076 · p95 0.080 · p99 0.088 · 2205 op/s · total p50 0.441</sub> | +14.5% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.038<br><sub>context: p90 0.071 · p95 0.075 · p99 0.084 · 27587 op/s · total p50 0.275</sub> | 0.044<br><sub>context: p90 0.075 · p95 0.081 · p99 0.099 · 17670 op/s · total p50 0.413</sub> | +15.8% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.006<br><sub>context: p90 0.009 · p95 0.009 · p99 0.013 · 4798 op/s · total p50 0.193</sub> | 0.006<br><sub>context: p90 0.009 · p95 0.010 · p99 0.016 · 4657 op/s · total p50 0.206</sub> | +0.5% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 31029 op/s · total p50 0.244</sub> | 0.004<br><sub>context: p90 0.006 · p95 0.007 · p99 0.009 · 29213 op/s · total p50 0.259</sub> | +3.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.055<br><sub>context: p90 0.079 · p95 0.086 · p99 0.100 · 2809 op/s · total p50 0.346</sub> | 0.054<br><sub>context: p90 0.085 · p95 0.087 · p99 0.097 · 2805 op/s · total p50 0.336</sub> | -1.8% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.048<br><sub>context: p90 0.078 · p95 0.084 · p99 0.091 · 24439 op/s · total p50 0.309</sub> | 0.048<br><sub>context: p90 0.079 · p95 0.084 · p99 0.094 · 24548 op/s · total p50 0.313</sub> | -0.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.073<br><sub>context: p90 0.132 · p95 0.137 · p99 0.152 · 2783 op/s · total p50 0.349</sub> | 0.072<br><sub>context: p90 0.132 · p95 0.139 · p99 0.155 · 2810 op/s · total p50 0.342</sub> | -0.4% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.069<br><sub>context: p90 0.134 · p95 0.143 · p99 0.153 · 23028 op/s · total p50 0.335</sub> | 0.070<br><sub>context: p90 0.134 · p95 0.144 · p99 0.154 · 22704 op/s · total p50 0.336</sub> | +0.8% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.052<br><sub>context: p90 0.083 · p95 0.092 · p99 0.109 · 2914 op/s · total p50 0.312</sub> | 0.040<br><sub>context: p90 0.069 · p95 0.076 · p99 0.084 · 3745 op/s · total p50 0.251</sub> | -23.6% (-0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.040<br><sub>context: p90 0.075 · p95 0.078 · p99 0.088 · 26973 op/s · total p50 0.281</sub> | 0.039<br><sub>context: p90 0.073 · p95 0.077 · p99 0.085 · 27372 op/s · total p50 0.276</sub> | -1.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.607<br><sub>context: p90 0.644 · p95 0.655 · p99 0.668 · 856 op/s · total p50 1.178</sub> | 0.632<br><sub>context: p90 0.664 · p95 0.679 · p99 0.739 · 758 op/s · total p50 1.311</sub> | +4.1% (+0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.614<br><sub>context: p90 0.717 · p95 0.749 · p99 0.796 · 6050 op/s · total p50 1.258</sub> | 0.659<br><sub>context: p90 0.785 · p95 0.838 · p99 0.925 · 5678 op/s · total p50 1.359</sub> | +7.4% (+0.045) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.602<br><sub>context: p90 0.634 · p95 0.651 · p99 0.679 · 982 op/s · total p50 1.008</sub> | 0.612<br><sub>context: p90 0.652 · p95 0.666 · p99 0.680 · 882 op/s · total p50 1.127</sub> | +1.7% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.597<br><sub>context: p90 0.681 · p95 0.709 · p99 0.765 · 7676 op/s · total p50 0.981</sub> | 0.630<br><sub>context: p90 0.749 · p95 0.801 · p99 0.865 · 6356 op/s · total p50 1.187</sub> | +5.5% (+0.033) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.077<br><sub>context: p90 0.102 · p95 0.111 · p99 0.121 · 2591 op/s · total p50 0.376</sub> | 0.078<br><sub>context: p90 0.111 · p95 0.116 · p99 0.126 · 2286 op/s · total p50 0.432</sub> | +1.6% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.066<br><sub>context: p90 0.096 · p95 0.105 · p99 0.120 · 20456 op/s · total p50 0.371</sub> | 0.066<br><sub>context: p90 0.096 · p95 0.103 · p99 0.118 · 20026 op/s · total p50 0.378</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.284 · p95 0.317 · p99 0.352 · 1715 op/s · total p50 0.577</sub> | 0.179<br><sub>context: p90 0.266 · p95 0.288 · p99 0.327 · 1909 op/s · total p50 0.516</sub> | -7.8% (-0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.174<br><sub>context: p90 0.262 · p95 0.287 · p99 0.345 · 16164 op/s · total p50 0.468</sub> | 0.178<br><sub>context: p90 0.270 · p95 0.299 · p99 0.353 · 16097 op/s · total p50 0.475</sub> | +2.0% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.045<br><sub>context: p90 0.050 · p95 0.052 · p99 0.057 · 3245 op/s · total p50 0.301</sub> | 0.031<br><sub>context: p90 0.049 · p95 0.052 · p99 0.063 · 3744 op/s · total p50 0.254</sub> | -32.4% (-0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.027<br><sub>context: p90 0.038 · p95 0.044 · p99 0.053 · 25429 op/s · total p50 0.302</sub> | 0.026<br><sub>context: p90 0.035 · p95 0.039 · p99 0.050 · 26920 op/s · total p50 0.281</sub> | -1.9% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.038<br><sub>context: p90 0.064 · p95 0.069 · p99 0.077 · 4300 op/s · total p50 0.216</sub> | 0.046<br><sub>context: p90 0.075 · p95 0.076 · p99 0.086 · 2728 op/s · total p50 0.354</sub> | +22.4% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.037<br><sub>context: p90 0.069 · p95 0.073 · p99 0.078 · 29148 op/s · total p50 0.261</sub> | 0.039<br><sub>context: p90 0.071 · p95 0.076 · p99 0.088 · 24553 op/s · total p50 0.297</sub> | +6.4% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.041<br><sub>context: p90 0.071 · p95 0.076 · p99 0.083 · 4032 op/s · total p50 0.232</sub> | 0.047<br><sub>context: p90 0.075 · p95 0.079 · p99 0.084 · 2392 op/s · total p50 0.401</sub> | +15.0% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.037<br><sub>context: p90 0.070 · p95 0.074 · p99 0.084 · 27900 op/s · total p50 0.267</sub> | 0.038<br><sub>context: p90 0.072 · p95 0.075 · p99 0.084 · 25499 op/s · total p50 0.278</sub> | +2.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.205<br><sub>context: p90 0.223 · p95 0.231 · p99 0.247 · 2152 op/s · total p50 0.450</sub> | 0.214<br><sub>context: p90 0.231 · p95 0.241 · p99 0.248 · 1544 op/s · total p50 0.640</sub> | +4.2% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.216<br><sub>context: p90 0.255 · p95 0.262 · p99 0.279 · 16907 op/s · total p50 0.452</sub> | 0.221<br><sub>context: p90 0.264 · p95 0.279 · p99 0.320 · 13327 op/s · total p50 0.557</sub> | +2.4% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>
