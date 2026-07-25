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

**main vs c-engine** — ⚠ pass, 1 diverged — no p50 regression beyond budget across 98 comparable cell(s); divergence is advisory under this policy

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:b8aef0c5253b1681187193435212df87f78ff52ccd009cfa75dfd3bfab57bd58

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.429<br><sub>context: p90 1.531 · p95 1.577 · p99 1.648 · 698 op/s</sub> | 0.778<br><sub>context: p90 0.819 · p95 0.835 · p99 0.872 · 1270 op/s</sub> | -45.5% (-0.651) | 150% AND 2 ms | 🟢 |
| 8 | 1.898<br><sub>context: p90 2.498 · p95 2.728 · p99 3.118 · 4066 op/s</sub> | 0.927<br><sub>context: p90 1.058 · p95 1.095 · p99 1.163 · 8532 op/s</sub> | -51.1% (-0.971) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.069<br><sub>context: p90 2.160 · p95 2.186 · p99 2.247 · 479 op/s</sub> | 0.874<br><sub>context: p90 0.938 · p95 0.949 · p99 0.991 · 1128 op/s</sub> | -57.8% (-1.195) | 150% AND 2 ms | 🟢 |
| 8 | 2.649<br><sub>context: p90 3.426 · p95 3.513 · p99 3.839 · 2888 op/s</sub> | 1.080<br><sub>context: p90 1.288 · p95 1.320 · p99 1.412 · 7296 op/s</sub> | -59.2% (-1.569) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.020<br><sub>context: p90 2.105 · p95 2.132 · p99 2.197 · 493 op/s</sub> | 1.322<br><sub>context: p90 1.394 · p95 1.409 · p99 1.436 · 748 op/s</sub> | -34.5% (-0.698) | 150% AND 2 ms | 🟢 |
| 8 | 2.789<br><sub>context: p90 3.569 · p95 3.772 · p99 4.179 · 2785 op/s</sub> | 1.796<br><sub>context: p90 2.359 · p95 2.531 · p99 2.836 · 4283 op/s</sub> | -35.6% (-0.993) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.607<br><sub>context: p90 2.686 · p95 2.718 · p99 2.804 · 382 op/s</sub> | 1.366<br><sub>context: p90 1.409 · p95 1.429 · p99 1.452 · 725 op/s</sub> | -47.6% (-1.241) | 150% AND 2 ms | 🟢 |
| 8 | 3.627<br><sub>context: p90 4.690 · p95 4.954 · p99 5.557 · 2122 op/s</sub> | 1.625<br><sub>context: p90 1.948 · p95 1.990 · p99 2.055 · 4773 op/s</sub> | -55.2% (-2.002) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.159<br><sub>context: p90 0.189 · p95 0.194 · p99 0.241 · 5861 op/s</sub> | 0.117<br><sub>context: p90 0.140 · p95 0.148 · p99 0.163 · 8144 op/s</sub> | -26.4% (-0.042) | 150% AND 2 ms | 🟢 |
| 8 | 0.238<br><sub>context: p90 0.296 · p95 0.316 · p99 0.364 · 31928 op/s</sub> | 0.212<br><sub>context: p90 0.273 · p95 0.292 · p99 0.322 · 36108 op/s</sub> | -10.8% (-0.026) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.341<br><sub>context: p90 0.406 · p95 0.435 · p99 0.456 · 2846 op/s</sub> | 0.226<br><sub>context: p90 0.287 · p95 0.316 · p99 0.355 · 4234 op/s</sub> | -33.5% (-0.114) | 150% AND 2 ms | 🟢 |
| 8 | 0.441<br><sub>context: p90 0.530 · p95 0.561 · p99 0.620 · 17636 op/s</sub> | 0.332<br><sub>context: p90 0.426 · p95 0.457 · p99 0.516 · 22912 op/s</sub> | -24.7% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.364<br><sub>context: p90 0.429 · p95 0.454 · p99 0.498 · 2664 op/s</sub> | 0.239<br><sub>context: p90 0.282 · p95 0.304 · p99 0.329 · 4099 op/s</sub> | -34.3% (-0.125) | 150% AND 2 ms | 🟢 |
| 8 | 0.467<br><sub>context: p90 0.552 · p95 0.580 · p99 0.626 · 16751 op/s</sub> | 0.365<br><sub>context: p90 0.453 · p95 0.488 · p99 0.552 · 21197 op/s</sub> | -21.8% (-0.102) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.410<br><sub>context: p90 0.491 · p95 0.521 · p99 0.566 · 2385 op/s</sub> | 0.370<br><sub>context: p90 0.421 · p95 0.449 · p99 0.497 · 2661 op/s</sub> | -9.7% (-0.040) | 150% AND 2 ms | 🟢 |
| 8 | 0.529<br><sub>context: p90 0.618 · p95 0.648 · p99 0.713 · 14843 op/s</sub> | 0.452<br><sub>context: p90 0.545 · p95 0.574 · p99 0.622 · 17282 op/s</sub> | -14.5% (-0.077) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.490 · p95 0.509 · p99 0.558 · 2337 op/s</sub> | 0.374<br><sub>context: p90 0.449 · p95 0.465 · p99 0.524 · 2595 op/s</sub> | -10.7% (-0.045) | 150% AND 2 ms | 🟢 |
| 8 | 0.564<br><sub>context: p90 0.661 · p95 0.694 · p99 0.762 · 13920 op/s</sub> | 0.484<br><sub>context: p90 0.584 · p95 0.615 · p99 0.690 · 15965 op/s</sub> | -14.1% (-0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.546<br><sub>context: p90 0.665 · p95 0.697 · p99 0.771 · 1800 op/s</sub> | 0.512<br><sub>context: p90 0.630 · p95 0.659 · p99 0.756 · 1913 op/s</sub> | -6.3% (-0.034) | 150% AND 2 ms | 🟢 |
| 8 | 0.740<br><sub>context: p90 0.931 · p95 1.003 · p99 1.118 · 10468 op/s</sub> | 0.655<br><sub>context: p90 0.847 · p95 0.898 · p99 1.040 · 11780 op/s</sub> | -11.4% (-0.084) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.573<br><sub>context: p90 0.722 · p95 0.753 · p99 0.825 · 1713 op/s</sub> | 0.486<br><sub>context: p90 0.590 · p95 0.625 · p99 0.700 · 2013 op/s</sub> | -15.2% (-0.087) | 150% AND 2 ms | 🟢 |
| 8 | 0.778<br><sub>context: p90 0.983 · p95 1.056 · p99 1.205 · 9945 op/s</sub> | 0.667<br><sub>context: p90 0.840 · p95 0.894 · p99 0.981 · 11564 op/s</sub> | -14.3% (-0.111) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.201<br><sub>context: p90 1.639 · p95 1.777 · p99 2.009 · 813 op/s</sub> | 1.024<br><sub>context: p90 1.389 · p95 1.502 · p99 1.699 · 961 op/s</sub> | -14.7% (-0.177) | 150% AND 2 ms | 🟢 |
| 8 | 1.560<br><sub>context: p90 2.157 · p95 2.356 · p99 2.753 · 4987 op/s</sub> | 1.279<br><sub>context: p90 1.763 · p95 1.901 · p99 2.275 · 6030 op/s</sub> | -18.0% (-0.281) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.203<br><sub>context: p90 1.724 · p95 1.975 · p99 2.163 · 780 op/s</sub> | 1.054<br><sub>context: p90 1.371 · p95 1.607 · p99 1.735 · 928 op/s</sub> | -12.4% (-0.149) | 150% AND 2 ms | 🟢 |
| 8 | 1.630<br><sub>context: p90 2.313 · p95 2.544 · p99 2.850 · 4667 op/s</sub> | 1.261<br><sub>context: p90 1.753 · p95 1.950 · p99 2.242 · 5985 op/s</sub> | -22.6% (-0.368) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.601<br><sub>context: p90 0.682 · p95 0.715 · p99 0.792 · 1646 op/s</sub> | 0.611<br><sub>context: p90 0.795 · p95 0.850 · p99 0.958 · 1570 op/s</sub> | +1.7% (+0.010) | 150% AND 2 ms | 🟢 |
| 8 | 0.774<br><sub>context: p90 0.897 · p95 0.936 · p99 1.020 · 10221 op/s</sub> | 0.764<br><sub>context: p90 1.006 · p95 1.073 · p99 1.197 · 10166 op/s</sub> | -1.3% (-0.010) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.401<br><sub>context: p90 0.494 · p95 0.516 · p99 0.540 · 2414 op/s</sub> | 0.357<br><sub>context: p90 0.440 · p95 0.481 · p99 0.552 · 2604 op/s</sub> | -11.0% (-0.044) | 150% AND 2 ms | 🟢 |
| 8 | 0.572<br><sub>context: p90 0.678 · p95 0.714 · p99 0.811 · 13637 op/s</sub> | 0.415<br><sub>context: p90 0.515 · p95 0.549 · p99 0.610 · 18661 op/s</sub> | -27.4% (-0.157) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.297 · p95 0.322 · p99 0.349 · 4099 op/s</sub> | 0.152<br><sub>context: p90 0.191 · p95 0.213 · p99 0.261 · 6290 op/s</sub> | -35.9% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.328<br><sub>context: p90 0.410 · p95 0.434 · p99 0.497 · 23429 op/s</sub> | 0.224<br><sub>context: p90 0.291 · p95 0.314 · p99 0.358 · 33979 op/s</sub> | -31.7% (-0.104) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.181<br><sub>context: p90 0.211 · p95 0.222 · p99 0.274 · 5250 op/s</sub> | 0.129<br><sub>context: p90 0.154 · p95 0.164 · p99 0.178 · 7265 op/s</sub> | -28.7% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.263<br><sub>context: p90 0.325 · p95 0.346 · p99 0.390 · 28990 op/s</sub> | 0.210<br><sub>context: p90 0.290 · p95 0.314 · p99 0.359 · 36240 op/s</sub> | -20.3% (-0.053) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.516<br><sub>context: p90 0.595 · p95 0.626 · p99 0.671 · 1896 op/s</sub> | 0.345<br><sub>context: p90 0.397 · p95 0.421 · p99 0.488 · 2850 op/s</sub> | -33.1% (-0.171) | 150% AND 2 ms | 🟢 |
| 8 | 0.664<br><sub>context: p90 0.780 · p95 0.809 · p99 0.889 · 11790 op/s</sub> | 0.443<br><sub>context: p90 0.541 · p95 0.573 · p99 0.630 · 17506 op/s</sub> | -33.3% (-0.221) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.984<br><sub>context: p90 1.414 · p95 1.554 · p99 1.743 · 979 op/s</sub> | 1.280<br><sub>context: p90 1.872 · p95 2.116 · p99 2.388 · 753 op/s</sub> | +30.1% (+0.296) | 150% AND 2 ms | 🟢 |
| 8 | 1.339<br><sub>context: p90 2.034 · p95 2.275 · p99 2.705 · 5612 op/s</sub> | 2.145<br><sub>context: p90 3.357 · p95 3.793 · p99 4.326 · 3568 op/s</sub> | +60.2% (+0.806) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.407<br><sub>context: p90 5.391 · p95 6.148 · p99 7.298 · 280 op/s</sub> | 4.443<br><sub>context: p90 6.749 · p95 7.483 · p99 7.966 · 216 op/s</sub> | +30.4% (+1.036) | 150% AND 2 ms | 🟢 |
| 8 | 4.429<br><sub>context: p90 7.694 · p95 8.979 · p99 11.012 · 1624 op/s</sub> | 8.131<br><sub>context: p90 12.386 · p95 13.644 · p99 15.107 · 955 op/s</sub> | +83.6% (+3.702) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.265<br><sub>context: p90 0.317 · p95 0.329 · p99 0.376 · 3669 op/s</sub> | 0.162<br><sub>context: p90 0.184 · p95 0.188 · p99 0.207 · 5941 op/s</sub> | -38.7% (-0.102) | 150% AND 2 ms | 🟢 |
| 8 | 0.365<br><sub>context: p90 0.447 · p95 0.473 · p99 0.528 · 21316 op/s</sub> | 0.296<br><sub>context: p90 0.390 · p95 0.422 · p99 0.516 · 25345 op/s</sub> | -18.8% (-0.069) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.245<br><sub>context: p90 0.278 · p95 0.287 · p99 0.311 · 3981 op/s</sub> | 0.167<br><sub>context: p90 0.206 · p95 0.210 · p99 0.227 · 5407 op/s</sub> | -31.8% (-0.078) | 150% AND 2 ms | 🟢 |
| 8 | 0.360<br><sub>context: p90 0.439 · p95 0.463 · p99 0.517 · 21334 op/s</sub> | 0.300<br><sub>context: p90 0.403 · p95 0.440 · p99 0.507 · 25240 op/s</sub> | -16.8% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.358<br><sub>context: p90 0.425 · p95 0.437 · p99 0.476 · 2733 op/s</sub> | 0.240<br><sub>context: p90 0.282 · p95 0.290 · p99 0.343 · 3965 op/s</sub> | -32.8% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.505<br><sub>context: p90 0.620 · p95 0.664 · p99 0.751 · 15262 op/s</sub> | 0.381<br><sub>context: p90 0.495 · p95 0.536 · p99 0.618 · 20065 op/s</sub> | -24.6% (-0.124) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.209<br><sub>context: p90 0.263 · p95 0.311 · p99 0.329 · 4557 op/s</sub> | 0.154<br><sub>context: p90 0.170 · p95 0.177 · p99 0.191 · 6430 op/s</sub> | -26.5% (-0.055) | 150% AND 2 ms | 🟢 |
| 8 | 0.304<br><sub>context: p90 0.374 · p95 0.398 · p99 0.443 · 25491 op/s</sub> | 0.222<br><sub>context: p90 0.286 · p95 0.311 · p99 0.357 · 34541 op/s</sub> | -27.1% (-0.082) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.356<br><sub>context: p90 0.434 · p95 0.451 · p99 0.478 · 2693 op/s</sub> | 0.342<br><sub>context: p90 0.403 · p95 0.420 · p99 0.492 · 2868 op/s</sub> | -4.0% (-0.014) | 150% AND 2 ms | 🟢 |
| 8 | 0.452<br><sub>context: p90 0.531 · p95 0.556 · p99 0.595 · 17249 op/s</sub> | 0.392<br><sub>context: p90 0.479 · p95 0.514 · p99 0.569 · 19769 op/s</sub> | -13.3% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.300<br><sub>context: p90 0.338 · p95 0.366 · p99 0.401 · 3257 op/s</sub> | 0.237<br><sub>context: p90 0.317 · p95 0.346 · p99 0.404 · 4025 op/s</sub> | -20.8% (-0.062) | 150% AND 2 ms | 🟢 |
| 8 | 0.375<br><sub>context: p90 0.457 · p95 0.485 · p99 0.530 · 20540 op/s</sub> | 0.331<br><sub>context: p90 0.418 · p95 0.449 · p99 0.512 · 23171 op/s</sub> | -11.8% (-0.044) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.380<br><sub>context: p90 0.440 · p95 0.454 · p99 0.494 · 2541 op/s</sub> | 0.321<br><sub>context: p90 0.387 · p95 0.403 · p99 0.442 · 3044 op/s</sub> | -15.5% (-0.059) | 150% AND 2 ms | 🟢 |
| 8 | 0.506<br><sub>context: p90 0.605 · p95 0.639 · p99 0.702 · 15410 op/s</sub> | 0.437<br><sub>context: p90 0.534 · p95 0.563 · p99 0.628 · 17735 op/s</sub> | -13.5% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.477<br><sub>context: p90 0.582 · p95 0.621 · p99 0.655 · 2054 op/s</sub> | 0.438<br><sub>context: p90 0.558 · p95 0.588 · p99 0.667 · 2223 op/s</sub> | -8.1% (-0.039) | 150% AND 2 ms | 🟢 |
| 8 | 0.642<br><sub>context: p90 0.811 · p95 0.864 · p99 0.979 · 12062 op/s</sub> | 0.597<br><sub>context: p90 0.780 · p95 0.841 · p99 0.966 · 12809 op/s</sub> | -7.0% (-0.045) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.517<br><sub>context: p90 0.618 · p95 0.639 · p99 0.700 · 1898 op/s</sub> | 0.472<br><sub>context: p90 0.569 · p95 0.587 · p99 0.709 · 2073 op/s</sub> | -8.8% (-0.045) | 150% AND 2 ms | 🟢 |
| 8 | 0.665<br><sub>context: p90 0.823 · p95 0.874 · p99 0.994 · 11665 op/s</sub> | 0.616<br><sub>context: p90 0.780 · p95 0.832 · p99 0.926 · 12439 op/s</sub> | -7.3% (-0.048) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.408<br><sub>context: p90 0.480 · p95 0.494 · p99 0.530 · 2391 op/s</sub> | 0.357<br><sub>context: p90 0.411 · p95 0.423 · p99 0.463 · 2765 op/s</sub> | -12.4% (-0.051) | 150% AND 2 ms | 🟢 |
| 8 | 0.547<br><sub>context: p90 0.648 · p95 0.677 · p99 0.758 · 14366 op/s</sub> | 0.467<br><sub>context: p90 0.569 · p95 0.598 · p99 0.679 · 16681 op/s</sub> | -14.6% (-0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.441<br><sub>context: p90 0.503 · p95 0.515 · p99 0.560 · 2238 op/s</sub> | 0.330<br><sub>context: p90 0.397 · p95 0.419 · p99 0.478 · 2997 op/s</sub> | -25.1% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.552<br><sub>context: p90 0.650 · p95 0.690 · p99 0.759 · 14094 op/s</sub> | 0.398<br><sub>context: p90 0.498 · p95 0.525 · p99 0.600 · 19102 op/s</sub> | -27.9% (-0.154) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.266<br><sub>context: p90 18.540 · p95 18.605 · p99 18.713 · 55 op/s</sub> | 15.265<br><sub>context: p90 15.488 · p95 15.539 · p99 15.707 · 65 op/s</sub> | -16.4% (-3.001) | 150% AND 2 ms | 🟢 |
| 8 | 23.734<br><sub>context: p90 31.967 · p95 35.629 · p99 39.167 · 305 op/s</sub> | 18.764<br><sub>context: p90 24.662 · p95 27.545 · p99 31.341 · 382 op/s</sub> | -20.9% (-4.971) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.468<br><sub>context: p90 0.526 · p95 0.552 · p99 0.573 · 2089 op/s</sub> | 0.365<br><sub>context: p90 0.451 · p95 0.508 · p99 0.558 · 2612 op/s</sub> | -22.0% (-0.103) | 150% AND 2 ms | 🟢 |
| 8 | 0.643<br><sub>context: p90 0.752 · p95 0.791 · p99 0.872 · 11990 op/s</sub> | 0.446<br><sub>context: p90 0.534 · p95 0.572 · p99 0.625 · 17491 op/s</sub> | -30.8% (-0.198) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.361<br><sub>context: p90 1.845 · p95 2.024 · p99 2.419 · 724 op/s</sub> | 1.250<br><sub>context: p90 1.639 · p95 1.747 · p99 2.028 · 796 op/s</sub> | -8.2% (-0.111) | 150% AND 2 ms | 🟢 |
| 8 | 1.768<br><sub>context: p90 2.488 · p95 2.702 · p99 3.167 · 4383 op/s</sub> | 1.555<br><sub>context: p90 2.192 · p95 2.407 · p99 2.821 · 4937 op/s</sub> | -12.0% (-0.213) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.514 · p95 0.548 · p99 0.589 · 2291 op/s</sub> | 0.351<br><sub>context: p90 0.405 · p95 0.429 · p99 0.459 · 2820 op/s</sub> | -16.2% (-0.068) | 150% AND 2 ms | 🟢 |
| 8 | 0.556<br><sub>context: p90 0.664 · p95 0.704 · p99 0.769 · 14115 op/s</sub> | 0.474<br><sub>context: p90 0.580 · p95 0.616 · p99 0.680 · 16356 op/s</sub> | -14.7% (-0.082) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.194<br><sub>context: p90 2.787 · p95 2.839 · p99 2.918 · 467 op/s</sub> | 0.336<br><sub>context: p90 0.408 · p95 0.427 · p99 0.450 · 2915 op/s</sub> | -84.7% (-1.858) | 150% AND 2 ms | 🟢 |
| 8 | 2.555<br><sub>context: p90 3.305 · p95 3.460 · p99 3.712 · 3188 op/s</sub> | 0.423<br><sub>context: p90 0.538 · p95 0.569 · p99 0.664 · 18065 op/s</sub> | -83.4% (-2.132) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.178<br><sub>context: p90 2.796 · p95 2.884 · p99 2.976 · 471 op/s</sub> | 0.350<br><sub>context: p90 0.421 · p95 0.446 · p99 0.507 · 2779 op/s</sub> | -83.9% (-1.828) | 150% AND 2 ms | 🟢 |
| 8 | 2.582<br><sub>context: p90 3.323 · p95 3.495 · p99 3.742 · 3178 op/s</sub> | 0.438<br><sub>context: p90 0.546 · p95 0.578 · p99 0.647 · 17424 op/s</sub> | -83.0% (-2.143) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.203<br><sub>context: p90 0.238 · p95 0.244 · p99 0.254 · 4820 op/s</sub> | 0.175<br><sub>context: p90 0.212 · p95 0.223 · p99 0.287 · 5452 op/s</sub> | -13.6% (-0.028) | 150% AND 2 ms | 🟢 |
| 8 | 0.292<br><sub>context: p90 0.361 · p95 0.386 · p99 0.428 · 26179 op/s</sub> | 0.304<br><sub>context: p90 0.404 · p95 0.437 · p99 0.512 · 24883 op/s</sub> | +4.0% (+0.012) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.339<br><sub>context: p90 0.417 · p95 0.431 · p99 0.472 · 2843 op/s</sub> | 0.166<br><sub>context: p90 0.189 · p95 0.194 · p99 0.200 · 5830 op/s</sub> | -51.1% (-0.173) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.413<br><sub>context: p90 0.498 · p95 0.522 · p99 0.593 · 18667 op/s</sub> | 0.238<br><sub>context: p90 0.310 · p95 0.338 · p99 0.390 · 31971 op/s</sub> | -42.3% (-0.174) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.329<br><sub>context: p90 0.392 · p95 0.407 · p99 0.438 · 2940 op/s</sub> | 0.271<br><sub>context: p90 0.329 · p95 0.351 · p99 0.394 · 3583 op/s</sub> | -17.6% (-0.058) | 150% AND 2 ms | 🟢 |
| 8 | 0.448<br><sub>context: p90 0.543 · p95 0.569 · p99 0.641 · 17180 op/s</sub> | 0.378<br><sub>context: p90 0.493 · p95 0.532 · p99 0.625 · 20242 op/s</sub> | -15.6% (-0.070) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.313<br><sub>context: p90 0.361 · p95 0.379 · p99 0.421 · 3085 op/s</sub> | 0.278<br><sub>context: p90 0.359 · p95 0.376 · p99 0.418 · 3447 op/s</sub> | -11.2% (-0.035) | 150% AND 2 ms | 🟢 |
| 8 | 0.441<br><sub>context: p90 0.542 · p95 0.577 · p99 0.669 · 17130 op/s</sub> | 0.387<br><sub>context: p90 0.501 · p95 0.542 · p99 0.647 · 19723 op/s</sub> | -12.2% (-0.054) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.319<br><sub>context: p90 0.389 · p95 0.409 · p99 0.471 · 3020 op/s</sub> | 0.209<br><sub>context: p90 0.263 · p95 0.292 · p99 0.351 · 4604 op/s</sub> | -34.7% (-0.111) | 150% AND 2 ms | 🟢 |
| 8 | 0.401<br><sub>context: p90 0.486 · p95 0.513 · p99 0.584 · 18885 op/s</sub> | 0.322<br><sub>context: p90 0.419 · p95 0.459 · p99 0.546 · 23341 op/s</sub> | -19.7% (-0.079) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.353<br><sub>context: p90 1.452 · p95 1.485 · p99 1.615 · 732 op/s</sub> | 0.998<br><sub>context: p90 1.098 · p95 1.127 · p99 1.206 · 992 op/s</sub> | -26.2% (-0.355) | 150% AND 2 ms | 🟢 |
| 8 | 1.776<br><sub>context: p90 2.128 · p95 2.242 · p99 2.577 · 4357 op/s</sub> | 1.191<br><sub>context: p90 1.353 · p95 1.411 · p99 1.520 · 6625 op/s</sub> | -32.9% (-0.585) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.263<br><sub>context: p90 1.377 · p95 1.407 · p99 1.526 · 790 op/s</sub> | 0.994<br><sub>context: p90 1.084 · p95 1.108 · p99 1.154 · 994 op/s</sub> | -21.3% (-0.270) | 150% AND 2 ms | 🟢 |
| 8 | 1.613<br><sub>context: p90 1.962 · p95 2.125 · p99 2.525 · 4716 op/s</sub> | 1.106<br><sub>context: p90 1.250 · p95 1.291 · p99 1.425 · 7114 op/s</sub> | -31.4% (-0.507) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.371<br><sub>context: p90 0.458 · p95 0.471 · p99 0.502 · 2602 op/s</sub> | 0.268<br><sub>context: p90 0.345 · p95 0.371 · p99 0.404 · 3596 op/s</sub> | -27.6% (-0.102) | 150% AND 2 ms | 🟢 |
| 8 | 0.472<br><sub>context: p90 0.571 · p95 0.604 · p99 0.666 · 16179 op/s</sub> | 0.393<br><sub>context: p90 0.502 · p95 0.529 · p99 0.607 · 19599 op/s</sub> | -16.6% (-0.078) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.459<br><sub>context: p90 0.524 · p95 0.541 · p99 0.569 · 2155 op/s</sub> | 0.384<br><sub>context: p90 0.466 · p95 0.490 · p99 0.546 · 2516 op/s</sub> | -16.2% (-0.075) | 150% AND 2 ms | 🟢 |
| 8 | 0.586<br><sub>context: p90 0.702 · p95 0.736 · p99 0.823 · 13008 op/s</sub> | 0.490<br><sub>context: p90 0.603 · p95 0.639 · p99 0.716 · 15969 op/s</sub> | -16.3% (-0.096) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.286<br><sub>context: p90 0.317 · p95 0.331 · p99 0.379 · 3390 op/s</sub> | 0.174<br><sub>context: p90 0.210 · p95 0.216 · p99 0.225 · 5491 op/s</sub> | -39.2% (-0.112) | 150% AND 2 ms | 🟢 |
| 8 | 0.406<br><sub>context: p90 0.491 · p95 0.524 · p99 0.598 · 18747 op/s</sub> | 0.319<br><sub>context: p90 0.414 · p95 0.449 · p99 0.513 · 23792 op/s</sub> | -21.6% (-0.088) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.208<br><sub>context: p90 0.242 · p95 0.252 · p99 0.299 · 4639 op/s</sub> | 0.180<br><sub>context: p90 0.219 · p95 0.239 · p99 0.281 · 5289 op/s</sub> | -13.2% (-0.027) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.357 · p95 0.377 · p99 0.435 · 26323 op/s</sub> | 0.299<br><sub>context: p90 0.395 · p95 0.435 · p99 0.505 · 25235 op/s</sub> | +2.2% (+0.006) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.210<br><sub>context: p90 0.249 · p95 0.275 · p99 0.309 · 4561 op/s</sub> | 0.171<br><sub>context: p90 0.213 · p95 0.240 · p99 0.272 · 5532 op/s</sub> | -18.7% (-0.039) | 150% AND 2 ms | 🟢 |
| 8 | 0.294<br><sub>context: p90 0.360 · p95 0.381 · p99 0.428 · 26238 op/s</sub> | 0.302<br><sub>context: p90 0.415 · p95 0.444 · p99 0.528 · 24758 op/s</sub> | +2.7% (+0.008) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.956<br><sub>context: p90 1.020 · p95 1.037 · p99 1.079 · 1031 op/s</sub> | 0.373<br><sub>context: p90 0.417 · p95 0.426 · p99 0.450 · 2615 op/s</sub> | -61.0% (-0.583) | 150% AND 2 ms | 🟢 |
| 8 | 1.270<br><sub>context: p90 1.648 · p95 1.839 · p99 2.174 · 6007 op/s</sub> | 0.450<br><sub>context: p90 0.522 · p95 0.543 · p99 0.575 · 17390 op/s</sub> | -64.6% (-0.820) | 150% AND 2 ms | 🟢 |

</details>
