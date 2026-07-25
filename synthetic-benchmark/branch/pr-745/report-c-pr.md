### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:43f50272e0d144075251b10bcc8f77b24e371e5f6c7244c4b1848ab0fbe1937c |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:43f50272e0d144075251b10bcc8f77b24e371e5f6c7244c4b1848ab0fbe1937c

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.429<br><sub>context: p90 1.531 · p95 1.577 · p99 1.648 · 698 op/s</sub> | 1.178<br><sub>context: p90 1.343 · p95 1.402 · p99 1.451 · 858 op/s</sub> | -17.6% (-0.251) | 150% AND 2 ms | 🟢 |
| 8 | 1.898<br><sub>context: p90 2.498 · p95 2.728 · p99 3.118 · 4066 op/s</sub> | 0.943<br><sub>context: p90 1.079 · p95 1.116 · p99 1.203 · 8256 op/s</sub> | -50.3% (-0.955) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.069<br><sub>context: p90 2.160 · p95 2.186 · p99 2.247 · 479 op/s</sub> | 0.865<br><sub>context: p90 0.902 · p95 0.919 · p99 0.972 · 1145 op/s</sub> | -58.2% (-1.204) | 150% AND 2 ms | 🟢 |
| 8 | 2.649<br><sub>context: p90 3.426 · p95 3.513 · p99 3.839 · 2888 op/s</sub> | 1.093<br><sub>context: p90 1.302 · p95 1.342 · p99 1.418 · 7192 op/s</sub> | -58.7% (-1.556) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.020<br><sub>context: p90 2.105 · p95 2.132 · p99 2.197 · 493 op/s</sub> | 1.317<br><sub>context: p90 1.372 · p95 1.396 · p99 1.429 · 751 op/s</sub> | -34.8% (-0.703) | 150% AND 2 ms | 🟢 |
| 8 | 2.789<br><sub>context: p90 3.569 · p95 3.772 · p99 4.179 · 2785 op/s</sub> | 1.770<br><sub>context: p90 2.343 · p95 2.541 · p99 2.882 · 4301 op/s</sub> | -36.5% (-1.019) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.607<br><sub>context: p90 2.686 · p95 2.718 · p99 2.804 · 382 op/s</sub> | 1.387<br><sub>context: p90 1.431 · p95 1.457 · p99 1.503 · 717 op/s</sub> | -46.8% (-1.220) | 150% AND 2 ms | 🟢 |
| 8 | 3.627<br><sub>context: p90 4.690 · p95 4.954 · p99 5.557 · 2122 op/s</sub> | 1.638<br><sub>context: p90 1.970 · p95 2.015 · p99 2.111 · 4726 op/s</sub> | -54.8% (-1.989) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.159<br><sub>context: p90 0.189 · p95 0.194 · p99 0.241 · 5861 op/s</sub> | 0.149<br><sub>context: p90 0.168 · p95 0.173 · p99 0.185 · 6380 op/s</sub> | -6.8% (-0.011) | 150% AND 2 ms | 🟢 |
| 8 | 0.238<br><sub>context: p90 0.296 · p95 0.316 · p99 0.364 · 31928 op/s</sub> | 0.219<br><sub>context: p90 0.285 · p95 0.314 · p99 0.365 · 35013 op/s</sub> | -8.1% (-0.019) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.341<br><sub>context: p90 0.406 · p95 0.435 · p99 0.456 · 2846 op/s</sub> | 0.411<br><sub>context: p90 0.552 · p95 0.585 · p99 0.615 · 2330 op/s</sub> | +20.5% (+0.070) | 150% AND 2 ms | 🟢 |
| 8 | 0.441<br><sub>context: p90 0.530 · p95 0.561 · p99 0.620 · 17636 op/s</sub> | 0.338<br><sub>context: p90 0.433 · p95 0.458 · p99 0.528 · 22404 op/s</sub> | -23.3% (-0.103) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.364<br><sub>context: p90 0.429 · p95 0.454 · p99 0.498 · 2664 op/s</sub> | 0.241<br><sub>context: p90 0.321 · p95 0.344 · p99 0.397 · 3886 op/s</sub> | -33.7% (-0.122) | 150% AND 2 ms | 🟢 |
| 8 | 0.467<br><sub>context: p90 0.552 · p95 0.580 · p99 0.626 · 16751 op/s</sub> | 0.354<br><sub>context: p90 0.441 · p95 0.473 · p99 0.540 · 21704 op/s</sub> | -24.3% (-0.113) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.410<br><sub>context: p90 0.491 · p95 0.521 · p99 0.566 · 2385 op/s</sub> | 0.384<br><sub>context: p90 0.462 · p95 0.486 · p99 0.536 · 2564 op/s</sub> | -6.4% (-0.026) | 150% AND 2 ms | 🟢 |
| 8 | 0.529<br><sub>context: p90 0.618 · p95 0.648 · p99 0.713 · 14843 op/s</sub> | 0.460<br><sub>context: p90 0.562 · p95 0.595 · p99 0.659 · 17003 op/s</sub> | -13.0% (-0.069) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.490 · p95 0.509 · p99 0.558 · 2337 op/s</sub> | 0.374<br><sub>context: p90 0.446 · p95 0.467 · p99 0.507 · 2604 op/s</sub> | -10.7% (-0.045) | 150% AND 2 ms | 🟢 |
| 8 | 0.564<br><sub>context: p90 0.661 · p95 0.694 · p99 0.762 · 13920 op/s</sub> | 0.490<br><sub>context: p90 0.599 · p95 0.640 · p99 0.710 · 15676 op/s</sub> | -13.2% (-0.074) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.546<br><sub>context: p90 0.665 · p95 0.697 · p99 0.771 · 1800 op/s</sub> | 0.466<br><sub>context: p90 0.572 · p95 0.616 · p99 0.712 · 2076 op/s</sub> | -14.7% (-0.080) | 150% AND 2 ms | 🟢 |
| 8 | 0.740<br><sub>context: p90 0.931 · p95 1.003 · p99 1.118 · 10468 op/s</sub> | 0.656<br><sub>context: p90 0.837 · p95 0.891 · p99 1.037 · 11689 op/s</sub> | -11.3% (-0.083) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.573<br><sub>context: p90 0.722 · p95 0.753 · p99 0.825 · 1713 op/s</sub> | 0.489<br><sub>context: p90 0.599 · p95 0.636 · p99 0.684 · 1990 op/s</sub> | -14.6% (-0.084) | 150% AND 2 ms | 🟢 |
| 8 | 0.778<br><sub>context: p90 0.983 · p95 1.056 · p99 1.205 · 9945 op/s</sub> | 0.680<br><sub>context: p90 0.870 · p95 0.930 · p99 1.061 · 11337 op/s</sub> | -12.6% (-0.098) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.201<br><sub>context: p90 1.639 · p95 1.777 · p99 2.009 · 813 op/s</sub> | 1.005<br><sub>context: p90 1.322 · p95 1.506 · p99 1.729 · 968 op/s</sub> | -16.3% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 1.560<br><sub>context: p90 2.157 · p95 2.356 · p99 2.753 · 4987 op/s</sub> | 1.269<br><sub>context: p90 1.792 · p95 1.930 · p99 2.266 · 6030 op/s</sub> | -18.7% (-0.291) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.203<br><sub>context: p90 1.724 · p95 1.975 · p99 2.163 · 780 op/s</sub> | 1.162<br><sub>context: p90 1.546 · p95 1.709 · p99 1.881 · 829 op/s</sub> | -3.4% (-0.041) | 150% AND 2 ms | 🟢 |
| 8 | 1.630<br><sub>context: p90 2.313 · p95 2.544 · p99 2.850 · 4667 op/s</sub> | 1.421<br><sub>context: p90 1.941 · p95 2.135 · p99 2.483 · 5389 op/s</sub> | -12.8% (-0.209) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.601<br><sub>context: p90 0.682 · p95 0.715 · p99 0.792 · 1646 op/s</sub> | 0.576<br><sub>context: p90 0.759 · p95 0.800 · p99 0.984 · 1690 op/s</sub> | -4.0% (-0.024) | 150% AND 2 ms | 🟢 |
| 8 | 0.774<br><sub>context: p90 0.897 · p95 0.936 · p99 1.020 · 10221 op/s</sub> | 0.745<br><sub>context: p90 0.984 · p95 1.055 · p99 1.172 · 10559 op/s</sub> | -3.7% (-0.029) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.401<br><sub>context: p90 0.494 · p95 0.516 · p99 0.540 · 2414 op/s</sub> | 0.340<br><sub>context: p90 0.426 · p95 0.444 · p99 0.480 · 2883 op/s</sub> | -15.2% (-0.061) | 150% AND 2 ms | 🟢 |
| 8 | 0.572<br><sub>context: p90 0.678 · p95 0.714 · p99 0.811 · 13637 op/s</sub> | 0.401<br><sub>context: p90 0.502 · p95 0.534 · p99 0.593 · 18905 op/s</sub> | -30.0% (-0.171) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.297 · p95 0.322 · p99 0.349 · 4099 op/s</sub> | 0.142<br><sub>context: p90 0.175 · p95 0.181 · p99 0.204 · 6621 op/s</sub> | -40.3% (-0.096) | 150% AND 2 ms | 🟢 |
| 8 | 0.328<br><sub>context: p90 0.410 · p95 0.434 · p99 0.497 · 23429 op/s</sub> | 0.223<br><sub>context: p90 0.287 · p95 0.307 · p99 0.341 · 34323 op/s</sub> | -32.1% (-0.105) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.181<br><sub>context: p90 0.211 · p95 0.222 · p99 0.274 · 5250 op/s</sub> | 0.121<br><sub>context: p90 0.131 · p95 0.135 · p99 0.143 · 7984 op/s</sub> | -33.3% (-0.060) | 150% AND 2 ms | 🟢 |
| 8 | 0.263<br><sub>context: p90 0.325 · p95 0.346 · p99 0.390 · 28990 op/s</sub> | 0.203<br><sub>context: p90 0.273 · p95 0.296 · p99 0.346 · 38088 op/s</sub> | -23.0% (-0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.516<br><sub>context: p90 0.595 · p95 0.626 · p99 0.671 · 1896 op/s</sub> | 0.304<br><sub>context: p90 0.359 · p95 0.373 · p99 0.399 · 3172 op/s</sub> | -41.0% (-0.211) | 150% AND 2 ms | 🟢 |
| 8 | 0.664<br><sub>context: p90 0.780 · p95 0.809 · p99 0.889 · 11790 op/s</sub> | 0.435<br><sub>context: p90 0.535 · p95 0.560 · p99 0.635 · 17626 op/s</sub> | -34.5% (-0.229) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.984<br><sub>context: p90 1.414 · p95 1.554 · p99 1.743 · 979 op/s</sub> | 1.269<br><sub>context: p90 1.865 · p95 2.116 · p99 2.336 · 765 op/s</sub> | +29.1% (+0.286) | 150% AND 2 ms | 🟢 |
| 8 | 1.339<br><sub>context: p90 2.034 · p95 2.275 · p99 2.705 · 5612 op/s</sub> | 2.083<br><sub>context: p90 3.349 · p95 3.687 · p99 4.209 · 3623 op/s</sub> | +55.5% (+0.744) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 3.407<br><sub>context: p90 5.391 · p95 6.148 · p99 7.298 · 280 op/s</sub> | 4.361<br><sub>context: p90 6.622 · p95 7.350 · p99 7.860 · 221 op/s</sub> | +28.0% (+0.955) | 150% AND 2 ms | 🟢 |
| 8 | 4.429<br><sub>context: p90 7.694 · p95 8.979 · p99 11.012 · 1624 op/s</sub> | 8.207<br><sub>context: p90 12.790 · p95 14.211 · p99 15.554 · 940 op/s</sub> | +85.3% (+3.778) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.265<br><sub>context: p90 0.317 · p95 0.329 · p99 0.376 · 3669 op/s</sub> | 0.170<br><sub>context: p90 0.202 · p95 0.208 · p99 0.237 · 5534 op/s</sub> | -35.7% (-0.095) | 150% AND 2 ms | 🟢 |
| 8 | 0.365<br><sub>context: p90 0.447 · p95 0.473 · p99 0.528 · 21316 op/s</sub> | 0.293<br><sub>context: p90 0.394 · p95 0.417 · p99 0.497 · 25746 op/s</sub> | -19.8% (-0.072) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.245<br><sub>context: p90 0.278 · p95 0.287 · p99 0.311 · 3981 op/s</sub> | 0.171<br><sub>context: p90 0.194 · p95 0.202 · p99 0.224 · 5581 op/s</sub> | -30.2% (-0.074) | 150% AND 2 ms | 🟢 |
| 8 | 0.360<br><sub>context: p90 0.439 · p95 0.463 · p99 0.517 · 21334 op/s</sub> | 0.298<br><sub>context: p90 0.390 · p95 0.428 · p99 0.533 · 25505 op/s</sub> | -17.4% (-0.063) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.358<br><sub>context: p90 0.425 · p95 0.437 · p99 0.476 · 2733 op/s</sub> | 0.274<br><sub>context: p90 0.320 · p95 0.334 · p99 0.372 · 3526 op/s</sub> | -23.3% (-0.083) | 150% AND 2 ms | 🟢 |
| 8 | 0.505<br><sub>context: p90 0.620 · p95 0.664 · p99 0.751 · 15262 op/s</sub> | 0.356<br><sub>context: p90 0.461 · p95 0.497 · p99 0.563 · 21259 op/s</sub> | -29.6% (-0.150) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.209<br><sub>context: p90 0.263 · p95 0.311 · p99 0.329 · 4557 op/s</sub> | 0.139<br><sub>context: p90 0.171 · p95 0.194 · p99 0.225 · 6698 op/s</sub> | -33.3% (-0.070) | 150% AND 2 ms | 🟢 |
| 8 | 0.304<br><sub>context: p90 0.374 · p95 0.398 · p99 0.443 · 25491 op/s</sub> | 0.219<br><sub>context: p90 0.285 · p95 0.304 · p99 0.349 · 34915 op/s</sub> | -28.1% (-0.085) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.356<br><sub>context: p90 0.434 · p95 0.451 · p99 0.478 · 2693 op/s</sub> | 0.324<br><sub>context: p90 0.375 · p95 0.394 · p99 0.449 · 3038 op/s</sub> | -8.9% (-0.032) | 150% AND 2 ms | 🟢 |
| 8 | 0.452<br><sub>context: p90 0.531 · p95 0.556 · p99 0.595 · 17249 op/s</sub> | 0.394<br><sub>context: p90 0.483 · p95 0.512 · p99 0.566 · 19614 op/s</sub> | -12.8% (-0.058) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.300<br><sub>context: p90 0.338 · p95 0.366 · p99 0.401 · 3257 op/s</sub> | 0.258<br><sub>context: p90 0.380 · p95 0.396 · p99 0.440 · 3621 op/s</sub> | -13.9% (-0.042) | 150% AND 2 ms | 🟢 |
| 8 | 0.375<br><sub>context: p90 0.457 · p95 0.485 · p99 0.530 · 20540 op/s</sub> | 0.325<br><sub>context: p90 0.409 · p95 0.439 · p99 0.499 · 23599 op/s</sub> | -13.4% (-0.050) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.380<br><sub>context: p90 0.440 · p95 0.454 · p99 0.494 · 2541 op/s</sub> | 0.362<br><sub>context: p90 0.453 · p95 0.472 · p99 0.528 · 2638 op/s</sub> | -4.9% (-0.019) | 150% AND 2 ms | 🟢 |
| 8 | 0.506<br><sub>context: p90 0.605 · p95 0.639 · p99 0.702 · 15410 op/s</sub> | 0.456<br><sub>context: p90 0.562 · p95 0.595 · p99 0.670 · 16891 op/s</sub> | -9.9% (-0.050) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.477<br><sub>context: p90 0.582 · p95 0.621 · p99 0.655 · 2054 op/s</sub> | 0.731<br><sub>context: p90 0.862 · p95 0.897 · p99 1.012 · 1370 op/s</sub> | +53.5% (+0.255) | 150% AND 2 ms | 🟢 |
| 8 | 0.642<br><sub>context: p90 0.811 · p95 0.864 · p99 0.979 · 12062 op/s</sub> | 0.619<br><sub>context: p90 0.801 · p95 0.874 · p99 0.977 · 12319 op/s</sub> | -3.5% (-0.023) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.517<br><sub>context: p90 0.618 · p95 0.639 · p99 0.700 · 1898 op/s</sub> | 0.502<br><sub>context: p90 0.603 · p95 0.633 · p99 0.690 · 1980 op/s</sub> | -2.9% (-0.015) | 150% AND 2 ms | 🟢 |
| 8 | 0.665<br><sub>context: p90 0.823 · p95 0.874 · p99 0.994 · 11665 op/s</sub> | 0.623<br><sub>context: p90 0.783 · p95 0.843 · p99 0.956 · 12416 op/s</sub> | -6.3% (-0.042) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.408<br><sub>context: p90 0.480 · p95 0.494 · p99 0.530 · 2391 op/s</sub> | 0.452<br><sub>context: p90 0.567 · p95 0.621 · p99 0.716 · 2166 op/s</sub> | +10.8% (+0.044) | 150% AND 2 ms | 🟢 |
| 8 | 0.547<br><sub>context: p90 0.648 · p95 0.677 · p99 0.758 · 14366 op/s</sub> | 0.478<br><sub>context: p90 0.597 · p95 0.631 · p99 0.702 · 16295 op/s</sub> | -12.5% (-0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.441<br><sub>context: p90 0.503 · p95 0.515 · p99 0.560 · 2238 op/s</sub> | 0.282<br><sub>context: p90 0.340 · p95 0.364 · p99 0.400 · 3431 op/s</sub> | -36.1% (-0.159) | 150% AND 2 ms | 🟢 |
| 8 | 0.552<br><sub>context: p90 0.650 · p95 0.690 · p99 0.759 · 14094 op/s</sub> | 0.380<br><sub>context: p90 0.488 · p95 0.517 · p99 0.585 · 20013 op/s</sub> | -31.2% (-0.172) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 18.266<br><sub>context: p90 18.540 · p95 18.605 · p99 18.713 · 55 op/s</sub> | 15.190<br><sub>context: p90 15.563 · p95 15.709 · p99 15.996 · 66 op/s</sub> | -16.8% (-3.076) | 150% AND 2 ms | 🟢 |
| 8 | 23.734<br><sub>context: p90 31.967 · p95 35.629 · p99 39.167 · 305 op/s</sub> | 18.528<br><sub>context: p90 23.607 · p95 25.966 · p99 30.844 · 390 op/s</sub> | -21.9% (-5.206) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.468<br><sub>context: p90 0.526 · p95 0.552 · p99 0.573 · 2089 op/s</sub> | 0.370<br><sub>context: p90 0.429 · p95 0.453 · p99 0.489 · 2653 op/s</sub> | -21.0% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.643<br><sub>context: p90 0.752 · p95 0.791 · p99 0.872 · 11990 op/s</sub> | 0.465<br><sub>context: p90 0.558 · p95 0.593 · p99 0.658 · 16704 op/s</sub> | -27.7% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.361<br><sub>context: p90 1.845 · p95 2.024 · p99 2.419 · 724 op/s</sub> | 1.266<br><sub>context: p90 1.731 · p95 1.859 · p99 2.005 · 777 op/s</sub> | -7.0% (-0.095) | 150% AND 2 ms | 🟢 |
| 8 | 1.768<br><sub>context: p90 2.488 · p95 2.702 · p99 3.167 · 4383 op/s</sub> | 1.590<br><sub>context: p90 2.215 · p95 2.378 · p99 2.915 · 4903 op/s</sub> | -10.1% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.514 · p95 0.548 · p99 0.589 · 2291 op/s</sub> | 0.404<br><sub>context: p90 0.502 · p95 0.533 · p99 0.607 · 2384 op/s</sub> | -3.5% (-0.015) | 150% AND 2 ms | 🟢 |
| 8 | 0.556<br><sub>context: p90 0.664 · p95 0.704 · p99 0.769 · 14115 op/s</sub> | 0.478<br><sub>context: p90 0.595 · p95 0.634 · p99 0.697 · 16228 op/s</sub> | -14.0% (-0.078) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.194<br><sub>context: p90 2.787 · p95 2.839 · p99 2.918 · 467 op/s</sub> | 0.346<br><sub>context: p90 0.433 · p95 0.458 · p99 0.507 · 2812 op/s</sub> | -84.2% (-1.848) | 150% AND 2 ms | 🟢 |
| 8 | 2.555<br><sub>context: p90 3.305 · p95 3.460 · p99 3.712 · 3188 op/s</sub> | 0.416<br><sub>context: p90 0.520 · p95 0.557 · p99 0.657 · 18274 op/s</sub> | -83.7% (-2.139) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.178<br><sub>context: p90 2.796 · p95 2.884 · p99 2.976 · 471 op/s</sub> | 0.336<br><sub>context: p90 0.401 · p95 0.414 · p99 0.442 · 2905 op/s</sub> | -84.6% (-1.842) | 150% AND 2 ms | 🟢 |
| 8 | 2.582<br><sub>context: p90 3.323 · p95 3.495 · p99 3.742 · 3178 op/s</sub> | 0.472<br><sub>context: p90 0.611 · p95 0.659 · p99 0.745 · 16059 op/s</sub> | -81.7% (-2.110) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.203<br><sub>context: p90 0.238 · p95 0.244 · p99 0.254 · 4820 op/s</sub> | 0.408<br><sub>context: p90 0.618 · p95 0.680 · p99 0.734 · 2303 op/s</sub> | +101.4% (+0.206) | 150% AND 2 ms | 🟢 |
| 8 | 0.292<br><sub>context: p90 0.361 · p95 0.386 · p99 0.428 · 26179 op/s</sub> | 0.484<br><sub>context: p90 0.902 · p95 1.057 · p99 1.361 · 13080 op/s</sub> | +65.9% (+0.192) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.339<br><sub>context: p90 0.417 · p95 0.431 · p99 0.472 · 2843 op/s</sub> | 0.145<br><sub>context: p90 0.177 · p95 0.182 · p99 0.194 · 6582 op/s</sub> | -57.3% (-0.194) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.413<br><sub>context: p90 0.498 · p95 0.522 · p99 0.593 · 18667 op/s</sub> | 0.242<br><sub>context: p90 0.312 · p95 0.333 · p99 0.371 · 31763 op/s</sub> | -41.4% (-0.171) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.329<br><sub>context: p90 0.392 · p95 0.407 · p99 0.438 · 2940 op/s</sub> | 0.247<br><sub>context: p90 0.306 · p95 0.337 · p99 0.363 · 3951 op/s</sub> | -24.8% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.448<br><sub>context: p90 0.543 · p95 0.569 · p99 0.641 · 17180 op/s</sub> | 0.368<br><sub>context: p90 0.477 · p95 0.518 · p99 0.591 · 20888 op/s</sub> | -17.8% (-0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.313<br><sub>context: p90 0.361 · p95 0.379 · p99 0.421 · 3085 op/s</sub> | 0.252<br><sub>context: p90 0.338 · p95 0.352 · p99 0.386 · 3738 op/s</sub> | -19.4% (-0.061) | 150% AND 2 ms | 🟢 |
| 8 | 0.441<br><sub>context: p90 0.542 · p95 0.577 · p99 0.669 · 17130 op/s</sub> | 0.374<br><sub>context: p90 0.487 · p95 0.516 · p99 0.571 · 20286 op/s</sub> | -15.0% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.319<br><sub>context: p90 0.389 · p95 0.409 · p99 0.471 · 3020 op/s</sub> | 0.203<br><sub>context: p90 0.245 · p95 0.258 · p99 0.293 · 4738 op/s</sub> | -36.6% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.401<br><sub>context: p90 0.486 · p95 0.513 · p99 0.584 · 18885 op/s</sub> | 0.313<br><sub>context: p90 0.402 · p95 0.435 · p99 0.496 · 24331 op/s</sub> | -22.1% (-0.089) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.353<br><sub>context: p90 1.452 · p95 1.485 · p99 1.615 · 732 op/s</sub> | 0.970<br><sub>context: p90 1.026 · p95 1.044 · p99 1.083 · 1031 op/s</sub> | -28.3% (-0.383) | 150% AND 2 ms | 🟢 |
| 8 | 1.776<br><sub>context: p90 2.128 · p95 2.242 · p99 2.577 · 4357 op/s</sub> | 1.188<br><sub>context: p90 1.358 · p95 1.411 · p99 1.532 · 6558 op/s</sub> | -33.1% (-0.589) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.263<br><sub>context: p90 1.377 · p95 1.407 · p99 1.526 · 790 op/s</sub> | 0.960<br><sub>context: p90 1.015 · p95 1.033 · p99 1.071 · 1032 op/s</sub> | -24.0% (-0.303) | 150% AND 2 ms | 🟢 |
| 8 | 1.613<br><sub>context: p90 1.962 · p95 2.125 · p99 2.525 · 4716 op/s</sub> | 1.110<br><sub>context: p90 1.248 · p95 1.292 · p99 1.414 · 7116 op/s</sub> | -31.2% (-0.503) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.371<br><sub>context: p90 0.458 · p95 0.471 · p99 0.502 · 2602 op/s</sub> | 0.278<br><sub>context: p90 0.361 · p95 0.383 · p99 0.432 · 3364 op/s</sub> | -25.0% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.472<br><sub>context: p90 0.571 · p95 0.604 · p99 0.666 · 16179 op/s</sub> | 0.377<br><sub>context: p90 0.469 · p95 0.502 · p99 0.562 · 20231 op/s</sub> | -20.1% (-0.095) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.459<br><sub>context: p90 0.524 · p95 0.541 · p99 0.569 · 2155 op/s</sub> | 0.367<br><sub>context: p90 0.442 · p95 0.483 · p99 0.526 · 2648 op/s</sub> | -20.1% (-0.092) | 150% AND 2 ms | 🟢 |
| 8 | 0.586<br><sub>context: p90 0.702 · p95 0.736 · p99 0.823 · 13008 op/s</sub> | 0.467<br><sub>context: p90 0.584 · p95 0.617 · p99 0.697 · 16621 op/s</sub> | -20.2% (-0.118) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.286<br><sub>context: p90 0.317 · p95 0.331 · p99 0.379 · 3390 op/s</sub> | 0.192<br><sub>context: p90 0.218 · p95 0.221 · p99 0.224 · 5158 op/s</sub> | -33.0% (-0.095) | 150% AND 2 ms | 🟢 |
| 8 | 0.406<br><sub>context: p90 0.491 · p95 0.524 · p99 0.598 · 18747 op/s</sub> | 0.314<br><sub>context: p90 0.405 · p95 0.434 · p99 0.489 · 24165 op/s</sub> | -22.6% (-0.092) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.208<br><sub>context: p90 0.242 · p95 0.252 · p99 0.299 · 4639 op/s</sub> | 0.201<br><sub>context: p90 0.245 · p95 0.263 · p99 0.310 · 4771 op/s</sub> | -3.1% (-0.006) | 150% AND 2 ms | 🟢 |
| 8 | 0.293<br><sub>context: p90 0.357 · p95 0.377 · p99 0.435 · 26323 op/s</sub> | 0.301<br><sub>context: p90 0.399 · p95 0.431 · p99 0.499 · 24989 op/s</sub> | +2.7% (+0.008) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.210<br><sub>context: p90 0.249 · p95 0.275 · p99 0.309 · 4561 op/s</sub> | 0.178<br><sub>context: p90 0.208 · p95 0.221 · p99 0.238 · 5349 op/s</sub> | -15.4% (-0.032) | 150% AND 2 ms | 🟢 |
| 8 | 0.294<br><sub>context: p90 0.360 · p95 0.381 · p99 0.428 · 26238 op/s</sub> | 0.304<br><sub>context: p90 0.407 · p95 0.442 · p99 0.520 · 24966 op/s</sub> | +3.3% (+0.010) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.956<br><sub>context: p90 1.020 · p95 1.037 · p99 1.079 · 1031 op/s</sub> | 0.387<br><sub>context: p90 0.432 · p95 0.446 · p99 0.460 · 2524 op/s</sub> | -59.5% (-0.569) | 150% AND 2 ms | 🟢 |
| 8 | 1.270<br><sub>context: p90 1.648 · p95 1.839 · p99 2.174 · 6007 op/s</sub> | 0.448<br><sub>context: p90 0.521 · p95 0.547 · p99 0.590 · 17506 op/s</sub> | -64.7% (-0.822) | 150% AND 2 ms | 🟢 |

</details>
