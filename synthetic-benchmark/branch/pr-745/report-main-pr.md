### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 | ghcr.io/falkordb/falkordb-server@sha256:43f50272e0d144075251b10bcc8f77b24e371e5f6c7244c4b1848ab0fbe1937c |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58 → ghcr.io/falkordb/falkordb-server@sha256:43f50272e0d144075251b10bcc8f77b24e371e5f6c7244c4b1848ab0fbe1937c

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.778<br><sub>context: p90 0.819 · p95 0.835 · p99 0.872 · 1270 op/s</sub> | 1.178<br><sub>context: p90 1.343 · p95 1.402 · p99 1.451 · 858 op/s</sub> | +51.4% (+0.400) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.927<br><sub>context: p90 1.058 · p95 1.095 · p99 1.163 · 8532 op/s</sub> | 0.943<br><sub>context: p90 1.079 · p95 1.116 · p99 1.203 · 8256 op/s</sub> | +1.7% (+0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.874<br><sub>context: p90 0.938 · p95 0.949 · p99 0.991 · 1128 op/s</sub> | 0.865<br><sub>context: p90 0.902 · p95 0.919 · p99 0.972 · 1145 op/s</sub> | -1.0% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.080<br><sub>context: p90 1.288 · p95 1.320 · p99 1.412 · 7296 op/s</sub> | 1.093<br><sub>context: p90 1.302 · p95 1.342 · p99 1.418 · 7192 op/s</sub> | +1.2% (+0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.322<br><sub>context: p90 1.394 · p95 1.409 · p99 1.436 · 748 op/s</sub> | 1.317<br><sub>context: p90 1.372 · p95 1.396 · p99 1.429 · 751 op/s</sub> | -0.4% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.796<br><sub>context: p90 2.359 · p95 2.531 · p99 2.836 · 4283 op/s</sub> | 1.770<br><sub>context: p90 2.343 · p95 2.541 · p99 2.882 · 4301 op/s</sub> | -1.4% (-0.026) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.366<br><sub>context: p90 1.409 · p95 1.429 · p99 1.452 · 725 op/s</sub> | 1.387<br><sub>context: p90 1.431 · p95 1.457 · p99 1.503 · 717 op/s</sub> | +1.5% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.625<br><sub>context: p90 1.948 · p95 1.990 · p99 2.055 · 4773 op/s</sub> | 1.638<br><sub>context: p90 1.970 · p95 2.015 · p99 2.111 · 4726 op/s</sub> | +0.8% (+0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.117<br><sub>context: p90 0.140 · p95 0.148 · p99 0.163 · 8144 op/s</sub> | 0.149<br><sub>context: p90 0.168 · p95 0.173 · p99 0.185 · 6380 op/s</sub> | +26.6% (+0.031) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.212<br><sub>context: p90 0.273 · p95 0.292 · p99 0.322 · 36108 op/s</sub> | 0.219<br><sub>context: p90 0.285 · p95 0.314 · p99 0.365 · 35013 op/s</sub> | +3.0% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.287 · p95 0.316 · p99 0.355 · 4234 op/s</sub> | 0.411<br><sub>context: p90 0.552 · p95 0.585 · p99 0.615 · 2330 op/s</sub> | +81.4% (+0.184) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.332<br><sub>context: p90 0.426 · p95 0.457 · p99 0.516 · 22912 op/s</sub> | 0.338<br><sub>context: p90 0.433 · p95 0.458 · p99 0.528 · 22404 op/s</sub> | +1.8% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.239<br><sub>context: p90 0.282 · p95 0.304 · p99 0.329 · 4099 op/s</sub> | 0.241<br><sub>context: p90 0.321 · p95 0.344 · p99 0.397 · 3886 op/s</sub> | +0.9% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.365<br><sub>context: p90 0.453 · p95 0.488 · p99 0.552 · 21197 op/s</sub> | 0.354<br><sub>context: p90 0.441 · p95 0.473 · p99 0.540 · 21704 op/s</sub> | -3.2% (-0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.421 · p95 0.449 · p99 0.497 · 2661 op/s</sub> | 0.384<br><sub>context: p90 0.462 · p95 0.486 · p99 0.536 · 2564 op/s</sub> | +3.7% (+0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.452<br><sub>context: p90 0.545 · p95 0.574 · p99 0.622 · 17282 op/s</sub> | 0.460<br><sub>context: p90 0.562 · p95 0.595 · p99 0.659 · 17003 op/s</sub> | +1.7% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.374<br><sub>context: p90 0.449 · p95 0.465 · p99 0.524 · 2595 op/s</sub> | 0.374<br><sub>context: p90 0.446 · p95 0.467 · p99 0.507 · 2604 op/s</sub> | +0.0% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.484<br><sub>context: p90 0.584 · p95 0.615 · p99 0.690 · 15965 op/s</sub> | 0.490<br><sub>context: p90 0.599 · p95 0.640 · p99 0.710 · 15676 op/s</sub> | +1.1% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.512<br><sub>context: p90 0.630 · p95 0.659 · p99 0.756 · 1913 op/s</sub> | 0.466<br><sub>context: p90 0.572 · p95 0.616 · p99 0.712 · 2076 op/s</sub> | -9.0% (-0.046) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.655<br><sub>context: p90 0.847 · p95 0.898 · p99 1.040 · 11780 op/s</sub> | 0.656<br><sub>context: p90 0.837 · p95 0.891 · p99 1.037 · 11689 op/s</sub> | +0.2% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.486<br><sub>context: p90 0.590 · p95 0.625 · p99 0.700 · 2013 op/s</sub> | 0.489<br><sub>context: p90 0.599 · p95 0.636 · p99 0.684 · 1990 op/s</sub> | +0.6% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.667<br><sub>context: p90 0.840 · p95 0.894 · p99 0.981 · 11564 op/s</sub> | 0.680<br><sub>context: p90 0.870 · p95 0.930 · p99 1.061 · 11337 op/s</sub> | +2.0% (+0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.024<br><sub>context: p90 1.389 · p95 1.502 · p99 1.699 · 961 op/s</sub> | 1.005<br><sub>context: p90 1.322 · p95 1.506 · p99 1.729 · 968 op/s</sub> | -1.9% (-0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.279<br><sub>context: p90 1.763 · p95 1.901 · p99 2.275 · 6030 op/s</sub> | 1.269<br><sub>context: p90 1.792 · p95 1.930 · p99 2.266 · 6030 op/s</sub> | -0.8% (-0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.054<br><sub>context: p90 1.371 · p95 1.607 · p99 1.735 · 928 op/s</sub> | 1.162<br><sub>context: p90 1.546 · p95 1.709 · p99 1.881 · 829 op/s</sub> | +10.3% (+0.108) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.261<br><sub>context: p90 1.753 · p95 1.950 · p99 2.242 · 5985 op/s</sub> | 1.421<br><sub>context: p90 1.941 · p95 2.135 · p99 2.483 · 5389 op/s</sub> | +12.7% (+0.160) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.611<br><sub>context: p90 0.795 · p95 0.850 · p99 0.958 · 1570 op/s</sub> | 0.576<br><sub>context: p90 0.759 · p95 0.800 · p99 0.984 · 1690 op/s</sub> | -5.6% (-0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.764<br><sub>context: p90 1.006 · p95 1.073 · p99 1.197 · 10166 op/s</sub> | 0.745<br><sub>context: p90 0.984 · p95 1.055 · p99 1.172 · 10559 op/s</sub> | -2.5% (-0.019) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.357<br><sub>context: p90 0.440 · p95 0.481 · p99 0.552 · 2604 op/s</sub> | 0.340<br><sub>context: p90 0.426 · p95 0.444 · p99 0.480 · 2883 op/s</sub> | -4.8% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.415<br><sub>context: p90 0.515 · p95 0.549 · p99 0.610 · 18661 op/s</sub> | 0.401<br><sub>context: p90 0.502 · p95 0.534 · p99 0.593 · 18905 op/s</sub> | -3.5% (-0.014) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.152<br><sub>context: p90 0.191 · p95 0.213 · p99 0.261 · 6290 op/s</sub> | 0.142<br><sub>context: p90 0.175 · p95 0.181 · p99 0.204 · 6621 op/s</sub> | -7.0% (-0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.224<br><sub>context: p90 0.291 · p95 0.314 · p99 0.358 · 33979 op/s</sub> | 0.223<br><sub>context: p90 0.287 · p95 0.307 · p99 0.341 · 34323 op/s</sub> | -0.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.129<br><sub>context: p90 0.154 · p95 0.164 · p99 0.178 · 7265 op/s</sub> | 0.121<br><sub>context: p90 0.131 · p95 0.135 · p99 0.143 · 7984 op/s</sub> | -6.4% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.210<br><sub>context: p90 0.290 · p95 0.314 · p99 0.359 · 36240 op/s</sub> | 0.203<br><sub>context: p90 0.273 · p95 0.296 · p99 0.346 · 38088 op/s</sub> | -3.4% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.345<br><sub>context: p90 0.397 · p95 0.421 · p99 0.488 · 2850 op/s</sub> | 0.304<br><sub>context: p90 0.359 · p95 0.373 · p99 0.399 · 3172 op/s</sub> | -11.8% (-0.041) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.443<br><sub>context: p90 0.541 · p95 0.573 · p99 0.630 · 17506 op/s</sub> | 0.435<br><sub>context: p90 0.535 · p95 0.560 · p99 0.635 · 17626 op/s</sub> | -1.8% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.280<br><sub>context: p90 1.872 · p95 2.116 · p99 2.388 · 753 op/s</sub> | 1.269<br><sub>context: p90 1.865 · p95 2.116 · p99 2.336 · 765 op/s</sub> | -0.8% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.145<br><sub>context: p90 3.357 · p95 3.793 · p99 4.326 · 3568 op/s</sub> | 2.083<br><sub>context: p90 3.349 · p95 3.687 · p99 4.209 · 3623 op/s</sub> | -2.9% (-0.062) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.443<br><sub>context: p90 6.749 · p95 7.483 · p99 7.966 · 216 op/s</sub> | 4.361<br><sub>context: p90 6.622 · p95 7.350 · p99 7.860 · 221 op/s</sub> | -1.8% (-0.082) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.131<br><sub>context: p90 12.386 · p95 13.644 · p99 15.107 · 955 op/s</sub> | 8.207<br><sub>context: p90 12.790 · p95 14.211 · p99 15.554 · 940 op/s</sub> | +0.9% (+0.077) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.162<br><sub>context: p90 0.184 · p95 0.188 · p99 0.207 · 5941 op/s</sub> | 0.170<br><sub>context: p90 0.202 · p95 0.208 · p99 0.237 · 5534 op/s</sub> | +4.8% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.390 · p95 0.422 · p99 0.516 · 25345 op/s</sub> | 0.293<br><sub>context: p90 0.394 · p95 0.417 · p99 0.497 · 25746 op/s</sub> | -1.2% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.167<br><sub>context: p90 0.206 · p95 0.210 · p99 0.227 · 5407 op/s</sub> | 0.171<br><sub>context: p90 0.194 · p95 0.202 · p99 0.224 · 5581 op/s</sub> | +2.3% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.403 · p95 0.440 · p99 0.507 · 25240 op/s</sub> | 0.298<br><sub>context: p90 0.390 · p95 0.428 · p99 0.533 · 25505 op/s</sub> | -0.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.282 · p95 0.290 · p99 0.343 · 3965 op/s</sub> | 0.274<br><sub>context: p90 0.320 · p95 0.334 · p99 0.372 · 3526 op/s</sub> | +14.2% (+0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.381<br><sub>context: p90 0.495 · p95 0.536 · p99 0.618 · 20065 op/s</sub> | 0.356<br><sub>context: p90 0.461 · p95 0.497 · p99 0.563 · 21259 op/s</sub> | -6.6% (-0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.154<br><sub>context: p90 0.170 · p95 0.177 · p99 0.191 · 6430 op/s</sub> | 0.139<br><sub>context: p90 0.171 · p95 0.194 · p99 0.225 · 6698 op/s</sub> | -9.4% (-0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.222<br><sub>context: p90 0.286 · p95 0.311 · p99 0.357 · 34541 op/s</sub> | 0.219<br><sub>context: p90 0.285 · p95 0.304 · p99 0.349 · 34915 op/s</sub> | -1.4% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.342<br><sub>context: p90 0.403 · p95 0.420 · p99 0.492 · 2868 op/s</sub> | 0.324<br><sub>context: p90 0.375 · p95 0.394 · p99 0.449 · 3038 op/s</sub> | -5.0% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.392<br><sub>context: p90 0.479 · p95 0.514 · p99 0.569 · 19769 op/s</sub> | 0.394<br><sub>context: p90 0.483 · p95 0.512 · p99 0.566 · 19614 op/s</sub> | +0.6% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.317 · p95 0.346 · p99 0.404 · 4025 op/s</sub> | 0.258<br><sub>context: p90 0.380 · p95 0.396 · p99 0.440 · 3621 op/s</sub> | +8.8% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.331<br><sub>context: p90 0.418 · p95 0.449 · p99 0.512 · 23171 op/s</sub> | 0.325<br><sub>context: p90 0.409 · p95 0.439 · p99 0.499 · 23599 op/s</sub> | -1.8% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.321<br><sub>context: p90 0.387 · p95 0.403 · p99 0.442 · 3044 op/s</sub> | 0.362<br><sub>context: p90 0.453 · p95 0.472 · p99 0.528 · 2638 op/s</sub> | +12.5% (+0.040) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.534 · p95 0.563 · p99 0.628 · 17735 op/s</sub> | 0.456<br><sub>context: p90 0.562 · p95 0.595 · p99 0.670 · 16891 op/s</sub> | +4.2% (+0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.438<br><sub>context: p90 0.558 · p95 0.588 · p99 0.667 · 2223 op/s</sub> | 0.731<br><sub>context: p90 0.862 · p95 0.897 · p99 1.012 · 1370 op/s</sub> | +67.0% (+0.293) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.597<br><sub>context: p90 0.780 · p95 0.841 · p99 0.966 · 12809 op/s</sub> | 0.619<br><sub>context: p90 0.801 · p95 0.874 · p99 0.977 · 12319 op/s</sub> | +3.7% (+0.022) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.472<br><sub>context: p90 0.569 · p95 0.587 · p99 0.709 · 2073 op/s</sub> | 0.502<br><sub>context: p90 0.603 · p95 0.633 · p99 0.690 · 1980 op/s</sub> | +6.4% (+0.030) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.616<br><sub>context: p90 0.780 · p95 0.832 · p99 0.926 · 12439 op/s</sub> | 0.623<br><sub>context: p90 0.783 · p95 0.843 · p99 0.956 · 12416 op/s</sub> | +1.0% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.357<br><sub>context: p90 0.411 · p95 0.423 · p99 0.463 · 2765 op/s</sub> | 0.452<br><sub>context: p90 0.567 · p95 0.621 · p99 0.716 · 2166 op/s</sub> | +26.5% (+0.095) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.467<br><sub>context: p90 0.569 · p95 0.598 · p99 0.679 · 16681 op/s</sub> | 0.478<br><sub>context: p90 0.597 · p95 0.631 · p99 0.702 · 16295 op/s</sub> | +2.4% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.397 · p95 0.419 · p99 0.478 · 2997 op/s</sub> | 0.282<br><sub>context: p90 0.340 · p95 0.364 · p99 0.400 · 3431 op/s</sub> | -14.7% (-0.049) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.398<br><sub>context: p90 0.498 · p95 0.525 · p99 0.600 · 19102 op/s</sub> | 0.380<br><sub>context: p90 0.488 · p95 0.517 · p99 0.585 · 20013 op/s</sub> | -4.5% (-0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.265<br><sub>context: p90 15.488 · p95 15.539 · p99 15.707 · 65 op/s</sub> | 15.190<br><sub>context: p90 15.563 · p95 15.709 · p99 15.996 · 66 op/s</sub> | -0.5% (-0.075) | 10% AND 0.5 ms | 🟢 |
| 8 | 18.764<br><sub>context: p90 24.662 · p95 27.545 · p99 31.341 · 382 op/s</sub> | 18.528<br><sub>context: p90 23.607 · p95 25.966 · p99 30.844 · 390 op/s</sub> | -1.3% (-0.235) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.365<br><sub>context: p90 0.451 · p95 0.508 · p99 0.558 · 2612 op/s</sub> | 0.370<br><sub>context: p90 0.429 · p95 0.453 · p99 0.489 · 2653 op/s</sub> | +1.3% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.534 · p95 0.572 · p99 0.625 · 17491 op/s</sub> | 0.465<br><sub>context: p90 0.558 · p95 0.593 · p99 0.658 · 16704 op/s</sub> | +4.4% (+0.020) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.250<br><sub>context: p90 1.639 · p95 1.747 · p99 2.028 · 796 op/s</sub> | 1.266<br><sub>context: p90 1.731 · p95 1.859 · p99 2.005 · 777 op/s</sub> | +1.3% (+0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.555<br><sub>context: p90 2.192 · p95 2.407 · p99 2.821 · 4937 op/s</sub> | 1.590<br><sub>context: p90 2.215 · p95 2.378 · p99 2.915 · 4903 op/s</sub> | +2.3% (+0.035) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.351<br><sub>context: p90 0.405 · p95 0.429 · p99 0.459 · 2820 op/s</sub> | 0.404<br><sub>context: p90 0.502 · p95 0.533 · p99 0.607 · 2384 op/s</sub> | +15.1% (+0.053) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.474<br><sub>context: p90 0.580 · p95 0.616 · p99 0.680 · 16356 op/s</sub> | 0.478<br><sub>context: p90 0.595 · p95 0.634 · p99 0.697 · 16228 op/s</sub> | +0.9% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.336<br><sub>context: p90 0.408 · p95 0.427 · p99 0.450 · 2915 op/s</sub> | 0.346<br><sub>context: p90 0.433 · p95 0.458 · p99 0.507 · 2812 op/s</sub> | +2.9% (+0.010) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.423<br><sub>context: p90 0.538 · p95 0.569 · p99 0.664 · 18065 op/s</sub> | 0.416<br><sub>context: p90 0.520 · p95 0.557 · p99 0.657 · 18274 op/s</sub> | -1.7% (-0.007) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.350<br><sub>context: p90 0.421 · p95 0.446 · p99 0.507 · 2779 op/s</sub> | 0.336<br><sub>context: p90 0.401 · p95 0.414 · p99 0.442 · 2905 op/s</sub> | -3.9% (-0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.438<br><sub>context: p90 0.546 · p95 0.578 · p99 0.647 · 17424 op/s</sub> | 0.472<br><sub>context: p90 0.611 · p95 0.659 · p99 0.745 · 16059 op/s</sub> | +7.7% (+0.034) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.175<br><sub>context: p90 0.212 · p95 0.223 · p99 0.287 · 5452 op/s</sub> | 0.408<br><sub>context: p90 0.618 · p95 0.680 · p99 0.734 · 2303 op/s</sub> | +133.0% (+0.233) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.304<br><sub>context: p90 0.404 · p95 0.437 · p99 0.512 · 24883 op/s</sub> | 0.484<br><sub>context: p90 0.902 · p95 1.057 · p99 1.361 · 13080 op/s</sub> | +59.5% (+0.181) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.166<br><sub>context: p90 0.189 · p95 0.194 · p99 0.200 · 5830 op/s</sub> | 0.145<br><sub>context: p90 0.177 · p95 0.182 · p99 0.194 · 6582 op/s</sub> | -12.6% (-0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.238<br><sub>context: p90 0.310 · p95 0.338 · p99 0.390 · 31971 op/s</sub> | 0.242<br><sub>context: p90 0.312 · p95 0.333 · p99 0.371 · 31763 op/s</sub> | +1.6% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.271<br><sub>context: p90 0.329 · p95 0.351 · p99 0.394 · 3583 op/s</sub> | 0.247<br><sub>context: p90 0.306 · p95 0.337 · p99 0.363 · 3951 op/s</sub> | -8.7% (-0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.378<br><sub>context: p90 0.493 · p95 0.532 · p99 0.625 · 20242 op/s</sub> | 0.368<br><sub>context: p90 0.477 · p95 0.518 · p99 0.591 · 20888 op/s</sub> | -2.6% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.278<br><sub>context: p90 0.359 · p95 0.376 · p99 0.418 · 3447 op/s</sub> | 0.252<br><sub>context: p90 0.338 · p95 0.352 · p99 0.386 · 3738 op/s</sub> | -9.3% (-0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.387<br><sub>context: p90 0.501 · p95 0.542 · p99 0.647 · 19723 op/s</sub> | 0.374<br><sub>context: p90 0.487 · p95 0.516 · p99 0.571 · 20286 op/s</sub> | -3.3% (-0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.209<br><sub>context: p90 0.263 · p95 0.292 · p99 0.351 · 4604 op/s</sub> | 0.203<br><sub>context: p90 0.245 · p95 0.258 · p99 0.293 · 4738 op/s</sub> | -2.9% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.322<br><sub>context: p90 0.419 · p95 0.459 · p99 0.546 · 23341 op/s</sub> | 0.313<br><sub>context: p90 0.402 · p95 0.435 · p99 0.496 · 24331 op/s</sub> | -2.9% (-0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.998<br><sub>context: p90 1.098 · p95 1.127 · p99 1.206 · 992 op/s</sub> | 0.970<br><sub>context: p90 1.026 · p95 1.044 · p99 1.083 · 1031 op/s</sub> | -2.8% (-0.028) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.191<br><sub>context: p90 1.353 · p95 1.411 · p99 1.520 · 6625 op/s</sub> | 1.188<br><sub>context: p90 1.358 · p95 1.411 · p99 1.532 · 6558 op/s</sub> | -0.3% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.994<br><sub>context: p90 1.084 · p95 1.108 · p99 1.154 · 994 op/s</sub> | 0.960<br><sub>context: p90 1.015 · p95 1.033 · p99 1.071 · 1032 op/s</sub> | -3.4% (-0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.106<br><sub>context: p90 1.250 · p95 1.291 · p99 1.425 · 7114 op/s</sub> | 1.110<br><sub>context: p90 1.248 · p95 1.292 · p99 1.414 · 7116 op/s</sub> | +0.3% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.268<br><sub>context: p90 0.345 · p95 0.371 · p99 0.404 · 3596 op/s</sub> | 0.278<br><sub>context: p90 0.361 · p95 0.383 · p99 0.432 · 3364 op/s</sub> | +3.7% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.393<br><sub>context: p90 0.502 · p95 0.529 · p99 0.607 · 19599 op/s</sub> | 0.377<br><sub>context: p90 0.469 · p95 0.502 · p99 0.562 · 20231 op/s</sub> | -4.2% (-0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.384<br><sub>context: p90 0.466 · p95 0.490 · p99 0.546 · 2516 op/s</sub> | 0.367<br><sub>context: p90 0.442 · p95 0.483 · p99 0.526 · 2648 op/s</sub> | -4.6% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.490<br><sub>context: p90 0.603 · p95 0.639 · p99 0.716 · 15969 op/s</sub> | 0.467<br><sub>context: p90 0.584 · p95 0.617 · p99 0.697 · 16621 op/s</sub> | -4.6% (-0.023) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.174<br><sub>context: p90 0.210 · p95 0.216 · p99 0.225 · 5491 op/s</sub> | 0.192<br><sub>context: p90 0.218 · p95 0.221 · p99 0.224 · 5158 op/s</sub> | +10.2% (+0.018) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.319<br><sub>context: p90 0.414 · p95 0.449 · p99 0.513 · 23792 op/s</sub> | 0.314<br><sub>context: p90 0.405 · p95 0.434 · p99 0.489 · 24165 op/s</sub> | -1.4% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.219 · p95 0.239 · p99 0.281 · 5289 op/s</sub> | 0.201<br><sub>context: p90 0.245 · p95 0.263 · p99 0.310 · 4771 op/s</sub> | +11.6% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.395 · p95 0.435 · p99 0.505 · 25235 op/s</sub> | 0.301<br><sub>context: p90 0.399 · p95 0.431 · p99 0.499 · 24989 op/s</sub> | +0.5% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.171<br><sub>context: p90 0.213 · p95 0.240 · p99 0.272 · 5532 op/s</sub> | 0.178<br><sub>context: p90 0.208 · p95 0.221 · p99 0.238 · 5349 op/s</sub> | +4.1% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.302<br><sub>context: p90 0.415 · p95 0.444 · p99 0.528 · 24758 op/s</sub> | 0.304<br><sub>context: p90 0.407 · p95 0.442 · p99 0.520 · 24966 op/s</sub> | +0.5% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.373<br><sub>context: p90 0.417 · p95 0.426 · p99 0.450 · 2615 op/s</sub> | 0.387<br><sub>context: p90 0.432 · p95 0.446 · p99 0.460 · 2524 op/s</sub> | +3.8% (+0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.450<br><sub>context: p90 0.522 · p95 0.543 · p99 0.575 · 17390 op/s</sub> | 0.448<br><sub>context: p90 0.521 · p95 0.547 · p99 0.590 · 17506 op/s</sub> | -0.5% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>
