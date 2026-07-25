### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 | ghcr.io/falkordb/falkordb-server@sha256:1328197de8d3dfbb87a1597dc2909530990d7a80ef4a845562899a1b5dc02497 |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 → ghcr.io/falkordb/falkordb-server@sha256:1328197de8d3dfbb87a1597dc2909530990d7a80ef4a845562899a1b5dc02497

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.997<br><sub>context: p90 1.089 · p95 1.135 · p99 1.171 · 993 op/s</sub> | 0.985<br><sub>context: p90 1.066 · p95 1.089 · p99 1.121 · 1004 op/s</sub> | -1.1% (-0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.967<br><sub>context: p90 1.125 · p95 1.172 · p99 1.289 · 8125 op/s</sub> | 0.963<br><sub>context: p90 1.102 · p95 1.147 · p99 1.231 · 8167 op/s</sub> | -0.4% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.042<br><sub>context: p90 1.134 · p95 1.167 · p99 1.213 · 946 op/s</sub> | 1.017<br><sub>context: p90 1.134 · p95 1.158 · p99 1.200 · 966 op/s</sub> | -2.4% (-0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.120<br><sub>context: p90 1.344 · p95 1.383 · p99 1.474 · 6955 op/s</sub> | 1.125<br><sub>context: p90 1.362 · p95 1.413 · p99 1.494 · 6910 op/s</sub> | +0.4% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.502<br><sub>context: p90 1.612 · p95 1.635 · p99 1.677 · 658 op/s</sub> | 1.516<br><sub>context: p90 1.605 · p95 1.654 · p99 1.696 · 653 op/s</sub> | +0.9% (+0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.812<br><sub>context: p90 2.343 · p95 2.521 · p99 2.830 · 4230 op/s</sub> | 1.821<br><sub>context: p90 2.323 · p95 2.515 · p99 2.861 · 4251 op/s</sub> | +0.5% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.555<br><sub>context: p90 1.659 · p95 1.700 · p99 1.786 · 637 op/s</sub> | 1.571<br><sub>context: p90 1.669 · p95 1.695 · p99 1.772 · 631 op/s</sub> | +1.0% (+0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.720<br><sub>context: p90 2.053 · p95 2.121 · p99 2.303 · 4512 op/s</sub> | 1.710<br><sub>context: p90 2.033 · p95 2.099 · p99 2.220 · 4526 op/s</sub> | -0.6% (-0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.177<br><sub>context: p90 0.246 · p95 0.281 · p99 0.328 · 5302 op/s</sub> | 0.169<br><sub>context: p90 0.238 · p95 0.271 · p99 0.307 · 5507 op/s</sub> | -4.7% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.224<br><sub>context: p90 0.299 · p95 0.322 · p99 0.353 · 34420 op/s</sub> | 0.228<br><sub>context: p90 0.319 · p95 0.350 · p99 0.449 · 27175 op/s</sub> | +2.0% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.377<br><sub>context: p90 0.482 · p95 0.504 · p99 0.550 · 2519 op/s</sub> | 0.398<br><sub>context: p90 0.494 · p95 0.532 · p99 0.571 · 2478 op/s</sub> | +5.7% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.339<br><sub>context: p90 0.440 · p95 0.463 · p99 0.536 · 22346 op/s</sub> | 0.351<br><sub>context: p90 0.468 · p95 0.507 · p99 0.584 · 21317 op/s</sub> | +3.7% (+0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.395<br><sub>context: p90 0.485 · p95 0.520 · p99 0.549 · 2475 op/s</sub> | 0.466<br><sub>context: p90 0.556 · p95 0.583 · p99 0.625 · 2127 op/s</sub> | +17.9% (+0.071) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.362<br><sub>context: p90 0.459 · p95 0.492 · p99 0.556 · 21013 op/s</sub> | 0.357<br><sub>context: p90 0.454 · p95 0.489 · p99 0.565 · 21359 op/s</sub> | -1.5% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.561<br><sub>context: p90 0.665 · p95 0.705 · p99 0.788 · 1744 op/s</sub> | 0.542<br><sub>context: p90 0.657 · p95 0.688 · p99 0.758 · 1822 op/s</sub> | -3.4% (-0.019) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.456<br><sub>context: p90 0.573 · p95 0.611 · p99 0.684 · 16827 op/s</sub> | 0.472<br><sub>context: p90 0.579 · p95 0.612 · p99 0.688 · 16217 op/s</sub> | +3.4% (+0.015) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.545<br><sub>context: p90 0.644 · p95 0.683 · p99 0.721 · 1807 op/s</sub> | 0.609<br><sub>context: p90 0.733 · p95 0.764 · p99 0.811 · 1632 op/s</sub> | +11.8% (+0.064) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.483<br><sub>context: p90 0.599 · p95 0.634 · p99 0.712 · 15887 op/s</sub> | 0.501<br><sub>context: p90 0.614 · p95 0.651 · p99 0.714 · 15395 op/s</sub> | +3.7% (+0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.706<br><sub>context: p90 0.839 · p95 0.891 · p99 0.991 · 1401 op/s</sub> | 0.680<br><sub>context: p90 0.805 · p95 0.832 · p99 0.905 · 1461 op/s</sub> | -3.7% (-0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.659<br><sub>context: p90 0.837 · p95 0.905 · p99 1.056 · 11545 op/s</sub> | 0.663<br><sub>context: p90 0.844 · p95 0.903 · p99 1.007 · 11636 op/s</sub> | +0.6% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.671<br><sub>context: p90 0.830 · p95 0.858 · p99 0.986 · 1444 op/s</sub> | 0.680<br><sub>context: p90 0.810 · p95 0.835 · p99 0.923 · 1460 op/s</sub> | +1.2% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.687<br><sub>context: p90 0.877 · p95 0.944 · p99 1.082 · 11236 op/s</sub> | 0.693<br><sub>context: p90 0.892 · p95 0.946 · p99 1.078 · 11084 op/s</sub> | +0.9% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.235<br><sub>context: p90 1.635 · p95 1.815 · p99 1.904 · 799 op/s</sub> | 1.198<br><sub>context: p90 1.529 · p95 1.693 · p99 1.890 · 826 op/s</sub> | -3.1% (-0.038) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.313<br><sub>context: p90 1.842 · p95 2.024 · p99 2.321 · 5811 op/s</sub> | 1.336<br><sub>context: p90 1.837 · p95 2.000 · p99 2.321 · 5754 op/s</sub> | +1.7% (+0.023) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.277<br><sub>context: p90 1.666 · p95 1.872 · p99 2.084 · 760 op/s</sub> | 1.290<br><sub>context: p90 1.667 · p95 1.791 · p99 2.097 · 750 op/s</sub> | +1.0% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.379<br><sub>context: p90 1.923 · p95 2.115 · p99 2.453 · 5511 op/s</sub> | 1.377<br><sub>context: p90 1.890 · p95 2.061 · p99 2.370 · 5557 op/s</sub> | -0.1% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.778<br><sub>context: p90 0.962 · p95 0.999 · p99 1.058 · 1279 op/s</sub> | 0.845<br><sub>context: p90 1.058 · p95 1.102 · p99 1.193 · 1184 op/s</sub> | +8.7% (+0.068) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.793<br><sub>context: p90 1.036 · p95 1.109 · p99 1.284 · 9850 op/s</sub> | 0.797<br><sub>context: p90 1.063 · p95 1.147 · p99 1.272 · 9763 op/s</sub> | +0.4% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.477<br><sub>context: p90 0.591 · p95 0.622 · p99 0.662 · 2057 op/s</sub> | 0.544<br><sub>context: p90 0.631 · p95 0.657 · p99 0.685 · 1834 op/s</sub> | +14.0% (+0.067) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.419<br><sub>context: p90 0.520 · p95 0.549 · p99 0.602 · 18380 op/s</sub> | 0.429<br><sub>context: p90 0.542 · p95 0.572 · p99 0.646 · 16200 op/s</sub> | +2.6% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.248 · p95 0.273 · p99 0.299 · 5248 op/s</sub> | 0.207<br><sub>context: p90 0.302 · p95 0.330 · p99 0.351 · 4472 op/s</sub> | +16.1% (+0.029) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.361<br><sub>context: p90 0.771 · p95 0.903 · p99 1.079 · 17970 op/s</sub> | 0.233<br><sub>context: p90 0.297 · p95 0.319 · p99 0.360 · 33392 op/s</sub> | -35.4% (-0.128) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.220 · p95 0.243 · p99 0.279 · 5975 op/s</sub> | 0.142<br><sub>context: p90 0.209 · p95 0.217 · p99 0.227 · 6543 op/s</sub> | -9.2% (-0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.210<br><sub>context: p90 0.291 · p95 0.319 · p99 0.385 · 36332 op/s</sub> | 0.209<br><sub>context: p90 0.293 · p95 0.318 · p99 0.411 · 36065 op/s</sub> | -0.2% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.546<br><sub>context: p90 0.661 · p95 0.707 · p99 0.767 · 1791 op/s</sub> | 0.527<br><sub>context: p90 0.633 · p95 0.659 · p99 0.686 · 1866 op/s</sub> | -3.6% (-0.019) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.461<br><sub>context: p90 0.573 · p95 0.609 · p99 0.691 · 16675 op/s</sub> | 0.460<br><sub>context: p90 0.577 · p95 0.613 · p99 0.683 · 16686 op/s</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.538<br><sub>context: p90 2.184 · p95 2.365 · p99 2.671 · 639 op/s</sub> | 1.485<br><sub>context: p90 2.146 · p95 2.327 · p99 2.642 · 652 op/s</sub> | -3.4% (-0.053) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.253<br><sub>context: p90 3.449 · p95 3.891 · p99 4.389 · 3401 op/s</sub> | 2.152<br><sub>context: p90 3.334 · p95 3.746 · p99 4.329 · 3505 op/s</sub> | -4.5% (-0.101) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.648<br><sub>context: p90 6.902 · p95 7.699 · p99 8.154 · 207 op/s</sub> | 4.699<br><sub>context: p90 6.974 · p95 7.693 · p99 8.243 · 205 op/s</sub> | +1.1% (+0.051) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.267<br><sub>context: p90 12.635 · p95 13.928 · p99 15.901 · 933 op/s</sub> | 8.201<br><sub>context: p90 12.370 · p95 13.654 · p99 15.242 · 946 op/s</sub> | -0.8% (-0.066) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.302<br><sub>context: p90 0.466 · p95 0.494 · p99 0.535 · 2982 op/s</sub> | 0.269<br><sub>context: p90 0.343 · p95 0.373 · p99 0.413 · 3585 op/s</sub> | -11.2% (-0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.322<br><sub>context: p90 0.469 · p95 0.523 · p99 0.637 · 21969 op/s</sub> | 0.311<br><sub>context: p90 0.424 · p95 0.458 · p99 0.552 · 23985 op/s</sub> | -3.6% (-0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.334<br><sub>context: p90 0.495 · p95 0.516 · p99 0.549 · 2837 op/s</sub> | 0.238<br><sub>context: p90 0.318 · p95 0.345 · p99 0.390 · 4019 op/s</sub> | -28.9% (-0.096) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.313<br><sub>context: p90 0.424 · p95 0.464 · p99 0.552 · 23744 op/s</sub> | 0.305<br><sub>context: p90 0.406 · p95 0.450 · p99 0.523 · 25030 op/s</sub> | -2.5% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.398<br><sub>context: p90 0.528 · p95 0.557 · p99 0.596 · 2444 op/s</sub> | 0.409<br><sub>context: p90 0.525 · p95 0.550 · p99 0.592 · 2410 op/s</sub> | +2.7% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.368<br><sub>context: p90 0.471 · p95 0.507 · p99 0.575 · 20676 op/s</sub> | 0.386<br><sub>context: p90 0.502 · p95 0.539 · p99 0.648 · 19592 op/s</sub> | +4.8% (+0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.206<br><sub>context: p90 0.254 · p95 0.288 · p99 0.342 · 4697 op/s</sub> | 0.174<br><sub>context: p90 0.240 · p95 0.246 · p99 0.275 · 5438 op/s</sub> | -15.7% (-0.032) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.232<br><sub>context: p90 0.313 · p95 0.340 · p99 0.405 · 32491 op/s</sub> | 0.230<br><sub>context: p90 0.306 · p95 0.334 · p99 0.369 · 33090 op/s</sub> | -0.9% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.486<br><sub>context: p90 0.601 · p95 0.637 · p99 0.689 · 2015 op/s</sub> | 0.501<br><sub>context: p90 0.601 · p95 0.638 · p99 0.675 · 1968 op/s</sub> | +3.2% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.412<br><sub>context: p90 0.519 · p95 0.565 · p99 0.677 · 18403 op/s</sub> | 0.420<br><sub>context: p90 0.531 · p95 0.570 · p99 0.651 · 17993 op/s</sub> | +1.8% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.453<br><sub>context: p90 0.551 · p95 0.584 · p99 0.631 · 2199 op/s</sub> | 0.395<br><sub>context: p90 0.507 · p95 0.523 · p99 0.615 · 2437 op/s</sub> | -12.9% (-0.058) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.351<br><sub>context: p90 0.469 · p95 0.515 · p99 0.613 · 20941 op/s</sub> | 0.348<br><sub>context: p90 0.457 · p95 0.488 · p99 0.561 · 21884 op/s</sub> | -0.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.581<br><sub>context: p90 0.698 · p95 0.744 · p99 0.797 · 1684 op/s</sub> | 0.514<br><sub>context: p90 0.608 · p95 0.659 · p99 0.718 · 1904 op/s</sub> | -11.5% (-0.067) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.476<br><sub>context: p90 0.598 · p95 0.634 · p99 0.737 · 16191 op/s</sub> | 0.450<br><sub>context: p90 0.561 · p95 0.604 · p99 0.675 · 16917 op/s</sub> | -5.4% (-0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.680<br><sub>context: p90 0.814 · p95 0.832 · p99 0.941 · 1470 op/s</sub> | 0.640<br><sub>context: p90 0.779 · p95 0.802 · p99 0.912 · 1541 op/s</sub> | -5.9% (-0.040) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.636<br><sub>context: p90 0.827 · p95 0.888 · p99 1.020 · 11855 op/s</sub> | 0.647<br><sub>context: p90 0.842 · p95 0.905 · p99 1.021 · 11806 op/s</sub> | +1.7% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.644<br><sub>context: p90 0.760 · p95 0.793 · p99 0.904 · 1537 op/s</sub> | 0.646<br><sub>context: p90 0.770 · p95 0.815 · p99 0.918 · 1529 op/s</sub> | +0.4% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.647<br><sub>context: p90 0.809 · p95 0.865 · p99 0.970 · 11893 op/s</sub> | 0.656<br><sub>context: p90 0.823 · p95 0.886 · p99 0.977 · 11707 op/s</sub> | +1.4% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.555<br><sub>context: p90 0.651 · p95 0.683 · p99 0.764 · 1786 op/s</sub> | 0.515<br><sub>context: p90 0.613 · p95 0.648 · p99 0.702 · 1904 op/s</sub> | -7.2% (-0.040) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.488<br><sub>context: p90 0.594 · p95 0.623 · p99 0.692 · 15834 op/s</sub> | 0.483<br><sub>context: p90 0.611 · p95 0.657 · p99 0.782 · 15935 op/s</sub> | -1.1% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.470<br><sub>context: p90 0.586 · p95 0.618 · p99 0.641 · 2082 op/s</sub> | 0.525<br><sub>context: p90 0.633 · p95 0.688 · p99 0.722 · 1890 op/s</sub> | +11.9% (+0.056) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.409<br><sub>context: p90 0.525 · p95 0.561 · p99 0.636 · 18615 op/s</sub> | 0.425<br><sub>context: p90 0.557 · p95 0.608 · p99 0.691 · 17938 op/s</sub> | +3.9% (+0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.496<br><sub>context: p90 15.676 · p95 15.786 · p99 15.925 · 64 op/s</sub> | 15.418<br><sub>context: p90 15.580 · p95 15.633 · p99 15.735 · 65 op/s</sub> | -0.5% (-0.078) | 10% AND 0.5 ms | 🟢 |
| 8 | 18.912<br><sub>context: p90 25.178 · p95 27.864 · p99 31.332 · 379 op/s</sub> | 18.779<br><sub>context: p90 25.033 · p95 28.322 · p99 31.965 · 380 op/s</sub> | -0.7% (-0.133) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.527<br><sub>context: p90 0.644 · p95 0.663 · p99 0.741 · 1832 op/s</sub> | 0.557<br><sub>context: p90 0.661 · p95 0.680 · p99 0.702 · 1761 op/s</sub> | +5.7% (+0.030) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.470<br><sub>context: p90 0.570 · p95 0.597 · p99 0.672 · 16205 op/s</sub> | 0.468<br><sub>context: p90 0.585 · p95 0.620 · p99 0.687 · 16351 op/s</sub> | -0.4% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.416<br><sub>context: p90 1.937 · p95 2.077 · p99 2.379 · 691 op/s</sub> | 1.568<br><sub>context: p90 2.098 · p95 2.206 · p99 2.423 · 630 op/s</sub> | +10.8% (+0.153) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.619<br><sub>context: p90 2.212 · p95 2.438 · p99 2.935 · 4821 op/s</sub> | 1.683<br><sub>context: p90 2.345 · p95 2.560 · p99 2.984 · 4586 op/s</sub> | +4.0% (+0.064) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.524<br><sub>context: p90 0.613 · p95 0.633 · p99 0.681 · 1889 op/s</sub> | 0.590<br><sub>context: p90 0.702 · p95 0.737 · p99 0.793 · 1662 op/s</sub> | +12.7% (+0.066) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.494<br><sub>context: p90 0.623 · p95 0.666 · p99 0.760 · 15633 op/s</sub> | 0.541<br><sub>context: p90 0.712 · p95 0.775 · p99 0.887 · 13923 op/s</sub> | +9.4% (+0.047) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.470<br><sub>context: p90 0.569 · p95 0.587 · p99 0.642 · 2129 op/s</sub> | 0.545<br><sub>context: p90 0.676 · p95 0.743 · p99 0.820 · 1788 op/s</sub> | +16.1% (+0.076) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.431<br><sub>context: p90 0.547 · p95 0.584 · p99 0.659 · 17810 op/s</sub> | 0.461<br><sub>context: p90 0.596 · p95 0.642 · p99 0.769 · 16549 op/s</sub> | +7.0% (+0.030) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.509<br><sub>context: p90 0.606 · p95 0.631 · p99 0.697 · 1943 op/s</sub> | 0.565<br><sub>context: p90 0.681 · p95 0.712 · p99 0.790 · 1750 op/s</sub> | +11.0% (+0.056) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.454<br><sub>context: p90 0.577 · p95 0.616 · p99 0.695 · 16984 op/s</sub> | 0.464<br><sub>context: p90 0.594 · p95 0.646 · p99 0.757 · 16415 op/s</sub> | +2.2% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.280<br><sub>context: p90 0.384 · p95 0.413 · p99 0.459 · 3424 op/s</sub> | 0.336<br><sub>context: p90 0.421 · p95 0.446 · p99 0.494 · 2964 op/s</sub> | +20.2% (+0.056) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.315<br><sub>context: p90 0.430 · p95 0.474 · p99 0.551 · 23858 op/s</sub> | 0.325<br><sub>context: p90 0.443 · p95 0.494 · p99 0.569 · 22774 op/s</sub> | +3.3% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.254<br><sub>context: p90 0.315 · p95 0.328 · p99 0.392 · 3893 op/s</sub> | 0.233<br><sub>context: p90 0.296 · p95 0.316 · p99 0.385 · 4208 op/s</sub> | -8.2% (-0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.253<br><sub>context: p90 0.334 · p95 0.368 · p99 0.442 · 27168 op/s</sub> | 0.249<br><sub>context: p90 0.318 · p95 0.342 · p99 0.377 · 30729 op/s</sub> | -1.6% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.432<br><sub>context: p90 0.533 · p95 0.561 · p99 0.630 · 2290 op/s</sub> | 0.419<br><sub>context: p90 0.539 · p95 0.578 · p99 0.623 · 2247 op/s</sub> | -3.1% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.380<br><sub>context: p90 0.502 · p95 0.556 · p99 0.655 · 19683 op/s</sub> | 0.392<br><sub>context: p90 0.518 · p95 0.558 · p99 0.631 · 19348 op/s</sub> | +3.1% (+0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.428<br><sub>context: p90 0.522 · p95 0.550 · p99 0.645 · 2281 op/s</sub> | 0.406<br><sub>context: p90 0.507 · p95 0.541 · p99 0.608 · 2402 op/s</sub> | -5.1% (-0.022) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.416<br><sub>context: p90 0.581 · p95 0.643 · p99 0.777 · 17803 op/s</sub> | 0.414<br><sub>context: p90 0.562 · p95 0.619 · p99 0.724 · 17976 op/s</sub> | -0.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.331<br><sub>context: p90 0.436 · p95 0.478 · p99 0.529 · 2899 op/s</sub> | 0.379<br><sub>context: p90 0.495 · p95 0.529 · p99 0.570 · 2595 op/s</sub> | +14.6% (+0.048) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.318<br><sub>context: p90 0.419 · p95 0.453 · p99 0.509 · 23563 op/s</sub> | 0.327<br><sub>context: p90 0.429 · p95 0.474 · p99 0.543 · 23485 op/s</sub> | +2.8% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.216<br><sub>context: p90 1.322 · p95 1.371 · p99 1.542 · 818 op/s</sub> | 1.249<br><sub>context: p90 1.380 · p95 1.416 · p99 1.487 · 792 op/s</sub> | +2.7% (+0.033) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.272<br><sub>context: p90 1.467 · p95 1.532 · p99 1.630 · 6153 op/s</sub> | 1.251<br><sub>context: p90 1.446 · p95 1.520 · p99 1.635 · 6263 op/s</sub> | -1.6% (-0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.195<br><sub>context: p90 1.296 · p95 1.323 · p99 1.372 · 829 op/s</sub> | 1.145<br><sub>context: p90 1.242 · p95 1.266 · p99 1.337 · 865 op/s</sub> | -4.2% (-0.051) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.163<br><sub>context: p90 1.327 · p95 1.391 · p99 1.528 · 6690 op/s</sub> | 1.146<br><sub>context: p90 1.321 · p95 1.373 · p99 1.488 · 6851 op/s</sub> | -1.4% (-0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.412<br><sub>context: p90 0.519 · p95 0.540 · p99 0.589 · 2341 op/s</sub> | 0.474<br><sub>context: p90 0.595 · p95 0.627 · p99 0.679 · 2062 op/s</sub> | +15.3% (+0.063) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.390<br><sub>context: p90 0.494 · p95 0.524 · p99 0.576 · 19630 op/s</sub> | 0.399<br><sub>context: p90 0.511 · p95 0.551 · p99 0.636 · 18804 op/s</sub> | +2.2% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.545<br><sub>context: p90 0.672 · p95 0.703 · p99 0.766 · 1788 op/s</sub> | 0.637<br><sub>context: p90 0.752 · p95 0.784 · p99 0.862 · 1559 op/s</sub> | +16.8% (+0.092) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.506<br><sub>context: p90 0.638 · p95 0.678 · p99 0.772 · 15315 op/s</sub> | 0.493<br><sub>context: p90 0.623 · p95 0.676 · p99 0.757 · 15345 op/s</sub> | -2.5% (-0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.358<br><sub>context: p90 0.469 · p95 0.500 · p99 0.546 · 2682 op/s</sub> | 0.361<br><sub>context: p90 0.435 · p95 0.471 · p99 0.518 · 2757 op/s</sub> | +1.0% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.356<br><sub>context: p90 0.479 · p95 0.523 · p99 0.607 · 21045 op/s</sub> | 0.330<br><sub>context: p90 0.435 · p95 0.470 · p99 0.544 · 22690 op/s</sub> | -7.1% (-0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.369 · p95 0.418 · p99 0.487 · 3396 op/s</sub> | 0.342<br><sub>context: p90 0.438 · p95 0.474 · p99 0.610 · 2855 op/s</sub> | +21.5% (+0.061) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.312<br><sub>context: p90 0.414 · p95 0.452 · p99 0.537 · 23957 op/s</sub> | 0.318<br><sub>context: p90 0.437 · p95 0.478 · p99 0.557 · 23363 op/s</sub> | +2.2% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.432 · p95 0.470 · p99 0.507 · 3078 op/s</sub> | 0.327<br><sub>context: p90 0.440 · p95 0.480 · p99 0.520 · 2984 op/s</sub> | +2.7% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.329<br><sub>context: p90 0.461 · p95 0.503 · p99 0.600 · 22874 op/s</sub> | 0.328<br><sub>context: p90 0.453 · p95 0.496 · p99 0.604 · 22813 op/s</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.508<br><sub>context: p90 0.611 · p95 0.634 · p99 0.672 · 1924 op/s</sub> | 0.534<br><sub>context: p90 0.625 · p95 0.649 · p99 0.723 · 1848 op/s</sub> | +5.2% (+0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.472<br><sub>context: p90 0.557 · p95 0.591 · p99 0.635 · 16480 op/s</sub> | 0.476<br><sub>context: p90 0.573 · p95 0.603 · p99 0.666 · 16228 op/s</sub> | +0.8% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>
