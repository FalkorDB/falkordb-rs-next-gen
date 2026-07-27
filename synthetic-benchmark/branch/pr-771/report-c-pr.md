### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
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

**pr vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.256<br><sub>context: p90 1.307 · p95 1.326 · p99 1.353 · 628 op/s · total p50 1.584</sub> | 0.612<br><sub>context: p90 0.637 · p95 0.648 · p99 0.670 · 960 op/s · total p50 1.023</sub> | -51.3% (-0.644) | 150% AND 2 ms | 🟢 |
| 8 | 1.322<br><sub>context: p90 1.801 · p95 2.076 · p99 2.375 · 4726 op/s · total p50 1.551</sub> | 0.624<br><sub>context: p90 0.711 · p95 0.743 · p99 0.808 · 6787 op/s · total p50 1.089</sub> | -52.8% (-0.698) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.939<br><sub>context: p90 1.993 · p95 2.004 · p99 2.028 · 437 op/s · total p50 2.275</sub> | 0.646<br><sub>context: p90 0.672 · p95 0.681 · p99 0.695 · 903 op/s · total p50 1.102</sub> | -66.7% (-1.293) | 150% AND 2 ms | 🟢 |
| 8 | 2.007<br><sub>context: p90 2.392 · p95 2.588 · p99 2.902 · 3194 op/s · total p50 2.273</sub> | 0.668<br><sub>context: p90 0.769 · p95 0.818 · p99 0.906 · 6373 op/s · total p50 1.154</sub> | -66.7% (-1.338) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.888<br><sub>context: p90 1.975 · p95 1.997 · p99 2.029 · 444 op/s · total p50 2.241</sub> | 1.115<br><sub>context: p90 1.144 · p95 1.152 · p99 1.174 · 626 op/s · total p50 1.592</sub> | -40.9% (-0.773) | 150% AND 2 ms | 🟢 |
| 8 | 2.041<br><sub>context: p90 2.739 · p95 2.993 · p99 3.384 · 3217 op/s · total p50 2.287</sub> | 1.793<br><sub>context: p90 2.510 · p95 2.736 · p99 2.997 · 3338 op/s · total p50 2.284</sub> | -12.1% (-0.248) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.503<br><sub>context: p90 2.570 · p95 2.584 · p99 2.613 · 348 op/s · total p50 2.872</sub> | 1.209<br><sub>context: p90 1.240 · p95 1.248 · p99 1.275 · 596 op/s · total p50 1.683</sub> | -51.7% (-1.294) | 150% AND 2 ms | 🟢 |
| 8 | 2.866<br><sub>context: p90 3.769 · p95 4.077 · p99 4.640 · 2387 op/s · total p50 3.137</sub> | 1.227<br><sub>context: p90 1.370 · p95 1.428 · p99 1.526 · 4350 op/s · total p50 1.672</sub> | -57.2% (-1.640) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.126 · p95 0.130 · p99 0.142 · 3718 op/s · total p50 0.259</sub> | 0.038<br><sub>context: p90 0.044 · p95 0.047 · p99 0.053 · 2871 op/s · total p50 0.324</sub> | -62.6% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.113<br><sub>context: p90 0.165 · p95 0.187 · p99 0.239 · 31931 op/s · total p50 0.236</sub> | 0.021<br><sub>context: p90 0.042 · p95 0.047 · p99 0.058 · 26228 op/s · total p50 0.279</sub> | -81.1% (-0.092) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.258<br><sub>context: p90 0.283 · p95 0.292 · p99 0.311 · 2072 op/s · total p50 0.462</sub> | 0.069<br><sub>context: p90 0.098 · p95 0.103 · p99 0.119 · 2511 op/s · total p50 0.374</sub> | -73.2% (-0.189) | 150% AND 2 ms | 🟢 |
| 8 | 0.273<br><sub>context: p90 0.358 · p95 0.388 · p99 0.469 · 16765 op/s · total p50 0.451</sub> | 0.058<br><sub>context: p90 0.089 · p95 0.098 · p99 0.115 · 19309 op/s · total p50 0.371</sub> | -78.7% (-0.215) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.306 · p95 0.311 · p99 0.321 · 1987 op/s · total p50 0.486</sub> | 0.071<br><sub>context: p90 0.098 · p95 0.102 · p99 0.109 · 2117 op/s · total p50 0.462</sub> | -75.2% (-0.214) | 150% AND 2 ms | 🟢 |
| 8 | 0.298<br><sub>context: p90 0.387 · p95 0.425 · p99 0.505 · 15798 op/s · total p50 0.475</sub> | 0.064<br><sub>context: p90 0.094 · p95 0.101 · p99 0.121 · 16307 op/s · total p50 0.449</sub> | -78.6% (-0.234) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.304<br><sub>context: p90 0.329 · p95 0.336 · p99 0.353 · 1838 op/s · total p50 0.529</sub> | 0.137<br><sub>context: p90 0.165 · p95 0.176 · p99 0.184 · 1704 op/s · total p50 0.583</sub> | -55.0% (-0.167) | 150% AND 2 ms | 🟢 |
| 8 | 0.321<br><sub>context: p90 0.411 · p95 0.447 · p99 0.543 · 13953 op/s · total p50 0.541</sub> | 0.120<br><sub>context: p90 0.161 · p95 0.173 · p99 0.201 · 14146 op/s · total p50 0.545</sub> | -62.6% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.358 · p95 0.364 · p99 0.371 · 1663 op/s · total p50 0.595</sub> | 0.140<br><sub>context: p90 0.168 · p95 0.176 · p99 0.184 · 1527 op/s · total p50 0.649</sub> | -56.6% (-0.183) | 150% AND 2 ms | 🟢 |
| 8 | 0.357<br><sub>context: p90 0.452 · p95 0.490 · p99 0.577 · 13502 op/s · total p50 0.567</sub> | 0.134<br><sub>context: p90 0.173 · p95 0.187 · p99 0.239 · 11027 op/s · total p50 0.685</sub> | -62.5% (-0.223) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.385<br><sub>context: p90 0.441 · p95 0.456 · p99 0.504 · 1282 op/s · total p50 0.768</sub> | 0.177<br><sub>context: p90 0.213 · p95 0.226 · p99 0.249 · 1299 op/s · total p50 0.768</sub> | -54.0% (-0.208) | 150% AND 2 ms | 🟢 |
| 8 | 0.406<br><sub>context: p90 0.507 · p95 0.544 · p99 0.602 · 10238 op/s · total p50 0.754</sub> | 0.166<br><sub>context: p90 0.217 · p95 0.235 · p99 0.267 · 10475 op/s · total p50 0.721</sub> | -59.0% (-0.239) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.408<br><sub>context: p90 0.480 · p95 0.497 · p99 0.534 · 1234 op/s · total p50 0.800</sub> | 0.183<br><sub>context: p90 0.230 · p95 0.240 · p99 0.254 · 1266 op/s · total p50 0.779</sub> | -55.2% (-0.225) | 150% AND 2 ms | 🟢 |
| 8 | 0.439<br><sub>context: p90 0.558 · p95 0.596 · p99 0.673 · 9732 op/s · total p50 0.792</sub> | 0.181<br><sub>context: p90 0.241 · p95 0.267 · p99 0.320 · 9141 op/s · total p50 0.829</sub> | -58.7% (-0.258) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.623<br><sub>context: p90 0.770 · p95 0.811 · p99 0.865 · 695 op/s · total p50 1.429</sub> | 0.316<br><sub>context: p90 0.387 · p95 0.398 · p99 0.443 · 764 op/s · total p50 1.314</sub> | -49.3% (-0.307) | 150% AND 2 ms | 🟢 |
| 8 | 0.688<br><sub>context: p90 0.907 · p95 0.989 · p99 1.155 · 4841 op/s · total p50 1.581</sub> | 0.326<br><sub>context: p90 0.433 · p95 0.470 · p99 0.550 · 5218 op/s · total p50 1.475</sub> | -52.6% (-0.362) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.688<br><sub>context: p90 0.858 · p95 0.939 · p99 1.028 · 656 op/s · total p50 1.472</sub> | 0.331<br><sub>context: p90 0.413 · p95 0.432 · p99 0.472 · 697 op/s · total p50 1.410</sub> | -51.9% (-0.357) | 150% AND 2 ms | 🟢 |
| 8 | 0.754<br><sub>context: p90 1.032 · p95 1.120 · p99 1.308 · 4604 op/s · total p50 1.664</sub> | 0.341<br><sub>context: p90 0.465 · p95 0.511 · p99 0.604 · 4938 op/s · total p50 1.545</sub> | -54.8% (-0.413) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.524<br><sub>context: p90 0.601 · p95 0.630 · p99 0.676 · 1293 op/s · total p50 0.767</sub> | 0.361<br><sub>context: p90 0.544 · p95 0.586 · p99 0.667 · 1270 op/s · total p50 0.780</sub> | -31.1% (-0.163) | 150% AND 2 ms | 🟢 |
| 8 | 0.558<br><sub>context: p90 0.711 · p95 0.764 · p99 0.864 · 10047 op/s · total p50 0.757</sub> | 0.421<br><sub>context: p90 0.636 · p95 0.693 · p99 0.784 · 8994 op/s · total p50 0.844</sub> | -24.6% (-0.137) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.356 · p95 0.359 · p99 0.369 · 1672 op/s · total p50 0.596</sub> | 0.103<br><sub>context: p90 0.132 · p95 0.141 · p99 0.151 · 2327 op/s · total p50 0.426</sub> | -68.8% (-0.227) | 150% AND 2 ms | 🟢 |
| 8 | 0.367<br><sub>context: p90 0.466 · p95 0.502 · p99 0.605 · 13961 op/s · total p50 0.540</sub> | 0.088<br><sub>context: p90 0.121 · p95 0.131 · p99 0.151 · 19289 op/s · total p50 0.395</sub> | -76.1% (-0.279) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.203 · p95 0.207 · p99 0.215 · 2274 op/s · total p50 0.431</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 5254 op/s · total p50 0.176</sub> | -98.4% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.193<br><sub>context: p90 0.278 · p95 0.321 · p99 0.404 · 21746 op/s · total p50 0.335</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 34156 op/s · total p50 0.223</sub> | -98.7% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.129<br><sub>context: p90 0.146 · p95 0.150 · p99 0.161 · 3244 op/s · total p50 0.300</sub> | 0.002<br><sub>context: p90 0.006 · p95 0.006 · p99 0.006 · 6559 op/s · total p50 0.143</sub> | -98.2% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.146<br><sub>context: p90 0.232 · p95 0.264 · p99 0.317 · 23818 op/s · total p50 0.297</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 37852 op/s · total p50 0.203</sub> | -98.4% (-0.144) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.378<br><sub>context: p90 0.413 · p95 0.422 · p99 0.433 · 1604 op/s · total p50 0.613</sub> | 0.086<br><sub>context: p90 0.114 · p95 0.122 · p99 0.132 · 2066 op/s · total p50 0.467</sub> | -77.1% (-0.292) | 150% AND 2 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.548 · p95 0.589 · p99 0.703 · 12028 op/s · total p50 0.641</sub> | 0.076<br><sub>context: p90 0.108 · p95 0.116 · p99 0.131 · 17481 op/s · total p50 0.439</sub> | -82.6% (-0.361) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.701<br><sub>context: p90 0.974 · p95 1.088 · p99 1.228 · 1034 op/s · total p50 0.940</sub> | 1.183<br><sub>context: p90 1.895 · p95 2.127 · p99 2.452 · 605 op/s · total p50 1.621</sub> | +68.6% (+0.481) | 150% AND 2 ms | 🟢 |
| 8 | 0.743<br><sub>context: p90 1.135 · p95 1.254 · p99 1.530 · 7167 op/s · total p50 1.032</sub> | 1.721<br><sub>context: p90 2.851 · p95 3.204 · p99 3.687 · 3542 op/s · total p50 2.142</sub> | +131.6% (+0.978) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.292<br><sub>context: p90 3.620 · p95 4.156 · p99 4.664 · 364 op/s · total p50 2.647</sub> | 4.630<br><sub>context: p90 7.114 · p95 7.912 · p99 8.482 · 186 op/s · total p50 5.211</sub> | +102.0% (+2.338) | 150% AND 2 ms | 🟢 |
| 8 | 2.402<br><sub>context: p90 4.098 · p95 4.500 · p99 5.369 · 2814 op/s · total p50 2.664</sub> | 7.488<br><sub>context: p90 11.537 · p95 12.550 · p99 13.708 · 984 op/s · total p50 7.899</sub> | +211.7% (+5.086) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.200 · p95 0.205 · p99 0.213 · 2487 op/s · total p50 0.397</sub> | 0.029<br><sub>context: p90 0.038 · p95 0.041 · p99 0.051 · 3862 op/s · total p50 0.252</sub> | -83.7% (-0.151) | 150% AND 2 ms | 🟢 |
| 8 | 0.184<br><sub>context: p90 0.241 · p95 0.261 · p99 0.309 · 21456 op/s · total p50 0.355</sub> | 0.020<br><sub>context: p90 0.027 · p95 0.032 · p99 0.040 · 26897 op/s · total p50 0.282</sub> | -89.4% (-0.165) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.150<br><sub>context: p90 0.192 · p95 0.196 · p99 0.208 · 2962 op/s · total p50 0.336</sub> | 0.026<br><sub>context: p90 0.036 · p95 0.038 · p99 0.045 · 3892 op/s · total p50 0.239</sub> | -82.6% (-0.124) | 150% AND 2 ms | 🟢 |
| 8 | 0.184<br><sub>context: p90 0.238 · p95 0.263 · p99 0.308 · 21684 op/s · total p50 0.350</sub> | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.035 · 28128 op/s · total p50 0.265</sub> | -89.8% (-0.166) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.209 · p95 0.211 · p99 0.228 · 2179 op/s · total p50 0.449</sub> | 0.017<br><sub>context: p90 0.019 · p95 0.020 · p99 0.023 · 2832 op/s · total p50 0.336</sub> | -91.1% (-0.169) | 150% AND 2 ms | 🟢 |
| 8 | 0.200<br><sub>context: p90 0.259 · p95 0.279 · p99 0.335 · 15559 op/s · total p50 0.491</sub> | 0.012<br><sub>context: p90 0.017 · p95 0.019 · p99 0.022 · 17452 op/s · total p50 0.397</sub> | -94.0% (-0.188) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.119<br><sub>context: p90 0.166 · p95 0.171 · p99 0.178 · 3903 op/s · total p50 0.241</sub> | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.016 · 4863 op/s · total p50 0.192</sub> | -91.7% (-0.109) | 150% AND 2 ms | 🟢 |
| 8 | 0.155<br><sub>context: p90 0.208 · p95 0.230 · p99 0.276 · 26292 op/s · total p50 0.288</sub> | 0.007<br><sub>context: p90 0.013 · p95 0.015 · p99 0.017 · 28226 op/s · total p50 0.250</sub> | -95.6% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.244<br><sub>context: p90 0.287 · p95 0.293 · p99 0.300 · 2279 op/s · total p50 0.430</sub> | 0.142<br><sub>context: p90 0.185 · p95 0.192 · p99 0.215 · 2243 op/s · total p50 0.437</sub> | -41.9% (-0.102) | 150% AND 2 ms | 🟢 |
| 8 | 0.284<br><sub>context: p90 0.360 · p95 0.382 · p99 0.435 · 16186 op/s · total p50 0.470</sub> | 0.139<br><sub>context: p90 0.183 · p95 0.195 · p99 0.221 · 19627 op/s · total p50 0.388</sub> | -50.9% (-0.144) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.188<br><sub>context: p90 0.232 · p95 0.238 · p99 0.247 · 2659 op/s · total p50 0.373</sub> | 0.076<br><sub>context: p90 0.105 · p95 0.111 · p99 0.134 · 3180 op/s · total p50 0.294</sub> | -59.7% (-0.112) | 150% AND 2 ms | 🟢 |
| 8 | 0.221<br><sub>context: p90 0.289 · p95 0.315 · p99 0.362 · 20569 op/s · total p50 0.371</sub> | 0.072<br><sub>context: p90 0.104 · p95 0.114 · p99 0.137 · 23943 op/s · total p50 0.318</sub> | -67.6% (-0.149) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.295<br><sub>context: p90 0.315 · p95 0.323 · p99 0.332 · 1856 op/s · total p50 0.531</sub> | 0.129<br><sub>context: p90 0.157 · p95 0.164 · p99 0.170 · 1751 op/s · total p50 0.558</sub> | -56.3% (-0.166) | 150% AND 2 ms | 🟢 |
| 8 | 0.313<br><sub>context: p90 0.411 · p95 0.449 · p99 0.507 · 13865 op/s · total p50 0.543</sub> | 0.115<br><sub>context: p90 0.152 · p95 0.164 · p99 0.190 · 14140 op/s · total p50 0.534</sub> | -63.3% (-0.198) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.324 · p95 0.328 · p99 0.337 · 1460 op/s · total p50 0.684</sub> | 0.131<br><sub>context: p90 0.164 · p95 0.172 · p99 0.181 · 1256 op/s · total p50 0.792</sub> | -56.3% (-0.168) | 150% AND 2 ms | 🟢 |
| 8 | 0.308<br><sub>context: p90 0.389 · p95 0.421 · p99 0.482 · 11368 op/s · total p50 0.672</sub> | 0.126<br><sub>context: p90 0.164 · p95 0.178 · p99 0.211 · 10538 op/s · total p50 0.732</sub> | -59.1% (-0.182) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.333<br><sub>context: p90 0.356 · p95 0.362 · p99 0.382 · 1345 op/s · total p50 0.744</sub> | 0.134<br><sub>context: p90 0.162 · p95 0.173 · p99 0.187 · 1409 op/s · total p50 0.695</sub> | -59.6% (-0.198) | 150% AND 2 ms | 🟢 |
| 8 | 0.351<br><sub>context: p90 0.442 · p95 0.479 · p99 0.554 · 10083 op/s · total p50 0.714</sub> | 0.130<br><sub>context: p90 0.169 · p95 0.183 · p99 0.219 · 10129 op/s · total p50 0.741</sub> | -63.0% (-0.221) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.353 · p95 0.358 · p99 0.366 · 1572 op/s · total p50 0.627</sub> | 0.134<br><sub>context: p90 0.159 · p95 0.165 · p99 0.185 · 1525 op/s · total p50 0.650</sub> | -59.4% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 0.348<br><sub>context: p90 0.445 · p95 0.475 · p99 0.535 · 13293 op/s · total p50 0.575</sub> | 0.128<br><sub>context: p90 0.168 · p95 0.185 · p99 0.229 · 11236 op/s · total p50 0.680</sub> | -63.2% (-0.220) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.305<br><sub>context: p90 0.340 · p95 0.344 · p99 0.357 · 1932 op/s · total p50 0.511</sub> | 0.091<br><sub>context: p90 0.124 · p95 0.134 · p99 0.142 · 2693 op/s · total p50 0.355</sub> | -70.1% (-0.214) | 150% AND 2 ms | 🟢 |
| 8 | 0.349<br><sub>context: p90 0.443 · p95 0.482 · p99 0.574 · 14181 op/s · total p50 0.536</sub> | 0.084<br><sub>context: p90 0.117 · p95 0.127 · p99 0.150 · 19564 op/s · total p50 0.388</sub> | -75.9% (-0.265) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 6.271<br><sub>context: p90 6.371 · p95 6.405 · p99 6.469 · 52 op/s · total p50 19.060</sub> | 2.936<br><sub>context: p90 2.984 · p95 3.002 · p99 3.082 · 64 op/s · total p50 15.590</sub> | -53.2% (-3.335) | 150% AND 2 ms | 🟢 |
| 8 | 8.091<br><sub>context: p90 9.893 · p95 10.192 · p99 10.672 · 306 op/s · total p50 24.178</sub> | 3.675<br><sub>context: p90 4.055 · p95 4.096 · p99 4.290 · 377 op/s · total p50 18.649</sub> | -54.6% (-4.416) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.454 · p95 0.470 · p99 0.493 · 1443 op/s · total p50 0.689</sub> | 0.137<br><sub>context: p90 0.168 · p95 0.173 · p99 0.187 · 1681 op/s · total p50 0.587</sub> | -67.4% (-0.282) | 150% AND 2 ms | 🟢 |
| 8 | 0.464<br><sub>context: p90 0.591 · p95 0.658 · p99 0.770 · 11694 op/s · total p50 0.649</sub> | 0.126<br><sub>context: p90 0.167 · p95 0.179 · p99 0.220 · 13212 op/s · total p50 0.567</sub> | -72.7% (-0.337) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.610<br><sub>context: p90 0.714 · p95 0.750 · p99 0.780 · 583 op/s · total p50 1.689</sub> | 0.301<br><sub>context: p90 0.364 · p95 0.388 · p99 0.413 · 605 op/s · total p50 1.652</sub> | -50.7% (-0.310) | 150% AND 2 ms | 🟢 |
| 8 | 0.650<br><sub>context: p90 0.822 · p95 0.883 · p99 0.972 · 4201 op/s · total p50 1.838</sub> | 0.305<br><sub>context: p90 0.413 · p95 0.444 · p99 0.520 · 4415 op/s · total p50 1.744</sub> | -53.1% (-0.345) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.325<br><sub>context: p90 0.350 · p95 0.358 · p99 0.363 · 1570 op/s · total p50 0.625</sub> | 0.132<br><sub>context: p90 0.165 · p95 0.170 · p99 0.181 · 1495 op/s · total p50 0.656</sub> | -59.4% (-0.193) | 150% AND 2 ms | 🟢 |
| 8 | 0.352<br><sub>context: p90 0.450 · p95 0.496 · p99 0.610 · 11947 op/s · total p50 0.612</sub> | 0.128<br><sub>context: p90 0.170 · p95 0.184 · p99 0.236 · 12165 op/s · total p50 0.627</sub> | -63.7% (-0.224) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.093<br><sub>context: p90 2.675 · p95 2.725 · p99 2.833 · 412 op/s · total p50 2.503</sub> | 0.142<br><sub>context: p90 0.199 · p95 0.208 · p99 0.235 · 1894 op/s · total p50 0.517</sub> | -93.2% (-1.951) | 150% AND 2 ms | 🟢 |
| 8 | 2.104<br><sub>context: p90 2.743 · p95 2.893 · p99 3.409 · 3369 op/s · total p50 2.353</sub> | 0.131<br><sub>context: p90 0.191 · p95 0.211 · p99 0.258 · 15511 op/s · total p50 0.480</sub> | -93.8% (-1.974) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.108<br><sub>context: p90 2.728 · p95 2.783 · p99 2.864 · 413 op/s · total p50 2.496</sub> | 0.148<br><sub>context: p90 0.206 · p95 0.231 · p99 0.259 · 1731 op/s · total p50 0.569</sub> | -93.0% (-1.960) | 150% AND 2 ms | 🟢 |
| 8 | 2.161<br><sub>context: p90 2.854 · p95 2.984 · p99 3.536 · 3350 op/s · total p50 2.408</sub> | 0.132<br><sub>context: p90 0.196 · p95 0.216 · p99 0.254 · 15402 op/s · total p50 0.481</sub> | -93.9% (-2.029) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.155<br><sub>context: p90 0.176 · p95 0.189 · p99 0.203 · 2978 op/s · total p50 0.333</sub> | 0.048<br><sub>context: p90 0.076 · p95 0.080 · p99 0.088 · 2205 op/s · total p50 0.441</sub> | -69.0% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.151<br><sub>context: p90 0.198 · p95 0.217 · p99 0.261 · 25578 op/s · total p50 0.294</sub> | 0.044<br><sub>context: p90 0.075 · p95 0.081 · p99 0.099 · 17670 op/s · total p50 0.413</sub> | -71.1% (-0.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.253 · p95 0.260 · p99 0.270 · 2260 op/s · total p50 0.432</sub> | 0.006<br><sub>context: p90 0.009 · p95 0.010 · p99 0.016 · 4657 op/s · total p50 0.206</sub> | -97.5% (-0.221) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.250<br><sub>context: p90 0.319 · p95 0.342 · p99 0.429 · 18520 op/s · total p50 0.408</sub> | 0.004<br><sub>context: p90 0.006 · p95 0.007 · p99 0.009 · 29213 op/s · total p50 0.259</sub> | -98.3% (-0.246) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.229<br><sub>context: p90 0.253 · p95 0.260 · p99 0.270 · 2211 op/s · total p50 0.449</sub> | 0.054<br><sub>context: p90 0.085 · p95 0.087 · p99 0.097 · 2805 op/s · total p50 0.336</sub> | -76.2% (-0.174) | 150% AND 2 ms | 🟢 |
| 8 | 0.269<br><sub>context: p90 0.355 · p95 0.393 · p99 0.474 · 17861 op/s · total p50 0.424</sub> | 0.048<br><sub>context: p90 0.079 · p95 0.084 · p99 0.094 · 24548 op/s · total p50 0.313</sub> | -82.3% (-0.221) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.244<br><sub>context: p90 0.269 · p95 0.276 · p99 0.306 · 2234 op/s · total p50 0.432</sub> | 0.072<br><sub>context: p90 0.132 · p95 0.139 · p99 0.155 · 2810 op/s · total p50 0.342</sub> | -70.4% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 0.285<br><sub>context: p90 0.372 · p95 0.408 · p99 0.503 · 17497 op/s · total p50 0.435</sub> | 0.070<br><sub>context: p90 0.134 · p95 0.144 · p99 0.154 · 22704 op/s · total p50 0.336</sub> | -75.5% (-0.215) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.242 · p95 0.247 · p99 0.256 · 2202 op/s · total p50 0.445</sub> | 0.040<br><sub>context: p90 0.069 · p95 0.076 · p99 0.084 · 3745 op/s · total p50 0.251</sub> | -82.4% (-0.186) | 150% AND 2 ms | 🟢 |
| 8 | 0.236<br><sub>context: p90 0.311 · p95 0.337 · p99 0.402 · 19664 op/s · total p50 0.386</sub> | 0.039<br><sub>context: p90 0.073 · p95 0.077 · p99 0.085 · 27372 op/s · total p50 0.276</sub> | -83.3% (-0.197) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.140<br><sub>context: p90 1.200 · p95 1.210 · p99 1.226 · 615 op/s · total p50 1.613</sub> | 0.632<br><sub>context: p90 0.664 · p95 0.679 · p99 0.739 · 758 op/s · total p50 1.311</sub> | -44.5% (-0.508) | 150% AND 2 ms | 🟢 |
| 8 | 1.224<br><sub>context: p90 1.496 · p95 1.573 · p99 1.863 · 4353 op/s · total p50 1.707</sub> | 0.659<br><sub>context: p90 0.785 · p95 0.838 · p99 0.925 · 5678 op/s · total p50 1.359</sub> | -46.1% (-0.565) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.148<br><sub>context: p90 1.211 · p95 1.227 · p99 1.252 · 706 op/s · total p50 1.415</sub> | 0.612<br><sub>context: p90 0.652 · p95 0.666 · p99 0.680 · 882 op/s · total p50 1.127</sub> | -46.7% (-0.536) | 150% AND 2 ms | 🟢 |
| 8 | 1.182<br><sub>context: p90 1.377 · p95 1.471 · p99 1.692 · 4983 op/s · total p50 1.441</sub> | 0.630<br><sub>context: p90 0.749 · p95 0.801 · p99 0.865 · 6356 op/s · total p50 1.187</sub> | -46.7% (-0.553) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.258<br><sub>context: p90 0.282 · p95 0.290 · p99 0.299 · 1910 op/s · total p50 0.520</sub> | 0.078<br><sub>context: p90 0.111 · p95 0.116 · p99 0.126 · 2286 op/s · total p50 0.432</sub> | -69.6% (-0.179) | 150% AND 2 ms | 🟢 |
| 8 | 0.258<br><sub>context: p90 0.324 · p95 0.354 · p99 0.404 · 16431 op/s · total p50 0.466</sub> | 0.066<br><sub>context: p90 0.096 · p95 0.103 · p99 0.118 · 20026 op/s · total p50 0.378</sub> | -74.4% (-0.192) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.395<br><sub>context: p90 0.466 · p95 0.485 · p99 0.521 · 1478 op/s · total p50 0.660</sub> | 0.179<br><sub>context: p90 0.266 · p95 0.288 · p99 0.327 · 1909 op/s · total p50 0.516</sub> | -54.7% (-0.216) | 150% AND 2 ms | 🟢 |
| 8 | 0.400<br><sub>context: p90 0.529 · p95 0.576 · p99 0.682 · 12599 op/s · total p50 0.606</sub> | 0.178<br><sub>context: p90 0.270 · p95 0.299 · p99 0.353 · 16097 op/s · total p50 0.475</sub> | -55.6% (-0.223) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.199<br><sub>context: p90 0.230 · p95 0.237 · p99 0.248 · 2369 op/s · total p50 0.415</sub> | 0.031<br><sub>context: p90 0.049 · p95 0.052 · p99 0.063 · 3744 op/s · total p50 0.254</sub> | -84.6% (-0.168) | 150% AND 2 ms | 🟢 |
| 8 | 0.224<br><sub>context: p90 0.288 · p95 0.312 · p99 0.384 · 19075 op/s · total p50 0.398</sub> | 0.026<br><sub>context: p90 0.035 · p95 0.039 · p99 0.050 · 26920 op/s · total p50 0.281</sub> | -88.3% (-0.198) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.143<br><sub>context: p90 0.176 · p95 0.183 · p99 0.198 · 3146 op/s · total p50 0.303</sub> | 0.046<br><sub>context: p90 0.075 · p95 0.076 · p99 0.086 · 2728 op/s · total p50 0.354</sub> | -68.0% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.159<br><sub>context: p90 0.222 · p95 0.250 · p99 0.294 · 23282 op/s · total p50 0.316</sub> | 0.039<br><sub>context: p90 0.071 · p95 0.076 · p99 0.088 · 24553 op/s · total p50 0.297</sub> | -75.4% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.140<br><sub>context: p90 0.175 · p95 0.181 · p99 0.189 · 3105 op/s · total p50 0.318</sub> | 0.047<br><sub>context: p90 0.075 · p95 0.079 · p99 0.084 · 2392 op/s · total p50 0.401</sub> | -66.8% (-0.094) | 150% AND 2 ms | 🟢 |
| 8 | 0.159<br><sub>context: p90 0.218 · p95 0.247 · p99 0.297 · 23790 op/s · total p50 0.317</sub> | 0.038<br><sub>context: p90 0.072 · p95 0.075 · p99 0.084 · 25499 op/s · total p50 0.278</sub> | -75.8% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.864<br><sub>context: p90 0.901 · p95 0.907 · p99 0.927 · 856 op/s · total p50 1.161</sub> | 0.214<br><sub>context: p90 0.231 · p95 0.241 · p99 0.248 · 1544 op/s · total p50 0.640</sub> | -75.2% (-0.650) | 150% AND 2 ms | 🟢 |
| 8 | 0.894<br><sub>context: p90 1.192 · p95 1.394 · p99 1.621 · 5988 op/s · total p50 1.153</sub> | 0.221<br><sub>context: p90 0.264 · p95 0.279 · p99 0.320 · 13327 op/s · total p50 0.557</sub> | -75.3% (-0.673) | 150% AND 2 ms | 🟢 |

</details>
