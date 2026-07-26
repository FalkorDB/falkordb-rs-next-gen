### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:8b5e5c3b1fe54d523552910dd06e2ae5fa9e8fc2d74fd4968b2eadbd1d1c986c |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:8b5e5c3b1fe54d523552910dd06e2ae5fa9e8fc2d74fd4968b2eadbd1d1c986c

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.378<br><sub>context: p90 1.467 · p95 1.510 · p99 1.548 · 722 op/s</sub> | 0.735<br><sub>context: p90 0.804 · p95 0.824 · p99 0.856 · 1330 op/s</sub> | -46.6% (-0.643) | 150% AND 2 ms | 🟢 |
| 8 | 1.887<br><sub>context: p90 2.408 · p95 2.598 · p99 2.965 · 4116 op/s</sub> | 0.903<br><sub>context: p90 1.048 · p95 1.085 · p99 1.149 · 8750 op/s</sub> | -52.1% (-0.984) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.047<br><sub>context: p90 2.128 · p95 2.150 · p99 2.242 · 486 op/s</sub> | 0.797<br><sub>context: p90 0.851 · p95 0.875 · p99 0.896 · 1238 op/s</sub> | -61.1% (-1.251) | 150% AND 2 ms | 🟢 |
| 8 | 2.614<br><sub>context: p90 3.406 · p95 3.492 · p99 3.821 · 2904 op/s</sub> | 0.995<br><sub>context: p90 1.190 · p95 1.229 · p99 1.294 · 7864 op/s</sub> | -61.9% (-1.619) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.016<br><sub>context: p90 2.134 · p95 2.186 · p99 2.249 · 493 op/s</sub> | 1.269<br><sub>context: p90 1.306 · p95 1.317 · p99 1.345 · 780 op/s</sub> | -37.1% (-0.748) | 150% AND 2 ms | 🟢 |
| 8 | 2.685<br><sub>context: p90 3.467 · p95 3.643 · p99 4.073 · 2867 op/s</sub> | 1.751<br><sub>context: p90 2.273 · p95 2.456 · p99 2.735 · 4392 op/s</sub> | -34.8% (-0.934) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.653<br><sub>context: p90 2.766 · p95 2.800 · p99 2.850 · 375 op/s</sub> | 1.345<br><sub>context: p90 1.390 · p95 1.409 · p99 1.438 · 736 op/s</sub> | -49.3% (-1.307) | 150% AND 2 ms | 🟢 |
| 8 | 3.641<br><sub>context: p90 4.716 · p95 4.952 · p99 5.465 · 2115 op/s</sub> | 1.621<br><sub>context: p90 1.947 · p95 1.991 · p99 2.061 · 4788 op/s</sub> | -55.5% (-2.021) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.142<br><sub>context: p90 0.173 · p95 0.178 · p99 0.193 · 6633 op/s</sub> | 0.125<br><sub>context: p90 0.135 · p95 0.143 · p99 0.162 · 7457 op/s</sub> | -12.0% (-0.017) | 150% AND 2 ms | 🟢 |
| 8 | 0.235<br><sub>context: p90 0.295 · p95 0.314 · p99 0.349 · 32269 op/s</sub> | 0.213<br><sub>context: p90 0.277 · p95 0.302 · p99 0.353 · 36558 op/s</sub> | -9.5% (-0.022) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.332<br><sub>context: p90 0.377 · p95 0.395 · p99 0.449 · 2837 op/s</sub> | 0.227<br><sub>context: p90 0.272 · p95 0.291 · p99 0.310 · 4214 op/s</sub> | -31.5% (-0.104) | 150% AND 2 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.514 · p95 0.545 · p99 0.626 · 17972 op/s</sub> | 0.321<br><sub>context: p90 0.410 · p95 0.445 · p99 0.515 · 23669 op/s</sub> | -25.7% (-0.111) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.353<br><sub>context: p90 0.388 · p95 0.397 · p99 0.419 · 2798 op/s</sub> | 0.243<br><sub>context: p90 0.294 · p95 0.332 · p99 0.369 · 3973 op/s</sub> | -31.2% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.470<br><sub>context: p90 0.562 · p95 0.588 · p99 0.655 · 16622 op/s</sub> | 0.344<br><sub>context: p90 0.432 · p95 0.464 · p99 0.504 · 22389 op/s</sub> | -26.7% (-0.126) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.392<br><sub>context: p90 0.443 · p95 0.453 · p99 0.488 · 2506 op/s</sub> | 0.323<br><sub>context: p90 0.387 · p95 0.405 · p99 0.438 · 3022 op/s</sub> | -17.6% (-0.069) | 150% AND 2 ms | 🟢 |
| 8 | 0.519<br><sub>context: p90 0.615 · p95 0.646 · p99 0.725 · 14940 op/s</sub> | 0.446<br><sub>context: p90 0.543 · p95 0.573 · p99 0.627 · 17258 op/s</sub> | -14.0% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.467<br><sub>context: p90 0.560 · p95 0.584 · p99 0.630 · 2092 op/s</sub> | 0.370<br><sub>context: p90 0.412 · p95 0.428 · p99 0.449 · 2672 op/s</sub> | -20.7% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.560<br><sub>context: p90 0.664 · p95 0.696 · p99 0.784 · 13905 op/s</sub> | 0.466<br><sub>context: p90 0.574 · p95 0.604 · p99 0.663 · 16318 op/s</sub> | -16.8% (-0.094) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.553<br><sub>context: p90 0.676 · p95 0.750 · p99 0.871 · 1781 op/s</sub> | 0.461<br><sub>context: p90 0.556 · p95 0.596 · p99 0.638 · 2127 op/s</sub> | -16.5% (-0.091) | 150% AND 2 ms | 🟢 |
| 8 | 0.742<br><sub>context: p90 0.931 · p95 1.001 · p99 1.134 · 10366 op/s</sub> | 0.617<br><sub>context: p90 0.784 · p95 0.835 · p99 0.955 · 12430 op/s</sub> | -16.9% (-0.125) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.590<br><sub>context: p90 0.728 · p95 0.791 · p99 0.881 · 1665 op/s</sub> | 0.491<br><sub>context: p90 0.597 · p95 0.642 · p99 0.688 · 1993 op/s</sub> | -16.7% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.778<br><sub>context: p90 0.981 · p95 1.047 · p99 1.180 · 9987 op/s</sub> | 0.653<br><sub>context: p90 0.836 · p95 0.883 · p99 0.988 · 11744 op/s</sub> | -16.1% (-0.125) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.203<br><sub>context: p90 1.702 · p95 1.810 · p99 2.135 · 808 op/s</sub> | 0.974<br><sub>context: p90 1.258 · p95 1.529 · p99 1.680 · 1027 op/s</sub> | -19.0% (-0.229) | 150% AND 2 ms | 🟢 |
| 8 | 1.564<br><sub>context: p90 2.216 · p95 2.402 · p99 2.862 · 4907 op/s</sub> | 1.253<br><sub>context: p90 1.770 · p95 1.923 · p99 2.218 · 6188 op/s</sub> | -19.8% (-0.310) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.274<br><sub>context: p90 1.724 · p95 1.980 · p99 2.129 · 756 op/s</sub> | 0.966<br><sub>context: p90 1.275 · p95 1.447 · p99 1.707 · 1000 op/s</sub> | -24.2% (-0.309) | 150% AND 2 ms | 🟢 |
| 8 | 1.611<br><sub>context: p90 2.255 · p95 2.458 · p99 2.887 · 4729 op/s</sub> | 1.258<br><sub>context: p90 1.759 · p95 1.919 · p99 2.223 · 6029 op/s</sub> | -22.0% (-0.354) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.606<br><sub>context: p90 0.700 · p95 0.728 · p99 0.814 · 1632 op/s</sub> | 0.591<br><sub>context: p90 0.734 · p95 0.789 · p99 0.871 · 1700 op/s</sub> | -2.6% (-0.016) | 150% AND 2 ms | 🟢 |
| 8 | 0.776<br><sub>context: p90 0.911 · p95 0.954 · p99 1.051 · 10147 op/s</sub> | 0.747<br><sub>context: p90 0.990 · p95 1.081 · p99 1.227 · 10415 op/s</sub> | -3.7% (-0.029) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.409<br><sub>context: p90 0.483 · p95 0.506 · p99 0.534 · 2372 op/s</sub> | 0.306<br><sub>context: p90 0.355 · p95 0.369 · p99 0.416 · 3251 op/s</sub> | -25.1% (-0.103) | 150% AND 2 ms | 🟢 |
| 8 | 0.547<br><sub>context: p90 0.654 · p95 0.702 · p99 0.783 · 14146 op/s</sub> | 0.397<br><sub>context: p90 0.490 · p95 0.519 · p99 0.578 · 19544 op/s</sub> | -27.4% (-0.150) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.275 · p95 0.284 · p99 0.310 · 4240 op/s</sub> | 0.128<br><sub>context: p90 0.163 · p95 0.170 · p99 0.182 · 7332 op/s</sub> | -42.2% (-0.094) | 150% AND 2 ms | 🟢 |
| 8 | 0.322<br><sub>context: p90 0.397 · p95 0.425 · p99 0.480 · 23658 op/s</sub> | 0.226<br><sub>context: p90 0.298 · p95 0.321 · p99 0.359 · 33933 op/s</sub> | -29.8% (-0.096) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.201 · p95 0.207 · p99 0.216 · 5514 op/s</sub> | 0.113<br><sub>context: p90 0.123 · p95 0.128 · p99 0.141 · 8480 op/s</sub> | -36.0% (-0.063) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.337 · p95 0.359 · p99 0.407 · 28537 op/s</sub> | 0.199<br><sub>context: p90 0.280 · p95 0.309 · p99 0.380 · 38004 op/s</sub> | -25.5% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.483<br><sub>context: p90 0.535 · p95 0.549 · p99 0.585 · 2027 op/s</sub> | 0.323<br><sub>context: p90 0.390 · p95 0.412 · p99 0.455 · 2979 op/s</sub> | -33.1% (-0.160) | 150% AND 2 ms | 🟢 |
| 8 | 0.662<br><sub>context: p90 0.786 · p95 0.837 · p99 0.971 · 11635 op/s</sub> | 0.446<br><sub>context: p90 0.544 · p95 0.579 · p99 0.634 · 17414 op/s</sub> | -32.7% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.989<br><sub>context: p90 1.414 · p95 1.534 · p99 1.750 · 969 op/s</sub> | 1.210<br><sub>context: p90 1.858 · p95 2.035 · p99 2.284 · 795 op/s</sub> | +22.3% (+0.220) | 150% AND 2 ms | 🟢 |
| 8 | 1.344<br><sub>context: p90 2.070 · p95 2.365 · p99 2.900 · 5524 op/s</sub> | 2.167<br><sub>context: p90 3.428 · p95 3.875 · p99 4.389 · 3490 op/s</sub> | +61.3% (+0.823) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.315<br><sub>context: p90 5.359 · p95 6.039 · p99 6.645 · 287 op/s</sub> | 4.396<br><sub>context: p90 6.575 · p95 7.276 · p99 7.984 · 222 op/s</sub> | +32.6% (+1.082) | 150% AND 2 ms | 🟢 |
| 8 | 4.472<br><sub>context: p90 7.937 · p95 8.855 · p99 11.395 · 1631 op/s</sub> | 8.325<br><sub>context: p90 12.717 · p95 13.787 · p99 15.668 · 934 op/s</sub> | +86.1% (+3.853) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.242<br><sub>context: p90 0.288 · p95 0.307 · p99 0.352 · 3983 op/s</sub> | 0.168<br><sub>context: p90 0.199 · p95 0.210 · p99 0.224 · 5612 op/s</sub> | -30.5% (-0.074) | 150% AND 2 ms | 🟢 |
| 8 | 0.357<br><sub>context: p90 0.436 · p95 0.455 · p99 0.517 · 21644 op/s</sub> | 0.296<br><sub>context: p90 0.389 · p95 0.431 · p99 0.497 · 25615 op/s</sub> | -17.1% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.256<br><sub>context: p90 0.322 · p95 0.351 · p99 0.386 · 3764 op/s</sub> | 0.162<br><sub>context: p90 0.190 · p95 0.198 · p99 0.212 · 6010 op/s</sub> | -37.0% (-0.095) | 150% AND 2 ms | 🟢 |
| 8 | 0.358<br><sub>context: p90 0.433 · p95 0.459 · p99 0.505 · 21576 op/s</sub> | 0.298<br><sub>context: p90 0.399 · p95 0.432 · p99 0.508 · 25499 op/s</sub> | -16.8% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.342<br><sub>context: p90 0.423 · p95 0.462 · p99 0.528 · 2797 op/s</sub> | 0.278<br><sub>context: p90 0.296 · p95 0.304 · p99 0.332 · 3505 op/s</sub> | -18.7% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.478<br><sub>context: p90 0.600 · p95 0.639 · p99 0.711 · 15946 op/s</sub> | 0.358<br><sub>context: p90 0.450 · p95 0.488 · p99 0.563 · 21265 op/s</sub> | -25.1% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.195<br><sub>context: p90 0.247 · p95 0.278 · p99 0.337 · 4792 op/s</sub> | 0.136<br><sub>context: p90 0.165 · p95 0.174 · p99 0.193 · 6848 op/s</sub> | -30.0% (-0.058) | 150% AND 2 ms | 🟢 |
| 8 | 0.284<br><sub>context: p90 0.348 · p95 0.374 · p99 0.419 · 26987 op/s</sub> | 0.221<br><sub>context: p90 0.281 · p95 0.302 · p99 0.335 · 35082 op/s</sub> | -22.3% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.345<br><sub>context: p90 0.374 · p95 0.387 · p99 0.407 · 2819 op/s</sub> | 0.341<br><sub>context: p90 0.383 · p95 0.400 · p99 0.447 · 2912 op/s</sub> | -1.2% (-0.004) | 150% AND 2 ms | 🟢 |
| 8 | 0.445<br><sub>context: p90 0.518 · p95 0.543 · p99 0.594 · 17564 op/s</sub> | 0.391<br><sub>context: p90 0.472 · p95 0.498 · p99 0.555 · 20035 op/s</sub> | -12.1% (-0.054) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.284 · p95 0.319 · p99 0.331 · 4014 op/s</sub> | 0.249<br><sub>context: p90 0.303 · p95 0.326 · p99 0.358 · 3922 op/s</sub> | +4.1% (+0.010) | 150% AND 2 ms | 🟢 |
| 8 | 0.373<br><sub>context: p90 0.443 · p95 0.470 · p99 0.510 · 20784 op/s</sub> | 0.319<br><sub>context: p90 0.405 · p95 0.436 · p99 0.500 · 23762 op/s</sub> | -14.3% (-0.053) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.383<br><sub>context: p90 0.433 · p95 0.447 · p99 0.480 · 2564 op/s</sub> | 0.318<br><sub>context: p90 0.378 · p95 0.388 · p99 0.432 · 3081 op/s</sub> | -17.1% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.502<br><sub>context: p90 0.600 · p95 0.627 · p99 0.700 · 15503 op/s</sub> | 0.429<br><sub>context: p90 0.534 · p95 0.564 · p99 0.621 · 17927 op/s</sub> | -14.4% (-0.072) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.463<br><sub>context: p90 0.563 · p95 0.601 · p99 0.678 · 2094 op/s</sub> | 0.446<br><sub>context: p90 0.540 · p95 0.565 · p99 0.598 · 2195 op/s</sub> | -3.8% (-0.018) | 150% AND 2 ms | 🟢 |
| 8 | 0.649<br><sub>context: p90 0.819 · p95 0.873 · p99 1.006 · 11924 op/s</sub> | 0.602<br><sub>context: p90 0.771 · p95 0.823 · p99 0.944 · 12772 op/s</sub> | -7.2% (-0.047) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.476<br><sub>context: p90 0.553 · p95 0.580 · p99 0.634 · 2075 op/s</sub> | 0.452<br><sub>context: p90 0.516 · p95 0.550 · p99 0.594 · 2211 op/s</sub> | -4.9% (-0.023) | 150% AND 2 ms | 🟢 |
| 8 | 0.669<br><sub>context: p90 0.822 · p95 0.875 · p99 0.987 · 11671 op/s</sub> | 0.607<br><sub>context: p90 0.761 · p95 0.804 · p99 0.896 · 12779 op/s</sub> | -9.3% (-0.062) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.418<br><sub>context: p90 0.473 · p95 0.500 · p99 0.566 · 2345 op/s</sub> | 0.340<br><sub>context: p90 0.394 · p95 0.412 · p99 0.449 · 2883 op/s</sub> | -18.9% (-0.079) | 150% AND 2 ms | 🟢 |
| 8 | 0.544<br><sub>context: p90 0.643 · p95 0.682 · p99 0.757 · 14331 op/s</sub> | 0.461<br><sub>context: p90 0.571 · p95 0.606 · p99 0.678 · 16787 op/s</sub> | -15.4% (-0.084) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.400<br><sub>context: p90 0.465 · p95 0.487 · p99 0.536 · 2402 op/s</sub> | 0.254<br><sub>context: p90 0.308 · p95 0.322 · p99 0.344 · 3829 op/s</sub> | -36.5% (-0.146) | 150% AND 2 ms | 🟢 |
| 8 | 0.538<br><sub>context: p90 0.635 · p95 0.669 · p99 0.749 · 14413 op/s</sub> | 0.379<br><sub>context: p90 0.468 · p95 0.500 · p99 0.559 · 20484 op/s</sub> | -29.5% (-0.159) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.209<br><sub>context: p90 18.394 · p95 18.454 · p99 18.529 · 55 op/s</sub> | 15.091<br><sub>context: p90 15.285 · p95 15.359 · p99 15.467 · 66 op/s</sub> | -17.1% (-3.117) | 150% AND 2 ms | 🟢 |
| 8 | 22.795<br><sub>context: p90 31.252 · p95 34.476 · p99 38.839 · 320 op/s</sub> | 18.238<br><sub>context: p90 23.512 · p95 26.386 · p99 30.128 · 393 op/s</sub> | -20.0% (-4.557) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.472<br><sub>context: p90 0.521 · p95 0.542 · p99 0.574 · 2091 op/s</sub> | 0.347<br><sub>context: p90 0.402 · p95 0.427 · p99 0.473 · 2793 op/s</sub> | -26.4% (-0.125) | 150% AND 2 ms | 🟢 |
| 8 | 0.639<br><sub>context: p90 0.736 · p95 0.771 · p99 0.830 · 12321 op/s</sub> | 0.446<br><sub>context: p90 0.532 · p95 0.561 · p99 0.620 · 17386 op/s</sub> | -30.1% (-0.192) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.346<br><sub>context: p90 1.847 · p95 2.014 · p99 2.183 · 731 op/s</sub> | 1.224<br><sub>context: p90 1.687 · p95 1.873 · p99 2.050 · 802 op/s</sub> | -9.0% (-0.122) | 150% AND 2 ms | 🟢 |
| 8 | 1.759<br><sub>context: p90 2.497 · p95 2.699 · p99 3.299 · 4340 op/s</sub> | 1.567<br><sub>context: p90 2.170 · p95 2.394 · p99 2.832 · 5003 op/s</sub> | -10.9% (-0.192) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.391<br><sub>context: p90 0.448 · p95 0.464 · p99 0.486 · 2506 op/s</sub> | 0.350<br><sub>context: p90 0.410 · p95 0.420 · p99 0.456 · 2809 op/s</sub> | -10.3% (-0.040) | 150% AND 2 ms | 🟢 |
| 8 | 0.550<br><sub>context: p90 0.663 · p95 0.695 · p99 0.773 · 14157 op/s</sub> | 0.465<br><sub>context: p90 0.578 · p95 0.608 · p99 0.673 · 16407 op/s</sub> | -15.5% (-0.085) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.172<br><sub>context: p90 2.737 · p95 2.830 · p99 2.999 · 473 op/s</sub> | 0.299<br><sub>context: p90 0.342 · p95 0.365 · p99 0.385 · 3329 op/s</sub> | -86.2% (-1.873) | 150% AND 2 ms | 🟢 |
| 8 | 2.490<br><sub>context: p90 3.237 · p95 3.432 · p99 3.708 · 3198 op/s</sub> | 0.412<br><sub>context: p90 0.524 · p95 0.559 · p99 0.630 · 18534 op/s</sub> | -83.5% (-2.078) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.180<br><sub>context: p90 2.785 · p95 2.865 · p99 2.930 · 470 op/s</sub> | 0.314<br><sub>context: p90 0.367 · p95 0.379 · p99 0.392 · 3123 op/s</sub> | -85.6% (-1.866) | 150% AND 2 ms | 🟢 |
| 8 | 2.531<br><sub>context: p90 3.300 · p95 3.435 · p99 3.717 · 3182 op/s</sub> | 0.433<br><sub>context: p90 0.544 · p95 0.572 · p99 0.656 · 17845 op/s</sub> | -82.9% (-2.098) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.238 · p95 0.246 · p99 0.304 · 4876 op/s</sub> | 0.186<br><sub>context: p90 0.223 · p95 0.240 · p99 0.281 · 5105 op/s</sub> | -4.1% (-0.008) | 150% AND 2 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.369 · p95 0.397 · p99 0.449 · 25829 op/s</sub> | 0.298<br><sub>context: p90 0.403 · p95 0.438 · p99 0.519 · 25373 op/s</sub> | +0.7% (+0.002) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.310<br><sub>context: p90 0.344 · p95 0.362 · p99 0.405 · 3199 op/s</sub> | 0.159<br><sub>context: p90 0.182 · p95 0.187 · p99 0.193 · 6117 op/s</sub> | -48.6% (-0.150) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.409<br><sub>context: p90 0.486 · p95 0.512 · p99 0.562 · 18972 op/s</sub> | 0.243<br><sub>context: p90 0.313 · p95 0.339 · p99 0.385 · 31197 op/s</sub> | -40.6% (-0.166) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.304<br><sub>context: p90 0.342 · p95 0.356 · p99 0.379 · 3203 op/s</sub> | 0.215<br><sub>context: p90 0.265 · p95 0.279 · p99 0.328 · 4459 op/s</sub> | -29.3% (-0.089) | 150% AND 2 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.524 · p95 0.553 · p99 0.613 · 17741 op/s</sub> | 0.368<br><sub>context: p90 0.482 · p95 0.514 · p99 0.599 · 20598 op/s</sub> | -15.0% (-0.065) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.307<br><sub>context: p90 0.349 · p95 0.356 · p99 0.400 · 3104 op/s</sub> | 0.237<br><sub>context: p90 0.310 · p95 0.326 · p99 0.347 · 3989 op/s</sub> | -22.8% (-0.070) | 150% AND 2 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.522 · p95 0.555 · p99 0.628 · 17822 op/s</sub> | 0.372<br><sub>context: p90 0.480 · p95 0.519 · p99 0.595 · 20581 op/s</sub> | -14.5% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.271<br><sub>context: p90 0.314 · p95 0.318 · p99 0.334 · 3646 op/s</sub> | 0.194<br><sub>context: p90 0.228 · p95 0.236 · p99 0.256 · 4972 op/s</sub> | -28.5% (-0.077) | 150% AND 2 ms | 🟢 |
| 8 | 0.390<br><sub>context: p90 0.469 · p95 0.499 · p99 0.557 · 19768 op/s</sub> | 0.309<br><sub>context: p90 0.399 · p95 0.430 · p99 0.496 · 24747 op/s</sub> | -20.8% (-0.081) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.351<br><sub>context: p90 1.457 · p95 1.537 · p99 1.603 · 735 op/s</sub> | 0.931<br><sub>context: p90 1.009 · p95 1.036 · p99 1.087 · 1060 op/s</sub> | -31.1% (-0.420) | 150% AND 2 ms | 🟢 |
| 8 | 1.789<br><sub>context: p90 2.156 · p95 2.280 · p99 2.623 · 4340 op/s</sub> | 1.181<br><sub>context: p90 1.348 · p95 1.400 · p99 1.517 · 6663 op/s</sub> | -34.0% (-0.608) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.232<br><sub>context: p90 1.330 · p95 1.486 · p99 1.513 · 801 op/s</sub> | 0.949<br><sub>context: p90 1.004 · p95 1.032 · p99 1.082 · 1042 op/s</sub> | -22.9% (-0.283) | 150% AND 2 ms | 🟢 |
| 8 | 1.593<br><sub>context: p90 1.939 · p95 2.042 · p99 2.508 · 4824 op/s</sub> | 1.104<br><sub>context: p90 1.236 · p95 1.281 · p99 1.405 · 7200 op/s</sub> | -30.7% (-0.489) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.338<br><sub>context: p90 0.377 · p95 0.393 · p99 0.404 · 2920 op/s</sub> | 0.228<br><sub>context: p90 0.268 · p95 0.280 · p99 0.313 · 4281 op/s</sub> | -32.5% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.560 · p95 0.592 · p99 0.651 · 16757 op/s</sub> | 0.374<br><sub>context: p90 0.470 · p95 0.505 · p99 0.571 · 20608 op/s</sub> | -19.7% (-0.092) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.426<br><sub>context: p90 0.490 · p95 0.506 · p99 0.566 · 2292 op/s</sub> | 0.382<br><sub>context: p90 0.453 · p95 0.468 · p99 0.546 · 2566 op/s</sub> | -10.3% (-0.044) | 150% AND 2 ms | 🟢 |
| 8 | 0.579<br><sub>context: p90 0.693 · p95 0.726 · p99 0.794 · 13588 op/s</sub> | 0.464<br><sub>context: p90 0.581 · p95 0.614 · p99 0.696 · 16674 op/s</sub> | -19.8% (-0.114) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.289<br><sub>context: p90 0.359 · p95 0.382 · p99 0.399 · 3262 op/s</sub> | 0.178<br><sub>context: p90 0.220 · p95 0.230 · p99 0.244 · 5454 op/s</sub> | -38.5% (-0.111) | 150% AND 2 ms | 🟢 |
| 8 | 0.404<br><sub>context: p90 0.495 · p95 0.525 · p99 0.583 · 19063 op/s</sub> | 0.314<br><sub>context: p90 0.407 · p95 0.436 · p99 0.486 · 24168 op/s</sub> | -22.4% (-0.091) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.209<br><sub>context: p90 0.248 · p95 0.260 · p99 0.277 · 4603 op/s</sub> | 0.173<br><sub>context: p90 0.221 · p95 0.245 · p99 0.301 · 5480 op/s</sub> | -17.0% (-0.036) | 150% AND 2 ms | 🟢 |
| 8 | 0.289<br><sub>context: p90 0.353 · p95 0.373 · p99 0.418 · 26748 op/s</sub> | 0.295<br><sub>context: p90 0.391 · p95 0.429 · p99 0.505 · 25642 op/s</sub> | +2.1% (+0.006) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.218 · p95 0.236 · p99 0.249 · 5224 op/s</sub> | 0.177<br><sub>context: p90 0.223 · p95 0.240 · p99 0.256 · 5533 op/s</sub> | -4.9% (-0.009) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.361 · p95 0.378 · p99 0.426 · 26211 op/s</sub> | 0.297<br><sub>context: p90 0.399 · p95 0.431 · p99 0.504 · 25619 op/s</sub> | +1.3% (+0.004) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.950<br><sub>context: p90 0.996 · p95 1.016 · p99 1.051 · 1040 op/s</sub> | 0.362<br><sub>context: p90 0.384 · p95 0.395 · p99 0.417 · 2701 op/s</sub> | -61.9% (-0.588) | 150% AND 2 ms | 🟢 |
| 8 | 1.272<br><sub>context: p90 1.732 · p95 1.936 · p99 2.218 · 5900 op/s</sub> | 0.446<br><sub>context: p90 0.518 · p95 0.542 · p99 0.598 · 17588 op/s</sub> | -65.0% (-0.826) | 150% AND 2 ms | 🟢 |

</details>
