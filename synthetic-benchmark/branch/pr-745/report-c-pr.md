### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:0b7e1e313cced37a47421e2414707354e36ac241692e24dc766f2ddaa75d5aee |
| workload_hash | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` |
| samples / warmup | 200 / 50 | 200 / 50 |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:0b7e1e313cced37a47421e2414707354e36ac241692e24dc766f2ddaa75d5aee

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.219<br><sub>context: p90 1.282 · p95 1.297 · p99 1.326 · 813 op/s</sub> | 0.722<br><sub>context: p90 0.831 · p95 0.847 · p99 0.928 · 1354 op/s</sub> | -40.8% (-0.497) | 150% AND 2 ms | 🟢 |
| 8 | 1.626<br><sub>context: p90 2.196 · p95 2.525 · p99 2.957 · 4601 op/s</sub> | 0.832<br><sub>context: p90 0.989 · p95 1.036 · p99 1.114 · 9360 op/s</sub> | -48.8% (-0.794) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.823<br><sub>context: p90 1.902 · p95 1.921 · p99 1.980 · 546 op/s</sub> | 0.792<br><sub>context: p90 0.868 · p95 0.893 · p99 0.939 · 1238 op/s</sub> | -56.6% (-1.031) | 150% AND 2 ms | 🟢 |
| 8 | 2.331<br><sub>context: p90 2.867 · p95 3.148 · p99 3.685 · 3285 op/s</sub> | 0.998<br><sub>context: p90 1.234 · p95 1.280 · p99 1.357 · 7814 op/s</sub> | -57.2% (-1.333) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.830<br><sub>context: p90 1.908 · p95 1.926 · p99 1.996 · 545 op/s</sub> | 1.157<br><sub>context: p90 1.224 · p95 1.240 · p99 1.291 · 852 op/s</sub> | -36.8% (-0.674) | 150% AND 2 ms | 🟢 |
| 8 | 2.382<br><sub>context: p90 3.116 · p95 3.391 · p99 3.836 · 3190 op/s</sub> | 1.564<br><sub>context: p90 2.204 · p95 2.429 · p99 2.782 · 4743 op/s</sub> | -34.3% (-0.818) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.389<br><sub>context: p90 2.484 · p95 2.516 · p99 2.601 · 417 op/s</sub> | 1.216<br><sub>context: p90 1.297 · p95 1.315 · p99 1.339 · 813 op/s</sub> | -49.1% (-1.173) | 150% AND 2 ms | 🟢 |
| 8 | 3.311<br><sub>context: p90 4.484 · p95 4.922 · p99 5.750 · 2286 op/s</sub> | 1.429<br><sub>context: p90 1.735 · p95 1.823 · p99 1.955 · 5371 op/s</sub> | -56.8% (-1.882) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.150<br><sub>context: p90 0.184 · p95 0.189 · p99 0.203 · 6325 op/s</sub> | 0.130<br><sub>context: p90 0.150 · p95 0.155 · p99 0.161 · 7481 op/s</sub> | -13.6% (-0.020) | 150% AND 2 ms | 🟢 |
| 8 | 0.237<br><sub>context: p90 0.300 · p95 0.321 · p99 0.369 · 32096 op/s</sub> | 0.221<br><sub>context: p90 0.291 · p95 0.311 · p99 0.351 · 34965 op/s</sub> | -7.1% (-0.017) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.326<br><sub>context: p90 0.353 · p95 0.361 · p99 0.394 · 3050 op/s</sub> | 0.200<br><sub>context: p90 0.238 · p95 0.246 · p99 0.261 · 4875 op/s</sub> | -38.5% (-0.125) | 150% AND 2 ms | 🟢 |
| 8 | 0.418<br><sub>context: p90 0.513 · p95 0.539 · p99 0.592 · 18478 op/s</sub> | 0.330<br><sub>context: p90 0.436 · p95 0.470 · p99 0.530 · 22823 op/s</sub> | -21.1% (-0.088) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.332<br><sub>context: p90 0.367 · p95 0.392 · p99 0.459 · 2991 op/s</sub> | 0.251<br><sub>context: p90 0.298 · p95 0.314 · p99 0.365 · 3865 op/s</sub> | -24.3% (-0.081) | 150% AND 2 ms | 🟢 |
| 8 | 0.448<br><sub>context: p90 0.546 · p95 0.576 · p99 0.641 · 17237 op/s</sub> | 0.348<br><sub>context: p90 0.442 · p95 0.472 · p99 0.548 · 21962 op/s</sub> | -22.2% (-0.099) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.354<br><sub>context: p90 0.454 · p95 0.476 · p99 0.504 · 2676 op/s</sub> | 0.331<br><sub>context: p90 0.399 · p95 0.433 · p99 0.465 · 2951 op/s</sub> | -6.6% (-0.023) | 150% AND 2 ms | 🟢 |
| 8 | 0.489<br><sub>context: p90 0.592 · p95 0.623 · p99 0.691 · 15830 op/s</sub> | 0.459<br><sub>context: p90 0.568 · p95 0.610 · p99 0.695 · 16760 op/s</sub> | -6.2% (-0.030) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.396<br><sub>context: p90 0.468 · p95 0.481 · p99 0.522 · 2478 op/s</sub> | 0.349<br><sub>context: p90 0.420 · p95 0.437 · p99 0.474 · 2776 op/s</sub> | -11.9% (-0.047) | 150% AND 2 ms | 🟢 |
| 8 | 0.532<br><sub>context: p90 0.644 · p95 0.679 · p99 0.760 · 14601 op/s</sub> | 0.464<br><sub>context: p90 0.570 · p95 0.598 · p99 0.655 · 16611 op/s</sub> | -12.8% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.505<br><sub>context: p90 0.611 · p95 0.655 · p99 0.726 · 1914 op/s</sub> | 0.466<br><sub>context: p90 0.556 · p95 0.576 · p99 0.639 · 2127 op/s</sub> | -7.8% (-0.039) | 150% AND 2 ms | 🟢 |
| 8 | 0.695<br><sub>context: p90 0.890 · p95 0.960 · p99 1.100 · 10939 op/s</sub> | 0.600<br><sub>context: p90 0.759 · p95 0.816 · p99 0.900 · 12941 op/s</sub> | -13.6% (-0.095) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.552<br><sub>context: p90 0.695 · p95 0.710 · p99 0.740 · 1783 op/s</sub> | 0.475<br><sub>context: p90 0.577 · p95 0.620 · p99 0.672 · 2065 op/s</sub> | -13.8% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.732<br><sub>context: p90 0.932 · p95 1.005 · p99 1.140 · 10483 op/s</sub> | 0.615<br><sub>context: p90 0.774 · p95 0.818 · p99 0.916 · 12633 op/s</sub> | -15.9% (-0.117) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.089<br><sub>context: p90 1.460 · p95 1.566 · p99 1.821 · 907 op/s</sub> | 0.917<br><sub>context: p90 1.234 · p95 1.335 · p99 1.607 · 1060 op/s</sub> | -15.8% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 1.420<br><sub>context: p90 1.976 · p95 2.116 · p99 2.405 · 5488 op/s</sub> | 1.127<br><sub>context: p90 1.569 · p95 1.692 · p99 2.020 · 6791 op/s</sub> | -20.6% (-0.293) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.044<br><sub>context: p90 1.459 · p95 1.604 · p99 1.820 · 906 op/s</sub> | 0.874<br><sub>context: p90 1.136 · p95 1.335 · p99 1.527 · 1092 op/s</sub> | -16.3% (-0.170) | 150% AND 2 ms | 🟢 |
| 8 | 1.477<br><sub>context: p90 2.041 · p95 2.217 · p99 2.502 · 5208 op/s</sub> | 1.183<br><sub>context: p90 1.632 · p95 1.789 · p99 2.070 · 6463 op/s</sub> | -19.9% (-0.294) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.578<br><sub>context: p90 0.685 · p95 0.731 · p99 0.806 · 1680 op/s</sub> | 0.605<br><sub>context: p90 0.756 · p95 0.791 · p99 0.871 · 1643 op/s</sub> | +4.8% (+0.028) | 150% AND 2 ms | 🟢 |
| 8 | 0.762<br><sub>context: p90 0.908 · p95 0.952 · p99 1.060 · 10241 op/s</sub> | 0.842<br><sub>context: p90 1.148 · p95 1.232 · p99 1.379 · 9224 op/s</sub> | +10.5% (+0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.382<br><sub>context: p90 0.443 · p95 0.472 · p99 0.553 · 2470 op/s</sub> | 0.300<br><sub>context: p90 0.366 · p95 0.380 · p99 0.454 · 3214 op/s</sub> | -21.5% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.527<br><sub>context: p90 0.642 · p95 0.676 · p99 0.761 · 14667 op/s</sub> | 0.415<br><sub>context: p90 0.516 · p95 0.547 · p99 0.617 · 18565 op/s</sub> | -21.3% (-0.112) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.227<br><sub>context: p90 0.273 · p95 0.283 · p99 0.301 · 4178 op/s</sub> | 0.152<br><sub>context: p90 0.178 · p95 0.186 · p99 0.212 · 6241 op/s</sub> | -33.4% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.422 · p95 0.451 · p99 0.514 · 22905 op/s</sub> | 0.235<br><sub>context: p90 0.305 · p95 0.322 · p99 0.367 · 32365 op/s</sub> | -29.8% (-0.100) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.169<br><sub>context: p90 0.193 · p95 0.203 · p99 0.215 · 5726 op/s</sub> | 0.121<br><sub>context: p90 0.142 · p95 0.146 · p99 0.161 · 7864 op/s</sub> | -28.3% (-0.048) | 150% AND 2 ms | 🟢 |
| 8 | 0.280<br><sub>context: p90 0.353 · p95 0.378 · p99 0.426 · 26379 op/s</sub> | 0.205<br><sub>context: p90 0.276 · p95 0.299 · p99 0.356 · 37680 op/s</sub> | -27.0% (-0.076) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.428<br><sub>context: p90 0.523 · p95 0.531 · p99 0.545 · 2225 op/s</sub> | 0.346<br><sub>context: p90 0.390 · p95 0.407 · p99 0.480 · 2881 op/s</sub> | -19.2% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.606<br><sub>context: p90 0.731 · p95 0.786 · p99 0.929 · 12724 op/s</sub> | 0.439<br><sub>context: p90 0.544 · p95 0.579 · p99 0.628 · 17631 op/s</sub> | -27.5% (-0.166) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.927<br><sub>context: p90 1.370 · p95 1.437 · p99 1.725 · 1040 op/s</sub> | 1.154<br><sub>context: p90 1.689 · p95 1.889 · p99 2.121 · 840 op/s</sub> | +24.5% (+0.227) | 150% AND 2 ms | 🟢 |
| 8 | 1.214<br><sub>context: p90 1.793 · p95 1.968 · p99 2.257 · 6269 op/s</sub> | 3.084<br><sub>context: p90 5.070 · p95 5.735 · p99 6.943 · 2430 op/s</sub> | +154.0% (+1.870) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.030<br><sub>context: p90 4.785 · p95 5.394 · p99 6.035 · 316 op/s</sub> | 3.847<br><sub>context: p90 5.828 · p95 6.354 · p99 6.841 · 252 op/s</sub> | +26.9% (+0.816) | 150% AND 2 ms | 🟢 |
| 8 | 3.866<br><sub>context: p90 6.596 · p95 7.444 · p99 8.965 · 1884 op/s</sub> | 12.761<br><sub>context: p90 19.368 · p95 21.351 · p99 23.493 · 612 op/s</sub> | +230.1% (+8.895) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.216<br><sub>context: p90 0.256 · p95 0.269 · p99 0.287 · 4427 op/s</sub> | 0.167<br><sub>context: p90 0.192 · p95 0.203 · p99 0.212 · 5712 op/s</sub> | -22.8% (-0.049) | 150% AND 2 ms | 🟢 |
| 8 | 0.343<br><sub>context: p90 0.427 · p95 0.456 · p99 0.513 · 22286 op/s</sub> | 0.304<br><sub>context: p90 0.416 · p95 0.451 · p99 0.524 · 25068 op/s</sub> | -11.5% (-0.039) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.217<br><sub>context: p90 0.255 · p95 0.262 · p99 0.284 · 4396 op/s</sub> | 0.160<br><sub>context: p90 0.193 · p95 0.198 · p99 0.208 · 5797 op/s</sub> | -26.5% (-0.057) | 150% AND 2 ms | 🟢 |
| 8 | 0.349<br><sub>context: p90 0.433 · p95 0.465 · p99 0.532 · 21892 op/s</sub> | 0.307<br><sub>context: p90 0.418 · p95 0.450 · p99 0.537 · 24553 op/s</sub> | -11.9% (-0.042) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.284<br><sub>context: p90 0.330 · p95 0.337 · p99 0.345 · 3416 op/s</sub> | 0.243<br><sub>context: p90 0.287 · p95 0.302 · p99 0.348 · 3918 op/s</sub> | -14.4% (-0.041) | 150% AND 2 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.585 · p95 0.621 · p99 0.700 · 16452 op/s</sub> | 0.346<br><sub>context: p90 0.444 · p95 0.479 · p99 0.559 · 21812 op/s</sub> | -25.7% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.211 · p95 0.217 · p99 0.237 · 5303 op/s</sub> | 0.151<br><sub>context: p90 0.174 · p95 0.180 · p99 0.198 · 6221 op/s</sub> | -14.9% (-0.027) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.366 · p95 0.388 · p99 0.445 · 26341 op/s</sub> | 0.224<br><sub>context: p90 0.298 · p95 0.321 · p99 0.366 · 33760 op/s</sub> | -22.7% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.328<br><sub>context: p90 0.358 · p95 0.368 · p99 0.400 · 3005 op/s</sub> | 0.317<br><sub>context: p90 0.377 · p95 0.410 · p99 0.473 · 3085 op/s</sub> | -3.2% (-0.011) | 150% AND 2 ms | 🟢 |
| 8 | 0.428<br><sub>context: p90 0.502 · p95 0.527 · p99 0.576 · 18102 op/s</sub> | 0.387<br><sub>context: p90 0.480 · p95 0.512 · p99 0.566 · 19691 op/s</sub> | -9.6% (-0.041) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.248<br><sub>context: p90 0.304 · p95 0.318 · p99 0.336 · 3881 op/s</sub> | 0.227<br><sub>context: p90 0.286 · p95 0.306 · p99 0.350 · 4171 op/s</sub> | -8.5% (-0.021) | 150% AND 2 ms | 🟢 |
| 8 | 0.363<br><sub>context: p90 0.444 · p95 0.469 · p99 0.523 · 20904 op/s</sub> | 0.337<br><sub>context: p90 0.432 · p95 0.463 · p99 0.519 · 22720 op/s</sub> | -7.0% (-0.026) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.389<br><sub>context: p90 0.469 · p95 0.499 · p99 0.517 · 2522 op/s</sub> | 0.342<br><sub>context: p90 0.446 · p95 0.474 · p99 0.534 · 2836 op/s</sub> | -12.1% (-0.047) | 150% AND 2 ms | 🟢 |
| 8 | 0.485<br><sub>context: p90 0.583 · p95 0.612 · p99 0.690 · 15904 op/s</sub> | 0.449<br><sub>context: p90 0.561 · p95 0.600 · p99 0.687 · 17246 op/s</sub> | -7.4% (-0.036) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.423<br><sub>context: p90 0.504 · p95 0.536 · p99 0.589 · 2312 op/s</sub> | 0.436<br><sub>context: p90 0.523 · p95 0.556 · p99 0.603 · 2267 op/s</sub> | +3.0% (+0.013) | 150% AND 2 ms | 🟢 |
| 8 | 0.598<br><sub>context: p90 0.758 · p95 0.811 · p99 0.923 · 12917 op/s</sub> | 0.588<br><sub>context: p90 0.751 · p95 0.817 · p99 0.929 · 13154 op/s</sub> | -1.7% (-0.010) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.425<br><sub>context: p90 0.501 · p95 0.528 · p99 0.622 · 2311 op/s</sub> | 0.425<br><sub>context: p90 0.515 · p95 0.543 · p99 0.634 · 2298 op/s</sub> | +0.0% (+0.000) | 150% AND 2 ms | 🟢 |
| 8 | 0.622<br><sub>context: p90 0.780 · p95 0.836 · p99 0.951 · 12352 op/s</sub> | 0.593<br><sub>context: p90 0.760 · p95 0.811 · p99 0.906 · 13082 op/s</sub> | -4.6% (-0.029) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.374<br><sub>context: p90 0.445 · p95 0.487 · p99 0.549 · 2570 op/s</sub> | 0.351<br><sub>context: p90 0.435 · p95 0.468 · p99 0.503 · 2739 op/s</sub> | -6.4% (-0.024) | 150% AND 2 ms | 🟢 |
| 8 | 0.504<br><sub>context: p90 0.614 · p95 0.647 · p99 0.728 · 15346 op/s</sub> | 0.458<br><sub>context: p90 0.571 · p95 0.603 · p99 0.686 · 16972 op/s</sub> | -9.1% (-0.046) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.389<br><sub>context: p90 0.450 · p95 0.468 · p99 0.506 · 2521 op/s</sub> | 0.281<br><sub>context: p90 0.339 · p95 0.359 · p99 0.394 · 3472 op/s</sub> | -27.8% (-0.108) | 150% AND 2 ms | 🟢 |
| 8 | 0.525<br><sub>context: p90 0.629 · p95 0.665 · p99 0.722 · 14830 op/s</sub> | 0.393<br><sub>context: p90 0.495 · p95 0.521 · p99 0.572 · 19518 op/s</sub> | -25.2% (-0.132) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 16.029<br><sub>context: p90 16.263 · p95 16.361 · p99 16.588 · 62 op/s</sub> | 12.895<br><sub>context: p90 13.055 · p95 13.149 · p99 13.264 · 77 op/s</sub> | -19.6% (-3.134) | 150% AND 2 ms | 🟢 |
| 8 | 21.068<br><sub>context: p90 25.603 · p95 27.001 · p99 29.649 · 368 op/s</sub> | 16.094<br><sub>context: p90 20.319 · p95 22.099 · p99 25.046 · 471 op/s</sub> | -23.6% (-4.974) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.431<br><sub>context: p90 0.515 · p95 0.537 · p99 0.605 · 2220 op/s</sub> | 0.352<br><sub>context: p90 0.416 · p95 0.435 · p99 0.464 · 2764 op/s</sub> | -18.2% (-0.079) | 150% AND 2 ms | 🟢 |
| 8 | 0.595<br><sub>context: p90 0.705 · p95 0.753 · p99 0.831 · 13050 op/s</sub> | 0.440<br><sub>context: p90 0.538 · p95 0.564 · p99 0.619 · 17543 op/s</sub> | -26.0% (-0.155) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.238<br><sub>context: p90 1.679 · p95 1.837 · p99 2.124 · 793 op/s</sub> | 1.119<br><sub>context: p90 1.445 · p95 1.632 · p99 1.764 · 890 op/s</sub> | -9.7% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 1.604<br><sub>context: p90 2.258 · p95 2.429 · p99 2.938 · 4779 op/s</sub> | 1.409<br><sub>context: p90 1.935 · p95 2.105 · p99 2.530 · 5457 op/s</sub> | -12.2% (-0.195) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.367<br><sub>context: p90 0.441 · p95 0.458 · p99 0.502 · 2639 op/s</sub> | 0.351<br><sub>context: p90 0.451 · p95 0.485 · p99 0.551 · 2727 op/s</sub> | -4.3% (-0.016) | 150% AND 2 ms | 🟢 |
| 8 | 0.512<br><sub>context: p90 0.624 · p95 0.659 · p99 0.754 · 15162 op/s</sub> | 0.461<br><sub>context: p90 0.576 · p95 0.610 · p99 0.687 · 16648 op/s</sub> | -10.0% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.833<br><sub>context: p90 2.294 · p95 2.344 · p99 2.431 · 557 op/s</sub> | 0.311<br><sub>context: p90 0.375 · p95 0.389 · p99 0.473 · 3178 op/s</sub> | -83.1% (-1.522) | 150% AND 2 ms | 🟢 |
| 8 | 2.124<br><sub>context: p90 2.733 · p95 2.878 · p99 3.115 · 3757 op/s</sub> | 0.418<br><sub>context: p90 0.535 · p95 0.575 · p99 0.649 · 18374 op/s</sub> | -80.3% (-1.706) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.844<br><sub>context: p90 2.318 · p95 2.412 · p99 2.534 · 559 op/s</sub> | 0.349<br><sub>context: p90 0.457 · p95 0.488 · p99 0.535 · 2752 op/s</sub> | -81.1% (-1.495) | 150% AND 2 ms | 🟢 |
| 8 | 2.144<br><sub>context: p90 2.785 · p95 2.895 · p99 3.138 · 3762 op/s</sub> | 0.438<br><sub>context: p90 0.558 · p95 0.598 · p99 0.681 · 17481 op/s</sub> | -79.6% (-1.705) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.197<br><sub>context: p90 0.235 · p95 0.243 · p99 0.253 · 5006 op/s</sub> | 0.169<br><sub>context: p90 0.204 · p95 0.218 · p99 0.257 · 5506 op/s</sub> | -14.1% (-0.028) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.374 · p95 0.395 · p99 0.462 · 25993 op/s</sub> | 0.306<br><sub>context: p90 0.414 · p95 0.459 · p99 0.546 · 24570 op/s</sub> | +4.5% (+0.013) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.260<br><sub>context: p90 0.293 · p95 0.309 · p99 0.347 · 3688 op/s</sub> | 0.168<br><sub>context: p90 0.186 · p95 0.192 · p99 0.208 · 5869 op/s</sub> | -35.4% (-0.092) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.380<br><sub>context: p90 0.451 · p95 0.481 · p99 0.528 · 20409 op/s</sub> | 0.244<br><sub>context: p90 0.323 · p95 0.352 · p99 0.403 · 31022 op/s</sub> | -35.8% (-0.136) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.323 · p95 0.336 · p99 0.358 · 3565 op/s</sub> | 0.222<br><sub>context: p90 0.260 · p95 0.287 · p99 0.325 · 4327 op/s</sub> | -18.5% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.434<br><sub>context: p90 0.537 · p95 0.567 · p99 0.647 · 17630 op/s</sub> | 0.363<br><sub>context: p90 0.489 · p95 0.523 · p99 0.595 · 20493 op/s</sub> | -16.4% (-0.071) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.297<br><sub>context: p90 0.331 · p95 0.342 · p99 0.363 · 3288 op/s</sub> | 0.269<br><sub>context: p90 0.358 · p95 0.373 · p99 0.397 · 3560 op/s</sub> | -9.4% (-0.028) | 150% AND 2 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.533 · p95 0.567 · p99 0.645 · 17668 op/s</sub> | 0.385<br><sub>context: p90 0.499 · p95 0.541 · p99 0.614 · 19868 op/s</sub> | -11.5% (-0.050) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.323 · p95 0.344 · p99 0.375 · 3540 op/s</sub> | 0.195<br><sub>context: p90 0.224 · p95 0.237 · p99 0.276 · 4892 op/s</sub> | -28.4% (-0.077) | 150% AND 2 ms | 🟢 |
| 8 | 0.380<br><sub>context: p90 0.469 · p95 0.493 · p99 0.555 · 20180 op/s</sub> | 0.308<br><sub>context: p90 0.401 · p95 0.431 · p99 0.493 · 24707 op/s</sub> | -19.1% (-0.072) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.212<br><sub>context: p90 1.308 · p95 1.353 · p99 1.413 · 818 op/s</sub> | 0.921<br><sub>context: p90 1.053 · p95 1.094 · p99 1.151 · 1062 op/s</sub> | -24.1% (-0.292) | 150% AND 2 ms | 🟢 |
| 8 | 1.579<br><sub>context: p90 1.873 · p95 1.997 · p99 2.457 · 4905 op/s</sub> | 1.113<br><sub>context: p90 1.305 · p95 1.371 · p99 1.502 · 7016 op/s</sub> | -29.5% (-0.466) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.127<br><sub>context: p90 1.212 · p95 1.229 · p99 1.256 · 882 op/s</sub> | 0.885<br><sub>context: p90 0.954 · p95 0.970 · p99 0.995 · 1118 op/s</sub> | -21.5% (-0.242) | 150% AND 2 ms | 🟢 |
| 8 | 1.427<br><sub>context: p90 1.744 · p95 1.886 · p99 2.289 · 5408 op/s</sub> | 1.016<br><sub>context: p90 1.155 · p95 1.201 · p99 1.294 · 7707 op/s</sub> | -28.8% (-0.411) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.322<br><sub>context: p90 0.405 · p95 0.424 · p99 0.449 · 3010 op/s</sub> | 0.240<br><sub>context: p90 0.300 · p95 0.329 · p99 0.376 · 3983 op/s</sub> | -25.4% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.555 · p95 0.585 · p99 0.645 · 17245 op/s</sub> | 0.370<br><sub>context: p90 0.471 · p95 0.501 · p99 0.546 · 20872 op/s</sub> | -17.1% (-0.076) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.417<br><sub>context: p90 0.509 · p95 0.547 · p99 0.633 · 2337 op/s</sub> | 0.389<br><sub>context: p90 0.496 · p95 0.542 · p99 0.636 · 2480 op/s</sub> | -6.8% (-0.029) | 150% AND 2 ms | 🟢 |
| 8 | 0.565<br><sub>context: p90 0.685 · p95 0.723 · p99 0.827 · 13783 op/s</sub> | 0.507<br><sub>context: p90 0.660 · p95 0.702 · p99 0.813 · 15204 op/s</sub> | -10.3% (-0.058) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.251<br><sub>context: p90 0.294 · p95 0.302 · p99 0.322 · 3827 op/s</sub> | 0.188<br><sub>context: p90 0.216 · p95 0.226 · p99 0.236 · 5147 op/s</sub> | -25.1% (-0.063) | 150% AND 2 ms | 🟢 |
| 8 | 0.382<br><sub>context: p90 0.475 · p95 0.514 · p99 0.575 · 20109 op/s</sub> | 0.322<br><sub>context: p90 0.433 · p95 0.470 · p99 0.521 · 23585 op/s</sub> | -15.7% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.195<br><sub>context: p90 0.234 · p95 0.247 · p99 0.272 · 4719 op/s</sub> | 0.188<br><sub>context: p90 0.221 · p95 0.233 · p99 0.263 · 5208 op/s</sub> | -3.6% (-0.007) | 150% AND 2 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.373 · p95 0.396 · p99 0.438 · 25564 op/s</sub> | 0.302<br><sub>context: p90 0.408 · p95 0.443 · p99 0.516 · 24985 op/s</sub> | +0.8% (+0.002) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.221 · p95 0.236 · p99 0.272 · 5119 op/s</sub> | 0.178<br><sub>context: p90 0.211 · p95 0.226 · p99 0.248 · 5397 op/s</sub> | -3.6% (-0.007) | 150% AND 2 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.371 · p95 0.396 · p99 0.450 · 25703 op/s</sub> | 0.307<br><sub>context: p90 0.421 · p95 0.451 · p99 0.544 · 24268 op/s</sub> | +2.8% (+0.008) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.864<br><sub>context: p90 0.945 · p95 0.962 · p99 1.023 · 1135 op/s</sub> | 0.363<br><sub>context: p90 0.436 · p95 0.463 · p99 0.488 · 2639 op/s</sub> | -58.0% (-0.501) | 150% AND 2 ms | 🟢 |
| 8 | 1.130<br><sub>context: p90 1.552 · p95 1.753 · p99 2.018 · 6512 op/s</sub> | 0.429<br><sub>context: p90 0.504 · p95 0.524 · p99 0.563 · 18306 op/s</sub> | -62.1% (-0.702) | 150% AND 2 ms | 🟢 |

</details>
