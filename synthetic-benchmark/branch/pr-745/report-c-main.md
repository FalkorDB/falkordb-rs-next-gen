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

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.088<br><sub>context: p90 1.202 · p95 1.236 · p99 1.277 · 870 op/s · total p50 4.601</sub> | 0.539<br><sub>context: p90 0.569 · p95 0.577 · p99 0.586 · 1613 op/s · total p50 2.486</sub> | -50.4% (-0.548) | 150% AND 2 ms | 🟢 |
| 8 | 1.293<br><sub>context: p90 1.914 · p95 2.198 · p99 2.476 · 5220 op/s · total p50 5.855</sub> | 0.596<br><sub>context: p90 0.659 · p95 0.682 · p99 0.729 · 9697 op/s · total p50 3.157</sub> | -53.9% (-0.697) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.650<br><sub>context: p90 1.770 · p95 1.794 · p99 1.854 · 584 op/s · total p50 6.813</sub> | 0.563<br><sub>context: p90 0.595 · p95 0.604 · p99 0.624 · 1543 op/s · total p50 2.588</sub> | -65.9% (-1.087) | 150% AND 2 ms | 🟢 |
| 8 | 1.846<br><sub>context: p90 2.307 · p95 2.528 · p99 2.836 · 3880 op/s · total p50 7.930</sub> | 0.598<br><sub>context: p90 0.643 · p95 0.661 · p99 0.708 · 9790 op/s · total p50 3.144</sub> | -67.6% (-1.248) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.650<br><sub>context: p90 1.758 · p95 1.778 · p99 1.844 · 583 op/s · total p50 6.856</sub> | 1.022<br><sub>context: p90 1.068 · p95 1.080 · p99 1.102 · 873 op/s · total p50 4.605</sub> | -38.0% (-0.628) | 150% AND 2 ms | 🟢 |
| 8 | 1.918<br><sub>context: p90 2.645 · p95 2.949 · p99 3.362 · 3682 op/s · total p50 8.330</sub> | 1.625<br><sub>context: p90 2.339 · p95 2.503 · p99 2.820 · 4121 op/s · total p50 7.580</sub> | -15.3% (-0.293) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.232<br><sub>context: p90 2.369 · p95 2.410 · p99 2.478 · 435 op/s · total p50 9.140</sub> | 1.065<br><sub>context: p90 1.129 · p95 1.153 · p99 1.181 · 866 op/s · total p50 4.636</sub> | -52.3% (-1.166) | 150% AND 2 ms | 🟢 |
| 8 | 2.514<br><sub>context: p90 3.317 · p95 3.638 · p99 4.386 · 2793 op/s · total p50 10.811</sub> | 1.137<br><sub>context: p90 1.287 · p95 1.328 · p99 1.409 · 6045 op/s · total p50 5.215</sub> | -54.8% (-1.377) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.063<br><sub>context: p90 0.081 · p95 0.102 · p99 0.115 · 10374 op/s · total p50 0.373</sub> | 0.013<br><sub>context: p90 0.015 · p95 0.016 · p99 0.022 · 15797 op/s · total p50 0.249</sub> | -79.2% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.160<br><sub>context: p90 0.279 · p95 0.324 · p99 0.408 · 36348 op/s · total p50 0.821</sub> | 0.015<br><sub>context: p90 0.022 · p95 0.024 · p99 0.029 · 61782 op/s · total p50 0.393</sub> | -90.7% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.162<br><sub>context: p90 0.200 · p95 0.214 · p99 0.226 · 4863 op/s · total p50 0.816</sub> | 0.043<br><sub>context: p90 0.068 · p95 0.072 · p99 0.082 · 7683 op/s · total p50 0.513</sub> | -73.8% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.269<br><sub>context: p90 0.399 · p95 0.471 · p99 0.579 · 21590 op/s · total p50 1.426</sub> | 0.052<br><sub>context: p90 0.085 · p95 0.089 · p99 0.096 · 38411 op/s · total p50 0.796</sub> | -80.7% (-0.217) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.174<br><sub>context: p90 0.218 · p95 0.231 · p99 0.242 · 4497 op/s · total p50 0.880</sub> | 0.043<br><sub>context: p90 0.069 · p95 0.072 · p99 0.077 · 6977 op/s · total p50 0.567</sub> | -75.3% (-0.131) | 150% AND 2 ms | 🟢 |
| 8 | 0.309<br><sub>context: p90 0.442 · p95 0.507 · p99 0.636 · 19455 op/s · total p50 1.584</sub> | 0.052<br><sub>context: p90 0.083 · p95 0.087 · p99 0.098 · 36952 op/s · total p50 0.806</sub> | -83.2% (-0.257) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.253 · p95 0.274 · p99 0.298 · 4195 op/s · total p50 0.942</sub> | 0.079<br><sub>context: p90 0.105 · p95 0.117 · p99 0.134 · 5262 op/s · total p50 0.745</sub> | -57.5% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.340<br><sub>context: p90 0.504 · p95 0.570 · p99 0.690 · 16938 op/s · total p50 1.810</sub> | 0.097<br><sub>context: p90 0.129 · p95 0.137 · p99 0.152 · 29588 op/s · total p50 1.006</sub> | -71.4% (-0.243) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.239<br><sub>context: p90 0.300 · p95 0.320 · p99 0.348 · 3379 op/s · total p50 1.193</sub> | 0.093<br><sub>context: p90 0.122 · p95 0.128 · p99 0.144 · 4443 op/s · total p50 0.900</sub> | -61.1% (-0.146) | 150% AND 2 ms | 🟢 |
| 8 | 0.361<br><sub>context: p90 0.510 · p95 0.568 · p99 0.692 · 16265 op/s · total p50 1.902</sub> | 0.104<br><sub>context: p90 0.138 · p95 0.147 · p99 0.164 · 25496 op/s · total p50 1.115</sub> | -71.3% (-0.257) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.279<br><sub>context: p90 0.345 · p95 0.368 · p99 0.387 · 2914 op/s · total p50 1.366</sub> | 0.112<br><sub>context: p90 0.149 · p95 0.158 · p99 0.181 · 4126 op/s · total p50 0.954</sub> | -59.8% (-0.167) | 150% AND 2 ms | 🟢 |
| 8 | 0.407<br><sub>context: p90 0.573 · p95 0.657 · p99 0.823 · 13025 op/s · total p50 2.328</sub> | 0.137<br><sub>context: p90 0.187 · p95 0.201 · p99 0.224 · 16049 op/s · total p50 1.875</sub> | -66.3% (-0.270) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.301<br><sub>context: p90 0.378 · p95 0.392 · p99 0.443 · 2736 op/s · total p50 1.450</sub> | 0.129<br><sub>context: p90 0.172 · p95 0.179 · p99 0.205 · 3485 op/s · total p50 1.133</sub> | -57.0% (-0.171) | 150% AND 2 ms | 🟢 |
| 8 | 0.469<br><sub>context: p90 0.636 · p95 0.703 · p99 0.870 · 12234 op/s · total p50 2.474</sub> | 0.150<br><sub>context: p90 0.205 · p95 0.221 · p99 0.246 · 14456 op/s · total p50 2.090</sub> | -68.1% (-0.319) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.484<br><sub>context: p90 0.643 · p95 0.686 · p99 0.774 · 1694 op/s · total p50 2.324</sub> | 0.226<br><sub>context: p90 0.301 · p95 0.323 · p99 0.351 · 2037 op/s · total p50 1.836</sub> | -53.4% (-0.259) | 150% AND 2 ms | 🟢 |
| 8 | 0.548<br><sub>context: p90 0.787 · p95 0.870 · p99 1.015 · 4284 op/s · total p50 6.973</sub> | 0.255<br><sub>context: p90 0.349 · p95 0.384 · p99 0.425 · 4138 op/s · total p50 7.360</sub> | -53.5% (-0.293) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.553<br><sub>context: p90 0.705 · p95 0.732 · p99 0.830 · 1524 op/s · total p50 2.570</sub> | 0.240<br><sub>context: p90 0.321 · p95 0.367 · p99 0.433 · 2050 op/s · total p50 1.833</sub> | -56.6% (-0.313) | 150% AND 2 ms | 🟢 |
| 8 | 0.639<br><sub>context: p90 0.894 · p95 0.961 · p99 1.103 · 4050 op/s · total p50 7.421</sub> | 0.280<br><sub>context: p90 0.375 · p95 0.405 · p99 0.463 · 3867 op/s · total p50 8.002</sub> | -56.3% (-0.360) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.400<br><sub>context: p90 0.487 · p95 0.517 · p99 0.559 · 2203 op/s · total p50 1.799</sub> | 0.307<br><sub>context: p90 0.467 · p95 0.499 · p99 0.575 · 2235 op/s · total p50 1.780</sub> | -23.3% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.563<br><sub>context: p90 0.737 · p95 0.793 · p99 0.947 · 10790 op/s · total p50 2.852</sub> | 0.375<br><sub>context: p90 0.580 · p95 0.659 · p99 0.767 · 13050 op/s · total p50 2.276</sub> | -33.4% (-0.188) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.216<br><sub>context: p90 0.270 · p95 0.281 · p99 0.318 · 3783 op/s · total p50 1.043</sub> | 0.063<br><sub>context: p90 0.089 · p95 0.100 · p99 0.105 · 5786 op/s · total p50 0.679</sub> | -70.6% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.378<br><sub>context: p90 0.547 · p95 0.626 · p99 0.743 · 15928 op/s · total p50 1.918</sub> | 0.077<br><sub>context: p90 0.109 · p95 0.116 · p99 0.134 · 32904 op/s · total p50 0.930</sub> | -79.6% (-0.301) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.140 · p95 0.155 · p99 0.176 · 6817 op/s · total p50 0.570</sub> | 0.001<br><sub>context: p90 0.002 · p95 0.002 · p99 0.002 · 14063 op/s · total p50 0.279</sub> | -98.6% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 0.225<br><sub>context: p90 0.361 · p95 0.421 · p99 0.516 · 26969 op/s · total p50 1.133</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 60949 op/s · total p50 0.383</sub> | -99.1% (-0.223) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.083<br><sub>context: p90 0.118 · p95 0.132 · p99 0.140 · 7645 op/s · total p50 0.514</sub> | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 18780 op/s · total p50 0.209</sub> | -98.2% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.179<br><sub>context: p90 0.286 · p95 0.342 · p99 0.432 · 33672 op/s · total p50 0.897</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 61614 op/s · total p50 0.370</sub> | -98.8% (-0.177) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.278<br><sub>context: p90 0.351 · p95 0.368 · p99 0.412 · 2927 op/s · total p50 1.358</sub> | 0.065<br><sub>context: p90 0.093 · p95 0.097 · p99 0.111 · 4557 op/s · total p50 0.862</sub> | -76.7% (-0.213) | 150% AND 2 ms | 🟢 |
| 8 | 0.451<br><sub>context: p90 0.623 · p95 0.708 · p99 0.854 · 13601 op/s · total p50 2.225</sub> | 0.076<br><sub>context: p90 0.105 · p95 0.114 · p99 0.129 · 25103 op/s · total p50 1.206</sub> | -83.1% (-0.375) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.567<br><sub>context: p90 0.727 · p95 0.766 · p99 0.853 · 1664 op/s · total p50 2.380</sub> | 1.003<br><sub>context: p90 1.421 · p95 1.501 · p99 1.665 · 860 op/s · total p50 4.651</sub> | +76.8% (+0.436) | 150% AND 2 ms | 🟢 |
| 8 | 0.705<br><sub>context: p90 0.999 · p95 1.095 · p99 1.366 · 8003 op/s · total p50 3.814</sub> | 1.585<br><sub>context: p90 2.376 · p95 2.542 · p99 2.883 · 4083 op/s · total p50 7.328</sub> | +124.8% (+0.880) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.994<br><sub>context: p90 3.489 · p95 3.721 · p99 3.935 · 444 op/s · total p50 8.931</sub> | 4.264<br><sub>context: p90 6.675 · p95 7.163 · p99 7.537 · 216 op/s · total p50 18.028</sub> | +113.9% (+2.270) | 150% AND 2 ms | 🟢 |
| 8 | 2.150<br><sub>context: p90 3.728 · p95 4.135 · p99 5.224 · 3147 op/s · total p50 9.611</sub> | 6.600<br><sub>context: p90 10.311 · p95 11.026 · p99 12.317 · 1098 op/s · total p50 28.550</sub> | +207.0% (+4.450) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.092<br><sub>context: p90 0.134 · p95 0.155 · p99 0.174 · 6879 op/s · total p50 0.559</sub> | 0.014<br><sub>context: p90 0.021 · p95 0.022 · p99 0.028 · 11012 op/s · total p50 0.350</sub> | -84.7% (-0.078) | 150% AND 2 ms | 🟢 |
| 8 | 0.216<br><sub>context: p90 0.342 · p95 0.396 · p99 0.490 · 26695 op/s · total p50 1.153</sub> | 0.018<br><sub>context: p90 0.023 · p95 0.025 · p99 0.029 · 45166 op/s · total p50 0.633</sub> | -91.6% (-0.198) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.113<br><sub>context: p90 0.140 · p95 0.151 · p99 0.158 · 6685 op/s · total p50 0.598</sub> | 0.012<br><sub>context: p90 0.020 · p95 0.021 · p99 0.024 · 10785 op/s · total p50 0.366</sub> | -89.3% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.219<br><sub>context: p90 0.351 · p95 0.413 · p99 0.533 · 25709 op/s · total p50 1.190</sub> | 0.017<br><sub>context: p90 0.023 · p95 0.025 · p99 0.031 · 45999 op/s · total p50 0.579</sub> | -92.0% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.121 · p95 0.132 · p99 0.145 · 6234 op/s · total p50 0.616</sub> | 0.008<br><sub>context: p90 0.011 · p95 0.012 · p99 0.014 · 10486 op/s · total p50 0.376</sub> | -92.9% (-0.099) | 150% AND 2 ms | 🟢 |
| 8 | 0.177<br><sub>context: p90 0.235 · p95 0.255 · p99 0.312 · 17228 op/s · total p50 1.774</sub> | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.017 · 18010 op/s · total p50 1.697</sub> | -94.5% (-0.167) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.080<br><sub>context: p90 0.110 · p95 0.114 · p99 0.135 · 8663 op/s · total p50 0.456</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 13093 op/s · total p50 0.302</sub> | -94.4% (-0.075) | 150% AND 2 ms | 🟢 |
| 8 | 0.178<br><sub>context: p90 0.296 · p95 0.341 · p99 0.410 · 32699 op/s · total p50 0.936</sub> | 0.005<br><sub>context: p90 0.007 · p95 0.008 · p99 0.010 · 60066 op/s · total p50 0.406</sub> | -97.0% (-0.173) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.197<br><sub>context: p90 0.232 · p95 0.240 · p99 0.271 · 4242 op/s · total p50 0.922</sub> | 0.130<br><sub>context: p90 0.164 · p95 0.174 · p99 0.200 · 4518 op/s · total p50 0.858</sub> | -34.3% (-0.068) | 150% AND 2 ms | 🟢 |
| 8 | 0.292<br><sub>context: p90 0.406 · p95 0.460 · p99 0.540 · 19478 op/s · total p50 1.586</sub> | 0.139<br><sub>context: p90 0.181 · p95 0.193 · p99 0.221 · 29148 op/s · total p50 1.031</sub> | -52.2% (-0.152) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.144<br><sub>context: p90 0.176 · p95 0.185 · p99 0.197 · 5597 op/s · total p50 0.706</sub> | 0.059<br><sub>context: p90 0.087 · p95 0.095 · p99 0.104 · 7003 op/s · total p50 0.559</sub> | -58.9% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.355 · p95 0.407 · p99 0.494 · 24564 op/s · total p50 1.234</sub> | 0.070<br><sub>context: p90 0.102 · p95 0.110 · p99 0.139 · 40079 op/s · total p50 0.757</sub> | -70.3% (-0.165) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.190<br><sub>context: p90 0.258 · p95 0.265 · p99 0.287 · 4076 op/s · total p50 0.966</sub> | 0.070<br><sub>context: p90 0.100 · p95 0.107 · p99 0.117 · 5961 op/s · total p50 0.663</sub> | -63.3% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.318<br><sub>context: p90 0.468 · p95 0.523 · p99 0.640 · 18051 op/s · total p50 1.667</sub> | 0.093<br><sub>context: p90 0.126 · p95 0.134 · p99 0.159 · 28460 op/s · total p50 1.063</sub> | -70.9% (-0.225) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.234 · p95 0.242 · p99 0.258 · 3898 op/s · total p50 1.011</sub> | 0.067<br><sub>context: p90 0.096 · p95 0.103 · p99 0.118 · 5402 op/s · total p50 0.726</sub> | -63.6% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.297<br><sub>context: p90 0.417 · p95 0.469 · p99 0.565 · 16291 op/s · total p50 1.856</sub> | 0.096<br><sub>context: p90 0.132 · p95 0.142 · p99 0.160 · 15421 op/s · total p50 1.945</sub> | -67.7% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.213<br><sub>context: p90 0.274 · p95 0.296 · p99 0.321 · 3556 op/s · total p50 1.103</sub> | 0.094<br><sub>context: p90 0.129 · p95 0.136 · p99 0.154 · 4122 op/s · total p50 0.943</sub> | -55.8% (-0.119) | 150% AND 2 ms | 🟢 |
| 8 | 0.357<br><sub>context: p90 0.514 · p95 0.575 · p99 0.709 · 15240 op/s · total p50 1.971</sub> | 0.106<br><sub>context: p90 0.144 · p95 0.153 · p99 0.171 · 17311 op/s · total p50 1.734</sub> | -70.3% (-0.251) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.179<br><sub>context: p90 0.201 · p95 0.212 · p99 0.240 · 4416 op/s · total p50 0.875</sub> | 0.083<br><sub>context: p90 0.111 · p95 0.118 · p99 0.128 · 4631 op/s · total p50 0.843</sub> | -53.7% (-0.096) | 150% AND 2 ms | 🟢 |
| 8 | 0.345<br><sub>context: p90 0.487 · p95 0.556 · p99 0.669 · 16846 op/s · total p50 1.793</sub> | 0.100<br><sub>context: p90 0.134 · p95 0.144 · p99 0.160 · 25860 op/s · total p50 1.193</sub> | -70.9% (-0.244) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.232<br><sub>context: p90 0.305 · p95 0.315 · p99 0.338 · 3465 op/s · total p50 1.153</sub> | 0.069<br><sub>context: p90 0.089 · p95 0.100 · p99 0.107 · 5748 op/s · total p50 0.686</sub> | -70.2% (-0.163) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.512 · p95 0.577 · p99 0.703 · 16300 op/s · total p50 1.903</sub> | 0.077<br><sub>context: p90 0.112 · p95 0.120 · p99 0.144 · 32443 op/s · total p50 0.927</sub> | -78.5% (-0.282) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.482<br><sub>context: p90 5.817 · p95 6.204 · p99 6.855 · 71 op/s · total p50 54.327</sub> | 2.546<br><sub>context: p90 2.681 · p95 2.723 · p99 2.753 · 101 op/s · total p50 39.298</sub> | -53.6% (-2.937) | 150% AND 2 ms | 🟢 |
| 8 | 5.557<br><sub>context: p90 6.361 · p95 7.253 · p99 7.761 · 176 op/s · total p50 173.316</sub> | 2.571<br><sub>context: p90 2.732 · p95 2.885 · p99 3.283 · 168 op/s · total p50 176.270</sub> | -53.7% (-2.985) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.334 · p95 0.348 · p99 0.372 · 3055 op/s · total p50 1.317</sub> | 0.093<br><sub>context: p90 0.118 · p95 0.129 · p99 0.145 · 4267 op/s · total p50 0.922</sub> | -67.3% (-0.192) | 150% AND 2 ms | 🟢 |
| 8 | 0.450<br><sub>context: p90 0.629 · p95 0.694 · p99 0.864 · 13263 op/s · total p50 2.325</sub> | 0.103<br><sub>context: p90 0.135 · p95 0.143 · p99 0.163 · 24730 op/s · total p50 1.239</sub> | -77.2% (-0.347) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.465<br><sub>context: p90 0.566 · p95 0.628 · p99 0.716 · 1467 op/s · total p50 2.453</sub> | 0.218<br><sub>context: p90 0.291 · p95 0.317 · p99 0.343 · 1325 op/s · total p50 2.862</sub> | -53.0% (-0.246) | 150% AND 2 ms | 🟢 |
| 8 | 0.491<br><sub>context: p90 0.673 · p95 0.720 · p99 0.825 · 2820 op/s · total p50 10.962</sub> | 0.236<br><sub>context: p90 0.327 · p95 0.355 · p99 0.418 · 2857 op/s · total p50 10.537</sub> | -51.9% (-0.255) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.260 · p95 0.280 · p99 0.301 · 3976 op/s · total p50 0.995</sub> | 0.094<br><sub>context: p90 0.118 · p95 0.125 · p99 0.145 · 4691 op/s · total p50 0.838</sub> | -51.4% (-0.100) | 150% AND 2 ms | 🟢 |
| 8 | 0.343<br><sub>context: p90 0.496 · p95 0.564 · p99 0.682 · 17101 op/s · total p50 1.789</sub> | 0.103<br><sub>context: p90 0.141 · p95 0.149 · p99 0.168 · 26278 op/s · total p50 1.141</sub> | -69.9% (-0.240) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.828<br><sub>context: p90 2.407 · p95 2.457 · p99 2.553 · 537 op/s · total p50 7.556</sub> | 0.103<br><sub>context: p90 0.140 · p95 0.153 · p99 0.187 · 4649 op/s · total p50 0.855</sub> | -94.4% (-1.725) | 150% AND 2 ms | 🟢 |
| 8 | 2.030<br><sub>context: p90 2.576 · p95 2.731 · p99 3.711 · 3734 op/s · total p50 8.397</sub> | 0.117<br><sub>context: p90 0.161 · p95 0.182 · p99 0.231 · 27109 op/s · total p50 1.128</sub> | -94.2% (-1.913) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.757<br><sub>context: p90 2.238 · p95 2.370 · p99 2.533 · 560 op/s · total p50 7.137</sub> | 0.103<br><sub>context: p90 0.147 · p95 0.169 · p99 0.182 · 4330 op/s · total p50 0.915</sub> | -94.1% (-1.654) | 150% AND 2 ms | 🟢 |
| 8 | 1.935<br><sub>context: p90 2.571 · p95 2.728 · p99 3.687 · 3909 op/s · total p50 7.877</sub> | 0.111<br><sub>context: p90 0.160 · p95 0.174 · p99 0.194 · 26366 op/s · total p50 1.075</sub> | -94.3% (-1.824) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.115<br><sub>context: p90 0.133 · p95 0.142 · p99 0.163 · 6728 op/s · total p50 0.580</sub> | 0.032<br><sub>context: p90 0.063 · p95 0.066 · p99 0.068 · 8269 op/s · total p50 0.466</sub> | -71.8% (-0.083) | 150% AND 2 ms | 🟢 |
| 8 | 0.174<br><sub>context: p90 0.292 · p95 0.329 · p99 0.420 · 32069 op/s · total p50 0.950</sub> | 0.034<br><sub>context: p90 0.065 · p95 0.067 · p99 0.072 · 50617 op/s · total p50 0.516</sub> | -80.4% (-0.140) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.206 · p95 0.208 · p99 0.226 · 4741 op/s · total p50 0.837</sub> | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 11722 op/s · total p50 0.315</sub> | -98.5% (-0.173) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.260<br><sub>context: p90 0.377 · p95 0.430 · p99 0.519 · 22135 op/s · total p50 1.392</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 49315 op/s · total p50 0.481</sub> | -98.6% (-0.257) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.144<br><sub>context: p90 0.187 · p95 0.197 · p99 0.211 · 5340 op/s · total p50 0.747</sub> | 0.041<br><sub>context: p90 0.063 · p95 0.066 · p99 0.073 · 7255 op/s · total p50 0.536</sub> | -71.2% (-0.103) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.453 · p95 0.530 · p99 0.664 · 20079 op/s · total p50 1.530</sub> | 0.046<br><sub>context: p90 0.077 · p95 0.081 · p99 0.091 · 38986 op/s · total p50 0.768</sub> | -84.8% (-0.258) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.170<br><sub>context: p90 0.202 · p95 0.217 · p99 0.239 · 4874 op/s · total p50 0.805</sub> | 0.060<br><sub>context: p90 0.111 · p95 0.115 · p99 0.131 · 6391 op/s · total p50 0.618</sub> | -64.8% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.444 · p95 0.513 · p99 0.637 · 20552 op/s · total p50 1.497</sub> | 0.069<br><sub>context: p90 0.130 · p95 0.136 · p99 0.145 · 35779 op/s · total p50 0.832</sub> | -77.5% (-0.237) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.172 · p95 0.176 · p99 0.183 · 5711 op/s · total p50 0.689</sub> | 0.032<br><sub>context: p90 0.060 · p95 0.061 · p99 0.070 · 8003 op/s · total p50 0.496</sub> | -75.4% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.272<br><sub>context: p90 0.419 · p95 0.475 · p99 0.571 · 22134 op/s · total p50 1.398</sub> | 0.039<br><sub>context: p90 0.070 · p95 0.073 · p99 0.083 · 42173 op/s · total p50 0.722</sub> | -85.5% (-0.233) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.953<br><sub>context: p90 1.040 · p95 1.059 · p99 1.138 · 988 op/s · total p50 4.040</sub> | 0.509<br><sub>context: p90 0.544 · p95 0.561 · p99 0.571 · 1533 op/s · total p50 2.600</sub> | -46.5% (-0.443) | 150% AND 2 ms | 🟢 |
| 8 | 1.134<br><sub>context: p90 1.408 · p95 1.597 · p99 1.873 · 6099 op/s · total p50 4.984</sub> | 0.576<br><sub>context: p90 0.709 · p95 0.741 · p99 0.821 · 8502 op/s · total p50 3.611</sub> | -49.2% (-0.558) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.959<br><sub>context: p90 1.051 · p95 1.075 · p99 1.112 · 986 op/s · total p50 4.070</sub> | 0.511<br><sub>context: p90 0.558 · p95 0.570 · p99 0.579 · 1565 op/s · total p50 2.560</sub> | -46.7% (-0.448) | 150% AND 2 ms | 🟢 |
| 8 | 1.125<br><sub>context: p90 1.399 · p95 1.557 · p99 1.871 · 6355 op/s · total p50 4.846</sub> | 0.553<br><sub>context: p90 0.612 · p95 0.632 · p99 0.675 · 9751 op/s · total p50 3.128</sub> | -50.8% (-0.572) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.168<br><sub>context: p90 0.208 · p95 0.211 · p99 0.227 · 4627 op/s · total p50 0.825</sub> | 0.052<br><sub>context: p90 0.081 · p95 0.095 · p99 0.102 · 6499 op/s · total p50 0.598</sub> | -68.9% (-0.116) | 150% AND 2 ms | 🟢 |
| 8 | 0.288<br><sub>context: p90 0.419 · p95 0.469 · p99 0.602 · 20282 op/s · total p50 1.521</sub> | 0.061<br><sub>context: p90 0.095 · p95 0.101 · p99 0.114 · 34903 op/s · total p50 0.825</sub> | -78.7% (-0.227) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.258<br><sub>context: p90 0.324 · p95 0.348 · p99 0.383 · 3286 op/s · total p50 1.182</sub> | 0.125<br><sub>context: p90 0.200 · p95 0.227 · p99 0.249 · 3822 op/s · total p50 1.009</sub> | -51.4% (-0.133) | 150% AND 2 ms | 🟢 |
| 8 | 0.371<br><sub>context: p90 0.512 · p95 0.583 · p99 0.698 · 15674 op/s · total p50 2.005</sub> | 0.157<br><sub>context: p90 0.240 · p95 0.272 · p99 0.320 · 20970 op/s · total p50 1.472</sub> | -57.7% (-0.214) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.138<br><sub>context: p90 0.180 · p95 0.186 · p99 0.241 · 5438 op/s · total p50 0.724</sub> | 0.017<br><sub>context: p90 0.020 · p95 0.021 · p99 0.024 · 9811 op/s · total p50 0.387</sub> | -87.8% (-0.121) | 150% AND 2 ms | 🟢 |
| 8 | 0.239<br><sub>context: p90 0.368 · p95 0.424 · p99 0.516 · 24133 op/s · total p50 1.270</sub> | 0.024<br><sub>context: p90 0.032 · p95 0.036 · p99 0.044 · 39391 op/s · total p50 0.732</sub> | -90.0% (-0.215) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.086<br><sub>context: p90 0.119 · p95 0.123 · p99 0.140 · 8056 op/s · total p50 0.492</sub> | 0.036<br><sub>context: p90 0.068 · p95 0.070 · p99 0.074 · 7726 op/s · total p50 0.502</sub> | -58.0% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.169<br><sub>context: p90 0.278 · p95 0.316 · p99 0.407 · 32572 op/s · total p50 0.943</sub> | 0.034<br><sub>context: p90 0.044 · p95 0.063 · p99 0.067 · 50484 op/s · total p50 0.478</sub> | -79.9% (-0.135) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.097<br><sub>context: p90 0.119 · p95 0.124 · p99 0.136 · 7732 op/s · total p50 0.509</sub> | 0.031<br><sub>context: p90 0.058 · p95 0.065 · p99 0.068 · 9797 op/s · total p50 0.400</sub> | -67.9% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.168<br><sub>context: p90 0.288 · p95 0.331 · p99 0.418 · 32528 op/s · total p50 0.932</sub> | 0.037<br><sub>context: p90 0.069 · p95 0.071 · p99 0.079 · 48826 op/s · total p50 0.518</sub> | -78.3% (-0.132) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.708<br><sub>context: p90 0.790 · p95 0.811 · p99 0.831 · 1299 op/s · total p50 3.017</sub> | 0.185<br><sub>context: p90 0.197 · p95 0.199 · p99 0.208 · 3781 op/s · total p50 1.055</sub> | -73.9% (-0.523) | 150% AND 2 ms | 🟢 |
| 8 | 0.820<br><sub>context: p90 1.388 · p95 1.507 · p99 1.639 · 6610 op/s · total p50 4.553</sub> | 0.220<br><sub>context: p90 0.243 · p95 0.251 · p99 0.290 · 20738 op/s · total p50 1.464</sub> | -73.2% (-0.600) | 150% AND 2 ms | 🟢 |

</details>
