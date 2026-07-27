### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
| workload_hash | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**main vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.129<br><sub>context: p90 1.199 · p95 1.212 · p99 1.232 · 734 op/s · total p50 1.353</sub> | 0.480<br><sub>context: p90 0.523 · p95 0.533 · p99 0.546 · 1382 op/s · total p50 0.709</sub> | -57.5% (-0.649) | 150% AND 2 ms | 🟢 |
| 8 | 1.203<br><sub>context: p90 1.693 · p95 1.897 · p99 2.109 · 4523 op/s · total p50 1.455</sub> | 0.496<br><sub>context: p90 0.563 · p95 0.591 · p99 0.631 · 9098 op/s · total p50 0.798</sub> | -58.7% (-0.707) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.693<br><sub>context: p90 1.756 · p95 1.776 · p99 1.834 · 511 op/s · total p50 1.947</sub> | 0.547<br><sub>context: p90 0.579 · p95 0.589 · p99 0.607 · 1081 op/s · total p50 0.916</sub> | -67.7% (-1.147) | 150% AND 2 ms | 🟢 |
| 8 | 1.808<br><sub>context: p90 2.208 · p95 2.446 · p99 2.731 · 3610 op/s · total p50 2.039</sub> | 0.559<br><sub>context: p90 0.634 · p95 0.669 · p99 0.728 · 8798 op/s · total p50 0.856</sub> | -69.1% (-1.249) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.690<br><sub>context: p90 1.762 · p95 1.772 · p99 1.800 · 502 op/s · total p50 1.988</sub> | 0.941<br><sub>context: p90 0.982 · p95 0.989 · p99 1.009 · 738 op/s · total p50 1.360</sub> | -44.3% (-0.749) | 150% AND 2 ms | 🟢 |
| 8 | 1.793<br><sub>context: p90 2.726 · p95 2.989 · p99 3.293 · 3267 op/s · total p50 2.098</sub> | 1.679<br><sub>context: p90 2.721 · p95 3.035 · p99 3.528 · 3462 op/s · total p50 2.111</sub> | -6.4% (-0.114) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.250<br><sub>context: p90 2.320 · p95 2.347 · p99 2.395 · 387 op/s · total p50 2.583</sub> | 0.990<br><sub>context: p90 1.015 · p95 1.026 · p99 1.045 · 725 op/s · total p50 1.371</sub> | -56.0% (-1.260) | 150% AND 2 ms | 🟢 |
| 8 | 2.517<br><sub>context: p90 3.806 · p95 4.242 · p99 5.063 · 2512 op/s · total p50 2.800</sub> | 0.986<br><sub>context: p90 1.063 · p95 1.092 · p99 1.132 · 5450 op/s · total p50 1.343</sub> | -60.8% (-1.531) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.060<br><sub>context: p90 0.096 · p95 0.109 · p99 0.119 · 5541 op/s · total p50 0.158</sub> | 0.019<br><sub>context: p90 0.045 · p95 0.048 · p99 0.051 · 5466 op/s · total p50 0.163</sub> | -68.1% (-0.041) | 150% AND 2 ms | 🟢 |
| 8 | 0.123<br><sub>context: p90 0.193 · p95 0.224 · p99 0.269 · 28773 op/s · total p50 0.250</sub> | 0.016<br><sub>context: p90 0.026 · p95 0.029 · p99 0.038 · 33690 op/s · total p50 0.223</sub> | -86.6% (-0.106) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.233<br><sub>context: p90 0.285 · p95 0.300 · p99 0.313 · 2332 op/s · total p50 0.415</sub> | 0.062<br><sub>context: p90 0.096 · p95 0.102 · p99 0.119 · 3042 op/s · total p50 0.328</sub> | -73.2% (-0.171) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.347 · p95 0.377 · p99 0.453 · 16959 op/s · total p50 0.440</sub> | 0.050<br><sub>context: p90 0.079 · p95 0.084 · p99 0.094 · 24082 op/s · total p50 0.313</sub> | -81.1% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.310 · p95 0.314 · p99 0.332 · 2315 op/s · total p50 0.417</sub> | 0.059<br><sub>context: p90 0.083 · p95 0.096 · p99 0.109 · 3323 op/s · total p50 0.274</sub> | -73.5% (-0.163) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.383 · p95 0.418 · p99 0.486 · 16295 op/s · total p50 0.459</sub> | 0.052<br><sub>context: p90 0.080 · p95 0.086 · p99 0.096 · 23670 op/s · total p50 0.327</sub> | -82.0% (-0.238) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.330 · p95 0.338 · p99 0.345 · 2041 op/s · total p50 0.472</sub> | 0.103<br><sub>context: p90 0.150 · p95 0.162 · p99 0.174 · 2564 op/s · total p50 0.371</sub> | -60.5% (-0.158) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.395 · p95 0.431 · p99 0.506 · 14933 op/s · total p50 0.505</sub> | 0.106<br><sub>context: p90 0.141 · p95 0.151 · p99 0.171 · 17021 op/s · total p50 0.448</sub> | -65.2% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.329<br><sub>context: p90 0.370 · p95 0.379 · p99 0.392 · 1820 op/s · total p50 0.553</sub> | 0.104<br><sub>context: p90 0.146 · p95 0.155 · p99 0.181 · 2344 op/s · total p50 0.406</sub> | -68.4% (-0.225) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.431 · p95 0.466 · p99 0.543 · 14216 op/s · total p50 0.539</sub> | 0.109<br><sub>context: p90 0.144 · p95 0.153 · p99 0.173 · 16435 op/s · total p50 0.465</sub> | -67.8% (-0.230) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.365<br><sub>context: p90 0.421 · p95 0.439 · p99 0.474 · 1418 op/s · total p50 0.701</sub> | 0.157<br><sub>context: p90 0.199 · p95 0.208 · p99 0.227 · 1613 op/s · total p50 0.608</sub> | -56.9% (-0.208) | 150% AND 2 ms | 🟢 |
| 8 | 0.386<br><sub>context: p90 0.492 · p95 0.527 · p99 0.596 · 10751 op/s · total p50 0.702</sub> | 0.149<br><sub>context: p90 0.194 · p95 0.206 · p99 0.234 · 12743 op/s · total p50 0.608</sub> | -61.5% (-0.237) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.481 · p95 0.496 · p99 0.539 · 1272 op/s · total p50 0.770</sub> | 0.156<br><sub>context: p90 0.218 · p95 0.227 · p99 0.242 · 1746 op/s · total p50 0.564</sub> | -62.7% (-0.263) | 150% AND 2 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.546 · p95 0.584 · p99 0.649 · 10071 op/s · total p50 0.768</sub> | 0.155<br><sub>context: p90 0.205 · p95 0.218 · p99 0.240 · 11686 op/s · total p50 0.654</sub> | -64.6% (-0.282) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.626<br><sub>context: p90 0.748 · p95 0.785 · p99 0.893 · 757 op/s · total p50 1.301</sub> | 0.286<br><sub>context: p90 0.363 · p95 0.374 · p99 0.393 · 892 op/s · total p50 1.090</sub> | -54.4% (-0.341) | 150% AND 2 ms | 🟢 |
| 8 | 0.655<br><sub>context: p90 0.856 · p95 0.932 · p99 1.054 · 5261 op/s · total p50 1.463</sub> | 0.285<br><sub>context: p90 0.378 · p95 0.397 · p99 0.442 · 6305 op/s · total p50 1.221</sub> | -56.4% (-0.369) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.622<br><sub>context: p90 0.790 · p95 0.843 · p99 0.910 · 745 op/s · total p50 1.307</sub> | 0.255<br><sub>context: p90 0.341 · p95 0.366 · p99 0.426 · 869 op/s · total p50 1.121</sub> | -58.9% (-0.366) | 150% AND 2 ms | 🟢 |
| 8 | 0.725<br><sub>context: p90 0.959 · p95 1.027 · p99 1.170 · 5013 op/s · total p50 1.524</sub> | 0.296<br><sub>context: p90 0.386 · p95 0.419 · p99 0.475 · 6197 op/s · total p50 1.236</sub> | -59.2% (-0.429) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.475<br><sub>context: p90 0.557 · p95 0.579 · p99 0.613 · 1430 op/s · total p50 0.691</sub> | 0.329<br><sub>context: p90 0.484 · p95 0.546 · p99 0.593 · 1461 op/s · total p50 0.680</sub> | -30.7% (-0.146) | 150% AND 2 ms | 🟢 |
| 8 | 0.555<br><sub>context: p90 0.707 · p95 0.753 · p99 0.877 · 9864 op/s · total p50 0.765</sub> | 0.500<br><sub>context: p90 0.798 · p95 0.877 · p99 1.030 · 9228 op/s · total p50 0.830</sub> | -9.9% (-0.055) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.358 · p95 0.367 · p99 0.378 · 2000 op/s · total p50 0.487</sub> | 0.092<br><sub>context: p90 0.128 · p95 0.138 · p99 0.150 · 2757 op/s · total p50 0.339</sub> | -67.4% (-0.191) | 150% AND 2 ms | 🟢 |
| 8 | 0.372<br><sub>context: p90 0.496 · p95 0.540 · p99 0.657 · 13663 op/s · total p50 0.553</sub> | 0.085<br><sub>context: p90 0.117 · p95 0.127 · p99 0.145 · 19912 op/s · total p50 0.377</sub> | -77.2% (-0.287) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.194 · p95 0.215 · p99 0.227 · 3885 op/s · total p50 0.240</sub> | 0.002<br><sub>context: p90 0.006 · p95 0.006 · p99 0.007 · 5730 op/s · total p50 0.150</sub> | -98.2% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.194<br><sub>context: p90 0.274 · p95 0.304 · p99 0.385 · 23186 op/s · total p50 0.326</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 33991 op/s · total p50 0.223</sub> | -98.8% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.092<br><sub>context: p90 0.155 · p95 0.161 · p99 0.172 · 3996 op/s · total p50 0.225</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.004 · 7802 op/s · total p50 0.110</sub> | -98.3% (-0.090) | 150% AND 2 ms | 🟢 |
| 8 | 0.143<br><sub>context: p90 0.201 · p95 0.222 · p99 0.273 · 27426 op/s · total p50 0.272</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.004 · 37223 op/s · total p50 0.204</sub> | -98.6% (-0.141) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.336<br><sub>context: p90 0.414 · p95 0.424 · p99 0.440 · 1831 op/s · total p50 0.525</sub> | 0.084<br><sub>context: p90 0.112 · p95 0.127 · p99 0.135 · 2137 op/s · total p50 0.467</sub> | -75.0% (-0.252) | 150% AND 2 ms | 🟢 |
| 8 | 0.425<br><sub>context: p90 0.541 · p95 0.594 · p99 0.704 · 12316 op/s · total p50 0.621</sub> | 0.073<br><sub>context: p90 0.105 · p95 0.113 · p99 0.128 · 18544 op/s · total p50 0.414</sub> | -82.7% (-0.352) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.670<br><sub>context: p90 0.942 · p95 1.016 · p99 1.114 · 1105 op/s · total p50 0.901</sub> | 1.031<br><sub>context: p90 1.594 · p95 1.775 · p99 2.063 · 686 op/s · total p50 1.409</sub> | +54.0% (+0.362) | 150% AND 2 ms | 🟢 |
| 8 | 0.724<br><sub>context: p90 1.086 · p95 1.197 · p99 1.445 · 7647 op/s · total p50 0.979</sub> | 2.723<br><sub>context: p90 4.546 · p95 5.117 · p99 5.987 · 2398 op/s · total p50 3.154</sub> | +276.3% (+2.000) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.099<br><sub>context: p90 3.231 · p95 3.678 · p99 4.055 · 408 op/s · total p50 2.381</sub> | 4.122<br><sub>context: p90 6.285 · p95 7.114 · p99 7.463 · 208 op/s · total p50 4.684</sub> | +96.4% (+2.023) | 150% AND 2 ms | 🟢 |
| 8 | 2.190<br><sub>context: p90 3.653 · p95 4.064 · p99 4.708 · 3021 op/s · total p50 2.432</sub> | 12.800<br><sub>context: p90 19.330 · p95 21.470 · p99 23.513 · 584 op/s · total p50 13.254</sub> | +484.4% (+10.610) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.186 · p95 0.208 · p99 0.221 · 3517 op/s · total p50 0.250</sub> | 0.018<br><sub>context: p90 0.041 · p95 0.045 · p99 0.050 · 4927 op/s · total p50 0.187</sub> | -82.9% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.177<br><sub>context: p90 0.239 · p95 0.265 · p99 0.318 · 21694 op/s · total p50 0.337</sub> | 0.018<br><sub>context: p90 0.024 · p95 0.028 · p99 0.034 · 28755 op/s · total p50 0.262</sub> | -89.9% (-0.160) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.127<br><sub>context: p90 0.204 · p95 0.211 · p99 0.220 · 3440 op/s · total p50 0.269</sub> | 0.013<br><sub>context: p90 0.042 · p95 0.045 · p99 0.052 · 4417 op/s · total p50 0.196</sub> | -90.0% (-0.114) | 150% AND 2 ms | 🟢 |
| 8 | 0.179<br><sub>context: p90 0.238 · p95 0.268 · p99 0.324 · 22042 op/s · total p50 0.341</sub> | 0.017<br><sub>context: p90 0.023 · p95 0.026 · p99 0.033 · 29437 op/s · total p50 0.257</sub> | -90.6% (-0.162) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.142<br><sub>context: p90 0.212 · p95 0.222 · p99 0.231 · 2674 op/s · total p50 0.360</sub> | 0.012<br><sub>context: p90 0.020 · p95 0.021 · p99 0.025 · 3563 op/s · total p50 0.268</sub> | -91.5% (-0.130) | 150% AND 2 ms | 🟢 |
| 8 | 0.189<br><sub>context: p90 0.251 · p95 0.275 · p99 0.319 · 16694 op/s · total p50 0.457</sub> | 0.011<br><sub>context: p90 0.016 · p95 0.018 · p99 0.022 · 21718 op/s · total p50 0.343</sub> | -94.3% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.149<br><sub>context: p90 0.181 · p95 0.187 · p99 0.200 · 2642 op/s · total p50 0.354</sub> | 0.005<br><sub>context: p90 0.006 · p95 0.007 · p99 0.007 · 7272 op/s · total p50 0.136</sub> | -96.4% (-0.143) | 150% AND 2 ms | 🟢 |
| 8 | 0.152<br><sub>context: p90 0.211 · p95 0.238 · p99 0.290 · 26627 op/s · total p50 0.282</sub> | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.012 · 34125 op/s · total p50 0.222</sub> | -96.2% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.288 · p95 0.297 · p99 0.330 · 2511 op/s · total p50 0.386</sub> | 0.130<br><sub>context: p90 0.175 · p95 0.184 · p99 0.204 · 2233 op/s · total p50 0.441</sub> | -42.3% (-0.095) | 150% AND 2 ms | 🟢 |
| 8 | 0.273<br><sub>context: p90 0.348 · p95 0.374 · p99 0.429 · 17435 op/s · total p50 0.436</sub> | 0.128<br><sub>context: p90 0.165 · p95 0.178 · p99 0.200 · 18651 op/s · total p50 0.411</sub> | -53.0% (-0.144) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.226 · p95 0.239 · p99 0.255 · 3357 op/s · total p50 0.274</sub> | 0.067<br><sub>context: p90 0.100 · p95 0.110 · p99 0.121 · 3795 op/s · total p50 0.246</sub> | -56.1% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.220<br><sub>context: p90 0.289 · p95 0.319 · p99 0.375 · 20816 op/s · total p50 0.365</sub> | 0.066<br><sub>context: p90 0.095 · p95 0.104 · p99 0.123 · 24662 op/s · total p50 0.311</sub> | -70.1% (-0.154) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.323 · p95 0.327 · p99 0.345 · 2151 op/s · total p50 0.451</sub> | 0.122<br><sub>context: p90 0.152 · p95 0.166 · p99 0.173 · 1946 op/s · total p50 0.508</sub> | -53.3% (-0.139) | 150% AND 2 ms | 🟢 |
| 8 | 0.297<br><sub>context: p90 0.385 · p95 0.417 · p99 0.497 · 15134 op/s · total p50 0.500</sub> | 0.097<br><sub>context: p90 0.129 · p95 0.139 · p99 0.161 · 18456 op/s · total p50 0.418</sub> | -67.2% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.264<br><sub>context: p90 0.318 · p95 0.324 · p99 0.338 · 1672 op/s · total p50 0.588</sub> | 0.118<br><sub>context: p90 0.151 · p95 0.158 · p99 0.178 · 1746 op/s · total p50 0.559</sub> | -55.3% (-0.146) | 150% AND 2 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.384 · p95 0.410 · p99 0.453 · 11703 op/s · total p50 0.642</sub> | 0.108<br><sub>context: p90 0.142 · p95 0.153 · p99 0.173 · 13206 op/s · total p50 0.581</sub> | -63.8% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.309<br><sub>context: p90 0.353 · p95 0.359 · p99 0.381 · 1658 op/s · total p50 0.596</sub> | 0.127<br><sub>context: p90 0.156 · p95 0.163 · p99 0.179 · 1655 op/s · total p50 0.593</sub> | -59.0% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.330<br><sub>context: p90 0.419 · p95 0.453 · p99 0.522 · 11667 op/s · total p50 0.658</sub> | 0.114<br><sub>context: p90 0.152 · p95 0.164 · p99 0.188 · 12532 op/s · total p50 0.606</sub> | -65.4% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.298<br><sub>context: p90 0.356 · p95 0.365 · p99 0.386 · 1759 op/s · total p50 0.557</sub> | 0.091<br><sub>context: p90 0.152 · p95 0.161 · p99 0.178 · 2414 op/s · total p50 0.392</sub> | -69.6% (-0.207) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.436 · p95 0.477 · p99 0.558 · 13595 op/s · total p50 0.550</sub> | 0.104<br><sub>context: p90 0.136 · p95 0.145 · p99 0.165 · 16208 op/s · total p50 0.474</sub> | -68.6% (-0.228) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.271<br><sub>context: p90 0.354 · p95 0.365 · p99 0.385 · 2217 op/s · total p50 0.438</sub> | 0.060<br><sub>context: p90 0.121 · p95 0.135 · p99 0.151 · 3272 op/s · total p50 0.281</sub> | -77.8% (-0.211) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.442 · p95 0.486 · p99 0.578 · 14905 op/s · total p50 0.511</sub> | 0.081<br><sub>context: p90 0.114 · p95 0.123 · p99 0.139 · 20405 op/s · total p50 0.372</sub> | -75.9% (-0.257) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.781<br><sub>context: p90 5.986 · p95 6.032 · p99 6.250 · 60 op/s · total p50 16.734</sub> | 2.524<br><sub>context: p90 2.570 · p95 2.581 · p99 2.608 · 73 op/s · total p50 13.681</sub> | -56.3% (-3.258) | 150% AND 2 ms | 🟢 |
| 8 | 7.237<br><sub>context: p90 8.850 · p95 9.272 · p99 10.230 · 372 op/s · total p50 20.842</sub> | 3.155<br><sub>context: p90 3.436 · p95 3.470 · p99 3.629 · 465 op/s · total p50 16.187</sub> | -56.4% (-4.082) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.332<br><sub>context: p90 0.422 · p95 0.441 · p99 0.449 · 1797 op/s · total p50 0.531</sub> | 0.104<br><sub>context: p90 0.153 · p95 0.165 · p99 0.182 · 2532 op/s · total p50 0.367</sub> | -68.7% (-0.228) | 150% AND 2 ms | 🟢 |
| 8 | 0.428<br><sub>context: p90 0.545 · p95 0.585 · p99 0.674 · 12601 op/s · total p50 0.601</sub> | 0.112<br><sub>context: p90 0.145 · p95 0.156 · p99 0.181 · 16488 op/s · total p50 0.465</sub> | -73.8% (-0.316) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.570<br><sub>context: p90 0.668 · p95 0.691 · p99 0.741 · 656 op/s · total p50 1.501</sub> | 0.255<br><sub>context: p90 0.324 · p95 0.336 · p99 0.362 · 744 op/s · total p50 1.320</sub> | -55.2% (-0.315) | 150% AND 2 ms | 🟢 |
| 8 | 0.614<br><sub>context: p90 0.782 · p95 0.836 · p99 0.925 · 4632 op/s · total p50 1.672</sub> | 0.268<br><sub>context: p90 0.351 · p95 0.374 · p99 0.414 · 5338 op/s · total p50 1.439</sub> | -56.4% (-0.346) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.345 · p95 0.354 · p99 0.375 · 1811 op/s · total p50 0.554</sub> | 0.107<br><sub>context: p90 0.151 · p95 0.161 · p99 0.171 · 2139 op/s · total p50 0.461</sub> | -64.1% (-0.192) | 150% AND 2 ms | 🟢 |
| 8 | 0.315<br><sub>context: p90 0.405 · p95 0.432 · p99 0.510 · 14090 op/s · total p50 0.534</sub> | 0.105<br><sub>context: p90 0.139 · p95 0.149 · p99 0.174 · 16704 op/s · total p50 0.460</sub> | -66.6% (-0.210) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.741<br><sub>context: p90 2.194 · p95 2.242 · p99 2.330 · 490 op/s · total p50 2.085</sub> | 0.138<br><sub>context: p90 0.191 · p95 0.207 · p99 0.244 · 2162 op/s · total p50 0.456</sub> | -92.1% (-1.603) | 150% AND 2 ms | 🟢 |
| 8 | 1.778<br><sub>context: p90 2.293 · p95 2.389 · p99 2.547 · 3938 op/s · total p50 2.015</sub> | 0.116<br><sub>context: p90 0.168 · p95 0.182 · p99 0.213 · 18771 op/s · total p50 0.403</sub> | -93.5% (-1.662) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.772<br><sub>context: p90 2.278 · p95 2.316 · p99 2.394 · 482 op/s · total p50 2.123</sub> | 0.129<br><sub>context: p90 0.187 · p95 0.206 · p99 0.219 · 2277 op/s · total p50 0.433</sub> | -92.7% (-1.643) | 150% AND 2 ms | 🟢 |
| 8 | 1.837<br><sub>context: p90 2.355 · p95 2.446 · p99 2.612 · 3439 op/s · total p50 2.238</sub> | 0.123<br><sub>context: p90 0.180 · p95 0.198 · p99 0.236 · 16600 op/s · total p50 0.454</sub> | -93.3% (-1.714) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.183 · p95 0.188 · p99 0.195 · 3546 op/s · total p50 0.270</sub> | 0.045<br><sub>context: p90 0.074 · p95 0.077 · p99 0.084 · 3220 op/s · total p50 0.310</sub> | -65.1% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.150<br><sub>context: p90 0.203 · p95 0.223 · p99 0.273 · 25565 op/s · total p50 0.296</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.066 · p99 0.073 · 29272 op/s · total p50 0.261</sub> | -77.1% (-0.116) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.248 · p95 0.254 · p99 0.267 · 2995 op/s · total p50 0.317</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.008 · p99 0.009 · 5252 op/s · total p50 0.165</sub> | -98.2% (-0.150) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.235<br><sub>context: p90 0.309 · p95 0.337 · p99 0.418 · 18850 op/s · total p50 0.400</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 30043 op/s · total p50 0.252</sub> | -98.4% (-0.231) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.260 · p95 0.264 · p99 0.272 · 2556 op/s · total p50 0.381</sub> | 0.046<br><sub>context: p90 0.073 · p95 0.083 · p99 0.091 · 3381 op/s · total p50 0.268</sub> | -75.5% (-0.140) | 150% AND 2 ms | 🟢 |
| 8 | 0.279<br><sub>context: p90 0.371 · p95 0.409 · p99 0.485 · 17548 op/s · total p50 0.434</sub> | 0.044<br><sub>context: p90 0.073 · p95 0.077 · p99 0.087 · 24185 op/s · total p50 0.314</sub> | -84.1% (-0.235) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.182<br><sub>context: p90 0.271 · p95 0.278 · p99 0.297 · 2682 op/s · total p50 0.349</sub> | 0.061<br><sub>context: p90 0.114 · p95 0.121 · p99 0.134 · 3599 op/s · total p50 0.256</sub> | -66.5% (-0.121) | 150% AND 2 ms | 🟢 |
| 8 | 0.295<br><sub>context: p90 0.401 · p95 0.449 · p99 0.547 · 17252 op/s · total p50 0.441</sub> | 0.065<br><sub>context: p90 0.125 · p95 0.132 · p99 0.141 · 24405 op/s · total p50 0.318</sub> | -78.0% (-0.231) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.236 · p95 0.248 · p99 0.259 · 3078 op/s · total p50 0.296</sub> | 0.033<br><sub>context: p90 0.061 · p95 0.075 · p99 0.084 · 4326 op/s · total p50 0.201</sub> | -78.6% (-0.122) | 150% AND 2 ms | 🟢 |
| 8 | 0.230<br><sub>context: p90 0.312 · p95 0.339 · p99 0.417 · 20373 op/s · total p50 0.374</sub> | 0.037<br><sub>context: p90 0.067 · p95 0.070 · p99 0.078 · 28649 op/s · total p50 0.268</sub> | -84.0% (-0.193) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.000<br><sub>context: p90 1.067 · p95 1.095 · p99 1.114 · 721 op/s · total p50 1.371</sub> | 0.532<br><sub>context: p90 0.571 · p95 0.587 · p99 0.602 · 1022 op/s · total p50 0.982</sub> | -46.8% (-0.468) | 150% AND 2 ms | 🟢 |
| 8 | 1.083<br><sub>context: p90 1.332 · p95 1.418 · p99 1.756 · 4953 op/s · total p50 1.512</sub> | 0.558<br><sub>context: p90 0.644 · p95 0.680 · p99 0.741 · 6309 op/s · total p50 1.194</sub> | -48.5% (-0.525) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.014<br><sub>context: p90 1.091 · p95 1.110 · p99 1.123 · 786 op/s · total p50 1.272</sub> | 0.557<br><sub>context: p90 0.597 · p95 0.613 · p99 0.665 · 1072 op/s · total p50 0.923</sub> | -45.1% (-0.458) | 150% AND 2 ms | 🟢 |
| 8 | 1.067<br><sub>context: p90 1.243 · p95 1.308 · p99 1.501 · 5576 op/s · total p50 1.290</sub> | 0.545<br><sub>context: p90 0.620 · p95 0.641 · p99 0.688 · 8122 op/s · total p50 0.922</sub> | -48.9% (-0.522) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.201<br><sub>context: p90 0.280 · p95 0.288 · p99 0.304 · 2680 op/s · total p50 0.362</sub> | 0.071<br><sub>context: p90 0.098 · p95 0.108 · p99 0.120 · 2705 op/s · total p50 0.359</sub> | -64.7% (-0.130) | 150% AND 2 ms | 🟢 |
| 8 | 0.250<br><sub>context: p90 0.333 · p95 0.363 · p99 0.434 · 16655 op/s · total p50 0.455</sub> | 0.064<br><sub>context: p90 0.090 · p95 0.097 · p99 0.112 · 21080 op/s · total p50 0.366</sub> | -74.6% (-0.187) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.449 · p95 0.472 · p99 0.500 · 1774 op/s · total p50 0.560</sub> | 0.177<br><sub>context: p90 0.268 · p95 0.296 · p99 0.362 · 2084 op/s · total p50 0.464</sub> | -52.2% (-0.193) | 150% AND 2 ms | 🟢 |
| 8 | 0.391<br><sub>context: p90 0.506 · p95 0.544 · p99 0.604 · 13361 op/s · total p50 0.574</sub> | 0.207<br><sub>context: p90 0.338 · p95 0.375 · p99 0.435 · 15005 op/s · total p50 0.509</sub> | -47.0% (-0.184) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.245 · p95 0.254 · p99 0.265 · 2739 op/s · total p50 0.363</sub> | 0.030<br><sub>context: p90 0.060 · p95 0.063 · p99 0.070 · 3833 op/s · total p50 0.233</sub> | -83.8% (-0.156) | 150% AND 2 ms | 🟢 |
| 8 | 0.208<br><sub>context: p90 0.274 · p95 0.305 · p99 0.369 · 20416 op/s · total p50 0.368</sub> | 0.027<br><sub>context: p90 0.036 · p95 0.040 · p99 0.048 · 25214 op/s · total p50 0.302</sub> | -87.1% (-0.181) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.132<br><sub>context: p90 0.187 · p95 0.192 · p99 0.207 · 3212 op/s · total p50 0.301</sub> | 0.034<br><sub>context: p90 0.059 · p95 0.070 · p99 0.081 · 4224 op/s · total p50 0.195</sub> | -74.5% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.155<br><sub>context: p90 0.218 · p95 0.243 · p99 0.288 · 23397 op/s · total p50 0.311</sub> | 0.034<br><sub>context: p90 0.062 · p95 0.066 · p99 0.071 · 29698 op/s · total p50 0.257</sub> | -78.0% (-0.121) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.180 · p95 0.186 · p99 0.192 · 3772 op/s · total p50 0.248</sub> | 0.032<br><sub>context: p90 0.058 · p95 0.061 · p99 0.065 · 5036 op/s · total p50 0.182</sub> | -73.8% (-0.090) | 150% AND 2 ms | 🟢 |
| 8 | 0.151<br><sub>context: p90 0.199 · p95 0.222 · p99 0.262 · 25779 op/s · total p50 0.295</sub> | 0.033<br><sub>context: p90 0.063 · p95 0.065 · p99 0.073 · 30666 op/s · total p50 0.252</sub> | -77.8% (-0.117) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.754<br><sub>context: p90 0.777 · p95 0.783 · p99 0.817 · 968 op/s · total p50 1.007</sub> | 0.185<br><sub>context: p90 0.209 · p95 0.214 · p99 0.219 · 2131 op/s · total p50 0.464</sub> | -75.5% (-0.569) | 150% AND 2 ms | 🟢 |
| 8 | 0.791<br><sub>context: p90 1.172 · p95 1.342 · p99 1.515 · 6616 op/s · total p50 1.020</sub> | 0.190<br><sub>context: p90 0.219 · p95 0.227 · p99 0.245 · 17084 op/s · total p50 0.449</sub> | -76.0% (-0.601) | 150% AND 2 ms | 🟢 |

</details>
