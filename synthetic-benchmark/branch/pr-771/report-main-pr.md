### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
| workload_hash | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` | `sha256:c51a7926ad110d35c36af442710c9b16a29099ecac19ebaaccab614e996f085d` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 10% | 0.5 ms |
| `expand_hops_5` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `match_by_index` | 15% | 0.5 ms |
| `property_projection` | 15% | 0.5 ms |
| `return_const` | 15% | 0.5 ms |
| `shortest_path` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `single_edge_update` | 25% | 0.5 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**pr vs main** — 🟢 no p50 regression beyond budget across 100 comparable cell(s)

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.479<br><sub>context: p90 0.517 · p95 0.526 · p99 0.537 · 1256 op/s · total p50 0.782</sub> | 0.468<br><sub>context: p90 0.497 · p95 0.518 · p99 0.532 · 1418 op/s · total p50 0.685</sub> | -2.2% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.496<br><sub>context: p90 0.553 · p95 0.576 · p99 0.613 · 8831 op/s · total p50 0.834</sub> | 0.502<br><sub>context: p90 0.567 · p95 0.592 · p99 0.624 · 9144 op/s · total p50 0.819</sub> | +1.2% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.525<br><sub>context: p90 0.560 · p95 0.568 · p99 0.581 · 1219 op/s · total p50 0.803</sub> | 0.520<br><sub>context: p90 0.556 · p95 0.570 · p99 0.587 · 1290 op/s · total p50 0.758</sub> | -1.0% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.553<br><sub>context: p90 0.647 · p95 0.681 · p99 0.738 · 8688 op/s · total p50 0.857</sub> | 0.559<br><sub>context: p90 0.653 · p95 0.682 · p99 0.730 · 8834 op/s · total p50 0.857</sub> | +1.0% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.905<br><sub>context: p90 0.945 · p95 0.959 · p99 0.981 · 834 op/s · total p50 1.176</sub> | 0.916<br><sub>context: p90 0.963 · p95 0.976 · p99 0.996 · 816 op/s · total p50 1.204</sub> | +1.2% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.796<br><sub>context: p90 2.792 · p95 3.089 · p99 3.537 · 3511 op/s · total p50 2.172</sub> | 1.746<br><sub>context: p90 2.809 · p95 3.118 · p99 3.619 · 3411 op/s · total p50 2.172</sub> | -2.8% (-0.050) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.958<br><sub>context: p90 0.993 · p95 1.006 · p99 1.026 · 791 op/s · total p50 1.233</sub> | 0.955<br><sub>context: p90 0.975 · p95 0.985 · p99 0.991 · 819 op/s · total p50 1.209</sub> | -0.4% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.985<br><sub>context: p90 1.047 · p95 1.073 · p99 1.114 · 5435 op/s · total p50 1.344</sub> | 0.985<br><sub>context: p90 1.087 · p95 1.127 · p99 1.177 · 5474 op/s · total p50 1.349</sub> | +0.1% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.013<br><sub>context: p90 0.018 · p95 0.018 · p99 0.021 · 6772 op/s · total p50 0.146</sub> | 0.014<br><sub>context: p90 0.017 · p95 0.018 · p99 0.022 · 6143 op/s · total p50 0.159</sub> | +7.5% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.016<br><sub>context: p90 0.026 · p95 0.029 · p99 0.039 · 32860 op/s · total p50 0.225</sub> | 0.018<br><sub>context: p90 0.028 · p95 0.031 · p99 0.039 · 32242 op/s · total p50 0.238</sub> | +9.9% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.044<br><sub>context: p90 0.068 · p95 0.070 · p99 0.086 · 3554 op/s · total p50 0.215</sub> | 0.045<br><sub>context: p90 0.072 · p95 0.075 · p99 0.081 · 4572 op/s · total p50 0.213</sub> | +3.8% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.051<br><sub>context: p90 0.077 · p95 0.084 · p99 0.093 · 24518 op/s · total p50 0.307</sub> | 0.051<br><sub>context: p90 0.076 · p95 0.083 · p99 0.096 · 23067 op/s · total p50 0.337</sub> | -1.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.051<br><sub>context: p90 0.078 · p95 0.084 · p99 0.107 · 3430 op/s · total p50 0.260</sub> | 0.047<br><sub>context: p90 0.075 · p95 0.080 · p99 0.089 · 3876 op/s · total p50 0.239</sub> | -8.8% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.053<br><sub>context: p90 0.083 · p95 0.089 · p99 0.102 · 22262 op/s · total p50 0.338</sub> | 0.052<br><sub>context: p90 0.079 · p95 0.086 · p99 0.096 · 23306 op/s · total p50 0.328</sub> | -2.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.101<br><sub>context: p90 0.142 · p95 0.150 · p99 0.166 · 2566 op/s · total p50 0.351</sub> | 0.111<br><sub>context: p90 0.150 · p95 0.162 · p99 0.175 · 2420 op/s · total p50 0.407</sub> | +9.7% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.108<br><sub>context: p90 0.146 · p95 0.160 · p99 0.179 · 16542 op/s · total p50 0.447</sub> | 0.106<br><sub>context: p90 0.142 · p95 0.153 · p99 0.177 · 16421 op/s · total p50 0.466</sub> | -2.3% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.115<br><sub>context: p90 0.157 · p95 0.174 · p99 0.192 · 2149 op/s · total p50 0.430</sub> | 0.110<br><sub>context: p90 0.157 · p95 0.169 · p99 0.193 · 2263 op/s · total p50 0.428</sub> | -4.2% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.114<br><sub>context: p90 0.152 · p95 0.164 · p99 0.187 · 15060 op/s · total p50 0.506</sub> | 0.111<br><sub>context: p90 0.147 · p95 0.160 · p99 0.184 · 15630 op/s · total p50 0.493</sub> | -2.3% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.201 · p95 0.213 · p99 0.223 · 1677 op/s · total p50 0.567</sub> | 0.144<br><sub>context: p90 0.184 · p95 0.198 · p99 0.214 · 1803 op/s · total p50 0.541</sub> | -8.0% (-0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.154<br><sub>context: p90 0.200 · p95 0.216 · p99 0.247 · 11540 op/s · total p50 0.648</sub> | 0.152<br><sub>context: p90 0.198 · p95 0.211 · p99 0.239 · 12222 op/s · total p50 0.629</sub> | -1.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.149<br><sub>context: p90 0.210 · p95 0.222 · p99 0.239 · 1670 op/s · total p50 0.558</sub> | 0.141<br><sub>context: p90 0.194 · p95 0.214 · p99 0.236 · 1896 op/s · total p50 0.517</sub> | -5.5% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.163<br><sub>context: p90 0.211 · p95 0.224 · p99 0.252 · 11293 op/s · total p50 0.675</sub> | 0.161<br><sub>context: p90 0.207 · p95 0.220 · p99 0.247 · 11161 op/s · total p50 0.677</sub> | -1.3% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.263<br><sub>context: p90 0.328 · p95 0.342 · p99 0.373 · 981 op/s · total p50 1.008</sub> | 0.248<br><sub>context: p90 0.310 · p95 0.342 · p99 0.378 · 1010 op/s · total p50 0.962</sub> | -6.0% (-0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.288<br><sub>context: p90 0.377 · p95 0.404 · p99 0.440 · 6305 op/s · total p50 1.207</sub> | 0.287<br><sub>context: p90 0.375 · p95 0.398 · p99 0.435 · 6311 op/s · total p50 1.213</sub> | -0.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.263<br><sub>context: p90 0.338 · p95 0.363 · p99 0.417 · 994 op/s · total p50 0.987</sub> | 0.276<br><sub>context: p90 0.357 · p95 0.378 · p99 0.400 · 901 op/s · total p50 1.055</sub> | +5.1% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.392 · p95 0.418 · p99 0.465 · 6119 op/s · total p50 1.247</sub> | 0.299<br><sub>context: p90 0.399 · p95 0.422 · p99 0.469 · 6205 op/s · total p50 1.234</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.309<br><sub>context: p90 0.477 · p95 0.514 · p99 0.578 · 1587 op/s · total p50 0.616</sub> | 0.347<br><sub>context: p90 0.484 · p95 0.545 · p99 0.619 · 1309 op/s · total p50 0.724</sub> | +12.3% (+0.038) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.499<br><sub>context: p90 0.785 · p95 0.861 · p99 1.016 · 9105 op/s · total p50 0.836</sub> | 0.485<br><sub>context: p90 0.761 · p95 0.849 · p99 0.972 · 8848 op/s · total p50 0.850</sub> | -2.8% (-0.014) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.080<br><sub>context: p90 0.116 · p95 0.131 · p99 0.151 · 2998 op/s · total p50 0.301</sub> | 0.088<br><sub>context: p90 0.127 · p95 0.142 · p99 0.155 · 2471 op/s · total p50 0.333</sub> | +11.1% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.089<br><sub>context: p90 0.123 · p95 0.134 · p99 0.157 · 19039 op/s · total p50 0.400</sub> | 0.090<br><sub>context: p90 0.128 · p95 0.140 · p99 0.159 · 17940 op/s · total p50 0.418</sub> | +1.9% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 7439 op/s · total p50 0.133</sub> | 0.002<br><sub>context: p90 0.006 · p95 0.007 · p99 0.007 · 5042 op/s · total p50 0.165</sub> | +39.1% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 31874 op/s · total p50 0.241</sub> | 0.002<br><sub>context: p90 0.004 · p95 0.004 · p99 0.006 · 31302 op/s · total p50 0.242</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.001<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 7821 op/s · total p50 0.120</sub> | 0.002<br><sub>context: p90 0.006 · p95 0.007 · p99 0.008 · 5800 op/s · total p50 0.155</sub> | +28.3% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 35387 op/s · total p50 0.214</sub> | 0.002<br><sub>context: p90 0.004 · p95 0.006 · p99 0.007 · 30887 op/s · total p50 0.224</sub> | +4.9% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.070<br><sub>context: p90 0.095 · p95 0.100 · p99 0.113 · 2700 op/s · total p50 0.359</sub> | 0.068<br><sub>context: p90 0.101 · p95 0.114 · p99 0.123 · 2436 op/s · total p50 0.357</sub> | -3.4% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.078<br><sub>context: p90 0.108 · p95 0.115 · p99 0.127 · 16844 op/s · total p50 0.452</sub> | 0.077<br><sub>context: p90 0.110 · p95 0.119 · p99 0.134 · 16356 op/s · total p50 0.461</sub> | -0.7% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.984<br><sub>context: p90 1.614 · p95 1.810 · p99 2.076 · 732 op/s · total p50 1.318</sub> | 1.061<br><sub>context: p90 1.605 · p95 1.837 · p99 2.181 · 643 op/s · total p50 1.501</sub> | +7.8% (+0.077) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.820<br><sub>context: p90 4.751 · p95 5.374 · p99 6.195 · 2325 op/s · total p50 3.255</sub> | 2.828<br><sub>context: p90 4.651 · p95 5.196 · p99 6.007 · 2347 op/s · total p50 3.269</sub> | +0.3% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.005<br><sub>context: p90 6.239 · p95 6.906 · p99 7.574 · 217 op/s · total p50 4.428</sub> | 4.088<br><sub>context: p90 6.304 · p95 6.969 · p99 7.501 · 208 op/s · total p50 4.706</sub> | +2.1% (+0.083) | 10% AND 0.5 ms | 🟢 |
| 8 | 12.428<br><sub>context: p90 18.739 · p95 20.598 · p99 22.470 · 607 op/s · total p50 12.876</sub> | 12.619<br><sub>context: p90 19.390 · p95 21.202 · p99 23.476 · 584 op/s · total p50 13.216</sub> | +1.5% (+0.190) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.016<br><sub>context: p90 0.021 · p95 0.024 · p99 0.030 · 4843 op/s · total p50 0.189</sub> | 0.017<br><sub>context: p90 0.044 · p95 0.045 · p99 0.055 · 4188 op/s · total p50 0.190</sub> | +6.2% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.018<br><sub>context: p90 0.024 · p95 0.026 · p99 0.035 · 28934 op/s · total p50 0.261</sub> | 0.020<br><sub>context: p90 0.034 · p95 0.042 · p99 0.050 · 23652 op/s · total p50 0.287</sub> | +13.5% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.014<br><sub>context: p90 0.018 · p95 0.020 · p99 0.024 · 5245 op/s · total p50 0.172</sub> | 0.013<br><sub>context: p90 0.043 · p95 0.046 · p99 0.049 · 4317 op/s · total p50 0.187</sub> | -12.7% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.017<br><sub>context: p90 0.023 · p95 0.026 · p99 0.032 · 28622 op/s · total p50 0.263</sub> | 0.017<br><sub>context: p90 0.023 · p95 0.027 · p99 0.034 · 28842 op/s · total p50 0.263</sub> | +1.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.009<br><sub>context: p90 0.013 · p95 0.015 · p99 0.017 · 3870 op/s · total p50 0.240</sub> | 0.010<br><sub>context: p90 0.014 · p95 0.015 · p99 0.020 · 4127 op/s · total p50 0.228</sub> | +9.3% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.011<br><sub>context: p90 0.016 · p95 0.018 · p99 0.021 · 21756 op/s · total p50 0.348</sub> | 0.011<br><sub>context: p90 0.017 · p95 0.019 · p99 0.024 · 21021 op/s · total p50 0.357</sub> | +4.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.006<br><sub>context: p90 0.015 · p95 0.016 · p99 0.018 · 5908 op/s · total p50 0.143</sub> | 0.005<br><sub>context: p90 0.007 · p95 0.007 · p99 0.009 · 6268 op/s · total p50 0.133</sub> | -9.4% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.012 · 33251 op/s · total p50 0.229</sub> | 0.006<br><sub>context: p90 0.010 · p95 0.014 · p99 0.017 · 28915 op/s · total p50 0.242</sub> | +4.7% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.125<br><sub>context: p90 0.160 · p95 0.175 · p99 0.194 · 2894 op/s · total p50 0.336</sub> | 0.130<br><sub>context: p90 0.175 · p95 0.184 · p99 0.202 · 2343 op/s · total p50 0.356</sub> | +4.1% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.130<br><sub>context: p90 0.168 · p95 0.180 · p99 0.206 · 20013 op/s · total p50 0.384</sub> | 0.131<br><sub>context: p90 0.171 · p95 0.182 · p99 0.206 · 18899 op/s · total p50 0.393</sub> | +0.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.063<br><sub>context: p90 0.091 · p95 0.108 · p99 0.114 · 3901 op/s · total p50 0.236</sub> | 0.064<br><sub>context: p90 0.092 · p95 0.104 · p99 0.118 · 3718 op/s · total p50 0.244</sub> | +1.8% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.066<br><sub>context: p90 0.095 · p95 0.103 · p99 0.122 · 24661 op/s · total p50 0.310</sub> | 0.068<br><sub>context: p90 0.097 · p95 0.106 · p99 0.123 · 23858 op/s · total p50 0.313</sub> | +1.7% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.076<br><sub>context: p90 0.108 · p95 0.113 · p99 0.132 · 3346 op/s · total p50 0.279</sub> | 0.074<br><sub>context: p90 0.100 · p95 0.111 · p99 0.128 · 3238 op/s · total p50 0.298</sub> | -1.7% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.101<br><sub>context: p90 0.135 · p95 0.147 · p99 0.171 · 17337 op/s · total p50 0.446</sub> | 0.102<br><sub>context: p90 0.135 · p95 0.145 · p99 0.164 · 16932 op/s · total p50 0.455</sub> | +0.9% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.085<br><sub>context: p90 0.109 · p95 0.117 · p99 0.124 · 2363 op/s · total p50 0.418</sub> | 0.082<br><sub>context: p90 0.125 · p95 0.133 · p99 0.167 · 2194 op/s · total p50 0.434</sub> | -3.5% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.108<br><sub>context: p90 0.145 · p95 0.156 · p99 0.179 · 13214 op/s · total p50 0.582</sub> | 0.109<br><sub>context: p90 0.144 · p95 0.153 · p99 0.172 · 12672 op/s · total p50 0.608</sub> | +1.2% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.094<br><sub>context: p90 0.120 · p95 0.127 · p99 0.141 · 2230 op/s · total p50 0.439</sub> | 0.099<br><sub>context: p90 0.135 · p95 0.141 · p99 0.154 · 2130 op/s · total p50 0.462</sub> | +5.5% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.115<br><sub>context: p90 0.152 · p95 0.163 · p99 0.183 · 12534 op/s · total p50 0.616</sub> | 0.113<br><sub>context: p90 0.149 · p95 0.158 · p99 0.177 · 12512 op/s · total p50 0.609</sub> | -1.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.082<br><sub>context: p90 0.113 · p95 0.129 · p99 0.139 · 2904 op/s · total p50 0.329</sub> | 0.087<br><sub>context: p90 0.115 · p95 0.125 · p99 0.137 · 2925 op/s · total p50 0.336</sub> | +6.2% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.108<br><sub>context: p90 0.137 · p95 0.147 · p99 0.171 · 15217 op/s · total p50 0.511</sub> | 0.106<br><sub>context: p90 0.138 · p95 0.148 · p99 0.166 · 15520 op/s · total p50 0.500</sub> | -1.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.068<br><sub>context: p90 0.106 · p95 0.115 · p99 0.143 · 3261 op/s · total p50 0.280</sub> | 0.103<br><sub>context: p90 0.133 · p95 0.147 · p99 0.158 · 2437 op/s · total p50 0.398</sub> | +50.8% (+0.035) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.084<br><sub>context: p90 0.116 · p95 0.127 · p99 0.145 · 19575 op/s · total p50 0.388</sub> | 0.085<br><sub>context: p90 0.119 · p95 0.130 · p99 0.154 · 19730 op/s · total p50 0.383</sub> | +1.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.537<br><sub>context: p90 2.583 · p95 2.602 · p99 2.631 · 73 op/s · total p50 13.567</sub> | 2.528<br><sub>context: p90 2.572 · p95 2.589 · p99 2.633 · 74 op/s · total p50 13.515</sub> | -0.3% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 3.184<br><sub>context: p90 3.469 · p95 3.509 · p99 3.788 · 455 op/s · total p50 16.560</sub> | 3.148<br><sub>context: p90 3.445 · p95 3.490 · p99 3.740 · 471 op/s · total p50 16.033</sub> | -1.1% (-0.036) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.097<br><sub>context: p90 0.133 · p95 0.141 · p99 0.150 · 2828 op/s · total p50 0.338</sub> | 0.096<br><sub>context: p90 0.127 · p95 0.133 · p99 0.146 · 2834 op/s · total p50 0.340</sub> | -0.7% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.114<br><sub>context: p90 0.147 · p95 0.155 · p99 0.173 · 16196 op/s · total p50 0.472</sub> | 0.114<br><sub>context: p90 0.145 · p95 0.154 · p99 0.174 · 15495 op/s · total p50 0.499</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.296 · p95 0.309 · p99 0.354 · 856 op/s · total p50 1.141</sub> | 0.230<br><sub>context: p90 0.295 · p95 0.310 · p99 0.361 · 842 op/s · total p50 1.187</sub> | +3.3% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.270<br><sub>context: p90 0.356 · p95 0.375 · p99 0.416 · 5302 op/s · total p50 1.457</sub> | 0.270<br><sub>context: p90 0.348 · p95 0.368 · p99 0.406 · 5345 op/s · total p50 1.465</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.090<br><sub>context: p90 0.124 · p95 0.131 · p99 0.152 · 2916 op/s · total p50 0.336</sub> | 0.091<br><sub>context: p90 0.131 · p95 0.145 · p99 0.163 · 2655 op/s · total p50 0.357</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.107<br><sub>context: p90 0.143 · p95 0.150 · p99 0.169 · 16564 op/s · total p50 0.462</sub> | 0.109<br><sub>context: p90 0.144 · p95 0.156 · p99 0.183 · 15761 op/s · total p50 0.493</sub> | +1.1% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.109<br><sub>context: p90 0.157 · p95 0.166 · p99 0.192 · 2962 op/s · total p50 0.337</sub> | 0.108<br><sub>context: p90 0.152 · p95 0.166 · p99 0.193 · 2744 op/s · total p50 0.347</sub> | -0.2% (-0.000) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.116<br><sub>context: p90 0.165 · p95 0.180 · p99 0.211 · 19371 op/s · total p50 0.398</sub> | 0.116<br><sub>context: p90 0.165 · p95 0.177 · p99 0.203 · 18805 op/s · total p50 0.404</sub> | -0.0% (-0.000) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.109<br><sub>context: p90 0.163 · p95 0.176 · p99 0.206 · 2785 op/s · total p50 0.346</sub> | 0.112<br><sub>context: p90 0.161 · p95 0.180 · p99 0.206 · 2785 op/s · total p50 0.352</sub> | +3.1% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.119<br><sub>context: p90 0.176 · p95 0.195 · p99 0.222 · 18666 op/s · total p50 0.410</sub> | 0.121<br><sub>context: p90 0.174 · p95 0.192 · p99 0.228 · 17086 op/s · total p50 0.454</sub> | +1.4% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.032<br><sub>context: p90 0.056 · p95 0.061 · p99 0.079 · 4413 op/s · total p50 0.192</sub> | 0.033<br><sub>context: p90 0.057 · p95 0.060 · p99 0.071 · 4773 op/s · total p50 0.200</sub> | +2.7% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.036<br><sub>context: p90 0.064 · p95 0.070 · p99 0.082 · 23678 op/s · total p50 0.285</sub> | 0.034<br><sub>context: p90 0.062 · p95 0.066 · p99 0.073 · 28219 op/s · total p50 0.271</sub> | -3.7% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.004<br><sub>context: p90 0.004 · p95 0.005 · p99 0.005 · 5987 op/s · total p50 0.160</sub> | 0.004<br><sub>context: p90 0.011 · p95 0.011 · p99 0.012 · 4109 op/s · total p50 0.198</sub> | +7.2% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 28689 op/s · total p50 0.270</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.008 · 28582 op/s · total p50 0.265</sub> | -0.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.044<br><sub>context: p90 0.070 · p95 0.076 · p99 0.091 · 3741 op/s · total p50 0.245</sub> | 0.046<br><sub>context: p90 0.075 · p95 0.089 · p99 0.093 · 2890 op/s · total p50 0.265</sub> | +5.8% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.045<br><sub>context: p90 0.074 · p95 0.077 · p99 0.083 · 23414 op/s · total p50 0.325</sub> | 0.046<br><sub>context: p90 0.074 · p95 0.079 · p99 0.092 · 21578 op/s · total p50 0.338</sub> | +2.4% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.059<br><sub>context: p90 0.112 · p95 0.117 · p99 0.136 · 3438 op/s · total p50 0.256</sub> | 0.061<br><sub>context: p90 0.115 · p95 0.121 · p99 0.140 · 3104 op/s · total p50 0.248</sub> | +3.3% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.065<br><sub>context: p90 0.125 · p95 0.132 · p99 0.141 · 22962 op/s · total p50 0.328</sub> | 0.065<br><sub>context: p90 0.125 · p95 0.132 · p99 0.141 · 23011 op/s · total p50 0.330</sub> | -1.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.034<br><sub>context: p90 0.060 · p95 0.066 · p99 0.068 · 4779 op/s · total p50 0.203</sub> | 0.038<br><sub>context: p90 0.066 · p95 0.079 · p99 0.085 · 4006 op/s · total p50 0.225</sub> | +10.2% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.038<br><sub>context: p90 0.067 · p95 0.071 · p99 0.079 · 26069 op/s · total p50 0.293</sub> | 0.038<br><sub>context: p90 0.067 · p95 0.071 · p99 0.084 · 25559 op/s · total p50 0.286</sub> | -0.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.491<br><sub>context: p90 0.530 · p95 0.549 · p99 0.574 · 1108 op/s · total p50 0.876</sub> | 0.504<br><sub>context: p90 0.540 · p95 0.557 · p99 0.568 · 1094 op/s · total p50 0.905</sub> | +2.6% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.566<br><sub>context: p90 0.671 · p95 0.711 · p99 0.812 · 6826 op/s · total p50 1.122</sub> | 0.540<br><sub>context: p90 0.601 · p95 0.632 · p99 0.672 · 6196 op/s · total p50 1.200</sub> | -4.6% (-0.026) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.491<br><sub>context: p90 0.526 · p95 0.535 · p99 0.553 · 1313 op/s · total p50 0.749</sub> | 0.493<br><sub>context: p90 0.534 · p95 0.545 · p99 0.584 · 1278 op/s · total p50 0.773</sub> | +0.5% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.539<br><sub>context: p90 0.630 · p95 0.664 · p99 0.710 · 7828 op/s · total p50 0.973</sub> | 0.540<br><sub>context: p90 0.610 · p95 0.634 · p99 0.683 · 7809 op/s · total p50 0.946</sub> | +0.1% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.058<br><sub>context: p90 0.084 · p95 0.088 · p99 0.097 · 3559 op/s · total p50 0.273</sub> | 0.075<br><sub>context: p90 0.104 · p95 0.111 · p99 0.122 · 2562 op/s · total p50 0.360</sub> | +28.9% (+0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.065<br><sub>context: p90 0.093 · p95 0.100 · p99 0.113 · 20878 op/s · total p50 0.368</sub> | 0.066<br><sub>context: p90 0.096 · p95 0.104 · p99 0.122 · 19308 op/s · total p50 0.388</sub> | +1.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.237 · p95 0.258 · p99 0.282 · 2398 op/s · total p50 0.409</sub> | 0.189<br><sub>context: p90 0.273 · p95 0.294 · p99 0.350 · 1784 op/s · total p50 0.535</sub> | +21.2% (+0.033) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.200<br><sub>context: p90 0.314 · p95 0.350 · p99 0.400 · 15259 op/s · total p50 0.495</sub> | 0.203<br><sub>context: p90 0.324 · p95 0.358 · p99 0.424 · 14548 op/s · total p50 0.503</sub> | +1.6% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.024<br><sub>context: p90 0.031 · p95 0.035 · p99 0.038 · 4484 op/s · total p50 0.210</sub> | 0.029<br><sub>context: p90 0.061 · p95 0.063 · p99 0.078 · 3580 op/s · total p50 0.243</sub> | +20.7% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.026<br><sub>context: p90 0.036 · p95 0.040 · p99 0.050 · 25315 op/s · total p50 0.300</sub> | 0.026<br><sub>context: p90 0.035 · p95 0.039 · p99 0.048 · 25563 op/s · total p50 0.298</sub> | -1.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.031<br><sub>context: p90 0.056 · p95 0.061 · p99 0.063 · 5100 op/s · total p50 0.190</sub> | 0.031<br><sub>context: p90 0.056 · p95 0.058 · p99 0.065 · 5110 op/s · total p50 0.186</sub> | -0.2% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.034<br><sub>context: p90 0.062 · p95 0.066 · p99 0.071 · 28164 op/s · total p50 0.269</sub> | 0.034<br><sub>context: p90 0.061 · p95 0.065 · p99 0.071 · 28562 op/s · total p50 0.270</sub> | -0.9% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.032<br><sub>context: p90 0.059 · p95 0.062 · p99 0.065 · 5358 op/s · total p50 0.183</sub> | 0.031<br><sub>context: p90 0.056 · p95 0.058 · p99 0.060 · 4813 op/s · total p50 0.195</sub> | -4.2% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.034<br><sub>context: p90 0.063 · p95 0.066 · p99 0.072 · 29714 op/s · total p50 0.258</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.065 · p99 0.072 · 28105 op/s · total p50 0.273</sub> | -0.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.169<br><sub>context: p90 0.177 · p95 0.181 · p99 0.194 · 2621 op/s · total p50 0.368</sub> | 0.175<br><sub>context: p90 0.198 · p95 0.204 · p99 0.216 · 2482 op/s · total p50 0.381</sub> | +3.4% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.189<br><sub>context: p90 0.218 · p95 0.224 · p99 0.241 · 17544 op/s · total p50 0.440</sub> | 0.187<br><sub>context: p90 0.220 · p95 0.227 · p99 0.245 · 17975 op/s · total p50 0.429</sub> | -1.3% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>
