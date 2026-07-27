### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3 |
| workload_hash | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.374<br><sub>context: p90 1.451 · p95 1.479 · p99 1.568 · 720 op/s</sub> | 0.759<br><sub>context: p90 0.824 · p95 0.860 · p99 0.901 · 1294 op/s</sub> | -44.8% (-0.615) | 150% AND 2 ms | 🟢 |
| 8 | 1.883<br><sub>context: p90 2.440 · p95 2.681 · p99 3.073 · 4038 op/s</sub> | 0.889<br><sub>context: p90 1.022 · p95 1.056 · p99 1.155 · 8888 op/s</sub> | -52.8% (-0.994) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.078<br><sub>context: p90 2.182 · p95 2.223 · p99 2.322 · 474 op/s</sub> | 0.817<br><sub>context: p90 0.882 · p95 0.907 · p99 0.973 · 1208 op/s</sub> | -60.7% (-1.261) | 150% AND 2 ms | 🟢 |
| 8 | 2.659<br><sub>context: p90 3.454 · p95 3.568 · p99 4.099 · 2863 op/s</sub> | 0.983<br><sub>context: p90 1.161 · p95 1.193 · p99 1.266 · 8003 op/s</sub> | -63.0% (-1.676) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.033<br><sub>context: p90 2.133 · p95 2.165 · p99 2.256 · 488 op/s</sub> | 1.309<br><sub>context: p90 1.388 · p95 1.406 · p99 1.462 · 756 op/s</sub> | -35.6% (-0.723) | 150% AND 2 ms | 🟢 |
| 8 | 2.684<br><sub>context: p90 3.509 · p95 3.699 · p99 4.167 · 2824 op/s</sub> | 1.810<br><sub>context: p90 2.267 · p95 2.491 · p99 2.805 · 4289 op/s</sub> | -32.6% (-0.874) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.654<br><sub>context: p90 2.802 · p95 2.834 · p99 2.943 · 374 op/s</sub> | 1.410<br><sub>context: p90 1.508 · p95 1.530 · p99 1.595 · 702 op/s</sub> | -46.9% (-1.244) | 150% AND 2 ms | 🟢 |
| 8 | 3.658<br><sub>context: p90 4.699 · p95 4.970 · p99 5.498 · 2121 op/s</sub> | 1.608<br><sub>context: p90 1.942 · p95 1.983 · p99 2.053 · 4791 op/s</sub> | -56.1% (-2.050) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.183 · p95 0.193 · p99 0.211 · 6126 op/s</sub> | 0.129<br><sub>context: p90 0.160 · p95 0.167 · p99 0.192 · 7197 op/s</sub> | -17.0% (-0.027) | 150% AND 2 ms | 🟢 |
| 8 | 0.238<br><sub>context: p90 0.302 · p95 0.322 · p99 0.366 · 30895 op/s</sub> | 0.219<br><sub>context: p90 0.284 · p95 0.305 · p99 0.347 · 35030 op/s</sub> | -7.9% (-0.019) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.340<br><sub>context: p90 0.389 · p95 0.415 · p99 0.433 · 2856 op/s</sub> | 0.216<br><sub>context: p90 0.262 · p95 0.282 · p99 0.295 · 4418 op/s</sub> | -36.3% (-0.123) | 150% AND 2 ms | 🟢 |
| 8 | 0.438<br><sub>context: p90 0.524 · p95 0.556 · p99 0.614 · 17822 op/s</sub> | 0.335<br><sub>context: p90 0.419 · p95 0.451 · p99 0.505 · 22904 op/s</sub> | -23.5% (-0.103) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.355<br><sub>context: p90 0.401 · p95 0.414 · p99 0.460 · 2746 op/s</sub> | 0.258<br><sub>context: p90 0.337 · p95 0.354 · p99 0.397 · 3566 op/s</sub> | -27.4% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.473<br><sub>context: p90 0.565 · p95 0.596 · p99 0.662 · 16422 op/s</sub> | 0.349<br><sub>context: p90 0.446 · p95 0.476 · p99 0.536 · 21459 op/s</sub> | -26.1% (-0.124) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.412<br><sub>context: p90 0.522 · p95 0.547 · p99 0.584 · 2328 op/s</sub> | 0.347<br><sub>context: p90 0.419 · p95 0.445 · p99 0.467 · 2824 op/s</sub> | -15.6% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.531<br><sub>context: p90 0.638 · p95 0.673 · p99 0.744 · 14743 op/s</sub> | 0.456<br><sub>context: p90 0.557 · p95 0.585 · p99 0.649 · 17076 op/s</sub> | -14.2% (-0.075) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.449<br><sub>context: p90 0.507 · p95 0.533 · p99 0.592 · 2204 op/s</sub> | 0.379<br><sub>context: p90 0.463 · p95 0.486 · p99 0.525 · 2557 op/s</sub> | -15.6% (-0.070) | 150% AND 2 ms | 🟢 |
| 8 | 0.560<br><sub>context: p90 0.659 · p95 0.699 · p99 0.751 · 13873 op/s</sub> | 0.479<br><sub>context: p90 0.584 · p95 0.618 · p99 0.674 · 16216 op/s</sub> | -14.4% (-0.081) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.562<br><sub>context: p90 0.706 · p95 0.749 · p99 0.931 · 1706 op/s</sub> | 0.522<br><sub>context: p90 0.622 · p95 0.669 · p99 0.759 · 1900 op/s</sub> | -7.2% (-0.040) | 150% AND 2 ms | 🟢 |
| 8 | 0.738<br><sub>context: p90 0.940 · p95 1.000 · p99 1.098 · 10417 op/s</sub> | 0.637<br><sub>context: p90 0.797 · p95 0.853 · p99 0.966 · 12141 op/s</sub> | -13.7% (-0.101) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.603<br><sub>context: p90 0.740 · p95 0.804 · p99 0.898 · 1594 op/s</sub> | 0.576<br><sub>context: p90 0.719 · p95 0.769 · p99 0.864 · 1708 op/s</sub> | -4.4% (-0.027) | 150% AND 2 ms | 🟢 |
| 8 | 0.785<br><sub>context: p90 0.990 · p95 1.063 · p99 1.237 · 9869 op/s</sub> | 0.670<br><sub>context: p90 0.848 · p95 0.915 · p99 1.012 · 11480 op/s</sub> | -14.6% (-0.115) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.195<br><sub>context: p90 1.644 · p95 1.788 · p99 2.048 · 811 op/s</sub> | 1.064<br><sub>context: p90 1.441 · p95 1.538 · p99 1.768 · 932 op/s</sub> | -11.0% (-0.132) | 150% AND 2 ms | 🟢 |
| 8 | 1.549<br><sub>context: p90 2.193 · p95 2.383 · p99 2.777 · 4941 op/s</sub> | 1.303<br><sub>context: p90 1.798 · p95 1.964 · p99 2.306 · 5932 op/s</sub> | -15.9% (-0.246) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.230<br><sub>context: p90 1.695 · p95 1.906 · p99 2.143 · 780 op/s</sub> | 1.045<br><sub>context: p90 1.389 · p95 1.494 · p99 1.731 · 926 op/s</sub> | -15.0% (-0.185) | 150% AND 2 ms | 🟢 |
| 8 | 1.604<br><sub>context: p90 2.252 · p95 2.504 · p99 2.911 · 4715 op/s</sub> | 1.327<br><sub>context: p90 1.826 · p95 1.989 · p99 2.275 · 5799 op/s</sub> | -17.3% (-0.277) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.581<br><sub>context: p90 0.650 · p95 0.669 · p99 0.702 · 1711 op/s</sub> | 0.573<br><sub>context: p90 0.747 · p95 0.791 · p99 0.859 · 1699 op/s</sub> | -1.5% (-0.009) | 150% AND 2 ms | 🟢 |
| 8 | 0.771<br><sub>context: p90 0.899 · p95 0.947 · p99 1.038 · 10214 op/s</sub> | 0.782<br><sub>context: p90 1.054 · p95 1.134 · p99 1.305 · 9876 op/s</sub> | +1.5% (+0.011) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.413<br><sub>context: p90 0.484 · p95 0.501 · p99 0.552 · 2352 op/s</sub> | 0.342<br><sub>context: p90 0.421 · p95 0.439 · p99 0.550 · 2813 op/s</sub> | -17.2% (-0.071) | 150% AND 2 ms | 🟢 |
| 8 | 0.552<br><sub>context: p90 0.652 · p95 0.684 · p99 0.781 · 14081 op/s</sub> | 0.403<br><sub>context: p90 0.505 · p95 0.537 · p99 0.593 · 19018 op/s</sub> | -27.1% (-0.150) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.228<br><sub>context: p90 0.281 · p95 0.304 · p99 0.348 · 4156 op/s</sub> | 0.155<br><sub>context: p90 0.196 · p95 0.211 · p99 0.230 · 5676 op/s</sub> | -32.0% (-0.073) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.408 · p95 0.438 · p99 0.506 · 22999 op/s</sub> | 0.224<br><sub>context: p90 0.298 · p95 0.318 · p99 0.363 · 34302 op/s</sub> | -32.6% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.183<br><sub>context: p90 0.231 · p95 0.271 · p99 0.307 · 4920 op/s</sub> | 0.116<br><sub>context: p90 0.138 · p95 0.144 · p99 0.165 · 8331 op/s</sub> | -36.6% (-0.067) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.336 · p95 0.356 · p99 0.406 · 27518 op/s</sub> | 0.203<br><sub>context: p90 0.282 · p95 0.302 · p99 0.338 · 37127 op/s</sub> | -23.8% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.525<br><sub>context: p90 0.613 · p95 0.631 · p99 0.675 · 1869 op/s</sub> | 0.361<br><sub>context: p90 0.422 · p95 0.443 · p99 0.465 · 2721 op/s</sub> | -31.2% (-0.164) | 150% AND 2 ms | 🟢 |
| 8 | 0.667<br><sub>context: p90 0.786 · p95 0.821 · p99 0.889 · 11657 op/s</sub> | 0.442<br><sub>context: p90 0.548 · p95 0.581 · p99 0.647 · 17280 op/s</sub> | -33.7% (-0.225) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.784<br><sub>context: p90 1.074 · p95 1.157 · p99 1.316 · 1230 op/s</sub> | 1.414<br><sub>context: p90 2.069 · p95 2.314 · p99 2.627 · 684 op/s</sub> | +80.4% (+0.630) | 150% AND 2 ms | 🟢 |
| 8 | 1.048<br><sub>context: p90 1.562 · p95 1.730 · p99 2.040 · 7117 op/s</sub> | 2.270<br><sub>context: p90 3.517 · p95 3.950 · p99 4.492 · 3375 op/s</sub> | +116.5% (+1.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.391<br><sub>context: p90 3.730 · p95 4.251 · p99 4.594 · 397 op/s</sub> | 4.903<br><sub>context: p90 7.356 · p95 8.238 · p99 8.868 · 196 op/s</sub> | +105.0% (+2.511) | 150% AND 2 ms | 🟢 |
| 8 | 3.224<br><sub>context: p90 5.238 · p95 5.934 · p99 7.144 · 2310 op/s</sub> | 8.452<br><sub>context: p90 13.028 · p95 14.329 · p99 15.939 · 912 op/s</sub> | +162.2% (+5.228) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.310 · p95 0.325 · p99 0.361 · 3922 op/s</sub> | 0.177<br><sub>context: p90 0.205 · p95 0.220 · p99 0.252 · 5235 op/s</sub> | -26.0% (-0.062) | 150% AND 2 ms | 🟢 |
| 8 | 0.360<br><sub>context: p90 0.437 · p95 0.462 · p99 0.512 · 21545 op/s</sub> | 0.296<br><sub>context: p90 0.409 · p95 0.442 · p99 0.508 · 25438 op/s</sub> | -17.7% (-0.064) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.239<br><sub>context: p90 0.294 · p95 0.332 · p99 0.353 · 3987 op/s</sub> | 0.178<br><sub>context: p90 0.206 · p95 0.211 · p99 0.226 · 5400 op/s</sub> | -25.4% (-0.061) | 150% AND 2 ms | 🟢 |
| 8 | 0.362<br><sub>context: p90 0.443 · p95 0.465 · p99 0.514 · 21247 op/s</sub> | 0.298<br><sub>context: p90 0.414 · p95 0.452 · p99 0.533 · 25128 op/s</sub> | -17.8% (-0.064) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.360<br><sub>context: p90 0.430 · p95 0.462 · p99 0.476 · 2662 op/s</sub> | 0.236<br><sub>context: p90 0.267 · p95 0.276 · p99 0.296 · 4129 op/s</sub> | -34.4% (-0.124) | 150% AND 2 ms | 🟢 |
| 8 | 0.480<br><sub>context: p90 0.594 · p95 0.636 · p99 0.728 · 15944 op/s</sub> | 0.357<br><sub>context: p90 0.446 · p95 0.472 · p99 0.539 · 21557 op/s</sub> | -25.6% (-0.123) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.201<br><sub>context: p90 0.242 · p95 0.256 · p99 0.288 · 4701 op/s</sub> | 0.161<br><sub>context: p90 0.184 · p95 0.191 · p99 0.217 · 5916 op/s</sub> | -19.8% (-0.040) | 150% AND 2 ms | 🟢 |
| 8 | 0.291<br><sub>context: p90 0.358 · p95 0.383 · p99 0.429 · 26433 op/s</sub> | 0.225<br><sub>context: p90 0.292 · p95 0.311 · p99 0.355 · 34252 op/s</sub> | -22.8% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.361<br><sub>context: p90 0.439 · p95 0.469 · p99 0.502 · 2670 op/s</sub> | 0.340<br><sub>context: p90 0.395 · p95 0.433 · p99 0.489 · 2875 op/s</sub> | -5.8% (-0.021) | 150% AND 2 ms | 🟢 |
| 8 | 0.452<br><sub>context: p90 0.536 · p95 0.566 · p99 0.622 · 17144 op/s</sub> | 0.397<br><sub>context: p90 0.492 · p95 0.523 · p99 0.576 · 19336 op/s</sub> | -12.1% (-0.055) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.340 · p95 0.372 · p99 0.407 · 3460 op/s</sub> | 0.246<br><sub>context: p90 0.301 · p95 0.324 · p99 0.358 · 3887 op/s</sub> | -12.4% (-0.035) | 150% AND 2 ms | 🟢 |
| 8 | 0.381<br><sub>context: p90 0.459 · p95 0.479 · p99 0.536 · 20331 op/s</sub> | 0.324<br><sub>context: p90 0.417 · p95 0.450 · p99 0.505 · 23484 op/s</sub> | -14.8% (-0.056) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.421<br><sub>context: p90 0.504 · p95 0.534 · p99 0.598 · 2334 op/s</sub> | 0.355<br><sub>context: p90 0.400 · p95 0.415 · p99 0.461 · 2794 op/s</sub> | -15.7% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.508<br><sub>context: p90 0.611 · p95 0.645 · p99 0.709 · 15163 op/s</sub> | 0.446<br><sub>context: p90 0.551 · p95 0.584 · p99 0.647 · 17381 op/s</sub> | -12.3% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.497<br><sub>context: p90 0.606 · p95 0.633 · p99 0.662 · 1971 op/s</sub> | 0.475<br><sub>context: p90 0.593 · p95 0.626 · p99 0.708 · 2046 op/s</sub> | -4.6% (-0.023) | 150% AND 2 ms | 🟢 |
| 8 | 0.663<br><sub>context: p90 0.830 · p95 0.877 · p99 1.030 · 11711 op/s</sub> | 0.620<br><sub>context: p90 0.803 · p95 0.865 · p99 0.971 · 12373 op/s</sub> | -6.5% (-0.043) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.566<br><sub>context: p90 0.692 · p95 0.761 · p99 0.788 · 1735 op/s</sub> | 0.521<br><sub>context: p90 0.623 · p95 0.647 · p99 0.735 · 1883 op/s</sub> | -7.9% (-0.045) | 150% AND 2 ms | 🟢 |
| 8 | 0.666<br><sub>context: p90 0.821 · p95 0.883 · p99 1.024 · 11494 op/s</sub> | 0.628<br><sub>context: p90 0.786 · p95 0.839 · p99 0.938 · 12313 op/s</sub> | -5.7% (-0.038) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.487<br><sub>context: p90 0.602 · p95 0.631 · p99 0.716 · 1995 op/s</sub> | 0.362<br><sub>context: p90 0.443 · p95 0.469 · p99 0.504 · 2668 op/s</sub> | -25.7% (-0.125) | 150% AND 2 ms | 🟢 |
| 8 | 0.549<br><sub>context: p90 0.658 · p95 0.693 · p99 0.780 · 14171 op/s</sub> | 0.472<br><sub>context: p90 0.578 · p95 0.612 · p99 0.665 · 16503 op/s</sub> | -14.0% (-0.077) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.407<br><sub>context: p90 0.445 · p95 0.462 · p99 0.490 · 2410 op/s</sub> | 0.288<br><sub>context: p90 0.356 · p95 0.375 · p99 0.405 · 3347 op/s</sub> | -29.1% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.549<br><sub>context: p90 0.647 · p95 0.682 · p99 0.742 · 14299 op/s</sub> | 0.393<br><sub>context: p90 0.493 · p95 0.523 · p99 0.579 · 19494 op/s</sub> | -28.5% (-0.157) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.402<br><sub>context: p90 18.554 · p95 18.644 · p99 18.718 · 54 op/s</sub> | 15.167<br><sub>context: p90 15.329 · p95 15.369 · p99 15.493 · 66 op/s</sub> | -17.6% (-3.236) | 150% AND 2 ms | 🟢 |
| 8 | 23.938<br><sub>context: p90 32.661 · p95 35.696 · p99 40.431 · 309 op/s</sub> | 18.840<br><sub>context: p90 25.917 · p95 29.593 · p99 32.751 · 381 op/s</sub> | -21.3% (-5.098) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.482<br><sub>context: p90 0.544 · p95 0.568 · p99 0.602 · 2031 op/s</sub> | 0.372<br><sub>context: p90 0.414 · p95 0.437 · p99 0.470 · 2653 op/s</sub> | -22.9% (-0.111) | 150% AND 2 ms | 🟢 |
| 8 | 0.646<br><sub>context: p90 0.756 · p95 0.791 · p99 0.867 · 12061 op/s</sub> | 0.458<br><sub>context: p90 0.557 · p95 0.589 · p99 0.655 · 17076 op/s</sub> | -29.1% (-0.188) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.478<br><sub>context: p90 1.982 · p95 2.216 · p99 2.676 · 664 op/s</sub> | 1.292<br><sub>context: p90 1.732 · p95 1.872 · p99 2.101 · 771 op/s</sub> | -12.6% (-0.186) | 150% AND 2 ms | 🟢 |
| 8 | 1.819<br><sub>context: p90 2.565 · p95 2.840 · p99 3.358 · 4253 op/s</sub> | 1.595<br><sub>context: p90 2.293 · p95 2.493 · p99 3.055 · 4817 op/s</sub> | -12.3% (-0.223) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.421<br><sub>context: p90 0.504 · p95 0.534 · p99 0.582 · 2315 op/s</sub> | 0.404<br><sub>context: p90 0.496 · p95 0.529 · p99 0.618 · 2415 op/s</sub> | -4.1% (-0.017) | 150% AND 2 ms | 🟢 |
| 8 | 0.559<br><sub>context: p90 0.673 · p95 0.711 · p99 0.774 · 13970 op/s</sub> | 0.477<br><sub>context: p90 0.597 · p95 0.634 · p99 0.691 · 16057 op/s</sub> | -14.6% (-0.082) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.256<br><sub>context: p90 2.812 · p95 2.871 · p99 2.998 · 455 op/s</sub> | 0.369<br><sub>context: p90 0.460 · p95 0.485 · p99 0.595 · 2625 op/s</sub> | -83.6% (-1.887) | 150% AND 2 ms | 🟢 |
| 8 | 2.559<br><sub>context: p90 3.302 · p95 3.497 · p99 3.710 · 3145 op/s</sub> | 0.440<br><sub>context: p90 0.557 · p95 0.599 · p99 0.683 · 17534 op/s</sub> | -82.8% (-2.119) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.254<br><sub>context: p90 2.865 · p95 2.924 · p99 3.083 · 457 op/s</sub> | 0.369<br><sub>context: p90 0.455 · p95 0.490 · p99 0.558 · 2617 op/s</sub> | -83.6% (-1.885) | 150% AND 2 ms | 🟢 |
| 8 | 2.579<br><sub>context: p90 3.334 · p95 3.541 · p99 3.783 · 3141 op/s</sub> | 0.458<br><sub>context: p90 0.583 · p95 0.618 · p99 0.715 · 16725 op/s</sub> | -82.2% (-2.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.239<br><sub>context: p90 0.289 · p95 0.313 · p99 0.345 · 4090 op/s</sub> | 0.187<br><sub>context: p90 0.236 · p95 0.277 · p99 0.290 · 5022 op/s</sub> | -21.8% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.302<br><sub>context: p90 0.377 · p95 0.405 · p99 0.452 · 25167 op/s</sub> | 0.309<br><sub>context: p90 0.429 · p95 0.467 · p99 0.549 · 24331 op/s</sub> | +2.3% (+0.007) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.320<br><sub>context: p90 0.369 · p95 0.387 · p99 0.409 · 3056 op/s</sub> | 0.151<br><sub>context: p90 0.179 · p95 0.184 · p99 0.212 · 6211 op/s</sub> | -52.9% (-0.169) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.414<br><sub>context: p90 0.490 · p95 0.515 · p99 0.569 · 18744 op/s</sub> | 0.247<br><sub>context: p90 0.317 · p95 0.338 · p99 0.382 · 31285 op/s</sub> | -40.3% (-0.167) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.312<br><sub>context: p90 0.363 · p95 0.379 · p99 0.405 · 3135 op/s</sub> | 0.224<br><sub>context: p90 0.276 · p95 0.302 · p99 0.337 · 4214 op/s</sub> | -28.1% (-0.088) | 150% AND 2 ms | 🟢 |
| 8 | 0.432<br><sub>context: p90 0.518 · p95 0.547 · p99 0.614 · 17999 op/s</sub> | 0.372<br><sub>context: p90 0.485 · p95 0.529 · p99 0.587 · 20519 op/s</sub> | -14.1% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.375 · p95 0.405 · p99 0.460 · 3045 op/s</sub> | 0.265<br><sub>context: p90 0.342 · p95 0.371 · p99 0.405 · 3559 op/s</sub> | -15.9% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.444<br><sub>context: p90 0.536 · p95 0.573 · p99 0.650 · 17394 op/s</sub> | 0.382<br><sub>context: p90 0.498 · p95 0.534 · p99 0.618 · 20099 op/s</sub> | -13.9% (-0.062) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.324 · p95 0.339 · p99 0.394 · 3638 op/s</sub> | 0.207<br><sub>context: p90 0.311 · p95 0.360 · p99 0.396 · 4382 op/s</sub> | -20.6% (-0.054) | 150% AND 2 ms | 🟢 |
| 8 | 0.387<br><sub>context: p90 0.467 · p95 0.490 · p99 0.547 · 20071 op/s</sub> | 0.318<br><sub>context: p90 0.417 · p95 0.458 · p99 0.511 · 23940 op/s</sub> | -17.7% (-0.069) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.374<br><sub>context: p90 1.455 · p95 1.492 · p99 1.566 · 723 op/s</sub> | 0.936<br><sub>context: p90 1.027 · p95 1.048 · p99 1.127 · 1052 op/s</sub> | -31.8% (-0.437) | 150% AND 2 ms | 🟢 |
| 8 | 1.827<br><sub>context: p90 2.192 · p95 2.330 · p99 2.677 · 4256 op/s</sub> | 1.168<br><sub>context: p90 1.326 · p95 1.380 · p99 1.483 · 6738 op/s</sub> | -36.1% (-0.659) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.287<br><sub>context: p90 1.393 · p95 1.416 · p99 1.501 · 772 op/s</sub> | 0.947<br><sub>context: p90 1.004 · p95 1.029 · p99 1.060 · 1049 op/s</sub> | -26.4% (-0.340) | 150% AND 2 ms | 🟢 |
| 8 | 1.622<br><sub>context: p90 1.989 · p95 2.083 · p99 2.415 · 4760 op/s</sub> | 1.100<br><sub>context: p90 1.254 · p95 1.304 · p99 1.410 · 7159 op/s</sub> | -32.2% (-0.522) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.340<br><sub>context: p90 0.389 · p95 0.403 · p99 0.440 · 2878 op/s</sub> | 0.256<br><sub>context: p90 0.290 · p95 0.307 · p99 0.343 · 3895 op/s</sub> | -24.8% (-0.084) | 150% AND 2 ms | 🟢 |
| 8 | 0.468<br><sub>context: p90 0.571 · p95 0.601 · p99 0.664 · 16440 op/s</sub> | 0.392<br><sub>context: p90 0.494 · p95 0.525 · p99 0.598 · 19729 op/s</sub> | -16.3% (-0.076) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.457<br><sub>context: p90 0.530 · p95 0.543 · p99 0.589 · 2163 op/s</sub> | 0.378<br><sub>context: p90 0.460 · p95 0.484 · p99 0.521 · 2580 op/s</sub> | -17.4% (-0.079) | 150% AND 2 ms | 🟢 |
| 8 | 0.575<br><sub>context: p90 0.688 · p95 0.733 · p99 0.812 · 13523 op/s</sub> | 0.475<br><sub>context: p90 0.591 · p95 0.625 · p99 0.705 · 16250 op/s</sub> | -17.5% (-0.101) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.287<br><sub>context: p90 0.343 · p95 0.359 · p99 0.378 · 3389 op/s</sub> | 0.182<br><sub>context: p90 0.223 · p95 0.228 · p99 0.242 · 5237 op/s</sub> | -36.6% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 0.405<br><sub>context: p90 0.487 · p95 0.518 · p99 0.577 · 19182 op/s</sub> | 0.324<br><sub>context: p90 0.427 · p95 0.460 · p99 0.519 · 23394 op/s</sub> | -20.1% (-0.081) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.234<br><sub>context: p90 0.297 · p95 0.311 · p99 0.342 · 4046 op/s</sub> | 0.203<br><sub>context: p90 0.288 · p95 0.319 · p99 0.358 · 4530 op/s</sub> | -13.5% (-0.032) | 150% AND 2 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.366 · p95 0.389 · p99 0.437 · 26034 op/s</sub> | 0.298<br><sub>context: p90 0.388 · p95 0.424 · p99 0.481 · 25258 op/s</sub> | +0.5% (+0.002) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.225<br><sub>context: p90 0.259 · p95 0.265 · p99 0.292 · 4282 op/s</sub> | 0.177<br><sub>context: p90 0.219 · p95 0.241 · p99 0.272 · 5408 op/s</sub> | -21.5% (-0.048) | 150% AND 2 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.366 · p95 0.391 · p99 0.436 · 25556 op/s</sub> | 0.304<br><sub>context: p90 0.402 · p95 0.438 · p99 0.523 · 24896 op/s</sub> | +1.8% (+0.005) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.965<br><sub>context: p90 1.028 · p95 1.053 · p99 1.138 · 1021 op/s</sub> | 0.422<br><sub>context: p90 0.483 · p95 0.503 · p99 0.553 · 2283 op/s</sub> | -56.3% (-0.544) | 150% AND 2 ms | 🟢 |
| 8 | 1.278<br><sub>context: p90 1.627 · p95 1.876 · p99 2.166 · 5930 op/s</sub> | 0.469<br><sub>context: p90 0.544 · p95 0.568 · p99 0.623 · 16726 op/s</sub> | -63.3% (-0.809) | 150% AND 2 ms | 🟢 |

</details>
