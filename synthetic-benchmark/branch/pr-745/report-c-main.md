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
| 1 | 1.583<br><sub>context: p90 1.662 · p95 1.685 · p99 1.731 · 628 op/s</sub> | 0.997<br><sub>context: p90 1.089 · p95 1.135 · p99 1.171 · 993 op/s</sub> | -37.0% (-0.586) | 150% AND 2 ms | 🟢 |
| 8 | 1.930<br><sub>context: p90 2.493 · p95 2.699 · p99 3.078 · 3947 op/s</sub> | 0.967<br><sub>context: p90 1.125 · p95 1.172 · p99 1.289 · 8125 op/s</sub> | -49.9% (-0.963) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.260<br><sub>context: p90 2.377 · p95 2.411 · p99 2.496 · 437 op/s</sub> | 1.042<br><sub>context: p90 1.134 · p95 1.167 · p99 1.213 · 946 op/s</sub> | -53.9% (-1.217) | 150% AND 2 ms | 🟢 |
| 8 | 2.776<br><sub>context: p90 3.571 · p95 3.727 · p99 4.071 · 2771 op/s</sub> | 1.120<br><sub>context: p90 1.344 · p95 1.383 · p99 1.474 · 6955 op/s</sub> | -59.7% (-1.656) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.220<br><sub>context: p90 2.305 · p95 2.341 · p99 2.435 · 446 op/s</sub> | 1.502<br><sub>context: p90 1.612 · p95 1.635 · p99 1.677 · 658 op/s</sub> | -32.3% (-0.717) | 150% AND 2 ms | 🟢 |
| 8 | 2.862<br><sub>context: p90 3.608 · p95 3.812 · p99 4.268 · 2721 op/s</sub> | 1.812<br><sub>context: p90 2.343 · p95 2.521 · p99 2.830 · 4230 op/s</sub> | -36.7% (-1.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.874<br><sub>context: p90 2.959 · p95 2.987 · p99 3.025 · 346 op/s</sub> | 1.555<br><sub>context: p90 1.659 · p95 1.700 · p99 1.786 · 637 op/s</sub> | -45.9% (-1.319) | 150% AND 2 ms | 🟢 |
| 8 | 3.817<br><sub>context: p90 4.819 · p95 5.077 · p99 5.536 · 2059 op/s</sub> | 1.720<br><sub>context: p90 2.053 · p95 2.121 · p99 2.303 · 4512 op/s</sub> | -54.9% (-2.097) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.279<br><sub>context: p90 0.413 · p95 0.453 · p99 0.532 · 3283 op/s</sub> | 0.177<br><sub>context: p90 0.246 · p95 0.281 · p99 0.328 · 5302 op/s</sub> | -36.6% (-0.102) | 150% AND 2 ms | 🟢 |
| 8 | 0.285<br><sub>context: p90 0.422 · p95 0.466 · p99 0.560 · 23908 op/s</sub> | 0.224<br><sub>context: p90 0.299 · p95 0.322 · p99 0.353 · 34420 op/s</sub> | -21.6% (-0.062) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.577<br><sub>context: p90 0.685 · p95 0.705 · p99 0.740 · 1699 op/s</sub> | 0.377<br><sub>context: p90 0.482 · p95 0.504 · p99 0.550 · 2519 op/s</sub> | -34.7% (-0.200) | 150% AND 2 ms | 🟢 |
| 8 | 0.486<br><sub>context: p90 0.621 · p95 0.674 · p99 0.814 · 14423 op/s</sub> | 0.339<br><sub>context: p90 0.440 · p95 0.463 · p99 0.536 · 22346 op/s</sub> | -30.4% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.609<br><sub>context: p90 0.696 · p95 0.722 · p99 0.793 · 1622 op/s</sub> | 0.395<br><sub>context: p90 0.485 · p95 0.520 · p99 0.549 · 2475 op/s</sub> | -35.1% (-0.214) | 150% AND 2 ms | 🟢 |
| 8 | 0.573<br><sub>context: p90 0.782 · p95 0.868 · p99 1.071 · 12755 op/s</sub> | 0.362<br><sub>context: p90 0.459 · p95 0.492 · p99 0.556 · 21013 op/s</sub> | -36.9% (-0.211) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.603<br><sub>context: p90 0.720 · p95 0.743 · p99 0.835 · 1594 op/s</sub> | 0.561<br><sub>context: p90 0.665 · p95 0.705 · p99 0.788 · 1744 op/s</sub> | -6.9% (-0.042) | 150% AND 2 ms | 🟢 |
| 8 | 0.630<br><sub>context: p90 0.834 · p95 0.904 · p99 1.084 · 11747 op/s</sub> | 0.456<br><sub>context: p90 0.573 · p95 0.611 · p99 0.684 · 16827 op/s</sub> | -27.6% (-0.174) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.668<br><sub>context: p90 0.743 · p95 0.771 · p99 0.856 · 1485 op/s</sub> | 0.545<br><sub>context: p90 0.644 · p95 0.683 · p99 0.721 · 1807 op/s</sub> | -18.4% (-0.123) | 150% AND 2 ms | 🟢 |
| 8 | 0.614<br><sub>context: p90 0.762 · p95 0.811 · p99 0.931 · 12454 op/s</sub> | 0.483<br><sub>context: p90 0.599 · p95 0.634 · p99 0.712 · 15887 op/s</sub> | -21.3% (-0.131) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.797<br><sub>context: p90 0.943 · p95 0.987 · p99 1.146 · 1234 op/s</sub> | 0.706<br><sub>context: p90 0.839 · p95 0.891 · p99 0.991 · 1401 op/s</sub> | -11.4% (-0.091) | 150% AND 2 ms | 🟢 |
| 8 | 0.819<br><sub>context: p90 1.046 · p95 1.124 · p99 1.273 · 9422 op/s</sub> | 0.659<br><sub>context: p90 0.837 · p95 0.905 · p99 1.056 · 11545 op/s</sub> | -19.5% (-0.160) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.834<br><sub>context: p90 0.986 · p95 1.024 · p99 1.146 · 1184 op/s</sub> | 0.671<br><sub>context: p90 0.830 · p95 0.858 · p99 0.986 · 1444 op/s</sub> | -19.5% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.835<br><sub>context: p90 1.080 · p95 1.159 · p99 1.336 · 9249 op/s</sub> | 0.687<br><sub>context: p90 0.877 · p95 0.944 · p99 1.082 · 11236 op/s</sub> | -17.7% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.393<br><sub>context: p90 1.844 · p95 1.938 · p99 2.221 · 711 op/s</sub> | 1.235<br><sub>context: p90 1.635 · p95 1.815 · p99 1.904 · 799 op/s</sub> | -11.4% (-0.158) | 150% AND 2 ms | 🟢 |
| 8 | 1.689<br><sub>context: p90 2.427 · p95 2.659 · p99 3.161 · 4467 op/s</sub> | 1.313<br><sub>context: p90 1.842 · p95 2.024 · p99 2.321 · 5811 op/s</sub> | -22.3% (-0.376) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.493<br><sub>context: p90 1.964 · p95 2.251 · p99 2.665 · 645 op/s</sub> | 1.277<br><sub>context: p90 1.666 · p95 1.872 · p99 2.084 · 760 op/s</sub> | -14.5% (-0.217) | 150% AND 2 ms | 🟢 |
| 8 | 1.675<br><sub>context: p90 2.385 · p95 2.657 · p99 3.107 · 4541 op/s</sub> | 1.379<br><sub>context: p90 1.923 · p95 2.115 · p99 2.453 · 5511 op/s</sub> | -17.7% (-0.296) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.780<br><sub>context: p90 0.909 · p95 0.924 · p99 1.086 · 1260 op/s</sub> | 0.778<br><sub>context: p90 0.962 · p95 0.999 · p99 1.058 · 1279 op/s</sub> | -0.3% (-0.003) | 150% AND 2 ms | 🟢 |
| 8 | 0.813<br><sub>context: p90 0.967 · p95 1.013 · p99 1.149 · 9590 op/s</sub> | 0.793<br><sub>context: p90 1.036 · p95 1.109 · p99 1.284 · 9850 op/s</sub> | -2.5% (-0.020) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.551<br><sub>context: p90 0.638 · p95 0.662 · p99 0.700 · 1786 op/s</sub> | 0.477<br><sub>context: p90 0.591 · p95 0.622 · p99 0.662 · 2057 op/s</sub> | -13.4% (-0.074) | 150% AND 2 ms | 🟢 |
| 8 | 0.567<br><sub>context: p90 0.684 · p95 0.727 · p99 0.825 · 13653 op/s</sub> | 0.419<br><sub>context: p90 0.520 · p95 0.549 · p99 0.602 · 18380 op/s</sub> | -26.1% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.379<br><sub>context: p90 0.483 · p95 0.532 · p99 0.610 · 2517 op/s</sub> | 0.178<br><sub>context: p90 0.248 · p95 0.273 · p99 0.299 · 5248 op/s</sub> | -53.0% (-0.201) | 150% AND 2 ms | 🟢 |
| 8 | 0.342<br><sub>context: p90 0.426 · p95 0.467 · p99 0.529 · 22009 op/s</sub> | 0.361<br><sub>context: p90 0.771 · p95 0.903 · p99 1.079 · 17970 op/s</sub> | +5.7% (+0.019) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.219<br><sub>context: p90 0.285 · p95 0.315 · p99 0.350 · 4366 op/s</sub> | 0.156<br><sub>context: p90 0.220 · p95 0.243 · p99 0.279 · 5975 op/s</sub> | -28.8% (-0.063) | 150% AND 2 ms | 🟢 |
| 8 | 0.272<br><sub>context: p90 0.342 · p95 0.369 · p99 0.425 · 27997 op/s</sub> | 0.210<br><sub>context: p90 0.291 · p95 0.319 · p99 0.385 · 36332 op/s</sub> | -23.0% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.656<br><sub>context: p90 0.761 · p95 0.790 · p99 0.870 · 1508 op/s</sub> | 0.546<br><sub>context: p90 0.661 · p95 0.707 · p99 0.767 · 1791 op/s</sub> | -16.8% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.677<br><sub>context: p90 0.823 · p95 0.869 · p99 0.980 · 11445 op/s</sub> | 0.461<br><sub>context: p90 0.573 · p95 0.609 · p99 0.691 · 16675 op/s</sub> | -31.9% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.159<br><sub>context: p90 1.667 · p95 1.776 · p99 1.990 · 825 op/s</sub> | 1.538<br><sub>context: p90 2.184 · p95 2.365 · p99 2.671 · 639 op/s</sub> | +32.8% (+0.380) | 150% AND 2 ms | 🟢 |
| 8 | 1.367<br><sub>context: p90 2.075 · p95 2.357 · p99 2.801 · 5502 op/s</sub> | 2.253<br><sub>context: p90 3.449 · p95 3.891 · p99 4.389 · 3401 op/s</sub> | +64.9% (+0.886) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.571<br><sub>context: p90 5.534 · p95 6.268 · p99 6.761 · 268 op/s</sub> | 4.648<br><sub>context: p90 6.902 · p95 7.699 · p99 8.154 · 207 op/s</sub> | +30.1% (+1.076) | 150% AND 2 ms | 🟢 |
| 8 | 4.518<br><sub>context: p90 7.979 · p95 9.054 · p99 11.163 · 1594 op/s</sub> | 8.267<br><sub>context: p90 12.635 · p95 13.928 · p99 15.901 · 933 op/s</sub> | +83.0% (+3.749) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.398<br><sub>context: p90 0.505 · p95 0.543 · p99 0.583 · 2464 op/s</sub> | 0.302<br><sub>context: p90 0.466 · p95 0.494 · p99 0.535 · 2982 op/s</sub> | -24.0% (-0.096) | 150% AND 2 ms | 🟢 |
| 8 | 0.368<br><sub>context: p90 0.451 · p95 0.481 · p99 0.553 · 20796 op/s</sub> | 0.322<br><sub>context: p90 0.469 · p95 0.523 · p99 0.637 · 21969 op/s</sub> | -12.5% (-0.046) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.444<br><sub>context: p90 0.537 · p95 0.569 · p99 0.617 · 2225 op/s</sub> | 0.334<br><sub>context: p90 0.495 · p95 0.516 · p99 0.549 · 2837 op/s</sub> | -24.7% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.369<br><sub>context: p90 0.458 · p95 0.493 · p99 0.547 · 20450 op/s</sub> | 0.313<br><sub>context: p90 0.424 · p95 0.464 · p99 0.552 · 23744 op/s</sub> | -15.3% (-0.056) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.519<br><sub>context: p90 0.618 · p95 0.656 · p99 0.672 · 1894 op/s</sub> | 0.398<br><sub>context: p90 0.528 · p95 0.557 · p99 0.596 · 2444 op/s</sub> | -23.2% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.494<br><sub>context: p90 0.613 · p95 0.652 · p99 0.718 · 15478 op/s</sub> | 0.368<br><sub>context: p90 0.471 · p95 0.507 · p99 0.575 · 20676 op/s</sub> | -25.5% (-0.126) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.311<br><sub>context: p90 0.394 · p95 0.449 · p99 0.522 · 3134 op/s</sub> | 0.206<br><sub>context: p90 0.254 · p95 0.288 · p99 0.342 · 4697 op/s</sub> | -33.7% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.368 · p95 0.391 · p99 0.442 · 25947 op/s</sub> | 0.232<br><sub>context: p90 0.313 · p95 0.340 · p99 0.405 · 32491 op/s</sub> | -20.7% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.509<br><sub>context: p90 0.609 · p95 0.636 · p99 0.681 · 1920 op/s</sub> | 0.486<br><sub>context: p90 0.601 · p95 0.637 · p99 0.689 · 2015 op/s</sub> | -4.6% (-0.023) | 150% AND 2 ms | 🟢 |
| 8 | 0.469<br><sub>context: p90 0.570 · p95 0.598 · p99 0.654 · 16536 op/s</sub> | 0.412<br><sub>context: p90 0.519 · p95 0.565 · p99 0.677 · 18403 op/s</sub> | -12.1% (-0.057) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.440<br><sub>context: p90 0.530 · p95 0.571 · p99 0.598 · 2244 op/s</sub> | 0.453<br><sub>context: p90 0.551 · p95 0.584 · p99 0.631 · 2199 op/s</sub> | +3.1% (+0.014) | 150% AND 2 ms | 🟢 |
| 8 | 0.388<br><sub>context: p90 0.471 · p95 0.504 · p99 0.556 · 19573 op/s</sub> | 0.351<br><sub>context: p90 0.469 · p95 0.515 · p99 0.613 · 20941 op/s</sub> | -9.6% (-0.037) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.579<br><sub>context: p90 0.686 · p95 0.728 · p99 0.784 · 1698 op/s</sub> | 0.581<br><sub>context: p90 0.698 · p95 0.744 · p99 0.797 · 1684 op/s</sub> | +0.3% (+0.002) | 150% AND 2 ms | 🟢 |
| 8 | 0.530<br><sub>context: p90 0.650 · p95 0.685 · p99 0.772 · 14553 op/s</sub> | 0.476<br><sub>context: p90 0.598 · p95 0.634 · p99 0.737 · 16191 op/s</sub> | -10.3% (-0.054) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.670<br><sub>context: p90 0.794 · p95 0.825 · p99 0.921 · 1467 op/s</sub> | 0.680<br><sub>context: p90 0.814 · p95 0.832 · p99 0.941 · 1470 op/s</sub> | +1.6% (+0.011) | 150% AND 2 ms | 🟢 |
| 8 | 0.692<br><sub>context: p90 0.877 · p95 0.937 · p99 1.034 · 11103 op/s</sub> | 0.636<br><sub>context: p90 0.827 · p95 0.888 · p99 1.020 · 11855 op/s</sub> | -8.1% (-0.056) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.688<br><sub>context: p90 0.801 · p95 0.865 · p99 0.946 · 1421 op/s</sub> | 0.644<br><sub>context: p90 0.760 · p95 0.793 · p99 0.904 · 1537 op/s</sub> | -6.4% (-0.044) | 150% AND 2 ms | 🟢 |
| 8 | 0.697<br><sub>context: p90 0.882 · p95 0.934 · p99 1.058 · 11043 op/s</sub> | 0.647<br><sub>context: p90 0.809 · p95 0.865 · p99 0.970 · 11893 op/s</sub> | -7.2% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.581<br><sub>context: p90 0.668 · p95 0.699 · p99 0.729 · 1707 op/s</sub> | 0.555<br><sub>context: p90 0.651 · p95 0.683 · p99 0.764 · 1786 op/s</sub> | -4.5% (-0.026) | 150% AND 2 ms | 🟢 |
| 8 | 0.560<br><sub>context: p90 0.672 · p95 0.706 · p99 0.788 · 13958 op/s</sub> | 0.488<br><sub>context: p90 0.594 · p95 0.623 · p99 0.692 · 15834 op/s</sub> | -12.8% (-0.072) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.569<br><sub>context: p90 0.671 · p95 0.686 · p99 0.740 · 1727 op/s</sub> | 0.470<br><sub>context: p90 0.586 · p95 0.618 · p99 0.641 · 2082 op/s</sub> | -17.5% (-0.099) | 150% AND 2 ms | 🟢 |
| 8 | 0.558<br><sub>context: p90 0.672 · p95 0.711 · p99 0.796 · 13839 op/s</sub> | 0.409<br><sub>context: p90 0.525 · p95 0.561 · p99 0.636 · 18615 op/s</sub> | -26.7% (-0.149) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.917<br><sub>context: p90 19.149 · p95 19.276 · p99 19.472 · 53 op/s</sub> | 15.496<br><sub>context: p90 15.676 · p95 15.786 · p99 15.925 · 64 op/s</sub> | -18.1% (-3.420) | 150% AND 2 ms | 🟢 |
| 8 | 24.359<br><sub>context: p90 33.759 · p95 37.422 · p99 42.112 · 298 op/s</sub> | 18.912<br><sub>context: p90 25.178 · p95 27.864 · p99 31.332 · 379 op/s</sub> | -22.4% (-5.447) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.685<br><sub>context: p90 0.768 · p95 0.789 · p99 0.876 · 1456 op/s</sub> | 0.527<br><sub>context: p90 0.644 · p95 0.663 · p99 0.741 · 1832 op/s</sub> | -23.0% (-0.158) | 150% AND 2 ms | 🟢 |
| 8 | 0.669<br><sub>context: p90 0.810 · p95 0.861 · p99 0.952 · 11176 op/s</sub> | 0.470<br><sub>context: p90 0.570 · p95 0.597 · p99 0.672 · 16205 op/s</sub> | -29.7% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.635<br><sub>context: p90 2.184 · p95 2.337 · p99 2.540 · 606 op/s</sub> | 1.416<br><sub>context: p90 1.937 · p95 2.077 · p99 2.379 · 691 op/s</sub> | -13.4% (-0.219) | 150% AND 2 ms | 🟢 |
| 8 | 1.897<br><sub>context: p90 2.671 · p95 2.898 · p99 3.335 · 4065 op/s</sub> | 1.619<br><sub>context: p90 2.212 · p95 2.438 · p99 2.935 · 4821 op/s</sub> | -14.6% (-0.278) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.623<br><sub>context: p90 0.713 · p95 0.740 · p99 0.796 · 1581 op/s</sub> | 0.524<br><sub>context: p90 0.613 · p95 0.633 · p99 0.681 · 1889 op/s</sub> | -15.9% (-0.099) | 150% AND 2 ms | 🟢 |
| 8 | 0.586<br><sub>context: p90 0.709 · p95 0.755 · p99 0.847 · 13212 op/s</sub> | 0.494<br><sub>context: p90 0.623 · p95 0.666 · p99 0.760 · 15633 op/s</sub> | -15.7% (-0.092) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.387<br><sub>context: p90 2.981 · p95 3.058 · p99 3.351 · 426 op/s</sub> | 0.470<br><sub>context: p90 0.569 · p95 0.587 · p99 0.642 · 2129 op/s</sub> | -80.3% (-1.918) | 150% AND 2 ms | 🟢 |
| 8 | 2.582<br><sub>context: p90 3.350 · p95 3.554 · p99 3.824 · 3042 op/s</sub> | 0.431<br><sub>context: p90 0.547 · p95 0.584 · p99 0.659 · 17810 op/s</sub> | -83.3% (-2.152) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.509<br><sub>context: p90 3.083 · p95 3.188 · p99 3.293 · 414 op/s</sub> | 0.509<br><sub>context: p90 0.606 · p95 0.631 · p99 0.697 · 1943 op/s</sub> | -79.7% (-2.000) | 150% AND 2 ms | 🟢 |
| 8 | 2.656<br><sub>context: p90 3.448 · p95 3.620 · p99 3.893 · 3059 op/s</sub> | 0.454<br><sub>context: p90 0.577 · p95 0.616 · p99 0.695 · 16984 op/s</sub> | -82.9% (-2.202) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.447 · p95 0.484 · p99 0.504 · 2638 op/s</sub> | 0.280<br><sub>context: p90 0.384 · p95 0.413 · p99 0.459 · 3424 op/s</sub> | -24.4% (-0.091) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.385 · p95 0.413 · p99 0.470 · 25107 op/s</sub> | 0.315<br><sub>context: p90 0.430 · p95 0.474 · p99 0.551 · 23858 op/s</sub> | +3.1% (+0.010) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.452<br><sub>context: p90 0.533 · p95 0.552 · p99 0.595 · 2185 op/s</sub> | 0.254<br><sub>context: p90 0.315 · p95 0.328 · p99 0.392 · 3893 op/s</sub> | -43.9% (-0.199) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.423<br><sub>context: p90 0.504 · p95 0.536 · p99 0.600 · 18403 op/s</sub> | 0.253<br><sub>context: p90 0.334 · p95 0.368 · p99 0.442 · 27168 op/s</sub> | -40.1% (-0.169) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.450<br><sub>context: p90 0.564 · p95 0.590 · p99 0.637 · 2151 op/s</sub> | 0.432<br><sub>context: p90 0.533 · p95 0.561 · p99 0.630 · 2290 op/s</sub> | -3.9% (-0.018) | 150% AND 2 ms | 🟢 |
| 8 | 0.454<br><sub>context: p90 0.556 · p95 0.593 · p99 0.668 · 16975 op/s</sub> | 0.380<br><sub>context: p90 0.502 · p95 0.556 · p99 0.655 · 19683 op/s</sub> | -16.2% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.460<br><sub>context: p90 0.553 · p95 0.591 · p99 0.655 · 2147 op/s</sub> | 0.428<br><sub>context: p90 0.522 · p95 0.550 · p99 0.645 · 2281 op/s</sub> | -6.8% (-0.031) | 150% AND 2 ms | 🟢 |
| 8 | 0.456<br><sub>context: p90 0.564 · p95 0.609 · p99 0.711 · 16965 op/s</sub> | 0.416<br><sub>context: p90 0.581 · p95 0.643 · p99 0.777 · 17803 op/s</sub> | -8.8% (-0.040) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.467<br><sub>context: p90 0.551 · p95 0.574 · p99 0.643 · 2122 op/s</sub> | 0.331<br><sub>context: p90 0.436 · p95 0.478 · p99 0.529 · 2899 op/s</sub> | -29.2% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.403<br><sub>context: p90 0.510 · p95 0.552 · p99 0.634 · 18403 op/s</sub> | 0.318<br><sub>context: p90 0.419 · p95 0.453 · p99 0.509 · 23563 op/s</sub> | -21.1% (-0.085) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.602<br><sub>context: p90 1.771 · p95 1.813 · p99 1.926 · 618 op/s</sub> | 1.216<br><sub>context: p90 1.322 · p95 1.371 · p99 1.542 · 818 op/s</sub> | -24.1% (-0.386) | 150% AND 2 ms | 🟢 |
| 8 | 1.835<br><sub>context: p90 2.230 · p95 2.363 · p99 2.772 · 4170 op/s</sub> | 1.272<br><sub>context: p90 1.467 · p95 1.532 · p99 1.630 · 6153 op/s</sub> | -30.7% (-0.564) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.443<br><sub>context: p90 1.553 · p95 1.604 · p99 1.662 · 689 op/s</sub> | 1.195<br><sub>context: p90 1.296 · p95 1.323 · p99 1.372 · 829 op/s</sub> | -17.2% (-0.248) | 150% AND 2 ms | 🟢 |
| 8 | 1.665<br><sub>context: p90 2.061 · p95 2.172 · p99 2.497 · 4625 op/s</sub> | 1.163<br><sub>context: p90 1.327 · p95 1.391 · p99 1.528 · 6690 op/s</sub> | -30.2% (-0.503) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.624<br><sub>context: p90 0.780 · p95 0.819 · p99 0.888 · 1546 op/s</sub> | 0.412<br><sub>context: p90 0.519 · p95 0.540 · p99 0.589 · 2341 op/s</sub> | -34.0% (-0.212) | 150% AND 2 ms | 🟢 |
| 8 | 0.486<br><sub>context: p90 0.598 · p95 0.642 · p99 0.740 · 15702 op/s</sub> | 0.390<br><sub>context: p90 0.494 · p95 0.524 · p99 0.576 · 19630 op/s</sub> | -19.7% (-0.096) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.652<br><sub>context: p90 0.760 · p95 0.778 · p99 0.839 · 1493 op/s</sub> | 0.545<br><sub>context: p90 0.672 · p95 0.703 · p99 0.766 · 1788 op/s</sub> | -16.3% (-0.106) | 150% AND 2 ms | 🟢 |
| 8 | 0.595<br><sub>context: p90 0.718 · p95 0.758 · p99 0.860 · 13048 op/s</sub> | 0.506<br><sub>context: p90 0.638 · p95 0.678 · p99 0.772 · 15315 op/s</sub> | -15.0% (-0.089) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.409<br><sub>context: p90 0.495 · p95 0.540 · p99 0.624 · 2399 op/s</sub> | 0.358<br><sub>context: p90 0.469 · p95 0.500 · p99 0.546 · 2682 op/s</sub> | -12.6% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.414<br><sub>context: p90 0.506 · p95 0.536 · p99 0.607 · 18643 op/s</sub> | 0.356<br><sub>context: p90 0.479 · p95 0.523 · p99 0.607 · 21045 op/s</sub> | -14.1% (-0.059) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.499<br><sub>context: p90 0.655 · p95 0.686 · p99 0.767 · 1966 op/s</sub> | 0.281<br><sub>context: p90 0.369 · p95 0.418 · p99 0.487 · 3396 op/s</sub> | -43.6% (-0.218) | 150% AND 2 ms | 🟢 |
| 8 | 0.316<br><sub>context: p90 0.400 · p95 0.430 · p99 0.485 · 24136 op/s</sub> | 0.312<br><sub>context: p90 0.414 · p95 0.452 · p99 0.537 · 23957 op/s</sub> | -1.2% (-0.004) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.401<br><sub>context: p90 0.492 · p95 0.526 · p99 0.569 · 2462 op/s</sub> | 0.318<br><sub>context: p90 0.432 · p95 0.470 · p99 0.507 · 3078 op/s</sub> | -20.7% (-0.083) | 150% AND 2 ms | 🟢 |
| 8 | 0.327<br><sub>context: p90 0.450 · p95 0.496 · p99 0.576 · 22873 op/s</sub> | 0.329<br><sub>context: p90 0.461 · p95 0.503 · p99 0.600 · 22874 op/s</sub> | +0.6% (+0.002) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.174<br><sub>context: p90 1.268 · p95 1.297 · p99 1.334 · 842 op/s</sub> | 0.508<br><sub>context: p90 0.611 · p95 0.634 · p99 0.672 · 1924 op/s</sub> | -56.8% (-0.666) | 150% AND 2 ms | 🟢 |
| 8 | 1.345<br><sub>context: p90 1.847 · p95 2.034 · p99 2.301 · 5580 op/s</sub> | 0.472<br><sub>context: p90 0.557 · p95 0.591 · p99 0.635 · 16480 op/s</sub> | -64.9% (-0.874) | 150% AND 2 ms | 🟢 |

</details>
