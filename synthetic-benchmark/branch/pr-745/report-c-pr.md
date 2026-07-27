### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb |
| workload_hash | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.581<br><sub>context: p90 1.662 · p95 1.683 · p99 1.713 · 629 op/s</sub> | 0.935<br><sub>context: p90 1.037 · p95 1.075 · p99 1.093 · 1058 op/s</sub> | -40.8% (-0.645) | 150% AND 2 ms | 🟢 |
| 8 | 1.863<br><sub>context: p90 2.428 · p95 2.608 · p99 3.023 · 4084 op/s</sub> | 0.919<br><sub>context: p90 1.059 · p95 1.102 · p99 1.213 · 8595 op/s</sub> | -50.7% (-0.944) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.280<br><sub>context: p90 2.359 · p95 2.384 · p99 2.433 · 438 op/s</sub> | 0.954<br><sub>context: p90 1.050 · p95 1.071 · p99 1.120 · 1042 op/s</sub> | -58.2% (-1.327) | 150% AND 2 ms | 🟢 |
| 8 | 2.674<br><sub>context: p90 3.513 · p95 3.635 · p99 3.965 · 2771 op/s</sub> | 1.026<br><sub>context: p90 1.224 · p95 1.264 · p99 1.359 · 7653 op/s</sub> | -61.6% (-1.648) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.204<br><sub>context: p90 2.298 · p95 2.354 · p99 2.400 · 451 op/s</sub> | 1.479<br><sub>context: p90 1.562 · p95 1.592 · p99 1.652 · 668 op/s</sub> | -32.9% (-0.725) | 150% AND 2 ms | 🟢 |
| 8 | 2.700<br><sub>context: p90 3.509 · p95 3.698 · p99 4.134 · 2777 op/s</sub> | 1.812<br><sub>context: p90 2.344 · p95 2.536 · p99 2.805 · 4271 op/s</sub> | -32.9% (-0.889) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.843<br><sub>context: p90 2.940 · p95 2.969 · p99 3.044 · 351 op/s</sub> | 1.562<br><sub>context: p90 1.660 · p95 1.696 · p99 1.737 · 636 op/s</sub> | -45.1% (-1.281) | 150% AND 2 ms | 🟢 |
| 8 | 3.789<br><sub>context: p90 4.818 · p95 5.097 · p99 5.623 · 2061 op/s</sub> | 1.659<br><sub>context: p90 1.981 · p95 2.051 · p99 2.172 · 4663 op/s</sub> | -56.2% (-2.131) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.200<br><sub>context: p90 0.272 · p95 0.298 · p99 0.342 · 4788 op/s</sub> | 0.173<br><sub>context: p90 0.242 · p95 0.256 · p99 0.307 · 5374 op/s</sub> | -13.5% (-0.027) | 150% AND 2 ms | 🟢 |
| 8 | 0.242<br><sub>context: p90 0.314 · p95 0.342 · p99 0.384 · 30742 op/s</sub> | 0.225<br><sub>context: p90 0.290 · p95 0.308 · p99 0.350 · 34575 op/s</sub> | -7.0% (-0.017) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.521<br><sub>context: p90 0.621 · p95 0.640 · p99 0.694 · 1896 op/s</sub> | 0.375<br><sub>context: p90 0.466 · p95 0.493 · p99 0.544 · 2625 op/s</sub> | -28.0% (-0.146) | 150% AND 2 ms | 🟢 |
| 8 | 0.456<br><sub>context: p90 0.553 · p95 0.583 · p99 0.654 · 16916 op/s</sub> | 0.339<br><sub>context: p90 0.439 · p95 0.469 · p99 0.552 · 21827 op/s</sub> | -25.7% (-0.117) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.546<br><sub>context: p90 0.642 · p95 0.665 · p99 0.723 · 1816 op/s</sub> | 0.424<br><sub>context: p90 0.516 · p95 0.544 · p99 0.612 · 2245 op/s</sub> | -22.3% (-0.122) | 150% AND 2 ms | 🟢 |
| 8 | 0.495<br><sub>context: p90 0.611 · p95 0.656 · p99 0.736 · 15531 op/s</sub> | 0.359<br><sub>context: p90 0.461 · p95 0.497 · p99 0.567 · 21127 op/s</sub> | -27.5% (-0.136) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.576<br><sub>context: p90 0.677 · p95 0.705 · p99 0.790 · 1713 op/s</sub> | 0.528<br><sub>context: p90 0.625 · p95 0.660 · p99 0.703 · 1871 op/s</sub> | -8.4% (-0.049) | 150% AND 2 ms | 🟢 |
| 8 | 0.542<br><sub>context: p90 0.664 · p95 0.706 · p99 0.796 · 14240 op/s</sub> | 0.492<br><sub>context: p90 0.641 · p95 0.700 · p99 0.830 · 15183 op/s</sub> | -9.3% (-0.050) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.608<br><sub>context: p90 0.692 · p95 0.717 · p99 0.775 · 1625 op/s</sub> | 0.568<br><sub>context: p90 0.649 · p95 0.677 · p99 0.707 · 1746 op/s</sub> | -6.5% (-0.040) | 150% AND 2 ms | 🟢 |
| 8 | 0.596<br><sub>context: p90 0.728 · p95 0.781 · p99 0.925 · 12849 op/s</sub> | 0.500<br><sub>context: p90 0.608 · p95 0.642 · p99 0.727 · 15191 op/s</sub> | -16.0% (-0.095) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.742<br><sub>context: p90 0.890 · p95 0.943 · p99 1.073 · 1322 op/s</sub> | 0.728<br><sub>context: p90 0.853 · p95 0.916 · p99 1.013 · 1357 op/s</sub> | -2.0% (-0.015) | 150% AND 2 ms | 🟢 |
| 8 | 0.779<br><sub>context: p90 0.996 · p95 1.060 · p99 1.212 · 9911 op/s</sub> | 0.670<br><sub>context: p90 0.854 · p95 0.915 · p99 1.063 · 11522 op/s</sub> | -14.0% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.781<br><sub>context: p90 0.975 · p95 1.021 · p99 1.098 · 1251 op/s</sub> | 0.732<br><sub>context: p90 0.877 · p95 0.924 · p99 0.977 · 1351 op/s</sub> | -6.3% (-0.049) | 150% AND 2 ms | 🟢 |
| 8 | 0.824<br><sub>context: p90 1.055 · p95 1.135 · p99 1.320 · 9132 op/s</sub> | 0.701<br><sub>context: p90 0.895 · p95 0.959 · p99 1.136 · 10942 op/s</sub> | -15.0% (-0.123) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.385<br><sub>context: p90 1.885 · p95 2.025 · p99 2.291 · 704 op/s</sub> | 1.268<br><sub>context: p90 1.658 · p95 1.773 · p99 2.073 · 770 op/s</sub> | -8.5% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 1.618<br><sub>context: p90 2.275 · p95 2.500 · p99 2.880 · 4722 op/s</sub> | 1.363<br><sub>context: p90 1.876 · p95 2.042 · p99 2.423 · 5679 op/s</sub> | -15.8% (-0.255) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.463<br><sub>context: p90 1.926 · p95 2.152 · p99 2.346 · 663 op/s</sub> | 1.264<br><sub>context: p90 1.670 · p95 1.775 · p99 2.023 · 773 op/s</sub> | -13.6% (-0.199) | 150% AND 2 ms | 🟢 |
| 8 | 1.792<br><sub>context: p90 2.529 · p95 2.744 · p99 3.212 · 4255 op/s</sub> | 1.381<br><sub>context: p90 1.902 · p95 2.076 · p99 2.481 · 5508 op/s</sub> | -22.9% (-0.411) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.757<br><sub>context: p90 0.847 · p95 0.887 · p99 0.939 · 1335 op/s</sub> | 0.754<br><sub>context: p90 0.981 · p95 1.050 · p99 1.190 · 1306 op/s</sub> | -0.4% (-0.003) | 150% AND 2 ms | 🟢 |
| 8 | 0.785<br><sub>context: p90 0.927 · p95 0.972 · p99 1.059 · 9944 op/s</sub> | 0.796<br><sub>context: p90 1.066 · p95 1.128 · p99 1.240 · 9779 op/s</sub> | +1.3% (+0.010) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.531<br><sub>context: p90 0.622 · p95 0.642 · p99 0.673 · 1858 op/s</sub> | 0.433<br><sub>context: p90 0.534 · p95 0.564 · p99 0.607 · 2237 op/s</sub> | -18.5% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.566<br><sub>context: p90 0.670 · p95 0.704 · p99 0.802 · 13584 op/s</sub> | 0.412<br><sub>context: p90 0.517 · p95 0.549 · p99 0.614 · 18693 op/s</sub> | -27.2% (-0.154) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.364<br><sub>context: p90 0.451 · p95 0.471 · p99 0.514 · 2653 op/s</sub> | 0.177<br><sub>context: p90 0.258 · p95 0.292 · p99 0.325 · 5253 op/s</sub> | -51.4% (-0.187) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.417 · p95 0.446 · p99 0.507 · 22723 op/s</sub> | 0.232<br><sub>context: p90 0.300 · p95 0.323 · p99 0.371 · 32899 op/s</sub> | -30.2% (-0.101) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.241<br><sub>context: p90 0.302 · p95 0.336 · p99 0.381 · 3987 op/s</sub> | 0.166<br><sub>context: p90 0.212 · p95 0.237 · p99 0.249 · 5788 op/s</sub> | -30.9% (-0.074) | 150% AND 2 ms | 🟢 |
| 8 | 0.280<br><sub>context: p90 0.366 · p95 0.393 · p99 0.450 · 27039 op/s</sub> | 0.209<br><sub>context: p90 0.283 · p95 0.310 · p99 0.358 · 37174 op/s</sub> | -25.3% (-0.071) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.640<br><sub>context: p90 0.746 · p95 0.766 · p99 0.860 · 1538 op/s</sub> | 0.492<br><sub>context: p90 0.602 · p95 0.633 · p99 0.698 · 2000 op/s</sub> | -23.1% (-0.148) | 150% AND 2 ms | 🟢 |
| 8 | 0.670<br><sub>context: p90 0.790 · p95 0.831 · p99 0.930 · 11715 op/s</sub> | 0.449<br><sub>context: p90 0.555 · p95 0.589 · p99 0.646 · 17274 op/s</sub> | -33.0% (-0.221) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.954<br><sub>context: p90 1.270 · p95 1.357 · p99 1.530 · 1021 op/s</sub> | 1.611<br><sub>context: p90 2.344 · p95 2.577 · p99 2.896 · 596 op/s</sub> | +68.9% (+0.657) | 150% AND 2 ms | 🟢 |
| 8 | 1.086<br><sub>context: p90 1.585 · p95 1.749 · p99 2.036 · 7013 op/s</sub> | 2.302<br><sub>context: p90 3.563 · p95 4.023 · p99 4.561 · 3316 op/s</sub> | +112.0% (+1.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.608<br><sub>context: p90 4.005 · p95 4.467 · p99 4.847 · 366 op/s</sub> | 5.054<br><sub>context: p90 7.580 · p95 8.378 · p99 8.834 · 190 op/s</sub> | +93.8% (+2.446) | 150% AND 2 ms | 🟢 |
| 8 | 3.257<br><sub>context: p90 5.494 · p95 6.269 · p99 7.562 · 2248 op/s</sub> | 8.645<br><sub>context: p90 13.144 · p95 14.525 · p99 15.984 · 897 op/s</sub> | +165.4% (+5.387) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.364<br><sub>context: p90 0.463 · p95 0.499 · p99 0.549 · 2655 op/s</sub> | 0.248<br><sub>context: p90 0.345 · p95 0.376 · p99 0.483 · 3870 op/s</sub> | -31.9% (-0.116) | 150% AND 2 ms | 🟢 |
| 8 | 0.361<br><sub>context: p90 0.446 · p95 0.473 · p99 0.535 · 21088 op/s</sub> | 0.302<br><sub>context: p90 0.410 · p95 0.445 · p99 0.523 · 24874 op/s</sub> | -16.4% (-0.059) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.388<br><sub>context: p90 0.462 · p95 0.491 · p99 0.536 · 2562 op/s</sub> | 0.251<br><sub>context: p90 0.313 · p95 0.339 · p99 0.423 · 3935 op/s</sub> | -35.4% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.368<br><sub>context: p90 0.455 · p95 0.482 · p99 0.556 · 21043 op/s</sub> | 0.304<br><sub>context: p90 0.413 · p95 0.447 · p99 0.507 · 24764 op/s</sub> | -17.5% (-0.064) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.444<br><sub>context: p90 0.528 · p95 0.550 · p99 0.601 · 2221 op/s</sub> | 0.375<br><sub>context: p90 0.482 · p95 0.533 · p99 0.574 · 2535 op/s</sub> | -15.6% (-0.069) | 150% AND 2 ms | 🟢 |
| 8 | 0.498<br><sub>context: p90 0.613 · p95 0.651 · p99 0.727 · 15407 op/s</sub> | 0.372<br><sub>context: p90 0.475 · p95 0.517 · p99 0.600 · 20178 op/s</sub> | -25.3% (-0.126) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.406 · p95 0.448 · p99 0.477 · 3102 op/s</sub> | 0.195<br><sub>context: p90 0.243 · p95 0.257 · p99 0.290 · 4949 op/s</sub> | -38.2% (-0.121) | 150% AND 2 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.373 · p95 0.399 · p99 0.461 · 25705 op/s</sub> | 0.222<br><sub>context: p90 0.292 · p95 0.317 · p99 0.358 · 34393 op/s</sub> | -25.9% (-0.078) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.498<br><sub>context: p90 0.586 · p95 0.613 · p99 0.657 · 1981 op/s</sub> | 0.445<br><sub>context: p90 0.528 · p95 0.556 · p99 0.611 · 2217 op/s</sub> | -10.5% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.458<br><sub>context: p90 0.541 · p95 0.567 · p99 0.619 · 16834 op/s</sub> | 0.398<br><sub>context: p90 0.488 · p95 0.515 · p99 0.581 · 19411 op/s</sub> | -12.9% (-0.059) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.420<br><sub>context: p90 0.505 · p95 0.524 · p99 0.568 · 2358 op/s</sub> | 0.361<br><sub>context: p90 0.455 · p95 0.485 · p99 0.539 · 2738 op/s</sub> | -14.1% (-0.059) | 150% AND 2 ms | 🟢 |
| 8 | 0.386<br><sub>context: p90 0.468 · p95 0.503 · p99 0.556 · 19943 op/s</sub> | 0.331<br><sub>context: p90 0.428 · p95 0.458 · p99 0.514 · 22992 op/s</sub> | -14.4% (-0.056) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.572<br><sub>context: p90 0.678 · p95 0.701 · p99 0.764 · 1723 op/s</sub> | 0.510<br><sub>context: p90 0.623 · p95 0.669 · p99 0.722 · 1912 op/s</sub> | -10.8% (-0.062) | 150% AND 2 ms | 🟢 |
| 8 | 0.527<br><sub>context: p90 0.637 · p95 0.675 · p99 0.745 · 14765 op/s</sub> | 0.457<br><sub>context: p90 0.566 · p95 0.603 · p99 0.687 · 17016 op/s</sub> | -13.3% (-0.070) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.671<br><sub>context: p90 0.790 · p95 0.858 · p99 0.911 · 1472 op/s</sub> | 0.662<br><sub>context: p90 0.835 · p95 0.891 · p99 0.994 · 1476 op/s</sub> | -1.4% (-0.009) | 150% AND 2 ms | 🟢 |
| 8 | 0.692<br><sub>context: p90 0.905 · p95 0.968 · p99 1.090 · 11042 op/s</sub> | 0.649<br><sub>context: p90 0.825 · p95 0.872 · p99 0.999 · 11914 op/s</sub> | -6.2% (-0.043) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.700<br><sub>context: p90 0.842 · p95 0.859 · p99 0.969 · 1404 op/s</sub> | 0.712<br><sub>context: p90 0.826 · p95 0.865 · p99 0.970 · 1409 op/s</sub> | +1.6% (+0.011) | 150% AND 2 ms | 🟢 |
| 8 | 0.691<br><sub>context: p90 0.864 · p95 0.922 · p99 1.058 · 11119 op/s</sub> | 0.663<br><sub>context: p90 0.833 · p95 0.886 · p99 1.020 · 11622 op/s</sub> | -4.0% (-0.028) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.602<br><sub>context: p90 0.704 · p95 0.732 · p99 0.840 · 1632 op/s</sub> | 0.582<br><sub>context: p90 0.676 · p95 0.709 · p99 0.754 · 1715 op/s</sub> | -3.3% (-0.020) | 150% AND 2 ms | 🟢 |
| 8 | 0.576<br><sub>context: p90 0.698 · p95 0.746 · p99 0.832 · 13474 op/s</sub> | 0.511<br><sub>context: p90 0.639 · p95 0.679 · p99 0.769 · 15149 op/s</sub> | -11.2% (-0.065) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.551<br><sub>context: p90 0.631 · p95 0.661 · p99 0.699 · 1793 op/s</sub> | 0.407<br><sub>context: p90 0.520 · p95 0.560 · p99 0.615 · 2412 op/s</sub> | -26.2% (-0.144) | 150% AND 2 ms | 🟢 |
| 8 | 0.552<br><sub>context: p90 0.655 · p95 0.692 · p99 0.770 · 14121 op/s</sub> | 0.408<br><sub>context: p90 0.540 · p95 0.592 · p99 0.702 · 17454 op/s</sub> | -26.1% (-0.144) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.660<br><sub>context: p90 18.866 · p95 18.929 · p99 19.024 · 53 op/s</sub> | 15.387<br><sub>context: p90 15.694 · p95 15.798 · p99 16.066 · 64 op/s</sub> | -17.5% (-3.273) | 150% AND 2 ms | 🟢 |
| 8 | 23.812<br><sub>context: p90 31.315 · p95 34.362 · p99 38.187 · 310 op/s</sub> | 18.991<br><sub>context: p90 24.990 · p95 28.374 · p99 31.421 · 382 op/s</sub> | -20.2% (-4.821) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.645<br><sub>context: p90 0.745 · p95 0.780 · p99 0.813 · 1537 op/s</sub> | 0.528<br><sub>context: p90 0.640 · p95 0.668 · p99 0.760 · 1847 op/s</sub> | -18.2% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.665<br><sub>context: p90 0.787 · p95 0.827 · p99 0.939 · 11619 op/s</sub> | 0.520<br><sub>context: p90 0.651 · p95 0.706 · p99 0.803 · 14440 op/s</sub> | -21.8% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.664<br><sub>context: p90 2.180 · p95 2.394 · p99 2.730 · 593 op/s</sub> | 1.559<br><sub>context: p90 2.050 · p95 2.165 · p99 2.375 · 644 op/s</sub> | -6.3% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 1.879<br><sub>context: p90 2.616 · p95 2.845 · p99 3.430 · 4128 op/s</sub> | 1.677<br><sub>context: p90 2.317 · p95 2.478 · p99 2.870 · 4671 op/s</sub> | -10.7% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.593<br><sub>context: p90 0.694 · p95 0.729 · p99 0.803 · 1663 op/s</sub> | 0.609<br><sub>context: p90 0.724 · p95 0.757 · p99 0.818 · 1621 op/s</sub> | +2.7% (+0.016) | 150% AND 2 ms | 🟢 |
| 8 | 0.575<br><sub>context: p90 0.690 · p95 0.730 · p99 0.813 · 13537 op/s</sub> | 0.535<br><sub>context: p90 0.690 · p95 0.745 · p99 0.831 · 14408 op/s</sub> | -6.9% (-0.039) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.455<br><sub>context: p90 3.071 · p95 3.138 · p99 3.231 · 418 op/s</sub> | 0.552<br><sub>context: p90 0.674 · p95 0.709 · p99 0.745 · 1773 op/s</sub> | -77.5% (-1.904) | 150% AND 2 ms | 🟢 |
| 8 | 2.608<br><sub>context: p90 3.416 · p95 3.594 · p99 3.941 · 3038 op/s</sub> | 0.473<br><sub>context: p90 0.626 · p95 0.686 · p99 0.833 · 15943 op/s</sub> | -81.8% (-2.135) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.400<br><sub>context: p90 3.068 · p95 3.140 · p99 3.212 · 423 op/s</sub> | 0.561<br><sub>context: p90 0.658 · p95 0.696 · p99 0.761 · 1773 op/s</sub> | -76.6% (-1.839) | 150% AND 2 ms | 🟢 |
| 8 | 2.629<br><sub>context: p90 3.411 · p95 3.595 · p99 3.819 · 3098 op/s</sub> | 0.471<br><sub>context: p90 0.595 · p95 0.641 · p99 0.746 · 16124 op/s</sub> | -82.1% (-2.158) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.355<br><sub>context: p90 0.443 · p95 0.468 · p99 0.537 · 2801 op/s</sub> | 0.302<br><sub>context: p90 0.420 · p95 0.451 · p99 0.523 · 3156 op/s</sub> | -15.1% (-0.054) | 150% AND 2 ms | 🟢 |
| 8 | 0.306<br><sub>context: p90 0.383 · p95 0.414 · p99 0.484 · 24949 op/s</sub> | 0.312<br><sub>context: p90 0.415 · p95 0.450 · p99 0.522 · 24394 op/s</sub> | +1.9% (+0.006) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.400<br><sub>context: p90 0.467 · p95 0.494 · p99 0.537 · 2472 op/s</sub> | 0.195<br><sub>context: p90 0.275 · p95 0.308 · p99 0.363 · 4778 op/s</sub> | -51.3% (-0.206) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.420<br><sub>context: p90 0.509 · p95 0.538 · p99 0.615 · 18450 op/s</sub> | 0.246<br><sub>context: p90 0.316 · p95 0.339 · p99 0.403 · 30807 op/s</sub> | -41.3% (-0.173) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.434<br><sub>context: p90 0.524 · p95 0.558 · p99 0.617 · 2262 op/s</sub> | 0.366<br><sub>context: p90 0.451 · p95 0.484 · p99 0.562 · 2711 op/s</sub> | -15.8% (-0.068) | 150% AND 2 ms | 🟢 |
| 8 | 0.444<br><sub>context: p90 0.543 · p95 0.579 · p99 0.666 · 17421 op/s</sub> | 0.380<br><sub>context: p90 0.498 · p95 0.546 · p99 0.647 · 19767 op/s</sub> | -14.4% (-0.064) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.409<br><sub>context: p90 0.502 · p95 0.519 · p99 0.574 · 2376 op/s</sub> | 0.399<br><sub>context: p90 0.501 · p95 0.534 · p99 0.580 · 2465 op/s</sub> | -2.4% (-0.010) | 150% AND 2 ms | 🟢 |
| 8 | 0.453<br><sub>context: p90 0.547 · p95 0.585 · p99 0.648 · 16667 op/s</sub> | 0.379<br><sub>context: p90 0.505 · p95 0.539 · p99 0.609 · 19955 op/s</sub> | -16.2% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.437<br><sub>context: p90 0.531 · p95 0.549 · p99 0.579 · 2264 op/s</sub> | 0.319<br><sub>context: p90 0.409 · p95 0.427 · p99 0.460 · 3109 op/s</sub> | -27.0% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.394<br><sub>context: p90 0.476 · p95 0.502 · p99 0.576 · 19149 op/s</sub> | 0.322<br><sub>context: p90 0.431 · p95 0.469 · p99 0.537 · 23333 op/s</sub> | -18.4% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.588<br><sub>context: p90 1.697 · p95 1.742 · p99 1.836 · 627 op/s</sub> | 1.216<br><sub>context: p90 1.346 · p95 1.367 · p99 1.489 · 813 op/s</sub> | -23.4% (-0.371) | 150% AND 2 ms | 🟢 |
| 8 | 1.856<br><sub>context: p90 2.206 · p95 2.324 · p99 2.695 · 4185 op/s</sub> | 1.250<br><sub>context: p90 1.451 · p95 1.518 · p99 1.721 · 6235 op/s</sub> | -32.7% (-0.606) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.430<br><sub>context: p90 1.561 · p95 1.592 · p99 1.691 · 698 op/s</sub> | 1.219<br><sub>context: p90 1.316 · p95 1.347 · p99 1.414 · 816 op/s</sub> | -14.7% (-0.210) | 150% AND 2 ms | 🟢 |
| 8 | 1.639<br><sub>context: p90 1.993 · p95 2.100 · p99 2.477 · 4760 op/s</sub> | 1.151<br><sub>context: p90 1.313 · p95 1.358 · p99 1.515 · 6748 op/s</sub> | -29.7% (-0.487) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.453<br><sub>context: p90 0.557 · p95 0.593 · p99 0.657 · 2149 op/s</sub> | 0.390<br><sub>context: p90 0.482 · p95 0.530 · p99 0.579 · 2512 op/s</sub> | -14.0% (-0.063) | 150% AND 2 ms | 🟢 |
| 8 | 0.477<br><sub>context: p90 0.578 · p95 0.618 · p99 0.693 · 16261 op/s</sub> | 0.391<br><sub>context: p90 0.499 · p95 0.536 · p99 0.608 · 19816 op/s</sub> | -18.0% (-0.086) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.592<br><sub>context: p90 0.713 · p95 0.748 · p99 0.779 · 1658 op/s</sub> | 0.570<br><sub>context: p90 0.712 · p95 0.763 · p99 0.831 · 1721 op/s</sub> | -3.8% (-0.023) | 150% AND 2 ms | 🟢 |
| 8 | 0.592<br><sub>context: p90 0.712 · p95 0.747 · p99 0.839 · 13238 op/s</sub> | 0.501<br><sub>context: p90 0.638 · p95 0.676 · p99 0.777 · 15410 op/s</sub> | -15.3% (-0.090) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.447<br><sub>context: p90 0.543 · p95 0.575 · p99 0.623 · 2122 op/s</sub> | 0.324<br><sub>context: p90 0.445 · p95 0.469 · p99 0.515 · 2905 op/s</sub> | -27.5% (-0.123) | 150% AND 2 ms | 🟢 |
| 8 | 0.416<br><sub>context: p90 0.513 · p95 0.549 · p99 0.635 · 18247 op/s</sub> | 0.335<br><sub>context: p90 0.432 · p95 0.471 · p99 0.563 · 22462 op/s</sub> | -19.5% (-0.081) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.298<br><sub>context: p90 0.359 · p95 0.372 · p99 0.412 · 3309 op/s</sub> | 0.303<br><sub>context: p90 0.418 · p95 0.465 · p99 0.508 · 3175 op/s</sub> | +1.5% (+0.004) | 150% AND 2 ms | 🟢 |
| 8 | 0.302<br><sub>context: p90 0.380 · p95 0.407 · p99 0.460 · 25273 op/s</sub> | 0.332<br><sub>context: p90 0.458 · p95 0.507 · p99 0.618 · 22071 op/s</sub> | +10.0% (+0.030) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.343<br><sub>context: p90 0.441 · p95 0.470 · p99 0.565 · 2806 op/s</sub> | 0.350<br><sub>context: p90 0.466 · p95 0.519 · p99 0.556 · 2766 op/s</sub> | +2.1% (+0.007) | 150% AND 2 ms | 🟢 |
| 8 | 0.307<br><sub>context: p90 0.380 · p95 0.401 · p99 0.461 · 24666 op/s</sub> | 0.320<br><sub>context: p90 0.431 · p95 0.471 · p99 0.543 · 23367 op/s</sub> | +4.4% (+0.013) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.134<br><sub>context: p90 1.218 · p95 1.247 · p99 1.272 · 877 op/s</sub> | 0.594<br><sub>context: p90 0.695 · p95 0.713 · p99 0.805 · 1661 op/s</sub> | -47.7% (-0.541) | 150% AND 2 ms | 🟢 |
| 8 | 1.341<br><sub>context: p90 1.827 · p95 2.010 · p99 2.328 · 5663 op/s</sub> | 0.520<br><sub>context: p90 0.639 · p95 0.681 · p99 0.765 · 14789 op/s</sub> | -61.2% (-0.821) | 150% AND 2 ms | 🟢 |

</details>
