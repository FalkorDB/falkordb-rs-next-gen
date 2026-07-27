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
| 1 | 1.256<br><sub>context: p90 1.307 · p95 1.326 · p99 1.353 · 628 op/s · total p50 1.584</sub> | 0.599<br><sub>context: p90 0.621 · p95 0.631 · p99 0.643 · 1080 op/s · total p50 0.919</sub> | -52.3% (-0.657) | 150% AND 2 ms | 🟢 |
| 8 | 1.322<br><sub>context: p90 1.801 · p95 2.076 · p99 2.375 · 4726 op/s · total p50 1.551</sub> | 0.607<br><sub>context: p90 0.680 · p95 0.707 · p99 0.746 · 8091 op/s · total p50 0.899</sub> | -54.1% (-0.715) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.939<br><sub>context: p90 1.993 · p95 2.004 · p99 2.028 · 437 op/s · total p50 2.275</sub> | 0.631<br><sub>context: p90 0.651 · p95 0.658 · p99 0.667 · 1061 op/s · total p50 0.944</sub> | -67.5% (-1.309) | 150% AND 2 ms | 🟢 |
| 8 | 2.007<br><sub>context: p90 2.392 · p95 2.588 · p99 2.902 · 3194 op/s · total p50 2.273</sub> | 0.637<br><sub>context: p90 0.726 · p95 0.757 · p99 0.827 · 7722 op/s · total p50 0.939</sub> | -68.2% (-1.369) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.888<br><sub>context: p90 1.975 · p95 1.997 · p99 2.029 · 444 op/s · total p50 2.241</sub> | 1.111<br><sub>context: p90 1.134 · p95 1.143 · p99 1.155 · 657 op/s · total p50 1.518</sub> | -41.2% (-0.777) | 150% AND 2 ms | 🟢 |
| 8 | 2.041<br><sub>context: p90 2.739 · p95 2.993 · p99 3.384 · 3217 op/s · total p50 2.287</sub> | 1.611<br><sub>context: p90 2.347 · p95 2.489 · p99 2.880 · 3785 op/s · total p50 1.992</sub> | -21.1% (-0.431) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.503<br><sub>context: p90 2.570 · p95 2.584 · p99 2.613 · 348 op/s · total p50 2.872</sub> | 1.200<br><sub>context: p90 1.228 · p95 1.235 · p99 1.248 · 614 op/s · total p50 1.620</sub> | -52.1% (-1.303) | 150% AND 2 ms | 🟢 |
| 8 | 2.866<br><sub>context: p90 3.769 · p95 4.077 · p99 4.640 · 2387 op/s · total p50 3.137</sub> | 1.205<br><sub>context: p90 1.340 · p95 1.385 · p99 1.446 · 4730 op/s · total p50 1.556</sub> | -57.9% (-1.661) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.126 · p95 0.130 · p99 0.142 · 3718 op/s · total p50 0.259</sub> | 0.023<br><sub>context: p90 0.040 · p95 0.041 · p99 0.050 · 5652 op/s · total p50 0.159</sub> | -77.4% (-0.080) | 150% AND 2 ms | 🟢 |
| 8 | 0.113<br><sub>context: p90 0.165 · p95 0.187 · p99 0.239 · 31931 op/s · total p50 0.236</sub> | 0.017<br><sub>context: p90 0.024 · p95 0.026 · p99 0.032 · 35517 op/s · total p50 0.218</sub> | -84.9% (-0.096) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.258<br><sub>context: p90 0.283 · p95 0.292 · p99 0.311 · 2072 op/s · total p50 0.462</sub> | 0.066<br><sub>context: p90 0.095 · p95 0.100 · p99 0.107 · 2617 op/s · total p50 0.373</sub> | -74.3% (-0.192) | 150% AND 2 ms | 🟢 |
| 8 | 0.273<br><sub>context: p90 0.358 · p95 0.388 · p99 0.469 · 16765 op/s · total p50 0.451</sub> | 0.054<br><sub>context: p90 0.083 · p95 0.090 · p99 0.103 · 22479 op/s · total p50 0.335</sub> | -80.3% (-0.219) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.306 · p95 0.311 · p99 0.321 · 1987 op/s · total p50 0.486</sub> | 0.068<br><sub>context: p90 0.098 · p95 0.102 · p99 0.106 · 2419 op/s · total p50 0.402</sub> | -76.1% (-0.217) | 150% AND 2 ms | 🟢 |
| 8 | 0.298<br><sub>context: p90 0.387 · p95 0.425 · p99 0.505 · 15798 op/s · total p50 0.475</sub> | 0.055<br><sub>context: p90 0.086 · p95 0.093 · p99 0.103 · 22311 op/s · total p50 0.345</sub> | -81.5% (-0.242) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.304<br><sub>context: p90 0.329 · p95 0.336 · p99 0.353 · 1838 op/s · total p50 0.529</sub> | 0.116<br><sub>context: p90 0.149 · p95 0.161 · p99 0.172 · 2302 op/s · total p50 0.430</sub> | -61.7% (-0.187) | 150% AND 2 ms | 🟢 |
| 8 | 0.321<br><sub>context: p90 0.411 · p95 0.447 · p99 0.543 · 13953 op/s · total p50 0.541</sub> | 0.109<br><sub>context: p90 0.148 · p95 0.160 · p99 0.186 · 16721 op/s · total p50 0.460</sub> | -66.0% (-0.212) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.358 · p95 0.364 · p99 0.371 · 1663 op/s · total p50 0.595</sub> | 0.130<br><sub>context: p90 0.160 · p95 0.165 · p99 0.176 · 1922 op/s · total p50 0.516</sub> | -59.7% (-0.193) | 150% AND 2 ms | 🟢 |
| 8 | 0.357<br><sub>context: p90 0.452 · p95 0.490 · p99 0.577 · 13502 op/s · total p50 0.567</sub> | 0.114<br><sub>context: p90 0.151 · p95 0.162 · p99 0.181 · 16350 op/s · total p50 0.471</sub> | -68.1% (-0.243) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.385<br><sub>context: p90 0.441 · p95 0.456 · p99 0.504 · 1282 op/s · total p50 0.768</sub> | 0.162<br><sub>context: p90 0.198 · p95 0.208 · p99 0.229 · 1525 op/s · total p50 0.648</sub> | -57.9% (-0.223) | 150% AND 2 ms | 🟢 |
| 8 | 0.406<br><sub>context: p90 0.507 · p95 0.544 · p99 0.602 · 10238 op/s · total p50 0.754</sub> | 0.162<br><sub>context: p90 0.210 · p95 0.222 · p99 0.248 · 11357 op/s · total p50 0.680</sub> | -60.2% (-0.244) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.408<br><sub>context: p90 0.480 · p95 0.497 · p99 0.534 · 1234 op/s · total p50 0.800</sub> | 0.170<br><sub>context: p90 0.218 · p95 0.228 · p99 0.257 · 1443 op/s · total p50 0.673</sub> | -58.2% (-0.238) | 150% AND 2 ms | 🟢 |
| 8 | 0.439<br><sub>context: p90 0.558 · p95 0.596 · p99 0.673 · 9732 op/s · total p50 0.792</sub> | 0.161<br><sub>context: p90 0.207 · p95 0.221 · p99 0.254 · 11370 op/s · total p50 0.683</sub> | -63.4% (-0.278) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.623<br><sub>context: p90 0.770 · p95 0.811 · p99 0.865 · 695 op/s · total p50 1.429</sub> | 0.304<br><sub>context: p90 0.375 · p95 0.401 · p99 0.420 · 814 op/s · total p50 1.222</sub> | -51.2% (-0.319) | 150% AND 2 ms | 🟢 |
| 8 | 0.688<br><sub>context: p90 0.907 · p95 0.989 · p99 1.155 · 4841 op/s · total p50 1.581</sub> | 0.310<br><sub>context: p90 0.407 · p95 0.436 · p99 0.481 · 5751 op/s · total p50 1.321</sub> | -55.0% (-0.378) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.688<br><sub>context: p90 0.858 · p95 0.939 · p99 1.028 · 656 op/s · total p50 1.472</sub> | 0.322<br><sub>context: p90 0.400 · p95 0.422 · p99 0.476 · 765 op/s · total p50 1.262</sub> | -53.2% (-0.366) | 150% AND 2 ms | 🟢 |
| 8 | 0.754<br><sub>context: p90 1.032 · p95 1.120 · p99 1.308 · 4604 op/s · total p50 1.664</sub> | 0.318<br><sub>context: p90 0.424 · p95 0.457 · p99 0.522 · 5733 op/s · total p50 1.317</sub> | -57.9% (-0.437) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.524<br><sub>context: p90 0.601 · p95 0.630 · p99 0.676 · 1293 op/s · total p50 0.767</sub> | 0.370<br><sub>context: p90 0.538 · p95 0.572 · p99 0.663 · 1338 op/s · total p50 0.738</sub> | -29.4% (-0.154) | 150% AND 2 ms | 🟢 |
| 8 | 0.558<br><sub>context: p90 0.711 · p95 0.764 · p99 0.864 · 10047 op/s · total p50 0.757</sub> | 0.428<br><sub>context: p90 0.651 · p95 0.700 · p99 0.802 · 9029 op/s · total p50 0.825</sub> | -23.2% (-0.130) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.356 · p95 0.359 · p99 0.369 · 1672 op/s · total p50 0.596</sub> | 0.102<br><sub>context: p90 0.139 · p95 0.144 · p99 0.153 · 2500 op/s · total p50 0.390</sub> | -69.1% (-0.228) | 150% AND 2 ms | 🟢 |
| 8 | 0.367<br><sub>context: p90 0.466 · p95 0.502 · p99 0.605 · 13961 op/s · total p50 0.540</sub> | 0.088<br><sub>context: p90 0.120 · p95 0.129 · p99 0.147 · 19126 op/s · total p50 0.402</sub> | -76.1% (-0.279) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.203 · p95 0.207 · p99 0.215 · 2274 op/s · total p50 0.431</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 5148 op/s · total p50 0.178</sub> | -98.4% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.193<br><sub>context: p90 0.278 · p95 0.321 · p99 0.404 · 21746 op/s · total p50 0.335</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 34292 op/s · total p50 0.222</sub> | -98.7% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.129<br><sub>context: p90 0.146 · p95 0.150 · p99 0.161 · 3244 op/s · total p50 0.300</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 6204 op/s · total p50 0.147</sub> | -97.7% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.146<br><sub>context: p90 0.232 · p95 0.264 · p99 0.317 · 23818 op/s · total p50 0.297</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.004 · 37789 op/s · total p50 0.202</sub> | -98.4% (-0.144) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.378<br><sub>context: p90 0.413 · p95 0.422 · p99 0.433 · 1604 op/s · total p50 0.613</sub> | 0.085<br><sub>context: p90 0.114 · p95 0.124 · p99 0.135 · 2067 op/s · total p50 0.473</sub> | -77.6% (-0.293) | 150% AND 2 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.548 · p95 0.589 · p99 0.703 · 12028 op/s · total p50 0.641</sub> | 0.077<br><sub>context: p90 0.109 · p95 0.117 · p99 0.132 · 16827 op/s · total p50 0.454</sub> | -82.4% (-0.361) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.701<br><sub>context: p90 0.974 · p95 1.088 · p99 1.228 · 1034 op/s · total p50 0.940</sub> | 1.192<br><sub>context: p90 1.907 · p95 2.105 · p99 2.413 · 599 op/s · total p50 1.614</sub> | +70.0% (+0.491) | 150% AND 2 ms | 🟢 |
| 8 | 0.743<br><sub>context: p90 1.135 · p95 1.254 · p99 1.530 · 7167 op/s · total p50 1.032</sub> | 1.762<br><sub>context: p90 2.949 · p95 3.288 · p99 3.871 · 3405 op/s · total p50 2.219</sub> | +137.0% (+1.018) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.292<br><sub>context: p90 3.620 · p95 4.156 · p99 4.664 · 364 op/s · total p50 2.647</sub> | 4.654<br><sub>context: p90 7.208 · p95 7.992 · p99 8.818 · 185 op/s · total p50 5.198</sub> | +103.1% (+2.362) | 150% AND 2 ms | 🟢 |
| 8 | 2.402<br><sub>context: p90 4.098 · p95 4.500 · p99 5.369 · 2814 op/s · total p50 2.664</sub> | 7.908<br><sub>context: p90 12.150 · p95 13.412 · p99 14.840 · 929 op/s · total p50 8.369</sub> | +229.3% (+5.506) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.200 · p95 0.205 · p99 0.213 · 2487 op/s · total p50 0.397</sub> | 0.031<br><sub>context: p90 0.038 · p95 0.040 · p99 0.047 · 3666 op/s · total p50 0.255</sub> | -83.1% (-0.150) | 150% AND 2 ms | 🟢 |
| 8 | 0.184<br><sub>context: p90 0.241 · p95 0.261 · p99 0.309 · 21456 op/s · total p50 0.355</sub> | 0.020<br><sub>context: p90 0.027 · p95 0.032 · p99 0.038 · 26105 op/s · total p50 0.285</sub> | -89.2% (-0.164) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.150<br><sub>context: p90 0.192 · p95 0.196 · p99 0.208 · 2962 op/s · total p50 0.336</sub> | 0.027<br><sub>context: p90 0.037 · p95 0.040 · p99 0.043 · 4194 op/s · total p50 0.242</sub> | -82.1% (-0.123) | 150% AND 2 ms | 🟢 |
| 8 | 0.184<br><sub>context: p90 0.238 · p95 0.263 · p99 0.308 · 21684 op/s · total p50 0.350</sub> | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.036 · 27617 op/s · total p50 0.270</sub> | -89.7% (-0.166) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.209 · p95 0.211 · p99 0.228 · 2179 op/s · total p50 0.449</sub> | 0.013<br><sub>context: p90 0.018 · p95 0.019 · p99 0.025 · 3056 op/s · total p50 0.309</sub> | -93.0% (-0.173) | 150% AND 2 ms | 🟢 |
| 8 | 0.200<br><sub>context: p90 0.259 · p95 0.279 · p99 0.335 · 15559 op/s · total p50 0.491</sub> | 0.012<br><sub>context: p90 0.016 · p95 0.017 · p99 0.021 · 20566 op/s · total p50 0.365</sub> | -94.2% (-0.188) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.119<br><sub>context: p90 0.166 · p95 0.171 · p99 0.178 · 3903 op/s · total p50 0.241</sub> | 0.006<br><sub>context: p90 0.013 · p95 0.014 · p99 0.015 · 6019 op/s · total p50 0.144</sub> | -94.8% (-0.112) | 150% AND 2 ms | 🟢 |
| 8 | 0.155<br><sub>context: p90 0.208 · p95 0.230 · p99 0.276 · 26292 op/s · total p50 0.288</sub> | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.011 · 34942 op/s · total p50 0.218</sub> | -96.0% (-0.149) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.244<br><sub>context: p90 0.287 · p95 0.293 · p99 0.300 · 2279 op/s · total p50 0.430</sub> | 0.146<br><sub>context: p90 0.187 · p95 0.196 · p99 0.228 · 2078 op/s · total p50 0.466</sub> | -40.5% (-0.099) | 150% AND 2 ms | 🟢 |
| 8 | 0.284<br><sub>context: p90 0.360 · p95 0.382 · p99 0.435 · 16186 op/s · total p50 0.470</sub> | 0.139<br><sub>context: p90 0.184 · p95 0.197 · p99 0.228 · 20123 op/s · total p50 0.380</sub> | -51.0% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.188<br><sub>context: p90 0.232 · p95 0.238 · p99 0.247 · 2659 op/s · total p50 0.373</sub> | 0.078<br><sub>context: p90 0.108 · p95 0.117 · p99 0.133 · 2588 op/s · total p50 0.375</sub> | -58.3% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.221<br><sub>context: p90 0.289 · p95 0.315 · p99 0.362 · 20569 op/s · total p50 0.371</sub> | 0.072<br><sub>context: p90 0.106 · p95 0.113 · p99 0.136 · 24176 op/s · total p50 0.318</sub> | -67.4% (-0.149) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.295<br><sub>context: p90 0.315 · p95 0.323 · p99 0.332 · 1856 op/s · total p50 0.531</sub> | 0.122<br><sub>context: p90 0.154 · p95 0.165 · p99 0.183 · 1972 op/s · total p50 0.497</sub> | -58.6% (-0.173) | 150% AND 2 ms | 🟢 |
| 8 | 0.313<br><sub>context: p90 0.411 · p95 0.449 · p99 0.507 · 13865 op/s · total p50 0.543</sub> | 0.108<br><sub>context: p90 0.147 · p95 0.156 · p99 0.174 · 16144 op/s · total p50 0.474</sub> | -65.5% (-0.205) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.324 · p95 0.328 · p99 0.337 · 1460 op/s · total p50 0.684</sub> | 0.120<br><sub>context: p90 0.154 · p95 0.157 · p99 0.166 · 1805 op/s · total p50 0.548</sub> | -59.7% (-0.178) | 150% AND 2 ms | 🟢 |
| 8 | 0.308<br><sub>context: p90 0.389 · p95 0.421 · p99 0.482 · 11368 op/s · total p50 0.672</sub> | 0.112<br><sub>context: p90 0.150 · p95 0.160 · p99 0.180 · 12673 op/s · total p50 0.600</sub> | -63.7% (-0.196) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.333<br><sub>context: p90 0.356 · p95 0.362 · p99 0.382 · 1345 op/s · total p50 0.744</sub> | 0.131<br><sub>context: p90 0.161 · p95 0.168 · p99 0.177 · 1527 op/s · total p50 0.653</sub> | -60.6% (-0.202) | 150% AND 2 ms | 🟢 |
| 8 | 0.351<br><sub>context: p90 0.442 · p95 0.479 · p99 0.554 · 10083 op/s · total p50 0.714</sub> | 0.122<br><sub>context: p90 0.162 · p95 0.174 · p99 0.197 · 11605 op/s · total p50 0.656</sub> | -65.2% (-0.229) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.353 · p95 0.358 · p99 0.366 · 1572 op/s · total p50 0.627</sub> | 0.126<br><sub>context: p90 0.157 · p95 0.164 · p99 0.174 · 1880 op/s · total p50 0.526</sub> | -61.8% (-0.204) | 150% AND 2 ms | 🟢 |
| 8 | 0.348<br><sub>context: p90 0.445 · p95 0.475 · p99 0.535 · 13293 op/s · total p50 0.575</sub> | 0.106<br><sub>context: p90 0.141 · p95 0.154 · p99 0.172 · 16397 op/s · total p50 0.470</sub> | -69.4% (-0.241) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.305<br><sub>context: p90 0.340 · p95 0.344 · p99 0.357 · 1932 op/s · total p50 0.511</sub> | 0.104<br><sub>context: p90 0.138 · p95 0.141 · p99 0.154 · 2274 op/s · total p50 0.428</sub> | -65.7% (-0.201) | 150% AND 2 ms | 🟢 |
| 8 | 0.349<br><sub>context: p90 0.443 · p95 0.482 · p99 0.574 · 14181 op/s · total p50 0.536</sub> | 0.089<br><sub>context: p90 0.128 · p95 0.141 · p99 0.173 · 18145 op/s · total p50 0.406</sub> | -74.4% (-0.260) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 6.271<br><sub>context: p90 6.371 · p95 6.405 · p99 6.469 · 52 op/s · total p50 19.060</sub> | 2.937<br><sub>context: p90 2.978 · p95 2.996 · p99 3.050 · 64 op/s · total p50 15.687</sub> | -53.2% (-3.333) | 150% AND 2 ms | 🟢 |
| 8 | 8.091<br><sub>context: p90 9.893 · p95 10.192 · p99 10.672 · 306 op/s · total p50 24.178</sub> | 3.725<br><sub>context: p90 4.050 · p95 4.078 · p99 4.171 · 385 op/s · total p50 18.581</sub> | -54.0% (-4.366) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.454 · p95 0.470 · p99 0.493 · 1443 op/s · total p50 0.689</sub> | 0.133<br><sub>context: p90 0.164 · p95 0.171 · p99 0.185 · 1973 op/s · total p50 0.509</sub> | -68.3% (-0.286) | 150% AND 2 ms | 🟢 |
| 8 | 0.464<br><sub>context: p90 0.591 · p95 0.658 · p99 0.770 · 11694 op/s · total p50 0.649</sub> | 0.109<br><sub>context: p90 0.145 · p95 0.154 · p99 0.175 · 17934 op/s · total p50 0.430</sub> | -76.6% (-0.355) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.610<br><sub>context: p90 0.714 · p95 0.750 · p99 0.780 · 583 op/s · total p50 1.689</sub> | 0.283<br><sub>context: p90 0.348 · p95 0.354 · p99 0.380 · 670 op/s · total p50 1.466</sub> | -53.7% (-0.328) | 150% AND 2 ms | 🟢 |
| 8 | 0.650<br><sub>context: p90 0.822 · p95 0.883 · p99 0.972 · 4201 op/s · total p50 1.838</sub> | 0.294<br><sub>context: p90 0.388 · p95 0.416 · p99 0.460 · 4794 op/s · total p50 1.619</sub> | -54.8% (-0.356) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.325<br><sub>context: p90 0.350 · p95 0.358 · p99 0.363 · 1570 op/s · total p50 0.625</sub> | 0.123<br><sub>context: p90 0.156 · p95 0.163 · p99 0.177 · 2075 op/s · total p50 0.477</sub> | -62.1% (-0.202) | 150% AND 2 ms | 🟢 |
| 8 | 0.352<br><sub>context: p90 0.450 · p95 0.496 · p99 0.610 · 11947 op/s · total p50 0.612</sub> | 0.112<br><sub>context: p90 0.145 · p95 0.157 · p99 0.184 · 15667 op/s · total p50 0.491</sub> | -68.3% (-0.240) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.093<br><sub>context: p90 2.675 · p95 2.725 · p99 2.833 · 412 op/s · total p50 2.503</sub> | 0.138<br><sub>context: p90 0.188 · p95 0.200 · p99 0.229 · 2089 op/s · total p50 0.473</sub> | -93.4% (-1.955) | 150% AND 2 ms | 🟢 |
| 8 | 2.104<br><sub>context: p90 2.743 · p95 2.893 · p99 3.409 · 3369 op/s · total p50 2.353</sub> | 0.124<br><sub>context: p90 0.175 · p95 0.193 · p99 0.229 · 17215 op/s · total p50 0.438</sub> | -94.1% (-1.980) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.108<br><sub>context: p90 2.728 · p95 2.783 · p99 2.864 · 413 op/s · total p50 2.496</sub> | 0.140<br><sub>context: p90 0.201 · p95 0.215 · p99 0.242 · 1940 op/s · total p50 0.510</sub> | -93.3% (-1.967) | 150% AND 2 ms | 🟢 |
| 8 | 2.161<br><sub>context: p90 2.854 · p95 2.984 · p99 3.536 · 3350 op/s · total p50 2.408</sub> | 0.131<br><sub>context: p90 0.191 · p95 0.211 · p99 0.244 · 15536 op/s · total p50 0.472</sub> | -93.9% (-2.031) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.155<br><sub>context: p90 0.176 · p95 0.189 · p99 0.203 · 2978 op/s · total p50 0.333</sub> | 0.042<br><sub>context: p90 0.073 · p95 0.075 · p99 0.081 · 3471 op/s · total p50 0.271</sub> | -72.9% (-0.113) | 150% AND 2 ms | 🟢 |
| 8 | 0.151<br><sub>context: p90 0.198 · p95 0.217 · p99 0.261 · 25578 op/s · total p50 0.294</sub> | 0.038<br><sub>context: p90 0.071 · p95 0.075 · p99 0.084 · 27587 op/s · total p50 0.275</sub> | -75.1% (-0.114) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.253 · p95 0.260 · p99 0.270 · 2260 op/s · total p50 0.432</sub> | 0.006<br><sub>context: p90 0.009 · p95 0.009 · p99 0.013 · 4798 op/s · total p50 0.193</sub> | -97.5% (-0.221) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.250<br><sub>context: p90 0.319 · p95 0.342 · p99 0.429 · 18520 op/s · total p50 0.408</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 31029 op/s · total p50 0.244</sub> | -98.4% (-0.246) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.229<br><sub>context: p90 0.253 · p95 0.260 · p99 0.270 · 2211 op/s · total p50 0.449</sub> | 0.055<br><sub>context: p90 0.079 · p95 0.086 · p99 0.100 · 2809 op/s · total p50 0.346</sub> | -75.8% (-0.173) | 150% AND 2 ms | 🟢 |
| 8 | 0.269<br><sub>context: p90 0.355 · p95 0.393 · p99 0.474 · 17861 op/s · total p50 0.424</sub> | 0.048<br><sub>context: p90 0.078 · p95 0.084 · p99 0.091 · 24439 op/s · total p50 0.309</sub> | -82.3% (-0.221) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.244<br><sub>context: p90 0.269 · p95 0.276 · p99 0.306 · 2234 op/s · total p50 0.432</sub> | 0.073<br><sub>context: p90 0.132 · p95 0.137 · p99 0.152 · 2783 op/s · total p50 0.349</sub> | -70.3% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 0.285<br><sub>context: p90 0.372 · p95 0.408 · p99 0.503 · 17497 op/s · total p50 0.435</sub> | 0.069<br><sub>context: p90 0.134 · p95 0.143 · p99 0.153 · 23028 op/s · total p50 0.335</sub> | -75.7% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.242 · p95 0.247 · p99 0.256 · 2202 op/s · total p50 0.445</sub> | 0.052<br><sub>context: p90 0.083 · p95 0.092 · p99 0.109 · 2914 op/s · total p50 0.312</sub> | -77.0% (-0.174) | 150% AND 2 ms | 🟢 |
| 8 | 0.236<br><sub>context: p90 0.311 · p95 0.337 · p99 0.402 · 19664 op/s · total p50 0.386</sub> | 0.040<br><sub>context: p90 0.075 · p95 0.078 · p99 0.088 · 26973 op/s · total p50 0.281</sub> | -83.1% (-0.196) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.140<br><sub>context: p90 1.200 · p95 1.210 · p99 1.226 · 615 op/s · total p50 1.613</sub> | 0.607<br><sub>context: p90 0.644 · p95 0.655 · p99 0.668 · 856 op/s · total p50 1.178</sub> | -46.7% (-0.533) | 150% AND 2 ms | 🟢 |
| 8 | 1.224<br><sub>context: p90 1.496 · p95 1.573 · p99 1.863 · 4353 op/s · total p50 1.707</sub> | 0.614<br><sub>context: p90 0.717 · p95 0.749 · p99 0.796 · 6050 op/s · total p50 1.258</sub> | -49.8% (-0.610) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.148<br><sub>context: p90 1.211 · p95 1.227 · p99 1.252 · 706 op/s · total p50 1.415</sub> | 0.602<br><sub>context: p90 0.634 · p95 0.651 · p99 0.679 · 982 op/s · total p50 1.008</sub> | -47.6% (-0.546) | 150% AND 2 ms | 🟢 |
| 8 | 1.182<br><sub>context: p90 1.377 · p95 1.471 · p99 1.692 · 4983 op/s · total p50 1.441</sub> | 0.597<br><sub>context: p90 0.681 · p95 0.709 · p99 0.765 · 7676 op/s · total p50 0.981</sub> | -49.5% (-0.585) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.258<br><sub>context: p90 0.282 · p95 0.290 · p99 0.299 · 1910 op/s · total p50 0.520</sub> | 0.077<br><sub>context: p90 0.102 · p95 0.111 · p99 0.121 · 2591 op/s · total p50 0.376</sub> | -70.1% (-0.181) | 150% AND 2 ms | 🟢 |
| 8 | 0.258<br><sub>context: p90 0.324 · p95 0.354 · p99 0.404 · 16431 op/s · total p50 0.466</sub> | 0.066<br><sub>context: p90 0.096 · p95 0.105 · p99 0.120 · 20456 op/s · total p50 0.371</sub> | -74.5% (-0.192) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.395<br><sub>context: p90 0.466 · p95 0.485 · p99 0.521 · 1478 op/s · total p50 0.660</sub> | 0.194<br><sub>context: p90 0.284 · p95 0.317 · p99 0.352 · 1715 op/s · total p50 0.577</sub> | -50.9% (-0.201) | 150% AND 2 ms | 🟢 |
| 8 | 0.400<br><sub>context: p90 0.529 · p95 0.576 · p99 0.682 · 12599 op/s · total p50 0.606</sub> | 0.174<br><sub>context: p90 0.262 · p95 0.287 · p99 0.345 · 16164 op/s · total p50 0.468</sub> | -56.5% (-0.226) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.199<br><sub>context: p90 0.230 · p95 0.237 · p99 0.248 · 2369 op/s · total p50 0.415</sub> | 0.045<br><sub>context: p90 0.050 · p95 0.052 · p99 0.057 · 3245 op/s · total p50 0.301</sub> | -77.3% (-0.154) | 150% AND 2 ms | 🟢 |
| 8 | 0.224<br><sub>context: p90 0.288 · p95 0.312 · p99 0.384 · 19075 op/s · total p50 0.398</sub> | 0.027<br><sub>context: p90 0.038 · p95 0.044 · p99 0.053 · 25429 op/s · total p50 0.302</sub> | -88.1% (-0.197) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.143<br><sub>context: p90 0.176 · p95 0.183 · p99 0.198 · 3146 op/s · total p50 0.303</sub> | 0.038<br><sub>context: p90 0.064 · p95 0.069 · p99 0.077 · 4300 op/s · total p50 0.216</sub> | -73.8% (-0.106) | 150% AND 2 ms | 🟢 |
| 8 | 0.159<br><sub>context: p90 0.222 · p95 0.250 · p99 0.294 · 23282 op/s · total p50 0.316</sub> | 0.037<br><sub>context: p90 0.069 · p95 0.073 · p99 0.078 · 29148 op/s · total p50 0.261</sub> | -76.9% (-0.123) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.140<br><sub>context: p90 0.175 · p95 0.181 · p99 0.189 · 3105 op/s · total p50 0.318</sub> | 0.041<br><sub>context: p90 0.071 · p95 0.076 · p99 0.083 · 4032 op/s · total p50 0.232</sub> | -71.1% (-0.100) | 150% AND 2 ms | 🟢 |
| 8 | 0.159<br><sub>context: p90 0.218 · p95 0.247 · p99 0.297 · 23790 op/s · total p50 0.317</sub> | 0.037<br><sub>context: p90 0.070 · p95 0.074 · p99 0.084 · 27900 op/s · total p50 0.267</sub> | -76.4% (-0.121) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.864<br><sub>context: p90 0.901 · p95 0.907 · p99 0.927 · 856 op/s · total p50 1.161</sub> | 0.205<br><sub>context: p90 0.223 · p95 0.231 · p99 0.247 · 2152 op/s · total p50 0.450</sub> | -76.2% (-0.659) | 150% AND 2 ms | 🟢 |
| 8 | 0.894<br><sub>context: p90 1.192 · p95 1.394 · p99 1.621 · 5988 op/s · total p50 1.153</sub> | 0.216<br><sub>context: p90 0.255 · p95 0.262 · p99 0.279 · 16907 op/s · total p50 0.452</sub> | -75.9% (-0.678) | 150% AND 2 ms | 🟢 |

</details>
