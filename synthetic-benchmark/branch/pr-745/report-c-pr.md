### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:1328197de8d3dfbb87a1597dc2909530990d7a80ef4a845562899a1b5dc02497 |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:1328197de8d3dfbb87a1597dc2909530990d7a80ef4a845562899a1b5dc02497

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.583<br><sub>context: p90 1.662 · p95 1.685 · p99 1.731 · 628 op/s</sub> | 0.985<br><sub>context: p90 1.066 · p95 1.089 · p99 1.121 · 1004 op/s</sub> | -37.7% (-0.597) | 150% AND 2 ms | 🟢 |
| 8 | 1.930<br><sub>context: p90 2.493 · p95 2.699 · p99 3.078 · 3947 op/s</sub> | 0.963<br><sub>context: p90 1.102 · p95 1.147 · p99 1.231 · 8167 op/s</sub> | -50.1% (-0.967) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.260<br><sub>context: p90 2.377 · p95 2.411 · p99 2.496 · 437 op/s</sub> | 1.017<br><sub>context: p90 1.134 · p95 1.158 · p99 1.200 · 966 op/s</sub> | -55.0% (-1.242) | 150% AND 2 ms | 🟢 |
| 8 | 2.776<br><sub>context: p90 3.571 · p95 3.727 · p99 4.071 · 2771 op/s</sub> | 1.125<br><sub>context: p90 1.362 · p95 1.413 · p99 1.494 · 6910 op/s</sub> | -59.5% (-1.651) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.220<br><sub>context: p90 2.305 · p95 2.341 · p99 2.435 · 446 op/s</sub> | 1.516<br><sub>context: p90 1.605 · p95 1.654 · p99 1.696 · 653 op/s</sub> | -31.7% (-0.704) | 150% AND 2 ms | 🟢 |
| 8 | 2.862<br><sub>context: p90 3.608 · p95 3.812 · p99 4.268 · 2721 op/s</sub> | 1.821<br><sub>context: p90 2.323 · p95 2.515 · p99 2.861 · 4251 op/s</sub> | -36.4% (-1.041) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.874<br><sub>context: p90 2.959 · p95 2.987 · p99 3.025 · 346 op/s</sub> | 1.571<br><sub>context: p90 1.669 · p95 1.695 · p99 1.772 · 631 op/s</sub> | -45.3% (-1.303) | 150% AND 2 ms | 🟢 |
| 8 | 3.817<br><sub>context: p90 4.819 · p95 5.077 · p99 5.536 · 2059 op/s</sub> | 1.710<br><sub>context: p90 2.033 · p95 2.099 · p99 2.220 · 4526 op/s</sub> | -55.2% (-2.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.279<br><sub>context: p90 0.413 · p95 0.453 · p99 0.532 · 3283 op/s</sub> | 0.169<br><sub>context: p90 0.238 · p95 0.271 · p99 0.307 · 5507 op/s</sub> | -39.5% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.285<br><sub>context: p90 0.422 · p95 0.466 · p99 0.560 · 23908 op/s</sub> | 0.228<br><sub>context: p90 0.319 · p95 0.350 · p99 0.449 · 27175 op/s</sub> | -20.0% (-0.057) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.577<br><sub>context: p90 0.685 · p95 0.705 · p99 0.740 · 1699 op/s</sub> | 0.398<br><sub>context: p90 0.494 · p95 0.532 · p99 0.571 · 2478 op/s</sub> | -31.0% (-0.179) | 150% AND 2 ms | 🟢 |
| 8 | 0.486<br><sub>context: p90 0.621 · p95 0.674 · p99 0.814 · 14423 op/s</sub> | 0.351<br><sub>context: p90 0.468 · p95 0.507 · p99 0.584 · 21317 op/s</sub> | -27.8% (-0.135) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.609<br><sub>context: p90 0.696 · p95 0.722 · p99 0.793 · 1622 op/s</sub> | 0.466<br><sub>context: p90 0.556 · p95 0.583 · p99 0.625 · 2127 op/s</sub> | -23.5% (-0.143) | 150% AND 2 ms | 🟢 |
| 8 | 0.573<br><sub>context: p90 0.782 · p95 0.868 · p99 1.071 · 12755 op/s</sub> | 0.357<br><sub>context: p90 0.454 · p95 0.489 · p99 0.565 · 21359 op/s</sub> | -37.8% (-0.217) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.603<br><sub>context: p90 0.720 · p95 0.743 · p99 0.835 · 1594 op/s</sub> | 0.542<br><sub>context: p90 0.657 · p95 0.688 · p99 0.758 · 1822 op/s</sub> | -10.1% (-0.061) | 150% AND 2 ms | 🟢 |
| 8 | 0.630<br><sub>context: p90 0.834 · p95 0.904 · p99 1.084 · 11747 op/s</sub> | 0.472<br><sub>context: p90 0.579 · p95 0.612 · p99 0.688 · 16217 op/s</sub> | -25.1% (-0.158) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.668<br><sub>context: p90 0.743 · p95 0.771 · p99 0.856 · 1485 op/s</sub> | 0.609<br><sub>context: p90 0.733 · p95 0.764 · p99 0.811 · 1632 op/s</sub> | -8.8% (-0.059) | 150% AND 2 ms | 🟢 |
| 8 | 0.614<br><sub>context: p90 0.762 · p95 0.811 · p99 0.931 · 12454 op/s</sub> | 0.501<br><sub>context: p90 0.614 · p95 0.651 · p99 0.714 · 15395 op/s</sub> | -18.5% (-0.113) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.797<br><sub>context: p90 0.943 · p95 0.987 · p99 1.146 · 1234 op/s</sub> | 0.680<br><sub>context: p90 0.805 · p95 0.832 · p99 0.905 · 1461 op/s</sub> | -14.7% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.819<br><sub>context: p90 1.046 · p95 1.124 · p99 1.273 · 9422 op/s</sub> | 0.663<br><sub>context: p90 0.844 · p95 0.903 · p99 1.007 · 11636 op/s</sub> | -19.0% (-0.156) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.834<br><sub>context: p90 0.986 · p95 1.024 · p99 1.146 · 1184 op/s</sub> | 0.680<br><sub>context: p90 0.810 · p95 0.835 · p99 0.923 · 1460 op/s</sub> | -18.5% (-0.154) | 150% AND 2 ms | 🟢 |
| 8 | 0.835<br><sub>context: p90 1.080 · p95 1.159 · p99 1.336 · 9249 op/s</sub> | 0.693<br><sub>context: p90 0.892 · p95 0.946 · p99 1.078 · 11084 op/s</sub> | -17.0% (-0.142) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.393<br><sub>context: p90 1.844 · p95 1.938 · p99 2.221 · 711 op/s</sub> | 1.198<br><sub>context: p90 1.529 · p95 1.693 · p99 1.890 · 826 op/s</sub> | -14.1% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 1.689<br><sub>context: p90 2.427 · p95 2.659 · p99 3.161 · 4467 op/s</sub> | 1.336<br><sub>context: p90 1.837 · p95 2.000 · p99 2.321 · 5754 op/s</sub> | -20.9% (-0.353) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.493<br><sub>context: p90 1.964 · p95 2.251 · p99 2.665 · 645 op/s</sub> | 1.290<br><sub>context: p90 1.667 · p95 1.791 · p99 2.097 · 750 op/s</sub> | -13.6% (-0.204) | 150% AND 2 ms | 🟢 |
| 8 | 1.675<br><sub>context: p90 2.385 · p95 2.657 · p99 3.107 · 4541 op/s</sub> | 1.377<br><sub>context: p90 1.890 · p95 2.061 · p99 2.370 · 5557 op/s</sub> | -17.8% (-0.298) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.780<br><sub>context: p90 0.909 · p95 0.924 · p99 1.086 · 1260 op/s</sub> | 0.845<br><sub>context: p90 1.058 · p95 1.102 · p99 1.193 · 1184 op/s</sub> | +8.4% (+0.065) | 150% AND 2 ms | 🟢 |
| 8 | 0.813<br><sub>context: p90 0.967 · p95 1.013 · p99 1.149 · 9590 op/s</sub> | 0.797<br><sub>context: p90 1.063 · p95 1.147 · p99 1.272 · 9763 op/s</sub> | -2.0% (-0.017) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.551<br><sub>context: p90 0.638 · p95 0.662 · p99 0.700 · 1786 op/s</sub> | 0.544<br><sub>context: p90 0.631 · p95 0.657 · p99 0.685 · 1834 op/s</sub> | -1.3% (-0.007) | 150% AND 2 ms | 🟢 |
| 8 | 0.567<br><sub>context: p90 0.684 · p95 0.727 · p99 0.825 · 13653 op/s</sub> | 0.429<br><sub>context: p90 0.542 · p95 0.572 · p99 0.646 · 16200 op/s</sub> | -24.2% (-0.137) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.379<br><sub>context: p90 0.483 · p95 0.532 · p99 0.610 · 2517 op/s</sub> | 0.207<br><sub>context: p90 0.302 · p95 0.330 · p99 0.351 · 4472 op/s</sub> | -45.4% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 0.342<br><sub>context: p90 0.426 · p95 0.467 · p99 0.529 · 22009 op/s</sub> | 0.233<br><sub>context: p90 0.297 · p95 0.319 · p99 0.360 · 33392 op/s</sub> | -31.7% (-0.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.219<br><sub>context: p90 0.285 · p95 0.315 · p99 0.350 · 4366 op/s</sub> | 0.142<br><sub>context: p90 0.209 · p95 0.217 · p99 0.227 · 6543 op/s</sub> | -35.3% (-0.077) | 150% AND 2 ms | 🟢 |
| 8 | 0.272<br><sub>context: p90 0.342 · p95 0.369 · p99 0.425 · 27997 op/s</sub> | 0.209<br><sub>context: p90 0.293 · p95 0.318 · p99 0.411 · 36065 op/s</sub> | -23.2% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.656<br><sub>context: p90 0.761 · p95 0.790 · p99 0.870 · 1508 op/s</sub> | 0.527<br><sub>context: p90 0.633 · p95 0.659 · p99 0.686 · 1866 op/s</sub> | -19.7% (-0.129) | 150% AND 2 ms | 🟢 |
| 8 | 0.677<br><sub>context: p90 0.823 · p95 0.869 · p99 0.980 · 11445 op/s</sub> | 0.460<br><sub>context: p90 0.577 · p95 0.613 · p99 0.683 · 16686 op/s</sub> | -32.1% (-0.217) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.159<br><sub>context: p90 1.667 · p95 1.776 · p99 1.990 · 825 op/s</sub> | 1.485<br><sub>context: p90 2.146 · p95 2.327 · p99 2.642 · 652 op/s</sub> | +28.2% (+0.327) | 150% AND 2 ms | 🟢 |
| 8 | 1.367<br><sub>context: p90 2.075 · p95 2.357 · p99 2.801 · 5502 op/s</sub> | 2.152<br><sub>context: p90 3.334 · p95 3.746 · p99 4.329 · 3505 op/s</sub> | +57.4% (+0.785) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.571<br><sub>context: p90 5.534 · p95 6.268 · p99 6.761 · 268 op/s</sub> | 4.699<br><sub>context: p90 6.974 · p95 7.693 · p99 8.243 · 205 op/s</sub> | +31.6% (+1.128) | 150% AND 2 ms | 🟢 |
| 8 | 4.518<br><sub>context: p90 7.979 · p95 9.054 · p99 11.163 · 1594 op/s</sub> | 8.201<br><sub>context: p90 12.370 · p95 13.654 · p99 15.242 · 946 op/s</sub> | +81.5% (+3.683) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.398<br><sub>context: p90 0.505 · p95 0.543 · p99 0.583 · 2464 op/s</sub> | 0.269<br><sub>context: p90 0.343 · p95 0.373 · p99 0.413 · 3585 op/s</sub> | -32.6% (-0.130) | 150% AND 2 ms | 🟢 |
| 8 | 0.368<br><sub>context: p90 0.451 · p95 0.481 · p99 0.553 · 20796 op/s</sub> | 0.311<br><sub>context: p90 0.424 · p95 0.458 · p99 0.552 · 23985 op/s</sub> | -15.6% (-0.058) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.444<br><sub>context: p90 0.537 · p95 0.569 · p99 0.617 · 2225 op/s</sub> | 0.238<br><sub>context: p90 0.318 · p95 0.345 · p99 0.390 · 4019 op/s</sub> | -46.5% (-0.206) | 150% AND 2 ms | 🟢 |
| 8 | 0.369<br><sub>context: p90 0.458 · p95 0.493 · p99 0.547 · 20450 op/s</sub> | 0.305<br><sub>context: p90 0.406 · p95 0.450 · p99 0.523 · 25030 op/s</sub> | -17.4% (-0.064) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.519<br><sub>context: p90 0.618 · p95 0.656 · p99 0.672 · 1894 op/s</sub> | 0.409<br><sub>context: p90 0.525 · p95 0.550 · p99 0.592 · 2410 op/s</sub> | -21.2% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.494<br><sub>context: p90 0.613 · p95 0.652 · p99 0.718 · 15478 op/s</sub> | 0.386<br><sub>context: p90 0.502 · p95 0.539 · p99 0.648 · 19592 op/s</sub> | -21.9% (-0.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.311<br><sub>context: p90 0.394 · p95 0.449 · p99 0.522 · 3134 op/s</sub> | 0.174<br><sub>context: p90 0.240 · p95 0.246 · p99 0.275 · 5438 op/s</sub> | -44.1% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.368 · p95 0.391 · p99 0.442 · 25947 op/s</sub> | 0.230<br><sub>context: p90 0.306 · p95 0.334 · p99 0.369 · 33090 op/s</sub> | -21.4% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.509<br><sub>context: p90 0.609 · p95 0.636 · p99 0.681 · 1920 op/s</sub> | 0.501<br><sub>context: p90 0.601 · p95 0.638 · p99 0.675 · 1968 op/s</sub> | -1.6% (-0.008) | 150% AND 2 ms | 🟢 |
| 8 | 0.469<br><sub>context: p90 0.570 · p95 0.598 · p99 0.654 · 16536 op/s</sub> | 0.420<br><sub>context: p90 0.531 · p95 0.570 · p99 0.651 · 17993 op/s</sub> | -10.4% (-0.049) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.440<br><sub>context: p90 0.530 · p95 0.571 · p99 0.598 · 2244 op/s</sub> | 0.395<br><sub>context: p90 0.507 · p95 0.523 · p99 0.615 · 2437 op/s</sub> | -10.2% (-0.045) | 150% AND 2 ms | 🟢 |
| 8 | 0.388<br><sub>context: p90 0.471 · p95 0.504 · p99 0.556 · 19573 op/s</sub> | 0.348<br><sub>context: p90 0.457 · p95 0.488 · p99 0.561 · 21884 op/s</sub> | -10.1% (-0.039) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.579<br><sub>context: p90 0.686 · p95 0.728 · p99 0.784 · 1698 op/s</sub> | 0.514<br><sub>context: p90 0.608 · p95 0.659 · p99 0.718 · 1904 op/s</sub> | -11.3% (-0.065) | 150% AND 2 ms | 🟢 |
| 8 | 0.530<br><sub>context: p90 0.650 · p95 0.685 · p99 0.772 · 14553 op/s</sub> | 0.450<br><sub>context: p90 0.561 · p95 0.604 · p99 0.675 · 16917 op/s</sub> | -15.1% (-0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.670<br><sub>context: p90 0.794 · p95 0.825 · p99 0.921 · 1467 op/s</sub> | 0.640<br><sub>context: p90 0.779 · p95 0.802 · p99 0.912 · 1541 op/s</sub> | -4.4% (-0.030) | 150% AND 2 ms | 🟢 |
| 8 | 0.692<br><sub>context: p90 0.877 · p95 0.937 · p99 1.034 · 11103 op/s</sub> | 0.647<br><sub>context: p90 0.842 · p95 0.905 · p99 1.021 · 11806 op/s</sub> | -6.5% (-0.045) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.688<br><sub>context: p90 0.801 · p95 0.865 · p99 0.946 · 1421 op/s</sub> | 0.646<br><sub>context: p90 0.770 · p95 0.815 · p99 0.918 · 1529 op/s</sub> | -6.1% (-0.042) | 150% AND 2 ms | 🟢 |
| 8 | 0.697<br><sub>context: p90 0.882 · p95 0.934 · p99 1.058 · 11043 op/s</sub> | 0.656<br><sub>context: p90 0.823 · p95 0.886 · p99 0.977 · 11707 op/s</sub> | -5.9% (-0.041) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.581<br><sub>context: p90 0.668 · p95 0.699 · p99 0.729 · 1707 op/s</sub> | 0.515<br><sub>context: p90 0.613 · p95 0.648 · p99 0.702 · 1904 op/s</sub> | -11.3% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.560<br><sub>context: p90 0.672 · p95 0.706 · p99 0.788 · 13958 op/s</sub> | 0.483<br><sub>context: p90 0.611 · p95 0.657 · p99 0.782 · 15935 op/s</sub> | -13.8% (-0.077) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.569<br><sub>context: p90 0.671 · p95 0.686 · p99 0.740 · 1727 op/s</sub> | 0.525<br><sub>context: p90 0.633 · p95 0.688 · p99 0.722 · 1890 op/s</sub> | -7.7% (-0.044) | 150% AND 2 ms | 🟢 |
| 8 | 0.558<br><sub>context: p90 0.672 · p95 0.711 · p99 0.796 · 13839 op/s</sub> | 0.425<br><sub>context: p90 0.557 · p95 0.608 · p99 0.691 · 17938 op/s</sub> | -23.8% (-0.133) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.917<br><sub>context: p90 19.149 · p95 19.276 · p99 19.472 · 53 op/s</sub> | 15.418<br><sub>context: p90 15.580 · p95 15.633 · p99 15.735 · 65 op/s</sub> | -18.5% (-3.498) | 150% AND 2 ms | 🟢 |
| 8 | 24.359<br><sub>context: p90 33.759 · p95 37.422 · p99 42.112 · 298 op/s</sub> | 18.779<br><sub>context: p90 25.033 · p95 28.322 · p99 31.965 · 380 op/s</sub> | -22.9% (-5.580) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.685<br><sub>context: p90 0.768 · p95 0.789 · p99 0.876 · 1456 op/s</sub> | 0.557<br><sub>context: p90 0.661 · p95 0.680 · p99 0.702 · 1761 op/s</sub> | -18.7% (-0.128) | 150% AND 2 ms | 🟢 |
| 8 | 0.669<br><sub>context: p90 0.810 · p95 0.861 · p99 0.952 · 11176 op/s</sub> | 0.468<br><sub>context: p90 0.585 · p95 0.620 · p99 0.687 · 16351 op/s</sub> | -30.0% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.635<br><sub>context: p90 2.184 · p95 2.337 · p99 2.540 · 606 op/s</sub> | 1.568<br><sub>context: p90 2.098 · p95 2.206 · p99 2.423 · 630 op/s</sub> | -4.1% (-0.067) | 150% AND 2 ms | 🟢 |
| 8 | 1.897<br><sub>context: p90 2.671 · p95 2.898 · p99 3.335 · 4065 op/s</sub> | 1.683<br><sub>context: p90 2.345 · p95 2.560 · p99 2.984 · 4586 op/s</sub> | -11.2% (-0.213) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.623<br><sub>context: p90 0.713 · p95 0.740 · p99 0.796 · 1581 op/s</sub> | 0.590<br><sub>context: p90 0.702 · p95 0.737 · p99 0.793 · 1662 op/s</sub> | -5.2% (-0.033) | 150% AND 2 ms | 🟢 |
| 8 | 0.586<br><sub>context: p90 0.709 · p95 0.755 · p99 0.847 · 13212 op/s</sub> | 0.541<br><sub>context: p90 0.712 · p95 0.775 · p99 0.887 · 13923 op/s</sub> | -7.7% (-0.045) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.387<br><sub>context: p90 2.981 · p95 3.058 · p99 3.351 · 426 op/s</sub> | 0.545<br><sub>context: p90 0.676 · p95 0.743 · p99 0.820 · 1788 op/s</sub> | -77.2% (-1.842) | 150% AND 2 ms | 🟢 |
| 8 | 2.582<br><sub>context: p90 3.350 · p95 3.554 · p99 3.824 · 3042 op/s</sub> | 0.461<br><sub>context: p90 0.596 · p95 0.642 · p99 0.769 · 16549 op/s</sub> | -82.2% (-2.122) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.509<br><sub>context: p90 3.083 · p95 3.188 · p99 3.293 · 414 op/s</sub> | 0.565<br><sub>context: p90 0.681 · p95 0.712 · p99 0.790 · 1750 op/s</sub> | -77.5% (-1.945) | 150% AND 2 ms | 🟢 |
| 8 | 2.656<br><sub>context: p90 3.448 · p95 3.620 · p99 3.893 · 3059 op/s</sub> | 0.464<br><sub>context: p90 0.594 · p95 0.646 · p99 0.757 · 16415 op/s</sub> | -82.5% (-2.192) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.447 · p95 0.484 · p99 0.504 · 2638 op/s</sub> | 0.336<br><sub>context: p90 0.421 · p95 0.446 · p99 0.494 · 2964 op/s</sub> | -9.2% (-0.034) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.385 · p95 0.413 · p99 0.470 · 25107 op/s</sub> | 0.325<br><sub>context: p90 0.443 · p95 0.494 · p99 0.569 · 22774 op/s</sub> | +6.5% (+0.020) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.452<br><sub>context: p90 0.533 · p95 0.552 · p99 0.595 · 2185 op/s</sub> | 0.233<br><sub>context: p90 0.296 · p95 0.316 · p99 0.385 · 4208 op/s</sub> | -48.5% (-0.219) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.423<br><sub>context: p90 0.504 · p95 0.536 · p99 0.600 · 18403 op/s</sub> | 0.249<br><sub>context: p90 0.318 · p95 0.342 · p99 0.377 · 30729 op/s</sub> | -41.0% (-0.173) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.450<br><sub>context: p90 0.564 · p95 0.590 · p99 0.637 · 2151 op/s</sub> | 0.419<br><sub>context: p90 0.539 · p95 0.578 · p99 0.623 · 2247 op/s</sub> | -6.9% (-0.031) | 150% AND 2 ms | 🟢 |
| 8 | 0.454<br><sub>context: p90 0.556 · p95 0.593 · p99 0.668 · 16975 op/s</sub> | 0.392<br><sub>context: p90 0.518 · p95 0.558 · p99 0.631 · 19348 op/s</sub> | -13.5% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.460<br><sub>context: p90 0.553 · p95 0.591 · p99 0.655 · 2147 op/s</sub> | 0.406<br><sub>context: p90 0.507 · p95 0.541 · p99 0.608 · 2402 op/s</sub> | -11.6% (-0.053) | 150% AND 2 ms | 🟢 |
| 8 | 0.456<br><sub>context: p90 0.564 · p95 0.609 · p99 0.711 · 16965 op/s</sub> | 0.414<br><sub>context: p90 0.562 · p95 0.619 · p99 0.724 · 17976 op/s</sub> | -9.1% (-0.041) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.467<br><sub>context: p90 0.551 · p95 0.574 · p99 0.643 · 2122 op/s</sub> | 0.379<br><sub>context: p90 0.495 · p95 0.529 · p99 0.570 · 2595 op/s</sub> | -18.9% (-0.088) | 150% AND 2 ms | 🟢 |
| 8 | 0.403<br><sub>context: p90 0.510 · p95 0.552 · p99 0.634 · 18403 op/s</sub> | 0.327<br><sub>context: p90 0.429 · p95 0.474 · p99 0.543 · 23485 op/s</sub> | -18.9% (-0.076) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.602<br><sub>context: p90 1.771 · p95 1.813 · p99 1.926 · 618 op/s</sub> | 1.249<br><sub>context: p90 1.380 · p95 1.416 · p99 1.487 · 792 op/s</sub> | -22.0% (-0.353) | 150% AND 2 ms | 🟢 |
| 8 | 1.835<br><sub>context: p90 2.230 · p95 2.363 · p99 2.772 · 4170 op/s</sub> | 1.251<br><sub>context: p90 1.446 · p95 1.520 · p99 1.635 · 6263 op/s</sub> | -31.8% (-0.584) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.443<br><sub>context: p90 1.553 · p95 1.604 · p99 1.662 · 689 op/s</sub> | 1.145<br><sub>context: p90 1.242 · p95 1.266 · p99 1.337 · 865 op/s</sub> | -20.7% (-0.299) | 150% AND 2 ms | 🟢 |
| 8 | 1.665<br><sub>context: p90 2.061 · p95 2.172 · p99 2.497 · 4625 op/s</sub> | 1.146<br><sub>context: p90 1.321 · p95 1.373 · p99 1.488 · 6851 op/s</sub> | -31.2% (-0.519) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.624<br><sub>context: p90 0.780 · p95 0.819 · p99 0.888 · 1546 op/s</sub> | 0.474<br><sub>context: p90 0.595 · p95 0.627 · p99 0.679 · 2062 op/s</sub> | -24.0% (-0.150) | 150% AND 2 ms | 🟢 |
| 8 | 0.486<br><sub>context: p90 0.598 · p95 0.642 · p99 0.740 · 15702 op/s</sub> | 0.399<br><sub>context: p90 0.511 · p95 0.551 · p99 0.636 · 18804 op/s</sub> | -18.0% (-0.088) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.652<br><sub>context: p90 0.760 · p95 0.778 · p99 0.839 · 1493 op/s</sub> | 0.637<br><sub>context: p90 0.752 · p95 0.784 · p99 0.862 · 1559 op/s</sub> | -2.2% (-0.015) | 150% AND 2 ms | 🟢 |
| 8 | 0.595<br><sub>context: p90 0.718 · p95 0.758 · p99 0.860 · 13048 op/s</sub> | 0.493<br><sub>context: p90 0.623 · p95 0.676 · p99 0.757 · 15345 op/s</sub> | -17.1% (-0.102) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.409<br><sub>context: p90 0.495 · p95 0.540 · p99 0.624 · 2399 op/s</sub> | 0.361<br><sub>context: p90 0.435 · p95 0.471 · p99 0.518 · 2757 op/s</sub> | -11.7% (-0.048) | 150% AND 2 ms | 🟢 |
| 8 | 0.414<br><sub>context: p90 0.506 · p95 0.536 · p99 0.607 · 18643 op/s</sub> | 0.330<br><sub>context: p90 0.435 · p95 0.470 · p99 0.544 · 22690 op/s</sub> | -20.2% (-0.084) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.499<br><sub>context: p90 0.655 · p95 0.686 · p99 0.767 · 1966 op/s</sub> | 0.342<br><sub>context: p90 0.438 · p95 0.474 · p99 0.610 · 2855 op/s</sub> | -31.5% (-0.157) | 150% AND 2 ms | 🟢 |
| 8 | 0.316<br><sub>context: p90 0.400 · p95 0.430 · p99 0.485 · 24136 op/s</sub> | 0.318<br><sub>context: p90 0.437 · p95 0.478 · p99 0.557 · 23363 op/s</sub> | +0.9% (+0.003) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.401<br><sub>context: p90 0.492 · p95 0.526 · p99 0.569 · 2462 op/s</sub> | 0.327<br><sub>context: p90 0.440 · p95 0.480 · p99 0.520 · 2984 op/s</sub> | -18.6% (-0.074) | 150% AND 2 ms | 🟢 |
| 8 | 0.327<br><sub>context: p90 0.450 · p95 0.496 · p99 0.576 · 22873 op/s</sub> | 0.328<br><sub>context: p90 0.453 · p95 0.496 · p99 0.604 · 22813 op/s</sub> | +0.4% (+0.001) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.174<br><sub>context: p90 1.268 · p95 1.297 · p99 1.334 · 842 op/s</sub> | 0.534<br><sub>context: p90 0.625 · p95 0.649 · p99 0.723 · 1848 op/s</sub> | -54.5% (-0.640) | 150% AND 2 ms | 🟢 |
| 8 | 1.345<br><sub>context: p90 1.847 · p95 2.034 · p99 2.301 · 5580 op/s</sub> | 0.476<br><sub>context: p90 0.573 · p95 0.603 · p99 0.666 · 16228 op/s</sub> | -64.7% (-0.870) | 150% AND 2 ms | 🟢 |

</details>
