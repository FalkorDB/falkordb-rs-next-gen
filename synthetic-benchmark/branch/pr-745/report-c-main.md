### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 |
| workload_hash | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` |
| samples / warmup | 200 / 50 | 200 / 50 |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**main vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.219<br><sub>context: p90 1.282 · p95 1.297 · p99 1.326 · 813 op/s</sub> | 0.688<br><sub>context: p90 0.736 · p95 0.751 · p99 0.796 · 1431 op/s</sub> | -43.6% (-0.532) | 150% AND 2 ms | 🟢 |
| 8 | 1.626<br><sub>context: p90 2.196 · p95 2.525 · p99 2.957 · 4601 op/s</sub> | 0.836<br><sub>context: p90 0.988 · p95 1.029 · p99 1.116 · 9415 op/s</sub> | -48.6% (-0.790) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.823<br><sub>context: p90 1.902 · p95 1.921 · p99 1.980 · 546 op/s</sub> | 0.766<br><sub>context: p90 0.805 · p95 0.820 · p99 0.841 · 1292 op/s</sub> | -58.0% (-1.057) | 150% AND 2 ms | 🟢 |
| 8 | 2.331<br><sub>context: p90 2.867 · p95 3.148 · p99 3.685 · 3285 op/s</sub> | 0.983<br><sub>context: p90 1.220 · p95 1.261 · p99 1.338 · 7865 op/s</sub> | -57.8% (-1.348) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.830<br><sub>context: p90 1.908 · p95 1.926 · p99 1.996 · 545 op/s</sub> | 1.159<br><sub>context: p90 1.225 · p95 1.252 · p99 1.294 · 851 op/s</sub> | -36.7% (-0.671) | 150% AND 2 ms | 🟢 |
| 8 | 2.382<br><sub>context: p90 3.116 · p95 3.391 · p99 3.836 · 3190 op/s</sub> | 1.675<br><sub>context: p90 2.442 · p95 2.651 · p99 3.097 · 4411 op/s</sub> | -29.7% (-0.707) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.389<br><sub>context: p90 2.484 · p95 2.516 · p99 2.601 · 417 op/s</sub> | 1.190<br><sub>context: p90 1.237 · p95 1.258 · p99 1.301 · 831 op/s</sub> | -50.2% (-1.199) | 150% AND 2 ms | 🟢 |
| 8 | 3.311<br><sub>context: p90 4.484 · p95 4.922 · p99 5.750 · 2286 op/s</sub> | 1.425<br><sub>context: p90 1.722 · p95 1.829 · p99 1.940 · 5400 op/s</sub> | -57.0% (-1.886) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.150<br><sub>context: p90 0.184 · p95 0.189 · p99 0.203 · 6325 op/s</sub> | 0.143<br><sub>context: p90 0.163 · p95 0.170 · p99 0.201 · 6732 op/s</sub> | -5.0% (-0.007) | 150% AND 2 ms | 🟢 |
| 8 | 0.237<br><sub>context: p90 0.300 · p95 0.321 · p99 0.369 · 32096 op/s</sub> | 0.223<br><sub>context: p90 0.296 · p95 0.319 · p99 0.357 · 34369 op/s</sub> | -6.1% (-0.014) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.326<br><sub>context: p90 0.353 · p95 0.361 · p99 0.394 · 3050 op/s</sub> | 0.200<br><sub>context: p90 0.235 · p95 0.245 · p99 0.262 · 4868 op/s</sub> | -38.7% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.418<br><sub>context: p90 0.513 · p95 0.539 · p99 0.592 · 18478 op/s</sub> | 0.325<br><sub>context: p90 0.420 · p95 0.447 · p99 0.503 · 23305 op/s</sub> | -22.2% (-0.093) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.332<br><sub>context: p90 0.367 · p95 0.392 · p99 0.459 · 2991 op/s</sub> | 0.242<br><sub>context: p90 0.308 · p95 0.345 · p99 0.405 · 3901 op/s</sub> | -27.2% (-0.090) | 150% AND 2 ms | 🟢 |
| 8 | 0.448<br><sub>context: p90 0.546 · p95 0.576 · p99 0.641 · 17237 op/s</sub> | 0.356<br><sub>context: p90 0.458 · p95 0.491 · p99 0.543 · 21383 op/s</sub> | -20.5% (-0.092) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.354<br><sub>context: p90 0.454 · p95 0.476 · p99 0.504 · 2676 op/s</sub> | 0.370<br><sub>context: p90 0.487 · p95 0.507 · p99 0.562 · 2577 op/s</sub> | +4.5% (+0.016) | 150% AND 2 ms | 🟢 |
| 8 | 0.489<br><sub>context: p90 0.592 · p95 0.623 · p99 0.691 · 15830 op/s</sub> | 0.456<br><sub>context: p90 0.561 · p95 0.586 · p99 0.689 · 17028 op/s</sub> | -6.8% (-0.033) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.396<br><sub>context: p90 0.468 · p95 0.481 · p99 0.522 · 2478 op/s</sub> | 0.362<br><sub>context: p90 0.413 · p95 0.429 · p99 0.494 · 2693 op/s</sub> | -8.4% (-0.033) | 150% AND 2 ms | 🟢 |
| 8 | 0.532<br><sub>context: p90 0.644 · p95 0.679 · p99 0.760 · 14601 op/s</sub> | 0.481<br><sub>context: p90 0.596 · p95 0.631 · p99 0.697 · 16010 op/s</sub> | -9.5% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.505<br><sub>context: p90 0.611 · p95 0.655 · p99 0.726 · 1914 op/s</sub> | 0.465<br><sub>context: p90 0.538 · p95 0.570 · p99 0.646 · 2124 op/s</sub> | -8.0% (-0.041) | 150% AND 2 ms | 🟢 |
| 8 | 0.695<br><sub>context: p90 0.890 · p95 0.960 · p99 1.100 · 10939 op/s</sub> | 0.602<br><sub>context: p90 0.760 · p95 0.805 · p99 0.906 · 12804 op/s</sub> | -13.4% (-0.093) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.552<br><sub>context: p90 0.695 · p95 0.710 · p99 0.740 · 1783 op/s</sub> | 0.482<br><sub>context: p90 0.568 · p95 0.603 · p99 0.643 · 2036 op/s</sub> | -12.7% (-0.070) | 150% AND 2 ms | 🟢 |
| 8 | 0.732<br><sub>context: p90 0.932 · p95 1.005 · p99 1.140 · 10483 op/s</sub> | 0.632<br><sub>context: p90 0.800 · p95 0.855 · p99 0.961 · 12188 op/s</sub> | -13.6% (-0.099) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.089<br><sub>context: p90 1.460 · p95 1.566 · p99 1.821 · 907 op/s</sub> | 0.924<br><sub>context: p90 1.187 · p95 1.311 · p99 1.630 · 1068 op/s</sub> | -15.1% (-0.165) | 150% AND 2 ms | 🟢 |
| 8 | 1.420<br><sub>context: p90 1.976 · p95 2.116 · p99 2.405 · 5488 op/s</sub> | 1.161<br><sub>context: p90 1.603 · p95 1.742 · p99 2.014 · 6644 op/s</sub> | -18.2% (-0.258) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.044<br><sub>context: p90 1.459 · p95 1.604 · p99 1.820 · 906 op/s</sub> | 0.871<br><sub>context: p90 1.163 · p95 1.302 · p99 1.500 · 1098 op/s</sub> | -16.5% (-0.173) | 150% AND 2 ms | 🟢 |
| 8 | 1.477<br><sub>context: p90 2.041 · p95 2.217 · p99 2.502 · 5208 op/s</sub> | 1.192<br><sub>context: p90 1.645 · p95 1.781 · p99 2.045 · 6399 op/s</sub> | -19.3% (-0.285) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.578<br><sub>context: p90 0.685 · p95 0.731 · p99 0.806 · 1680 op/s</sub> | 0.539<br><sub>context: p90 0.691 · p95 0.721 · p99 0.770 · 1802 op/s</sub> | -6.6% (-0.038) | 150% AND 2 ms | 🟢 |
| 8 | 0.762<br><sub>context: p90 0.908 · p95 0.952 · p99 1.060 · 10241 op/s</sub> | 0.818<br><sub>context: p90 1.103 · p95 1.192 · p99 1.334 · 9539 op/s</sub> | +7.3% (+0.056) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.382<br><sub>context: p90 0.443 · p95 0.472 · p99 0.553 · 2470 op/s</sub> | 0.311<br><sub>context: p90 0.412 · p95 0.444 · p99 0.538 · 3031 op/s</sub> | -18.5% (-0.071) | 150% AND 2 ms | 🟢 |
| 8 | 0.527<br><sub>context: p90 0.642 · p95 0.676 · p99 0.761 · 14667 op/s</sub> | 0.422<br><sub>context: p90 0.531 · p95 0.570 · p99 0.664 · 18233 op/s</sub> | -19.9% (-0.105) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.227<br><sub>context: p90 0.273 · p95 0.283 · p99 0.301 · 4178 op/s</sub> | 0.139<br><sub>context: p90 0.169 · p95 0.176 · p99 0.188 · 6817 op/s</sub> | -38.7% (-0.088) | 150% AND 2 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.422 · p95 0.451 · p99 0.514 · 22905 op/s</sub> | 0.226<br><sub>context: p90 0.298 · p95 0.322 · p99 0.357 · 33687 op/s</sub> | -32.5% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.169<br><sub>context: p90 0.193 · p95 0.203 · p99 0.215 · 5726 op/s</sub> | 0.125<br><sub>context: p90 0.134 · p95 0.136 · p99 0.141 · 8084 op/s</sub> | -26.1% (-0.044) | 150% AND 2 ms | 🟢 |
| 8 | 0.280<br><sub>context: p90 0.353 · p95 0.378 · p99 0.426 · 26379 op/s</sub> | 0.206<br><sub>context: p90 0.281 · p95 0.298 · p99 0.347 · 37477 op/s</sub> | -26.5% (-0.074) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.428<br><sub>context: p90 0.523 · p95 0.531 · p99 0.545 · 2225 op/s</sub> | 0.323<br><sub>context: p90 0.374 · p95 0.398 · p99 0.495 · 3021 op/s</sub> | -24.5% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 0.606<br><sub>context: p90 0.731 · p95 0.786 · p99 0.929 · 12724 op/s</sub> | 0.435<br><sub>context: p90 0.539 · p95 0.570 · p99 0.633 · 17781 op/s</sub> | -28.3% (-0.171) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.927<br><sub>context: p90 1.370 · p95 1.437 · p99 1.725 · 1040 op/s</sub> | 1.087<br><sub>context: p90 1.672 · p95 1.791 · p99 2.045 · 875 op/s</sub> | +17.3% (+0.161) | 150% AND 2 ms | 🟢 |
| 8 | 1.214<br><sub>context: p90 1.793 · p95 1.968 · p99 2.257 · 6269 op/s</sub> | 2.872<br><sub>context: p90 4.588 · p95 5.208 · p99 5.970 · 2650 op/s</sub> | +136.5% (+1.658) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.030<br><sub>context: p90 4.785 · p95 5.394 · p99 6.035 · 316 op/s</sub> | 3.852<br><sub>context: p90 5.793 · p95 6.379 · p99 6.805 · 250 op/s</sub> | +27.1% (+0.822) | 150% AND 2 ms | 🟢 |
| 8 | 3.866<br><sub>context: p90 6.596 · p95 7.444 · p99 8.965 · 1884 op/s</sub> | 13.263<br><sub>context: p90 19.965 · p95 21.690 · p99 24.335 · 588 op/s</sub> | +243.0% (+9.396) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.216<br><sub>context: p90 0.256 · p95 0.269 · p99 0.287 · 4427 op/s</sub> | 0.163<br><sub>context: p90 0.197 · p95 0.205 · p99 0.215 · 5853 op/s</sub> | -24.4% (-0.053) | 150% AND 2 ms | 🟢 |
| 8 | 0.343<br><sub>context: p90 0.427 · p95 0.456 · p99 0.513 · 22286 op/s</sub> | 0.300<br><sub>context: p90 0.402 · p95 0.439 · p99 0.524 · 25248 op/s</sub> | -12.5% (-0.043) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.217<br><sub>context: p90 0.255 · p95 0.262 · p99 0.284 · 4396 op/s</sub> | 0.157<br><sub>context: p90 0.170 · p95 0.180 · p99 0.185 · 6194 op/s</sub> | -27.7% (-0.060) | 150% AND 2 ms | 🟢 |
| 8 | 0.349<br><sub>context: p90 0.433 · p95 0.465 · p99 0.532 · 21892 op/s</sub> | 0.300<br><sub>context: p90 0.411 · p95 0.455 · p99 0.549 · 24953 op/s</sub> | -13.9% (-0.048) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.284<br><sub>context: p90 0.330 · p95 0.337 · p99 0.345 · 3416 op/s</sub> | 0.215<br><sub>context: p90 0.248 · p95 0.253 · p99 0.269 · 4462 op/s</sub> | -24.3% (-0.069) | 150% AND 2 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.585 · p95 0.621 · p99 0.700 · 16452 op/s</sub> | 0.348<br><sub>context: p90 0.446 · p95 0.488 · p99 0.580 · 21820 op/s</sub> | -25.4% (-0.118) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.211 · p95 0.217 · p99 0.237 · 5303 op/s</sub> | 0.136<br><sub>context: p90 0.166 · p95 0.171 · p99 0.208 · 6610 op/s</sub> | -23.2% (-0.041) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.366 · p95 0.388 · p99 0.445 · 26341 op/s</sub> | 0.228<br><sub>context: p90 0.301 · p95 0.326 · p99 0.367 · 33665 op/s</sub> | -21.6% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.328<br><sub>context: p90 0.358 · p95 0.368 · p99 0.400 · 3005 op/s</sub> | 0.316<br><sub>context: p90 0.364 · p95 0.380 · p99 0.414 · 3112 op/s</sub> | -3.7% (-0.012) | 150% AND 2 ms | 🟢 |
| 8 | 0.428<br><sub>context: p90 0.502 · p95 0.527 · p99 0.576 · 18102 op/s</sub> | 0.377<br><sub>context: p90 0.464 · p95 0.492 · p99 0.548 · 20302 op/s</sub> | -11.9% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.248<br><sub>context: p90 0.304 · p95 0.318 · p99 0.336 · 3881 op/s</sub> | 0.233<br><sub>context: p90 0.323 · p95 0.357 · p99 0.375 · 4057 op/s</sub> | -5.8% (-0.014) | 150% AND 2 ms | 🟢 |
| 8 | 0.363<br><sub>context: p90 0.444 · p95 0.469 · p99 0.523 · 20904 op/s</sub> | 0.330<br><sub>context: p90 0.425 · p95 0.457 · p99 0.526 · 23075 op/s</sub> | -8.9% (-0.032) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.389<br><sub>context: p90 0.469 · p95 0.499 · p99 0.517 · 2522 op/s</sub> | 0.303<br><sub>context: p90 0.336 · p95 0.350 · p99 0.378 · 3266 op/s</sub> | -22.2% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.485<br><sub>context: p90 0.583 · p95 0.612 · p99 0.690 · 15904 op/s</sub> | 0.432<br><sub>context: p90 0.536 · p95 0.567 · p99 0.629 · 18011 op/s</sub> | -10.9% (-0.053) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.423<br><sub>context: p90 0.504 · p95 0.536 · p99 0.589 · 2312 op/s</sub> | 0.420<br><sub>context: p90 0.506 · p95 0.534 · p99 0.598 · 2333 op/s</sub> | -0.8% (-0.003) | 150% AND 2 ms | 🟢 |
| 8 | 0.598<br><sub>context: p90 0.758 · p95 0.811 · p99 0.923 · 12917 op/s</sub> | 0.575<br><sub>context: p90 0.741 · p95 0.787 · p99 0.919 · 13206 op/s</sub> | -3.9% (-0.024) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.425<br><sub>context: p90 0.501 · p95 0.528 · p99 0.622 · 2311 op/s</sub> | 0.421<br><sub>context: p90 0.501 · p95 0.526 · p99 0.576 · 2325 op/s</sub> | -1.1% (-0.005) | 150% AND 2 ms | 🟢 |
| 8 | 0.622<br><sub>context: p90 0.780 · p95 0.836 · p99 0.951 · 12352 op/s</sub> | 0.592<br><sub>context: p90 0.745 · p95 0.800 · p99 0.894 · 12983 op/s</sub> | -4.8% (-0.030) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.374<br><sub>context: p90 0.445 · p95 0.487 · p99 0.549 · 2570 op/s</sub> | 0.366<br><sub>context: p90 0.422 · p95 0.442 · p99 0.496 · 2693 op/s</sub> | -2.1% (-0.008) | 150% AND 2 ms | 🟢 |
| 8 | 0.504<br><sub>context: p90 0.614 · p95 0.647 · p99 0.728 · 15346 op/s</sub> | 0.466<br><sub>context: p90 0.585 · p95 0.620 · p99 0.689 · 16585 op/s</sub> | -7.6% (-0.038) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.389<br><sub>context: p90 0.450 · p95 0.468 · p99 0.506 · 2521 op/s</sub> | 0.281<br><sub>context: p90 0.350 · p95 0.369 · p99 0.433 · 3360 op/s</sub> | -27.8% (-0.108) | 150% AND 2 ms | 🟢 |
| 8 | 0.525<br><sub>context: p90 0.629 · p95 0.665 · p99 0.722 · 14830 op/s</sub> | 0.387<br><sub>context: p90 0.482 · p95 0.518 · p99 0.593 · 19783 op/s</sub> | -26.3% (-0.138) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 16.029<br><sub>context: p90 16.263 · p95 16.361 · p99 16.588 · 62 op/s</sub> | 12.758<br><sub>context: p90 12.849 · p95 12.876 · p99 12.942 · 78 op/s</sub> | -20.4% (-3.270) | 150% AND 2 ms | 🟢 |
| 8 | 21.068<br><sub>context: p90 25.603 · p95 27.001 · p99 29.649 · 368 op/s</sub> | 15.931<br><sub>context: p90 20.276 · p95 22.486 · p99 25.360 · 479 op/s</sub> | -24.4% (-5.137) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.431<br><sub>context: p90 0.515 · p95 0.537 · p99 0.605 · 2220 op/s</sub> | 0.328<br><sub>context: p90 0.379 · p95 0.413 · p99 0.431 · 2948 op/s</sub> | -23.8% (-0.103) | 150% AND 2 ms | 🟢 |
| 8 | 0.595<br><sub>context: p90 0.705 · p95 0.753 · p99 0.831 · 13050 op/s</sub> | 0.447<br><sub>context: p90 0.549 · p95 0.580 · p99 0.675 · 17296 op/s</sub> | -24.8% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.238<br><sub>context: p90 1.679 · p95 1.837 · p99 2.124 · 793 op/s</sub> | 1.123<br><sub>context: p90 1.516 · p95 1.614 · p99 1.865 · 882 op/s</sub> | -9.3% (-0.115) | 150% AND 2 ms | 🟢 |
| 8 | 1.604<br><sub>context: p90 2.258 · p95 2.429 · p99 2.938 · 4779 op/s</sub> | 1.402<br><sub>context: p90 1.937 · p95 2.114 · p99 2.555 · 5537 op/s</sub> | -12.6% (-0.202) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.367<br><sub>context: p90 0.441 · p95 0.458 · p99 0.502 · 2639 op/s</sub> | 0.380<br><sub>context: p90 0.460 · p95 0.496 · p99 0.547 · 2586 op/s</sub> | +3.5% (+0.013) | 150% AND 2 ms | 🟢 |
| 8 | 0.512<br><sub>context: p90 0.624 · p95 0.659 · p99 0.754 · 15162 op/s</sub> | 0.468<br><sub>context: p90 0.579 · p95 0.611 · p99 0.682 · 16574 op/s</sub> | -8.5% (-0.044) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.833<br><sub>context: p90 2.294 · p95 2.344 · p99 2.431 · 557 op/s</sub> | 0.317<br><sub>context: p90 0.420 · p95 0.455 · p99 0.517 · 3038 op/s</sub> | -82.7% (-1.515) | 150% AND 2 ms | 🟢 |
| 8 | 2.124<br><sub>context: p90 2.733 · p95 2.878 · p99 3.115 · 3757 op/s</sub> | 0.413<br><sub>context: p90 0.529 · p95 0.568 · p99 0.644 · 18464 op/s</sub> | -80.6% (-1.711) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.844<br><sub>context: p90 2.318 · p95 2.412 · p99 2.534 · 559 op/s</sub> | 0.312<br><sub>context: p90 0.355 · p95 0.383 · p99 0.399 · 3145 op/s</sub> | -83.1% (-1.532) | 150% AND 2 ms | 🟢 |
| 8 | 2.144<br><sub>context: p90 2.785 · p95 2.895 · p99 3.138 · 3762 op/s</sub> | 0.435<br><sub>context: p90 0.549 · p95 0.586 · p99 0.660 · 17455 op/s</sub> | -79.7% (-1.709) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.197<br><sub>context: p90 0.235 · p95 0.243 · p99 0.253 · 5006 op/s</sub> | 0.170<br><sub>context: p90 0.196 · p95 0.208 · p99 0.219 · 5758 op/s</sub> | -13.9% (-0.027) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.374 · p95 0.395 · p99 0.462 · 25993 op/s</sub> | 0.298<br><sub>context: p90 0.399 · p95 0.433 · p99 0.498 · 25066 op/s</sub> | +1.8% (+0.005) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.260<br><sub>context: p90 0.293 · p95 0.309 · p99 0.347 · 3688 op/s</sub> | 0.156<br><sub>context: p90 0.178 · p95 0.186 · p99 0.197 · 6158 op/s</sub> | -40.1% (-0.104) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.380<br><sub>context: p90 0.451 · p95 0.481 · p99 0.528 · 20409 op/s</sub> | 0.248<br><sub>context: p90 0.324 · p95 0.346 · p99 0.393 · 30821 op/s</sub> | -34.7% (-0.132) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.323 · p95 0.336 · p99 0.358 · 3565 op/s</sub> | 0.229<br><sub>context: p90 0.269 · p95 0.293 · p99 0.331 · 4197 op/s</sub> | -15.7% (-0.043) | 150% AND 2 ms | 🟢 |
| 8 | 0.434<br><sub>context: p90 0.537 · p95 0.567 · p99 0.647 · 17630 op/s</sub> | 0.369<br><sub>context: p90 0.478 · p95 0.513 · p99 0.588 · 20732 op/s</sub> | -15.1% (-0.065) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.297<br><sub>context: p90 0.331 · p95 0.342 · p99 0.363 · 3288 op/s</sub> | 0.237<br><sub>context: p90 0.299 · p95 0.307 · p99 0.329 · 3987 op/s</sub> | -20.2% (-0.060) | 150% AND 2 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.533 · p95 0.567 · p99 0.645 · 17668 op/s</sub> | 0.374<br><sub>context: p90 0.492 · p95 0.529 · p99 0.621 · 20393 op/s</sub> | -14.0% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.323 · p95 0.344 · p99 0.375 · 3540 op/s</sub> | 0.180<br><sub>context: p90 0.214 · p95 0.239 · p99 0.264 · 5186 op/s</sub> | -33.9% (-0.092) | 150% AND 2 ms | 🟢 |
| 8 | 0.380<br><sub>context: p90 0.469 · p95 0.493 · p99 0.555 · 20180 op/s</sub> | 0.320<br><sub>context: p90 0.421 · p95 0.458 · p99 0.509 · 23885 op/s</sub> | -15.8% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.212<br><sub>context: p90 1.308 · p95 1.353 · p99 1.413 · 818 op/s</sub> | 0.909<br><sub>context: p90 1.011 · p95 1.044 · p99 1.124 · 1083 op/s</sub> | -25.0% (-0.304) | 150% AND 2 ms | 🟢 |
| 8 | 1.579<br><sub>context: p90 1.873 · p95 1.997 · p99 2.457 · 4905 op/s</sub> | 1.107<br><sub>context: p90 1.272 · p95 1.337 · p99 1.445 · 7077 op/s</sub> | -29.9% (-0.472) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.127<br><sub>context: p90 1.212 · p95 1.229 · p99 1.256 · 882 op/s</sub> | 0.871<br><sub>context: p90 0.930 · p95 0.959 · p99 1.003 · 1130 op/s</sub> | -22.7% (-0.256) | 150% AND 2 ms | 🟢 |
| 8 | 1.427<br><sub>context: p90 1.744 · p95 1.886 · p99 2.289 · 5408 op/s</sub> | 1.013<br><sub>context: p90 1.146 · p95 1.197 · p99 1.309 · 7734 op/s</sub> | -29.0% (-0.414) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.322<br><sub>context: p90 0.405 · p95 0.424 · p99 0.449 · 3010 op/s</sub> | 0.247<br><sub>context: p90 0.326 · p95 0.357 · p99 0.405 · 3739 op/s</sub> | -23.2% (-0.075) | 150% AND 2 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.555 · p95 0.585 · p99 0.645 · 17245 op/s</sub> | 0.375<br><sub>context: p90 0.479 · p95 0.510 · p99 0.584 · 20299 op/s</sub> | -16.0% (-0.071) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.417<br><sub>context: p90 0.509 · p95 0.547 · p99 0.633 · 2337 op/s</sub> | 0.359<br><sub>context: p90 0.451 · p95 0.482 · p99 0.547 · 2690 op/s</sub> | -14.0% (-0.059) | 150% AND 2 ms | 🟢 |
| 8 | 0.565<br><sub>context: p90 0.685 · p95 0.723 · p99 0.827 · 13783 op/s</sub> | 0.478<br><sub>context: p90 0.598 · p95 0.638 · p99 0.720 · 16292 op/s</sub> | -15.4% (-0.087) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.251<br><sub>context: p90 0.294 · p95 0.302 · p99 0.322 · 3827 op/s</sub> | 0.191<br><sub>context: p90 0.230 · p95 0.242 · p99 0.255 · 4964 op/s</sub> | -23.8% (-0.060) | 150% AND 2 ms | 🟢 |
| 8 | 0.382<br><sub>context: p90 0.475 · p95 0.514 · p99 0.575 · 20109 op/s</sub> | 0.332<br><sub>context: p90 0.439 · p95 0.475 · p99 0.552 · 22505 op/s</sub> | -13.1% (-0.050) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.195<br><sub>context: p90 0.234 · p95 0.247 · p99 0.272 · 4719 op/s</sub> | 0.176<br><sub>context: p90 0.218 · p95 0.230 · p99 0.285 · 5400 op/s</sub> | -9.6% (-0.019) | 150% AND 2 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.373 · p95 0.396 · p99 0.438 · 25564 op/s</sub> | 0.304<br><sub>context: p90 0.417 · p95 0.445 · p99 0.510 · 24997 op/s</sub> | +1.5% (+0.004) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.221 · p95 0.236 · p99 0.272 · 5119 op/s</sub> | 0.176<br><sub>context: p90 0.206 · p95 0.216 · p99 0.231 · 5530 op/s</sub> | -4.8% (-0.009) | 150% AND 2 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.371 · p95 0.396 · p99 0.450 · 25703 op/s</sub> | 0.305<br><sub>context: p90 0.415 · p95 0.454 · p99 0.536 · 24836 op/s</sub> | +2.1% (+0.006) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.864<br><sub>context: p90 0.945 · p95 0.962 · p99 1.023 · 1135 op/s</sub> | 0.361<br><sub>context: p90 0.402 · p95 0.416 · p99 0.448 · 2734 op/s</sub> | -58.2% (-0.503) | 150% AND 2 ms | 🟢 |
| 8 | 1.130<br><sub>context: p90 1.552 · p95 1.753 · p99 2.018 · 6512 op/s</sub> | 0.430<br><sub>context: p90 0.505 · p95 0.529 · p99 0.575 · 18352 op/s</sub> | -62.0% (-0.701) | 150% AND 2 ms | 🟢 |

</details>
