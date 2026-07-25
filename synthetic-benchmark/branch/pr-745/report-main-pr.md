### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 | ghcr.io/falkordb/falkordb-server@sha256:0b7e1e313cced37a47421e2414707354e36ac241692e24dc766f2ddaa75d5aee |
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

**pr vs main** — 🟢 no p50 regression beyond budget across 100 comparable cell(s)

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 → ghcr.io/falkordb/falkordb-server@sha256:0b7e1e313cced37a47421e2414707354e36ac241692e24dc766f2ddaa75d5aee

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.688<br><sub>context: p90 0.736 · p95 0.751 · p99 0.796 · 1431 op/s</sub> | 0.722<br><sub>context: p90 0.831 · p95 0.847 · p99 0.928 · 1354 op/s</sub> | +5.0% (+0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.836<br><sub>context: p90 0.988 · p95 1.029 · p99 1.116 · 9415 op/s</sub> | 0.832<br><sub>context: p90 0.989 · p95 1.036 · p99 1.114 · 9360 op/s</sub> | -0.4% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.766<br><sub>context: p90 0.805 · p95 0.820 · p99 0.841 · 1292 op/s</sub> | 0.792<br><sub>context: p90 0.868 · p95 0.893 · p99 0.939 · 1238 op/s</sub> | +3.4% (+0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.983<br><sub>context: p90 1.220 · p95 1.261 · p99 1.338 · 7865 op/s</sub> | 0.998<br><sub>context: p90 1.234 · p95 1.280 · p99 1.357 · 7814 op/s</sub> | +1.5% (+0.015) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.159<br><sub>context: p90 1.225 · p95 1.252 · p99 1.294 · 851 op/s</sub> | 1.157<br><sub>context: p90 1.224 · p95 1.240 · p99 1.291 · 852 op/s</sub> | -0.2% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.675<br><sub>context: p90 2.442 · p95 2.651 · p99 3.097 · 4411 op/s</sub> | 1.564<br><sub>context: p90 2.204 · p95 2.429 · p99 2.782 · 4743 op/s</sub> | -6.6% (-0.111) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.190<br><sub>context: p90 1.237 · p95 1.258 · p99 1.301 · 831 op/s</sub> | 1.216<br><sub>context: p90 1.297 · p95 1.315 · p99 1.339 · 813 op/s</sub> | +2.2% (+0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.425<br><sub>context: p90 1.722 · p95 1.829 · p99 1.940 · 5400 op/s</sub> | 1.429<br><sub>context: p90 1.735 · p95 1.823 · p99 1.955 · 5371 op/s</sub> | +0.3% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.143<br><sub>context: p90 0.163 · p95 0.170 · p99 0.201 · 6732 op/s</sub> | 0.130<br><sub>context: p90 0.150 · p95 0.155 · p99 0.161 · 7481 op/s</sub> | -9.1% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.223<br><sub>context: p90 0.296 · p95 0.319 · p99 0.357 · 34369 op/s</sub> | 0.221<br><sub>context: p90 0.291 · p95 0.311 · p99 0.351 · 34965 op/s</sub> | -1.1% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.200<br><sub>context: p90 0.235 · p95 0.245 · p99 0.262 · 4868 op/s</sub> | 0.200<br><sub>context: p90 0.238 · p95 0.246 · p99 0.261 · 4875 op/s</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.325<br><sub>context: p90 0.420 · p95 0.447 · p99 0.503 · 23305 op/s</sub> | 0.330<br><sub>context: p90 0.436 · p95 0.470 · p99 0.530 · 22823 op/s</sub> | +1.4% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.242<br><sub>context: p90 0.308 · p95 0.345 · p99 0.405 · 3901 op/s</sub> | 0.251<br><sub>context: p90 0.298 · p95 0.314 · p99 0.365 · 3865 op/s</sub> | +4.1% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.356<br><sub>context: p90 0.458 · p95 0.491 · p99 0.543 · 21383 op/s</sub> | 0.348<br><sub>context: p90 0.442 · p95 0.472 · p99 0.548 · 21962 op/s</sub> | -2.1% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.487 · p95 0.507 · p99 0.562 · 2577 op/s</sub> | 0.331<br><sub>context: p90 0.399 · p95 0.433 · p99 0.465 · 2951 op/s</sub> | -10.7% (-0.039) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.456<br><sub>context: p90 0.561 · p95 0.586 · p99 0.689 · 17028 op/s</sub> | 0.459<br><sub>context: p90 0.568 · p95 0.610 · p99 0.695 · 16760 op/s</sub> | +0.7% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.362<br><sub>context: p90 0.413 · p95 0.429 · p99 0.494 · 2693 op/s</sub> | 0.349<br><sub>context: p90 0.420 · p95 0.437 · p99 0.474 · 2776 op/s</sub> | -3.7% (-0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.481<br><sub>context: p90 0.596 · p95 0.631 · p99 0.697 · 16010 op/s</sub> | 0.464<br><sub>context: p90 0.570 · p95 0.598 · p99 0.655 · 16611 op/s</sub> | -3.6% (-0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.465<br><sub>context: p90 0.538 · p95 0.570 · p99 0.646 · 2124 op/s</sub> | 0.466<br><sub>context: p90 0.556 · p95 0.576 · p99 0.639 · 2127 op/s</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.602<br><sub>context: p90 0.760 · p95 0.805 · p99 0.906 · 12804 op/s</sub> | 0.600<br><sub>context: p90 0.759 · p95 0.816 · p99 0.900 · 12941 op/s</sub> | -0.3% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.482<br><sub>context: p90 0.568 · p95 0.603 · p99 0.643 · 2036 op/s</sub> | 0.475<br><sub>context: p90 0.577 · p95 0.620 · p99 0.672 · 2065 op/s</sub> | -1.3% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.632<br><sub>context: p90 0.800 · p95 0.855 · p99 0.961 · 12188 op/s</sub> | 0.615<br><sub>context: p90 0.774 · p95 0.818 · p99 0.916 · 12633 op/s</sub> | -2.8% (-0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.924<br><sub>context: p90 1.187 · p95 1.311 · p99 1.630 · 1068 op/s</sub> | 0.917<br><sub>context: p90 1.234 · p95 1.335 · p99 1.607 · 1060 op/s</sub> | -0.8% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.161<br><sub>context: p90 1.603 · p95 1.742 · p99 2.014 · 6644 op/s</sub> | 1.127<br><sub>context: p90 1.569 · p95 1.692 · p99 2.020 · 6791 op/s</sub> | -3.0% (-0.035) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.871<br><sub>context: p90 1.163 · p95 1.302 · p99 1.500 · 1098 op/s</sub> | 0.874<br><sub>context: p90 1.136 · p95 1.335 · p99 1.527 · 1092 op/s</sub> | +0.3% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.192<br><sub>context: p90 1.645 · p95 1.781 · p99 2.045 · 6399 op/s</sub> | 1.183<br><sub>context: p90 1.632 · p95 1.789 · p99 2.070 · 6463 op/s</sub> | -0.8% (-0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.539<br><sub>context: p90 0.691 · p95 0.721 · p99 0.770 · 1802 op/s</sub> | 0.605<br><sub>context: p90 0.756 · p95 0.791 · p99 0.871 · 1643 op/s</sub> | +12.2% (+0.066) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.818<br><sub>context: p90 1.103 · p95 1.192 · p99 1.334 · 9539 op/s</sub> | 0.842<br><sub>context: p90 1.148 · p95 1.232 · p99 1.379 · 9224 op/s</sub> | +3.0% (+0.024) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.311<br><sub>context: p90 0.412 · p95 0.444 · p99 0.538 · 3031 op/s</sub> | 0.300<br><sub>context: p90 0.366 · p95 0.380 · p99 0.454 · 3214 op/s</sub> | -3.7% (-0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.422<br><sub>context: p90 0.531 · p95 0.570 · p99 0.664 · 18233 op/s</sub> | 0.415<br><sub>context: p90 0.516 · p95 0.547 · p99 0.617 · 18565 op/s</sub> | -1.7% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.139<br><sub>context: p90 0.169 · p95 0.176 · p99 0.188 · 6817 op/s</sub> | 0.152<br><sub>context: p90 0.178 · p95 0.186 · p99 0.212 · 6241 op/s</sub> | +8.7% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.226<br><sub>context: p90 0.298 · p95 0.322 · p99 0.357 · 33687 op/s</sub> | 0.235<br><sub>context: p90 0.305 · p95 0.322 · p99 0.367 · 32365 op/s</sub> | +4.0% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.125<br><sub>context: p90 0.134 · p95 0.136 · p99 0.141 · 8084 op/s</sub> | 0.121<br><sub>context: p90 0.142 · p95 0.146 · p99 0.161 · 7864 op/s</sub> | -3.0% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.206<br><sub>context: p90 0.281 · p95 0.298 · p99 0.347 · 37477 op/s</sub> | 0.205<br><sub>context: p90 0.276 · p95 0.299 · p99 0.356 · 37680 op/s</sub> | -0.8% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.374 · p95 0.398 · p99 0.495 · 3021 op/s</sub> | 0.346<br><sub>context: p90 0.390 · p95 0.407 · p99 0.480 · 2881 op/s</sub> | +7.0% (+0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.539 · p95 0.570 · p99 0.633 · 17781 op/s</sub> | 0.439<br><sub>context: p90 0.544 · p95 0.579 · p99 0.628 · 17631 op/s</sub> | +1.1% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.087<br><sub>context: p90 1.672 · p95 1.791 · p99 2.045 · 875 op/s</sub> | 1.154<br><sub>context: p90 1.689 · p95 1.889 · p99 2.121 · 840 op/s</sub> | +6.1% (+0.067) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.872<br><sub>context: p90 4.588 · p95 5.208 · p99 5.970 · 2650 op/s</sub> | 3.084<br><sub>context: p90 5.070 · p95 5.735 · p99 6.943 · 2430 op/s</sub> | +7.4% (+0.212) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.852<br><sub>context: p90 5.793 · p95 6.379 · p99 6.805 · 250 op/s</sub> | 3.847<br><sub>context: p90 5.828 · p95 6.354 · p99 6.841 · 252 op/s</sub> | -0.1% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 13.263<br><sub>context: p90 19.965 · p95 21.690 · p99 24.335 · 588 op/s</sub> | 12.761<br><sub>context: p90 19.368 · p95 21.351 · p99 23.493 · 612 op/s</sub> | -3.8% (-0.501) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.163<br><sub>context: p90 0.197 · p95 0.205 · p99 0.215 · 5853 op/s</sub> | 0.167<br><sub>context: p90 0.192 · p95 0.203 · p99 0.212 · 5712 op/s</sub> | +2.1% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.402 · p95 0.439 · p99 0.524 · 25248 op/s</sub> | 0.304<br><sub>context: p90 0.416 · p95 0.451 · p99 0.524 · 25068 op/s</sub> | +1.2% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.157<br><sub>context: p90 0.170 · p95 0.180 · p99 0.185 · 6194 op/s</sub> | 0.160<br><sub>context: p90 0.193 · p95 0.198 · p99 0.208 · 5797 op/s</sub> | +1.8% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.411 · p95 0.455 · p99 0.549 · 24953 op/s</sub> | 0.307<br><sub>context: p90 0.418 · p95 0.450 · p99 0.537 · 24553 op/s</sub> | +2.3% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.215<br><sub>context: p90 0.248 · p95 0.253 · p99 0.269 · 4462 op/s</sub> | 0.243<br><sub>context: p90 0.287 · p95 0.302 · p99 0.348 · 3918 op/s</sub> | +13.0% (+0.028) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.348<br><sub>context: p90 0.446 · p95 0.488 · p99 0.580 · 21820 op/s</sub> | 0.346<br><sub>context: p90 0.444 · p95 0.479 · p99 0.559 · 21812 op/s</sub> | -0.5% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.136<br><sub>context: p90 0.166 · p95 0.171 · p99 0.208 · 6610 op/s</sub> | 0.151<br><sub>context: p90 0.174 · p95 0.180 · p99 0.198 · 6221 op/s</sub> | +10.7% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.228<br><sub>context: p90 0.301 · p95 0.326 · p99 0.367 · 33665 op/s</sub> | 0.224<br><sub>context: p90 0.298 · p95 0.321 · p99 0.366 · 33760 op/s</sub> | -1.4% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.364 · p95 0.380 · p99 0.414 · 3112 op/s</sub> | 0.317<br><sub>context: p90 0.377 · p95 0.410 · p99 0.473 · 3085 op/s</sub> | +0.5% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.377<br><sub>context: p90 0.464 · p95 0.492 · p99 0.548 · 20302 op/s</sub> | 0.387<br><sub>context: p90 0.480 · p95 0.512 · p99 0.566 · 19691 op/s</sub> | +2.6% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.233<br><sub>context: p90 0.323 · p95 0.357 · p99 0.375 · 4057 op/s</sub> | 0.227<br><sub>context: p90 0.286 · p95 0.306 · p99 0.350 · 4171 op/s</sub> | -2.8% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.330<br><sub>context: p90 0.425 · p95 0.457 · p99 0.526 · 23075 op/s</sub> | 0.337<br><sub>context: p90 0.432 · p95 0.463 · p99 0.519 · 22720 op/s</sub> | +2.0% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.303<br><sub>context: p90 0.336 · p95 0.350 · p99 0.378 · 3266 op/s</sub> | 0.342<br><sub>context: p90 0.446 · p95 0.474 · p99 0.534 · 2836 op/s</sub> | +12.9% (+0.039) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.432<br><sub>context: p90 0.536 · p95 0.567 · p99 0.629 · 18011 op/s</sub> | 0.449<br><sub>context: p90 0.561 · p95 0.600 · p99 0.687 · 17246 op/s</sub> | +3.9% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.420<br><sub>context: p90 0.506 · p95 0.534 · p99 0.598 · 2333 op/s</sub> | 0.436<br><sub>context: p90 0.523 · p95 0.556 · p99 0.603 · 2267 op/s</sub> | +3.8% (+0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.575<br><sub>context: p90 0.741 · p95 0.787 · p99 0.919 · 13206 op/s</sub> | 0.588<br><sub>context: p90 0.751 · p95 0.817 · p99 0.929 · 13154 op/s</sub> | +2.3% (+0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.421<br><sub>context: p90 0.501 · p95 0.526 · p99 0.576 · 2325 op/s</sub> | 0.425<br><sub>context: p90 0.515 · p95 0.543 · p99 0.634 · 2298 op/s</sub> | +1.1% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.592<br><sub>context: p90 0.745 · p95 0.800 · p99 0.894 · 12983 op/s</sub> | 0.593<br><sub>context: p90 0.760 · p95 0.811 · p99 0.906 · 13082 op/s</sub> | +0.2% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.366<br><sub>context: p90 0.422 · p95 0.442 · p99 0.496 · 2693 op/s</sub> | 0.351<br><sub>context: p90 0.435 · p95 0.468 · p99 0.503 · 2739 op/s</sub> | -4.4% (-0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.585 · p95 0.620 · p99 0.689 · 16585 op/s</sub> | 0.458<br><sub>context: p90 0.571 · p95 0.603 · p99 0.686 · 16972 op/s</sub> | -1.7% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.350 · p95 0.369 · p99 0.433 · 3360 op/s</sub> | 0.281<br><sub>context: p90 0.339 · p95 0.359 · p99 0.394 · 3472 op/s</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.387<br><sub>context: p90 0.482 · p95 0.518 · p99 0.593 · 19783 op/s</sub> | 0.393<br><sub>context: p90 0.495 · p95 0.521 · p99 0.572 · 19518 op/s</sub> | +1.5% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 12.758<br><sub>context: p90 12.849 · p95 12.876 · p99 12.942 · 78 op/s</sub> | 12.895<br><sub>context: p90 13.055 · p95 13.149 · p99 13.264 · 77 op/s</sub> | +1.1% (+0.136) | 10% AND 0.5 ms | 🟢 |
| 8 | 15.931<br><sub>context: p90 20.276 · p95 22.486 · p99 25.360 · 479 op/s</sub> | 16.094<br><sub>context: p90 20.319 · p95 22.099 · p99 25.046 · 471 op/s</sub> | +1.0% (+0.163) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.328<br><sub>context: p90 0.379 · p95 0.413 · p99 0.431 · 2948 op/s</sub> | 0.352<br><sub>context: p90 0.416 · p95 0.435 · p99 0.464 · 2764 op/s</sub> | +7.4% (+0.024) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.447<br><sub>context: p90 0.549 · p95 0.580 · p99 0.675 · 17296 op/s</sub> | 0.440<br><sub>context: p90 0.538 · p95 0.564 · p99 0.619 · 17543 op/s</sub> | -1.6% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.123<br><sub>context: p90 1.516 · p95 1.614 · p99 1.865 · 882 op/s</sub> | 1.119<br><sub>context: p90 1.445 · p95 1.632 · p99 1.764 · 890 op/s</sub> | -0.4% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.402<br><sub>context: p90 1.937 · p95 2.114 · p99 2.555 · 5537 op/s</sub> | 1.409<br><sub>context: p90 1.935 · p95 2.105 · p99 2.530 · 5457 op/s</sub> | +0.5% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.380<br><sub>context: p90 0.460 · p95 0.496 · p99 0.547 · 2586 op/s</sub> | 0.351<br><sub>context: p90 0.451 · p95 0.485 · p99 0.551 · 2727 op/s</sub> | -7.6% (-0.029) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.468<br><sub>context: p90 0.579 · p95 0.611 · p99 0.682 · 16574 op/s</sub> | 0.461<br><sub>context: p90 0.576 · p95 0.610 · p99 0.687 · 16648 op/s</sub> | -1.7% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.317<br><sub>context: p90 0.420 · p95 0.455 · p99 0.517 · 3038 op/s</sub> | 0.311<br><sub>context: p90 0.375 · p95 0.389 · p99 0.473 · 3178 op/s</sub> | -2.1% (-0.007) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.413<br><sub>context: p90 0.529 · p95 0.568 · p99 0.644 · 18464 op/s</sub> | 0.418<br><sub>context: p90 0.535 · p95 0.575 · p99 0.649 · 18374 op/s</sub> | +1.2% (+0.005) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.312<br><sub>context: p90 0.355 · p95 0.383 · p99 0.399 · 3145 op/s</sub> | 0.349<br><sub>context: p90 0.457 · p95 0.488 · p99 0.535 · 2752 op/s</sub> | +11.9% (+0.037) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.549 · p95 0.586 · p99 0.660 · 17455 op/s</sub> | 0.438<br><sub>context: p90 0.558 · p95 0.598 · p99 0.681 · 17481 op/s</sub> | +0.8% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.170<br><sub>context: p90 0.196 · p95 0.208 · p99 0.219 · 5758 op/s</sub> | 0.169<br><sub>context: p90 0.204 · p95 0.218 · p99 0.257 · 5506 op/s</sub> | -0.2% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.298<br><sub>context: p90 0.399 · p95 0.433 · p99 0.498 · 25066 op/s</sub> | 0.306<br><sub>context: p90 0.414 · p95 0.459 · p99 0.546 · 24570 op/s</sub> | +2.7% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.178 · p95 0.186 · p99 0.197 · 6158 op/s</sub> | 0.168<br><sub>context: p90 0.186 · p95 0.192 · p99 0.208 · 5869 op/s</sub> | +7.9% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.248<br><sub>context: p90 0.324 · p95 0.346 · p99 0.393 · 30821 op/s</sub> | 0.244<br><sub>context: p90 0.323 · p95 0.352 · p99 0.403 · 31022 op/s</sub> | -1.7% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.229<br><sub>context: p90 0.269 · p95 0.293 · p99 0.331 · 4197 op/s</sub> | 0.222<br><sub>context: p90 0.260 · p95 0.287 · p99 0.325 · 4327 op/s</sub> | -3.4% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.369<br><sub>context: p90 0.478 · p95 0.513 · p99 0.588 · 20732 op/s</sub> | 0.363<br><sub>context: p90 0.489 · p95 0.523 · p99 0.595 · 20493 op/s</sub> | -1.6% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.299 · p95 0.307 · p99 0.329 · 3987 op/s</sub> | 0.269<br><sub>context: p90 0.358 · p95 0.373 · p99 0.397 · 3560 op/s</sub> | +13.6% (+0.032) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.374<br><sub>context: p90 0.492 · p95 0.529 · p99 0.621 · 20393 op/s</sub> | 0.385<br><sub>context: p90 0.499 · p95 0.541 · p99 0.614 · 19868 op/s</sub> | +2.9% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.214 · p95 0.239 · p99 0.264 · 5186 op/s</sub> | 0.195<br><sub>context: p90 0.224 · p95 0.237 · p99 0.276 · 4892 op/s</sub> | +8.3% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.320<br><sub>context: p90 0.421 · p95 0.458 · p99 0.509 · 23885 op/s</sub> | 0.308<br><sub>context: p90 0.401 · p95 0.431 · p99 0.493 · 24707 op/s</sub> | -3.8% (-0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.909<br><sub>context: p90 1.011 · p95 1.044 · p99 1.124 · 1083 op/s</sub> | 0.921<br><sub>context: p90 1.053 · p95 1.094 · p99 1.151 · 1062 op/s</sub> | +1.3% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.107<br><sub>context: p90 1.272 · p95 1.337 · p99 1.445 · 7077 op/s</sub> | 1.113<br><sub>context: p90 1.305 · p95 1.371 · p99 1.502 · 7016 op/s</sub> | +0.5% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.871<br><sub>context: p90 0.930 · p95 0.959 · p99 1.003 · 1130 op/s</sub> | 0.885<br><sub>context: p90 0.954 · p95 0.970 · p99 0.995 · 1118 op/s</sub> | +1.6% (+0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.013<br><sub>context: p90 1.146 · p95 1.197 · p99 1.309 · 7734 op/s</sub> | 1.016<br><sub>context: p90 1.155 · p95 1.201 · p99 1.294 · 7707 op/s</sub> | +0.3% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.247<br><sub>context: p90 0.326 · p95 0.357 · p99 0.405 · 3739 op/s</sub> | 0.240<br><sub>context: p90 0.300 · p95 0.329 · p99 0.376 · 3983 op/s</sub> | -2.9% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.375<br><sub>context: p90 0.479 · p95 0.510 · p99 0.584 · 20299 op/s</sub> | 0.370<br><sub>context: p90 0.471 · p95 0.501 · p99 0.546 · 20872 op/s</sub> | -1.3% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.359<br><sub>context: p90 0.451 · p95 0.482 · p99 0.547 · 2690 op/s</sub> | 0.389<br><sub>context: p90 0.496 · p95 0.542 · p99 0.636 · 2480 op/s</sub> | +8.4% (+0.030) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.478<br><sub>context: p90 0.598 · p95 0.638 · p99 0.720 · 16292 op/s</sub> | 0.507<br><sub>context: p90 0.660 · p95 0.702 · p99 0.813 · 15204 op/s</sub> | +6.0% (+0.029) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.191<br><sub>context: p90 0.230 · p95 0.242 · p99 0.255 · 4964 op/s</sub> | 0.188<br><sub>context: p90 0.216 · p95 0.226 · p99 0.236 · 5147 op/s</sub> | -1.7% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.332<br><sub>context: p90 0.439 · p95 0.475 · p99 0.552 · 22505 op/s</sub> | 0.322<br><sub>context: p90 0.433 · p95 0.470 · p99 0.521 · 23585 op/s</sub> | -3.0% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.218 · p95 0.230 · p99 0.285 · 5400 op/s</sub> | 0.188<br><sub>context: p90 0.221 · p95 0.233 · p99 0.263 · 5208 op/s</sub> | +6.7% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.304<br><sub>context: p90 0.417 · p95 0.445 · p99 0.510 · 24997 op/s</sub> | 0.302<br><sub>context: p90 0.408 · p95 0.443 · p99 0.516 · 24985 op/s</sub> | -0.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.206 · p95 0.216 · p99 0.231 · 5530 op/s</sub> | 0.178<br><sub>context: p90 0.211 · p95 0.226 · p99 0.248 · 5397 op/s</sub> | +1.3% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.415 · p95 0.454 · p99 0.536 · 24836 op/s</sub> | 0.307<br><sub>context: p90 0.421 · p95 0.451 · p99 0.544 · 24268 op/s</sub> | +0.7% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.361<br><sub>context: p90 0.402 · p95 0.416 · p99 0.448 · 2734 op/s</sub> | 0.363<br><sub>context: p90 0.436 · p95 0.463 · p99 0.488 · 2639 op/s</sub> | +0.5% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.430<br><sub>context: p90 0.505 · p95 0.529 · p99 0.575 · 18352 op/s</sub> | 0.429<br><sub>context: p90 0.504 · p95 0.524 · p99 0.563 · 18306 op/s</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>
