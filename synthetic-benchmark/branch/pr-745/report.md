<!-- synthetic-benchmark -->
## 🧪 Synthetic per-op regression — PR vs main (`x86`)

Identical recorded workload replayed into each engine image, measured **back-to-back on one runner**, one container at a time. 🟢 faster or within budget · 🔴 slower than budget **or** results differ · N/A no perf verdict. **Non-blocking.**

### 🧪 Synthetic per-op regression — pr vs main

⏱ Computed in 4m 32s (benchmark + reporting).

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 | ghcr.io/falkordb/falkordb-server@sha256:7fffa030ddea640b8e76391508d68177c27fbf4ee14a38003c1ebe7bf8870f33 |
| workload_hash | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` |
| samples / warmup | 200 / 50 | 200 / 50 |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 10% | 0.5 ms |
| `expand_hops_5` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `match_by_index` | 15% | 0.5 ms |
| `property_projection` | 15% | 0.5 ms |
| `return_const` | 15% | 0.5 ms |
| `shortest_path` | 12% (c16 18%, c32 25%) | 0.5 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs main** — 🟢 no p50 regression beyond budget across 2 comparable cell(s)

> ⚠ baseline and candidate ran the same FalkorDB module version (99.99.99) — there is no version delta to measure

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 → ghcr.io/falkordb/falkordb-server@sha256:7fffa030ddea640b8e76391508d68177c27fbf4ee14a38003c1ebe7bf8870f33

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>N/A <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.053<br><sub>context: p90 1.176 · p99 1.245 · 933 op/s</sub> | 0.870<br><sub>context: p90 1.006 · p99 1.100 · 1117 op/s</sub> | -17.4% (-0.183) | — | N/A |
| 8 | 1.103<br><sub>context: p90 1.524 · p99 1.997 · 6594 op/s</sub> | 0.945<br><sub>context: p90 1.080 · p99 1.255 · 8306 op/s</sub> | -14.4% (-0.159) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.078<br><sub>context: p90 1.221 · p99 1.345 · 907 op/s</sub> | 0.931<br><sub>context: p90 0.997 · p99 1.064 · 1067 op/s</sub> | -13.7% (-0.147) | — | N/A |
| 8 | 1.156<br><sub>context: p90 1.416 · p99 1.743 · 6636 op/s</sub> | 1.136<br><sub>context: p90 1.383 · p99 1.634 · 6795 op/s</sub> | -1.8% (-0.021) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.448<br><sub>context: p90 1.576 · p99 1.698 · 678 op/s</sub> | 1.523<br><sub>context: p90 1.653 · p99 1.743 · 648 op/s</sub> | +5.2% (+0.076) | — | N/A |
| 8 | 1.856<br><sub>context: p90 2.378 · p99 2.895 · 4130 op/s</sub> | 1.873<br><sub>context: p90 2.492 · p99 2.971 · 4045 op/s</sub> | +0.9% (+0.017) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.663<br><sub>context: p90 1.796 · p99 2.039 · 595 op/s</sub> | 1.494<br><sub>context: p90 1.596 · p99 1.645 · 662 op/s</sub> | -10.2% (-0.169) | — | N/A |
| 8 | 1.833<br><sub>context: p90 2.280 · p99 2.907 · 4132 op/s</sub> | 1.681<br><sub>context: p90 1.991 · p99 2.143 · 4666 op/s</sub> | -8.3% (-0.152) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.190 · p99 0.232 · 6273 op/s</sub> | 0.160<br><sub>context: p90 0.192 · p99 0.218 · 5913 op/s</sub> | +5.2% (+0.008) | — | N/A |
| 8 | 0.216<br><sub>context: p90 0.288 · p99 0.355 · 35305 op/s</sub> | 0.225<br><sub>context: p90 0.293 · p99 0.348 · 34244 op/s</sub> | +4.4% (+0.009) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.454<br><sub>context: p90 0.617 · p99 0.710 · 2149 op/s</sub> | 0.279<br><sub>context: p90 0.368 · p99 0.452 · 3468 op/s</sub> | -38.5% (-0.175) | — | N/A |
| 8 | 0.398<br><sub>context: p90 0.569 · p99 0.800 · 18321 op/s</sub> | 0.345<br><sub>context: p90 0.459 · p99 0.590 · 21806 op/s</sub> | -13.3% (-0.053) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.424<br><sub>context: p90 0.573 · p99 0.670 · 2212 op/s</sub> | 0.269<br><sub>context: p90 0.354 · p99 0.391 · 3549 op/s</sub> | -36.6% (-0.155) | — | N/A |
| 8 | 0.398<br><sub>context: p90 0.554 · p99 0.718 · 17995 op/s</sub> | 0.374<br><sub>context: p90 0.498 · p99 0.615 · 20248 op/s</sub> | -6.1% (-0.024) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.454<br><sub>context: p90 0.547 · p99 0.639 · 2161 op/s</sub> | 0.399<br><sub>context: p90 0.471 · p99 0.526 · 2509 op/s</sub> | -12.2% (-0.055) | — | N/A |
| 8 | 0.474<br><sub>context: p90 0.606 · p99 0.764 · 16017 op/s</sub> | 0.447<br><sub>context: p90 0.545 · p99 0.635 · 17439 op/s</sub> | -5.7% (-0.027) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.470<br><sub>context: p90 0.598 · p99 0.690 · 2055 op/s</sub> | 0.452<br><sub>context: p90 0.518 · p99 0.581 · 2182 op/s</sub> | -3.8% (-0.018) | — | N/A |
| 8 | 0.470<br><sub>context: p90 0.568 · p99 0.645 · 16541 op/s</sub> | 0.500<br><sub>context: p90 0.626 · p99 0.806 · 15352 op/s</sub> | +6.5% (+0.030) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.632<br><sub>context: p90 0.780 · p99 0.907 · 1534 op/s</sub> | 0.558<br><sub>context: p90 0.708 · p99 0.863 · 1735 op/s</sub> | -11.7% (-0.074) | — | N/A |
| 8 | 0.640<br><sub>context: p90 0.823 · p99 1.000 · 11957 op/s</sub> | 0.640<br><sub>context: p90 0.812 · p99 0.985 · 11991 op/s</sub> | -0.0% (-0.000) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.637<br><sub>context: p90 0.798 · p99 0.926 · 1530 op/s</sub> | 0.605<br><sub>context: p90 0.749 · p99 0.888 · 1576 op/s</sub> | -5.1% (-0.032) | — | N/A |
| 8 | 0.737<br><sub>context: p90 0.975 · p99 1.352 · 9635 op/s</sub> | 0.664<br><sub>context: p90 0.849 · p99 1.019 · 11589 op/s</sub> | -9.8% (-0.072) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.220<br><sub>context: p90 1.594 · p99 1.837 · 803 op/s</sub> | 1.075<br><sub>context: p90 1.401 · p99 1.813 · 916 op/s</sub> | -11.9% (-0.145) | — | N/A |
| 8 | 1.331<br><sub>context: p90 1.854 · p99 2.389 · 5764 op/s</sub> | 1.290<br><sub>context: p90 1.783 · p99 2.206 · 5962 op/s</sub> | -3.1% (-0.041) | — | N/A |

</details>

<details><summary>N/A <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.193<br><sub>context: p90 1.546 · p99 1.922 · 825 op/s</sub> | 1.127<br><sub>context: p90 1.457 · p99 1.913 · 863 op/s</sub> | -5.5% (-0.066) | — | N/A |
| 8 | 1.402<br><sub>context: p90 1.938 · p99 2.519 · 5429 op/s</sub> | 1.382<br><sub>context: p90 1.918 · p99 2.416 · 5531 op/s</sub> | -1.4% (-0.019) | — | N/A |

</details>

<details><summary>N/A <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.066<br><sub>context: p90 1.460 · p99 1.870 · 892 op/s</sub> | 0.890<br><sub>context: p90 1.125 · p99 1.343 · 1107 op/s</sub> | -16.5% (-0.176) | — | N/A |
| 8 | 1.112<br><sub>context: p90 2.238 · p99 3.895 · 4526 op/s</sub> | 0.931<br><sub>context: p90 1.306 · p99 1.733 · 7992 op/s</sub> | -16.2% (-0.180) | — | N/A |

</details>

<details><summary>N/A <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.575<br><sub>context: p90 0.744 · p99 0.904 · 1695 op/s</sub> | 0.453<br><sub>context: p90 0.563 · p99 0.710 · 2161 op/s</sub> | -21.2% (-0.122) | — | N/A |
| 8 | 0.542<br><sub>context: p90 0.767 · p99 1.046 · 13124 op/s</sub> | 0.504<br><sub>context: p90 0.781 · p99 1.068 · 14054 op/s</sub> | -7.1% (-0.039) | — | N/A |

</details>

<details><summary>N/A <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.600<br><sub>context: p90 0.824 · p99 0.997 · 1559 op/s</sub> | 0.224<br><sub>context: p90 0.304 · p99 0.375 · 4192 op/s</sub> | -62.7% (-0.376) | — | N/A |
| 8 | 0.484<br><sub>context: p90 1.076 · p99 1.604 · 11426 op/s</sub> | 0.312<br><sub>context: p90 0.683 · p99 1.000 · 20126 op/s</sub> | -35.5% (-0.172) | — | N/A |

</details>

<details><summary>N/A <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.586<br><sub>context: p90 0.791 · p99 0.944 · 1634 op/s</sub> | 0.411<br><sub>context: p90 0.635 · p99 0.763 · 2291 op/s</sub> | -29.8% (-0.175) | — | N/A |
| 8 | 0.476<br><sub>context: p90 0.798 · p99 1.298 · 11459 op/s</sub> | 0.309<br><sub>context: p90 0.732 · p99 1.214 · 19386 op/s</sub> | -35.2% (-0.168) | — | N/A |

</details>

<details><summary>N/A <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.787<br><sub>context: p90 1.081 · p99 1.515 · 1133 op/s</sub> | 0.482<br><sub>context: p90 0.680 · p99 0.868 · 1945 op/s</sub> | -38.8% (-0.305) | — | N/A |
| 8 | 0.685<br><sub>context: p90 1.407 · p99 2.436 · 7438 op/s</sub> | 0.714<br><sub>context: p90 1.139 · p99 1.619 · 9731 op/s</sub> | +4.3% (+0.030) | — | N/A |

</details>

<details><summary>N/A <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.972<br><sub>context: p90 2.953 · p99 3.954 · 469 op/s</sub> | 1.598<br><sub>context: p90 2.319 · p99 2.781 · 608 op/s</sub> | -19.0% (-0.374) | — | N/A |
| 8 | 2.706<br><sub>context: p90 7.070 · p99 11.768 · 1921 op/s</sub> | 2.246<br><sub>context: p90 3.497 · p99 4.456 · 3366 op/s</sub> | -17.0% (-0.461) | — | N/A |

</details>

<details><summary>N/A <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.701<br><sub>context: p90 9.103 · p99 11.450 · 166 op/s</sub> | 4.769<br><sub>context: p90 7.021 · p99 8.413 · 203 op/s</sub> | -16.4% (-0.932) | — | N/A |
| 8 | 14.611<br><sub>context: p90 21.694 · p99 32.513 · 552 op/s</sub> | 9.212<br><sub>context: p90 14.523 · p99 18.834 · 821 op/s</sub> | -36.9% (-5.399) | — | N/A |

</details>

<details><summary>N/A <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.557<br><sub>context: p90 0.734 · p99 0.998 · 1728 op/s</sub> | 0.511<br><sub>context: p90 0.647 · p99 0.826 · 2070 op/s</sub> | -8.3% (-0.046) | — | N/A |
| 8 | 0.575<br><sub>context: p90 1.100 · p99 1.730 · 11486 op/s</sub> | 0.467<br><sub>context: p90 0.852 · p99 1.208 · 14216 op/s</sub> | -18.8% (-0.108) | — | N/A |

</details>

<details><summary>N/A <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.623<br><sub>context: p90 0.816 · p99 0.964 · 1465 op/s</sub> | 0.628<br><sub>context: p90 0.772 · p99 0.943 · 1559 op/s</sub> | +0.8% (+0.005) | — | N/A |
| 8 | 0.478<br><sub>context: p90 1.076 · p99 1.728 · 12572 op/s</sub> | 0.396<br><sub>context: p90 0.617 · p99 0.882 · 17964 op/s</sub> | -17.1% (-0.082) | — | N/A |

</details>

<details><summary>N/A <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.542<br><sub>context: p90 0.690 · p99 0.800 · 1738 op/s</sub> | 0.437<br><sub>context: p90 0.658 · p99 0.786 · 2158 op/s</sub> | -19.4% (-0.105) | — | N/A |
| 8 | 0.574<br><sub>context: p90 1.075 · p99 1.649 · 11583 op/s</sub> | 0.439<br><sub>context: p90 0.706 · p99 0.960 · 16060 op/s</sub> | -23.5% (-0.135) | — | N/A |

</details>

<details><summary>N/A <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.496<br><sub>context: p90 0.680 · p99 0.932 · 2092 op/s</sub> | 0.206<br><sub>context: p90 0.326 · p99 0.482 · 4413 op/s</sub> | -58.5% (-0.290) | — | N/A |
| 8 | 0.343<br><sub>context: p90 0.799 · p99 1.218 · 18177 op/s</sub> | 0.257<br><sub>context: p90 0.389 · p99 0.547 · 27997 op/s</sub> | -24.8% (-0.085) | — | N/A |

</details>

<details><summary>N/A <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.840<br><sub>context: p90 1.092 · p99 1.368 · 1062 op/s</sub> | 0.613<br><sub>context: p90 0.842 · p99 1.018 · 1592 op/s</sub> | -27.0% (-0.227) | — | N/A |
| 8 | 0.704<br><sub>context: p90 1.237 · p99 2.169 · 7042 op/s</sub> | 0.600<br><sub>context: p90 1.005 · p99 1.486 · 10870 op/s</sub> | -14.8% (-0.104) | — | N/A |

</details>

<details><summary>N/A <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.681<br><sub>context: p90 0.984 · p99 1.266 · 1300 op/s</sub> | 0.471<br><sub>context: p90 0.613 · p99 0.802 · 2105 op/s</sub> | -30.8% (-0.210) | — | N/A |
| 8 | 0.674<br><sub>context: p90 1.356 · p99 2.376 · 7318 op/s</sub> | 0.356<br><sub>context: p90 0.475 · p99 0.623 · 20846 op/s</sub> | -47.2% (-0.318) | — | N/A |

</details>

<details><summary>N/A <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.513<br><sub>context: p90 0.647 · p99 0.735 · 1905 op/s</sub> | 0.372<br><sub>context: p90 0.462 · p99 0.538 · 2603 op/s</sub> | -27.5% (-0.141) | — | N/A |
| 8 | 0.480<br><sub>context: p90 0.625 · p99 0.810 · 15687 op/s</sub> | 0.461<br><sub>context: p90 0.581 · p99 0.685 · 16727 op/s</sub> | -3.9% (-0.019) | — | N/A |

</details>

<details><summary>N/A <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.634<br><sub>context: p90 0.827 · p99 0.967 · 1527 op/s</sub> | 0.572<br><sub>context: p90 0.753 · p99 0.943 · 1688 op/s</sub> | -9.8% (-0.062) | — | N/A |
| 8 | 0.682<br><sub>context: p90 0.910 · p99 1.207 · 10996 op/s</sub> | 0.604<br><sub>context: p90 0.789 · p99 0.982 · 12530 op/s</sub> | -11.4% (-0.077) | — | N/A |

</details>

<details><summary>N/A <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.685<br><sub>context: p90 0.851 · p99 0.953 · 1429 op/s</sub> | 0.639<br><sub>context: p90 0.781 · p99 0.873 · 1548 op/s</sub> | -6.7% (-0.046) | — | N/A |
| 8 | 0.752<br><sub>context: p90 1.040 · p99 1.365 · 9732 op/s</sub> | 0.634<br><sub>context: p90 0.809 · p99 0.990 · 12041 op/s</sub> | -15.7% (-0.118) | — | N/A |

</details>

<details><summary>N/A <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.525<br><sub>context: p90 0.668 · p99 0.798 · 1838 op/s</sub> | 0.442<br><sub>context: p90 0.549 · p99 0.651 · 2200 op/s</sub> | -15.8% (-0.083) | — | N/A |
| 8 | 0.764<br><sub>context: p90 1.263 · p99 1.967 · 8747 op/s</sub> | 0.470<br><sub>context: p90 0.582 · p99 0.701 · 16584 op/s</sub> | -38.4% (-0.293) | — | N/A |

</details>

<details><summary>N/A <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.457<br><sub>context: p90 0.632 · p99 0.859 · 2088 op/s</sub> | 0.547<br><sub>context: p90 0.691 · p99 0.818 · 1819 op/s</sub> | +19.7% (+0.090) | — | N/A |
| 8 | 0.476<br><sub>context: p90 0.696 · p99 0.981 · 15119 op/s</sub> | 0.525<br><sub>context: p90 0.761 · p99 1.055 · 13634 op/s</sub> | +10.3% (+0.049) | — | N/A |

</details>

<details><summary>N/A <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.484<br><sub>context: p90 16.083 · p99 16.714 · 64 op/s</sub> | 15.483<br><sub>context: p90 16.350 · p99 16.872 · 64 op/s</sub> | -0.0% (-0.000) | — | N/A |
| 8 | 22.475<br><sub>context: p90 32.887 · p99 41.422 · 323 op/s</sub> | 19.637<br><sub>context: p90 27.257 · p99 35.019 · 363 op/s</sub> | -12.6% (-2.838) | — | N/A |

</details>

<details><summary>N/A <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.481<br><sub>context: p90 0.626 · p99 0.735 · 2003 op/s</sub> | 0.447<br><sub>context: p90 0.522 · p99 0.648 · 2196 op/s</sub> | -7.1% (-0.034) | — | N/A |
| 8 | 0.475<br><sub>context: p90 0.580 · p99 0.705 · 16208 op/s</sub> | 0.452<br><sub>context: p90 0.558 · p99 0.648 · 17119 op/s</sub> | -4.8% (-0.023) | — | N/A |

</details>

<details><summary>N/A <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.399<br><sub>context: p90 1.892 · p99 2.389 · 704 op/s</sub> | 1.475<br><sub>context: p90 1.992 · p99 2.440 · 658 op/s</sub> | +5.4% (+0.076) | — | N/A |
| 8 | 1.881<br><sub>context: p90 2.854 · p99 4.234 · 3858 op/s</sub> | 1.683<br><sub>context: p90 2.394 · p99 3.103 · 4508 op/s</sub> | -10.5% (-0.198) | — | N/A |

</details>

<details><summary>N/A <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.735<br><sub>context: p90 0.898 · p99 1.069 · 1333 op/s</sub> | 0.529<br><sub>context: p90 0.655 · p99 0.764 · 1842 op/s</sub> | -28.0% (-0.206) | — | N/A |
| 8 | 0.559<br><sub>context: p90 0.815 · p99 1.164 · 12618 op/s</sub> | 0.551<br><sub>context: p90 0.771 · p99 1.098 · 13210 op/s</sub> | -1.5% (-0.008) | — | N/A |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.444<br><sub>context: p90 0.563 · p99 0.652 · 2186 op/s</sub> | 0.486<br><sub>context: p90 0.636 · p99 0.732 · 2024 op/s</sub> | +9.5% (+0.042) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.568 · p99 0.705 · 17445 op/s</sub> | 0.517<br><sub>context: p90 0.761 · p99 1.034 · 14086 op/s</sub> | +18.2% (+0.080) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>N/A <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.443<br><sub>context: p90 0.548 · p99 0.641 · 2209 op/s</sub> | 0.398<br><sub>context: p90 0.488 · p99 0.645 · 2485 op/s</sub> | -10.2% (-0.045) | — | N/A |
| 8 | 0.484<br><sub>context: p90 0.678 · p99 0.928 · 14956 op/s</sub> | 0.448<br><sub>context: p90 0.564 · p99 0.687 · 16758 op/s</sub> | -7.5% (-0.036) | — | N/A |

</details>

<details><summary>N/A <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.337<br><sub>context: p90 0.488 · p99 0.566 · 2854 op/s</sub> | 0.227<br><sub>context: p90 0.333 · p99 0.432 · 4117 op/s</sub> | -32.7% (-0.110) | — | N/A |
| 8 | 0.347<br><sub>context: p90 0.531 · p99 0.750 · 20378 op/s</sub> | 0.323<br><sub>context: p90 0.456 · p99 0.629 · 22744 op/s</sub> | -6.7% (-0.023) | — | N/A |

</details>

<details><summary>N/A <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.549<br><sub>context: p90 0.704 · p99 0.928 · 1796 op/s</sub> | 0.521<br><sub>context: p90 0.664 · p99 0.790 · 1928 op/s</sub> | -5.1% (-0.028) | — | N/A |
| 8 | 0.416<br><sub>context: p90 1.037 · p99 1.752 · 13307 op/s</sub> | 0.366<br><sub>context: p90 0.581 · p99 0.858 · 19403 op/s</sub> | -12.0% (-0.050) | — | N/A |

</details>

<details><summary>N/A <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.491<br><sub>context: p90 0.652 · p99 0.767 · 1968 op/s</sub> | 0.435<br><sub>context: p90 0.574 · p99 0.739 · 2186 op/s</sub> | -11.4% (-0.056) | — | N/A |
| 8 | 0.642<br><sub>context: p90 1.322 · p99 2.344 · 8819 op/s</sub> | 0.417<br><sub>context: p90 0.586 · p99 0.796 · 17656 op/s</sub> | -35.0% (-0.224) | — | N/A |

</details>

<details><summary>N/A <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.521<br><sub>context: p90 0.668 · p99 0.788 · 1901 op/s</sub> | 0.389<br><sub>context: p90 0.514 · p99 0.721 · 2378 op/s</sub> | -25.4% (-0.133) | — | N/A |
| 8 | 0.465<br><sub>context: p90 0.764 · p99 1.151 · 12363 op/s</sub> | 0.470<br><sub>context: p90 0.711 · p99 0.999 · 15286 op/s</sub> | +1.1% (+0.005) | — | N/A |

</details>

<details><summary>N/A <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.394<br><sub>context: p90 0.754 · p99 1.039 · 2114 op/s</sub> | 0.484<br><sub>context: p90 0.646 · p99 0.785 · 2079 op/s</sub> | +22.7% (+0.090) | — | N/A |
| 8 | 0.467<br><sub>context: p90 0.830 · p99 1.380 · 10690 op/s</sub> | 0.515<br><sub>context: p90 0.886 · p99 1.421 · 12763 op/s</sub> | +10.5% (+0.049) | — | N/A |

</details>

<details><summary>N/A <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.275<br><sub>context: p90 1.461 · p99 1.641 · 776 op/s</sub> | 1.181<br><sub>context: p90 1.295 · p99 1.384 · 841 op/s</sub> | -7.4% (-0.095) | — | N/A |
| 8 | 1.394<br><sub>context: p90 1.813 · p99 2.374 · 5398 op/s</sub> | 1.268<br><sub>context: p90 1.529 · p99 1.800 · 6131 op/s</sub> | -9.1% (-0.126) | — | N/A |

</details>

<details><summary>N/A <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.106<br><sub>context: p90 1.233 · p99 1.336 · 884 op/s</sub> | 1.049<br><sub>context: p90 1.135 · p99 1.236 · 942 op/s</sub> | -5.2% (-0.057) | — | N/A |
| 8 | 1.261<br><sub>context: p90 1.568 · p99 1.945 · 5987 op/s</sub> | 1.146<br><sub>context: p90 1.332 · p99 1.539 · 6857 op/s</sub> | -9.2% (-0.115) | — | N/A |

</details>

<details><summary>N/A <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.609<br><sub>context: p90 0.880 · p99 1.214 · 1499 op/s</sub> | 0.680<br><sub>context: p90 0.867 · p99 1.052 · 1436 op/s</sub> | +11.6% (+0.071) | — | N/A |
| 8 | 0.634<br><sub>context: p90 1.409 · p99 2.464 · 8017 op/s</sub> | 0.558<br><sub>context: p90 0.913 · p99 1.414 · 11921 op/s</sub> | -12.0% (-0.076) | — | N/A |

</details>

<details><summary>N/A <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.881<br><sub>context: p90 1.133 · p99 1.370 · 1074 op/s</sub> | 0.615<br><sub>context: p90 0.839 · p99 1.102 · 1546 op/s</sub> | -30.2% (-0.266) | — | N/A |
| 8 | 0.848<br><sub>context: p90 1.808 · p99 3.051 · 6183 op/s</sub> | 0.544<br><sub>context: p90 0.796 · p99 1.121 · 13201 op/s</sub> | -35.9% (-0.304) | — | N/A |

</details>

<details><summary>N/A <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.655<br><sub>context: p90 0.756 · p99 0.925 · 1500 op/s</sub> | 0.406<br><sub>context: p90 0.658 · p99 0.861 · 2155 op/s</sub> | -38.1% (-0.250) | — | N/A |
| 8 | 0.531<br><sub>context: p90 1.250 · p99 2.157 · 10498 op/s</sub> | 0.464<br><sub>context: p90 0.743 · p99 1.064 · 14231 op/s</sub> | -12.6% (-0.067) | — | N/A |

</details>

<details><summary>N/A <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.336<br><sub>context: p90 0.503 · p99 0.735 · 2772 op/s</sub> | 0.281<br><sub>context: p90 0.418 · p99 0.481 · 3283 op/s</sub> | -16.2% (-0.055) | — | N/A |
| 8 | 0.337<br><sub>context: p90 0.499 · p99 0.737 · 20699 op/s</sub> | 0.349<br><sub>context: p90 0.523 · p99 0.701 · 20641 op/s</sub> | +3.8% (+0.013) | — | N/A |

</details>

<details><summary>N/A <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.407<br><sub>context: p90 0.598 · p99 0.680 · 2374 op/s</sub> | 0.260<br><sub>context: p90 0.333 · p99 0.413 · 3696 op/s</sub> | -36.1% (-0.147) | — | N/A |
| 8 | 0.323<br><sub>context: p90 0.469 · p99 0.681 · 22422 op/s</sub> | 0.382<br><sub>context: p90 0.630 · p99 0.938 · 18187 op/s</sub> | +18.4% (+0.059) | — | N/A |

</details>

<details><summary>N/A <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.494<br><sub>context: p90 0.660 · p99 0.822 · 1900 op/s</sub> | 0.450<br><sub>context: p90 0.537 · p99 0.610 · 2162 op/s</sub> | -8.8% (-0.043) | — | N/A |
| 8 | 0.536<br><sub>context: p90 0.738 · p99 0.980 · 13437 op/s</sub> | 0.457<br><sub>context: p90 0.547 · p99 0.636 · 16897 op/s</sub> | -14.7% (-0.079) | — | N/A |

</details>
