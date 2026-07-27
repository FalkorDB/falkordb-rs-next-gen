### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 | ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb |
| workload_hash | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` |
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

**pr vs main** — 🟢 no p50 regression beyond budget across 100 comparable cell(s)

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 → ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.883<br><sub>context: p90 0.973 · p95 1.016 · p99 1.048 · 1119 op/s</sub> | 0.935<br><sub>context: p90 1.037 · p95 1.075 · p99 1.093 · 1058 op/s</sub> | +5.9% (+0.052) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.921<br><sub>context: p90 1.069 · p95 1.108 · p99 1.205 · 8537 op/s</sub> | 0.919<br><sub>context: p90 1.059 · p95 1.102 · p99 1.213 · 8595 op/s</sub> | -0.2% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.955<br><sub>context: p90 1.031 · p95 1.059 · p99 1.105 · 1040 op/s</sub> | 0.954<br><sub>context: p90 1.050 · p95 1.071 · p99 1.120 · 1042 op/s</sub> | -0.2% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.029<br><sub>context: p90 1.230 · p95 1.266 · p99 1.355 · 7646 op/s</sub> | 1.026<br><sub>context: p90 1.224 · p95 1.264 · p99 1.359 · 7653 op/s</sub> | -0.3% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.511<br><sub>context: p90 1.621 · p95 1.641 · p99 1.670 · 657 op/s</sub> | 1.479<br><sub>context: p90 1.562 · p95 1.592 · p99 1.652 · 668 op/s</sub> | -2.1% (-0.032) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.799<br><sub>context: p90 2.312 · p95 2.471 · p99 2.774 · 4283 op/s</sub> | 1.812<br><sub>context: p90 2.344 · p95 2.536 · p99 2.805 · 4271 op/s</sub> | +0.7% (+0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.585<br><sub>context: p90 1.662 · p95 1.688 · p99 1.727 · 629 op/s</sub> | 1.562<br><sub>context: p90 1.660 · p95 1.696 · p99 1.737 · 636 op/s</sub> | -1.4% (-0.022) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.684<br><sub>context: p90 2.035 · p95 2.106 · p99 2.238 · 4595 op/s</sub> | 1.659<br><sub>context: p90 1.981 · p95 2.051 · p99 2.172 · 4663 op/s</sub> | -1.5% (-0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.167<br><sub>context: p90 0.221 · p95 0.231 · p99 0.299 · 5725 op/s</sub> | 0.173<br><sub>context: p90 0.242 · p95 0.256 · p99 0.307 · 5374 op/s</sub> | +3.5% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.217<br><sub>context: p90 0.293 · p95 0.314 · p99 0.364 · 34726 op/s</sub> | 0.225<br><sub>context: p90 0.290 · p95 0.308 · p99 0.350 · 34575 op/s</sub> | +3.6% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.376<br><sub>context: p90 0.474 · p95 0.504 · p99 0.535 · 2618 op/s</sub> | 0.375<br><sub>context: p90 0.466 · p95 0.493 · p99 0.544 · 2625 op/s</sub> | -0.3% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.336<br><sub>context: p90 0.436 · p95 0.468 · p99 0.549 · 22405 op/s</sub> | 0.339<br><sub>context: p90 0.439 · p95 0.469 · p99 0.552 · 21827 op/s</sub> | +0.7% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.428<br><sub>context: p90 0.519 · p95 0.553 · p99 0.577 · 2295 op/s</sub> | 0.424<br><sub>context: p90 0.516 · p95 0.544 · p99 0.612 · 2245 op/s</sub> | -0.9% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.371<br><sub>context: p90 0.484 · p95 0.527 · p99 0.643 · 19962 op/s</sub> | 0.359<br><sub>context: p90 0.461 · p95 0.497 · p99 0.567 · 21127 op/s</sub> | -3.4% (-0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.523<br><sub>context: p90 0.613 · p95 0.643 · p99 0.702 · 1892 op/s</sub> | 0.528<br><sub>context: p90 0.625 · p95 0.660 · p99 0.703 · 1871 op/s</sub> | +1.0% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.459<br><sub>context: p90 0.574 · p95 0.608 · p99 0.688 · 16687 op/s</sub> | 0.492<br><sub>context: p90 0.641 · p95 0.700 · p99 0.830 · 15183 op/s</sub> | +7.1% (+0.033) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.554<br><sub>context: p90 0.664 · p95 0.688 · p99 0.736 · 1779 op/s</sub> | 0.568<br><sub>context: p90 0.649 · p95 0.677 · p99 0.707 · 1746 op/s</sub> | +2.4% (+0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.535<br><sub>context: p90 0.662 · p95 0.710 · p99 0.808 · 14381 op/s</sub> | 0.500<br><sub>context: p90 0.608 · p95 0.642 · p99 0.727 · 15191 op/s</sub> | -6.4% (-0.034) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.681<br><sub>context: p90 0.825 · p95 0.848 · p99 0.950 · 1445 op/s</sub> | 0.728<br><sub>context: p90 0.853 · p95 0.916 · p99 1.013 · 1357 op/s</sub> | +6.8% (+0.046) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.672<br><sub>context: p90 0.849 · p95 0.915 · p99 1.050 · 11426 op/s</sub> | 0.670<br><sub>context: p90 0.854 · p95 0.915 · p99 1.063 · 11522 op/s</sub> | -0.4% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.720<br><sub>context: p90 0.873 · p95 0.914 · p99 0.975 · 1362 op/s</sub> | 0.732<br><sub>context: p90 0.877 · p95 0.924 · p99 0.977 · 1351 op/s</sub> | +1.6% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.718<br><sub>context: p90 0.921 · p95 0.995 · p99 1.130 · 10797 op/s</sub> | 0.701<br><sub>context: p90 0.895 · p95 0.959 · p99 1.136 · 10942 op/s</sub> | -2.4% (-0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.258<br><sub>context: p90 1.652 · p95 1.741 · p99 1.904 · 782 op/s</sub> | 1.268<br><sub>context: p90 1.658 · p95 1.773 · p99 2.073 · 770 op/s</sub> | +0.7% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.339<br><sub>context: p90 1.845 · p95 2.017 · p99 2.325 · 5726 op/s</sub> | 1.363<br><sub>context: p90 1.876 · p95 2.042 · p99 2.423 · 5679 op/s</sub> | +1.8% (+0.024) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.242<br><sub>context: p90 1.636 · p95 1.787 · p99 2.025 · 775 op/s</sub> | 1.264<br><sub>context: p90 1.670 · p95 1.775 · p99 2.023 · 773 op/s</sub> | +1.8% (+0.022) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.379<br><sub>context: p90 1.896 · p95 2.076 · p99 2.336 · 5572 op/s</sub> | 1.381<br><sub>context: p90 1.902 · p95 2.076 · p99 2.481 · 5508 op/s</sub> | +0.1% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.844<br><sub>context: p90 1.052 · p95 1.107 · p99 1.199 · 1179 op/s</sub> | 0.754<br><sub>context: p90 0.981 · p95 1.050 · p99 1.190 · 1306 op/s</sub> | -10.7% (-0.090) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.818<br><sub>context: p90 1.073 · p95 1.159 · p99 1.325 · 9560 op/s</sub> | 0.796<br><sub>context: p90 1.066 · p95 1.128 · p99 1.240 · 9779 op/s</sub> | -2.8% (-0.023) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.512<br><sub>context: p90 0.620 · p95 0.654 · p99 0.705 · 1925 op/s</sub> | 0.433<br><sub>context: p90 0.534 · p95 0.564 · p99 0.607 · 2237 op/s</sub> | -15.5% (-0.079) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.441<br><sub>context: p90 0.571 · p95 0.623 · p99 0.714 · 17217 op/s</sub> | 0.412<br><sub>context: p90 0.517 · p95 0.549 · p99 0.614 · 18693 op/s</sub> | -6.6% (-0.029) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.184<br><sub>context: p90 0.246 · p95 0.274 · p99 0.306 · 5073 op/s</sub> | 0.177<br><sub>context: p90 0.258 · p95 0.292 · p99 0.325 · 5253 op/s</sub> | -3.6% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.225<br><sub>context: p90 0.290 · p95 0.314 · p99 0.353 · 34360 op/s</sub> | 0.232<br><sub>context: p90 0.300 · p95 0.323 · p99 0.371 · 32899 op/s</sub> | +3.5% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.140<br><sub>context: p90 0.198 · p95 0.208 · p99 0.240 · 6628 op/s</sub> | 0.166<br><sub>context: p90 0.212 · p95 0.237 · p99 0.249 · 5788 op/s</sub> | +18.7% (+0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.206<br><sub>context: p90 0.277 · p95 0.299 · p99 0.347 · 37539 op/s</sub> | 0.209<br><sub>context: p90 0.283 · p95 0.310 · p99 0.358 · 37174 op/s</sub> | +1.6% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.477<br><sub>context: p90 0.570 · p95 0.596 · p99 0.628 · 2080 op/s</sub> | 0.492<br><sub>context: p90 0.602 · p95 0.633 · p99 0.698 · 2000 op/s</sub> | +3.2% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.458<br><sub>context: p90 0.568 · p95 0.601 · p99 0.682 · 16837 op/s</sub> | 0.449<br><sub>context: p90 0.555 · p95 0.589 · p99 0.646 · 17274 op/s</sub> | -2.1% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.664<br><sub>context: p90 2.341 · p95 2.558 · p99 2.903 · 588 op/s</sub> | 1.611<br><sub>context: p90 2.344 · p95 2.577 · p99 2.896 · 596 op/s</sub> | -3.2% (-0.053) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.325<br><sub>context: p90 3.603 · p95 3.989 · p99 4.565 · 3307 op/s</sub> | 2.302<br><sub>context: p90 3.563 · p95 4.023 · p99 4.561 · 3316 op/s</sub> | -1.0% (-0.023) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.103<br><sub>context: p90 7.613 · p95 8.572 · p99 8.994 · 189 op/s</sub> | 5.054<br><sub>context: p90 7.580 · p95 8.378 · p99 8.834 · 190 op/s</sub> | -1.0% (-0.049) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.603<br><sub>context: p90 13.211 · p95 14.467 · p99 16.443 · 895 op/s</sub> | 8.645<br><sub>context: p90 13.144 · p95 14.525 · p99 15.984 · 897 op/s</sub> | +0.5% (+0.041) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.246<br><sub>context: p90 0.325 · p95 0.352 · p99 0.390 · 3998 op/s</sub> | 0.248<br><sub>context: p90 0.345 · p95 0.376 · p99 0.483 · 3870 op/s</sub> | +0.8% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.306<br><sub>context: p90 0.414 · p95 0.448 · p99 0.533 · 24623 op/s</sub> | 0.302<br><sub>context: p90 0.410 · p95 0.445 · p99 0.523 · 24874 op/s</sub> | -1.4% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.253<br><sub>context: p90 0.320 · p95 0.337 · p99 0.381 · 4065 op/s</sub> | 0.251<br><sub>context: p90 0.313 · p95 0.339 · p99 0.423 · 3935 op/s</sub> | -0.8% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.308<br><sub>context: p90 0.415 · p95 0.465 · p99 0.536 · 24539 op/s</sub> | 0.304<br><sub>context: p90 0.413 · p95 0.447 · p99 0.507 · 24764 op/s</sub> | -1.3% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.394<br><sub>context: p90 0.539 · p95 0.586 · p99 0.636 · 2427 op/s</sub> | 0.375<br><sub>context: p90 0.482 · p95 0.533 · p99 0.574 · 2535 op/s</sub> | -5.0% (-0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.374<br><sub>context: p90 0.495 · p95 0.542 · p99 0.653 · 19771 op/s</sub> | 0.372<br><sub>context: p90 0.475 · p95 0.517 · p99 0.600 · 20178 op/s</sub> | -0.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.218<br><sub>context: p90 0.267 · p95 0.301 · p99 0.385 · 4503 op/s</sub> | 0.195<br><sub>context: p90 0.243 · p95 0.257 · p99 0.290 · 4949 op/s</sub> | -10.5% (-0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.232<br><sub>context: p90 0.317 · p95 0.347 · p99 0.441 · 32233 op/s</sub> | 0.222<br><sub>context: p90 0.292 · p95 0.317 · p99 0.358 · 34393 op/s</sub> | -3.9% (-0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.428<br><sub>context: p90 0.541 · p95 0.572 · p99 0.625 · 2278 op/s</sub> | 0.445<br><sub>context: p90 0.528 · p95 0.556 · p99 0.611 · 2217 op/s</sub> | +4.0% (+0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.406<br><sub>context: p90 0.504 · p95 0.533 · p99 0.595 · 18883 op/s</sub> | 0.398<br><sub>context: p90 0.488 · p95 0.515 · p99 0.581 · 19411 op/s</sub> | -1.8% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.357<br><sub>context: p90 0.452 · p95 0.485 · p99 0.533 · 2732 op/s</sub> | 0.361<br><sub>context: p90 0.455 · p95 0.485 · p99 0.539 · 2738 op/s</sub> | +1.2% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.421 · p95 0.458 · p99 0.544 · 22788 op/s</sub> | 0.331<br><sub>context: p90 0.428 · p95 0.458 · p99 0.514 · 22992 op/s</sub> | -1.2% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.523<br><sub>context: p90 0.639 · p95 0.676 · p99 0.719 · 1875 op/s</sub> | 0.510<br><sub>context: p90 0.623 · p95 0.669 · p99 0.722 · 1912 op/s</sub> | -2.3% (-0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.474<br><sub>context: p90 0.598 · p95 0.654 · p99 0.763 · 16135 op/s</sub> | 0.457<br><sub>context: p90 0.566 · p95 0.603 · p99 0.687 · 17016 op/s</sub> | -3.5% (-0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.649<br><sub>context: p90 0.791 · p95 0.849 · p99 0.891 · 1516 op/s</sub> | 0.662<br><sub>context: p90 0.835 · p95 0.891 · p99 0.994 · 1476 op/s</sub> | +1.9% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.646<br><sub>context: p90 0.839 · p95 0.902 · p99 1.024 · 11880 op/s</sub> | 0.649<br><sub>context: p90 0.825 · p95 0.872 · p99 0.999 · 11914 op/s</sub> | +0.5% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.668<br><sub>context: p90 0.795 · p95 0.834 · p99 0.878 · 1477 op/s</sub> | 0.712<br><sub>context: p90 0.826 · p95 0.865 · p99 0.970 · 1409 op/s</sub> | +6.5% (+0.043) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.670<br><sub>context: p90 0.835 · p95 0.897 · p99 0.997 · 11400 op/s</sub> | 0.663<br><sub>context: p90 0.833 · p95 0.886 · p99 1.020 · 11622 op/s</sub> | -1.0% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.580<br><sub>context: p90 0.671 · p95 0.690 · p99 0.771 · 1693 op/s</sub> | 0.582<br><sub>context: p90 0.676 · p95 0.709 · p99 0.754 · 1715 op/s</sub> | +0.2% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.491<br><sub>context: p90 0.612 · p95 0.651 · p99 0.733 · 15664 op/s</sub> | 0.511<br><sub>context: p90 0.639 · p95 0.679 · p99 0.769 · 15149 op/s</sub> | +4.1% (+0.020) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.465<br><sub>context: p90 0.558 · p95 0.578 · p99 0.654 · 2108 op/s</sub> | 0.407<br><sub>context: p90 0.520 · p95 0.560 · p99 0.615 · 2412 op/s</sub> | -12.5% (-0.058) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.404<br><sub>context: p90 0.510 · p95 0.554 · p99 0.631 · 18975 op/s</sub> | 0.408<br><sub>context: p90 0.540 · p95 0.592 · p99 0.702 · 17454 op/s</sub> | +0.9% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.351<br><sub>context: p90 15.511 · p95 15.546 · p99 15.661 · 65 op/s</sub> | 15.387<br><sub>context: p90 15.694 · p95 15.798 · p99 16.066 · 64 op/s</sub> | +0.2% (+0.036) | 10% AND 0.5 ms | 🟢 |
| 8 | 19.258<br><sub>context: p90 26.418 · p95 29.582 · p99 32.819 · 373 op/s</sub> | 18.991<br><sub>context: p90 24.990 · p95 28.374 · p99 31.421 · 382 op/s</sub> | -1.4% (-0.268) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.523<br><sub>context: p90 0.634 · p95 0.659 · p99 0.685 · 1872 op/s</sub> | 0.528<br><sub>context: p90 0.640 · p95 0.668 · p99 0.760 · 1847 op/s</sub> | +1.0% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.471<br><sub>context: p90 0.576 · p95 0.611 · p99 0.684 · 16210 op/s</sub> | 0.520<br><sub>context: p90 0.651 · p95 0.706 · p99 0.803 · 14440 op/s</sub> | +10.5% (+0.049) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.492<br><sub>context: p90 1.976 · p95 2.136 · p99 2.402 · 665 op/s</sub> | 1.559<br><sub>context: p90 2.050 · p95 2.165 · p99 2.375 · 644 op/s</sub> | +4.5% (+0.067) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.643<br><sub>context: p90 2.322 · p95 2.532 · p99 3.026 · 4634 op/s</sub> | 1.677<br><sub>context: p90 2.317 · p95 2.478 · p99 2.870 · 4671 op/s</sub> | +2.1% (+0.034) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.538<br><sub>context: p90 0.635 · p95 0.648 · p99 0.710 · 1822 op/s</sub> | 0.609<br><sub>context: p90 0.724 · p95 0.757 · p99 0.818 · 1621 op/s</sub> | +13.3% (+0.072) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.495<br><sub>context: p90 0.613 · p95 0.646 · p99 0.725 · 15738 op/s</sub> | 0.535<br><sub>context: p90 0.690 · p95 0.745 · p99 0.831 · 14408 op/s</sub> | +8.1% (+0.040) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.480<br><sub>context: p90 0.584 · p95 0.608 · p99 0.669 · 2046 op/s</sub> | 0.552<br><sub>context: p90 0.674 · p95 0.709 · p99 0.745 · 1773 op/s</sub> | +15.0% (+0.072) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.445<br><sub>context: p90 0.569 · p95 0.624 · p99 0.705 · 17208 op/s</sub> | 0.473<br><sub>context: p90 0.626 · p95 0.686 · p99 0.833 · 15943 op/s</sub> | +6.5% (+0.029) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.489<br><sub>context: p90 0.604 · p95 0.644 · p99 0.720 · 2022 op/s</sub> | 0.561<br><sub>context: p90 0.658 · p95 0.696 · p99 0.761 · 1773 op/s</sub> | +14.7% (+0.072) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.597 · p95 0.636 · p99 0.749 · 16318 op/s</sub> | 0.471<br><sub>context: p90 0.595 · p95 0.641 · p99 0.746 · 16124 op/s</sub> | +1.0% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.363 · p95 0.384 · p99 0.421 · 3468 op/s</sub> | 0.302<br><sub>context: p90 0.420 · p95 0.451 · p99 0.523 · 3156 op/s</sub> | +6.5% (+0.018) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.310<br><sub>context: p90 0.424 · p95 0.464 · p99 0.541 · 24213 op/s</sub> | 0.312<br><sub>context: p90 0.415 · p95 0.450 · p99 0.522 · 24394 op/s</sub> | +0.5% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.211<br><sub>context: p90 0.280 · p95 0.302 · p99 0.360 · 4467 op/s</sub> | 0.195<br><sub>context: p90 0.275 · p95 0.308 · p99 0.363 · 4778 op/s</sub> | -7.4% (-0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.244<br><sub>context: p90 0.315 · p95 0.338 · p99 0.392 · 31297 op/s</sub> | 0.246<br><sub>context: p90 0.316 · p95 0.339 · p99 0.403 · 30807 op/s</sub> | +1.2% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.425<br><sub>context: p90 0.539 · p95 0.589 · p99 0.685 · 2281 op/s</sub> | 0.366<br><sub>context: p90 0.451 · p95 0.484 · p99 0.562 · 2711 op/s</sub> | -14.0% (-0.059) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.389<br><sub>context: p90 0.523 · p95 0.585 · p99 0.695 · 19135 op/s</sub> | 0.380<br><sub>context: p90 0.498 · p95 0.546 · p99 0.647 · 19767 op/s</sub> | -2.2% (-0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.406<br><sub>context: p90 0.507 · p95 0.544 · p99 0.593 · 2418 op/s</sub> | 0.399<br><sub>context: p90 0.501 · p95 0.534 · p99 0.580 · 2465 op/s</sub> | -1.7% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.390<br><sub>context: p90 0.515 · p95 0.562 · p99 0.666 · 19379 op/s</sub> | 0.379<br><sub>context: p90 0.505 · p95 0.539 · p99 0.609 · 19955 op/s</sub> | -2.8% (-0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.341<br><sub>context: p90 0.431 · p95 0.474 · p99 0.516 · 2850 op/s</sub> | 0.319<br><sub>context: p90 0.409 · p95 0.427 · p99 0.460 · 3109 op/s</sub> | -6.4% (-0.022) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.318<br><sub>context: p90 0.417 · p95 0.442 · p99 0.517 · 23612 op/s</sub> | 0.322<br><sub>context: p90 0.431 · p95 0.469 · p99 0.537 · 23333 op/s</sub> | +1.2% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.176<br><sub>context: p90 1.269 · p95 1.319 · p99 1.366 · 845 op/s</sub> | 1.216<br><sub>context: p90 1.346 · p95 1.367 · p99 1.489 · 813 op/s</sub> | +3.4% (+0.040) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.203<br><sub>context: p90 1.389 · p95 1.445 · p99 1.548 · 6449 op/s</sub> | 1.250<br><sub>context: p90 1.451 · p95 1.518 · p99 1.721 · 6235 op/s</sub> | +3.9% (+0.046) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.106<br><sub>context: p90 1.208 · p95 1.230 · p99 1.287 · 890 op/s</sub> | 1.219<br><sub>context: p90 1.316 · p95 1.347 · p99 1.414 · 816 op/s</sub> | +10.3% (+0.114) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.109<br><sub>context: p90 1.247 · p95 1.287 · p99 1.370 · 7135 op/s</sub> | 1.151<br><sub>context: p90 1.313 · p95 1.358 · p99 1.515 · 6748 op/s</sub> | +3.8% (+0.042) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.470<br><sub>context: p90 0.559 · p95 0.588 · p99 0.677 · 2129 op/s</sub> | 0.390<br><sub>context: p90 0.482 · p95 0.530 · p99 0.579 · 2512 op/s</sub> | -17.0% (-0.080) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.399<br><sub>context: p90 0.512 · p95 0.544 · p99 0.633 · 19272 op/s</sub> | 0.391<br><sub>context: p90 0.499 · p95 0.536 · p99 0.608 · 19816 op/s</sub> | -2.0% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.611<br><sub>context: p90 0.725 · p95 0.758 · p99 0.835 · 1633 op/s</sub> | 0.570<br><sub>context: p90 0.712 · p95 0.763 · p99 0.831 · 1721 op/s</sub> | -6.8% (-0.042) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.498<br><sub>context: p90 0.630 · p95 0.667 · p99 0.747 · 15501 op/s</sub> | 0.501<br><sub>context: p90 0.638 · p95 0.676 · p99 0.777 · 15410 op/s</sub> | +0.6% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.280<br><sub>context: p90 0.346 · p95 0.362 · p99 0.434 · 3525 op/s</sub> | 0.324<br><sub>context: p90 0.445 · p95 0.469 · p99 0.515 · 2905 op/s</sub> | +15.9% (+0.044) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.325<br><sub>context: p90 0.424 · p95 0.455 · p99 0.513 · 23255 op/s</sub> | 0.335<br><sub>context: p90 0.432 · p95 0.471 · p99 0.563 · 22462 op/s</sub> | +2.8% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.353 · p95 0.376 · p99 0.407 · 3722 op/s</sub> | 0.303<br><sub>context: p90 0.418 · p95 0.465 · p99 0.508 · 3175 op/s</sub> | +15.9% (+0.041) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.307<br><sub>context: p90 0.417 · p95 0.454 · p99 0.514 · 24489 op/s</sub> | 0.332<br><sub>context: p90 0.458 · p95 0.507 · p99 0.618 · 22071 op/s</sub> | +8.2% (+0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.277<br><sub>context: p90 0.359 · p95 0.389 · p99 0.441 · 3574 op/s</sub> | 0.350<br><sub>context: p90 0.466 · p95 0.519 · p99 0.556 · 2766 op/s</sub> | +26.4% (+0.073) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.312<br><sub>context: p90 0.420 · p95 0.453 · p99 0.530 · 24289 op/s</sub> | 0.320<br><sub>context: p90 0.431 · p95 0.471 · p99 0.543 · 23367 op/s</sub> | +2.8% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.528<br><sub>context: p90 0.625 · p95 0.645 · p99 0.685 · 1874 op/s</sub> | 0.594<br><sub>context: p90 0.695 · p95 0.713 · p99 0.805 · 1661 op/s</sub> | +12.5% (+0.066) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.467<br><sub>context: p90 0.547 · p95 0.569 · p99 0.613 · 16833 op/s</sub> | 0.520<br><sub>context: p90 0.639 · p95 0.681 · p99 0.765 · 14789 op/s</sub> | +11.4% (+0.053) | 10% AND 0.5 ms | 🟢 |

</details>
