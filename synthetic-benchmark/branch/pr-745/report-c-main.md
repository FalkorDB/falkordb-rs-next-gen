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
| 1 | 1.374<br><sub>context: p90 1.451 · p95 1.479 · p99 1.568 · 720 op/s</sub> | 0.750<br><sub>context: p90 0.825 · p95 0.853 · p99 0.885 · 1309 op/s</sub> | -45.4% (-0.624) | 150% AND 2 ms | 🟢 |
| 8 | 1.883<br><sub>context: p90 2.440 · p95 2.681 · p99 3.073 · 4038 op/s</sub> | 0.894<br><sub>context: p90 1.040 · p95 1.074 · p99 1.144 · 8750 op/s</sub> | -52.5% (-0.988) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.078<br><sub>context: p90 2.182 · p95 2.223 · p99 2.322 · 474 op/s</sub> | 0.809<br><sub>context: p90 0.877 · p95 0.908 · p99 0.963 · 1213 op/s</sub> | -61.1% (-1.269) | 150% AND 2 ms | 🟢 |
| 8 | 2.659<br><sub>context: p90 3.454 · p95 3.568 · p99 4.099 · 2863 op/s</sub> | 0.986<br><sub>context: p90 1.159 · p95 1.190 · p99 1.258 · 7967 op/s</sub> | -62.9% (-1.673) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.033<br><sub>context: p90 2.133 · p95 2.165 · p99 2.256 · 488 op/s</sub> | 1.312<br><sub>context: p90 1.368 · p95 1.385 · p99 1.419 · 756 op/s</sub> | -35.4% (-0.720) | 150% AND 2 ms | 🟢 |
| 8 | 2.684<br><sub>context: p90 3.509 · p95 3.699 · p99 4.167 · 2824 op/s</sub> | 1.793<br><sub>context: p90 2.327 · p95 2.520 · p99 2.881 · 4277 op/s</sub> | -33.2% (-0.890) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.654<br><sub>context: p90 2.802 · p95 2.834 · p99 2.943 · 374 op/s</sub> | 1.415<br><sub>context: p90 1.503 · p95 1.520 · p99 1.572 · 704 op/s</sub> | -46.7% (-1.239) | 150% AND 2 ms | 🟢 |
| 8 | 3.658<br><sub>context: p90 4.699 · p95 4.970 · p99 5.498 · 2121 op/s</sub> | 1.635<br><sub>context: p90 1.953 · p95 2.007 · p99 2.101 · 4732 op/s</sub> | -55.3% (-2.023) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.183 · p95 0.193 · p99 0.211 · 6126 op/s</sub> | 0.148<br><sub>context: p90 0.175 · p95 0.182 · p99 0.190 · 6596 op/s</sub> | -5.3% (-0.008) | 150% AND 2 ms | 🟢 |
| 8 | 0.238<br><sub>context: p90 0.302 · p95 0.322 · p99 0.366 · 30895 op/s</sub> | 0.220<br><sub>context: p90 0.284 · p95 0.299 · p99 0.335 · 34688 op/s</sub> | -7.6% (-0.018) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.340<br><sub>context: p90 0.389 · p95 0.415 · p99 0.433 · 2856 op/s</sub> | 0.214<br><sub>context: p90 0.249 · p95 0.259 · p99 0.284 · 4480 op/s</sub> | -37.0% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.438<br><sub>context: p90 0.524 · p95 0.556 · p99 0.614 · 17822 op/s</sub> | 0.328<br><sub>context: p90 0.418 · p95 0.449 · p99 0.497 · 23445 op/s</sub> | -25.1% (-0.110) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.355<br><sub>context: p90 0.401 · p95 0.414 · p99 0.460 · 2746 op/s</sub> | 0.243<br><sub>context: p90 0.306 · p95 0.337 · p99 0.391 · 3921 op/s</sub> | -31.6% (-0.112) | 150% AND 2 ms | 🟢 |
| 8 | 0.473<br><sub>context: p90 0.565 · p95 0.596 · p99 0.662 · 16422 op/s</sub> | 0.354<br><sub>context: p90 0.454 · p95 0.482 · p99 0.542 · 21481 op/s</sub> | -25.1% (-0.119) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.412<br><sub>context: p90 0.522 · p95 0.547 · p99 0.584 · 2328 op/s</sub> | 0.356<br><sub>context: p90 0.406 · p95 0.412 · p99 0.449 · 2768 op/s</sub> | -13.6% (-0.056) | 150% AND 2 ms | 🟢 |
| 8 | 0.531<br><sub>context: p90 0.638 · p95 0.673 · p99 0.744 · 14743 op/s</sub> | 0.451<br><sub>context: p90 0.546 · p95 0.581 · p99 0.642 · 17114 op/s</sub> | -15.1% (-0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.449<br><sub>context: p90 0.507 · p95 0.533 · p99 0.592 · 2204 op/s</sub> | 0.389<br><sub>context: p90 0.458 · p95 0.474 · p99 0.510 · 2538 op/s</sub> | -13.3% (-0.060) | 150% AND 2 ms | 🟢 |
| 8 | 0.560<br><sub>context: p90 0.659 · p95 0.699 · p99 0.751 · 13873 op/s</sub> | 0.479<br><sub>context: p90 0.585 · p95 0.620 · p99 0.703 · 16193 op/s</sub> | -14.5% (-0.081) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.562<br><sub>context: p90 0.706 · p95 0.749 · p99 0.931 · 1706 op/s</sub> | 0.482<br><sub>context: p90 0.597 · p95 0.623 · p99 0.722 · 2008 op/s</sub> | -14.3% (-0.080) | 150% AND 2 ms | 🟢 |
| 8 | 0.738<br><sub>context: p90 0.940 · p95 1.000 · p99 1.098 · 10417 op/s</sub> | 0.640<br><sub>context: p90 0.820 · p95 0.884 · p99 1.043 · 11997 op/s</sub> | -13.3% (-0.098) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.603<br><sub>context: p90 0.740 · p95 0.804 · p99 0.898 · 1594 op/s</sub> | 0.539<br><sub>context: p90 0.673 · p95 0.704 · p99 0.774 · 1798 op/s</sub> | -10.5% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.785<br><sub>context: p90 0.990 · p95 1.063 · p99 1.237 · 9869 op/s</sub> | 0.670<br><sub>context: p90 0.848 · p95 0.901 · p99 1.032 · 11537 op/s</sub> | -14.7% (-0.115) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.195<br><sub>context: p90 1.644 · p95 1.788 · p99 2.048 · 811 op/s</sub> | 0.999<br><sub>context: p90 1.327 · p95 1.475 · p99 1.577 · 990 op/s</sub> | -16.4% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 1.549<br><sub>context: p90 2.193 · p95 2.383 · p99 2.777 · 4941 op/s</sub> | 1.304<br><sub>context: p90 1.841 · p95 2.013 · p99 2.299 · 5880 op/s</sub> | -15.8% (-0.245) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.230<br><sub>context: p90 1.695 · p95 1.906 · p99 2.143 · 780 op/s</sub> | 1.059<br><sub>context: p90 1.418 · p95 1.612 · p99 1.789 · 919 op/s</sub> | -13.9% (-0.171) | 150% AND 2 ms | 🟢 |
| 8 | 1.604<br><sub>context: p90 2.252 · p95 2.504 · p99 2.911 · 4715 op/s</sub> | 1.319<br><sub>context: p90 1.863 · p95 2.033 · p99 2.334 · 5724 op/s</sub> | -17.8% (-0.285) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.581<br><sub>context: p90 0.650 · p95 0.669 · p99 0.702 · 1711 op/s</sub> | 0.587<br><sub>context: p90 0.770 · p95 0.804 · p99 0.948 · 1659 op/s</sub> | +1.0% (+0.006) | 150% AND 2 ms | 🟢 |
| 8 | 0.771<br><sub>context: p90 0.899 · p95 0.947 · p99 1.038 · 10214 op/s</sub> | 0.783<br><sub>context: p90 1.035 · p95 1.102 · p99 1.246 · 10036 op/s</sub> | +1.6% (+0.013) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.413<br><sub>context: p90 0.484 · p95 0.501 · p99 0.552 · 2352 op/s</sub> | 0.299<br><sub>context: p90 0.363 · p95 0.382 · p99 0.430 · 3222 op/s</sub> | -27.7% (-0.114) | 150% AND 2 ms | 🟢 |
| 8 | 0.552<br><sub>context: p90 0.652 · p95 0.684 · p99 0.781 · 14081 op/s</sub> | 0.407<br><sub>context: p90 0.508 · p95 0.538 · p99 0.579 · 19129 op/s</sub> | -26.4% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.228<br><sub>context: p90 0.281 · p95 0.304 · p99 0.348 · 4156 op/s</sub> | 0.152<br><sub>context: p90 0.184 · p95 0.199 · p99 0.243 · 6114 op/s</sub> | -33.4% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.408 · p95 0.438 · p99 0.506 · 22999 op/s</sub> | 0.222<br><sub>context: p90 0.290 · p95 0.310 · p99 0.360 · 34463 op/s</sub> | -33.2% (-0.111) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.183<br><sub>context: p90 0.231 · p95 0.271 · p99 0.307 · 4920 op/s</sub> | 0.119<br><sub>context: p90 0.140 · p95 0.144 · p99 0.161 · 8156 op/s</sub> | -35.1% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.336 · p95 0.356 · p99 0.406 · 27518 op/s</sub> | 0.197<br><sub>context: p90 0.270 · p95 0.295 · p99 0.369 · 38090 op/s</sub> | -26.2% (-0.070) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.525<br><sub>context: p90 0.613 · p95 0.631 · p99 0.675 · 1869 op/s</sub> | 0.357<br><sub>context: p90 0.406 · p95 0.431 · p99 0.468 · 2796 op/s</sub> | -32.0% (-0.168) | 150% AND 2 ms | 🟢 |
| 8 | 0.667<br><sub>context: p90 0.786 · p95 0.821 · p99 0.889 · 11657 op/s</sub> | 0.441<br><sub>context: p90 0.545 · p95 0.576 · p99 0.630 · 17430 op/s</sub> | -33.9% (-0.226) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.784<br><sub>context: p90 1.074 · p95 1.157 · p99 1.316 · 1230 op/s</sub> | 1.570<br><sub>context: p90 2.313 · p95 2.415 · p99 2.649 · 623 op/s</sub> | +100.2% (+0.786) | 150% AND 2 ms | 🟢 |
| 8 | 1.048<br><sub>context: p90 1.562 · p95 1.730 · p99 2.040 · 7117 op/s</sub> | 2.278<br><sub>context: p90 3.572 · p95 3.959 · p99 4.630 · 3375 op/s</sub> | +117.3% (+1.229) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.391<br><sub>context: p90 3.730 · p95 4.251 · p99 4.594 · 397 op/s</sub> | 4.932<br><sub>context: p90 7.523 · p95 8.248 · p99 8.710 · 196 op/s</sub> | +106.3% (+2.541) | 150% AND 2 ms | 🟢 |
| 8 | 3.224<br><sub>context: p90 5.238 · p95 5.934 · p99 7.144 · 2310 op/s</sub> | 8.702<br><sub>context: p90 13.267 · p95 14.489 · p99 16.345 · 890 op/s</sub> | +169.9% (+5.478) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.310 · p95 0.325 · p99 0.361 · 3922 op/s</sub> | 0.170<br><sub>context: p90 0.196 · p95 0.202 · p99 0.216 · 5693 op/s</sub> | -29.2% (-0.070) | 150% AND 2 ms | 🟢 |
| 8 | 0.360<br><sub>context: p90 0.437 · p95 0.462 · p99 0.512 · 21545 op/s</sub> | 0.299<br><sub>context: p90 0.402 · p95 0.434 · p99 0.517 · 25341 op/s</sub> | -17.1% (-0.062) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.239<br><sub>context: p90 0.294 · p95 0.332 · p99 0.353 · 3987 op/s</sub> | 0.189<br><sub>context: p90 0.213 · p95 0.229 · p99 0.238 · 5082 op/s</sub> | -20.9% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.362<br><sub>context: p90 0.443 · p95 0.465 · p99 0.514 · 21247 op/s</sub> | 0.296<br><sub>context: p90 0.388 · p95 0.419 · p99 0.478 · 25870 op/s</sub> | -18.1% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.360<br><sub>context: p90 0.430 · p95 0.462 · p99 0.476 · 2662 op/s</sub> | 0.252<br><sub>context: p90 0.306 · p95 0.317 · p99 0.359 · 3760 op/s</sub> | -29.9% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.480<br><sub>context: p90 0.594 · p95 0.636 · p99 0.728 · 15944 op/s</sub> | 0.362<br><sub>context: p90 0.465 · p95 0.513 · p99 0.590 · 20880 op/s</sub> | -24.5% (-0.117) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.201<br><sub>context: p90 0.242 · p95 0.256 · p99 0.288 · 4701 op/s</sub> | 0.135<br><sub>context: p90 0.165 · p95 0.172 · p99 0.180 · 6994 op/s</sub> | -32.7% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.291<br><sub>context: p90 0.358 · p95 0.383 · p99 0.429 · 26433 op/s</sub> | 0.223<br><sub>context: p90 0.292 · p95 0.311 · p99 0.348 · 34679 op/s</sub> | -23.2% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.361<br><sub>context: p90 0.439 · p95 0.469 · p99 0.502 · 2670 op/s</sub> | 0.338<br><sub>context: p90 0.387 · p95 0.401 · p99 0.421 · 2934 op/s</sub> | -6.3% (-0.023) | 150% AND 2 ms | 🟢 |
| 8 | 0.452<br><sub>context: p90 0.536 · p95 0.566 · p99 0.622 · 17144 op/s</sub> | 0.387<br><sub>context: p90 0.470 · p95 0.499 · p99 0.552 · 20011 op/s</sub> | -14.4% (-0.065) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.340 · p95 0.372 · p99 0.407 · 3460 op/s</sub> | 0.234<br><sub>context: p90 0.271 · p95 0.289 · p99 0.329 · 4108 op/s</sub> | -16.7% (-0.047) | 150% AND 2 ms | 🟢 |
| 8 | 0.381<br><sub>context: p90 0.459 · p95 0.479 · p99 0.536 · 20331 op/s</sub> | 0.327<br><sub>context: p90 0.413 · p95 0.439 · p99 0.500 · 23443 op/s</sub> | -14.2% (-0.054) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.421<br><sub>context: p90 0.504 · p95 0.534 · p99 0.598 · 2334 op/s</sub> | 0.346<br><sub>context: p90 0.397 · p95 0.419 · p99 0.455 · 2855 op/s</sub> | -17.8% (-0.075) | 150% AND 2 ms | 🟢 |
| 8 | 0.508<br><sub>context: p90 0.611 · p95 0.645 · p99 0.709 · 15163 op/s</sub> | 0.436<br><sub>context: p90 0.538 · p95 0.568 · p99 0.639 · 17595 op/s</sub> | -14.2% (-0.072) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.497<br><sub>context: p90 0.606 · p95 0.633 · p99 0.662 · 1971 op/s</sub> | 0.490<br><sub>context: p90 0.595 · p95 0.626 · p99 0.689 · 2014 op/s</sub> | -1.4% (-0.007) | 150% AND 2 ms | 🟢 |
| 8 | 0.663<br><sub>context: p90 0.830 · p95 0.877 · p99 1.030 · 11711 op/s</sub> | 0.620<br><sub>context: p90 0.798 · p95 0.843 · p99 0.935 · 12453 op/s</sub> | -6.4% (-0.042) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.566<br><sub>context: p90 0.692 · p95 0.761 · p99 0.788 · 1735 op/s</sub> | 0.485<br><sub>context: p90 0.591 · p95 0.618 · p99 0.684 · 2010 op/s</sub> | -14.3% (-0.081) | 150% AND 2 ms | 🟢 |
| 8 | 0.666<br><sub>context: p90 0.821 · p95 0.883 · p99 1.024 · 11494 op/s</sub> | 0.634<br><sub>context: p90 0.791 · p95 0.839 · p99 0.959 · 12184 op/s</sub> | -4.8% (-0.032) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.487<br><sub>context: p90 0.602 · p95 0.631 · p99 0.716 · 1995 op/s</sub> | 0.361<br><sub>context: p90 0.410 · p95 0.417 · p99 0.443 · 2752 op/s</sub> | -25.9% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.549<br><sub>context: p90 0.658 · p95 0.693 · p99 0.780 · 14171 op/s</sub> | 0.463<br><sub>context: p90 0.568 · p95 0.604 · p99 0.660 · 16536 op/s</sub> | -15.6% (-0.086) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.407<br><sub>context: p90 0.445 · p95 0.462 · p99 0.490 · 2410 op/s</sub> | 0.300<br><sub>context: p90 0.359 · p95 0.384 · p99 0.470 · 3244 op/s</sub> | -26.2% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.549<br><sub>context: p90 0.647 · p95 0.682 · p99 0.742 · 14299 op/s</sub> | 0.389<br><sub>context: p90 0.483 · p95 0.509 · p99 0.562 · 19784 op/s</sub> | -29.1% (-0.160) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.402<br><sub>context: p90 18.554 · p95 18.644 · p99 18.718 · 54 op/s</sub> | 15.252<br><sub>context: p90 15.419 · p95 15.521 · p99 15.700 · 66 op/s</sub> | -17.1% (-3.150) | 150% AND 2 ms | 🟢 |
| 8 | 23.938<br><sub>context: p90 32.661 · p95 35.696 · p99 40.431 · 309 op/s</sub> | 18.231<br><sub>context: p90 24.498 · p95 27.285 · p99 31.201 · 393 op/s</sub> | -23.8% (-5.707) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.482<br><sub>context: p90 0.544 · p95 0.568 · p99 0.602 · 2031 op/s</sub> | 0.381<br><sub>context: p90 0.472 · p95 0.497 · p99 0.537 · 2526 op/s</sub> | -20.9% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.646<br><sub>context: p90 0.756 · p95 0.791 · p99 0.867 · 12061 op/s</sub> | 0.449<br><sub>context: p90 0.545 · p95 0.575 · p99 0.637 · 17271 op/s</sub> | -30.5% (-0.197) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.478<br><sub>context: p90 1.982 · p95 2.216 · p99 2.676 · 664 op/s</sub> | 1.276<br><sub>context: p90 1.730 · p95 1.850 · p99 2.028 · 765 op/s</sub> | -13.7% (-0.202) | 150% AND 2 ms | 🟢 |
| 8 | 1.819<br><sub>context: p90 2.565 · p95 2.840 · p99 3.358 · 4253 op/s</sub> | 1.597<br><sub>context: p90 2.248 · p95 2.482 · p99 2.968 · 4841 op/s</sub> | -12.2% (-0.221) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.421<br><sub>context: p90 0.504 · p95 0.534 · p99 0.582 · 2315 op/s</sub> | 0.382<br><sub>context: p90 0.485 · p95 0.520 · p99 0.559 · 2510 op/s</sub> | -9.3% (-0.039) | 150% AND 2 ms | 🟢 |
| 8 | 0.559<br><sub>context: p90 0.673 · p95 0.711 · p99 0.774 · 13970 op/s</sub> | 0.490<br><sub>context: p90 0.606 · p95 0.638 · p99 0.729 · 15935 op/s</sub> | -12.4% (-0.069) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.256<br><sub>context: p90 2.812 · p95 2.871 · p99 2.998 · 455 op/s</sub> | 0.366<br><sub>context: p90 0.455 · p95 0.481 · p99 0.538 · 2665 op/s</sub> | -83.8% (-1.890) | 150% AND 2 ms | 🟢 |
| 8 | 2.559<br><sub>context: p90 3.302 · p95 3.497 · p99 3.710 · 3145 op/s</sub> | 0.433<br><sub>context: p90 0.541 · p95 0.576 · p99 0.651 · 17816 op/s</sub> | -83.1% (-2.127) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.254<br><sub>context: p90 2.865 · p95 2.924 · p99 3.083 · 457 op/s</sub> | 0.367<br><sub>context: p90 0.434 · p95 0.454 · p99 0.482 · 2701 op/s</sub> | -83.7% (-1.887) | 150% AND 2 ms | 🟢 |
| 8 | 2.579<br><sub>context: p90 3.334 · p95 3.541 · p99 3.783 · 3141 op/s</sub> | 0.447<br><sub>context: p90 0.557 · p95 0.596 · p99 0.680 · 17256 op/s</sub> | -82.7% (-2.131) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.239<br><sub>context: p90 0.289 · p95 0.313 · p99 0.345 · 4090 op/s</sub> | 0.178<br><sub>context: p90 0.214 · p95 0.222 · p99 0.251 · 5440 op/s</sub> | -25.8% (-0.062) | 150% AND 2 ms | 🟢 |
| 8 | 0.302<br><sub>context: p90 0.377 · p95 0.405 · p99 0.452 · 25167 op/s</sub> | 0.300<br><sub>context: p90 0.410 · p95 0.445 · p99 0.527 · 25159 op/s</sub> | -0.8% (-0.002) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.320<br><sub>context: p90 0.369 · p95 0.387 · p99 0.409 · 3056 op/s</sub> | 0.168<br><sub>context: p90 0.190 · p95 0.196 · p99 0.216 · 5686 op/s</sub> | -47.6% (-0.152) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.414<br><sub>context: p90 0.490 · p95 0.515 · p99 0.569 · 18744 op/s</sub> | 0.243<br><sub>context: p90 0.304 · p95 0.323 · p99 0.368 · 31838 op/s</sub> | -41.5% (-0.172) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.312<br><sub>context: p90 0.363 · p95 0.379 · p99 0.405 · 3135 op/s</sub> | 0.217<br><sub>context: p90 0.251 · p95 0.269 · p99 0.308 · 4410 op/s</sub> | -30.5% (-0.095) | 150% AND 2 ms | 🟢 |
| 8 | 0.432<br><sub>context: p90 0.518 · p95 0.547 · p99 0.614 · 17999 op/s</sub> | 0.363<br><sub>context: p90 0.465 · p95 0.501 · p99 0.575 · 21079 op/s</sub> | -16.0% (-0.069) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.375 · p95 0.405 · p99 0.460 · 3045 op/s</sub> | 0.253<br><sub>context: p90 0.324 · p95 0.341 · p99 0.384 · 3728 op/s</sub> | -19.8% (-0.062) | 150% AND 2 ms | 🟢 |
| 8 | 0.444<br><sub>context: p90 0.536 · p95 0.573 · p99 0.650 · 17394 op/s</sub> | 0.369<br><sub>context: p90 0.483 · p95 0.520 · p99 0.587 · 20544 op/s</sub> | -16.8% (-0.075) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.324 · p95 0.339 · p99 0.394 · 3638 op/s</sub> | 0.216<br><sub>context: p90 0.257 · p95 0.267 · p99 0.319 · 4581 op/s</sub> | -17.4% (-0.046) | 150% AND 2 ms | 🟢 |
| 8 | 0.387<br><sub>context: p90 0.467 · p95 0.490 · p99 0.547 · 20071 op/s</sub> | 0.311<br><sub>context: p90 0.403 · p95 0.436 · p99 0.513 · 24386 op/s</sub> | -19.6% (-0.076) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.374<br><sub>context: p90 1.455 · p95 1.492 · p99 1.566 · 723 op/s</sub> | 0.935<br><sub>context: p90 0.989 · p95 0.999 · p99 1.026 · 1062 op/s</sub> | -31.9% (-0.439) | 150% AND 2 ms | 🟢 |
| 8 | 1.827<br><sub>context: p90 2.192 · p95 2.330 · p99 2.677 · 4256 op/s</sub> | 1.185<br><sub>context: p90 1.345 · p95 1.402 · p99 1.507 · 6648 op/s</sub> | -35.1% (-0.642) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.287<br><sub>context: p90 1.393 · p95 1.416 · p99 1.501 · 772 op/s</sub> | 0.939<br><sub>context: p90 0.977 · p95 0.990 · p99 1.018 · 1061 op/s</sub> | -27.0% (-0.348) | 150% AND 2 ms | 🟢 |
| 8 | 1.622<br><sub>context: p90 1.989 · p95 2.083 · p99 2.415 · 4760 op/s</sub> | 1.094<br><sub>context: p90 1.219 · p95 1.261 · p99 1.377 · 7258 op/s</sub> | -32.5% (-0.528) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.340<br><sub>context: p90 0.389 · p95 0.403 · p99 0.440 · 2878 op/s</sub> | 0.252<br><sub>context: p90 0.310 · p95 0.342 · p99 0.363 · 3835 op/s</sub> | -26.1% (-0.089) | 150% AND 2 ms | 🟢 |
| 8 | 0.468<br><sub>context: p90 0.571 · p95 0.601 · p99 0.664 · 16440 op/s</sub> | 0.381<br><sub>context: p90 0.484 · p95 0.515 · p99 0.585 · 20044 op/s</sub> | -18.7% (-0.088) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.457<br><sub>context: p90 0.530 · p95 0.543 · p99 0.589 · 2163 op/s</sub> | 0.386<br><sub>context: p90 0.479 · p95 0.506 · p99 0.562 · 2515 op/s</sub> | -15.5% (-0.071) | 150% AND 2 ms | 🟢 |
| 8 | 0.575<br><sub>context: p90 0.688 · p95 0.733 · p99 0.812 · 13523 op/s</sub> | 0.486<br><sub>context: p90 0.612 · p95 0.647 · p99 0.722 · 15953 op/s</sub> | -15.5% (-0.089) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.287<br><sub>context: p90 0.343 · p95 0.359 · p99 0.378 · 3389 op/s</sub> | 0.180<br><sub>context: p90 0.212 · p95 0.216 · p99 0.224 · 5323 op/s</sub> | -37.2% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.405<br><sub>context: p90 0.487 · p95 0.518 · p99 0.577 · 19182 op/s</sub> | 0.320<br><sub>context: p90 0.418 · p95 0.454 · p99 0.513 · 23358 op/s</sub> | -20.9% (-0.085) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.234<br><sub>context: p90 0.297 · p95 0.311 · p99 0.342 · 4046 op/s</sub> | 0.169<br><sub>context: p90 0.204 · p95 0.213 · p99 0.252 · 5718 op/s</sub> | -27.7% (-0.065) | 150% AND 2 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.366 · p95 0.389 · p99 0.437 · 26034 op/s</sub> | 0.303<br><sub>context: p90 0.410 · p95 0.455 · p99 0.529 · 24873 op/s</sub> | +2.1% (+0.006) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.225<br><sub>context: p90 0.259 · p95 0.265 · p99 0.292 · 4282 op/s</sub> | 0.180<br><sub>context: p90 0.218 · p95 0.232 · p99 0.287 · 5369 op/s</sub> | -19.9% (-0.045) | 150% AND 2 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.366 · p95 0.391 · p99 0.436 · 25556 op/s</sub> | 0.302<br><sub>context: p90 0.408 · p95 0.440 · p99 0.506 · 24868 op/s</sub> | +1.1% (+0.003) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.965<br><sub>context: p90 1.028 · p95 1.053 · p99 1.138 · 1021 op/s</sub> | 0.388<br><sub>context: p90 0.462 · p95 0.484 · p99 0.512 · 2464 op/s</sub> | -59.8% (-0.577) | 150% AND 2 ms | 🟢 |
| 8 | 1.278<br><sub>context: p90 1.627 · p95 1.876 · p99 2.166 · 5930 op/s</sub> | 0.463<br><sub>context: p90 0.536 · p95 0.562 · p99 0.624 · 16856 op/s</sub> | -63.8% (-0.815) | 150% AND 2 ms | 🟢 |

</details>
