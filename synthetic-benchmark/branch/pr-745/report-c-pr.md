### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:d4f9dbb30b9f70e6d965d2dce47b239c27fb238fd4d3a272d1434cd7762414f7 |
| workload_hash | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` |
| samples / warmup | 200 / 50 | 200 / 50 |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs c-engine** — ⚠ pass, 1 diverged — no p50 regression beyond budget across 98 comparable cell(s); divergence is advisory under this policy

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:d4f9dbb30b9f70e6d965d2dce47b239c27fb238fd4d3a272d1434cd7762414f7

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.378<br><sub>context: p90 1.467 · p95 1.510 · p99 1.548 · 722 op/s</sub> | 0.779<br><sub>context: p90 0.803 · p95 0.818 · p99 0.831 · 1272 op/s</sub> | -43.5% (-0.599) | 150% AND 2 ms | 🟢 |
| 8 | 1.887<br><sub>context: p90 2.408 · p95 2.598 · p99 2.965 · 4116 op/s</sub> | 0.912<br><sub>context: p90 1.038 · p95 1.069 · p99 1.143 · 8612 op/s</sub> | -51.7% (-0.975) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.047<br><sub>context: p90 2.128 · p95 2.150 · p99 2.242 · 486 op/s</sub> | 0.855<br><sub>context: p90 0.934 · p95 0.946 · p99 0.961 · 1151 op/s</sub> | -58.3% (-1.193) | 150% AND 2 ms | 🟢 |
| 8 | 2.614<br><sub>context: p90 3.406 · p95 3.492 · p99 3.821 · 2904 op/s</sub> | 1.093<br><sub>context: p90 1.292 · p95 1.325 · p99 1.381 · 7198 op/s</sub> | -58.2% (-1.521) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.016<br><sub>context: p90 2.134 · p95 2.186 · p99 2.249 · 493 op/s</sub> | 1.291<br><sub>context: p90 1.339 · p95 1.355 · p99 1.377 · 765 op/s</sub> | -36.0% (-0.725) | 150% AND 2 ms | 🟢 |
| 8 | 2.685<br><sub>context: p90 3.467 · p95 3.643 · p99 4.073 · 2867 op/s</sub> | 1.771<br><sub>context: p90 2.254 · p95 2.454 · p99 2.763 · 4332 op/s</sub> | -34.0% (-0.914) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.653<br><sub>context: p90 2.766 · p95 2.800 · p99 2.850 · 375 op/s</sub> | 1.410<br><sub>context: p90 1.485 · p95 1.554 · p99 1.592 · 701 op/s</sub> | -46.9% (-1.243) | 150% AND 2 ms | 🟢 |
| 8 | 3.641<br><sub>context: p90 4.716 · p95 4.952 · p99 5.465 · 2115 op/s</sub> | 1.643<br><sub>context: p90 1.967 · p95 2.011 · p99 2.096 · 4718 op/s</sub> | -54.9% (-1.999) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.142<br><sub>context: p90 0.173 · p95 0.178 · p99 0.193 · 6633 op/s</sub> | 0.142<br><sub>context: p90 0.171 · p95 0.183 · p99 0.213 · 6530 op/s</sub> | +0.1% (+0.000) | 150% AND 2 ms | 🟢 |
| 8 | 0.235<br><sub>context: p90 0.295 · p95 0.314 · p99 0.349 · 32269 op/s</sub> | 0.222<br><sub>context: p90 0.287 · p95 0.306 · p99 0.345 · 34971 op/s</sub> | -5.4% (-0.013) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.332<br><sub>context: p90 0.377 · p95 0.395 · p99 0.449 · 2837 op/s</sub> | 0.212<br><sub>context: p90 0.258 · p95 0.280 · p99 0.291 · 4574 op/s</sub> | -36.1% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.514 · p95 0.545 · p99 0.626 · 17972 op/s</sub> | 0.324<br><sub>context: p90 0.411 · p95 0.437 · p99 0.501 · 23537 op/s</sub> | -25.0% (-0.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.353<br><sub>context: p90 0.388 · p95 0.397 · p99 0.419 · 2798 op/s</sub> | 0.276<br><sub>context: p90 0.369 · p95 0.387 · p99 0.424 · 3485 op/s</sub> | -21.6% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.470<br><sub>context: p90 0.562 · p95 0.588 · p99 0.655 · 16622 op/s</sub> | 0.349<br><sub>context: p90 0.433 · p95 0.458 · p99 0.531 · 21914 op/s</sub> | -25.8% (-0.121) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.392<br><sub>context: p90 0.443 · p95 0.453 · p99 0.488 · 2506 op/s</sub> | 0.344<br><sub>context: p90 0.397 · p95 0.417 · p99 0.472 · 2927 op/s</sub> | -12.3% (-0.048) | 150% AND 2 ms | 🟢 |
| 8 | 0.519<br><sub>context: p90 0.615 · p95 0.646 · p99 0.725 · 14940 op/s</sub> | 0.442<br><sub>context: p90 0.541 · p95 0.570 · p99 0.636 · 17557 op/s</sub> | -14.8% (-0.077) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.467<br><sub>context: p90 0.560 · p95 0.584 · p99 0.630 · 2092 op/s</sub> | 0.355<br><sub>context: p90 0.408 · p95 0.418 · p99 0.441 · 2784 op/s</sub> | -24.0% (-0.112) | 150% AND 2 ms | 🟢 |
| 8 | 0.560<br><sub>context: p90 0.664 · p95 0.696 · p99 0.784 · 13905 op/s</sub> | 0.471<br><sub>context: p90 0.579 · p95 0.610 · p99 0.675 · 16427 op/s</sub> | -15.9% (-0.089) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.553<br><sub>context: p90 0.676 · p95 0.750 · p99 0.871 · 1781 op/s</sub> | 0.466<br><sub>context: p90 0.562 · p95 0.587 · p99 0.687 · 2109 op/s</sub> | -15.6% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.742<br><sub>context: p90 0.931 · p95 1.001 · p99 1.134 · 10366 op/s</sub> | 0.619<br><sub>context: p90 0.788 · p95 0.839 · p99 0.945 · 12533 op/s</sub> | -16.5% (-0.123) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.590<br><sub>context: p90 0.728 · p95 0.791 · p99 0.881 · 1665 op/s</sub> | 0.512<br><sub>context: p90 0.633 · p95 0.693 · p99 0.765 · 1899 op/s</sub> | -13.1% (-0.077) | 150% AND 2 ms | 🟢 |
| 8 | 0.778<br><sub>context: p90 0.981 · p95 1.047 · p99 1.180 · 9987 op/s</sub> | 0.649<br><sub>context: p90 0.822 · p95 0.888 · p99 0.980 · 11908 op/s</sub> | -16.6% (-0.129) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.203<br><sub>context: p90 1.702 · p95 1.810 · p99 2.135 · 808 op/s</sub> | 0.979<br><sub>context: p90 1.271 · p95 1.444 · p99 1.663 · 1015 op/s</sub> | -18.6% (-0.224) | 150% AND 2 ms | 🟢 |
| 8 | 1.564<br><sub>context: p90 2.216 · p95 2.402 · p99 2.862 · 4907 op/s</sub> | 1.234<br><sub>context: p90 1.739 · p95 1.896 · p99 2.193 · 6233 op/s</sub> | -21.1% (-0.330) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.274<br><sub>context: p90 1.724 · p95 1.980 · p99 2.129 · 756 op/s</sub> | 0.986<br><sub>context: p90 1.308 · p95 1.475 · p99 1.713 · 987 op/s</sub> | -22.6% (-0.288) | 150% AND 2 ms | 🟢 |
| 8 | 1.611<br><sub>context: p90 2.255 · p95 2.458 · p99 2.887 · 4729 op/s</sub> | 1.272<br><sub>context: p90 1.721 · p95 1.928 · p99 2.225 · 6000 op/s</sub> | -21.0% (-0.339) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.606<br><sub>context: p90 0.700 · p95 0.728 · p99 0.814 · 1632 op/s</sub> | 0.549<br><sub>context: p90 0.706 · p95 0.740 · p99 0.792 · 1785 op/s</sub> | -9.5% (-0.058) | 150% AND 2 ms | 🟢 |
| 8 | 0.776<br><sub>context: p90 0.911 · p95 0.954 · p99 1.051 · 10147 op/s</sub> | 0.737<br><sub>context: p90 0.968 · p95 1.043 · p99 1.200 · 10559 op/s</sub> | -5.1% (-0.039) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.409<br><sub>context: p90 0.483 · p95 0.506 · p99 0.534 · 2372 op/s</sub> | 0.299<br><sub>context: p90 0.353 · p95 0.378 · p99 0.413 · 3261 op/s</sub> | -26.8% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.547<br><sub>context: p90 0.654 · p95 0.702 · p99 0.783 · 14146 op/s</sub> | 0.402<br><sub>context: p90 0.488 · p95 0.513 · p99 0.569 · 19240 op/s</sub> | -26.5% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.275 · p95 0.284 · p99 0.310 · 4240 op/s</sub> | 0.153<br><sub>context: p90 0.169 · p95 0.171 · p99 0.180 · 6398 op/s</sub> | -30.9% (-0.069) | 150% AND 2 ms | 🟢 |
| 8 | 0.322<br><sub>context: p90 0.397 · p95 0.425 · p99 0.480 · 23658 op/s</sub> | 0.221<br><sub>context: p90 0.283 · p95 0.301 · p99 0.342 · 34609 op/s</sub> | -31.3% (-0.101) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.201 · p95 0.207 · p99 0.216 · 5514 op/s</sub> | 0.112<br><sub>context: p90 0.134 · p95 0.137 · p99 0.143 · 8324 op/s</sub> | -36.5% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.337 · p95 0.359 · p99 0.407 · 28537 op/s</sub> | 0.201<br><sub>context: p90 0.271 · p95 0.292 · p99 0.340 · 37981 op/s</sub> | -24.7% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.483<br><sub>context: p90 0.535 · p95 0.549 · p99 0.585 · 2027 op/s</sub> | 0.319<br><sub>context: p90 0.382 · p95 0.398 · p99 0.434 · 3043 op/s</sub> | -34.1% (-0.165) | 150% AND 2 ms | 🟢 |
| 8 | 0.662<br><sub>context: p90 0.786 · p95 0.837 · p99 0.971 · 11635 op/s</sub> | 0.442<br><sub>context: p90 0.539 · p95 0.571 · p99 0.618 · 17538 op/s</sub> | -33.2% (-0.220) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.989<br><sub>context: p90 1.414 · p95 1.534 · p99 1.750 · 969 op/s</sub> | 1.196<br><sub>context: p90 1.798 · p95 2.000 · p99 2.208 · 803 op/s</sub> | +20.9% (+0.207) | 150% AND 2 ms | 🟢 |
| 8 | 1.344<br><sub>context: p90 2.070 · p95 2.365 · p99 2.900 · 5524 op/s</sub> | 2.094<br><sub>context: p90 3.206 · p95 3.618 · p99 4.176 · 3674 op/s</sub> | +55.8% (+0.750) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.315<br><sub>context: p90 5.359 · p95 6.039 · p99 6.645 · 287 op/s</sub> | 4.366<br><sub>context: p90 6.619 · p95 7.337 · p99 7.848 · 220 op/s</sub> | +31.7% (+1.052) | 150% AND 2 ms | 🟢 |
| 8 | 4.472<br><sub>context: p90 7.937 · p95 8.855 · p99 11.395 · 1631 op/s</sub> | 8.006<br><sub>context: p90 12.328 · p95 13.497 · p99 15.403 · 970 op/s</sub> | +79.0% (+3.534) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.242<br><sub>context: p90 0.288 · p95 0.307 · p99 0.352 · 3983 op/s</sub> | 0.167<br><sub>context: p90 0.187 · p95 0.194 · p99 0.208 · 5875 op/s</sub> | -31.2% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.357<br><sub>context: p90 0.436 · p95 0.455 · p99 0.517 · 21644 op/s</sub> | 0.296<br><sub>context: p90 0.395 · p95 0.436 · p99 0.510 · 25379 op/s</sub> | -17.1% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.256<br><sub>context: p90 0.322 · p95 0.351 · p99 0.386 · 3764 op/s</sub> | 0.154<br><sub>context: p90 0.194 · p95 0.210 · p99 0.252 · 5882 op/s</sub> | -39.8% (-0.102) | 150% AND 2 ms | 🟢 |
| 8 | 0.358<br><sub>context: p90 0.433 · p95 0.459 · p99 0.505 · 21576 op/s</sub> | 0.301<br><sub>context: p90 0.407 · p95 0.453 · p99 0.518 · 25092 op/s</sub> | -15.9% (-0.057) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.342<br><sub>context: p90 0.423 · p95 0.462 · p99 0.528 · 2797 op/s</sub> | 0.217<br><sub>context: p90 0.258 · p95 0.264 · p99 0.272 · 4361 op/s</sub> | -36.7% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.478<br><sub>context: p90 0.600 · p95 0.639 · p99 0.711 · 15946 op/s</sub> | 0.356<br><sub>context: p90 0.456 · p95 0.498 · p99 0.570 · 21365 op/s</sub> | -25.6% (-0.122) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.195<br><sub>context: p90 0.247 · p95 0.278 · p99 0.337 · 4792 op/s</sub> | 0.132<br><sub>context: p90 0.162 · p95 0.170 · p99 0.180 · 6853 op/s</sub> | -32.1% (-0.062) | 150% AND 2 ms | 🟢 |
| 8 | 0.284<br><sub>context: p90 0.348 · p95 0.374 · p99 0.419 · 26987 op/s</sub> | 0.218<br><sub>context: p90 0.286 · p95 0.304 · p99 0.337 · 35527 op/s</sub> | -23.4% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.345<br><sub>context: p90 0.374 · p95 0.387 · p99 0.407 · 2819 op/s</sub> | 0.334<br><sub>context: p90 0.400 · p95 0.430 · p99 0.474 · 2908 op/s</sub> | -3.1% (-0.011) | 150% AND 2 ms | 🟢 |
| 8 | 0.445<br><sub>context: p90 0.518 · p95 0.543 · p99 0.594 · 17564 op/s</sub> | 0.389<br><sub>context: p90 0.477 · p95 0.505 · p99 0.565 · 19813 op/s</sub> | -12.4% (-0.055) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.284 · p95 0.319 · p99 0.331 · 4014 op/s</sub> | 0.236<br><sub>context: p90 0.318 · p95 0.338 · p99 0.376 · 4018 op/s</sub> | -1.3% (-0.003) | 150% AND 2 ms | 🟢 |
| 8 | 0.373<br><sub>context: p90 0.443 · p95 0.470 · p99 0.510 · 20784 op/s</sub> | 0.329<br><sub>context: p90 0.407 · p95 0.432 · p99 0.479 · 23169 op/s</sub> | -11.8% (-0.044) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.383<br><sub>context: p90 0.433 · p95 0.447 · p99 0.480 · 2564 op/s</sub> | 0.347<br><sub>context: p90 0.412 · p95 0.445 · p99 0.486 · 2800 op/s</sub> | -9.3% (-0.036) | 150% AND 2 ms | 🟢 |
| 8 | 0.502<br><sub>context: p90 0.600 · p95 0.627 · p99 0.700 · 15503 op/s</sub> | 0.435<br><sub>context: p90 0.540 · p95 0.576 · p99 0.617 · 17711 op/s</sub> | -13.3% (-0.067) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.463<br><sub>context: p90 0.563 · p95 0.601 · p99 0.678 · 2094 op/s</sub> | 0.481<br><sub>context: p90 0.612 · p95 0.663 · p99 0.731 · 2031 op/s</sub> | +3.9% (+0.018) | 150% AND 2 ms | 🟢 |
| 8 | 0.649<br><sub>context: p90 0.819 · p95 0.873 · p99 1.006 · 11924 op/s</sub> | 0.600<br><sub>context: p90 0.768 · p95 0.821 · p99 0.933 · 12866 op/s</sub> | -7.6% (-0.049) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.476<br><sub>context: p90 0.553 · p95 0.580 · p99 0.634 · 2075 op/s</sub> | 0.504<br><sub>context: p90 0.617 · p95 0.659 · p99 0.737 · 1941 op/s</sub> | +5.9% (+0.028) | 150% AND 2 ms | 🟢 |
| 8 | 0.669<br><sub>context: p90 0.822 · p95 0.875 · p99 0.987 · 11671 op/s</sub> | 0.609<br><sub>context: p90 0.765 · p95 0.821 · p99 0.943 · 12694 op/s</sub> | -8.9% (-0.059) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.418<br><sub>context: p90 0.473 · p95 0.500 · p99 0.566 · 2345 op/s</sub> | 0.364<br><sub>context: p90 0.438 · p95 0.464 · p99 0.512 · 2673 op/s</sub> | -12.9% (-0.054) | 150% AND 2 ms | 🟢 |
| 8 | 0.544<br><sub>context: p90 0.643 · p95 0.682 · p99 0.757 · 14331 op/s</sub> | 0.454<br><sub>context: p90 0.551 · p95 0.585 · p99 0.647 · 16956 op/s</sub> | -16.6% (-0.091) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.400<br><sub>context: p90 0.465 · p95 0.487 · p99 0.536 · 2402 op/s</sub> | 0.287<br><sub>context: p90 0.364 · p95 0.391 · p99 0.417 · 3335 op/s</sub> | -28.3% (-0.113) | 150% AND 2 ms | 🟢 |
| 8 | 0.538<br><sub>context: p90 0.635 · p95 0.669 · p99 0.749 · 14413 op/s</sub> | 0.375<br><sub>context: p90 0.472 · p95 0.500 · p99 0.559 · 20410 op/s</sub> | -30.2% (-0.163) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.209<br><sub>context: p90 18.394 · p95 18.454 · p99 18.529 · 55 op/s</sub> | 15.074<br><sub>context: p90 15.286 · p95 15.371 · p99 15.454 · 66 op/s</sub> | -17.2% (-3.135) | 150% AND 2 ms | 🟢 |
| 8 | 22.795<br><sub>context: p90 31.252 · p95 34.476 · p99 38.839 · 320 op/s</sub> | 18.118<br><sub>context: p90 23.194 · p95 25.893 · p99 29.209 · 390 op/s</sub> | -20.5% (-4.677) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.472<br><sub>context: p90 0.521 · p95 0.542 · p99 0.574 · 2091 op/s</sub> | 0.368<br><sub>context: p90 0.455 · p95 0.484 · p99 0.548 · 2611 op/s</sub> | -22.0% (-0.104) | 150% AND 2 ms | 🟢 |
| 8 | 0.639<br><sub>context: p90 0.736 · p95 0.771 · p99 0.830 · 12321 op/s</sub> | 0.448<br><sub>context: p90 0.542 · p95 0.576 · p99 0.640 · 17313 op/s</sub> | -29.8% (-0.190) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.346<br><sub>context: p90 1.847 · p95 2.014 · p99 2.183 · 731 op/s</sub> | 1.211<br><sub>context: p90 1.652 · p95 1.798 · p99 1.958 · 809 op/s</sub> | -10.0% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 1.759<br><sub>context: p90 2.497 · p95 2.699 · p99 3.299 · 4340 op/s</sub> | 1.539<br><sub>context: p90 2.180 · p95 2.394 · p99 2.840 · 4968 op/s</sub> | -12.5% (-0.220) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.391<br><sub>context: p90 0.448 · p95 0.464 · p99 0.486 · 2506 op/s</sub> | 0.360<br><sub>context: p90 0.424 · p95 0.457 · p99 0.482 · 2717 op/s</sub> | -7.9% (-0.031) | 150% AND 2 ms | 🟢 |
| 8 | 0.550<br><sub>context: p90 0.663 · p95 0.695 · p99 0.773 · 14157 op/s</sub> | 0.473<br><sub>context: p90 0.590 · p95 0.622 · p99 0.709 · 16276 op/s</sub> | -14.0% (-0.077) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.172<br><sub>context: p90 2.737 · p95 2.830 · p99 2.999 · 473 op/s</sub> | 0.339<br><sub>context: p90 0.403 · p95 0.425 · p99 0.463 · 2865 op/s</sub> | -84.4% (-1.833) | 150% AND 2 ms | 🟢 |
| 8 | 2.490<br><sub>context: p90 3.237 · p95 3.432 · p99 3.708 · 3198 op/s</sub> | 0.416<br><sub>context: p90 0.523 · p95 0.558 · p99 0.629 · 18463 op/s</sub> | -83.3% (-2.074) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.180<br><sub>context: p90 2.785 · p95 2.865 · p99 2.930 · 470 op/s</sub> | 0.346<br><sub>context: p90 0.417 · p95 0.448 · p99 0.487 · 2782 op/s</sub> | -84.1% (-1.835) | 150% AND 2 ms | 🟢 |
| 8 | 2.531<br><sub>context: p90 3.300 · p95 3.435 · p99 3.717 · 3182 op/s</sub> | 0.436<br><sub>context: p90 0.549 · p95 0.580 · p99 0.666 · 17516 op/s</sub> | -82.8% (-2.094) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.238 · p95 0.246 · p99 0.304 · 4876 op/s</sub> | 0.177<br><sub>context: p90 0.221 · p95 0.246 · p99 0.278 · 5298 op/s</sub> | -8.3% (-0.016) | 150% AND 2 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.369 · p95 0.397 · p99 0.449 · 25829 op/s</sub> | 0.302<br><sub>context: p90 0.407 · p95 0.440 · p99 0.506 · 25240 op/s</sub> | +2.0% (+0.006) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.310<br><sub>context: p90 0.344 · p95 0.362 · p99 0.405 · 3199 op/s</sub> | 0.146<br><sub>context: p90 0.186 · p95 0.199 · p99 0.215 · 6279 op/s</sub> | -52.7% (-0.163) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.409<br><sub>context: p90 0.486 · p95 0.512 · p99 0.562 · 18972 op/s</sub> | 0.241<br><sub>context: p90 0.305 · p95 0.325 · p99 0.368 · 31726 op/s</sub> | -41.0% (-0.168) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.304<br><sub>context: p90 0.342 · p95 0.356 · p99 0.379 · 3203 op/s</sub> | 0.240<br><sub>context: p90 0.293 · p95 0.304 · p99 0.337 · 3562 op/s</sub> | -21.1% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.524 · p95 0.553 · p99 0.613 · 17741 op/s</sub> | 0.365<br><sub>context: p90 0.476 · p95 0.518 · p99 0.600 · 21027 op/s</sub> | -15.8% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.307<br><sub>context: p90 0.349 · p95 0.356 · p99 0.400 · 3104 op/s</sub> | 0.240<br><sub>context: p90 0.314 · p95 0.327 · p99 0.358 · 3942 op/s</sub> | -21.8% (-0.067) | 150% AND 2 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.522 · p95 0.555 · p99 0.628 · 17822 op/s</sub> | 0.368<br><sub>context: p90 0.480 · p95 0.522 · p99 0.607 · 20396 op/s</sub> | -15.5% (-0.067) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.271<br><sub>context: p90 0.314 · p95 0.318 · p99 0.334 · 3646 op/s</sub> | 0.202<br><sub>context: p90 0.277 · p95 0.312 · p99 0.345 · 4580 op/s</sub> | -25.5% (-0.069) | 150% AND 2 ms | 🟢 |
| 8 | 0.390<br><sub>context: p90 0.469 · p95 0.499 · p99 0.557 · 19768 op/s</sub> | 0.307<br><sub>context: p90 0.401 · p95 0.433 · p99 0.495 · 24628 op/s</sub> | -21.3% (-0.083) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.351<br><sub>context: p90 1.457 · p95 1.537 · p99 1.603 · 735 op/s</sub> | 0.958<br><sub>context: p90 1.039 · p95 1.076 · p99 1.219 · 1023 op/s</sub> | -29.1% (-0.392) | 150% AND 2 ms | 🟢 |
| 8 | 1.789<br><sub>context: p90 2.156 · p95 2.280 · p99 2.623 · 4340 op/s</sub> | 1.190<br><sub>context: p90 1.346 · p95 1.396 · p99 1.512 · 6605 op/s</sub> | -33.5% (-0.600) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.232<br><sub>context: p90 1.330 · p95 1.486 · p99 1.513 · 801 op/s</sub> | 0.954<br><sub>context: p90 1.029 · p95 1.051 · p99 1.091 · 1035 op/s</sub> | -22.5% (-0.278) | 150% AND 2 ms | 🟢 |
| 8 | 1.593<br><sub>context: p90 1.939 · p95 2.042 · p99 2.508 · 4824 op/s</sub> | 1.095<br><sub>context: p90 1.225 · p95 1.259 · p99 1.318 · 7232 op/s</sub> | -31.2% (-0.498) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.338<br><sub>context: p90 0.377 · p95 0.393 · p99 0.404 · 2920 op/s</sub> | 0.251<br><sub>context: p90 0.347 · p95 0.361 · p99 0.410 · 3713 op/s</sub> | -25.8% (-0.087) | 150% AND 2 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.560 · p95 0.592 · p99 0.651 · 16757 op/s</sub> | 0.380<br><sub>context: p90 0.470 · p95 0.506 · p99 0.567 · 20575 op/s</sub> | -18.4% (-0.086) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.426<br><sub>context: p90 0.490 · p95 0.506 · p99 0.566 · 2292 op/s</sub> | 0.360<br><sub>context: p90 0.435 · p95 0.453 · p99 0.487 · 2728 op/s</sub> | -15.6% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.579<br><sub>context: p90 0.693 · p95 0.726 · p99 0.794 · 13588 op/s</sub> | 0.458<br><sub>context: p90 0.570 · p95 0.608 · p99 0.673 · 16918 op/s</sub> | -20.8% (-0.121) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.289<br><sub>context: p90 0.359 · p95 0.382 · p99 0.399 · 3262 op/s</sub> | 0.182<br><sub>context: p90 0.229 · p95 0.249 · p99 0.286 · 5194 op/s</sub> | -37.0% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.404<br><sub>context: p90 0.495 · p95 0.525 · p99 0.583 · 19063 op/s</sub> | 0.317<br><sub>context: p90 0.407 · p95 0.438 · p99 0.500 · 24008 op/s</sub> | -21.5% (-0.087) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.209<br><sub>context: p90 0.248 · p95 0.260 · p99 0.277 · 4603 op/s</sub> | 0.169<br><sub>context: p90 0.205 · p95 0.213 · p99 0.281 · 5567 op/s</sub> | -19.1% (-0.040) | 150% AND 2 ms | 🟢 |
| 8 | 0.289<br><sub>context: p90 0.353 · p95 0.373 · p99 0.418 · 26748 op/s</sub> | 0.297<br><sub>context: p90 0.395 · p95 0.425 · p99 0.503 · 25613 op/s</sub> | +2.5% (+0.007) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.218 · p95 0.236 · p99 0.249 · 5224 op/s</sub> | 0.177<br><sub>context: p90 0.210 · p95 0.217 · p99 0.256 · 5359 op/s</sub> | -5.0% (-0.009) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.361 · p95 0.378 · p99 0.426 · 26211 op/s</sub> | 0.296<br><sub>context: p90 0.396 · p95 0.434 · p99 0.506 · 25405 op/s</sub> | +0.9% (+0.003) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.950<br><sub>context: p90 0.996 · p95 1.016 · p99 1.051 · 1040 op/s</sub> | 0.391<br><sub>context: p90 0.444 · p95 0.470 · p99 0.498 · 2498 op/s</sub> | -58.8% (-0.559) | 150% AND 2 ms | 🟢 |
| 8 | 1.272<br><sub>context: p90 1.732 · p95 1.936 · p99 2.218 · 5900 op/s</sub> | 0.440<br><sub>context: p90 0.510 · p95 0.529 · p99 0.571 · 17760 op/s</sub> | -65.4% (-0.832) | 150% AND 2 ms | 🟢 |

</details>
