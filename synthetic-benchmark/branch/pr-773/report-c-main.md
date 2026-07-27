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
| 1 | 1.241<br><sub>context: p90 1.289 · p95 1.311 · p99 1.365 · 748 op/s · total p50 5.343</sub> | 0.595<br><sub>context: p90 0.631 · p95 0.643 · p99 0.655 · 1377 op/s · total p50 2.884</sub> | -52.1% (-0.647) | 150% AND 2 ms | 🟢 |
| 8 | 1.373<br><sub>context: p90 1.946 · p95 2.193 · p99 2.515 · 4895 op/s · total p50 6.112</sub> | 0.634<br><sub>context: p90 0.702 · p95 0.727 · p99 0.780 · 8885 op/s · total p50 3.507</sub> | -53.8% (-0.738) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.923<br><sub>context: p90 1.971 · p95 1.985 · p99 2.012 · 490 op/s · total p50 8.167</sub> | 0.629<br><sub>context: p90 0.665 · p95 0.684 · p99 0.717 · 1313 op/s · total p50 3.044</sub> | -67.3% (-1.293) | 150% AND 2 ms | 🟢 |
| 8 | 2.078<br><sub>context: p90 2.523 · p95 2.741 · p99 3.124 · 3398 op/s · total p50 8.915</sub> | 0.665<br><sub>context: p90 0.740 · p95 0.771 · p99 0.834 · 8654 op/s · total p50 3.517</sub> | -68.0% (-1.412) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.889<br><sub>context: p90 1.975 · p95 1.999 · p99 2.059 · 494 op/s · total p50 8.080</sub> | 1.105<br><sub>context: p90 1.142 · p95 1.153 · p99 1.176 · 790 op/s · total p50 5.038</sub> | -41.5% (-0.784) | 150% AND 2 ms | 🟢 |
| 8 | 2.107<br><sub>context: p90 3.026 · p95 3.302 · p99 3.812 · 3230 op/s · total p50 9.295</sub> | 1.962<br><sub>context: p90 2.648 · p95 2.833 · p99 3.164 · 3571 op/s · total p50 8.654</sub> | -6.9% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.495<br><sub>context: p90 2.556 · p95 2.596 · p99 2.686 · 381 op/s · total p50 10.479</sub> | 1.179<br><sub>context: p90 1.222 · p95 1.236 · p99 1.249 · 753 op/s · total p50 5.298</sub> | -52.7% (-1.316) | 150% AND 2 ms | 🟢 |
| 8 | 2.819<br><sub>context: p90 3.751 · p95 4.030 · p99 4.831 · 2532 op/s · total p50 12.075</sub> | 1.225<br><sub>context: p90 1.305 · p95 1.347 · p99 1.419 · 4870 op/s · total p50 6.228</sub> | -56.6% (-1.595) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.100<br><sub>context: p90 0.134 · p95 0.140 · p99 0.146 · 6388 op/s · total p50 0.620</sub> | 0.012<br><sub>context: p90 0.020 · p95 0.023 · p99 0.029 · 12872 op/s · total p50 0.274</sub> | -87.8% (-0.088) | 150% AND 2 ms | 🟢 |
| 8 | 0.196<br><sub>context: p90 0.386 · p95 0.454 · p99 0.609 · 27031 op/s · total p50 1.087</sub> | 0.018<br><sub>context: p90 0.026 · p95 0.029 · p99 0.035 · 50305 op/s · total p50 0.468</sub> | -91.0% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.218<br><sub>context: p90 0.271 · p95 0.286 · p99 0.308 · 3431 op/s · total p50 1.141</sub> | 0.051<br><sub>context: p90 0.084 · p95 0.090 · p99 0.096 · 6288 op/s · total p50 0.623</sub> | -76.5% (-0.167) | 150% AND 2 ms | 🟢 |
| 8 | 0.321<br><sub>context: p90 0.492 · p95 0.557 · p99 0.681 · 17296 op/s · total p50 1.730</sub> | 0.060<br><sub>context: p90 0.096 · p95 0.103 · p99 0.119 · 31753 op/s · total p50 0.925</sub> | -81.3% (-0.261) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.292<br><sub>context: p90 0.333 · p95 0.342 · p99 0.358 · 2536 op/s · total p50 1.570</sub> | 0.052<br><sub>context: p90 0.079 · p95 0.085 · p99 0.100 · 5101 op/s · total p50 0.771</sub> | -82.1% (-0.240) | 150% AND 2 ms | 🟢 |
| 8 | 0.352<br><sub>context: p90 0.526 · p95 0.607 · p99 0.734 · 16323 op/s · total p50 1.859</sub> | 0.062<br><sub>context: p90 0.096 · p95 0.102 · p99 0.116 · 29808 op/s · total p50 1.008</sub> | -82.5% (-0.290) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.297<br><sub>context: p90 0.329 · p95 0.338 · p99 0.352 · 2601 op/s · total p50 1.493</sub> | 0.103<br><sub>context: p90 0.134 · p95 0.138 · p99 0.160 · 3973 op/s · total p50 0.980</sub> | -65.4% (-0.195) | 150% AND 2 ms | 🟢 |
| 8 | 0.428<br><sub>context: p90 0.679 · p95 0.789 · p99 0.964 · 12988 op/s · total p50 2.341</sub> | 0.112<br><sub>context: p90 0.147 · p95 0.160 · p99 0.188 · 25015 op/s · total p50 1.191</sub> | -73.9% (-0.316) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.357 · p95 0.370 · p99 0.394 · 2543 op/s · total p50 1.553</sub> | 0.103<br><sub>context: p90 0.144 · p95 0.158 · p99 0.175 · 3747 op/s · total p50 1.034</sub> | -67.2% (-0.212) | 150% AND 2 ms | 🟢 |
| 8 | 0.402<br><sub>context: p90 0.585 · p95 0.666 · p99 0.797 · 14320 op/s · total p50 2.118</sub> | 0.120<br><sub>context: p90 0.158 · p95 0.166 · p99 0.184 · 23170 op/s · total p50 1.291</sub> | -70.1% (-0.282) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.345<br><sub>context: p90 0.424 · p95 0.433 · p99 0.468 · 2363 op/s · total p50 1.675</sub> | 0.134<br><sub>context: p90 0.182 · p95 0.193 · p99 0.208 · 3388 op/s · total p50 1.166</sub> | -61.2% (-0.211) | 150% AND 2 ms | 🟢 |
| 8 | 0.455<br><sub>context: p90 0.641 · p95 0.713 · p99 0.868 · 12271 op/s · total p50 2.474</sub> | 0.162<br><sub>context: p90 0.213 · p95 0.230 · p99 0.255 · 14517 op/s · total p50 2.059</sub> | -64.5% (-0.294) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.355<br><sub>context: p90 0.452 · p95 0.471 · p99 0.525 · 2192 op/s · total p50 1.812</sub> | 0.142<br><sub>context: p90 0.192 · p95 0.216 · p99 0.228 · 2991 op/s · total p50 1.320</sub> | -59.9% (-0.213) | 150% AND 2 ms | 🟢 |
| 8 | 0.503<br><sub>context: p90 0.700 · p95 0.774 · p99 0.947 · 10761 op/s · total p50 2.773</sub> | 0.170<br><sub>context: p90 0.231 · p95 0.251 · p99 0.288 · 12895 op/s · total p50 2.347</sub> | -66.1% (-0.333) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.589<br><sub>context: p90 0.781 · p95 0.818 · p99 0.910 · 1380 op/s · total p50 2.842</sub> | 0.257<br><sub>context: p90 0.345 · p95 0.368 · p99 0.416 · 1797 op/s · total p50 2.049</sub> | -56.3% (-0.331) | 150% AND 2 ms | 🟢 |
| 8 | 0.658<br><sub>context: p90 0.893 · p95 0.977 · p99 1.123 · 3793 op/s · total p50 8.029</sub> | 0.282<br><sub>context: p90 0.379 · p95 0.415 · p99 0.465 · 3855 op/s · total p50 8.130</sub> | -57.2% (-0.376) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.678<br><sub>context: p90 0.824 · p95 0.871 · p99 0.987 · 1225 op/s · total p50 3.266</sub> | 0.282<br><sub>context: p90 0.354 · p95 0.374 · p99 0.407 · 1751 op/s · total p50 2.199</sub> | -58.4% (-0.396) | 150% AND 2 ms | 🟢 |
| 8 | 0.724<br><sub>context: p90 0.975 · p95 1.050 · p99 1.201 · 3687 op/s · total p50 8.269</sub> | 0.309<br><sub>context: p90 0.409 · p95 0.436 · p99 0.486 · 3800 op/s · total p50 8.010</sub> | -57.4% (-0.416) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.507<br><sub>context: p90 0.578 · p95 0.601 · p99 0.641 · 1656 op/s · total p50 2.384</sub> | 0.335<br><sub>context: p90 0.509 · p95 0.568 · p99 0.660 · 1949 op/s · total p50 2.037</sub> | -33.9% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 0.628<br><sub>context: p90 0.854 · p95 0.940 · p99 1.137 · 9267 op/s · total p50 3.335</sub> | 0.433<br><sub>context: p90 0.667 · p95 0.741 · p99 0.891 · 11129 op/s · total p50 2.710</sub> | -31.0% (-0.195) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.341<br><sub>context: p90 0.384 · p95 0.394 · p99 0.423 · 2300 op/s · total p50 1.736</sub> | 0.075<br><sub>context: p90 0.111 · p95 0.116 · p99 0.147 · 5024 op/s · total p50 0.783</sub> | -78.0% (-0.266) | 150% AND 2 ms | 🟢 |
| 8 | 0.468<br><sub>context: p90 0.734 · p95 0.833 · p99 1.049 · 12533 op/s · total p50 2.442</sub> | 0.095<br><sub>context: p90 0.131 · p95 0.139 · p99 0.161 · 26158 op/s · total p50 1.120</sub> | -79.7% (-0.373) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.191<br><sub>context: p90 0.221 · p95 0.233 · p99 0.259 · 3613 op/s · total p50 1.072</sub> | 0.002<br><sub>context: p90 0.002 · p95 0.003 · p99 0.003 · 12073 op/s · total p50 0.294</sub> | -99.1% (-0.189) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.509 · p95 0.594 · p99 0.795 · 19036 op/s · total p50 1.529</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 51714 op/s · total p50 0.468</sub> | -99.2% (-0.288) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.139<br><sub>context: p90 0.168 · p95 0.171 · p99 0.183 · 4883 op/s · total p50 0.804</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 14900 op/s · total p50 0.249</sub> | -98.7% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.218<br><sub>context: p90 0.386 · p95 0.462 · p99 0.601 · 26312 op/s · total p50 1.124</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 53547 op/s · total p50 0.445</sub> | -98.9% (-0.215) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.406<br><sub>context: p90 0.451 · p95 0.473 · p99 0.503 · 1895 op/s · total p50 2.089</sub> | 0.071<br><sub>context: p90 0.097 · p95 0.107 · p99 0.127 · 4168 op/s · total p50 0.926</sub> | -82.5% (-0.335) | 150% AND 2 ms | 🟢 |
| 8 | 0.557<br><sub>context: p90 0.940 · p95 1.093 · p99 1.392 · 10158 op/s · total p50 2.917</sub> | 0.085<br><sub>context: p90 0.118 · p95 0.130 · p99 0.147 · 22728 op/s · total p50 1.324</sub> | -84.8% (-0.472) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.712<br><sub>context: p90 0.892 · p95 0.947 · p99 1.023 · 1207 op/s · total p50 3.273</sub> | 1.093<br><sub>context: p90 1.525 · p95 1.618 · p99 1.709 · 799 op/s · total p50 4.968</sub> | +53.4% (+0.381) | 150% AND 2 ms | 🟢 |
| 8 | 0.851<br><sub>context: p90 1.219 · p95 1.347 · p99 1.652 · 6628 op/s · total p50 4.493</sub> | 1.876<br><sub>context: p90 2.798 · p95 2.991 · p99 3.391 · 3427 op/s · total p50 8.730</sub> | +120.4% (+1.025) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.329<br><sub>context: p90 4.007 · p95 4.295 · p99 4.437 · 374 op/s · total p50 10.381</sub> | 4.521<br><sub>context: p90 7.402 · p95 7.838 · p99 8.081 · 200 op/s · total p50 19.928</sub> | +94.1% (+2.192) | 150% AND 2 ms | 🟢 |
| 8 | 2.555<br><sub>context: p90 4.428 · p95 4.881 · p99 6.122 · 2618 op/s · total p50 11.537</sub> | 8.112<br><sub>context: p90 12.671 · p95 13.379 · p99 14.623 · 901 op/s · total p50 34.874</sub> | +217.5% (+5.557) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.189<br><sub>context: p90 0.209 · p95 0.220 · p99 0.235 · 3523 op/s · total p50 1.114</sub> | 0.019<br><sub>context: p90 0.023 · p95 0.036 · p99 0.039 · 7950 op/s · total p50 0.473</sub> | -90.1% (-0.171) | 150% AND 2 ms | 🟢 |
| 8 | 0.283<br><sub>context: p90 0.496 · p95 0.573 · p99 0.761 · 18993 op/s · total p50 1.557</sub> | 0.020<br><sub>context: p90 0.025 · p95 0.028 · p99 0.032 · 41879 op/s · total p50 0.601</sub> | -93.1% (-0.263) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.203<br><sub>context: p90 0.237 · p95 0.246 · p99 0.277 · 3143 op/s · total p50 1.251</sub> | 0.018<br><sub>context: p90 0.021 · p95 0.023 · p99 0.030 · 9272 op/s · total p50 0.405</sub> | -91.2% (-0.185) | 150% AND 2 ms | 🟢 |
| 8 | 0.256<br><sub>context: p90 0.424 · p95 0.499 · p99 0.628 · 21328 op/s · total p50 1.396</sub> | 0.019<br><sub>context: p90 0.024 · p95 0.027 · p99 0.030 · 43720 op/s · total p50 0.564</sub> | -92.7% (-0.238) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.192<br><sub>context: p90 0.231 · p95 0.247 · p99 0.259 · 3664 op/s · total p50 1.068</sub> | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.019 · 8404 op/s · total p50 0.463</sub> | -94.8% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.212<br><sub>context: p90 0.296 · p95 0.329 · p99 0.412 · 15576 op/s · total p50 1.922</sub> | 0.011<br><sub>context: p90 0.016 · p95 0.018 · p99 0.022 · 16925 op/s · total p50 1.755</sub> | -94.8% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.150<br><sub>context: p90 0.189 · p95 0.194 · p99 0.206 · 4774 op/s · total p50 0.821</sub> | 0.005<br><sub>context: p90 0.007 · p95 0.007 · p99 0.008 · 12993 op/s · total p50 0.298</sub> | -96.6% (-0.145) | 150% AND 2 ms | 🟢 |
| 8 | 0.224<br><sub>context: p90 0.397 · p95 0.454 · p99 0.615 · 24633 op/s · total p50 1.210</sub> | 0.006<br><sub>context: p90 0.009 · p95 0.010 · p99 0.013 · 48833 op/s · total p50 0.502</sub> | -97.3% (-0.218) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.275<br><sub>context: p90 0.304 · p95 0.310 · p99 0.332 · 2927 op/s · total p50 1.357</sub> | 0.143<br><sub>context: p90 0.188 · p95 0.195 · p99 0.213 · 3636 op/s · total p50 1.093</sub> | -47.8% (-0.131) | 150% AND 2 ms | 🟢 |
| 8 | 0.347<br><sub>context: p90 0.544 · p95 0.621 · p99 0.751 · 15638 op/s · total p50 1.913</sub> | 0.156<br><sub>context: p90 0.201 · p95 0.218 · p99 0.248 · 24879 op/s · total p50 1.181</sub> | -55.0% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.230<br><sub>context: p90 0.259 · p95 0.274 · p99 0.299 · 2902 op/s · total p50 1.365</sub> | 0.067<br><sub>context: p90 0.095 · p95 0.102 · p99 0.116 · 5669 op/s · total p50 0.705</sub> | -71.0% (-0.163) | 150% AND 2 ms | 🟢 |
| 8 | 0.313<br><sub>context: p90 0.546 · p95 0.639 · p99 0.821 · 17290 op/s · total p50 1.775</sub> | 0.077<br><sub>context: p90 0.114 · p95 0.123 · p99 0.156 · 33653 op/s · total p50 0.880</sub> | -75.3% (-0.235) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.293<br><sub>context: p90 0.334 · p95 0.348 · p99 0.369 · 2664 op/s · total p50 1.453</sub> | 0.096<br><sub>context: p90 0.130 · p95 0.141 · p99 0.171 · 4345 op/s · total p50 0.913</sub> | -67.2% (-0.197) | 150% AND 2 ms | 🟢 |
| 8 | 0.362<br><sub>context: p90 0.528 · p95 0.592 · p99 0.707 · 15769 op/s · total p50 1.919</sub> | 0.105<br><sub>context: p90 0.141 · p95 0.151 · p99 0.170 · 27228 op/s · total p50 1.079</sub> | -70.9% (-0.257) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.264<br><sub>context: p90 0.306 · p95 0.316 · p99 0.339 · 2853 op/s · total p50 1.397</sub> | 0.082<br><sub>context: p90 0.119 · p95 0.128 · p99 0.149 · 4082 op/s · total p50 0.960</sub> | -68.9% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.371<br><sub>context: p90 0.540 · p95 0.612 · p99 0.767 · 14496 op/s · total p50 2.107</sub> | 0.111<br><sub>context: p90 0.152 · p95 0.163 · p99 0.190 · 14341 op/s · total p50 2.063</sub> | -70.1% (-0.260) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.326<br><sub>context: p90 0.372 · p95 0.381 · p99 0.397 · 2303 op/s · total p50 1.722</sub> | 0.106<br><sub>context: p90 0.145 · p95 0.158 · p99 0.175 · 3450 op/s · total p50 1.158</sub> | -67.5% (-0.220) | 150% AND 2 ms | 🟢 |
| 8 | 0.407<br><sub>context: p90 0.606 · p95 0.680 · p99 0.852 · 12744 op/s · total p50 2.364</sub> | 0.118<br><sub>context: p90 0.159 · p95 0.172 · p99 0.194 · 15575 op/s · total p50 1.985</sub> | -71.1% (-0.289) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.312<br><sub>context: p90 0.346 · p95 0.359 · p99 0.376 · 2597 op/s · total p50 1.515</sub> | 0.094<br><sub>context: p90 0.125 · p95 0.134 · p99 0.141 · 4017 op/s · total p50 0.969</sub> | -69.9% (-0.218) | 150% AND 2 ms | 🟢 |
| 8 | 0.394<br><sub>context: p90 0.566 · p95 0.647 · p99 0.790 · 14972 op/s · total p50 2.069</sub> | 0.115<br><sub>context: p90 0.154 · p95 0.165 · p99 0.191 · 21539 op/s · total p50 1.415</sub> | -70.7% (-0.279) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.333<br><sub>context: p90 0.377 · p95 0.386 · p99 0.437 · 2416 op/s · total p50 1.628</sub> | 0.075<br><sub>context: p90 0.106 · p95 0.112 · p99 0.141 · 5365 op/s · total p50 0.715</sub> | -77.5% (-0.258) | 150% AND 2 ms | 🟢 |
| 8 | 0.420<br><sub>context: p90 0.619 · p95 0.723 · p99 0.884 · 13641 op/s · total p50 2.160</sub> | 0.091<br><sub>context: p90 0.128 · p95 0.137 · p99 0.160 · 28374 op/s · total p50 1.052</sub> | -78.4% (-0.329) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 6.313<br><sub>context: p90 6.774 · p95 6.881 · p99 7.215 · 88 op/s · total p50 43.825</sub> | 2.919<br><sub>context: p90 4.270 · p95 4.370 · p99 4.458 · 87 op/s · total p50 45.357</sub> | -53.8% (-3.394) | 150% AND 2 ms | 🟢 |
| 8 | 6.792<br><sub>context: p90 8.885 · p95 9.329 · p99 10.156 · 148 op/s · total p50 210.202</sub> | 2.911<br><sub>context: p90 3.963 · p95 4.071 · p99 4.212 · 150 op/s · total p50 206.424</sub> | -57.1% (-3.881) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.401<br><sub>context: p90 0.453 · p95 0.465 · p99 0.502 · 1984 op/s · total p50 1.993</sub> | 0.106<br><sub>context: p90 0.144 · p95 0.158 · p99 0.174 · 3642 op/s · total p50 1.096</sub> | -73.6% (-0.295) | 150% AND 2 ms | 🟢 |
| 8 | 0.525<br><sub>context: p90 0.749 · p95 0.855 · p99 1.071 · 10886 op/s · total p50 2.703</sub> | 0.119<br><sub>context: p90 0.157 · p95 0.166 · p99 0.188 · 20907 op/s · total p50 1.487</sub> | -77.3% (-0.406) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.547<br><sub>context: p90 0.654 · p95 0.684 · p99 0.753 · 1173 op/s · total p50 2.889</sub> | 0.258<br><sub>context: p90 0.324 · p95 0.346 · p99 0.387 · 1191 op/s · total p50 3.118</sub> | -52.8% (-0.289) | 150% AND 2 ms | 🟢 |
| 8 | 0.621<br><sub>context: p90 0.783 · p95 0.848 · p99 0.928 · 2642 op/s · total p50 11.355</sub> | 0.266<br><sub>context: p90 0.347 · p95 0.376 · p99 0.454 · 2572 op/s · total p50 11.747</sub> | -57.2% (-0.356) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.357 · p95 0.366 · p99 0.392 · 2486 op/s · total p50 1.604</sub> | 0.110<br><sub>context: p90 0.143 · p95 0.157 · p99 0.170 · 3769 op/s · total p50 1.029</sub> | -65.2% (-0.206) | 150% AND 2 ms | 🟢 |
| 8 | 0.410<br><sub>context: p90 0.623 · p95 0.718 · p99 0.890 · 13771 op/s · total p50 2.220</sub> | 0.116<br><sub>context: p90 0.154 · p95 0.163 · p99 0.191 · 24548 op/s · total p50 1.192</sub> | -71.7% (-0.294) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.125<br><sub>context: p90 2.693 · p95 2.763 · p99 2.834 · 460 op/s · total p50 8.877</sub> | 0.119<br><sub>context: p90 0.165 · p95 0.184 · p99 0.218 · 3810 op/s · total p50 1.026</sub> | -94.4% (-2.006) | 150% AND 2 ms | 🟢 |
| 8 | 2.303<br><sub>context: p90 2.926 · p95 3.108 · p99 4.168 · 3271 op/s · total p50 9.601</sub> | 0.133<br><sub>context: p90 0.185 · p95 0.209 · p99 0.260 · 23251 op/s · total p50 1.323</sub> | -94.2% (-2.171) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.031<br><sub>context: p90 2.661 · p95 2.741 · p99 2.839 · 469 op/s · total p50 8.452</sub> | 0.117<br><sub>context: p90 0.166 · p95 0.181 · p99 0.193 · 3585 op/s · total p50 1.105</sub> | -94.3% (-1.914) | 150% AND 2 ms | 🟢 |
| 8 | 2.195<br><sub>context: p90 2.851 · p95 3.043 · p99 4.168 · 3455 op/s · total p50 8.865</sub> | 0.129<br><sub>context: p90 0.184 · p95 0.201 · p99 0.232 · 23629 op/s · total p50 1.260</sub> | -94.1% (-2.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.165 · p95 0.170 · p99 0.187 · 5477 op/s · total p50 0.718</sub> | 0.040<br><sub>context: p90 0.072 · p95 0.076 · p99 0.079 · 7192 op/s · total p50 0.529</sub> | -67.0% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.324 · p95 0.369 · p99 0.472 · 27540 op/s · total p50 1.076</sub> | 0.040<br><sub>context: p90 0.074 · p95 0.078 · p99 0.084 · 41388 op/s · total p50 0.634</sub> | -79.1% (-0.150) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.238<br><sub>context: p90 0.274 · p95 0.288 · p99 0.301 · 3245 op/s · total p50 1.209</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 8445 op/s · total p50 0.447</sub> | -98.5% (-0.235) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.319<br><sub>context: p90 0.522 · p95 0.613 · p99 0.725 · 16701 op/s · total p50 1.817</sub> | 0.004<br><sub>context: p90 0.006 · p95 0.006 · p99 0.008 · 43617 op/s · total p50 0.559</sub> | -98.7% (-0.315) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.243<br><sub>context: p90 0.281 · p95 0.285 · p99 0.300 · 3141 op/s · total p50 1.260</sub> | 0.047<br><sub>context: p90 0.074 · p95 0.078 · p99 0.093 · 5576 op/s · total p50 0.689</sub> | -80.5% (-0.195) | 150% AND 2 ms | 🟢 |
| 8 | 0.370<br><sub>context: p90 0.604 · p95 0.714 · p99 0.883 · 16096 op/s · total p50 1.897</sub> | 0.052<br><sub>context: p90 0.087 · p95 0.091 · p99 0.101 · 33491 op/s · total p50 0.906</sub> | -85.9% (-0.317) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.253<br><sub>context: p90 0.289 · p95 0.302 · p99 0.331 · 2892 op/s · total p50 1.323</sub> | 0.067<br><sub>context: p90 0.124 · p95 0.133 · p99 0.142 · 5254 op/s · total p50 0.759</sub> | -73.5% (-0.186) | 150% AND 2 ms | 🟢 |
| 8 | 0.366<br><sub>context: p90 0.582 · p95 0.660 · p99 0.812 · 16590 op/s · total p50 1.848</sub> | 0.080<br><sub>context: p90 0.148 · p95 0.156 · p99 0.170 · 30154 op/s · total p50 1.011</sub> | -78.1% (-0.286) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.224<br><sub>context: p90 0.259 · p95 0.274 · p99 0.289 · 3428 op/s · total p50 1.151</sub> | 0.037<br><sub>context: p90 0.068 · p95 0.073 · p99 0.080 · 7171 op/s · total p50 0.538</sub> | -83.6% (-0.187) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.481 · p95 0.552 · p99 0.669 · 18864 op/s · total p50 1.628</sub> | 0.045<br><sub>context: p90 0.080 · p95 0.085 · p99 0.093 · 36868 op/s · total p50 0.828</sub> | -85.3% (-0.260) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.137<br><sub>context: p90 1.212 · p95 1.235 · p99 1.262 · 793 op/s · total p50 5.035</sub> | 0.574<br><sub>context: p90 0.630 · p95 0.642 · p99 0.659 · 1307 op/s · total p50 3.052</sub> | -49.5% (-0.563) | 150% AND 2 ms | 🟢 |
| 8 | 1.308<br><sub>context: p90 1.601 · p95 1.769 · p99 2.098 · 5178 op/s · total p50 5.784</sub> | 0.651<br><sub>context: p90 0.782 · p95 0.816 · p99 0.919 · 7824 op/s · total p50 3.804</sub> | -50.3% (-0.658) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.173<br><sub>context: p90 1.244 · p95 1.261 · p99 1.293 · 784 op/s · total p50 5.076</sub> | 0.570<br><sub>context: p90 0.612 · p95 0.621 · p99 0.645 · 1345 op/s · total p50 2.973</sub> | -51.4% (-0.603) | 150% AND 2 ms | 🟢 |
| 8 | 1.248<br><sub>context: p90 1.526 · p95 1.660 · p99 2.027 · 5538 op/s · total p50 5.493</sub> | 0.618<br><sub>context: p90 0.687 · p95 0.709 · p99 0.765 · 8149 op/s · total p50 3.619</sub> | -50.5% (-0.630) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.298 · p95 0.310 · p99 0.358 · 2768 op/s · total p50 1.402</sub> | 0.054<br><sub>context: p90 0.082 · p95 0.088 · p99 0.123 · 5974 op/s · total p50 0.638</sub> | -79.2% (-0.207) | 150% AND 2 ms | 🟢 |
| 8 | 0.349<br><sub>context: p90 0.548 · p95 0.618 · p99 0.778 · 15793 op/s · total p50 1.896</sub> | 0.068<br><sub>context: p90 0.103 · p95 0.110 · p99 0.123 · 30821 op/s · total p50 0.951</sub> | -80.5% (-0.281) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.347<br><sub>context: p90 0.433 · p95 0.458 · p99 0.470 · 2341 op/s · total p50 1.690</sub> | 0.138<br><sub>context: p90 0.218 · p95 0.232 · p99 0.246 · 3387 op/s · total p50 1.137</sub> | -60.3% (-0.209) | 150% AND 2 ms | 🟢 |
| 8 | 0.467<br><sub>context: p90 0.696 · p95 0.793 · p99 0.995 · 11738 op/s · total p50 2.557</sub> | 0.176<br><sub>context: p90 0.264 · p95 0.298 · p99 0.356 · 18630 op/s · total p50 1.637</sub> | -62.4% (-0.291) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.280 · p95 0.289 · p99 0.314 · 2816 op/s · total p50 1.397</sub> | 0.022<br><sub>context: p90 0.029 · p95 0.030 · p99 0.033 · 8005 op/s · total p50 0.471</sub> | -90.5% (-0.215) | 150% AND 2 ms | 🟢 |
| 8 | 0.348<br><sub>context: p90 0.660 · p95 0.783 · p99 0.986 · 14749 op/s · total p50 2.036</sub> | 0.028<br><sub>context: p90 0.038 · p95 0.042 · p99 0.050 · 36197 op/s · total p50 0.806</sub> | -92.0% (-0.320) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.171<br><sub>context: p90 0.206 · p95 0.214 · p99 0.239 · 3896 op/s · total p50 1.015</sub> | 0.038<br><sub>context: p90 0.067 · p95 0.072 · p99 0.078 · 7442 op/s · total p50 0.513</sub> | -77.5% (-0.133) | 150% AND 2 ms | 🟢 |
| 8 | 0.199<br><sub>context: p90 0.329 · p95 0.384 · p99 0.486 · 26845 op/s · total p50 1.105</sub> | 0.039<br><sub>context: p90 0.070 · p95 0.074 · p99 0.078 · 42632 op/s · total p50 0.588</sub> | -80.4% (-0.160) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.161<br><sub>context: p90 0.187 · p95 0.197 · p99 0.207 · 4414 op/s · total p50 0.871</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.076 · p99 0.083 · 8611 op/s · total p50 0.449</sub> | -78.8% (-0.127) | 150% AND 2 ms | 🟢 |
| 8 | 0.197<br><sub>context: p90 0.348 · p95 0.407 · p99 0.498 · 26858 op/s · total p50 1.119</sub> | 0.040<br><sub>context: p90 0.076 · p95 0.080 · p99 0.091 · 40284 op/s · total p50 0.619</sub> | -79.6% (-0.157) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.875<br><sub>context: p90 0.920 · p95 0.932 · p99 0.946 · 1012 op/s · total p50 3.936</sub> | 0.205<br><sub>context: p90 0.229 · p95 0.238 · p99 0.262 · 2989 op/s · total p50 1.322</sub> | -76.6% (-0.670) | 150% AND 2 ms | 🟢 |
| 8 | 0.974<br><sub>context: p90 1.686 · p95 1.828 · p99 2.072 · 5712 op/s · total p50 5.424</sub> | 0.240<br><sub>context: p90 0.267 · p95 0.275 · p99 0.313 · 20516 op/s · total p50 1.453</sub> | -75.4% (-0.734) | 150% AND 2 ms | 🟢 |

</details>
