### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:215d05fcfb400f14ccd553f34f1b188d1ffcd9850421cc5c5baab38c49e0c0c5 |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:215d05fcfb400f14ccd553f34f1b188d1ffcd9850421cc5c5baab38c49e0c0c5

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.434<br><sub>context: p90 1.560 · p95 1.630 · p99 1.731 · 692 op/s</sub> | 0.717<br><sub>context: p90 0.772 · p95 0.783 · p99 0.827 · 1367 op/s</sub> | -50.0% (-0.717) | 150% AND 2 ms | 🟢 |
| 8 | 1.883<br><sub>context: p90 2.443 · p95 2.684 · p99 3.129 · 4052 op/s</sub> | 0.879<br><sub>context: p90 0.995 · p95 1.025 · p99 1.089 · 9055 op/s</sub> | -53.3% (-1.005) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.076<br><sub>context: p90 2.235 · p95 2.305 · p99 2.417 · 475 op/s</sub> | 0.766<br><sub>context: p90 0.807 · p95 0.821 · p99 0.852 · 1288 op/s</sub> | -63.1% (-1.310) | 150% AND 2 ms | 🟢 |
| 8 | 2.644<br><sub>context: p90 3.416 · p95 3.502 · p99 3.871 · 2888 op/s</sub> | 0.986<br><sub>context: p90 1.149 · p95 1.183 · p99 1.239 · 8060 op/s</sub> | -62.7% (-1.658) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.027<br><sub>context: p90 2.150 · p95 2.231 · p99 2.304 · 487 op/s</sub> | 1.306<br><sub>context: p90 1.364 · p95 1.384 · p99 1.407 · 761 op/s</sub> | -35.6% (-0.721) | 150% AND 2 ms | 🟢 |
| 8 | 2.815<br><sub>context: p90 3.508 · p95 3.679 · p99 4.063 · 2791 op/s</sub> | 1.794<br><sub>context: p90 2.347 · p95 2.549 · p99 2.923 · 4267 op/s</sub> | -36.3% (-1.021) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.651<br><sub>context: p90 2.811 · p95 2.882 · p99 2.965 · 374 op/s</sub> | 1.354<br><sub>context: p90 1.442 · p95 1.464 · p99 1.516 · 730 op/s</sub> | -48.9% (-1.297) | 150% AND 2 ms | 🟢 |
| 8 | 3.589<br><sub>context: p90 4.598 · p95 4.833 · p99 5.339 · 2151 op/s</sub> | 1.585<br><sub>context: p90 1.931 · p95 1.979 · p99 2.050 · 4840 op/s</sub> | -55.8% (-2.004) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.148<br><sub>context: p90 0.186 · p95 0.199 · p99 0.223 · 6131 op/s</sub> | 0.156<br><sub>context: p90 0.193 · p95 0.223 · p99 0.247 · 6038 op/s</sub> | +4.8% (+0.007) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.297 · p95 0.315 · p99 0.347 · 32422 op/s</sub> | 0.217<br><sub>context: p90 0.286 · p95 0.305 · p99 0.345 · 35343 op/s</sub> | -7.5% (-0.018) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.322<br><sub>context: p90 0.371 · p95 0.387 · p99 0.413 · 3016 op/s</sub> | 0.215<br><sub>context: p90 0.254 · p95 0.276 · p99 0.326 · 4513 op/s</sub> | -33.3% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.520 · p95 0.555 · p99 0.625 · 17977 op/s</sub> | 0.324<br><sub>context: p90 0.412 · p95 0.442 · p99 0.507 · 23576 op/s</sub> | -25.2% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.352<br><sub>context: p90 0.404 · p95 0.421 · p99 0.443 · 2831 op/s</sub> | 0.234<br><sub>context: p90 0.279 · p95 0.313 · p99 0.348 · 4085 op/s</sub> | -33.5% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.461<br><sub>context: p90 0.541 · p95 0.570 · p99 0.635 · 16903 op/s</sub> | 0.342<br><sub>context: p90 0.433 · p95 0.464 · p99 0.518 · 22305 op/s</sub> | -25.8% (-0.119) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.369<br><sub>context: p90 0.425 · p95 0.448 · p99 0.501 · 2655 op/s</sub> | 0.337<br><sub>context: p90 0.414 · p95 0.439 · p99 0.488 · 2855 op/s</sub> | -8.5% (-0.031) | 150% AND 2 ms | 🟢 |
| 8 | 0.514<br><sub>context: p90 0.611 · p95 0.640 · p99 0.696 · 15188 op/s</sub> | 0.439<br><sub>context: p90 0.540 · p95 0.567 · p99 0.629 · 17610 op/s</sub> | -14.5% (-0.074) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.402<br><sub>context: p90 0.477 · p95 0.508 · p99 0.534 · 2382 op/s</sub> | 0.353<br><sub>context: p90 0.411 · p95 0.424 · p99 0.452 · 2786 op/s</sub> | -12.1% (-0.049) | 150% AND 2 ms | 🟢 |
| 8 | 0.545<br><sub>context: p90 0.658 · p95 0.693 · p99 0.778 · 14148 op/s</sub> | 0.466<br><sub>context: p90 0.568 · p95 0.598 · p99 0.659 · 16740 op/s</sub> | -14.5% (-0.079) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.532<br><sub>context: p90 0.659 · p95 0.697 · p99 0.755 · 1828 op/s</sub> | 0.465<br><sub>context: p90 0.554 · p95 0.580 · p99 0.646 · 2115 op/s</sub> | -12.7% (-0.068) | 150% AND 2 ms | 🟢 |
| 8 | 0.728<br><sub>context: p90 0.923 · p95 0.987 · p99 1.091 · 10630 op/s</sub> | 0.618<br><sub>context: p90 0.793 · p95 0.844 · p99 0.924 · 12492 op/s</sub> | -15.0% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.563<br><sub>context: p90 0.722 · p95 0.767 · p99 0.839 · 1715 op/s</sub> | 0.473<br><sub>context: p90 0.579 · p95 0.617 · p99 0.657 · 2057 op/s</sub> | -16.0% (-0.090) | 150% AND 2 ms | 🟢 |
| 8 | 0.772<br><sub>context: p90 0.968 · p95 1.035 · p99 1.148 · 10079 op/s</sub> | 0.652<br><sub>context: p90 0.822 · p95 0.869 · p99 0.964 · 11852 op/s</sub> | -15.6% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.149<br><sub>context: p90 1.549 · p95 1.711 · p99 1.943 · 855 op/s</sub> | 1.019<br><sub>context: p90 1.335 · p95 1.515 · p99 1.694 · 969 op/s</sub> | -11.3% (-0.130) | 150% AND 2 ms | 🟢 |
| 8 | 1.517<br><sub>context: p90 2.142 · p95 2.338 · p99 2.737 · 5033 op/s</sub> | 1.272<br><sub>context: p90 1.771 · p95 1.944 · p99 2.243 · 6056 op/s</sub> | -16.2% (-0.245) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.192<br><sub>context: p90 1.636 · p95 1.810 · p99 2.038 · 804 op/s</sub> | 0.981<br><sub>context: p90 1.352 · p95 1.443 · p99 1.650 · 986 op/s</sub> | -17.6% (-0.210) | 150% AND 2 ms | 🟢 |
| 8 | 1.584<br><sub>context: p90 2.239 · p95 2.447 · p99 2.804 · 4825 op/s</sub> | 1.307<br><sub>context: p90 1.802 · p95 1.982 · p99 2.302 · 5844 op/s</sub> | -17.5% (-0.277) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.614<br><sub>context: p90 0.731 · p95 0.746 · p99 0.804 · 1596 op/s</sub> | 0.573<br><sub>context: p90 0.727 · p95 0.769 · p99 0.836 · 1708 op/s</sub> | -6.7% (-0.041) | 150% AND 2 ms | 🟢 |
| 8 | 0.777<br><sub>context: p90 0.909 · p95 0.958 · p99 1.103 · 10140 op/s</sub> | 0.771<br><sub>context: p90 1.040 · p95 1.103 · p99 1.222 · 10106 op/s</sub> | -0.8% (-0.006) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.404<br><sub>context: p90 0.508 · p95 0.529 · p99 0.578 · 2350 op/s</sub> | 0.289<br><sub>context: p90 0.337 · p95 0.348 · p99 0.374 · 3406 op/s</sub> | -28.3% (-0.114) | 150% AND 2 ms | 🟢 |
| 8 | 0.543<br><sub>context: p90 0.640 · p95 0.671 · p99 0.752 · 14276 op/s</sub> | 0.396<br><sub>context: p90 0.492 · p95 0.521 · p99 0.571 · 19514 op/s</sub> | -27.0% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.216<br><sub>context: p90 0.277 · p95 0.290 · p99 0.335 · 4303 op/s</sub> | 0.153<br><sub>context: p90 0.172 · p95 0.175 · p99 0.185 · 6337 op/s</sub> | -29.2% (-0.063) | 150% AND 2 ms | 🟢 |
| 8 | 0.323<br><sub>context: p90 0.399 · p95 0.427 · p99 0.484 · 23649 op/s</sub> | 0.226<br><sub>context: p90 0.289 · p95 0.312 · p99 0.349 · 33881 op/s</sub> | -29.8% (-0.096) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.168<br><sub>context: p90 0.191 · p95 0.195 · p99 0.206 · 5819 op/s</sub> | 0.123<br><sub>context: p90 0.149 · p95 0.157 · p99 0.174 · 7577 op/s</sub> | -26.5% (-0.044) | 150% AND 2 ms | 🟢 |
| 8 | 0.262<br><sub>context: p90 0.328 · p95 0.352 · p99 0.391 · 28939 op/s</sub> | 0.203<br><sub>context: p90 0.272 · p95 0.300 · p99 0.353 · 37791 op/s</sub> | -22.5% (-0.059) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.490<br><sub>context: p90 0.558 · p95 0.584 · p99 0.616 · 1989 op/s</sub> | 0.354<br><sub>context: p90 0.467 · p95 0.495 · p99 0.539 · 2669 op/s</sub> | -27.9% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.650<br><sub>context: p90 0.766 · p95 0.804 · p99 0.899 · 12005 op/s</sub> | 0.439<br><sub>context: p90 0.531 · p95 0.560 · p99 0.618 · 17681 op/s</sub> | -32.4% (-0.210) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.027<br><sub>context: p90 1.463 · p95 1.588 · p99 1.775 · 947 op/s</sub> | 1.313<br><sub>context: p90 2.008 · p95 2.211 · p99 2.498 · 733 op/s</sub> | +27.8% (+0.285) | 150% AND 2 ms | 🟢 |
| 8 | 1.345<br><sub>context: p90 2.093 · p95 2.335 · p99 2.795 · 5573 op/s</sub> | 2.257<br><sub>context: p90 3.573 · p95 4.029 · p99 4.668 · 3347 op/s</sub> | +67.8% (+0.912) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.429<br><sub>context: p90 5.486 · p95 6.200 · p99 6.612 · 278 op/s</sub> | 4.796<br><sub>context: p90 7.289 · p95 8.106 · p99 8.687 · 201 op/s</sub> | +39.9% (+1.367) | 150% AND 2 ms | 🟢 |
| 8 | 4.492<br><sub>context: p90 7.981 · p95 8.998 · p99 11.089 · 1613 op/s</sub> | 8.661<br><sub>context: p90 13.376 · p95 14.699 · p99 16.870 · 890 op/s</sub> | +92.8% (+4.168) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.213<br><sub>context: p90 0.266 · p95 0.282 · p99 0.295 · 4455 op/s</sub> | 0.176<br><sub>context: p90 0.203 · p95 0.218 · p99 0.238 · 5524 op/s</sub> | -17.3% (-0.037) | 150% AND 2 ms | 🟢 |
| 8 | 0.358<br><sub>context: p90 0.437 · p95 0.463 · p99 0.514 · 20947 op/s</sub> | 0.300<br><sub>context: p90 0.398 · p95 0.424 · p99 0.508 · 25101 op/s</sub> | -16.2% (-0.058) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.229<br><sub>context: p90 0.273 · p95 0.282 · p99 0.288 · 4203 op/s</sub> | 0.153<br><sub>context: p90 0.193 · p95 0.202 · p99 0.213 · 6034 op/s</sub> | -33.3% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.435 · p95 0.463 · p99 0.519 · 21561 op/s</sub> | 0.299<br><sub>context: p90 0.405 · p95 0.434 · p99 0.507 · 25224 op/s</sub> | -16.7% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.358<br><sub>context: p90 0.430 · p95 0.451 · p99 0.510 · 2715 op/s</sub> | 0.222<br><sub>context: p90 0.261 · p95 0.267 · p99 0.279 · 4289 op/s</sub> | -38.0% (-0.136) | 150% AND 2 ms | 🟢 |
| 8 | 0.476<br><sub>context: p90 0.583 · p95 0.613 · p99 0.680 · 16285 op/s</sub> | 0.356<br><sub>context: p90 0.458 · p95 0.497 · p99 0.557 · 21179 op/s</sub> | -25.1% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.262 · p95 0.297 · p99 0.312 · 4882 op/s</sub> | 0.128<br><sub>context: p90 0.154 · p95 0.158 · p99 0.163 · 7294 op/s</sub> | -33.9% (-0.066) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.352 · p95 0.381 · p99 0.430 · 26698 op/s</sub> | 0.221<br><sub>context: p90 0.285 · p95 0.307 · p99 0.338 · 34789 op/s</sub> | -23.8% (-0.069) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.359<br><sub>context: p90 0.439 · p95 0.456 · p99 0.501 · 2659 op/s</sub> | 0.340<br><sub>context: p90 0.426 · p95 0.458 · p99 0.477 · 2834 op/s</sub> | -5.3% (-0.019) | 150% AND 2 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.521 · p95 0.549 · p99 0.608 · 17323 op/s</sub> | 0.395<br><sub>context: p90 0.483 · p95 0.512 · p99 0.561 · 19642 op/s</sub> | -11.4% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.345 · p95 0.392 · p99 0.417 · 3368 op/s</sub> | 0.230<br><sub>context: p90 0.319 · p95 0.349 · p99 0.371 · 4096 op/s</sub> | -18.6% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.372<br><sub>context: p90 0.440 · p95 0.467 · p99 0.519 · 21070 op/s</sub> | 0.329<br><sub>context: p90 0.420 · p95 0.459 · p99 0.515 · 23405 op/s</sub> | -11.4% (-0.043) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.420 · p95 0.440 · p99 0.483 · 2632 op/s</sub> | 0.353<br><sub>context: p90 0.455 · p95 0.475 · p99 0.546 · 2739 op/s</sub> | -4.8% (-0.018) | 150% AND 2 ms | 🟢 |
| 8 | 0.497<br><sub>context: p90 0.592 · p95 0.621 · p99 0.684 · 15646 op/s</sub> | 0.435<br><sub>context: p90 0.538 · p95 0.565 · p99 0.642 · 17532 op/s</sub> | -12.4% (-0.062) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.437<br><sub>context: p90 0.549 · p95 0.573 · p99 0.640 · 2213 op/s</sub> | 0.474<br><sub>context: p90 0.593 · p95 0.615 · p99 0.709 · 2085 op/s</sub> | +8.4% (+0.037) | 150% AND 2 ms | 🟢 |
| 8 | 0.642<br><sub>context: p90 0.821 · p95 0.878 · p99 1.032 · 12071 op/s</sub> | 0.614<br><sub>context: p90 0.776 · p95 0.828 · p99 0.932 · 12569 op/s</sub> | -4.3% (-0.028) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.477<br><sub>context: p90 0.581 · p95 0.632 · p99 0.675 · 2023 op/s</sub> | 0.481<br><sub>context: p90 0.585 · p95 0.628 · p99 0.676 · 2044 op/s</sub> | +0.9% (+0.004) | 150% AND 2 ms | 🟢 |
| 8 | 0.664<br><sub>context: p90 0.819 · p95 0.868 · p99 0.941 · 11717 op/s</sub> | 0.623<br><sub>context: p90 0.795 · p95 0.855 · p99 0.965 · 12222 op/s</sub> | -6.1% (-0.040) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.412<br><sub>context: p90 0.492 · p95 0.517 · p99 0.574 · 2350 op/s</sub> | 0.379<br><sub>context: p90 0.452 · p95 0.486 · p99 0.602 · 2575 op/s</sub> | -8.2% (-0.034) | 150% AND 2 ms | 🟢 |
| 8 | 0.535<br><sub>context: p90 0.634 · p95 0.666 · p99 0.727 · 14539 op/s</sub> | 0.455<br><sub>context: p90 0.552 · p95 0.587 · p99 0.651 · 16911 op/s</sub> | -14.9% (-0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.410<br><sub>context: p90 0.497 · p95 0.520 · p99 0.558 · 2346 op/s</sub> | 0.275<br><sub>context: p90 0.327 · p95 0.337 · p99 0.379 · 3566 op/s</sub> | -33.0% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.539<br><sub>context: p90 0.635 · p95 0.662 · p99 0.755 · 14245 op/s</sub> | 0.377<br><sub>context: p90 0.475 · p95 0.506 · p99 0.574 · 20434 op/s</sub> | -30.1% (-0.162) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.168<br><sub>context: p90 18.411 · p95 18.486 · p99 18.626 · 55 op/s</sub> | 15.059<br><sub>context: p90 15.472 · p95 15.509 · p99 15.721 · 66 op/s</sub> | -17.1% (-3.109) | 150% AND 2 ms | 🟢 |
| 8 | 23.186<br><sub>context: p90 31.414 · p95 34.808 · p99 39.168 · 314 op/s</sub> | 18.044<br><sub>context: p90 23.723 · p95 26.222 · p99 29.648 · 394 op/s</sub> | -22.2% (-5.141) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.475<br><sub>context: p90 0.557 · p95 0.582 · p99 0.637 · 2031 op/s</sub> | 0.366<br><sub>context: p90 0.462 · p95 0.488 · p99 0.505 · 2603 op/s</sub> | -23.1% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.645<br><sub>context: p90 0.757 · p95 0.789 · p99 0.890 · 11954 op/s</sub> | 0.446<br><sub>context: p90 0.542 · p95 0.570 · p99 0.633 · 17443 op/s</sub> | -30.8% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.472<br><sub>context: p90 2.006 · p95 2.196 · p99 2.608 · 672 op/s</sub> | 1.252<br><sub>context: p90 1.762 · p95 1.882 · p99 2.042 · 781 op/s</sub> | -14.9% (-0.220) | 150% AND 2 ms | 🟢 |
| 8 | 1.804<br><sub>context: p90 2.526 · p95 2.768 · p99 3.292 · 4319 op/s</sub> | 1.568<br><sub>context: p90 2.214 · p95 2.397 · p99 2.913 · 4914 op/s</sub> | -13.1% (-0.236) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.439<br><sub>context: p90 0.530 · p95 0.552 · p99 0.595 · 2221 op/s</sub> | 0.375<br><sub>context: p90 0.455 · p95 0.479 · p99 0.527 · 2612 op/s</sub> | -14.7% (-0.065) | 150% AND 2 ms | 🟢 |
| 8 | 0.555<br><sub>context: p90 0.666 · p95 0.700 · p99 0.775 · 13888 op/s</sub> | 0.464<br><sub>context: p90 0.570 · p95 0.605 · p99 0.661 · 16722 op/s</sub> | -16.5% (-0.092) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.199<br><sub>context: p90 2.775 · p95 2.870 · p99 3.000 · 464 op/s</sub> | 0.342<br><sub>context: p90 0.412 · p95 0.430 · p99 0.452 · 2849 op/s</sub> | -84.5% (-1.857) | 150% AND 2 ms | 🟢 |
| 8 | 2.531<br><sub>context: p90 3.244 · p95 3.414 · p99 3.611 · 3192 op/s</sub> | 0.423<br><sub>context: p90 0.536 · p95 0.561 · p99 0.652 · 18190 op/s</sub> | -83.3% (-2.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.197<br><sub>context: p90 2.787 · p95 2.884 · p99 2.993 · 467 op/s</sub> | 0.345<br><sub>context: p90 0.438 · p95 0.466 · p99 0.515 · 2771 op/s</sub> | -84.3% (-1.852) | 150% AND 2 ms | 🟢 |
| 8 | 2.539<br><sub>context: p90 3.349 · p95 3.516 · p99 3.757 · 3183 op/s</sub> | 0.444<br><sub>context: p90 0.560 · p95 0.596 · p99 0.669 · 17363 op/s</sub> | -82.5% (-2.095) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.197<br><sub>context: p90 0.242 · p95 0.253 · p99 0.284 · 4891 op/s</sub> | 0.176<br><sub>context: p90 0.218 · p95 0.242 · p99 0.318 · 5442 op/s</sub> | -10.9% (-0.022) | 150% AND 2 ms | 🟢 |
| 8 | 0.288<br><sub>context: p90 0.354 · p95 0.371 · p99 0.419 · 26873 op/s</sub> | 0.297<br><sub>context: p90 0.391 · p95 0.428 · p99 0.495 · 25352 op/s</sub> | +3.3% (+0.010) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.321<br><sub>context: p90 0.388 · p95 0.415 · p99 0.450 · 2979 op/s</sub> | 0.148<br><sub>context: p90 0.183 · p95 0.188 · p99 0.194 · 6385 op/s</sub> | -54.0% (-0.173) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.409<br><sub>context: p90 0.484 · p95 0.511 · p99 0.555 · 19181 op/s</sub> | 0.242<br><sub>context: p90 0.307 · p95 0.329 · p99 0.375 · 31518 op/s</sub> | -40.9% (-0.167) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.288<br><sub>context: p90 0.328 · p95 0.334 · p99 0.360 · 3349 op/s</sub> | 0.207<br><sub>context: p90 0.249 · p95 0.261 · p99 0.295 · 4564 op/s</sub> | -28.1% (-0.081) | 150% AND 2 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.531 · p95 0.569 · p99 0.628 · 17708 op/s</sub> | 0.356<br><sub>context: p90 0.460 · p95 0.492 · p99 0.569 · 21361 op/s</sub> | -18.2% (-0.079) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.315<br><sub>context: p90 0.365 · p95 0.384 · p99 0.422 · 3087 op/s</sub> | 0.252<br><sub>context: p90 0.319 · p95 0.332 · p99 0.369 · 3777 op/s</sub> | -19.9% (-0.063) | 150% AND 2 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.523 · p95 0.558 · p99 0.635 · 17769 op/s</sub> | 0.369<br><sub>context: p90 0.480 · p95 0.512 · p99 0.597 · 20876 op/s</sub> | -15.6% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.318 · p95 0.339 · p99 0.403 · 3659 op/s</sub> | 0.187<br><sub>context: p90 0.215 · p95 0.231 · p99 0.249 · 5288 op/s</sub> | -28.6% (-0.075) | 150% AND 2 ms | 🟢 |
| 8 | 0.388<br><sub>context: p90 0.464 · p95 0.491 · p99 0.545 · 19822 op/s</sub> | 0.305<br><sub>context: p90 0.395 · p95 0.425 · p99 0.498 · 24761 op/s</sub> | -21.5% (-0.084) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.389<br><sub>context: p90 1.495 · p95 1.554 · p99 1.685 · 717 op/s</sub> | 0.941<br><sub>context: p90 1.018 · p95 1.047 · p99 1.109 · 1053 op/s</sub> | -32.3% (-0.448) | 150% AND 2 ms | 🟢 |
| 8 | 1.807<br><sub>context: p90 2.155 · p95 2.293 · p99 2.560 · 4289 op/s</sub> | 1.163<br><sub>context: p90 1.326 · p95 1.374 · p99 1.469 · 6752 op/s</sub> | -35.6% (-0.644) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.271<br><sub>context: p90 1.364 · p95 1.420 · p99 1.514 · 783 op/s</sub> | 0.970<br><sub>context: p90 1.079 · p95 1.100 · p99 1.143 · 1015 op/s</sub> | -23.6% (-0.300) | 150% AND 2 ms | 🟢 |
| 8 | 1.599<br><sub>context: p90 1.939 · p95 2.078 · p99 2.478 · 4863 op/s</sub> | 1.088<br><sub>context: p90 1.213 · p95 1.250 · p99 1.329 · 7307 op/s</sub> | -32.0% (-0.512) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.360<br><sub>context: p90 0.460 · p95 0.480 · p99 0.560 · 2692 op/s</sub> | 0.235<br><sub>context: p90 0.275 · p95 0.295 · p99 0.347 · 4128 op/s</sub> | -34.7% (-0.125) | 150% AND 2 ms | 🟢 |
| 8 | 0.463<br><sub>context: p90 0.558 · p95 0.589 · p99 0.664 · 16636 op/s</sub> | 0.373<br><sub>context: p90 0.465 · p95 0.496 · p99 0.560 · 20561 op/s</sub> | -19.5% (-0.090) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.454<br><sub>context: p90 0.546 · p95 0.570 · p99 0.610 · 2156 op/s</sub> | 0.378<br><sub>context: p90 0.451 · p95 0.472 · p99 0.542 · 2578 op/s</sub> | -16.7% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.572<br><sub>context: p90 0.687 · p95 0.721 · p99 0.802 · 13624 op/s</sub> | 0.478<br><sub>context: p90 0.598 · p95 0.630 · p99 0.691 · 16377 op/s</sub> | -16.4% (-0.094) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.278<br><sub>context: p90 0.341 · p95 0.357 · p99 0.399 · 3477 op/s</sub> | 0.180<br><sub>context: p90 0.206 · p95 0.216 · p99 0.227 · 5384 op/s</sub> | -35.3% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.402<br><sub>context: p90 0.481 · p95 0.510 · p99 0.575 · 19337 op/s</sub> | 0.315<br><sub>context: p90 0.415 · p95 0.448 · p99 0.521 · 24029 op/s</sub> | -21.5% (-0.086) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.213<br><sub>context: p90 0.280 · p95 0.313 · p99 0.328 · 4427 op/s</sub> | 0.191<br><sub>context: p90 0.224 · p95 0.252 · p99 0.282 · 5029 op/s</sub> | -10.3% (-0.022) | 150% AND 2 ms | 🟢 |
| 8 | 0.295<br><sub>context: p90 0.363 · p95 0.385 · p99 0.445 · 25985 op/s</sub> | 0.297<br><sub>context: p90 0.404 · p95 0.445 · p99 0.522 · 25406 op/s</sub> | +0.7% (+0.002) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.207<br><sub>context: p90 0.238 · p95 0.247 · p99 0.278 · 4587 op/s</sub> | 0.174<br><sub>context: p90 0.221 · p95 0.243 · p99 0.276 · 5475 op/s</sub> | -16.2% (-0.034) | 150% AND 2 ms | 🟢 |
| 8 | 0.289<br><sub>context: p90 0.360 · p95 0.382 · p99 0.432 · 26578 op/s</sub> | 0.305<br><sub>context: p90 0.408 · p95 0.440 · p99 0.515 · 24772 op/s</sub> | +5.4% (+0.016) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.945<br><sub>context: p90 1.019 · p95 1.046 · p99 1.076 · 1039 op/s</sub> | 0.382<br><sub>context: p90 0.456 · p95 0.479 · p99 0.538 · 2523 op/s</sub> | -59.6% (-0.564) | 150% AND 2 ms | 🟢 |
| 8 | 1.273<br><sub>context: p90 1.787 · p95 1.962 · p99 2.259 · 5896 op/s</sub> | 0.460<br><sub>context: p90 0.538 · p95 0.562 · p99 0.605 · 16905 op/s</sub> | -63.8% (-0.812) | 150% AND 2 ms | 🟢 |

</details>
