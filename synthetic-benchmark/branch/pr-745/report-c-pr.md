### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.434<br><sub>context: p90 1.560 · p95 1.630 · p99 1.731 · 692 op/s</sub> | 0.775<br><sub>context: p90 0.872 · p95 0.916 · p99 0.957 · 1272 op/s</sub> | -46.0% (-0.659) | 150% AND 2 ms | 🟢 |
| 8 | 1.883<br><sub>context: p90 2.443 · p95 2.684 · p99 3.129 · 4052 op/s</sub> | 0.875<br><sub>context: p90 1.008 · p95 1.042 · p99 1.135 · 9024 op/s</sub> | -53.5% (-1.008) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.076<br><sub>context: p90 2.235 · p95 2.305 · p99 2.417 · 475 op/s</sub> | 0.834<br><sub>context: p90 0.927 · p95 0.968 · p99 1.009 · 1177 op/s</sub> | -59.8% (-1.242) | 150% AND 2 ms | 🟢 |
| 8 | 2.644<br><sub>context: p90 3.416 · p95 3.502 · p99 3.871 · 2888 op/s</sub> | 0.977<br><sub>context: p90 1.161 · p95 1.199 · p99 1.260 · 8065 op/s</sub> | -63.1% (-1.667) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.027<br><sub>context: p90 2.150 · p95 2.231 · p99 2.304 · 487 op/s</sub> | 1.329<br><sub>context: p90 1.406 · p95 1.442 · p99 1.505 · 743 op/s</sub> | -34.5% (-0.698) | 150% AND 2 ms | 🟢 |
| 8 | 2.815<br><sub>context: p90 3.508 · p95 3.679 · p99 4.063 · 2791 op/s</sub> | 1.794<br><sub>context: p90 2.326 · p95 2.537 · p99 2.815 · 4305 op/s</sub> | -36.3% (-1.022) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.651<br><sub>context: p90 2.811 · p95 2.882 · p99 2.965 · 374 op/s</sub> | 1.366<br><sub>context: p90 1.476 · p95 1.510 · p99 1.595 · 719 op/s</sub> | -48.5% (-1.285) | 150% AND 2 ms | 🟢 |
| 8 | 3.589<br><sub>context: p90 4.598 · p95 4.833 · p99 5.339 · 2151 op/s</sub> | 1.609<br><sub>context: p90 1.927 · p95 1.973 · p99 2.048 · 4803 op/s</sub> | -55.2% (-1.980) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.148<br><sub>context: p90 0.186 · p95 0.199 · p99 0.223 · 6131 op/s</sub> | 0.146<br><sub>context: p90 0.183 · p95 0.214 · p99 0.244 · 6358 op/s</sub> | -1.3% (-0.002) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.297 · p95 0.315 · p99 0.347 · 32422 op/s</sub> | 0.215<br><sub>context: p90 0.280 · p95 0.301 · p99 0.343 · 35816 op/s</sub> | -8.1% (-0.019) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.322<br><sub>context: p90 0.371 · p95 0.387 · p99 0.413 · 3016 op/s</sub> | 0.244<br><sub>context: p90 0.319 · p95 0.349 · p99 0.410 · 3853 op/s</sub> | -24.3% (-0.078) | 150% AND 2 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.520 · p95 0.555 · p99 0.625 · 17977 op/s</sub> | 0.328<br><sub>context: p90 0.425 · p95 0.459 · p99 0.519 · 23293 op/s</sub> | -24.3% (-0.105) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.352<br><sub>context: p90 0.404 · p95 0.421 · p99 0.443 · 2831 op/s</sub> | 0.267<br><sub>context: p90 0.364 · p95 0.393 · p99 0.449 · 3572 op/s</sub> | -24.1% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.461<br><sub>context: p90 0.541 · p95 0.570 · p99 0.635 · 16903 op/s</sub> | 0.347<br><sub>context: p90 0.435 · p95 0.466 · p99 0.521 · 21962 op/s</sub> | -24.7% (-0.114) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.369<br><sub>context: p90 0.425 · p95 0.448 · p99 0.501 · 2655 op/s</sub> | 0.353<br><sub>context: p90 0.443 · p95 0.494 · p99 0.555 · 2705 op/s</sub> | -4.1% (-0.015) | 150% AND 2 ms | 🟢 |
| 8 | 0.514<br><sub>context: p90 0.611 · p95 0.640 · p99 0.696 · 15188 op/s</sub> | 0.441<br><sub>context: p90 0.540 · p95 0.578 · p99 0.650 · 17588 op/s</sub> | -14.2% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.402<br><sub>context: p90 0.477 · p95 0.508 · p99 0.534 · 2382 op/s</sub> | 0.378<br><sub>context: p90 0.463 · p95 0.504 · p99 0.550 · 2534 op/s</sub> | -5.8% (-0.023) | 150% AND 2 ms | 🟢 |
| 8 | 0.545<br><sub>context: p90 0.658 · p95 0.693 · p99 0.778 · 14148 op/s</sub> | 0.475<br><sub>context: p90 0.577 · p95 0.604 · p99 0.670 · 16442 op/s</sub> | -12.7% (-0.069) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.532<br><sub>context: p90 0.659 · p95 0.697 · p99 0.755 · 1828 op/s</sub> | 0.494<br><sub>context: p90 0.609 · p95 0.648 · p99 0.687 · 1959 op/s</sub> | -7.1% (-0.038) | 150% AND 2 ms | 🟢 |
| 8 | 0.728<br><sub>context: p90 0.923 · p95 0.987 · p99 1.091 · 10630 op/s</sub> | 0.628<br><sub>context: p90 0.801 · p95 0.854 · p99 0.971 · 12222 op/s</sub> | -13.6% (-0.099) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.563<br><sub>context: p90 0.722 · p95 0.767 · p99 0.839 · 1715 op/s</sub> | 0.566<br><sub>context: p90 0.729 · p95 0.756 · p99 0.849 · 1686 op/s</sub> | +0.5% (+0.003) | 150% AND 2 ms | 🟢 |
| 8 | 0.772<br><sub>context: p90 0.968 · p95 1.035 · p99 1.148 · 10079 op/s</sub> | 0.655<br><sub>context: p90 0.832 · p95 0.893 · p99 0.993 · 11733 op/s</sub> | -15.1% (-0.117) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.149<br><sub>context: p90 1.549 · p95 1.711 · p99 1.943 · 855 op/s</sub> | 1.032<br><sub>context: p90 1.391 · p95 1.534 · p99 1.736 · 945 op/s</sub> | -10.2% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 1.517<br><sub>context: p90 2.142 · p95 2.338 · p99 2.737 · 5033 op/s</sub> | 1.279<br><sub>context: p90 1.805 · p95 1.958 · p99 2.286 · 6032 op/s</sub> | -15.7% (-0.238) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.192<br><sub>context: p90 1.636 · p95 1.810 · p99 2.038 · 804 op/s</sub> | 1.057<br><sub>context: p90 1.400 · p95 1.563 · p99 1.690 · 919 op/s</sub> | -11.3% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 1.584<br><sub>context: p90 2.239 · p95 2.447 · p99 2.804 · 4825 op/s</sub> | 1.300<br><sub>context: p90 1.784 · p95 1.950 · p99 2.225 · 5870 op/s</sub> | -17.9% (-0.284) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.614<br><sub>context: p90 0.731 · p95 0.746 · p99 0.804 · 1596 op/s</sub> | 0.576<br><sub>context: p90 0.741 · p95 0.795 · p99 0.871 · 1696 op/s</sub> | -6.2% (-0.038) | 150% AND 2 ms | 🟢 |
| 8 | 0.777<br><sub>context: p90 0.909 · p95 0.958 · p99 1.103 · 10140 op/s</sub> | 0.776<br><sub>context: p90 1.031 · p95 1.098 · p99 1.235 · 10073 op/s</sub> | -0.2% (-0.001) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.404<br><sub>context: p90 0.508 · p95 0.529 · p99 0.578 · 2350 op/s</sub> | 0.324<br><sub>context: p90 0.408 · p95 0.429 · p99 0.517 · 2978 op/s</sub> | -19.6% (-0.079) | 150% AND 2 ms | 🟢 |
| 8 | 0.543<br><sub>context: p90 0.640 · p95 0.671 · p99 0.752 · 14276 op/s</sub> | 0.397<br><sub>context: p90 0.493 · p95 0.525 · p99 0.586 · 19665 op/s</sub> | -26.8% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.216<br><sub>context: p90 0.277 · p95 0.290 · p99 0.335 · 4303 op/s</sub> | 0.148<br><sub>context: p90 0.187 · p95 0.196 · p99 0.213 · 6032 op/s</sub> | -31.3% (-0.068) | 150% AND 2 ms | 🟢 |
| 8 | 0.323<br><sub>context: p90 0.399 · p95 0.427 · p99 0.484 · 23649 op/s</sub> | 0.223<br><sub>context: p90 0.292 · p95 0.316 · p99 0.367 · 34494 op/s</sub> | -30.9% (-0.100) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.168<br><sub>context: p90 0.191 · p95 0.195 · p99 0.206 · 5819 op/s</sub> | 0.133<br><sub>context: p90 0.174 · p95 0.190 · p99 0.208 · 7052 op/s</sub> | -20.7% (-0.035) | 150% AND 2 ms | 🟢 |
| 8 | 0.262<br><sub>context: p90 0.328 · p95 0.352 · p99 0.391 · 28939 op/s</sub> | 0.201<br><sub>context: p90 0.271 · p95 0.294 · p99 0.343 · 38221 op/s</sub> | -23.3% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.490<br><sub>context: p90 0.558 · p95 0.584 · p99 0.616 · 1989 op/s</sub> | 0.328<br><sub>context: p90 0.368 · p95 0.390 · p99 0.442 · 3026 op/s</sub> | -33.0% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.650<br><sub>context: p90 0.766 · p95 0.804 · p99 0.899 · 12005 op/s</sub> | 0.438<br><sub>context: p90 0.537 · p95 0.569 · p99 0.634 · 17664 op/s</sub> | -32.6% (-0.212) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.027<br><sub>context: p90 1.463 · p95 1.588 · p99 1.775 · 947 op/s</sub> | 1.385<br><sub>context: p90 2.091 · p95 2.304 · p99 2.542 · 693 op/s</sub> | +34.8% (+0.358) | 150% AND 2 ms | 🟢 |
| 8 | 1.345<br><sub>context: p90 2.093 · p95 2.335 · p99 2.795 · 5573 op/s</sub> | 2.212<br><sub>context: p90 3.479 · p95 3.903 · p99 4.483 · 3442 op/s</sub> | +64.5% (+0.867) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.429<br><sub>context: p90 5.486 · p95 6.200 · p99 6.612 · 278 op/s</sub> | 4.863<br><sub>context: p90 7.455 · p95 8.038 · p99 8.727 · 197 op/s</sub> | +41.8% (+1.435) | 150% AND 2 ms | 🟢 |
| 8 | 4.492<br><sub>context: p90 7.981 · p95 8.998 · p99 11.089 · 1613 op/s</sub> | 8.363<br><sub>context: p90 12.756 · p95 14.264 · p99 16.264 · 919 op/s</sub> | +86.2% (+3.871) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.213<br><sub>context: p90 0.266 · p95 0.282 · p99 0.295 · 4455 op/s</sub> | 0.157<br><sub>context: p90 0.186 · p95 0.189 · p99 0.200 · 6082 op/s</sub> | -26.2% (-0.056) | 150% AND 2 ms | 🟢 |
| 8 | 0.358<br><sub>context: p90 0.437 · p95 0.463 · p99 0.514 · 20947 op/s</sub> | 0.307<br><sub>context: p90 0.421 · p95 0.461 · p99 0.550 · 24626 op/s</sub> | -14.3% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.229<br><sub>context: p90 0.273 · p95 0.282 · p99 0.288 · 4203 op/s</sub> | 0.167<br><sub>context: p90 0.203 · p95 0.217 · p99 0.249 · 5577 op/s</sub> | -26.9% (-0.062) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.435 · p95 0.463 · p99 0.519 · 21561 op/s</sub> | 0.299<br><sub>context: p90 0.404 · p95 0.443 · p99 0.517 · 25437 op/s</sub> | -16.9% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.358<br><sub>context: p90 0.430 · p95 0.451 · p99 0.510 · 2715 op/s</sub> | 0.232<br><sub>context: p90 0.263 · p95 0.271 · p99 0.302 · 4103 op/s</sub> | -35.1% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.476<br><sub>context: p90 0.583 · p95 0.613 · p99 0.680 · 16285 op/s</sub> | 0.356<br><sub>context: p90 0.455 · p95 0.500 · p99 0.577 · 21201 op/s</sub> | -25.2% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.262 · p95 0.297 · p99 0.312 · 4882 op/s</sub> | 0.141<br><sub>context: p90 0.169 · p95 0.185 · p99 0.198 · 6352 op/s</sub> | -27.2% (-0.053) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.352 · p95 0.381 · p99 0.430 · 26698 op/s</sub> | 0.218<br><sub>context: p90 0.286 · p95 0.306 · p99 0.346 · 35282 op/s</sub> | -24.8% (-0.072) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.359<br><sub>context: p90 0.439 · p95 0.456 · p99 0.501 · 2659 op/s</sub> | 0.332<br><sub>context: p90 0.390 · p95 0.415 · p99 0.447 · 2974 op/s</sub> | -7.4% (-0.027) | 150% AND 2 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.521 · p95 0.549 · p99 0.608 · 17323 op/s</sub> | 0.395<br><sub>context: p90 0.478 · p95 0.509 · p99 0.549 · 19480 op/s</sub> | -11.5% (-0.051) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.345 · p95 0.392 · p99 0.417 · 3368 op/s</sub> | 0.231<br><sub>context: p90 0.283 · p95 0.309 · p99 0.348 · 4161 op/s</sub> | -18.3% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.372<br><sub>context: p90 0.440 · p95 0.467 · p99 0.519 · 21070 op/s</sub> | 0.325<br><sub>context: p90 0.415 · p95 0.446 · p99 0.502 · 23248 op/s</sub> | -12.4% (-0.046) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.420 · p95 0.440 · p99 0.483 · 2632 op/s</sub> | 0.334<br><sub>context: p90 0.420 · p95 0.443 · p99 0.493 · 2914 op/s</sub> | -9.8% (-0.036) | 150% AND 2 ms | 🟢 |
| 8 | 0.497<br><sub>context: p90 0.592 · p95 0.621 · p99 0.684 · 15646 op/s</sub> | 0.438<br><sub>context: p90 0.540 · p95 0.567 · p99 0.647 · 17406 op/s</sub> | -11.8% (-0.059) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.437<br><sub>context: p90 0.549 · p95 0.573 · p99 0.640 · 2213 op/s</sub> | 0.471<br><sub>context: p90 0.556 · p95 0.593 · p99 0.665 · 2107 op/s</sub> | +7.9% (+0.034) | 150% AND 2 ms | 🟢 |
| 8 | 0.642<br><sub>context: p90 0.821 · p95 0.878 · p99 1.032 · 12071 op/s</sub> | 0.610<br><sub>context: p90 0.781 · p95 0.840 · p99 0.945 · 12593 op/s</sub> | -5.0% (-0.032) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.477<br><sub>context: p90 0.581 · p95 0.632 · p99 0.675 · 2023 op/s</sub> | 0.473<br><sub>context: p90 0.583 · p95 0.607 · p99 0.653 · 2082 op/s</sub> | -0.7% (-0.004) | 150% AND 2 ms | 🟢 |
| 8 | 0.664<br><sub>context: p90 0.819 · p95 0.868 · p99 0.941 · 11717 op/s</sub> | 0.615<br><sub>context: p90 0.767 · p95 0.823 · p99 0.899 · 12526 op/s</sub> | -7.3% (-0.049) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.412<br><sub>context: p90 0.492 · p95 0.517 · p99 0.574 · 2350 op/s</sub> | 0.352<br><sub>context: p90 0.462 · p95 0.483 · p99 0.520 · 2685 op/s</sub> | -14.7% (-0.061) | 150% AND 2 ms | 🟢 |
| 8 | 0.535<br><sub>context: p90 0.634 · p95 0.666 · p99 0.727 · 14539 op/s</sub> | 0.462<br><sub>context: p90 0.564 · p95 0.590 · p99 0.668 · 16802 op/s</sub> | -13.6% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.410<br><sub>context: p90 0.497 · p95 0.520 · p99 0.558 · 2346 op/s</sub> | 0.284<br><sub>context: p90 0.366 · p95 0.390 · p99 0.416 · 3377 op/s</sub> | -30.7% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.539<br><sub>context: p90 0.635 · p95 0.662 · p99 0.755 · 14245 op/s</sub> | 0.386<br><sub>context: p90 0.486 · p95 0.519 · p99 0.566 · 20016 op/s</sub> | -28.3% (-0.153) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.168<br><sub>context: p90 18.411 · p95 18.486 · p99 18.626 · 55 op/s</sub> | 15.117<br><sub>context: p90 15.448 · p95 15.523 · p99 15.639 · 66 op/s</sub> | -16.8% (-3.051) | 150% AND 2 ms | 🟢 |
| 8 | 23.186<br><sub>context: p90 31.414 · p95 34.808 · p99 39.168 · 314 op/s</sub> | 18.069<br><sub>context: p90 23.854 · p95 26.310 · p99 30.656 · 393 op/s</sub> | -22.1% (-5.117) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.475<br><sub>context: p90 0.557 · p95 0.582 · p99 0.637 · 2031 op/s</sub> | 0.365<br><sub>context: p90 0.422 · p95 0.439 · p99 0.478 · 2714 op/s</sub> | -23.3% (-0.111) | 150% AND 2 ms | 🟢 |
| 8 | 0.645<br><sub>context: p90 0.757 · p95 0.789 · p99 0.890 · 11954 op/s</sub> | 0.446<br><sub>context: p90 0.537 · p95 0.566 · p99 0.621 · 17360 op/s</sub> | -30.8% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.472<br><sub>context: p90 2.006 · p95 2.196 · p99 2.608 · 672 op/s</sub> | 1.291<br><sub>context: p90 1.774 · p95 1.914 · p99 2.283 · 755 op/s</sub> | -12.3% (-0.181) | 150% AND 2 ms | 🟢 |
| 8 | 1.804<br><sub>context: p90 2.526 · p95 2.768 · p99 3.292 · 4319 op/s</sub> | 1.576<br><sub>context: p90 2.219 · p95 2.400 · p99 2.788 · 4959 op/s</sub> | -12.7% (-0.228) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.439<br><sub>context: p90 0.530 · p95 0.552 · p99 0.595 · 2221 op/s</sub> | 0.368<br><sub>context: p90 0.447 · p95 0.461 · p99 0.539 · 2685 op/s</sub> | -16.2% (-0.071) | 150% AND 2 ms | 🟢 |
| 8 | 0.555<br><sub>context: p90 0.666 · p95 0.700 · p99 0.775 · 13888 op/s</sub> | 0.465<br><sub>context: p90 0.578 · p95 0.610 · p99 0.699 · 16303 op/s</sub> | -16.2% (-0.090) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.199<br><sub>context: p90 2.775 · p95 2.870 · p99 3.000 · 464 op/s</sub> | 0.344<br><sub>context: p90 0.435 · p95 0.466 · p99 0.542 · 2765 op/s</sub> | -84.4% (-1.855) | 150% AND 2 ms | 🟢 |
| 8 | 2.531<br><sub>context: p90 3.244 · p95 3.414 · p99 3.611 · 3192 op/s</sub> | 0.424<br><sub>context: p90 0.531 · p95 0.569 · p99 0.660 · 18204 op/s</sub> | -83.2% (-2.106) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.197<br><sub>context: p90 2.787 · p95 2.884 · p99 2.993 · 467 op/s</sub> | 0.364<br><sub>context: p90 0.452 · p95 0.484 · p99 0.549 · 2669 op/s</sub> | -83.4% (-1.833) | 150% AND 2 ms | 🟢 |
| 8 | 2.539<br><sub>context: p90 3.349 · p95 3.516 · p99 3.757 · 3183 op/s</sub> | 0.438<br><sub>context: p90 0.555 · p95 0.602 · p99 0.670 · 17547 op/s</sub> | -82.7% (-2.100) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.197<br><sub>context: p90 0.242 · p95 0.253 · p99 0.284 · 4891 op/s</sub> | 0.194<br><sub>context: p90 0.246 · p95 0.272 · p99 0.333 · 4873 op/s</sub> | -1.6% (-0.003) | 150% AND 2 ms | 🟢 |
| 8 | 0.288<br><sub>context: p90 0.354 · p95 0.371 · p99 0.419 · 26873 op/s</sub> | 0.303<br><sub>context: p90 0.401 · p95 0.442 · p99 0.510 · 24913 op/s</sub> | +5.2% (+0.015) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.321<br><sub>context: p90 0.388 · p95 0.415 · p99 0.450 · 2979 op/s</sub> | 0.165<br><sub>context: p90 0.193 · p95 0.200 · p99 0.219 · 5685 op/s</sub> | -48.7% (-0.156) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.409<br><sub>context: p90 0.484 · p95 0.511 · p99 0.555 · 19181 op/s</sub> | 0.238<br><sub>context: p90 0.311 · p95 0.339 · p99 0.381 · 31848 op/s</sub> | -41.8% (-0.171) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.288<br><sub>context: p90 0.328 · p95 0.334 · p99 0.360 · 3349 op/s</sub> | 0.220<br><sub>context: p90 0.262 · p95 0.272 · p99 0.291 · 4415 op/s</sub> | -23.6% (-0.068) | 150% AND 2 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.531 · p95 0.569 · p99 0.628 · 17708 op/s</sub> | 0.362<br><sub>context: p90 0.470 · p95 0.507 · p99 0.591 · 21005 op/s</sub> | -16.7% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.315<br><sub>context: p90 0.365 · p95 0.384 · p99 0.422 · 3087 op/s</sub> | 0.262<br><sub>context: p90 0.342 · p95 0.359 · p99 0.406 · 3651 op/s</sub> | -16.8% (-0.053) | 150% AND 2 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.523 · p95 0.558 · p99 0.635 · 17769 op/s</sub> | 0.376<br><sub>context: p90 0.491 · p95 0.527 · p99 0.613 · 20453 op/s</sub> | -14.0% (-0.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.318 · p95 0.339 · p99 0.403 · 3659 op/s</sub> | 0.188<br><sub>context: p90 0.241 · p95 0.255 · p99 0.309 · 4992 op/s</sub> | -28.2% (-0.074) | 150% AND 2 ms | 🟢 |
| 8 | 0.388<br><sub>context: p90 0.464 · p95 0.491 · p99 0.545 · 19822 op/s</sub> | 0.309<br><sub>context: p90 0.401 · p95 0.435 · p99 0.503 · 24485 op/s</sub> | -20.4% (-0.079) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.389<br><sub>context: p90 1.495 · p95 1.554 · p99 1.685 · 717 op/s</sub> | 0.956<br><sub>context: p90 1.064 · p95 1.102 · p99 1.160 · 1030 op/s</sub> | -31.2% (-0.434) | 150% AND 2 ms | 🟢 |
| 8 | 1.807<br><sub>context: p90 2.155 · p95 2.293 · p99 2.560 · 4289 op/s</sub> | 1.165<br><sub>context: p90 1.331 · p95 1.392 · p99 1.466 · 6765 op/s</sub> | -35.5% (-0.642) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.271<br><sub>context: p90 1.364 · p95 1.420 · p99 1.514 · 783 op/s</sub> | 0.945<br><sub>context: p90 1.049 · p95 1.081 · p99 1.139 · 1039 op/s</sub> | -25.7% (-0.326) | 150% AND 2 ms | 🟢 |
| 8 | 1.599<br><sub>context: p90 1.939 · p95 2.078 · p99 2.478 · 4863 op/s</sub> | 1.080<br><sub>context: p90 1.205 · p95 1.242 · p99 1.368 · 7327 op/s</sub> | -32.5% (-0.520) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.360<br><sub>context: p90 0.460 · p95 0.480 · p99 0.560 · 2692 op/s</sub> | 0.262<br><sub>context: p90 0.308 · p95 0.323 · p99 0.359 · 3733 op/s</sub> | -27.1% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.463<br><sub>context: p90 0.558 · p95 0.589 · p99 0.664 · 16636 op/s</sub> | 0.381<br><sub>context: p90 0.478 · p95 0.510 · p99 0.600 · 20136 op/s</sub> | -17.7% (-0.082) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.454<br><sub>context: p90 0.546 · p95 0.570 · p99 0.610 · 2156 op/s</sub> | 0.386<br><sub>context: p90 0.477 · p95 0.527 · p99 0.587 · 2508 op/s</sub> | -14.9% (-0.068) | 150% AND 2 ms | 🟢 |
| 8 | 0.572<br><sub>context: p90 0.687 · p95 0.721 · p99 0.802 · 13624 op/s</sub> | 0.477<br><sub>context: p90 0.598 · p95 0.636 · p99 0.716 · 16052 op/s</sub> | -16.5% (-0.094) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.278<br><sub>context: p90 0.341 · p95 0.357 · p99 0.399 · 3477 op/s</sub> | 0.176<br><sub>context: p90 0.207 · p95 0.218 · p99 0.224 · 5490 op/s</sub> | -36.5% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.402<br><sub>context: p90 0.481 · p95 0.510 · p99 0.575 · 19337 op/s</sub> | 0.314<br><sub>context: p90 0.409 · p95 0.440 · p99 0.508 · 23903 op/s</sub> | -21.8% (-0.088) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.213<br><sub>context: p90 0.280 · p95 0.313 · p99 0.328 · 4427 op/s</sub> | 0.189<br><sub>context: p90 0.243 · p95 0.269 · p99 0.315 · 4978 op/s</sub> | -11.5% (-0.024) | 150% AND 2 ms | 🟢 |
| 8 | 0.295<br><sub>context: p90 0.363 · p95 0.385 · p99 0.445 · 25985 op/s</sub> | 0.298<br><sub>context: p90 0.398 · p95 0.438 · p99 0.511 · 25366 op/s</sub> | +1.1% (+0.003) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.207<br><sub>context: p90 0.238 · p95 0.247 · p99 0.278 · 4587 op/s</sub> | 0.158<br><sub>context: p90 0.199 · p95 0.209 · p99 0.228 · 6045 op/s</sub> | -23.9% (-0.050) | 150% AND 2 ms | 🟢 |
| 8 | 0.289<br><sub>context: p90 0.360 · p95 0.382 · p99 0.432 · 26578 op/s</sub> | 0.300<br><sub>context: p90 0.398 · p95 0.434 · p99 0.518 · 25287 op/s</sub> | +3.7% (+0.011) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.945<br><sub>context: p90 1.019 · p95 1.046 · p99 1.076 · 1039 op/s</sub> | 0.380<br><sub>context: p90 0.452 · p95 0.476 · p99 0.500 · 2496 op/s</sub> | -59.8% (-0.565) | 150% AND 2 ms | 🟢 |
| 8 | 1.273<br><sub>context: p90 1.787 · p95 1.962 · p99 2.259 · 5896 op/s</sub> | 0.459<br><sub>context: p90 0.533 · p95 0.557 · p99 0.595 · 16991 op/s</sub> | -63.9% (-0.814) | 150% AND 2 ms | 🟢 |

</details>
