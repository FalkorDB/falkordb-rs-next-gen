### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
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

**pr vs c-engine** — 🔴 2 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.103<br><sub>context: p90 1.157 · p95 1.182 · p99 1.236 · 719 op/s · total p50 1.381</sub> | 0.468<br><sub>context: p90 0.497 · p95 0.518 · p99 0.532 · 1418 op/s · total p50 0.685</sub> | -57.5% (-0.635) | 150% AND 2 ms | 🟢 |
| 8 | 1.192<br><sub>context: p90 1.448 · p95 1.652 · p99 1.876 · 4810 op/s · total p50 1.438</sub> | 0.502<br><sub>context: p90 0.567 · p95 0.592 · p99 0.624 · 9144 op/s · total p50 0.819</sub> | -57.9% (-0.690) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.696<br><sub>context: p90 1.763 · p95 1.777 · p99 1.811 · 514 op/s · total p50 1.912</sub> | 0.520<br><sub>context: p90 0.556 · p95 0.570 · p99 0.587 · 1290 op/s · total p50 0.758</sub> | -69.4% (-1.177) | 150% AND 2 ms | 🟢 |
| 8 | 1.793<br><sub>context: p90 2.048 · p95 2.173 · p99 2.478 · 3586 op/s · total p50 2.027</sub> | 0.559<br><sub>context: p90 0.653 · p95 0.682 · p99 0.730 · 8834 op/s · total p50 0.857</sub> | -68.8% (-1.234) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.654<br><sub>context: p90 1.707 · p95 1.722 · p99 1.769 · 518 op/s · total p50 1.921</sub> | 0.916<br><sub>context: p90 0.963 · p95 0.976 · p99 0.996 · 816 op/s · total p50 1.204</sub> | -44.6% (-0.738) | 150% AND 2 ms | 🟢 |
| 8 | 1.819<br><sub>context: p90 2.780 · p95 3.082 · p99 3.489 · 3226 op/s · total p50 2.125</sub> | 1.746<br><sub>context: p90 2.809 · p95 3.118 · p99 3.619 · 3411 op/s · total p50 2.172</sub> | -4.0% (-0.073) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.235<br><sub>context: p90 2.306 · p95 2.318 · p99 2.361 · 393 op/s · total p50 2.538</sub> | 0.955<br><sub>context: p90 0.975 · p95 0.985 · p99 0.991 · 819 op/s · total p50 1.209</sub> | -57.3% (-1.280) | 150% AND 2 ms | 🟢 |
| 8 | 2.526<br><sub>context: p90 3.841 · p95 4.296 · p99 4.998 · 2507 op/s · total p50 2.798</sub> | 0.985<br><sub>context: p90 1.087 · p95 1.127 · p99 1.177 · 5474 op/s · total p50 1.349</sub> | -61.0% (-1.541) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.065<br><sub>context: p90 0.133 · p95 0.140 · p99 0.150 · 5084 op/s · total p50 0.169</sub> | 0.014<br><sub>context: p90 0.017 · p95 0.018 · p99 0.022 · 6143 op/s · total p50 0.159</sub> | -78.7% (-0.051) | 150% AND 2 ms | 🟢 |
| 8 | 0.121<br><sub>context: p90 0.179 · p95 0.204 · p99 0.241 · 27825 op/s · total p50 0.258</sub> | 0.018<br><sub>context: p90 0.028 · p95 0.031 · p99 0.039 · 32242 op/s · total p50 0.238</sub> | -85.5% (-0.104) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.195<br><sub>context: p90 0.248 · p95 0.287 · p99 0.298 · 2672 op/s · total p50 0.362</sub> | 0.045<br><sub>context: p90 0.072 · p95 0.075 · p99 0.081 · 4572 op/s · total p50 0.213</sub> | -76.6% (-0.149) | 150% AND 2 ms | 🟢 |
| 8 | 0.273<br><sub>context: p90 0.370 · p95 0.412 · p99 0.488 · 16232 op/s · total p50 0.457</sub> | 0.051<br><sub>context: p90 0.076 · p95 0.083 · p99 0.096 · 23067 op/s · total p50 0.337</sub> | -81.5% (-0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.189<br><sub>context: p90 0.288 · p95 0.310 · p99 0.323 · 2372 op/s · total p50 0.390</sub> | 0.047<br><sub>context: p90 0.075 · p95 0.080 · p99 0.089 · 3876 op/s · total p50 0.239</sub> | -75.2% (-0.143) | 150% AND 2 ms | 🟢 |
| 8 | 0.304<br><sub>context: p90 0.408 · p95 0.453 · p99 0.542 · 15399 op/s · total p50 0.493</sub> | 0.052<br><sub>context: p90 0.079 · p95 0.086 · p99 0.096 · 23306 op/s · total p50 0.328</sub> | -82.8% (-0.252) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.256<br><sub>context: p90 0.338 · p95 0.350 · p99 0.364 · 1960 op/s · total p50 0.477</sub> | 0.111<br><sub>context: p90 0.150 · p95 0.162 · p99 0.175 · 2420 op/s · total p50 0.407</sub> | -56.8% (-0.146) | 150% AND 2 ms | 🟢 |
| 8 | 0.321<br><sub>context: p90 0.416 · p95 0.446 · p99 0.546 · 14312 op/s · total p50 0.536</sub> | 0.106<br><sub>context: p90 0.142 · p95 0.153 · p99 0.177 · 16421 op/s · total p50 0.466</sub> | -67.0% (-0.215) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.279<br><sub>context: p90 0.353 · p95 0.363 · p99 0.375 · 2074 op/s · total p50 0.466</sub> | 0.110<br><sub>context: p90 0.157 · p95 0.169 · p99 0.193 · 2263 op/s · total p50 0.428</sub> | -60.4% (-0.168) | 150% AND 2 ms | 🟢 |
| 8 | 0.353<br><sub>context: p90 0.460 · p95 0.497 · p99 0.585 · 13109 op/s · total p50 0.575</sub> | 0.111<br><sub>context: p90 0.147 · p95 0.160 · p99 0.184 · 15630 op/s · total p50 0.493</sub> | -68.4% (-0.242) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.305<br><sub>context: p90 0.411 · p95 0.428 · p99 0.499 · 1726 op/s · total p50 0.563</sub> | 0.144<br><sub>context: p90 0.184 · p95 0.198 · p99 0.214 · 1803 op/s · total p50 0.541</sub> | -52.8% (-0.161) | 150% AND 2 ms | 🟢 |
| 8 | 0.403<br><sub>context: p90 0.523 · p95 0.565 · p99 0.649 · 10224 op/s · total p50 0.749</sub> | 0.152<br><sub>context: p90 0.198 · p95 0.211 · p99 0.239 · 12222 op/s · total p50 0.629</sub> | -62.3% (-0.251) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.361<br><sub>context: p90 0.443 · p95 0.458 · p99 0.497 · 1507 op/s · total p50 0.655</sub> | 0.141<br><sub>context: p90 0.194 · p95 0.214 · p99 0.236 · 1896 op/s · total p50 0.517</sub> | -61.0% (-0.220) | 150% AND 2 ms | 🟢 |
| 8 | 0.442<br><sub>context: p90 0.548 · p95 0.584 · p99 0.647 · 9847 op/s · total p50 0.782</sub> | 0.161<br><sub>context: p90 0.207 · p95 0.220 · p99 0.247 · 11161 op/s · total p50 0.677</sub> | -63.6% (-0.281) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.589<br><sub>context: p90 0.728 · p95 0.778 · p99 0.854 · 746 op/s · total p50 1.317</sub> | 0.248<br><sub>context: p90 0.310 · p95 0.342 · p99 0.378 · 1010 op/s · total p50 0.962</sub> | -58.0% (-0.342) | 150% AND 2 ms | 🟢 |
| 8 | 0.672<br><sub>context: p90 0.875 · p95 0.933 · p99 1.067 · 5186 op/s · total p50 1.490</sub> | 0.287<br><sub>context: p90 0.375 · p95 0.398 · p99 0.435 · 6311 op/s · total p50 1.213</sub> | -57.3% (-0.385) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.625<br><sub>context: p90 0.782 · p95 0.840 · p99 0.893 · 785 op/s · total p50 1.246</sub> | 0.276<br><sub>context: p90 0.357 · p95 0.378 · p99 0.400 · 901 op/s · total p50 1.055</sub> | -55.8% (-0.349) | 150% AND 2 ms | 🟢 |
| 8 | 0.733<br><sub>context: p90 0.949 · p95 1.039 · p99 1.204 · 4966 op/s · total p50 1.554</sub> | 0.299<br><sub>context: p90 0.399 · p95 0.422 · p99 0.469 · 6205 op/s · total p50 1.234</sub> | -59.2% (-0.434) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.462<br><sub>context: p90 0.548 · p95 0.596 · p99 0.669 · 1499 op/s · total p50 0.647</sub> | 0.347<br><sub>context: p90 0.484 · p95 0.545 · p99 0.619 · 1309 op/s · total p50 0.724</sub> | -24.8% (-0.114) | 150% AND 2 ms | 🟢 |
| 8 | 0.588<br><sub>context: p90 0.755 · p95 0.828 · p99 0.937 · 8876 op/s · total p50 0.849</sub> | 0.485<br><sub>context: p90 0.761 · p95 0.849 · p99 0.972 · 8848 op/s · total p50 0.850</sub> | -17.5% (-0.103) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.338<br><sub>context: p90 0.377 · p95 0.386 · p99 0.408 · 1717 op/s · total p50 0.576</sub> | 0.088<br><sub>context: p90 0.127 · p95 0.142 · p99 0.155 · 2471 op/s · total p50 0.333</sub> | -73.8% (-0.249) | 150% AND 2 ms | 🟢 |
| 8 | 0.373<br><sub>context: p90 0.495 · p95 0.545 · p99 0.657 · 13947 op/s · total p50 0.549</sub> | 0.090<br><sub>context: p90 0.128 · p95 0.140 · p99 0.159 · 17940 op/s · total p50 0.418</sub> | -75.8% (-0.282) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.137<br><sub>context: p90 0.212 · p95 0.222 · p99 0.236 · 3259 op/s · total p50 0.274</sub> | 0.002<br><sub>context: p90 0.006 · p95 0.007 · p99 0.007 · 5042 op/s · total p50 0.165</sub> | -98.2% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.203<br><sub>context: p90 0.277 · p95 0.305 · p99 0.371 · 21886 op/s · total p50 0.343</sub> | 0.002<br><sub>context: p90 0.004 · p95 0.004 · p99 0.006 · 31302 op/s · total p50 0.242</sub> | -98.8% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.162 · p95 0.170 · p99 0.179 · 3839 op/s · total p50 0.224</sub> | 0.002<br><sub>context: p90 0.006 · p95 0.007 · p99 0.008 · 5800 op/s · total p50 0.155</sub> | -98.1% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.152<br><sub>context: p90 0.222 · p95 0.247 · p99 0.299 · 25529 op/s · total p50 0.292</sub> | 0.002<br><sub>context: p90 0.004 · p95 0.006 · p99 0.007 · 30887 op/s · total p50 0.224</sub> | -98.6% (-0.150) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.327<br><sub>context: p90 0.412 · p95 0.425 · p99 0.440 · 1737 op/s · total p50 0.546</sub> | 0.068<br><sub>context: p90 0.101 · p95 0.114 · p99 0.123 · 2436 op/s · total p50 0.357</sub> | -79.4% (-0.260) | 150% AND 2 ms | 🟢 |
| 8 | 0.426<br><sub>context: p90 0.557 · p95 0.621 · p99 0.730 · 11995 op/s · total p50 0.636</sub> | 0.077<br><sub>context: p90 0.110 · p95 0.119 · p99 0.134 · 16356 op/s · total p50 0.461</sub> | -81.9% (-0.349) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.623<br><sub>context: p90 0.864 · p95 0.946 · p99 1.118 · 1245 op/s · total p50 0.799</sub> | 1.061<br><sub>context: p90 1.605 · p95 1.837 · p99 2.181 · 643 op/s · total p50 1.501</sub> | +70.3% (+0.438) | 150% AND 2 ms | 🟢 |
| 8 | 0.728<br><sub>context: p90 1.087 · p95 1.208 · p99 1.451 · 7948 op/s · total p50 0.944</sub> | 2.828<br><sub>context: p90 4.651 · p95 5.196 · p99 6.007 · 2347 op/s · total p50 3.269</sub> | +288.3% (+2.100) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.044<br><sub>context: p90 3.251 · p95 3.719 · p99 3.992 · 409 op/s · total p50 2.291</sub> | 4.088<br><sub>context: p90 6.304 · p95 6.969 · p99 7.501 · 208 op/s · total p50 4.706</sub> | +99.9% (+2.043) | 150% AND 2 ms | 🟢 |
| 8 | 2.192<br><sub>context: p90 3.699 · p95 4.213 · p99 5.139 · 2970 op/s · total p50 2.461</sub> | 12.619<br><sub>context: p90 19.390 · p95 21.202 · p99 23.476 · 584 op/s · total p50 13.216</sub> | +475.6% (+10.426) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.101<br><sub>context: p90 0.174 · p95 0.204 · p99 0.212 · 3811 op/s · total p50 0.233</sub> | 0.017<br><sub>context: p90 0.044 · p95 0.045 · p99 0.055 · 4188 op/s · total p50 0.190</sub> | -83.0% (-0.084) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.261 · p95 0.292 · p99 0.370 · 20676 op/s · total p50 0.360</sub> | 0.020<br><sub>context: p90 0.034 · p95 0.042 · p99 0.050 · 23652 op/s · total p50 0.287</sub> | -89.1% (-0.165) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.115<br><sub>context: p90 0.150 · p95 0.154 · p99 0.212 · 3726 op/s · total p50 0.255</sub> | 0.013<br><sub>context: p90 0.043 · p95 0.046 · p99 0.049 · 4317 op/s · total p50 0.187</sub> | -89.0% (-0.102) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.251 · p95 0.282 · p99 0.351 · 21701 op/s · total p50 0.349</sub> | 0.017<br><sub>context: p90 0.023 · p95 0.027 · p99 0.034 · 28842 op/s · total p50 0.263</sub> | -90.7% (-0.168) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.212 · p95 0.222 · p99 0.229 · 2900 op/s · total p50 0.315</sub> | 0.010<br><sub>context: p90 0.014 · p95 0.015 · p99 0.020 · 4127 op/s · total p50 0.228</sub> | -92.5% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.196<br><sub>context: p90 0.260 · p95 0.284 · p99 0.335 · 15963 op/s · total p50 0.478</sub> | 0.011<br><sub>context: p90 0.017 · p95 0.019 · p99 0.024 · 21021 op/s · total p50 0.357</sub> | -94.3% (-0.184) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.171 · p95 0.182 · p99 0.192 · 4108 op/s · total p50 0.228</sub> | 0.005<br><sub>context: p90 0.007 · p95 0.007 · p99 0.009 · 6268 op/s · total p50 0.133</sub> | -95.1% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.161<br><sub>context: p90 0.228 · p95 0.254 · p99 0.316 · 25360 op/s · total p50 0.299</sub> | 0.006<br><sub>context: p90 0.010 · p95 0.014 · p99 0.017 · 28915 op/s · total p50 0.242</sub> | -96.2% (-0.155) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.294 · p95 0.303 · p99 0.313 · 2309 op/s · total p50 0.388</sub> | 0.130<br><sub>context: p90 0.175 · p95 0.184 · p99 0.202 · 2343 op/s · total p50 0.356</sub> | -41.5% (-0.092) | 150% AND 2 ms | 🟢 |
| 8 | 0.276<br><sub>context: p90 0.357 · p95 0.383 · p99 0.456 · 16820 op/s · total p50 0.449</sub> | 0.131<br><sub>context: p90 0.171 · p95 0.182 · p99 0.206 · 18899 op/s · total p50 0.393</sub> | -52.7% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.179<br><sub>context: p90 0.246 · p95 0.255 · p99 0.264 · 2795 op/s · total p50 0.327</sub> | 0.064<br><sub>context: p90 0.092 · p95 0.104 · p99 0.118 · 3718 op/s · total p50 0.244</sub> | -63.9% (-0.114) | 150% AND 2 ms | 🟢 |
| 8 | 0.229<br><sub>context: p90 0.311 · p95 0.342 · p99 0.423 · 19470 op/s · total p50 0.388</sub> | 0.068<br><sub>context: p90 0.097 · p95 0.106 · p99 0.123 · 23858 op/s · total p50 0.313</sub> | -70.5% (-0.162) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.211<br><sub>context: p90 0.308 · p95 0.319 · p99 0.337 · 2284 op/s · total p50 0.422</sub> | 0.074<br><sub>context: p90 0.100 · p95 0.111 · p99 0.128 · 3238 op/s · total p50 0.298</sub> | -64.7% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.303<br><sub>context: p90 0.390 · p95 0.422 · p99 0.502 · 14928 op/s · total p50 0.508</sub> | 0.102<br><sub>context: p90 0.135 · p95 0.145 · p99 0.164 · 16932 op/s · total p50 0.455</sub> | -66.4% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.215<br><sub>context: p90 0.298 · p95 0.321 · p99 0.335 · 2092 op/s · total p50 0.456</sub> | 0.082<br><sub>context: p90 0.125 · p95 0.133 · p99 0.167 · 2194 op/s · total p50 0.434</sub> | -62.0% (-0.133) | 150% AND 2 ms | 🟢 |
| 8 | 0.308<br><sub>context: p90 0.392 · p95 0.422 · p99 0.497 · 11374 op/s · total p50 0.654</sub> | 0.109<br><sub>context: p90 0.144 · p95 0.153 · p99 0.172 · 12672 op/s · total p50 0.608</sub> | -64.6% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.260<br><sub>context: p90 0.360 · p95 0.376 · p99 0.404 · 1615 op/s · total p50 0.596</sub> | 0.099<br><sub>context: p90 0.135 · p95 0.141 · p99 0.154 · 2130 op/s · total p50 0.462</sub> | -61.8% (-0.161) | 150% AND 2 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.424 · p95 0.454 · p99 0.537 · 11579 op/s · total p50 0.656</sub> | 0.113<br><sub>context: p90 0.149 · p95 0.158 · p99 0.177 · 12512 op/s · total p50 0.609</sub> | -66.1% (-0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.231<br><sub>context: p90 0.343 · p95 0.357 · p99 0.363 · 2123 op/s · total p50 0.439</sub> | 0.087<br><sub>context: p90 0.115 · p95 0.125 · p99 0.137 · 2925 op/s · total p50 0.336</sub> | -62.3% (-0.144) | 150% AND 2 ms | 🟢 |
| 8 | 0.335<br><sub>context: p90 0.431 · p95 0.471 · p99 0.553 · 14075 op/s · total p50 0.547</sub> | 0.106<br><sub>context: p90 0.138 · p95 0.148 · p99 0.166 · 15520 op/s · total p50 0.500</sub> | -68.3% (-0.229) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.306<br><sub>context: p90 0.362 · p95 0.377 · p99 0.394 · 1831 op/s · total p50 0.517</sub> | 0.103<br><sub>context: p90 0.133 · p95 0.147 · p99 0.158 · 2437 op/s · total p50 0.398</sub> | -66.3% (-0.203) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.478 · p95 0.523 · p99 0.604 · 12933 op/s · total p50 0.581</sub> | 0.085<br><sub>context: p90 0.119 · p95 0.130 · p99 0.154 · 19730 op/s · total p50 0.383</sub> | -76.2% (-0.273) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.890<br><sub>context: p90 6.044 · p95 6.095 · p99 6.187 · 59 op/s · total p50 16.803</sub> | 2.528<br><sub>context: p90 2.572 · p95 2.589 · p99 2.633 · 74 op/s · total p50 13.515</sub> | -57.1% (-3.362) | 150% AND 2 ms | 🟢 |
| 8 | 7.553<br><sub>context: p90 9.255 · p95 9.671 · p99 10.311 · 349 op/s · total p50 21.825</sub> | 3.148<br><sub>context: p90 3.445 · p95 3.490 · p99 3.740 · 471 op/s · total p50 16.033</sub> | -58.3% (-4.406) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.430 · p95 0.444 · p99 0.453 · 1809 op/s · total p50 0.541</sub> | 0.096<br><sub>context: p90 0.127 · p95 0.133 · p99 0.146 · 2834 op/s · total p50 0.340</sub> | -70.9% (-0.234) | 150% AND 2 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.579 · p95 0.635 · p99 0.750 · 11931 op/s · total p50 0.637</sub> | 0.114<br><sub>context: p90 0.145 · p95 0.154 · p99 0.174 · 15495 op/s · total p50 0.499</sub> | -74.4% (-0.332) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.554<br><sub>context: p90 0.651 · p95 0.679 · p99 0.737 · 639 op/s · total p50 1.560</sub> | 0.230<br><sub>context: p90 0.295 · p95 0.310 · p99 0.361 · 842 op/s · total p50 1.187</sub> | -58.6% (-0.324) | 150% AND 2 ms | 🟢 |
| 8 | 0.626<br><sub>context: p90 0.783 · p95 0.828 · p99 0.911 · 4579 op/s · total p50 1.708</sub> | 0.270<br><sub>context: p90 0.348 · p95 0.368 · p99 0.406 · 5345 op/s · total p50 1.465</sub> | -56.8% (-0.356) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.349 · p95 0.364 · p99 0.370 · 1987 op/s · total p50 0.479</sub> | 0.091<br><sub>context: p90 0.131 · p95 0.145 · p99 0.163 · 2655 op/s · total p50 0.357</sub> | -66.8% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.326<br><sub>context: p90 0.419 · p95 0.457 · p99 0.543 · 13785 op/s · total p50 0.549</sub> | 0.109<br><sub>context: p90 0.144 · p95 0.156 · p99 0.183 · 15761 op/s · total p50 0.493</sub> | -66.6% (-0.217) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.717<br><sub>context: p90 2.185 · p95 2.252 · p99 2.302 · 527 op/s · total p50 1.928</sub> | 0.108<br><sub>context: p90 0.152 · p95 0.166 · p99 0.193 · 2744 op/s · total p50 0.347</sub> | -93.7% (-1.608) | 150% AND 2 ms | 🟢 |
| 8 | 1.806<br><sub>context: p90 2.299 · p95 2.398 · p99 2.567 · 3639 op/s · total p50 2.128</sub> | 0.116<br><sub>context: p90 0.165 · p95 0.177 · p99 0.203 · 18805 op/s · total p50 0.404</sub> | -93.6% (-1.690) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.765<br><sub>context: p90 2.243 · p95 2.317 · p99 2.446 · 492 op/s · total p50 2.068</sub> | 0.112<br><sub>context: p90 0.161 · p95 0.180 · p99 0.206 · 2785 op/s · total p50 0.352</sub> | -93.6% (-1.653) | 150% AND 2 ms | 🟢 |
| 8 | 1.842<br><sub>context: p90 2.360 · p95 2.472 · p99 2.740 · 3651 op/s · total p50 2.122</sub> | 0.121<br><sub>context: p90 0.174 · p95 0.192 · p99 0.228 · 17086 op/s · total p50 0.454</sub> | -93.4% (-1.721) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.191 · p95 0.197 · p99 0.213 · 3016 op/s · total p50 0.304</sub> | 0.033<br><sub>context: p90 0.057 · p95 0.060 · p99 0.071 · 4773 op/s · total p50 0.200</sub> | -74.7% (-0.097) | 150% AND 2 ms | 🟢 |
| 8 | 0.156<br><sub>context: p90 0.215 · p95 0.240 · p99 0.287 · 23517 op/s · total p50 0.317</sub> | 0.034<br><sub>context: p90 0.062 · p95 0.066 · p99 0.073 · 28219 op/s · total p50 0.271</sub> | -78.0% (-0.122) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.182<br><sub>context: p90 0.254 · p95 0.261 · p99 0.271 · 2800 op/s · total p50 0.339</sub> | 0.004<br><sub>context: p90 0.011 · p95 0.011 · p99 0.012 · 4109 op/s · total p50 0.198</sub> | -97.9% (-0.178) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.240<br><sub>context: p90 0.312 · p95 0.336 · p99 0.402 · 18362 op/s · total p50 0.414</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.008 · 28582 op/s · total p50 0.265</sub> | -98.4% (-0.237) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.161<br><sub>context: p90 0.206 · p95 0.213 · p99 0.234 · 3098 op/s · total p50 0.311</sub> | 0.046<br><sub>context: p90 0.075 · p95 0.089 · p99 0.093 · 2890 op/s · total p50 0.265</sub> | -71.3% (-0.114) | 150% AND 2 ms | 🟢 |
| 8 | 0.295<br><sub>context: p90 0.402 · p95 0.439 · p99 0.544 · 16395 op/s · total p50 0.461</sub> | 0.046<br><sub>context: p90 0.074 · p95 0.079 · p99 0.092 · 21578 op/s · total p50 0.338</sub> | -84.4% (-0.249) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.229 · p95 0.262 · p99 0.289 · 2867 op/s · total p50 0.328</sub> | 0.061<br><sub>context: p90 0.115 · p95 0.121 · p99 0.140 · 3104 op/s · total p50 0.248</sub> | -65.7% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.301<br><sub>context: p90 0.404 · p95 0.453 · p99 0.536 · 16330 op/s · total p50 0.463</sub> | 0.065<br><sub>context: p90 0.125 · p95 0.132 · p99 0.141 · 23011 op/s · total p50 0.330</sub> | -78.5% (-0.236) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.167<br><sub>context: p90 0.254 · p95 0.264 · p99 0.274 · 2590 op/s · total p50 0.374</sub> | 0.038<br><sub>context: p90 0.066 · p95 0.079 · p99 0.085 · 4006 op/s · total p50 0.225</sub> | -77.5% (-0.130) | 150% AND 2 ms | 🟢 |
| 8 | 0.242<br><sub>context: p90 0.323 · p95 0.358 · p99 0.453 · 19240 op/s · total p50 0.393</sub> | 0.038<br><sub>context: p90 0.067 · p95 0.071 · p99 0.084 · 25559 op/s · total p50 0.286</sub> | -84.4% (-0.204) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.978<br><sub>context: p90 1.068 · p95 1.089 · p99 1.099 · 715 op/s · total p50 1.387</sub> | 0.504<br><sub>context: p90 0.540 · p95 0.557 · p99 0.568 · 1094 op/s · total p50 0.905</sub> | -48.5% (-0.474) | 150% AND 2 ms | 🟢 |
| 8 | 1.095<br><sub>context: p90 1.327 · p95 1.396 · p99 1.709 · 4928 op/s · total p50 1.528</sub> | 0.540<br><sub>context: p90 0.601 · p95 0.632 · p99 0.672 · 6196 op/s · total p50 1.200</sub> | -50.7% (-0.555) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.005<br><sub>context: p90 1.088 · p95 1.109 · p99 1.145 · 788 op/s · total p50 1.238</sub> | 0.493<br><sub>context: p90 0.534 · p95 0.545 · p99 0.584 · 1278 op/s · total p50 0.773</sub> | -51.0% (-0.512) | 150% AND 2 ms | 🟢 |
| 8 | 1.082<br><sub>context: p90 1.251 · p95 1.312 · p99 1.494 · 5408 op/s · total p50 1.334</sub> | 0.540<br><sub>context: p90 0.610 · p95 0.634 · p99 0.683 · 7809 op/s · total p50 0.946</sub> | -50.1% (-0.542) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.217<br><sub>context: p90 0.290 · p95 0.299 · p99 0.314 · 2351 op/s · total p50 0.407</sub> | 0.075<br><sub>context: p90 0.104 · p95 0.111 · p99 0.122 · 2562 op/s · total p50 0.360</sub> | -65.4% (-0.142) | 150% AND 2 ms | 🟢 |
| 8 | 0.264<br><sub>context: p90 0.348 · p95 0.375 · p99 0.450 · 15903 op/s · total p50 0.474</sub> | 0.066<br><sub>context: p90 0.096 · p95 0.104 · p99 0.122 · 19308 op/s · total p50 0.388</sub> | -75.0% (-0.198) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.343<br><sub>context: p90 0.439 · p95 0.464 · p99 0.550 · 1767 op/s · total p50 0.539</sub> | 0.189<br><sub>context: p90 0.273 · p95 0.294 · p99 0.350 · 1784 op/s · total p50 0.535</sub> | -44.9% (-0.154) | 150% AND 2 ms | 🟢 |
| 8 | 0.412<br><sub>context: p90 0.537 · p95 0.570 · p99 0.640 · 12358 op/s · total p50 0.613</sub> | 0.203<br><sub>context: p90 0.324 · p95 0.358 · p99 0.424 · 14548 op/s · total p50 0.503</sub> | -50.7% (-0.209) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.146<br><sub>context: p90 0.237 · p95 0.251 · p99 0.261 · 3135 op/s · total p50 0.288</sub> | 0.029<br><sub>context: p90 0.061 · p95 0.063 · p99 0.078 · 3580 op/s · total p50 0.243</sub> | -80.4% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.221<br><sub>context: p90 0.295 · p95 0.323 · p99 0.387 · 18635 op/s · total p50 0.402</sub> | 0.026<br><sub>context: p90 0.035 · p95 0.039 · p99 0.048 · 25563 op/s · total p50 0.298</sub> | -88.2% (-0.195) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.187 · p95 0.194 · p99 0.212 · 3708 op/s · total p50 0.248</sub> | 0.031<br><sub>context: p90 0.056 · p95 0.058 · p99 0.065 · 5110 op/s · total p50 0.186</sub> | -74.6% (-0.091) | 150% AND 2 ms | 🟢 |
| 8 | 0.163<br><sub>context: p90 0.234 · p95 0.260 · p99 0.326 · 23138 op/s · total p50 0.324</sub> | 0.034<br><sub>context: p90 0.061 · p95 0.065 · p99 0.071 · 28562 op/s · total p50 0.270</sub> | -79.0% (-0.129) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.116<br><sub>context: p90 0.190 · p95 0.199 · p99 0.209 · 3623 op/s · total p50 0.247</sub> | 0.031<br><sub>context: p90 0.056 · p95 0.058 · p99 0.060 · 4813 op/s · total p50 0.195</sub> | -73.5% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.158<br><sub>context: p90 0.225 · p95 0.251 · p99 0.305 · 23381 op/s · total p50 0.321</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.065 · p99 0.072 · 28105 op/s · total p50 0.273</sub> | -78.6% (-0.125) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.759<br><sub>context: p90 0.797 · p95 0.811 · p99 0.850 · 988 op/s · total p50 0.997</sub> | 0.175<br><sub>context: p90 0.198 · p95 0.204 · p99 0.216 · 2482 op/s · total p50 0.381</sub> | -77.0% (-0.584) | 150% AND 2 ms | 🟢 |
| 8 | 0.803<br><sub>context: p90 1.217 · p95 1.372 · p99 1.612 · 6059 op/s · total p50 1.083</sub> | 0.187<br><sub>context: p90 0.220 · p95 0.227 · p99 0.245 · 17975 op/s · total p50 0.429</sub> | -76.7% (-0.616) | 150% AND 2 ms | 🟢 |

</details>
