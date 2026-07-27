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

**main vs c-engine** — 🔴 2 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.103<br><sub>context: p90 1.157 · p95 1.182 · p99 1.236 · 719 op/s · total p50 1.381</sub> | 0.479<br><sub>context: p90 0.517 · p95 0.526 · p99 0.537 · 1256 op/s · total p50 0.782</sub> | -56.6% (-0.625) | 150% AND 2 ms | 🟢 |
| 8 | 1.192<br><sub>context: p90 1.448 · p95 1.652 · p99 1.876 · 4810 op/s · total p50 1.438</sub> | 0.496<br><sub>context: p90 0.553 · p95 0.576 · p99 0.613 · 8831 op/s · total p50 0.834</sub> | -58.4% (-0.696) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.696<br><sub>context: p90 1.763 · p95 1.777 · p99 1.811 · 514 op/s · total p50 1.912</sub> | 0.525<br><sub>context: p90 0.560 · p95 0.568 · p99 0.581 · 1219 op/s · total p50 0.803</sub> | -69.1% (-1.172) | 150% AND 2 ms | 🟢 |
| 8 | 1.793<br><sub>context: p90 2.048 · p95 2.173 · p99 2.478 · 3586 op/s · total p50 2.027</sub> | 0.553<br><sub>context: p90 0.647 · p95 0.681 · p99 0.738 · 8688 op/s · total p50 0.857</sub> | -69.1% (-1.240) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.654<br><sub>context: p90 1.707 · p95 1.722 · p99 1.769 · 518 op/s · total p50 1.921</sub> | 0.905<br><sub>context: p90 0.945 · p95 0.959 · p99 0.981 · 834 op/s · total p50 1.176</sub> | -45.3% (-0.749) | 150% AND 2 ms | 🟢 |
| 8 | 1.819<br><sub>context: p90 2.780 · p95 3.082 · p99 3.489 · 3226 op/s · total p50 2.125</sub> | 1.796<br><sub>context: p90 2.792 · p95 3.089 · p99 3.537 · 3511 op/s · total p50 2.172</sub> | -1.3% (-0.023) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.235<br><sub>context: p90 2.306 · p95 2.318 · p99 2.361 · 393 op/s · total p50 2.538</sub> | 0.958<br><sub>context: p90 0.993 · p95 1.006 · p99 1.026 · 791 op/s · total p50 1.233</sub> | -57.1% (-1.277) | 150% AND 2 ms | 🟢 |
| 8 | 2.526<br><sub>context: p90 3.841 · p95 4.296 · p99 4.998 · 2507 op/s · total p50 2.798</sub> | 0.985<br><sub>context: p90 1.047 · p95 1.073 · p99 1.114 · 5435 op/s · total p50 1.344</sub> | -61.0% (-1.541) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.065<br><sub>context: p90 0.133 · p95 0.140 · p99 0.150 · 5084 op/s · total p50 0.169</sub> | 0.013<br><sub>context: p90 0.018 · p95 0.018 · p99 0.021 · 6772 op/s · total p50 0.146</sub> | -80.2% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.121<br><sub>context: p90 0.179 · p95 0.204 · p99 0.241 · 27825 op/s · total p50 0.258</sub> | 0.016<br><sub>context: p90 0.026 · p95 0.029 · p99 0.039 · 32860 op/s · total p50 0.225</sub> | -86.8% (-0.105) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.195<br><sub>context: p90 0.248 · p95 0.287 · p99 0.298 · 2672 op/s · total p50 0.362</sub> | 0.044<br><sub>context: p90 0.068 · p95 0.070 · p99 0.086 · 3554 op/s · total p50 0.215</sub> | -77.5% (-0.151) | 150% AND 2 ms | 🟢 |
| 8 | 0.273<br><sub>context: p90 0.370 · p95 0.412 · p99 0.488 · 16232 op/s · total p50 0.457</sub> | 0.051<br><sub>context: p90 0.077 · p95 0.084 · p99 0.093 · 24518 op/s · total p50 0.307</sub> | -81.3% (-0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.189<br><sub>context: p90 0.288 · p95 0.310 · p99 0.323 · 2372 op/s · total p50 0.390</sub> | 0.051<br><sub>context: p90 0.078 · p95 0.084 · p99 0.107 · 3430 op/s · total p50 0.260</sub> | -72.9% (-0.138) | 150% AND 2 ms | 🟢 |
| 8 | 0.304<br><sub>context: p90 0.408 · p95 0.453 · p99 0.542 · 15399 op/s · total p50 0.493</sub> | 0.053<br><sub>context: p90 0.083 · p95 0.089 · p99 0.102 · 22262 op/s · total p50 0.338</sub> | -82.4% (-0.250) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.256<br><sub>context: p90 0.338 · p95 0.350 · p99 0.364 · 1960 op/s · total p50 0.477</sub> | 0.101<br><sub>context: p90 0.142 · p95 0.150 · p99 0.166 · 2566 op/s · total p50 0.351</sub> | -60.6% (-0.155) | 150% AND 2 ms | 🟢 |
| 8 | 0.321<br><sub>context: p90 0.416 · p95 0.446 · p99 0.546 · 14312 op/s · total p50 0.536</sub> | 0.108<br><sub>context: p90 0.146 · p95 0.160 · p99 0.179 · 16542 op/s · total p50 0.447</sub> | -66.2% (-0.213) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.279<br><sub>context: p90 0.353 · p95 0.363 · p99 0.375 · 2074 op/s · total p50 0.466</sub> | 0.115<br><sub>context: p90 0.157 · p95 0.174 · p99 0.192 · 2149 op/s · total p50 0.430</sub> | -58.6% (-0.163) | 150% AND 2 ms | 🟢 |
| 8 | 0.353<br><sub>context: p90 0.460 · p95 0.497 · p99 0.585 · 13109 op/s · total p50 0.575</sub> | 0.114<br><sub>context: p90 0.152 · p95 0.164 · p99 0.187 · 15060 op/s · total p50 0.506</sub> | -67.7% (-0.239) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.305<br><sub>context: p90 0.411 · p95 0.428 · p99 0.499 · 1726 op/s · total p50 0.563</sub> | 0.156<br><sub>context: p90 0.201 · p95 0.213 · p99 0.223 · 1677 op/s · total p50 0.567</sub> | -48.7% (-0.148) | 150% AND 2 ms | 🟢 |
| 8 | 0.403<br><sub>context: p90 0.523 · p95 0.565 · p99 0.649 · 10224 op/s · total p50 0.749</sub> | 0.154<br><sub>context: p90 0.200 · p95 0.216 · p99 0.247 · 11540 op/s · total p50 0.648</sub> | -61.7% (-0.249) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.361<br><sub>context: p90 0.443 · p95 0.458 · p99 0.497 · 1507 op/s · total p50 0.655</sub> | 0.149<br><sub>context: p90 0.210 · p95 0.222 · p99 0.239 · 1670 op/s · total p50 0.558</sub> | -58.7% (-0.212) | 150% AND 2 ms | 🟢 |
| 8 | 0.442<br><sub>context: p90 0.548 · p95 0.584 · p99 0.647 · 9847 op/s · total p50 0.782</sub> | 0.163<br><sub>context: p90 0.211 · p95 0.224 · p99 0.252 · 11293 op/s · total p50 0.675</sub> | -63.1% (-0.279) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.589<br><sub>context: p90 0.728 · p95 0.778 · p99 0.854 · 746 op/s · total p50 1.317</sub> | 0.263<br><sub>context: p90 0.328 · p95 0.342 · p99 0.373 · 981 op/s · total p50 1.008</sub> | -55.3% (-0.326) | 150% AND 2 ms | 🟢 |
| 8 | 0.672<br><sub>context: p90 0.875 · p95 0.933 · p99 1.067 · 5186 op/s · total p50 1.490</sub> | 0.288<br><sub>context: p90 0.377 · p95 0.404 · p99 0.440 · 6305 op/s · total p50 1.207</sub> | -57.1% (-0.383) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.625<br><sub>context: p90 0.782 · p95 0.840 · p99 0.893 · 785 op/s · total p50 1.246</sub> | 0.263<br><sub>context: p90 0.338 · p95 0.363 · p99 0.417 · 994 op/s · total p50 0.987</sub> | -58.0% (-0.363) | 150% AND 2 ms | 🟢 |
| 8 | 0.733<br><sub>context: p90 0.949 · p95 1.039 · p99 1.204 · 4966 op/s · total p50 1.554</sub> | 0.300<br><sub>context: p90 0.392 · p95 0.418 · p99 0.465 · 6119 op/s · total p50 1.247</sub> | -59.1% (-0.433) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.462<br><sub>context: p90 0.548 · p95 0.596 · p99 0.669 · 1499 op/s · total p50 0.647</sub> | 0.309<br><sub>context: p90 0.477 · p95 0.514 · p99 0.578 · 1587 op/s · total p50 0.616</sub> | -33.0% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.588<br><sub>context: p90 0.755 · p95 0.828 · p99 0.937 · 8876 op/s · total p50 0.849</sub> | 0.499<br><sub>context: p90 0.785 · p95 0.861 · p99 1.016 · 9105 op/s · total p50 0.836</sub> | -15.2% (-0.089) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.338<br><sub>context: p90 0.377 · p95 0.386 · p99 0.408 · 1717 op/s · total p50 0.576</sub> | 0.080<br><sub>context: p90 0.116 · p95 0.131 · p99 0.151 · 2998 op/s · total p50 0.301</sub> | -76.4% (-0.258) | 150% AND 2 ms | 🟢 |
| 8 | 0.373<br><sub>context: p90 0.495 · p95 0.545 · p99 0.657 · 13947 op/s · total p50 0.549</sub> | 0.089<br><sub>context: p90 0.123 · p95 0.134 · p99 0.157 · 19039 op/s · total p50 0.400</sub> | -76.2% (-0.284) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.137<br><sub>context: p90 0.212 · p95 0.222 · p99 0.236 · 3259 op/s · total p50 0.274</sub> | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 7439 op/s · total p50 0.133</sub> | -98.7% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.203<br><sub>context: p90 0.277 · p95 0.305 · p99 0.371 · 21886 op/s · total p50 0.343</sub> | 0.002<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 31874 op/s · total p50 0.241</sub> | -98.8% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.162 · p95 0.170 · p99 0.179 · 3839 op/s · total p50 0.224</sub> | 0.001<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 7821 op/s · total p50 0.120</sub> | -98.5% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.152<br><sub>context: p90 0.222 · p95 0.247 · p99 0.299 · 25529 op/s · total p50 0.292</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 35387 op/s · total p50 0.214</sub> | -98.7% (-0.150) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.327<br><sub>context: p90 0.412 · p95 0.425 · p99 0.440 · 1737 op/s · total p50 0.546</sub> | 0.070<br><sub>context: p90 0.095 · p95 0.100 · p99 0.113 · 2700 op/s · total p50 0.359</sub> | -78.6% (-0.257) | 150% AND 2 ms | 🟢 |
| 8 | 0.426<br><sub>context: p90 0.557 · p95 0.621 · p99 0.730 · 11995 op/s · total p50 0.636</sub> | 0.078<br><sub>context: p90 0.108 · p95 0.115 · p99 0.127 · 16844 op/s · total p50 0.452</sub> | -81.7% (-0.348) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.623<br><sub>context: p90 0.864 · p95 0.946 · p99 1.118 · 1245 op/s · total p50 0.799</sub> | 0.984<br><sub>context: p90 1.614 · p95 1.810 · p99 2.076 · 732 op/s · total p50 1.318</sub> | +58.0% (+0.361) | 150% AND 2 ms | 🟢 |
| 8 | 0.728<br><sub>context: p90 1.087 · p95 1.208 · p99 1.451 · 7948 op/s · total p50 0.944</sub> | 2.820<br><sub>context: p90 4.751 · p95 5.374 · p99 6.195 · 2325 op/s · total p50 3.255</sub> | +287.2% (+2.092) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.044<br><sub>context: p90 3.251 · p95 3.719 · p99 3.992 · 409 op/s · total p50 2.291</sub> | 4.005<br><sub>context: p90 6.239 · p95 6.906 · p99 7.574 · 217 op/s · total p50 4.428</sub> | +95.9% (+1.961) | 150% AND 2 ms | 🟢 |
| 8 | 2.192<br><sub>context: p90 3.699 · p95 4.213 · p99 5.139 · 2970 op/s · total p50 2.461</sub> | 12.428<br><sub>context: p90 18.739 · p95 20.598 · p99 22.470 · 607 op/s · total p50 12.876</sub> | +466.9% (+10.236) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.101<br><sub>context: p90 0.174 · p95 0.204 · p99 0.212 · 3811 op/s · total p50 0.233</sub> | 0.016<br><sub>context: p90 0.021 · p95 0.024 · p99 0.030 · 4843 op/s · total p50 0.189</sub> | -84.0% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.261 · p95 0.292 · p99 0.370 · 20676 op/s · total p50 0.360</sub> | 0.018<br><sub>context: p90 0.024 · p95 0.026 · p99 0.035 · 28934 op/s · total p50 0.261</sub> | -90.4% (-0.168) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.115<br><sub>context: p90 0.150 · p95 0.154 · p99 0.212 · 3726 op/s · total p50 0.255</sub> | 0.014<br><sub>context: p90 0.018 · p95 0.020 · p99 0.024 · 5245 op/s · total p50 0.172</sub> | -87.4% (-0.100) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.251 · p95 0.282 · p99 0.351 · 21701 op/s · total p50 0.349</sub> | 0.017<br><sub>context: p90 0.023 · p95 0.026 · p99 0.032 · 28622 op/s · total p50 0.263</sub> | -90.8% (-0.169) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.212 · p95 0.222 · p99 0.229 · 2900 op/s · total p50 0.315</sub> | 0.009<br><sub>context: p90 0.013 · p95 0.015 · p99 0.017 · 3870 op/s · total p50 0.240</sub> | -93.1% (-0.121) | 150% AND 2 ms | 🟢 |
| 8 | 0.196<br><sub>context: p90 0.260 · p95 0.284 · p99 0.335 · 15963 op/s · total p50 0.478</sub> | 0.011<br><sub>context: p90 0.016 · p95 0.018 · p99 0.021 · 21756 op/s · total p50 0.348</sub> | -94.5% (-0.185) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.171 · p95 0.182 · p99 0.192 · 4108 op/s · total p50 0.228</sub> | 0.006<br><sub>context: p90 0.015 · p95 0.016 · p99 0.018 · 5908 op/s · total p50 0.143</sub> | -94.6% (-0.100) | 150% AND 2 ms | 🟢 |
| 8 | 0.161<br><sub>context: p90 0.228 · p95 0.254 · p99 0.316 · 25360 op/s · total p50 0.299</sub> | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.012 · 33251 op/s · total p50 0.229</sub> | -96.4% (-0.155) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.294 · p95 0.303 · p99 0.313 · 2309 op/s · total p50 0.388</sub> | 0.125<br><sub>context: p90 0.160 · p95 0.175 · p99 0.194 · 2894 op/s · total p50 0.336</sub> | -43.8% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.276<br><sub>context: p90 0.357 · p95 0.383 · p99 0.456 · 16820 op/s · total p50 0.449</sub> | 0.130<br><sub>context: p90 0.168 · p95 0.180 · p99 0.206 · 20013 op/s · total p50 0.384</sub> | -53.0% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.179<br><sub>context: p90 0.246 · p95 0.255 · p99 0.264 · 2795 op/s · total p50 0.327</sub> | 0.063<br><sub>context: p90 0.091 · p95 0.108 · p99 0.114 · 3901 op/s · total p50 0.236</sub> | -64.5% (-0.115) | 150% AND 2 ms | 🟢 |
| 8 | 0.229<br><sub>context: p90 0.311 · p95 0.342 · p99 0.423 · 19470 op/s · total p50 0.388</sub> | 0.066<br><sub>context: p90 0.095 · p95 0.103 · p99 0.122 · 24661 op/s · total p50 0.310</sub> | -71.0% (-0.163) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.211<br><sub>context: p90 0.308 · p95 0.319 · p99 0.337 · 2284 op/s · total p50 0.422</sub> | 0.076<br><sub>context: p90 0.108 · p95 0.113 · p99 0.132 · 3346 op/s · total p50 0.279</sub> | -64.1% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.303<br><sub>context: p90 0.390 · p95 0.422 · p99 0.502 · 14928 op/s · total p50 0.508</sub> | 0.101<br><sub>context: p90 0.135 · p95 0.147 · p99 0.171 · 17337 op/s · total p50 0.446</sub> | -66.7% (-0.202) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.215<br><sub>context: p90 0.298 · p95 0.321 · p99 0.335 · 2092 op/s · total p50 0.456</sub> | 0.085<br><sub>context: p90 0.109 · p95 0.117 · p99 0.124 · 2363 op/s · total p50 0.418</sub> | -60.6% (-0.130) | 150% AND 2 ms | 🟢 |
| 8 | 0.308<br><sub>context: p90 0.392 · p95 0.422 · p99 0.497 · 11374 op/s · total p50 0.654</sub> | 0.108<br><sub>context: p90 0.145 · p95 0.156 · p99 0.179 · 13214 op/s · total p50 0.582</sub> | -65.0% (-0.200) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.260<br><sub>context: p90 0.360 · p95 0.376 · p99 0.404 · 1615 op/s · total p50 0.596</sub> | 0.094<br><sub>context: p90 0.120 · p95 0.127 · p99 0.141 · 2230 op/s · total p50 0.439</sub> | -63.8% (-0.166) | 150% AND 2 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.424 · p95 0.454 · p99 0.537 · 11579 op/s · total p50 0.656</sub> | 0.115<br><sub>context: p90 0.152 · p95 0.163 · p99 0.183 · 12534 op/s · total p50 0.616</sub> | -65.6% (-0.220) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.231<br><sub>context: p90 0.343 · p95 0.357 · p99 0.363 · 2123 op/s · total p50 0.439</sub> | 0.082<br><sub>context: p90 0.113 · p95 0.129 · p99 0.139 · 2904 op/s · total p50 0.329</sub> | -64.5% (-0.149) | 150% AND 2 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.431 · p95 0.471 · p99 0.553 · 14075 op/s · total p50 0.547</sub> | 0.108<br><sub>context: p90 0.137 · p95 0.147 · p99 0.171 · 15217 op/s · total p50 0.511</sub> | -67.9% (-0.227) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.306<br><sub>context: p90 0.362 · p95 0.377 · p99 0.394 · 1831 op/s · total p50 0.517</sub> | 0.068<br><sub>context: p90 0.106 · p95 0.115 · p99 0.143 · 3261 op/s · total p50 0.280</sub> | -77.7% (-0.238) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.478 · p95 0.523 · p99 0.604 · 12933 op/s · total p50 0.581</sub> | 0.084<br><sub>context: p90 0.116 · p95 0.127 · p99 0.145 · 19575 op/s · total p50 0.388</sub> | -76.5% (-0.274) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.890<br><sub>context: p90 6.044 · p95 6.095 · p99 6.187 · 59 op/s · total p50 16.803</sub> | 2.537<br><sub>context: p90 2.583 · p95 2.602 · p99 2.631 · 73 op/s · total p50 13.567</sub> | -56.9% (-3.354) | 150% AND 2 ms | 🟢 |
| 8 | 7.553<br><sub>context: p90 9.255 · p95 9.671 · p99 10.311 · 349 op/s · total p50 21.825</sub> | 3.184<br><sub>context: p90 3.469 · p95 3.509 · p99 3.788 · 455 op/s · total p50 16.560</sub> | -57.9% (-4.370) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.430 · p95 0.444 · p99 0.453 · 1809 op/s · total p50 0.541</sub> | 0.097<br><sub>context: p90 0.133 · p95 0.141 · p99 0.150 · 2828 op/s · total p50 0.338</sub> | -70.7% (-0.233) | 150% AND 2 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.579 · p95 0.635 · p99 0.750 · 11931 op/s · total p50 0.637</sub> | 0.114<br><sub>context: p90 0.147 · p95 0.155 · p99 0.173 · 16196 op/s · total p50 0.472</sub> | -74.4% (-0.332) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.554<br><sub>context: p90 0.651 · p95 0.679 · p99 0.737 · 639 op/s · total p50 1.560</sub> | 0.222<br><sub>context: p90 0.296 · p95 0.309 · p99 0.354 · 856 op/s · total p50 1.141</sub> | -59.9% (-0.332) | 150% AND 2 ms | 🟢 |
| 8 | 0.626<br><sub>context: p90 0.783 · p95 0.828 · p99 0.911 · 4579 op/s · total p50 1.708</sub> | 0.270<br><sub>context: p90 0.356 · p95 0.375 · p99 0.416 · 5302 op/s · total p50 1.457</sub> | -56.9% (-0.356) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.349 · p95 0.364 · p99 0.370 · 1987 op/s · total p50 0.479</sub> | 0.090<br><sub>context: p90 0.124 · p95 0.131 · p99 0.152 · 2916 op/s · total p50 0.336</sub> | -66.8% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.326<br><sub>context: p90 0.419 · p95 0.457 · p99 0.543 · 13785 op/s · total p50 0.549</sub> | 0.107<br><sub>context: p90 0.143 · p95 0.150 · p99 0.169 · 16564 op/s · total p50 0.462</sub> | -67.0% (-0.218) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.717<br><sub>context: p90 2.185 · p95 2.252 · p99 2.302 · 527 op/s · total p50 1.928</sub> | 0.109<br><sub>context: p90 0.157 · p95 0.166 · p99 0.192 · 2962 op/s · total p50 0.337</sub> | -93.7% (-1.608) | 150% AND 2 ms | 🟢 |
| 8 | 1.806<br><sub>context: p90 2.299 · p95 2.398 · p99 2.567 · 3639 op/s · total p50 2.128</sub> | 0.116<br><sub>context: p90 0.165 · p95 0.180 · p99 0.211 · 19371 op/s · total p50 0.398</sub> | -93.6% (-1.690) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.765<br><sub>context: p90 2.243 · p95 2.317 · p99 2.446 · 492 op/s · total p50 2.068</sub> | 0.109<br><sub>context: p90 0.163 · p95 0.176 · p99 0.206 · 2785 op/s · total p50 0.346</sub> | -93.8% (-1.657) | 150% AND 2 ms | 🟢 |
| 8 | 1.842<br><sub>context: p90 2.360 · p95 2.472 · p99 2.740 · 3651 op/s · total p50 2.122</sub> | 0.119<br><sub>context: p90 0.176 · p95 0.195 · p99 0.222 · 18666 op/s · total p50 0.410</sub> | -93.5% (-1.723) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.191 · p95 0.197 · p99 0.213 · 3016 op/s · total p50 0.304</sub> | 0.032<br><sub>context: p90 0.056 · p95 0.061 · p99 0.079 · 4413 op/s · total p50 0.192</sub> | -75.3% (-0.098) | 150% AND 2 ms | 🟢 |
| 8 | 0.156<br><sub>context: p90 0.215 · p95 0.240 · p99 0.287 · 23517 op/s · total p50 0.317</sub> | 0.036<br><sub>context: p90 0.064 · p95 0.070 · p99 0.082 · 23678 op/s · total p50 0.285</sub> | -77.1% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.182<br><sub>context: p90 0.254 · p95 0.261 · p99 0.271 · 2800 op/s · total p50 0.339</sub> | 0.004<br><sub>context: p90 0.004 · p95 0.005 · p99 0.005 · 5987 op/s · total p50 0.160</sub> | -98.0% (-0.178) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.240<br><sub>context: p90 0.312 · p95 0.336 · p99 0.402 · 18362 op/s · total p50 0.414</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 28689 op/s · total p50 0.270</sub> | -98.4% (-0.237) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.161<br><sub>context: p90 0.206 · p95 0.213 · p99 0.234 · 3098 op/s · total p50 0.311</sub> | 0.044<br><sub>context: p90 0.070 · p95 0.076 · p99 0.091 · 3741 op/s · total p50 0.245</sub> | -72.9% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.295<br><sub>context: p90 0.402 · p95 0.439 · p99 0.544 · 16395 op/s · total p50 0.461</sub> | 0.045<br><sub>context: p90 0.074 · p95 0.077 · p99 0.083 · 23414 op/s · total p50 0.325</sub> | -84.8% (-0.250) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.229 · p95 0.262 · p99 0.289 · 2867 op/s · total p50 0.328</sub> | 0.059<br><sub>context: p90 0.112 · p95 0.117 · p99 0.136 · 3438 op/s · total p50 0.256</sub> | -66.8% (-0.119) | 150% AND 2 ms | 🟢 |
| 8 | 0.301<br><sub>context: p90 0.404 · p95 0.453 · p99 0.536 · 16330 op/s · total p50 0.463</sub> | 0.065<br><sub>context: p90 0.125 · p95 0.132 · p99 0.141 · 22962 op/s · total p50 0.328</sub> | -78.3% (-0.236) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.167<br><sub>context: p90 0.254 · p95 0.264 · p99 0.274 · 2590 op/s · total p50 0.374</sub> | 0.034<br><sub>context: p90 0.060 · p95 0.066 · p99 0.068 · 4779 op/s · total p50 0.203</sub> | -79.5% (-0.133) | 150% AND 2 ms | 🟢 |
| 8 | 0.242<br><sub>context: p90 0.323 · p95 0.358 · p99 0.453 · 19240 op/s · total p50 0.393</sub> | 0.038<br><sub>context: p90 0.067 · p95 0.071 · p99 0.079 · 26069 op/s · total p50 0.293</sub> | -84.3% (-0.204) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.978<br><sub>context: p90 1.068 · p95 1.089 · p99 1.099 · 715 op/s · total p50 1.387</sub> | 0.491<br><sub>context: p90 0.530 · p95 0.549 · p99 0.574 · 1108 op/s · total p50 0.876</sub> | -49.8% (-0.487) | 150% AND 2 ms | 🟢 |
| 8 | 1.095<br><sub>context: p90 1.327 · p95 1.396 · p99 1.709 · 4928 op/s · total p50 1.528</sub> | 0.566<br><sub>context: p90 0.671 · p95 0.711 · p99 0.812 · 6826 op/s · total p50 1.122</sub> | -48.3% (-0.529) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.005<br><sub>context: p90 1.088 · p95 1.109 · p99 1.145 · 788 op/s · total p50 1.238</sub> | 0.491<br><sub>context: p90 0.526 · p95 0.535 · p99 0.553 · 1313 op/s · total p50 0.749</sub> | -51.2% (-0.514) | 150% AND 2 ms | 🟢 |
| 8 | 1.082<br><sub>context: p90 1.251 · p95 1.312 · p99 1.494 · 5408 op/s · total p50 1.334</sub> | 0.539<br><sub>context: p90 0.630 · p95 0.664 · p99 0.710 · 7828 op/s · total p50 0.973</sub> | -50.2% (-0.543) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.217<br><sub>context: p90 0.290 · p95 0.299 · p99 0.314 · 2351 op/s · total p50 0.407</sub> | 0.058<br><sub>context: p90 0.084 · p95 0.088 · p99 0.097 · 3559 op/s · total p50 0.273</sub> | -73.2% (-0.159) | 150% AND 2 ms | 🟢 |
| 8 | 0.264<br><sub>context: p90 0.348 · p95 0.375 · p99 0.450 · 15903 op/s · total p50 0.474</sub> | 0.065<br><sub>context: p90 0.093 · p95 0.100 · p99 0.113 · 20878 op/s · total p50 0.368</sub> | -75.4% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.343<br><sub>context: p90 0.439 · p95 0.464 · p99 0.550 · 1767 op/s · total p50 0.539</sub> | 0.156<br><sub>context: p90 0.237 · p95 0.258 · p99 0.282 · 2398 op/s · total p50 0.409</sub> | -54.5% (-0.187) | 150% AND 2 ms | 🟢 |
| 8 | 0.412<br><sub>context: p90 0.537 · p95 0.570 · p99 0.640 · 12358 op/s · total p50 0.613</sub> | 0.200<br><sub>context: p90 0.314 · p95 0.350 · p99 0.400 · 15259 op/s · total p50 0.495</sub> | -51.5% (-0.212) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.146<br><sub>context: p90 0.237 · p95 0.251 · p99 0.261 · 3135 op/s · total p50 0.288</sub> | 0.024<br><sub>context: p90 0.031 · p95 0.035 · p99 0.038 · 4484 op/s · total p50 0.210</sub> | -83.8% (-0.122) | 150% AND 2 ms | 🟢 |
| 8 | 0.221<br><sub>context: p90 0.295 · p95 0.323 · p99 0.387 · 18635 op/s · total p50 0.402</sub> | 0.026<br><sub>context: p90 0.036 · p95 0.040 · p99 0.050 · 25315 op/s · total p50 0.300</sub> | -88.1% (-0.195) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.187 · p95 0.194 · p99 0.212 · 3708 op/s · total p50 0.248</sub> | 0.031<br><sub>context: p90 0.056 · p95 0.061 · p99 0.063 · 5100 op/s · total p50 0.190</sub> | -74.5% (-0.091) | 150% AND 2 ms | 🟢 |
| 8 | 0.163<br><sub>context: p90 0.234 · p95 0.260 · p99 0.326 · 23138 op/s · total p50 0.324</sub> | 0.034<br><sub>context: p90 0.062 · p95 0.066 · p99 0.071 · 28164 op/s · total p50 0.269</sub> | -78.8% (-0.128) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.116<br><sub>context: p90 0.190 · p95 0.199 · p99 0.209 · 3623 op/s · total p50 0.247</sub> | 0.032<br><sub>context: p90 0.059 · p95 0.062 · p99 0.065 · 5358 op/s · total p50 0.183</sub> | -72.3% (-0.084) | 150% AND 2 ms | 🟢 |
| 8 | 0.158<br><sub>context: p90 0.225 · p95 0.251 · p99 0.305 · 23381 op/s · total p50 0.321</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.066 · p99 0.072 · 29714 op/s · total p50 0.258</sub> | -78.5% (-0.124) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.759<br><sub>context: p90 0.797 · p95 0.811 · p99 0.850 · 988 op/s · total p50 0.997</sub> | 0.169<br><sub>context: p90 0.177 · p95 0.181 · p99 0.194 · 2621 op/s · total p50 0.368</sub> | -77.7% (-0.590) | 150% AND 2 ms | 🟢 |
| 8 | 0.803<br><sub>context: p90 1.217 · p95 1.372 · p99 1.612 · 6059 op/s · total p50 1.083</sub> | 0.189<br><sub>context: p90 0.218 · p95 0.224 · p99 0.241 · 17544 op/s · total p50 0.440</sub> | -76.4% (-0.614) | 150% AND 2 ms | 🟢 |

</details>
