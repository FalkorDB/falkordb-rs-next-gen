### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 | ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad |
| workload_hash | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**pr vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 → ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.088<br><sub>context: p90 1.202 · p95 1.236 · p99 1.277 · 870 op/s · total p50 4.601</sub> | 0.530<br><sub>context: p90 0.564 · p95 0.574 · p99 0.585 · 1634 op/s · total p50 2.437</sub> | -51.2% (-0.557) | 150% AND 2 ms | 🟢 |
| 8 | 1.293<br><sub>context: p90 1.914 · p95 2.198 · p99 2.476 · 5220 op/s · total p50 5.855</sub> | 0.587<br><sub>context: p90 0.653 · p95 0.682 · p99 0.725 · 9694 op/s · total p50 3.179</sub> | -54.6% (-0.706) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.650<br><sub>context: p90 1.770 · p95 1.794 · p99 1.854 · 584 op/s · total p50 6.813</sub> | 0.573<br><sub>context: p90 0.615 · p95 0.624 · p99 0.643 · 1481 op/s · total p50 2.708</sub> | -65.3% (-1.077) | 150% AND 2 ms | 🟢 |
| 8 | 1.846<br><sub>context: p90 2.307 · p95 2.528 · p99 2.836 · 3880 op/s · total p50 7.930</sub> | 0.608<br><sub>context: p90 0.660 · p95 0.682 · p99 0.731 · 9572 op/s · total p50 3.224</sub> | -67.1% (-1.238) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.650<br><sub>context: p90 1.758 · p95 1.778 · p99 1.844 · 583 op/s · total p50 6.856</sub> | 0.978<br><sub>context: p90 1.051 · p95 1.070 · p99 1.087 · 916 op/s · total p50 4.376</sub> | -40.7% (-0.672) | 150% AND 2 ms | 🟢 |
| 8 | 1.918<br><sub>context: p90 2.645 · p95 2.949 · p99 3.362 · 3682 op/s · total p50 8.330</sub> | 1.798<br><sub>context: p90 2.468 · p95 2.640 · p99 2.955 · 3891 op/s · total p50 7.988</sub> | -6.3% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.232<br><sub>context: p90 2.369 · p95 2.410 · p99 2.478 · 435 op/s · total p50 9.140</sub> | 1.114<br><sub>context: p90 1.167 · p95 1.178 · p99 1.189 · 805 op/s · total p50 4.969</sub> | -50.1% (-1.118) | 150% AND 2 ms | 🟢 |
| 8 | 2.514<br><sub>context: p90 3.317 · p95 3.638 · p99 4.386 · 2793 op/s · total p50 10.811</sub> | 1.175<br><sub>context: p90 1.308 · p95 1.346 · p99 1.431 · 5606 op/s · total p50 5.543</sub> | -53.3% (-1.339) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.063<br><sub>context: p90 0.081 · p95 0.102 · p99 0.115 · 10374 op/s · total p50 0.373</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.016 · p99 0.020 · 16475 op/s · total p50 0.228</sub> | -81.9% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.160<br><sub>context: p90 0.279 · p95 0.324 · p99 0.408 · 36348 op/s · total p50 0.821</sub> | 0.015<br><sub>context: p90 0.022 · p95 0.024 · p99 0.029 · 56127 op/s · total p50 0.410</sub> | -90.5% (-0.144) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.162<br><sub>context: p90 0.200 · p95 0.214 · p99 0.226 · 4863 op/s · total p50 0.816</sub> | 0.045<br><sub>context: p90 0.073 · p95 0.082 · p99 0.089 · 6401 op/s · total p50 0.612</sub> | -72.1% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.269<br><sub>context: p90 0.399 · p95 0.471 · p99 0.579 · 21590 op/s · total p50 1.426</sub> | 0.048<br><sub>context: p90 0.078 · p95 0.083 · p99 0.090 · 41809 op/s · total p50 0.724</sub> | -82.4% (-0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.174<br><sub>context: p90 0.218 · p95 0.231 · p99 0.242 · 4497 op/s · total p50 0.880</sub> | 0.047<br><sub>context: p90 0.073 · p95 0.077 · p99 0.090 · 6193 op/s · total p50 0.620</sub> | -72.7% (-0.126) | 150% AND 2 ms | 🟢 |
| 8 | 0.309<br><sub>context: p90 0.442 · p95 0.507 · p99 0.636 · 19455 op/s · total p50 1.584</sub> | 0.055<br><sub>context: p90 0.087 · p95 0.091 · p99 0.106 · 35434 op/s · total p50 0.838</sub> | -82.3% (-0.254) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.253 · p95 0.274 · p99 0.298 · 4195 op/s · total p50 0.942</sub> | 0.093<br><sub>context: p90 0.122 · p95 0.126 · p99 0.141 · 4289 op/s · total p50 0.898</sub> | -50.0% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.340<br><sub>context: p90 0.504 · p95 0.570 · p99 0.690 · 16938 op/s · total p50 1.810</sub> | 0.099<br><sub>context: p90 0.131 · p95 0.141 · p99 0.157 · 28740 op/s · total p50 1.011</sub> | -70.8% (-0.241) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.239<br><sub>context: p90 0.300 · p95 0.320 · p99 0.348 · 3379 op/s · total p50 1.193</sub> | 0.088<br><sub>context: p90 0.122 · p95 0.128 · p99 0.152 · 4352 op/s · total p50 0.916</sub> | -63.4% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.361<br><sub>context: p90 0.510 · p95 0.568 · p99 0.692 · 16265 op/s · total p50 1.902</sub> | 0.103<br><sub>context: p90 0.137 · p95 0.146 · p99 0.162 · 25934 op/s · total p50 1.153</sub> | -71.5% (-0.258) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.279<br><sub>context: p90 0.345 · p95 0.368 · p99 0.387 · 2914 op/s · total p50 1.366</sub> | 0.107<br><sub>context: p90 0.141 · p95 0.147 · p99 0.170 · 4173 op/s · total p50 0.954</sub> | -61.7% (-0.172) | 150% AND 2 ms | 🟢 |
| 8 | 0.407<br><sub>context: p90 0.573 · p95 0.657 · p99 0.823 · 13025 op/s · total p50 2.328</sub> | 0.129<br><sub>context: p90 0.176 · p95 0.189 · p99 0.214 · 16914 op/s · total p50 1.783</sub> | -68.3% (-0.278) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.301<br><sub>context: p90 0.378 · p95 0.392 · p99 0.443 · 2736 op/s · total p50 1.450</sub> | 0.136<br><sub>context: p90 0.177 · p95 0.183 · p99 0.193 · 3321 op/s · total p50 1.187</sub> | -54.8% (-0.165) | 150% AND 2 ms | 🟢 |
| 8 | 0.469<br><sub>context: p90 0.636 · p95 0.703 · p99 0.870 · 12234 op/s · total p50 2.474</sub> | 0.147<br><sub>context: p90 0.202 · p95 0.218 · p99 0.243 · 14482 op/s · total p50 2.096</sub> | -68.7% (-0.323) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.484<br><sub>context: p90 0.643 · p95 0.686 · p99 0.774 · 1694 op/s · total p50 2.324</sub> | 0.246<br><sub>context: p90 0.326 · p95 0.337 · p99 0.426 · 1966 op/s · total p50 1.935</sub> | -49.2% (-0.238) | 150% AND 2 ms | 🟢 |
| 8 | 0.548<br><sub>context: p90 0.787 · p95 0.870 · p99 1.015 · 4284 op/s · total p50 6.973</sub> | 0.247<br><sub>context: p90 0.351 · p95 0.383 · p99 0.419 · 4185 op/s · total p50 7.320</sub> | -55.0% (-0.301) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.553<br><sub>context: p90 0.705 · p95 0.732 · p99 0.830 · 1524 op/s · total p50 2.570</sub> | 0.259<br><sub>context: p90 0.333 · p95 0.370 · p99 0.389 · 1911 op/s · total p50 2.037</sub> | -53.1% (-0.293) | 150% AND 2 ms | 🟢 |
| 8 | 0.639<br><sub>context: p90 0.894 · p95 0.961 · p99 1.103 · 4050 op/s · total p50 7.421</sub> | 0.267<br><sub>context: p90 0.358 · p95 0.390 · p99 0.441 · 3831 op/s · total p50 7.870</sub> | -58.3% (-0.373) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.400<br><sub>context: p90 0.487 · p95 0.517 · p99 0.559 · 2203 op/s · total p50 1.799</sub> | 0.304<br><sub>context: p90 0.460 · p95 0.489 · p99 0.617 · 2179 op/s · total p50 1.809</sub> | -24.2% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.563<br><sub>context: p90 0.737 · p95 0.793 · p99 0.947 · 10790 op/s · total p50 2.852</sub> | 0.384<br><sub>context: p90 0.593 · p95 0.667 · p99 0.817 · 12355 op/s · total p50 2.469</sub> | -31.8% (-0.179) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.216<br><sub>context: p90 0.270 · p95 0.281 · p99 0.318 · 3783 op/s · total p50 1.043</sub> | 0.066<br><sub>context: p90 0.090 · p95 0.096 · p99 0.105 · 5664 op/s · total p50 0.689</sub> | -69.5% (-0.150) | 150% AND 2 ms | 🟢 |
| 8 | 0.378<br><sub>context: p90 0.547 · p95 0.626 · p99 0.743 · 15928 op/s · total p50 1.918</sub> | 0.081<br><sub>context: p90 0.115 · p95 0.122 · p99 0.144 · 30808 op/s · total p50 0.960</sub> | -78.5% (-0.297) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.140 · p95 0.155 · p99 0.176 · 6817 op/s · total p50 0.570</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 13860 op/s · total p50 0.284</sub> | -98.0% (-0.104) | 150% AND 2 ms | 🟢 |
| 8 | 0.225<br><sub>context: p90 0.361 · p95 0.421 · p99 0.516 · 26969 op/s · total p50 1.133</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 58677 op/s · total p50 0.415</sub> | -99.1% (-0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.083<br><sub>context: p90 0.118 · p95 0.132 · p99 0.140 · 7645 op/s · total p50 0.514</sub> | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.002 · 15995 op/s · total p50 0.246</sub> | -98.2% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.179<br><sub>context: p90 0.286 · p95 0.342 · p99 0.432 · 33672 op/s · total p50 0.897</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 61638 op/s · total p50 0.403</sub> | -98.7% (-0.177) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.278<br><sub>context: p90 0.351 · p95 0.368 · p99 0.412 · 2927 op/s · total p50 1.358</sub> | 0.059<br><sub>context: p90 0.089 · p95 0.092 · p99 0.097 · 4920 op/s · total p50 0.802</sub> | -78.8% (-0.219) | 150% AND 2 ms | 🟢 |
| 8 | 0.451<br><sub>context: p90 0.623 · p95 0.708 · p99 0.854 · 13601 op/s · total p50 2.225</sub> | 0.072<br><sub>context: p90 0.100 · p95 0.109 · p99 0.125 · 27475 op/s · total p50 1.059</sub> | -84.1% (-0.380) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.567<br><sub>context: p90 0.727 · p95 0.766 · p99 0.853 · 1664 op/s · total p50 2.380</sub> | 1.009<br><sub>context: p90 1.426 · p95 1.493 · p99 1.669 · 871 op/s · total p50 4.581</sub> | +77.9% (+0.442) | 150% AND 2 ms | 🟢 |
| 8 | 0.705<br><sub>context: p90 0.999 · p95 1.095 · p99 1.366 · 8003 op/s · total p50 3.814</sub> | 1.687<br><sub>context: p90 2.509 · p95 2.704 · p99 3.076 · 3866 op/s · total p50 7.789</sub> | +139.3% (+0.982) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.994<br><sub>context: p90 3.489 · p95 3.721 · p99 3.935 · 444 op/s · total p50 8.931</sub> | 4.236<br><sub>context: p90 6.566 · p95 7.168 · p99 7.646 · 218 op/s · total p50 18.448</sub> | +112.4% (+2.242) | 150% AND 2 ms | 🟢 |
| 8 | 2.150<br><sub>context: p90 3.728 · p95 4.135 · p99 5.224 · 3147 op/s · total p50 9.611</sub> | 7.072<br><sub>context: p90 11.151 · p95 11.908 · p99 13.003 · 1030 op/s · total p50 30.363</sub> | +229.0% (+4.922) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.092<br><sub>context: p90 0.134 · p95 0.155 · p99 0.174 · 6879 op/s · total p50 0.559</sub> | 0.013<br><sub>context: p90 0.017 · p95 0.018 · p99 0.018 · 12833 op/s · total p50 0.305</sub> | -86.0% (-0.079) | 150% AND 2 ms | 🟢 |
| 8 | 0.216<br><sub>context: p90 0.342 · p95 0.396 · p99 0.490 · 26695 op/s · total p50 1.153</sub> | 0.017<br><sub>context: p90 0.021 · p95 0.024 · p99 0.027 · 49630 op/s · total p50 0.567</sub> | -92.2% (-0.200) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.113<br><sub>context: p90 0.140 · p95 0.151 · p99 0.158 · 6685 op/s · total p50 0.598</sub> | 0.012<br><sub>context: p90 0.019 · p95 0.019 · p99 0.021 · 12182 op/s · total p50 0.317</sub> | -89.2% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.219<br><sub>context: p90 0.351 · p95 0.413 · p99 0.533 · 25709 op/s · total p50 1.190</sub> | 0.017<br><sub>context: p90 0.021 · p95 0.023 · p99 0.028 · 49205 op/s · total p50 0.538</sub> | -92.2% (-0.202) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.121 · p95 0.132 · p99 0.145 · 6234 op/s · total p50 0.616</sub> | 0.009<br><sub>context: p90 0.010 · p95 0.011 · p99 0.012 · 9743 op/s · total p50 0.392</sub> | -91.5% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.177<br><sub>context: p90 0.235 · p95 0.255 · p99 0.312 · 17228 op/s · total p50 1.774</sub> | 0.009<br><sub>context: p90 0.013 · p95 0.014 · p99 0.017 · 19823 op/s · total p50 1.593</sub> | -94.8% (-0.167) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.080<br><sub>context: p90 0.110 · p95 0.114 · p99 0.135 · 8663 op/s · total p50 0.456</sub> | 0.005<br><sub>context: p90 0.006 · p95 0.007 · p99 0.008 · 13721 op/s · total p50 0.281</sub> | -93.9% (-0.075) | 150% AND 2 ms | 🟢 |
| 8 | 0.178<br><sub>context: p90 0.296 · p95 0.341 · p99 0.410 · 32699 op/s · total p50 0.936</sub> | 0.005<br><sub>context: p90 0.007 · p95 0.008 · p99 0.010 · 58149 op/s · total p50 0.401</sub> | -97.1% (-0.173) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.197<br><sub>context: p90 0.232 · p95 0.240 · p99 0.271 · 4242 op/s · total p50 0.922</sub> | 0.124<br><sub>context: p90 0.158 · p95 0.164 · p99 0.187 · 4733 op/s · total p50 0.843</sub> | -37.2% (-0.073) | 150% AND 2 ms | 🟢 |
| 8 | 0.292<br><sub>context: p90 0.406 · p95 0.460 · p99 0.540 · 19478 op/s · total p50 1.586</sub> | 0.146<br><sub>context: p90 0.186 · p95 0.196 · p99 0.225 · 27987 op/s · total p50 1.061</sub> | -49.8% (-0.145) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.144<br><sub>context: p90 0.176 · p95 0.185 · p99 0.197 · 5597 op/s · total p50 0.706</sub> | 0.058<br><sub>context: p90 0.084 · p95 0.087 · p99 0.093 · 6970 op/s · total p50 0.570</sub> | -59.5% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.355 · p95 0.407 · p99 0.494 · 24564 op/s · total p50 1.234</sub> | 0.072<br><sub>context: p90 0.105 · p95 0.114 · p99 0.145 · 36980 op/s · total p50 0.796</sub> | -69.3% (-0.162) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.190<br><sub>context: p90 0.258 · p95 0.265 · p99 0.287 · 4076 op/s · total p50 0.966</sub> | 0.073<br><sub>context: p90 0.103 · p95 0.109 · p99 0.128 · 5395 op/s · total p50 0.733</sub> | -61.8% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.318<br><sub>context: p90 0.468 · p95 0.523 · p99 0.640 · 18051 op/s · total p50 1.667</sub> | 0.099<br><sub>context: p90 0.131 · p95 0.140 · p99 0.155 · 28700 op/s · total p50 1.010</sub> | -69.0% (-0.219) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.234 · p95 0.242 · p99 0.258 · 3898 op/s · total p50 1.011</sub> | 0.078<br><sub>context: p90 0.111 · p95 0.123 · p99 0.148 · 4559 op/s · total p50 0.835</sub> | -57.7% (-0.107) | 150% AND 2 ms | 🟢 |
| 8 | 0.297<br><sub>context: p90 0.417 · p95 0.469 · p99 0.565 · 16291 op/s · total p50 1.856</sub> | 0.101<br><sub>context: p90 0.137 · p95 0.148 · p99 0.171 · 15087 op/s · total p50 1.991</sub> | -65.9% (-0.196) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.213<br><sub>context: p90 0.274 · p95 0.296 · p99 0.321 · 3556 op/s · total p50 1.103</sub> | 0.100<br><sub>context: p90 0.136 · p95 0.143 · p99 0.163 · 3725 op/s · total p50 1.073</sub> | -53.1% (-0.113) | 150% AND 2 ms | 🟢 |
| 8 | 0.357<br><sub>context: p90 0.514 · p95 0.575 · p99 0.709 · 15240 op/s · total p50 1.971</sub> | 0.107<br><sub>context: p90 0.146 · p95 0.156 · p99 0.174 · 17307 op/s · total p50 1.766</sub> | -70.2% (-0.251) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.179<br><sub>context: p90 0.201 · p95 0.212 · p99 0.240 · 4416 op/s · total p50 0.875</sub> | 0.079<br><sub>context: p90 0.103 · p95 0.114 · p99 0.122 · 4656 op/s · total p50 0.786</sub> | -55.7% (-0.100) | 150% AND 2 ms | 🟢 |
| 8 | 0.345<br><sub>context: p90 0.487 · p95 0.556 · p99 0.669 · 16846 op/s · total p50 1.793</sub> | 0.103<br><sub>context: p90 0.136 · p95 0.147 · p99 0.172 · 24656 op/s · total p50 1.259</sub> | -70.2% (-0.242) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.232<br><sub>context: p90 0.305 · p95 0.315 · p99 0.338 · 3465 op/s · total p50 1.153</sub> | 0.062<br><sub>context: p90 0.089 · p95 0.095 · p99 0.100 · 6585 op/s · total p50 0.604</sub> | -73.3% (-0.170) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.512 · p95 0.577 · p99 0.703 · 16300 op/s · total p50 1.903</sub> | 0.076<br><sub>context: p90 0.109 · p95 0.116 · p99 0.141 · 34413 op/s · total p50 0.849</sub> | -78.8% (-0.283) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.482<br><sub>context: p90 5.817 · p95 6.204 · p99 6.855 · 71 op/s · total p50 54.327</sub> | 2.542<br><sub>context: p90 2.917 · p95 3.281 · p99 3.462 · 103 op/s · total p50 37.528</sub> | -53.6% (-2.940) | 150% AND 2 ms | 🟢 |
| 8 | 5.557<br><sub>context: p90 6.361 · p95 7.253 · p99 7.761 · 176 op/s · total p50 173.316</sub> | 2.593<br><sub>context: p90 2.727 · p95 2.776 · p99 3.059 · 171 op/s · total p50 179.566</sub> | -53.3% (-2.963) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.334 · p95 0.348 · p99 0.372 · 3055 op/s · total p50 1.317</sub> | 0.101<br><sub>context: p90 0.128 · p95 0.136 · p99 0.158 · 3834 op/s · total p50 1.031</sub> | -64.6% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.450<br><sub>context: p90 0.629 · p95 0.694 · p99 0.864 · 13263 op/s · total p50 2.325</sub> | 0.108<br><sub>context: p90 0.143 · p95 0.152 · p99 0.172 · 22948 op/s · total p50 1.365</sub> | -75.9% (-0.342) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.465<br><sub>context: p90 0.566 · p95 0.628 · p99 0.716 · 1467 op/s · total p50 2.453</sub> | 0.218<br><sub>context: p90 0.282 · p95 0.304 · p99 0.349 · 1267 op/s · total p50 3.198</sub> | -53.2% (-0.247) | 150% AND 2 ms | 🟢 |
| 8 | 0.491<br><sub>context: p90 0.673 · p95 0.720 · p99 0.825 · 2820 op/s · total p50 10.962</sub> | 0.237<br><sub>context: p90 0.323 · p95 0.345 · p99 0.408 · 2671 op/s · total p50 11.621</sub> | -51.7% (-0.254) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.260 · p95 0.280 · p99 0.301 · 3976 op/s · total p50 0.995</sub> | 0.093<br><sub>context: p90 0.126 · p95 0.136 · p99 0.154 · 4693 op/s · total p50 0.815</sub> | -52.0% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.343<br><sub>context: p90 0.496 · p95 0.564 · p99 0.682 · 17101 op/s · total p50 1.789</sub> | 0.107<br><sub>context: p90 0.143 · p95 0.152 · p99 0.174 · 25106 op/s · total p50 1.176</sub> | -68.9% (-0.236) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.828<br><sub>context: p90 2.407 · p95 2.457 · p99 2.553 · 537 op/s · total p50 7.556</sub> | 0.101<br><sub>context: p90 0.141 · p95 0.152 · p99 0.194 · 4797 op/s · total p50 0.832</sub> | -94.5% (-1.727) | 150% AND 2 ms | 🟢 |
| 8 | 2.030<br><sub>context: p90 2.576 · p95 2.731 · p99 3.711 · 3734 op/s · total p50 8.397</sub> | 0.121<br><sub>context: p90 0.168 · p95 0.186 · p99 0.234 · 26470 op/s · total p50 1.128</sub> | -94.0% (-1.909) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.757<br><sub>context: p90 2.238 · p95 2.370 · p99 2.533 · 560 op/s · total p50 7.137</sub> | 0.104<br><sub>context: p90 0.150 · p95 0.164 · p99 0.181 · 4286 op/s · total p50 0.924</sub> | -94.1% (-1.652) | 150% AND 2 ms | 🟢 |
| 8 | 1.935<br><sub>context: p90 2.571 · p95 2.728 · p99 3.687 · 3909 op/s · total p50 7.877</sub> | 0.123<br><sub>context: p90 0.176 · p95 0.196 · p99 0.229 · 24734 op/s · total p50 1.178</sub> | -93.6% (-1.812) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.115<br><sub>context: p90 0.133 · p95 0.142 · p99 0.163 · 6728 op/s · total p50 0.580</sub> | 0.035<br><sub>context: p90 0.068 · p95 0.069 · p99 0.074 · 8151 op/s · total p50 0.480</sub> | -69.7% (-0.080) | 150% AND 2 ms | 🟢 |
| 8 | 0.174<br><sub>context: p90 0.292 · p95 0.329 · p99 0.420 · 32069 op/s · total p50 0.950</sub> | 0.037<br><sub>context: p90 0.070 · p95 0.072 · p99 0.079 · 47117 op/s · total p50 0.572</sub> | -78.7% (-0.137) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.206 · p95 0.208 · p99 0.226 · 4741 op/s · total p50 0.837</sub> | 0.004<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 10989 op/s · total p50 0.359</sub> | -98.0% (-0.172) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.260<br><sub>context: p90 0.377 · p95 0.430 · p99 0.519 · 22135 op/s · total p50 1.392</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 50743 op/s · total p50 0.458</sub> | -98.6% (-0.257) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.144<br><sub>context: p90 0.187 · p95 0.197 · p99 0.211 · 5340 op/s · total p50 0.747</sub> | 0.039<br><sub>context: p90 0.064 · p95 0.070 · p99 0.076 · 7102 op/s · total p50 0.552</sub> | -73.1% (-0.105) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.453 · p95 0.530 · p99 0.664 · 20079 op/s · total p50 1.530</sub> | 0.047<br><sub>context: p90 0.077 · p95 0.081 · p99 0.089 · 38860 op/s · total p50 0.778</sub> | -84.7% (-0.258) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.170<br><sub>context: p90 0.202 · p95 0.217 · p99 0.239 · 4874 op/s · total p50 0.805</sub> | 0.061<br><sub>context: p90 0.114 · p95 0.118 · p99 0.121 · 6266 op/s · total p50 0.630</sub> | -64.0% (-0.109) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.444 · p95 0.513 · p99 0.637 · 20552 op/s · total p50 1.497</sub> | 0.066<br><sub>context: p90 0.124 · p95 0.132 · p99 0.142 · 37201 op/s · total p50 0.810</sub> | -78.4% (-0.239) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.172 · p95 0.176 · p99 0.183 · 5711 op/s · total p50 0.689</sub> | 0.033<br><sub>context: p90 0.062 · p95 0.068 · p99 0.073 · 8578 op/s · total p50 0.461</sub> | -74.7% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.272<br><sub>context: p90 0.419 · p95 0.475 · p99 0.571 · 22134 op/s · total p50 1.398</sub> | 0.040<br><sub>context: p90 0.072 · p95 0.074 · p99 0.084 · 41248 op/s · total p50 0.735</sub> | -85.4% (-0.232) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.953<br><sub>context: p90 1.040 · p95 1.059 · p99 1.138 · 988 op/s · total p50 4.040</sub> | 0.521<br><sub>context: p90 0.562 · p95 0.575 · p99 0.588 · 1508 op/s · total p50 2.637</sub> | -45.3% (-0.432) | 150% AND 2 ms | 🟢 |
| 8 | 1.134<br><sub>context: p90 1.408 · p95 1.597 · p99 1.873 · 6099 op/s · total p50 4.984</sub> | 0.593<br><sub>context: p90 0.725 · p95 0.759 · p99 0.865 · 8242 op/s · total p50 3.690</sub> | -47.7% (-0.541) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.959<br><sub>context: p90 1.051 · p95 1.075 · p99 1.112 · 986 op/s · total p50 4.070</sub> | 0.516<br><sub>context: p90 0.583 · p95 0.603 · p99 0.636 · 1532 op/s · total p50 2.595</sub> | -46.2% (-0.443) | 150% AND 2 ms | 🟢 |
| 8 | 1.125<br><sub>context: p90 1.399 · p95 1.557 · p99 1.871 · 6355 op/s · total p50 4.846</sub> | 0.573<br><sub>context: p90 0.661 · p95 0.691 · p99 0.758 · 9241 op/s · total p50 3.276</sub> | -49.0% (-0.551) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.168<br><sub>context: p90 0.208 · p95 0.211 · p99 0.227 · 4627 op/s · total p50 0.825</sub> | 0.052<br><sub>context: p90 0.078 · p95 0.087 · p99 0.093 · 6972 op/s · total p50 0.566</sub> | -68.9% (-0.116) | 150% AND 2 ms | 🟢 |
| 8 | 0.288<br><sub>context: p90 0.419 · p95 0.469 · p99 0.602 · 20282 op/s · total p50 1.521</sub> | 0.059<br><sub>context: p90 0.091 · p95 0.098 · p99 0.109 · 36613 op/s · total p50 0.813</sub> | -79.7% (-0.230) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.258<br><sub>context: p90 0.324 · p95 0.348 · p99 0.383 · 3286 op/s · total p50 1.182</sub> | 0.131<br><sub>context: p90 0.201 · p95 0.226 · p99 0.251 · 3652 op/s · total p50 1.056</sub> | -49.4% (-0.127) | 150% AND 2 ms | 🟢 |
| 8 | 0.371<br><sub>context: p90 0.512 · p95 0.583 · p99 0.698 · 15674 op/s · total p50 2.005</sub> | 0.161<br><sub>context: p90 0.247 · p95 0.274 · p99 0.331 · 21230 op/s · total p50 1.401</sub> | -56.5% (-0.209) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.138<br><sub>context: p90 0.180 · p95 0.186 · p99 0.241 · 5438 op/s · total p50 0.724</sub> | 0.020<br><sub>context: p90 0.031 · p95 0.037 · p99 0.049 · 8121 op/s · total p50 0.470</sub> | -85.3% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.239<br><sub>context: p90 0.368 · p95 0.424 · p99 0.516 · 24133 op/s · total p50 1.270</sub> | 0.023<br><sub>context: p90 0.030 · p95 0.033 · p99 0.039 · 45889 op/s · total p50 0.615</sub> | -90.3% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.086<br><sub>context: p90 0.119 · p95 0.123 · p99 0.140 · 8056 op/s · total p50 0.492</sub> | 0.031<br><sub>context: p90 0.057 · p95 0.068 · p99 0.069 · 8729 op/s · total p50 0.448</sub> | -63.7% (-0.055) | 150% AND 2 ms | 🟢 |
| 8 | 0.169<br><sub>context: p90 0.278 · p95 0.316 · p99 0.407 · 32572 op/s · total p50 0.943</sub> | 0.033<br><sub>context: p90 0.041 · p95 0.059 · p99 0.065 · 50204 op/s · total p50 0.491</sub> | -80.2% (-0.136) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.097<br><sub>context: p90 0.119 · p95 0.124 · p99 0.136 · 7732 op/s · total p50 0.509</sub> | 0.032<br><sub>context: p90 0.065 · p95 0.067 · p99 0.071 · 9336 op/s · total p50 0.419</sub> | -66.7% (-0.064) | 150% AND 2 ms | 🟢 |
| 8 | 0.168<br><sub>context: p90 0.288 · p95 0.331 · p99 0.418 · 32528 op/s · total p50 0.932</sub> | 0.035<br><sub>context: p90 0.066 · p95 0.068 · p99 0.075 · 48656 op/s · total p50 0.545</sub> | -79.3% (-0.133) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.708<br><sub>context: p90 0.790 · p95 0.811 · p99 0.831 · 1299 op/s · total p50 3.017</sub> | 0.183<br><sub>context: p90 0.189 · p95 0.192 · p99 0.201 · 3801 op/s · total p50 1.035</sub> | -74.2% (-0.525) | 150% AND 2 ms | 🟢 |
| 8 | 0.820<br><sub>context: p90 1.388 · p95 1.507 · p99 1.639 · 6610 op/s · total p50 4.553</sub> | 0.215<br><sub>context: p90 0.240 · p95 0.246 · p99 0.268 · 21950 op/s · total p50 1.399</sub> | -73.8% (-0.605) | 150% AND 2 ms | 🟢 |

</details>
