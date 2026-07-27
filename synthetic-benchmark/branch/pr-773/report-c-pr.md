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

**pr vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.230<br><sub>context: p90 1.287 · p95 1.299 · p99 1.337 · 769 op/s · total p50 5.185</sub> | 0.584<br><sub>context: p90 0.614 · p95 0.625 · p99 0.634 · 1456 op/s · total p50 2.739</sub> | -52.5% (-0.646) | 150% AND 2 ms | 🟢 |
| 8 | 1.384<br><sub>context: p90 2.045 · p95 2.283 · p99 2.603 · 4874 op/s · total p50 6.220</sub> | 0.621<br><sub>context: p90 0.688 · p95 0.717 · p99 0.751 · 9268 op/s · total p50 3.262</sub> | -55.2% (-0.764) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.887<br><sub>context: p90 1.959 · p95 1.974 · p99 2.012 · 509 op/s · total p50 7.812</sub> | 0.626<br><sub>context: p90 0.659 · p95 0.669 · p99 0.685 · 1334 op/s · total p50 2.983</sub> | -66.8% (-1.261) | 150% AND 2 ms | 🟢 |
| 8 | 2.028<br><sub>context: p90 2.460 · p95 2.638 · p99 3.004 · 3548 op/s · total p50 8.622</sub> | 0.656<br><sub>context: p90 0.707 · p95 0.729 · p99 0.775 · 8696 op/s · total p50 3.581</sub> | -67.7% (-1.372) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.865<br><sub>context: p90 1.935 · p95 1.953 · p99 1.977 · 515 op/s · total p50 7.748</sub> | 1.091<br><sub>context: p90 1.122 · p95 1.133 · p99 1.156 · 805 op/s · total p50 4.912</sub> | -41.5% (-0.774) | 150% AND 2 ms | 🟢 |
| 8 | 2.056<br><sub>context: p90 2.652 · p95 2.930 · p99 3.275 · 3386 op/s · total p50 8.913</sub> | 1.703<br><sub>context: p90 2.448 · p95 2.603 · p99 2.953 · 3973 op/s · total p50 7.853</sub> | -17.1% (-0.352) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.453<br><sub>context: p90 2.534 · p95 2.554 · p99 2.574 · 395 op/s · total p50 10.134</sub> | 1.175<br><sub>context: p90 1.207 · p95 1.218 · p99 1.236 · 758 op/s · total p50 5.241</sub> | -52.1% (-1.278) | 150% AND 2 ms | 🟢 |
| 8 | 2.813<br><sub>context: p90 3.681 · p95 3.958 · p99 4.914 · 2476 op/s · total p50 12.087</sub> | 1.210<br><sub>context: p90 1.309 · p95 1.353 · p99 1.402 · 5147 op/s · total p50 5.832</sub> | -57.0% (-1.603) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.066<br><sub>context: p90 0.095 · p95 0.109 · p99 0.133 · 9391 op/s · total p50 0.412</sub> | 0.014<br><sub>context: p90 0.018 · p95 0.020 · p99 0.022 · 14290 op/s · total p50 0.263</sub> | -79.0% (-0.052) | 150% AND 2 ms | 🟢 |
| 8 | 0.159<br><sub>context: p90 0.284 · p95 0.352 · p99 0.433 · 35635 op/s · total p50 0.850</sub> | 0.016<br><sub>context: p90 0.023 · p95 0.025 · p99 0.032 · 50648 op/s · total p50 0.431</sub> | -89.7% (-0.143) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.249 · p95 0.271 · p99 0.293 · 3963 op/s · total p50 0.996</sub> | 0.046<br><sub>context: p90 0.074 · p95 0.084 · p99 0.094 · 6763 op/s · total p50 0.580</sub> | -75.5% (-0.141) | 150% AND 2 ms | 🟢 |
| 8 | 0.307<br><sub>context: p90 0.455 · p95 0.516 · p99 0.631 · 18820 op/s · total p50 1.641</sub> | 0.054<br><sub>context: p90 0.088 · p95 0.092 · p99 0.101 · 36197 op/s · total p50 0.829</sub> | -82.3% (-0.252) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.236<br><sub>context: p90 0.279 · p95 0.301 · p99 0.314 · 3415 op/s · total p50 1.148</sub> | 0.054<br><sub>context: p90 0.087 · p95 0.089 · p99 0.097 · 5608 op/s · total p50 0.680</sub> | -77.3% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.334<br><sub>context: p90 0.467 · p95 0.527 · p99 0.645 · 17519 op/s · total p50 1.759</sub> | 0.057<br><sub>context: p90 0.091 · p95 0.095 · p99 0.108 · 35085 op/s · total p50 0.868</sub> | -82.8% (-0.277) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.243<br><sub>context: p90 0.300 · p95 0.308 · p99 0.326 · 3312 op/s · total p50 1.195</sub> | 0.095<br><sub>context: p90 0.124 · p95 0.131 · p99 0.156 · 4451 op/s · total p50 0.880</sub> | -61.0% (-0.148) | 150% AND 2 ms | 🟢 |
| 8 | 0.365<br><sub>context: p90 0.538 · p95 0.584 · p99 0.738 · 15973 op/s · total p50 1.937</sub> | 0.103<br><sub>context: p90 0.138 · p95 0.150 · p99 0.167 · 28149 op/s · total p50 1.069</sub> | -71.7% (-0.261) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.322 · p95 0.337 · p99 0.352 · 2900 op/s · total p50 1.369</sub> | 0.104<br><sub>context: p90 0.136 · p95 0.147 · p99 0.163 · 3836 op/s · total p50 1.051</sub> | -63.0% (-0.177) | 150% AND 2 ms | 🟢 |
| 8 | 0.394<br><sub>context: p90 0.563 · p95 0.643 · p99 0.751 · 14922 op/s · total p50 2.066</sub> | 0.112<br><sub>context: p90 0.150 · p95 0.160 · p99 0.178 · 23647 op/s · total p50 1.263</sub> | -71.6% (-0.282) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.293<br><sub>context: p90 0.373 · p95 0.394 · p99 0.418 · 2742 op/s · total p50 1.451</sub> | 0.136<br><sub>context: p90 0.182 · p95 0.194 · p99 0.222 · 3271 op/s · total p50 1.200</sub> | -53.5% (-0.157) | 150% AND 2 ms | 🟢 |
| 8 | 0.451<br><sub>context: p90 0.616 · p95 0.689 · p99 0.840 · 12491 op/s · total p50 2.431</sub> | 0.149<br><sub>context: p90 0.200 · p95 0.214 · p99 0.242 · 14983 op/s · total p50 2.035</sub> | -66.9% (-0.301) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.352<br><sub>context: p90 0.426 · p95 0.447 · p99 0.490 · 2368 op/s · total p50 1.683</sub> | 0.149<br><sub>context: p90 0.184 · p95 0.202 · p99 0.219 · 2986 op/s · total p50 1.329</sub> | -57.5% (-0.202) | 150% AND 2 ms | 🟢 |
| 8 | 0.499<br><sub>context: p90 0.693 · p95 0.767 · p99 0.900 · 11307 op/s · total p50 2.745</sub> | 0.159<br><sub>context: p90 0.218 · p95 0.233 · p99 0.260 · 13591 op/s · total p50 2.215</sub> | -68.1% (-0.340) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.561<br><sub>context: p90 0.734 · p95 0.822 · p99 0.892 · 1448 op/s · total p50 2.754</sub> | 0.241<br><sub>context: p90 0.323 · p95 0.338 · p99 0.385 · 1851 op/s · total p50 1.935</sub> | -57.0% (-0.320) | 150% AND 2 ms | 🟢 |
| 8 | 0.618<br><sub>context: p90 0.847 · p95 0.913 · p99 1.040 · 3773 op/s · total p50 8.128</sub> | 0.272<br><sub>context: p90 0.376 · p95 0.411 · p99 0.452 · 3971 op/s · total p50 7.548</sub> | -56.0% (-0.346) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.608<br><sub>context: p90 0.759 · p95 0.812 · p99 0.897 · 1401 op/s · total p50 2.870</sub> | 0.276<br><sub>context: p90 0.361 · p95 0.382 · p99 0.442 · 1734 op/s · total p50 2.162</sub> | -54.6% (-0.331) | 150% AND 2 ms | 🟢 |
| 8 | 0.698<br><sub>context: p90 0.954 · p95 1.035 · p99 1.191 · 3447 op/s · total p50 8.825</sub> | 0.300<br><sub>context: p90 0.390 · p95 0.420 · p99 0.470 · 3831 op/s · total p50 7.872</sub> | -57.0% (-0.398) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.443<br><sub>context: p90 0.523 · p95 0.540 · p99 0.567 · 1981 op/s · total p50 2.011</sub> | 0.325<br><sub>context: p90 0.519 · p95 0.538 · p99 0.629 · 1997 op/s · total p50 1.963</sub> | -26.6% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.579<br><sub>context: p90 0.751 · p95 0.826 · p99 0.999 · 10341 op/s · total p50 2.975</sub> | 0.414<br><sub>context: p90 0.640 · p95 0.715 · p99 0.843 · 11165 op/s · total p50 2.749</sub> | -28.5% (-0.165) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.272<br><sub>context: p90 0.332 · p95 0.346 · p99 0.362 · 3003 op/s · total p50 1.306</sub> | 0.078<br><sub>context: p90 0.107 · p95 0.112 · p99 0.127 · 4962 op/s · total p50 0.796</sub> | -71.5% (-0.195) | 150% AND 2 ms | 🟢 |
| 8 | 0.408<br><sub>context: p90 0.587 · p95 0.651 · p99 0.826 · 14784 op/s · total p50 2.078</sub> | 0.085<br><sub>context: p90 0.119 · p95 0.125 · p99 0.146 · 30254 op/s · total p50 0.991</sub> | -79.1% (-0.322) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.147 · p95 0.158 · p99 0.173 · 6241 op/s · total p50 0.625</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 12331 op/s · total p50 0.309</sub> | -98.2% (-0.120) | 150% AND 2 ms | 🟢 |
| 8 | 0.235<br><sub>context: p90 0.376 · p95 0.425 · p99 0.519 · 25436 op/s · total p50 1.202</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 54043 op/s · total p50 0.434</sub> | -99.1% (-0.233) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.088<br><sub>context: p90 0.105 · p95 0.110 · p99 0.143 · 8299 op/s · total p50 0.480</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 13709 op/s · total p50 0.274</sub> | -97.9% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.299 · p95 0.360 · p99 0.466 · 32482 op/s · total p50 0.941</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 57158 op/s · total p50 0.391</sub> | -98.8% (-0.187) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.388 · p95 0.399 · p99 0.426 · 2563 op/s · total p50 1.546</sub> | 0.067<br><sub>context: p90 0.095 · p95 0.100 · p99 0.122 · 4267 op/s · total p50 0.905</sub> | -79.4% (-0.256) | 150% AND 2 ms | 🟢 |
| 8 | 0.490<br><sub>context: p90 0.722 · p95 0.836 · p99 1.024 · 12433 op/s · total p50 2.487</sub> | 0.078<br><sub>context: p90 0.110 · p95 0.118 · p99 0.132 · 26036 op/s · total p50 1.141</sub> | -84.1% (-0.412) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.649<br><sub>context: p90 0.842 · p95 0.882 · p99 0.943 · 1404 op/s · total p50 2.869</sub> | 1.091<br><sub>context: p90 1.530 · p95 1.597 · p99 1.683 · 807 op/s · total p50 4.949</sub> | +68.0% (+0.441) | 150% AND 2 ms | 🟢 |
| 8 | 0.772<br><sub>context: p90 1.065 · p95 1.153 · p99 1.496 · 7571 op/s · total p50 4.003</sub> | 1.761<br><sub>context: p90 2.634 · p95 2.827 · p99 3.140 · 3694 op/s · total p50 8.098</sub> | +128.2% (+0.989) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.260<br><sub>context: p90 3.910 · p95 4.075 · p99 4.307 · 401 op/s · total p50 9.875</sub> | 4.505<br><sub>context: p90 7.279 · p95 7.619 · p99 7.922 · 203 op/s · total p50 19.545</sub> | +99.4% (+2.245) | 150% AND 2 ms | 🟢 |
| 8 | 2.386<br><sub>context: p90 4.030 · p95 4.556 · p99 5.867 · 2866 op/s · total p50 10.517</sub> | 7.542<br><sub>context: p90 11.785 · p95 12.486 · p99 13.602 · 966 op/s · total p50 32.702</sub> | +216.1% (+5.157) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.120<br><sub>context: p90 0.163 · p95 0.171 · p99 0.195 · 5886 op/s · total p50 0.670</sub> | 0.014<br><sub>context: p90 0.019 · p95 0.020 · p99 0.024 · 10895 op/s · total p50 0.349</sub> | -88.4% (-0.106) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.373 · p95 0.431 · p99 0.542 · 24170 op/s · total p50 1.271</sub> | 0.019<br><sub>context: p90 0.023 · p95 0.026 · p99 0.031 · 44353 op/s · total p50 0.614</sub> | -92.1% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.118<br><sub>context: p90 0.148 · p95 0.160 · p99 0.195 · 5838 op/s · total p50 0.656</sub> | 0.017<br><sub>context: p90 0.019 · p95 0.022 · p99 0.026 · 10626 op/s · total p50 0.362</sub> | -85.7% (-0.101) | 150% AND 2 ms | 🟢 |
| 8 | 0.234<br><sub>context: p90 0.381 · p95 0.435 · p99 0.553 · 23729 op/s · total p50 1.287</sub> | 0.018<br><sub>context: p90 0.022 · p95 0.023 · p99 0.027 · 50417 op/s · total p50 0.467</sub> | -92.4% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.129<br><sub>context: p90 0.181 · p95 0.207 · p99 0.216 · 5411 op/s · total p50 0.724</sub> | 0.010<br><sub>context: p90 0.012 · p95 0.013 · p99 0.017 · 7873 op/s · total p50 0.487</sub> | -92.5% (-0.119) | 150% AND 2 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.254 · p95 0.277 · p99 0.341 · 17197 op/s · total p50 1.778</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.016 · p99 0.020 · 17983 op/s · total p50 1.689</sub> | -94.5% (-0.179) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.097<br><sub>context: p90 0.129 · p95 0.149 · p99 0.162 · 7014 op/s · total p50 0.556</sub> | 0.004<br><sub>context: p90 0.006 · p95 0.007 · p99 0.008 · 14829 op/s · total p50 0.260</sub> | -95.6% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.197<br><sub>context: p90 0.323 · p95 0.374 · p99 0.486 · 29947 op/s · total p50 1.029</sub> | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.012 · 56154 op/s · total p50 0.444</sub> | -97.0% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.218<br><sub>context: p90 0.271 · p95 0.286 · p99 0.312 · 3575 op/s · total p50 1.101</sub> | 0.143<br><sub>context: p90 0.177 · p95 0.188 · p99 0.210 · 3737 op/s · total p50 1.059</sub> | -34.7% (-0.076) | 150% AND 2 ms | 🟢 |
| 8 | 0.313<br><sub>context: p90 0.447 · p95 0.505 · p99 0.589 · 17788 op/s · total p50 1.717</sub> | 0.153<br><sub>context: p90 0.196 · p95 0.211 · p99 0.240 · 25948 op/s · total p50 1.156</sub> | -51.1% (-0.160) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.158<br><sub>context: p90 0.200 · p95 0.210 · p99 0.227 · 4827 op/s · total p50 0.816</sub> | 0.065<br><sub>context: p90 0.093 · p95 0.098 · p99 0.104 · 6314 op/s · total p50 0.618</sub> | -59.0% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.258<br><sub>context: p90 0.398 · p95 0.449 · p99 0.536 · 21993 op/s · total p50 1.397</sub> | 0.075<br><sub>context: p90 0.109 · p95 0.121 · p99 0.150 · 35263 op/s · total p50 0.819</sub> | -71.0% (-0.183) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.245<br><sub>context: p90 0.290 · p95 0.300 · p99 0.314 · 3364 op/s · total p50 1.174</sub> | 0.098<br><sub>context: p90 0.128 · p95 0.139 · p99 0.152 · 4364 op/s · total p50 0.891</sub> | -60.1% (-0.147) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.478 · p95 0.528 · p99 0.639 · 17075 op/s · total p50 1.806</sub> | 0.104<br><sub>context: p90 0.140 · p95 0.149 · p99 0.170 · 26220 op/s · total p50 1.117</sub> | -69.2% (-0.234) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.205<br><sub>context: p90 0.291 · p95 0.313 · p99 0.336 · 3481 op/s · total p50 1.131</sub> | 0.101<br><sub>context: p90 0.136 · p95 0.147 · p99 0.176 · 3567 op/s · total p50 1.116</sub> | -50.6% (-0.104) | 150% AND 2 ms | 🟢 |
| 8 | 0.314<br><sub>context: p90 0.413 · p95 0.469 · p99 0.580 · 14030 op/s · total p50 2.108</sub> | 0.104<br><sub>context: p90 0.142 · p95 0.153 · p99 0.172 · 14358 op/s · total p50 2.078</sub> | -67.0% (-0.210) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.249<br><sub>context: p90 0.310 · p95 0.322 · p99 0.354 · 3046 op/s · total p50 1.310</sub> | 0.109<br><sub>context: p90 0.134 · p95 0.152 · p99 0.162 · 3546 op/s · total p50 1.094</sub> | -56.4% (-0.140) | 150% AND 2 ms | 🟢 |
| 8 | 0.374<br><sub>context: p90 0.512 · p95 0.570 · p99 0.700 · 13838 op/s · total p50 2.121</sub> | 0.111<br><sub>context: p90 0.149 · p95 0.160 · p99 0.182 · 16488 op/s · total p50 1.869</sub> | -70.4% (-0.264) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.244<br><sub>context: p90 0.301 · p95 0.312 · p99 0.349 · 3302 op/s · total p50 1.192</sub> | 0.088<br><sub>context: p90 0.126 · p95 0.137 · p99 0.157 · 4259 op/s · total p50 0.901</sub> | -63.7% (-0.155) | 150% AND 2 ms | 🟢 |
| 8 | 0.376<br><sub>context: p90 0.549 · p95 0.614 · p99 0.723 · 15677 op/s · total p50 1.967</sub> | 0.105<br><sub>context: p90 0.140 · p95 0.150 · p99 0.166 · 26173 op/s · total p50 1.140</sub> | -72.2% (-0.271) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.329 · p95 0.342 · p99 0.369 · 2995 op/s · total p50 1.334</sub> | 0.074<br><sub>context: p90 0.104 · p95 0.108 · p99 0.133 · 5306 op/s · total p50 0.741</sub> | -73.8% (-0.209) | 150% AND 2 ms | 🟢 |
| 8 | 0.389<br><sub>context: p90 0.555 · p95 0.622 · p99 0.765 · 15143 op/s · total p50 2.026</sub> | 0.085<br><sub>context: p90 0.122 · p95 0.132 · p99 0.156 · 30667 op/s · total p50 0.977</sub> | -78.3% (-0.304) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.632<br><sub>context: p90 5.785 · p95 5.867 · p99 6.058 · 72 op/s · total p50 53.929</sub> | 2.753<br><sub>context: p90 2.798 · p95 2.819 · p99 2.838 · 93 op/s · total p50 42.695</sub> | -51.1% (-2.879) | 150% AND 2 ms | 🟢 |
| 8 | 6.081<br><sub>context: p90 8.042 · p95 8.332 · p99 8.644 · 154 op/s · total p50 202.674</sub> | 2.758<br><sub>context: p90 2.831 · p95 2.859 · p99 2.920 · 158 op/s · total p50 197.354</sub> | -54.6% (-3.322) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.347<br><sub>context: p90 0.410 · p95 0.419 · p99 0.455 · 2461 op/s · total p50 1.619</sub> | 0.104<br><sub>context: p90 0.131 · p95 0.138 · p99 0.147 · 3870 op/s · total p50 1.014</sub> | -70.0% (-0.243) | 150% AND 2 ms | 🟢 |
| 8 | 0.489<br><sub>context: p90 0.675 · p95 0.763 · p99 0.916 · 12179 op/s · total p50 2.506</sub> | 0.109<br><sub>context: p90 0.144 · p95 0.153 · p99 0.170 · 24288 op/s · total p50 1.209</sub> | -77.7% (-0.380) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.533<br><sub>context: p90 0.657 · p95 0.698 · p99 0.772 · 1292 op/s · total p50 2.752</sub> | 0.249<br><sub>context: p90 0.327 · p95 0.352 · p99 0.408 · 1080 op/s · total p50 3.800</sub> | -53.2% (-0.283) | 150% AND 2 ms | 🟢 |
| 8 | 0.578<br><sub>context: p90 0.749 · p95 0.795 · p99 0.899 · 2530 op/s · total p50 11.790</sub> | 0.259<br><sub>context: p90 0.351 · p95 0.382 · p99 0.436 · 2620 op/s · total p50 11.470</sub> | -55.1% (-0.318) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.260<br><sub>context: p90 0.308 · p95 0.315 · p99 0.342 · 3145 op/s · total p50 1.255</sub> | 0.103<br><sub>context: p90 0.137 · p95 0.145 · p99 0.157 · 4051 op/s · total p50 0.958</sub> | -60.5% (-0.157) | 150% AND 2 ms | 🟢 |
| 8 | 0.382<br><sub>context: p90 0.538 · p95 0.590 · p99 0.727 · 15425 op/s · total p50 2.020</sub> | 0.111<br><sub>context: p90 0.148 · p95 0.157 · p99 0.185 · 25419 op/s · total p50 1.137</sub> | -70.8% (-0.270) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.041<br><sub>context: p90 2.602 · p95 2.637 · p99 2.706 · 486 op/s · total p50 8.411</sub> | 0.106<br><sub>context: p90 0.149 · p95 0.157 · p99 0.202 · 4372 op/s · total p50 0.891</sub> | -94.8% (-1.935) | 150% AND 2 ms | 🟢 |
| 8 | 2.224<br><sub>context: p90 2.801 · p95 2.911 · p99 3.796 · 3391 op/s · total p50 9.172</sub> | 0.126<br><sub>context: p90 0.175 · p95 0.197 · p99 0.240 · 24859 op/s · total p50 1.240</sub> | -94.3% (-2.098) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.965<br><sub>context: p90 2.575 · p95 2.636 · p99 2.748 · 494 op/s · total p50 8.022</sub> | 0.109<br><sub>context: p90 0.154 · p95 0.173 · p99 0.190 · 4087 op/s · total p50 0.949</sub> | -94.5% (-1.856) | 150% AND 2 ms | 🟢 |
| 8 | 2.156<br><sub>context: p90 2.797 · p95 2.942 · p99 3.913 · 3538 op/s · total p50 8.671</sub> | 0.121<br><sub>context: p90 0.171 · p95 0.191 · p99 0.215 · 25601 op/s · total p50 1.135</sub> | -94.4% (-2.035) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.120<br><sub>context: p90 0.145 · p95 0.156 · p99 0.178 · 6291 op/s · total p50 0.630</sub> | 0.035<br><sub>context: p90 0.070 · p95 0.073 · p99 0.077 · 8596 op/s · total p50 0.441</sub> | -71.2% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.311 · p95 0.356 · p99 0.461 · 29871 op/s · total p50 1.016</sub> | 0.038<br><sub>context: p90 0.072 · p95 0.074 · p99 0.084 · 46433 op/s · total p50 0.539</sub> | -79.6% (-0.148) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.189<br><sub>context: p90 0.252 · p95 0.260 · p99 0.294 · 4118 op/s · total p50 0.962</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.005 · p99 0.008 · 10472 op/s · total p50 0.374</sub> | -98.4% (-0.187) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.283<br><sub>context: p90 0.411 · p95 0.483 · p99 0.557 · 19635 op/s · total p50 1.588</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.007 · 48848 op/s · total p50 0.479</sub> | -98.6% (-0.279) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.191<br><sub>context: p90 0.235 · p95 0.245 · p99 0.258 · 4268 op/s · total p50 0.923</sub> | 0.045<br><sub>context: p90 0.072 · p95 0.077 · p99 0.085 · 6284 op/s · total p50 0.617</sub> | -76.6% (-0.146) | 150% AND 2 ms | 🟢 |
| 8 | 0.328<br><sub>context: p90 0.496 · p95 0.566 · p99 0.713 · 18874 op/s · total p50 1.627</sub> | 0.050<br><sub>context: p90 0.082 · p95 0.086 · p99 0.095 · 36673 op/s · total p50 0.807</sub> | -84.8% (-0.278) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.200<br><sub>context: p90 0.239 · p95 0.251 · p99 0.267 · 4152 op/s · total p50 0.955</sub> | 0.066<br><sub>context: p90 0.122 · p95 0.127 · p99 0.135 · 5810 op/s · total p50 0.675</sub> | -67.0% (-0.134) | 150% AND 2 ms | 🟢 |
| 8 | 0.336<br><sub>context: p90 0.492 · p95 0.555 · p99 0.704 · 18417 op/s · total p50 1.669</sub> | 0.076<br><sub>context: p90 0.143 · p95 0.147 · p99 0.159 · 33586 op/s · total p50 0.892</sub> | -77.4% (-0.260) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.235 · p95 0.250 · p99 0.264 · 4356 op/s · total p50 0.904</sub> | 0.040<br><sub>context: p90 0.070 · p95 0.071 · p99 0.084 · 7013 op/s · total p50 0.554</sub> | -77.8% (-0.138) | 150% AND 2 ms | 🟢 |
| 8 | 0.274<br><sub>context: p90 0.405 · p95 0.464 · p99 0.574 · 21109 op/s · total p50 1.455</sub> | 0.042<br><sub>context: p90 0.075 · p95 0.079 · p99 0.090 · 39621 op/s · total p50 0.763</sub> | -84.5% (-0.232) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.089<br><sub>context: p90 1.168 · p95 1.187 · p99 1.214 · 850 op/s · total p50 4.678</sub> | 0.568<br><sub>context: p90 0.615 · p95 0.620 · p99 0.628 · 1352 op/s · total p50 2.949</sub> | -47.8% (-0.521) | 150% AND 2 ms | 🟢 |
| 8 | 1.275<br><sub>context: p90 1.567 · p95 1.738 · p99 2.056 · 5389 op/s · total p50 5.555</sub> | 0.623<br><sub>context: p90 0.757 · p95 0.786 · p99 0.871 · 8439 op/s · total p50 3.640</sub> | -51.1% (-0.652) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.090<br><sub>context: p90 1.180 · p95 1.189 · p99 1.223 · 870 op/s · total p50 4.604</sub> | 0.559<br><sub>context: p90 0.595 · p95 0.601 · p99 0.621 · 1385 op/s · total p50 2.857</sub> | -48.7% (-0.530) | 150% AND 2 ms | 🟢 |
| 8 | 1.211<br><sub>context: p90 1.404 · p95 1.515 · p99 1.736 · 4982 op/s · total p50 6.130</sub> | 0.583<br><sub>context: p90 0.631 · p95 0.649 · p99 0.691 · 8927 op/s · total p50 3.470</sub> | -51.8% (-0.628) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.222 · p95 0.234 · p99 0.267 · 4184 op/s · total p50 0.941</sub> | 0.055<br><sub>context: p90 0.084 · p95 0.092 · p99 0.098 · 5913 op/s · total p50 0.661</sub> | -70.3% (-0.131) | 150% AND 2 ms | 🟢 |
| 8 | 0.307<br><sub>context: p90 0.448 · p95 0.503 · p99 0.624 · 18958 op/s · total p50 1.639</sub> | 0.065<br><sub>context: p90 0.100 · p95 0.107 · p99 0.121 · 34148 op/s · total p50 0.883</sub> | -78.6% (-0.241) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.309<br><sub>context: p90 0.382 · p95 0.406 · p99 0.457 · 2669 op/s · total p50 1.477</sub> | 0.134<br><sub>context: p90 0.216 · p95 0.241 · p99 0.280 · 3462 op/s · total p50 1.118</sub> | -56.7% (-0.175) | 150% AND 2 ms | 🟢 |
| 8 | 0.413<br><sub>context: p90 0.581 · p95 0.644 · p99 0.797 · 14111 op/s · total p50 2.178</sub> | 0.164<br><sub>context: p90 0.255 · p95 0.292 · p99 0.350 · 22019 op/s · total p50 1.353</sub> | -60.3% (-0.249) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.174<br><sub>context: p90 0.216 · p95 0.225 · p99 0.238 · 4632 op/s · total p50 0.853</sub> | 0.022<br><sub>context: p90 0.032 · p95 0.038 · p99 0.049 · 8400 op/s · total p50 0.451</sub> | -87.4% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.417 · p95 0.480 · p99 0.584 · 20660 op/s · total p50 1.502</sub> | 0.025<br><sub>context: p90 0.034 · p95 0.037 · p99 0.044 · 42155 op/s · total p50 0.659</sub> | -90.5% (-0.241) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.159 · p95 0.170 · p99 0.189 · 5904 op/s · total p50 0.664</sub> | 0.037<br><sub>context: p90 0.066 · p95 0.071 · p99 0.075 · 7999 op/s · total p50 0.490</sub> | -69.8% (-0.085) | 150% AND 2 ms | 🟢 |
| 8 | 0.183<br><sub>context: p90 0.293 · p95 0.337 · p99 0.446 · 29729 op/s · total p50 1.027</sub> | 0.037<br><sub>context: p90 0.052 · p95 0.069 · p99 0.071 · 45999 op/s · total p50 0.516</sub> | -79.9% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.121<br><sub>context: p90 0.147 · p95 0.162 · p99 0.185 · 6272 op/s · total p50 0.624</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.071 · p99 0.074 · 8103 op/s · total p50 0.483</sub> | -71.7% (-0.087) | 150% AND 2 ms | 🟢 |
| 8 | 0.186<br><sub>context: p90 0.309 · p95 0.350 · p99 0.446 · 29021 op/s · total p50 1.050</sub> | 0.038<br><sub>context: p90 0.071 · p95 0.074 · p99 0.082 · 47952 op/s · total p50 0.502</sub> | -79.8% (-0.149) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.833<br><sub>context: p90 0.863 · p95 0.876 · p99 0.898 · 1111 op/s · total p50 3.584</sub> | 0.201<br><sub>context: p90 0.227 · p95 0.233 · p99 0.244 · 3241 op/s · total p50 1.219</sub> | -75.9% (-0.632) | 150% AND 2 ms | 🟢 |
| 8 | 0.930<br><sub>context: p90 1.565 · p95 1.706 · p99 1.962 · 6093 op/s · total p50 4.938</sub> | 0.232<br><sub>context: p90 0.257 · p95 0.268 · p99 0.302 · 19142 op/s · total p50 1.606</sub> | -75.1% (-0.699) | 150% AND 2 ms | 🟢 |

</details>
