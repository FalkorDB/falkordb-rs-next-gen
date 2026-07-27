### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 |
| workload_hash | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**main vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.581<br><sub>context: p90 1.662 · p95 1.683 · p99 1.713 · 629 op/s</sub> | 0.883<br><sub>context: p90 0.973 · p95 1.016 · p99 1.048 · 1119 op/s</sub> | -44.1% (-0.698) | 150% AND 2 ms | 🟢 |
| 8 | 1.863<br><sub>context: p90 2.428 · p95 2.608 · p99 3.023 · 4084 op/s</sub> | 0.921<br><sub>context: p90 1.069 · p95 1.108 · p99 1.205 · 8537 op/s</sub> | -50.5% (-0.942) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.280<br><sub>context: p90 2.359 · p95 2.384 · p99 2.433 · 438 op/s</sub> | 0.955<br><sub>context: p90 1.031 · p95 1.059 · p99 1.105 · 1040 op/s</sub> | -58.1% (-1.325) | 150% AND 2 ms | 🟢 |
| 8 | 2.674<br><sub>context: p90 3.513 · p95 3.635 · p99 3.965 · 2771 op/s</sub> | 1.029<br><sub>context: p90 1.230 · p95 1.266 · p99 1.355 · 7646 op/s</sub> | -61.5% (-1.645) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.204<br><sub>context: p90 2.298 · p95 2.354 · p99 2.400 · 451 op/s</sub> | 1.511<br><sub>context: p90 1.621 · p95 1.641 · p99 1.670 · 657 op/s</sub> | -31.4% (-0.693) | 150% AND 2 ms | 🟢 |
| 8 | 2.700<br><sub>context: p90 3.509 · p95 3.698 · p99 4.134 · 2777 op/s</sub> | 1.799<br><sub>context: p90 2.312 · p95 2.471 · p99 2.774 · 4283 op/s</sub> | -33.4% (-0.902) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.843<br><sub>context: p90 2.940 · p95 2.969 · p99 3.044 · 351 op/s</sub> | 1.585<br><sub>context: p90 1.662 · p95 1.688 · p99 1.727 · 629 op/s</sub> | -44.3% (-1.259) | 150% AND 2 ms | 🟢 |
| 8 | 3.789<br><sub>context: p90 4.818 · p95 5.097 · p99 5.623 · 2061 op/s</sub> | 1.684<br><sub>context: p90 2.035 · p95 2.106 · p99 2.238 · 4595 op/s</sub> | -55.6% (-2.106) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.200<br><sub>context: p90 0.272 · p95 0.298 · p99 0.342 · 4788 op/s</sub> | 0.167<br><sub>context: p90 0.221 · p95 0.231 · p99 0.299 · 5725 op/s</sub> | -16.4% (-0.033) | 150% AND 2 ms | 🟢 |
| 8 | 0.242<br><sub>context: p90 0.314 · p95 0.342 · p99 0.384 · 30742 op/s</sub> | 0.217<br><sub>context: p90 0.293 · p95 0.314 · p99 0.364 · 34726 op/s</sub> | -10.2% (-0.025) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.521<br><sub>context: p90 0.621 · p95 0.640 · p99 0.694 · 1896 op/s</sub> | 0.376<br><sub>context: p90 0.474 · p95 0.504 · p99 0.535 · 2618 op/s</sub> | -27.7% (-0.145) | 150% AND 2 ms | 🟢 |
| 8 | 0.456<br><sub>context: p90 0.553 · p95 0.583 · p99 0.654 · 16916 op/s</sub> | 0.336<br><sub>context: p90 0.436 · p95 0.468 · p99 0.549 · 22405 op/s</sub> | -26.2% (-0.119) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.546<br><sub>context: p90 0.642 · p95 0.665 · p99 0.723 · 1816 op/s</sub> | 0.428<br><sub>context: p90 0.519 · p95 0.553 · p99 0.577 · 2295 op/s</sub> | -21.7% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.495<br><sub>context: p90 0.611 · p95 0.656 · p99 0.736 · 15531 op/s</sub> | 0.371<br><sub>context: p90 0.484 · p95 0.527 · p99 0.643 · 19962 op/s</sub> | -25.0% (-0.124) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.576<br><sub>context: p90 0.677 · p95 0.705 · p99 0.790 · 1713 op/s</sub> | 0.523<br><sub>context: p90 0.613 · p95 0.643 · p99 0.702 · 1892 op/s</sub> | -9.3% (-0.054) | 150% AND 2 ms | 🟢 |
| 8 | 0.542<br><sub>context: p90 0.664 · p95 0.706 · p99 0.796 · 14240 op/s</sub> | 0.459<br><sub>context: p90 0.574 · p95 0.608 · p99 0.688 · 16687 op/s</sub> | -15.3% (-0.083) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.608<br><sub>context: p90 0.692 · p95 0.717 · p99 0.775 · 1625 op/s</sub> | 0.554<br><sub>context: p90 0.664 · p95 0.688 · p99 0.736 · 1779 op/s</sub> | -8.8% (-0.053) | 150% AND 2 ms | 🟢 |
| 8 | 0.596<br><sub>context: p90 0.728 · p95 0.781 · p99 0.925 · 12849 op/s</sub> | 0.535<br><sub>context: p90 0.662 · p95 0.710 · p99 0.808 · 14381 op/s</sub> | -10.2% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.742<br><sub>context: p90 0.890 · p95 0.943 · p99 1.073 · 1322 op/s</sub> | 0.681<br><sub>context: p90 0.825 · p95 0.848 · p99 0.950 · 1445 op/s</sub> | -8.2% (-0.061) | 150% AND 2 ms | 🟢 |
| 8 | 0.779<br><sub>context: p90 0.996 · p95 1.060 · p99 1.212 · 9911 op/s</sub> | 0.672<br><sub>context: p90 0.849 · p95 0.915 · p99 1.050 · 11426 op/s</sub> | -13.7% (-0.107) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.781<br><sub>context: p90 0.975 · p95 1.021 · p99 1.098 · 1251 op/s</sub> | 0.720<br><sub>context: p90 0.873 · p95 0.914 · p99 0.975 · 1362 op/s</sub> | -7.8% (-0.061) | 150% AND 2 ms | 🟢 |
| 8 | 0.824<br><sub>context: p90 1.055 · p95 1.135 · p99 1.320 · 9132 op/s</sub> | 0.718<br><sub>context: p90 0.921 · p95 0.995 · p99 1.130 · 10797 op/s</sub> | -12.9% (-0.106) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.385<br><sub>context: p90 1.885 · p95 2.025 · p99 2.291 · 704 op/s</sub> | 1.258<br><sub>context: p90 1.652 · p95 1.741 · p99 1.904 · 782 op/s</sub> | -9.1% (-0.127) | 150% AND 2 ms | 🟢 |
| 8 | 1.618<br><sub>context: p90 2.275 · p95 2.500 · p99 2.880 · 4722 op/s</sub> | 1.339<br><sub>context: p90 1.845 · p95 2.017 · p99 2.325 · 5726 op/s</sub> | -17.2% (-0.278) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.463<br><sub>context: p90 1.926 · p95 2.152 · p99 2.346 · 663 op/s</sub> | 1.242<br><sub>context: p90 1.636 · p95 1.787 · p99 2.025 · 775 op/s</sub> | -15.1% (-0.221) | 150% AND 2 ms | 🟢 |
| 8 | 1.792<br><sub>context: p90 2.529 · p95 2.744 · p99 3.212 · 4255 op/s</sub> | 1.379<br><sub>context: p90 1.896 · p95 2.076 · p99 2.336 · 5572 op/s</sub> | -23.0% (-0.413) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.757<br><sub>context: p90 0.847 · p95 0.887 · p99 0.939 · 1335 op/s</sub> | 0.844<br><sub>context: p90 1.052 · p95 1.107 · p99 1.199 · 1179 op/s</sub> | +11.5% (+0.087) | 150% AND 2 ms | 🟢 |
| 8 | 0.785<br><sub>context: p90 0.927 · p95 0.972 · p99 1.059 · 9944 op/s</sub> | 0.818<br><sub>context: p90 1.073 · p95 1.159 · p99 1.325 · 9560 op/s</sub> | +4.2% (+0.033) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.531<br><sub>context: p90 0.622 · p95 0.642 · p99 0.673 · 1858 op/s</sub> | 0.512<br><sub>context: p90 0.620 · p95 0.654 · p99 0.705 · 1925 op/s</sub> | -3.6% (-0.019) | 150% AND 2 ms | 🟢 |
| 8 | 0.566<br><sub>context: p90 0.670 · p95 0.704 · p99 0.802 · 13584 op/s</sub> | 0.441<br><sub>context: p90 0.571 · p95 0.623 · p99 0.714 · 17217 op/s</sub> | -22.1% (-0.125) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.364<br><sub>context: p90 0.451 · p95 0.471 · p99 0.514 · 2653 op/s</sub> | 0.184<br><sub>context: p90 0.246 · p95 0.274 · p99 0.306 · 5073 op/s</sub> | -49.6% (-0.181) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.417 · p95 0.446 · p99 0.507 · 22723 op/s</sub> | 0.225<br><sub>context: p90 0.290 · p95 0.314 · p99 0.353 · 34360 op/s</sub> | -32.6% (-0.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.241<br><sub>context: p90 0.302 · p95 0.336 · p99 0.381 · 3987 op/s</sub> | 0.140<br><sub>context: p90 0.198 · p95 0.208 · p99 0.240 · 6628 op/s</sub> | -41.8% (-0.100) | 150% AND 2 ms | 🟢 |
| 8 | 0.280<br><sub>context: p90 0.366 · p95 0.393 · p99 0.450 · 27039 op/s</sub> | 0.206<br><sub>context: p90 0.277 · p95 0.299 · p99 0.347 · 37539 op/s</sub> | -26.5% (-0.074) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.640<br><sub>context: p90 0.746 · p95 0.766 · p99 0.860 · 1538 op/s</sub> | 0.477<br><sub>context: p90 0.570 · p95 0.596 · p99 0.628 · 2080 op/s</sub> | -25.5% (-0.163) | 150% AND 2 ms | 🟢 |
| 8 | 0.670<br><sub>context: p90 0.790 · p95 0.831 · p99 0.930 · 11715 op/s</sub> | 0.458<br><sub>context: p90 0.568 · p95 0.601 · p99 0.682 · 16837 op/s</sub> | -31.6% (-0.212) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.954<br><sub>context: p90 1.270 · p95 1.357 · p99 1.530 · 1021 op/s</sub> | 1.664<br><sub>context: p90 2.341 · p95 2.558 · p99 2.903 · 588 op/s</sub> | +74.4% (+0.710) | 150% AND 2 ms | 🟢 |
| 8 | 1.086<br><sub>context: p90 1.585 · p95 1.749 · p99 2.036 · 7013 op/s</sub> | 2.325<br><sub>context: p90 3.603 · p95 3.989 · p99 4.565 · 3307 op/s</sub> | +114.2% (+1.239) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.608<br><sub>context: p90 4.005 · p95 4.467 · p99 4.847 · 366 op/s</sub> | 5.103<br><sub>context: p90 7.613 · p95 8.572 · p99 8.994 · 189 op/s</sub> | +95.7% (+2.495) | 150% AND 2 ms | 🟢 |
| 8 | 3.257<br><sub>context: p90 5.494 · p95 6.269 · p99 7.562 · 2248 op/s</sub> | 8.603<br><sub>context: p90 13.211 · p95 14.467 · p99 16.443 · 895 op/s</sub> | +164.1% (+5.346) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.364<br><sub>context: p90 0.463 · p95 0.499 · p99 0.549 · 2655 op/s</sub> | 0.246<br><sub>context: p90 0.325 · p95 0.352 · p99 0.390 · 3998 op/s</sub> | -32.4% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.361<br><sub>context: p90 0.446 · p95 0.473 · p99 0.535 · 21088 op/s</sub> | 0.306<br><sub>context: p90 0.414 · p95 0.448 · p99 0.533 · 24623 op/s</sub> | -15.2% (-0.055) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.388<br><sub>context: p90 0.462 · p95 0.491 · p99 0.536 · 2562 op/s</sub> | 0.253<br><sub>context: p90 0.320 · p95 0.337 · p99 0.381 · 4065 op/s</sub> | -34.9% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.368<br><sub>context: p90 0.455 · p95 0.482 · p99 0.556 · 21043 op/s</sub> | 0.308<br><sub>context: p90 0.415 · p95 0.465 · p99 0.536 · 24539 op/s</sub> | -16.4% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.444<br><sub>context: p90 0.528 · p95 0.550 · p99 0.601 · 2221 op/s</sub> | 0.394<br><sub>context: p90 0.539 · p95 0.586 · p99 0.636 · 2427 op/s</sub> | -11.2% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.498<br><sub>context: p90 0.613 · p95 0.651 · p99 0.727 · 15407 op/s</sub> | 0.374<br><sub>context: p90 0.495 · p95 0.542 · p99 0.653 · 19771 op/s</sub> | -24.9% (-0.124) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.406 · p95 0.448 · p99 0.477 · 3102 op/s</sub> | 0.218<br><sub>context: p90 0.267 · p95 0.301 · p99 0.385 · 4503 op/s</sub> | -30.9% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.373 · p95 0.399 · p99 0.461 · 25705 op/s</sub> | 0.232<br><sub>context: p90 0.317 · p95 0.347 · p99 0.441 · 32233 op/s</sub> | -22.8% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.498<br><sub>context: p90 0.586 · p95 0.613 · p99 0.657 · 1981 op/s</sub> | 0.428<br><sub>context: p90 0.541 · p95 0.572 · p99 0.625 · 2278 op/s</sub> | -14.0% (-0.069) | 150% AND 2 ms | 🟢 |
| 8 | 0.458<br><sub>context: p90 0.541 · p95 0.567 · p99 0.619 · 16834 op/s</sub> | 0.406<br><sub>context: p90 0.504 · p95 0.533 · p99 0.595 · 18883 op/s</sub> | -11.3% (-0.052) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.420<br><sub>context: p90 0.505 · p95 0.524 · p99 0.568 · 2358 op/s</sub> | 0.357<br><sub>context: p90 0.452 · p95 0.485 · p99 0.533 · 2732 op/s</sub> | -15.1% (-0.063) | 150% AND 2 ms | 🟢 |
| 8 | 0.386<br><sub>context: p90 0.468 · p95 0.503 · p99 0.556 · 19943 op/s</sub> | 0.335<br><sub>context: p90 0.421 · p95 0.458 · p99 0.544 · 22788 op/s</sub> | -13.3% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.572<br><sub>context: p90 0.678 · p95 0.701 · p99 0.764 · 1723 op/s</sub> | 0.523<br><sub>context: p90 0.639 · p95 0.676 · p99 0.719 · 1875 op/s</sub> | -8.7% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.527<br><sub>context: p90 0.637 · p95 0.675 · p99 0.745 · 14765 op/s</sub> | 0.474<br><sub>context: p90 0.598 · p95 0.654 · p99 0.763 · 16135 op/s</sub> | -10.2% (-0.054) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.671<br><sub>context: p90 0.790 · p95 0.858 · p99 0.911 · 1472 op/s</sub> | 0.649<br><sub>context: p90 0.791 · p95 0.849 · p99 0.891 · 1516 op/s</sub> | -3.2% (-0.022) | 150% AND 2 ms | 🟢 |
| 8 | 0.692<br><sub>context: p90 0.905 · p95 0.968 · p99 1.090 · 11042 op/s</sub> | 0.646<br><sub>context: p90 0.839 · p95 0.902 · p99 1.024 · 11880 op/s</sub> | -6.7% (-0.046) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.700<br><sub>context: p90 0.842 · p95 0.859 · p99 0.969 · 1404 op/s</sub> | 0.668<br><sub>context: p90 0.795 · p95 0.834 · p99 0.878 · 1477 op/s</sub> | -4.6% (-0.032) | 150% AND 2 ms | 🟢 |
| 8 | 0.691<br><sub>context: p90 0.864 · p95 0.922 · p99 1.058 · 11119 op/s</sub> | 0.670<br><sub>context: p90 0.835 · p95 0.897 · p99 0.997 · 11400 op/s</sub> | -3.0% (-0.021) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.602<br><sub>context: p90 0.704 · p95 0.732 · p99 0.840 · 1632 op/s</sub> | 0.580<br><sub>context: p90 0.671 · p95 0.690 · p99 0.771 · 1693 op/s</sub> | -3.5% (-0.021) | 150% AND 2 ms | 🟢 |
| 8 | 0.576<br><sub>context: p90 0.698 · p95 0.746 · p99 0.832 · 13474 op/s</sub> | 0.491<br><sub>context: p90 0.612 · p95 0.651 · p99 0.733 · 15664 op/s</sub> | -14.7% (-0.085) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.551<br><sub>context: p90 0.631 · p95 0.661 · p99 0.699 · 1793 op/s</sub> | 0.465<br><sub>context: p90 0.558 · p95 0.578 · p99 0.654 · 2108 op/s</sub> | -15.6% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.552<br><sub>context: p90 0.655 · p95 0.692 · p99 0.770 · 14121 op/s</sub> | 0.404<br><sub>context: p90 0.510 · p95 0.554 · p99 0.631 · 18975 op/s</sub> | -26.8% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.660<br><sub>context: p90 18.866 · p95 18.929 · p99 19.024 · 53 op/s</sub> | 15.351<br><sub>context: p90 15.511 · p95 15.546 · p99 15.661 · 65 op/s</sub> | -17.7% (-3.309) | 150% AND 2 ms | 🟢 |
| 8 | 23.812<br><sub>context: p90 31.315 · p95 34.362 · p99 38.187 · 310 op/s</sub> | 19.258<br><sub>context: p90 26.418 · p95 29.582 · p99 32.819 · 373 op/s</sub> | -19.1% (-4.554) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.645<br><sub>context: p90 0.745 · p95 0.780 · p99 0.813 · 1537 op/s</sub> | 0.523<br><sub>context: p90 0.634 · p95 0.659 · p99 0.685 · 1872 op/s</sub> | -18.9% (-0.122) | 150% AND 2 ms | 🟢 |
| 8 | 0.665<br><sub>context: p90 0.787 · p95 0.827 · p99 0.939 · 11619 op/s</sub> | 0.471<br><sub>context: p90 0.576 · p95 0.611 · p99 0.684 · 16210 op/s</sub> | -29.2% (-0.194) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.664<br><sub>context: p90 2.180 · p95 2.394 · p99 2.730 · 593 op/s</sub> | 1.492<br><sub>context: p90 1.976 · p95 2.136 · p99 2.402 · 665 op/s</sub> | -10.3% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 1.879<br><sub>context: p90 2.616 · p95 2.845 · p99 3.430 · 4128 op/s</sub> | 1.643<br><sub>context: p90 2.322 · p95 2.532 · p99 3.026 · 4634 op/s</sub> | -12.5% (-0.236) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.593<br><sub>context: p90 0.694 · p95 0.729 · p99 0.803 · 1663 op/s</sub> | 0.538<br><sub>context: p90 0.635 · p95 0.648 · p99 0.710 · 1822 op/s</sub> | -9.3% (-0.055) | 150% AND 2 ms | 🟢 |
| 8 | 0.575<br><sub>context: p90 0.690 · p95 0.730 · p99 0.813 · 13537 op/s</sub> | 0.495<br><sub>context: p90 0.613 · p95 0.646 · p99 0.725 · 15738 op/s</sub> | -13.8% (-0.079) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.455<br><sub>context: p90 3.071 · p95 3.138 · p99 3.231 · 418 op/s</sub> | 0.480<br><sub>context: p90 0.584 · p95 0.608 · p99 0.669 · 2046 op/s</sub> | -80.5% (-1.976) | 150% AND 2 ms | 🟢 |
| 8 | 2.608<br><sub>context: p90 3.416 · p95 3.594 · p99 3.941 · 3038 op/s</sub> | 0.445<br><sub>context: p90 0.569 · p95 0.624 · p99 0.705 · 17208 op/s</sub> | -83.0% (-2.164) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.400<br><sub>context: p90 3.068 · p95 3.140 · p99 3.212 · 423 op/s</sub> | 0.489<br><sub>context: p90 0.604 · p95 0.644 · p99 0.720 · 2022 op/s</sub> | -79.6% (-1.911) | 150% AND 2 ms | 🟢 |
| 8 | 2.629<br><sub>context: p90 3.411 · p95 3.595 · p99 3.819 · 3098 op/s</sub> | 0.466<br><sub>context: p90 0.597 · p95 0.636 · p99 0.749 · 16318 op/s</sub> | -82.3% (-2.163) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.355<br><sub>context: p90 0.443 · p95 0.468 · p99 0.537 · 2801 op/s</sub> | 0.283<br><sub>context: p90 0.363 · p95 0.384 · p99 0.421 · 3468 op/s</sub> | -20.3% (-0.072) | 150% AND 2 ms | 🟢 |
| 8 | 0.306<br><sub>context: p90 0.383 · p95 0.414 · p99 0.484 · 24949 op/s</sub> | 0.310<br><sub>context: p90 0.424 · p95 0.464 · p99 0.541 · 24213 op/s</sub> | +1.4% (+0.004) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.400<br><sub>context: p90 0.467 · p95 0.494 · p99 0.537 · 2472 op/s</sub> | 0.211<br><sub>context: p90 0.280 · p95 0.302 · p99 0.360 · 4467 op/s</sub> | -47.4% (-0.190) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.420<br><sub>context: p90 0.509 · p95 0.538 · p99 0.615 · 18450 op/s</sub> | 0.244<br><sub>context: p90 0.315 · p95 0.338 · p99 0.392 · 31297 op/s</sub> | -41.9% (-0.176) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.434<br><sub>context: p90 0.524 · p95 0.558 · p99 0.617 · 2262 op/s</sub> | 0.425<br><sub>context: p90 0.539 · p95 0.589 · p99 0.685 · 2281 op/s</sub> | -2.1% (-0.009) | 150% AND 2 ms | 🟢 |
| 8 | 0.444<br><sub>context: p90 0.543 · p95 0.579 · p99 0.666 · 17421 op/s</sub> | 0.389<br><sub>context: p90 0.523 · p95 0.585 · p99 0.695 · 19135 op/s</sub> | -12.5% (-0.055) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.409<br><sub>context: p90 0.502 · p95 0.519 · p99 0.574 · 2376 op/s</sub> | 0.406<br><sub>context: p90 0.507 · p95 0.544 · p99 0.593 · 2418 op/s</sub> | -0.7% (-0.003) | 150% AND 2 ms | 🟢 |
| 8 | 0.453<br><sub>context: p90 0.547 · p95 0.585 · p99 0.648 · 16667 op/s</sub> | 0.390<br><sub>context: p90 0.515 · p95 0.562 · p99 0.666 · 19379 op/s</sub> | -13.8% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.437<br><sub>context: p90 0.531 · p95 0.549 · p99 0.579 · 2264 op/s</sub> | 0.341<br><sub>context: p90 0.431 · p95 0.474 · p99 0.516 · 2850 op/s</sub> | -22.0% (-0.096) | 150% AND 2 ms | 🟢 |
| 8 | 0.394<br><sub>context: p90 0.476 · p95 0.502 · p99 0.576 · 19149 op/s</sub> | 0.318<br><sub>context: p90 0.417 · p95 0.442 · p99 0.517 · 23612 op/s</sub> | -19.4% (-0.077) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.588<br><sub>context: p90 1.697 · p95 1.742 · p99 1.836 · 627 op/s</sub> | 1.176<br><sub>context: p90 1.269 · p95 1.319 · p99 1.366 · 845 op/s</sub> | -25.9% (-0.411) | 150% AND 2 ms | 🟢 |
| 8 | 1.856<br><sub>context: p90 2.206 · p95 2.324 · p99 2.695 · 4185 op/s</sub> | 1.203<br><sub>context: p90 1.389 · p95 1.445 · p99 1.548 · 6449 op/s</sub> | -35.2% (-0.653) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.430<br><sub>context: p90 1.561 · p95 1.592 · p99 1.691 · 698 op/s</sub> | 1.106<br><sub>context: p90 1.208 · p95 1.230 · p99 1.287 · 890 op/s</sub> | -22.7% (-0.324) | 150% AND 2 ms | 🟢 |
| 8 | 1.639<br><sub>context: p90 1.993 · p95 2.100 · p99 2.477 · 4760 op/s</sub> | 1.109<br><sub>context: p90 1.247 · p95 1.287 · p99 1.370 · 7135 op/s</sub> | -32.3% (-0.529) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.453<br><sub>context: p90 0.557 · p95 0.593 · p99 0.657 · 2149 op/s</sub> | 0.470<br><sub>context: p90 0.559 · p95 0.588 · p99 0.677 · 2129 op/s</sub> | +3.6% (+0.016) | 150% AND 2 ms | 🟢 |
| 8 | 0.477<br><sub>context: p90 0.578 · p95 0.618 · p99 0.693 · 16261 op/s</sub> | 0.399<br><sub>context: p90 0.512 · p95 0.544 · p99 0.633 · 19272 op/s</sub> | -16.4% (-0.078) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.592<br><sub>context: p90 0.713 · p95 0.748 · p99 0.779 · 1658 op/s</sub> | 0.611<br><sub>context: p90 0.725 · p95 0.758 · p99 0.835 · 1633 op/s</sub> | +3.2% (+0.019) | 150% AND 2 ms | 🟢 |
| 8 | 0.592<br><sub>context: p90 0.712 · p95 0.747 · p99 0.839 · 13238 op/s</sub> | 0.498<br><sub>context: p90 0.630 · p95 0.667 · p99 0.747 · 15501 op/s</sub> | -15.8% (-0.093) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.447<br><sub>context: p90 0.543 · p95 0.575 · p99 0.623 · 2122 op/s</sub> | 0.280<br><sub>context: p90 0.346 · p95 0.362 · p99 0.434 · 3525 op/s</sub> | -37.4% (-0.167) | 150% AND 2 ms | 🟢 |
| 8 | 0.416<br><sub>context: p90 0.513 · p95 0.549 · p99 0.635 · 18247 op/s</sub> | 0.325<br><sub>context: p90 0.424 · p95 0.455 · p99 0.513 · 23255 op/s</sub> | -21.7% (-0.090) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.298<br><sub>context: p90 0.359 · p95 0.372 · p99 0.412 · 3309 op/s</sub> | 0.261<br><sub>context: p90 0.353 · p95 0.376 · p99 0.407 · 3722 op/s</sub> | -12.4% (-0.037) | 150% AND 2 ms | 🟢 |
| 8 | 0.302<br><sub>context: p90 0.380 · p95 0.407 · p99 0.460 · 25273 op/s</sub> | 0.307<br><sub>context: p90 0.417 · p95 0.454 · p99 0.514 · 24489 op/s</sub> | +1.6% (+0.005) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.343<br><sub>context: p90 0.441 · p95 0.470 · p99 0.565 · 2806 op/s</sub> | 0.277<br><sub>context: p90 0.359 · p95 0.389 · p99 0.441 · 3574 op/s</sub> | -19.3% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.307<br><sub>context: p90 0.380 · p95 0.401 · p99 0.461 · 24666 op/s</sub> | 0.312<br><sub>context: p90 0.420 · p95 0.453 · p99 0.530 · 24289 op/s</sub> | +1.5% (+0.005) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.134<br><sub>context: p90 1.218 · p95 1.247 · p99 1.272 · 877 op/s</sub> | 0.528<br><sub>context: p90 0.625 · p95 0.645 · p99 0.685 · 1874 op/s</sub> | -53.5% (-0.607) | 150% AND 2 ms | 🟢 |
| 8 | 1.341<br><sub>context: p90 1.827 · p95 2.010 · p99 2.328 · 5663 op/s</sub> | 0.467<br><sub>context: p90 0.547 · p95 0.569 · p99 0.613 · 16833 op/s</sub> | -65.2% (-0.874) | 150% AND 2 ms | 🟢 |

</details>
