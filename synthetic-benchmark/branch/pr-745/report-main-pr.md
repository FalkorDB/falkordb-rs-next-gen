### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:8b5e5c3b1fe54d523552910dd06e2ae5fa9e8fc2d74fd4968b2eadbd1d1c986c | ghcr.io/falkordb/falkordb-server@sha256:d4f9dbb30b9f70e6d965d2dce47b239c27fb238fd4d3a272d1434cd7762414f7 |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:8b5e5c3b1fe54d523552910dd06e2ae5fa9e8fc2d74fd4968b2eadbd1d1c986c → ghcr.io/falkordb/falkordb-server@sha256:d4f9dbb30b9f70e6d965d2dce47b239c27fb238fd4d3a272d1434cd7762414f7

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.735<br><sub>context: p90 0.804 · p95 0.824 · p99 0.856 · 1330 op/s</sub> | 0.779<br><sub>context: p90 0.803 · p95 0.818 · p99 0.831 · 1272 op/s</sub> | +5.9% (+0.044) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.903<br><sub>context: p90 1.048 · p95 1.085 · p99 1.149 · 8750 op/s</sub> | 0.912<br><sub>context: p90 1.038 · p95 1.069 · p99 1.143 · 8612 op/s</sub> | +1.0% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.797<br><sub>context: p90 0.851 · p95 0.875 · p99 0.896 · 1238 op/s</sub> | 0.855<br><sub>context: p90 0.934 · p95 0.946 · p99 0.961 · 1151 op/s</sub> | +7.2% (+0.058) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.995<br><sub>context: p90 1.190 · p95 1.229 · p99 1.294 · 7864 op/s</sub> | 1.093<br><sub>context: p90 1.292 · p95 1.325 · p99 1.381 · 7198 op/s</sub> | +9.8% (+0.098) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.269<br><sub>context: p90 1.306 · p95 1.317 · p99 1.345 · 780 op/s</sub> | 1.291<br><sub>context: p90 1.339 · p95 1.355 · p99 1.377 · 765 op/s</sub> | +1.8% (+0.022) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.751<br><sub>context: p90 2.273 · p95 2.456 · p99 2.735 · 4392 op/s</sub> | 1.771<br><sub>context: p90 2.254 · p95 2.454 · p99 2.763 · 4332 op/s</sub> | +1.1% (+0.020) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.345<br><sub>context: p90 1.390 · p95 1.409 · p99 1.438 · 736 op/s</sub> | 1.410<br><sub>context: p90 1.485 · p95 1.554 · p99 1.592 · 701 op/s</sub> | +4.8% (+0.064) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.621<br><sub>context: p90 1.947 · p95 1.991 · p99 2.061 · 4788 op/s</sub> | 1.643<br><sub>context: p90 1.967 · p95 2.011 · p99 2.096 · 4718 op/s</sub> | +1.3% (+0.022) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.125<br><sub>context: p90 0.135 · p95 0.143 · p99 0.162 · 7457 op/s</sub> | 0.142<br><sub>context: p90 0.171 · p95 0.183 · p99 0.213 · 6530 op/s</sub> | +13.8% (+0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.213<br><sub>context: p90 0.277 · p95 0.302 · p99 0.353 · 36558 op/s</sub> | 0.222<br><sub>context: p90 0.287 · p95 0.306 · p99 0.345 · 34971 op/s</sub> | +4.5% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.227<br><sub>context: p90 0.272 · p95 0.291 · p99 0.310 · 4214 op/s</sub> | 0.212<br><sub>context: p90 0.258 · p95 0.280 · p99 0.291 · 4574 op/s</sub> | -6.7% (-0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.321<br><sub>context: p90 0.410 · p95 0.445 · p99 0.515 · 23669 op/s</sub> | 0.324<br><sub>context: p90 0.411 · p95 0.437 · p99 0.501 · 23537 op/s</sub> | +1.0% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.243<br><sub>context: p90 0.294 · p95 0.332 · p99 0.369 · 3973 op/s</sub> | 0.276<br><sub>context: p90 0.369 · p95 0.387 · p99 0.424 · 3485 op/s</sub> | +14.0% (+0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.344<br><sub>context: p90 0.432 · p95 0.464 · p99 0.504 · 22389 op/s</sub> | 0.349<br><sub>context: p90 0.433 · p95 0.458 · p99 0.531 · 21914 op/s</sub> | +1.3% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.387 · p95 0.405 · p99 0.438 · 3022 op/s</sub> | 0.344<br><sub>context: p90 0.397 · p95 0.417 · p99 0.472 · 2927 op/s</sub> | +6.5% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.543 · p95 0.573 · p99 0.627 · 17258 op/s</sub> | 0.442<br><sub>context: p90 0.541 · p95 0.570 · p99 0.636 · 17557 op/s</sub> | -0.9% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.412 · p95 0.428 · p99 0.449 · 2672 op/s</sub> | 0.355<br><sub>context: p90 0.408 · p95 0.418 · p99 0.441 · 2784 op/s</sub> | -4.2% (-0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.574 · p95 0.604 · p99 0.663 · 16318 op/s</sub> | 0.471<br><sub>context: p90 0.579 · p95 0.610 · p99 0.675 · 16427 op/s</sub> | +1.0% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.461<br><sub>context: p90 0.556 · p95 0.596 · p99 0.638 · 2127 op/s</sub> | 0.466<br><sub>context: p90 0.562 · p95 0.587 · p99 0.687 · 2109 op/s</sub> | +1.1% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.617<br><sub>context: p90 0.784 · p95 0.835 · p99 0.955 · 12430 op/s</sub> | 0.619<br><sub>context: p90 0.788 · p95 0.839 · p99 0.945 · 12533 op/s</sub> | +0.4% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.491<br><sub>context: p90 0.597 · p95 0.642 · p99 0.688 · 1993 op/s</sub> | 0.512<br><sub>context: p90 0.633 · p95 0.693 · p99 0.765 · 1899 op/s</sub> | +4.3% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.653<br><sub>context: p90 0.836 · p95 0.883 · p99 0.988 · 11744 op/s</sub> | 0.649<br><sub>context: p90 0.822 · p95 0.888 · p99 0.980 · 11908 op/s</sub> | -0.7% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.974<br><sub>context: p90 1.258 · p95 1.529 · p99 1.680 · 1027 op/s</sub> | 0.979<br><sub>context: p90 1.271 · p95 1.444 · p99 1.663 · 1015 op/s</sub> | +0.5% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.253<br><sub>context: p90 1.770 · p95 1.923 · p99 2.218 · 6188 op/s</sub> | 1.234<br><sub>context: p90 1.739 · p95 1.896 · p99 2.193 · 6233 op/s</sub> | -1.5% (-0.019) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.966<br><sub>context: p90 1.275 · p95 1.447 · p99 1.707 · 1000 op/s</sub> | 0.986<br><sub>context: p90 1.308 · p95 1.475 · p99 1.713 · 987 op/s</sub> | +2.2% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.258<br><sub>context: p90 1.759 · p95 1.919 · p99 2.223 · 6029 op/s</sub> | 1.272<br><sub>context: p90 1.721 · p95 1.928 · p99 2.225 · 6000 op/s</sub> | +1.2% (+0.015) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.591<br><sub>context: p90 0.734 · p95 0.789 · p99 0.871 · 1700 op/s</sub> | 0.549<br><sub>context: p90 0.706 · p95 0.740 · p99 0.792 · 1785 op/s</sub> | -7.1% (-0.042) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.747<br><sub>context: p90 0.990 · p95 1.081 · p99 1.227 · 10415 op/s</sub> | 0.737<br><sub>context: p90 0.968 · p95 1.043 · p99 1.200 · 10559 op/s</sub> | -1.4% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.306<br><sub>context: p90 0.355 · p95 0.369 · p99 0.416 · 3251 op/s</sub> | 0.299<br><sub>context: p90 0.353 · p95 0.378 · p99 0.413 · 3261 op/s</sub> | -2.3% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.397<br><sub>context: p90 0.490 · p95 0.519 · p99 0.578 · 19544 op/s</sub> | 0.402<br><sub>context: p90 0.488 · p95 0.513 · p99 0.569 · 19240 op/s</sub> | +1.2% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.128<br><sub>context: p90 0.163 · p95 0.170 · p99 0.182 · 7332 op/s</sub> | 0.153<br><sub>context: p90 0.169 · p95 0.171 · p99 0.180 · 6398 op/s</sub> | +19.5% (+0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.226<br><sub>context: p90 0.298 · p95 0.321 · p99 0.359 · 33933 op/s</sub> | 0.221<br><sub>context: p90 0.283 · p95 0.301 · p99 0.342 · 34609 op/s</sub> | -2.2% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.113<br><sub>context: p90 0.123 · p95 0.128 · p99 0.141 · 8480 op/s</sub> | 0.112<br><sub>context: p90 0.134 · p95 0.137 · p99 0.143 · 8324 op/s</sub> | -0.7% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.199<br><sub>context: p90 0.280 · p95 0.309 · p99 0.380 · 38004 op/s</sub> | 0.201<br><sub>context: p90 0.271 · p95 0.292 · p99 0.340 · 37981 op/s</sub> | +1.1% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.390 · p95 0.412 · p99 0.455 · 2979 op/s</sub> | 0.319<br><sub>context: p90 0.382 · p95 0.398 · p99 0.434 · 3043 op/s</sub> | -1.5% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.544 · p95 0.579 · p99 0.634 · 17414 op/s</sub> | 0.442<br><sub>context: p90 0.539 · p95 0.571 · p99 0.618 · 17538 op/s</sub> | -0.9% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.210<br><sub>context: p90 1.858 · p95 2.035 · p99 2.284 · 795 op/s</sub> | 1.196<br><sub>context: p90 1.798 · p95 2.000 · p99 2.208 · 803 op/s</sub> | -1.1% (-0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.167<br><sub>context: p90 3.428 · p95 3.875 · p99 4.389 · 3490 op/s</sub> | 2.094<br><sub>context: p90 3.206 · p95 3.618 · p99 4.176 · 3674 op/s</sub> | -3.4% (-0.073) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.396<br><sub>context: p90 6.575 · p95 7.276 · p99 7.984 · 222 op/s</sub> | 4.366<br><sub>context: p90 6.619 · p95 7.337 · p99 7.848 · 220 op/s</sub> | -0.7% (-0.030) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.325<br><sub>context: p90 12.717 · p95 13.787 · p99 15.668 · 934 op/s</sub> | 8.006<br><sub>context: p90 12.328 · p95 13.497 · p99 15.403 · 970 op/s</sub> | -3.8% (-0.319) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.168<br><sub>context: p90 0.199 · p95 0.210 · p99 0.224 · 5612 op/s</sub> | 0.167<br><sub>context: p90 0.187 · p95 0.194 · p99 0.208 · 5875 op/s</sub> | -1.1% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.389 · p95 0.431 · p99 0.497 · 25615 op/s</sub> | 0.296<br><sub>context: p90 0.395 · p95 0.436 · p99 0.510 · 25379 op/s</sub> | -0.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.162<br><sub>context: p90 0.190 · p95 0.198 · p99 0.212 · 6010 op/s</sub> | 0.154<br><sub>context: p90 0.194 · p95 0.210 · p99 0.252 · 5882 op/s</sub> | -4.4% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.298<br><sub>context: p90 0.399 · p95 0.432 · p99 0.508 · 25499 op/s</sub> | 0.301<br><sub>context: p90 0.407 · p95 0.453 · p99 0.518 · 25092 op/s</sub> | +1.1% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.278<br><sub>context: p90 0.296 · p95 0.304 · p99 0.332 · 3505 op/s</sub> | 0.217<br><sub>context: p90 0.258 · p95 0.264 · p99 0.272 · 4361 op/s</sub> | -22.1% (-0.061) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.358<br><sub>context: p90 0.450 · p95 0.488 · p99 0.563 · 21265 op/s</sub> | 0.356<br><sub>context: p90 0.456 · p95 0.498 · p99 0.570 · 21365 op/s</sub> | -0.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.136<br><sub>context: p90 0.165 · p95 0.174 · p99 0.193 · 6848 op/s</sub> | 0.132<br><sub>context: p90 0.162 · p95 0.170 · p99 0.180 · 6853 op/s</sub> | -3.0% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.221<br><sub>context: p90 0.281 · p95 0.302 · p99 0.335 · 35082 op/s</sub> | 0.218<br><sub>context: p90 0.286 · p95 0.304 · p99 0.337 · 35527 op/s</sub> | -1.4% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.341<br><sub>context: p90 0.383 · p95 0.400 · p99 0.447 · 2912 op/s</sub> | 0.334<br><sub>context: p90 0.400 · p95 0.430 · p99 0.474 · 2908 op/s</sub> | -2.0% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.391<br><sub>context: p90 0.472 · p95 0.498 · p99 0.555 · 20035 op/s</sub> | 0.389<br><sub>context: p90 0.477 · p95 0.505 · p99 0.565 · 19813 op/s</sub> | -0.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.249<br><sub>context: p90 0.303 · p95 0.326 · p99 0.358 · 3922 op/s</sub> | 0.236<br><sub>context: p90 0.318 · p95 0.338 · p99 0.376 · 4018 op/s</sub> | -5.2% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.319<br><sub>context: p90 0.405 · p95 0.436 · p99 0.500 · 23762 op/s</sub> | 0.329<br><sub>context: p90 0.407 · p95 0.432 · p99 0.479 · 23169 op/s</sub> | +2.8% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.378 · p95 0.388 · p99 0.432 · 3081 op/s</sub> | 0.347<br><sub>context: p90 0.412 · p95 0.445 · p99 0.486 · 2800 op/s</sub> | +9.4% (+0.030) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.429<br><sub>context: p90 0.534 · p95 0.564 · p99 0.621 · 17927 op/s</sub> | 0.435<br><sub>context: p90 0.540 · p95 0.576 · p99 0.617 · 17711 op/s</sub> | +1.3% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.446<br><sub>context: p90 0.540 · p95 0.565 · p99 0.598 · 2195 op/s</sub> | 0.481<br><sub>context: p90 0.612 · p95 0.663 · p99 0.731 · 2031 op/s</sub> | +8.0% (+0.036) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.602<br><sub>context: p90 0.771 · p95 0.823 · p99 0.944 · 12772 op/s</sub> | 0.600<br><sub>context: p90 0.768 · p95 0.821 · p99 0.933 · 12866 op/s</sub> | -0.4% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.452<br><sub>context: p90 0.516 · p95 0.550 · p99 0.594 · 2211 op/s</sub> | 0.504<br><sub>context: p90 0.617 · p95 0.659 · p99 0.737 · 1941 op/s</sub> | +11.4% (+0.052) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.607<br><sub>context: p90 0.761 · p95 0.804 · p99 0.896 · 12779 op/s</sub> | 0.609<br><sub>context: p90 0.765 · p95 0.821 · p99 0.943 · 12694 op/s</sub> | +0.4% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.340<br><sub>context: p90 0.394 · p95 0.412 · p99 0.449 · 2883 op/s</sub> | 0.364<br><sub>context: p90 0.438 · p95 0.464 · p99 0.512 · 2673 op/s</sub> | +7.3% (+0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.461<br><sub>context: p90 0.571 · p95 0.606 · p99 0.678 · 16787 op/s</sub> | 0.454<br><sub>context: p90 0.551 · p95 0.585 · p99 0.647 · 16956 op/s</sub> | -1.5% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.254<br><sub>context: p90 0.308 · p95 0.322 · p99 0.344 · 3829 op/s</sub> | 0.287<br><sub>context: p90 0.364 · p95 0.391 · p99 0.417 · 3335 op/s</sub> | +12.9% (+0.033) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.379<br><sub>context: p90 0.468 · p95 0.500 · p99 0.559 · 20484 op/s</sub> | 0.375<br><sub>context: p90 0.472 · p95 0.500 · p99 0.559 · 20410 op/s</sub> | -1.0% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.091<br><sub>context: p90 15.285 · p95 15.359 · p99 15.467 · 66 op/s</sub> | 15.074<br><sub>context: p90 15.286 · p95 15.371 · p99 15.454 · 66 op/s</sub> | -0.1% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 18.238<br><sub>context: p90 23.512 · p95 26.386 · p99 30.128 · 393 op/s</sub> | 18.118<br><sub>context: p90 23.194 · p95 25.893 · p99 29.209 · 390 op/s</sub> | -0.7% (-0.120) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.347<br><sub>context: p90 0.402 · p95 0.427 · p99 0.473 · 2793 op/s</sub> | 0.368<br><sub>context: p90 0.455 · p95 0.484 · p99 0.548 · 2611 op/s</sub> | +6.1% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.532 · p95 0.561 · p99 0.620 · 17386 op/s</sub> | 0.448<br><sub>context: p90 0.542 · p95 0.576 · p99 0.640 · 17313 op/s</sub> | +0.4% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.224<br><sub>context: p90 1.687 · p95 1.873 · p99 2.050 · 802 op/s</sub> | 1.211<br><sub>context: p90 1.652 · p95 1.798 · p99 1.958 · 809 op/s</sub> | -1.1% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.567<br><sub>context: p90 2.170 · p95 2.394 · p99 2.832 · 5003 op/s</sub> | 1.539<br><sub>context: p90 2.180 · p95 2.394 · p99 2.840 · 4968 op/s</sub> | -1.8% (-0.027) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.350<br><sub>context: p90 0.410 · p95 0.420 · p99 0.456 · 2809 op/s</sub> | 0.360<br><sub>context: p90 0.424 · p95 0.457 · p99 0.482 · 2717 op/s</sub> | +2.8% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.465<br><sub>context: p90 0.578 · p95 0.608 · p99 0.673 · 16407 op/s</sub> | 0.473<br><sub>context: p90 0.590 · p95 0.622 · p99 0.709 · 16276 op/s</sub> | +1.8% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.342 · p95 0.365 · p99 0.385 · 3329 op/s</sub> | 0.339<br><sub>context: p90 0.403 · p95 0.425 · p99 0.463 · 2865 op/s</sub> | +13.5% (+0.040) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.412<br><sub>context: p90 0.524 · p95 0.559 · p99 0.630 · 18534 op/s</sub> | 0.416<br><sub>context: p90 0.523 · p95 0.558 · p99 0.629 · 18463 op/s</sub> | +1.1% (+0.004) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.314<br><sub>context: p90 0.367 · p95 0.379 · p99 0.392 · 3123 op/s</sub> | 0.346<br><sub>context: p90 0.417 · p95 0.448 · p99 0.487 · 2782 op/s</sub> | +10.1% (+0.032) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.544 · p95 0.572 · p99 0.656 · 17845 op/s</sub> | 0.436<br><sub>context: p90 0.549 · p95 0.580 · p99 0.666 · 17516 op/s</sub> | +0.8% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.223 · p95 0.240 · p99 0.281 · 5105 op/s</sub> | 0.177<br><sub>context: p90 0.221 · p95 0.246 · p99 0.278 · 5298 op/s</sub> | -4.4% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.298<br><sub>context: p90 0.403 · p95 0.438 · p99 0.519 · 25373 op/s</sub> | 0.302<br><sub>context: p90 0.407 · p95 0.440 · p99 0.506 · 25240 op/s</sub> | +1.3% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.159<br><sub>context: p90 0.182 · p95 0.187 · p99 0.193 · 6117 op/s</sub> | 0.146<br><sub>context: p90 0.186 · p95 0.199 · p99 0.215 · 6279 op/s</sub> | -8.1% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.243<br><sub>context: p90 0.313 · p95 0.339 · p99 0.385 · 31197 op/s</sub> | 0.241<br><sub>context: p90 0.305 · p95 0.325 · p99 0.368 · 31726 op/s</sub> | -0.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.215<br><sub>context: p90 0.265 · p95 0.279 · p99 0.328 · 4459 op/s</sub> | 0.240<br><sub>context: p90 0.293 · p95 0.304 · p99 0.337 · 3562 op/s</sub> | +11.6% (+0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.368<br><sub>context: p90 0.482 · p95 0.514 · p99 0.599 · 20598 op/s</sub> | 0.365<br><sub>context: p90 0.476 · p95 0.518 · p99 0.600 · 21027 op/s</sub> | -0.9% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.310 · p95 0.326 · p99 0.347 · 3989 op/s</sub> | 0.240<br><sub>context: p90 0.314 · p95 0.327 · p99 0.358 · 3942 op/s</sub> | +1.3% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.372<br><sub>context: p90 0.480 · p95 0.519 · p99 0.595 · 20581 op/s</sub> | 0.368<br><sub>context: p90 0.480 · p95 0.522 · p99 0.607 · 20396 op/s</sub> | -1.2% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.228 · p95 0.236 · p99 0.256 · 4972 op/s</sub> | 0.202<br><sub>context: p90 0.277 · p95 0.312 · p99 0.345 · 4580 op/s</sub> | +4.2% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.309<br><sub>context: p90 0.399 · p95 0.430 · p99 0.496 · 24747 op/s</sub> | 0.307<br><sub>context: p90 0.401 · p95 0.433 · p99 0.495 · 24628 op/s</sub> | -0.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.931<br><sub>context: p90 1.009 · p95 1.036 · p99 1.087 · 1060 op/s</sub> | 0.958<br><sub>context: p90 1.039 · p95 1.076 · p99 1.219 · 1023 op/s</sub> | +2.9% (+0.027) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.181<br><sub>context: p90 1.348 · p95 1.400 · p99 1.517 · 6663 op/s</sub> | 1.190<br><sub>context: p90 1.346 · p95 1.396 · p99 1.512 · 6605 op/s</sub> | +0.7% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.949<br><sub>context: p90 1.004 · p95 1.032 · p99 1.082 · 1042 op/s</sub> | 0.954<br><sub>context: p90 1.029 · p95 1.051 · p99 1.091 · 1035 op/s</sub> | +0.5% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.104<br><sub>context: p90 1.236 · p95 1.281 · p99 1.405 · 7200 op/s</sub> | 1.095<br><sub>context: p90 1.225 · p95 1.259 · p99 1.318 · 7232 op/s</sub> | -0.8% (-0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.228<br><sub>context: p90 0.268 · p95 0.280 · p99 0.313 · 4281 op/s</sub> | 0.251<br><sub>context: p90 0.347 · p95 0.361 · p99 0.410 · 3713 op/s</sub> | +10.0% (+0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.374<br><sub>context: p90 0.470 · p95 0.505 · p99 0.571 · 20608 op/s</sub> | 0.380<br><sub>context: p90 0.470 · p95 0.506 · p99 0.567 · 20575 op/s</sub> | +1.6% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.382<br><sub>context: p90 0.453 · p95 0.468 · p99 0.546 · 2566 op/s</sub> | 0.360<br><sub>context: p90 0.435 · p95 0.453 · p99 0.487 · 2728 op/s</sub> | -5.9% (-0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.464<br><sub>context: p90 0.581 · p95 0.614 · p99 0.696 · 16674 op/s</sub> | 0.458<br><sub>context: p90 0.570 · p95 0.608 · p99 0.673 · 16918 op/s</sub> | -1.3% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.220 · p95 0.230 · p99 0.244 · 5454 op/s</sub> | 0.182<br><sub>context: p90 0.229 · p95 0.249 · p99 0.286 · 5194 op/s</sub> | +2.5% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.314<br><sub>context: p90 0.407 · p95 0.436 · p99 0.486 · 24168 op/s</sub> | 0.317<br><sub>context: p90 0.407 · p95 0.438 · p99 0.500 · 24008 op/s</sub> | +1.2% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.173<br><sub>context: p90 0.221 · p95 0.245 · p99 0.301 · 5480 op/s</sub> | 0.169<br><sub>context: p90 0.205 · p95 0.213 · p99 0.281 · 5567 op/s</sub> | -2.5% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.295<br><sub>context: p90 0.391 · p95 0.429 · p99 0.505 · 25642 op/s</sub> | 0.297<br><sub>context: p90 0.395 · p95 0.425 · p99 0.503 · 25613 op/s</sub> | +0.4% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.177<br><sub>context: p90 0.223 · p95 0.240 · p99 0.256 · 5533 op/s</sub> | 0.177<br><sub>context: p90 0.210 · p95 0.217 · p99 0.256 · 5359 op/s</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.297<br><sub>context: p90 0.399 · p95 0.431 · p99 0.504 · 25619 op/s</sub> | 0.296<br><sub>context: p90 0.396 · p95 0.434 · p99 0.506 · 25405 op/s</sub> | -0.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.362<br><sub>context: p90 0.384 · p95 0.395 · p99 0.417 · 2701 op/s</sub> | 0.391<br><sub>context: p90 0.444 · p95 0.470 · p99 0.498 · 2498 op/s</sub> | +8.0% (+0.029) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.518 · p95 0.542 · p99 0.598 · 17588 op/s</sub> | 0.440<br><sub>context: p90 0.510 · p95 0.529 · p99 0.571 · 17760 op/s</sub> | -1.3% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>
