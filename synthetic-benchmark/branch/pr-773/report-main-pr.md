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
| 1 | 0.595<br><sub>context: p90 0.631 · p95 0.643 · p99 0.655 · 1377 op/s · total p50 2.884</sub> | 0.601<br><sub>context: p90 0.631 · p95 0.641 · p99 0.657 · 1364 op/s · total p50 2.894</sub> | +1.0% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.634<br><sub>context: p90 0.702 · p95 0.727 · p99 0.780 · 8885 op/s · total p50 3.507</sub> | 0.634<br><sub>context: p90 0.699 · p95 0.727 · p99 0.770 · 8903 op/s · total p50 3.508</sub> | -0.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.629<br><sub>context: p90 0.665 · p95 0.684 · p99 0.717 · 1313 op/s · total p50 3.044</sub> | 0.622<br><sub>context: p90 0.656 · p95 0.671 · p99 0.688 · 1350 op/s · total p50 2.952</sub> | -1.1% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.665<br><sub>context: p90 0.740 · p95 0.771 · p99 0.834 · 8654 op/s · total p50 3.517</sub> | 0.677<br><sub>context: p90 0.735 · p95 0.763 · p99 0.820 · 8436 op/s · total p50 3.722</sub> | +1.7% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.105<br><sub>context: p90 1.142 · p95 1.153 · p99 1.176 · 790 op/s · total p50 5.038</sub> | 1.115<br><sub>context: p90 1.150 · p95 1.161 · p99 1.175 · 773 op/s · total p50 5.166</sub> | +0.9% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.962<br><sub>context: p90 2.648 · p95 2.833 · p99 3.164 · 3571 op/s · total p50 8.654</sub> | 1.944<br><sub>context: p90 2.728 · p95 2.919 · p99 3.344 · 3578 op/s · total p50 8.819</sub> | -0.9% (-0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.179<br><sub>context: p90 1.222 · p95 1.236 · p99 1.249 · 753 op/s · total p50 5.298</sub> | 1.191<br><sub>context: p90 1.232 · p95 1.242 · p99 1.286 · 736 op/s · total p50 5.423</sub> | +1.0% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.225<br><sub>context: p90 1.305 · p95 1.347 · p99 1.419 · 4870 op/s · total p50 6.228</sub> | 1.257<br><sub>context: p90 1.413 · p95 1.455 · p99 1.547 · 5288 op/s · total p50 5.876</sub> | +2.6% (+0.032) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.012<br><sub>context: p90 0.020 · p95 0.023 · p99 0.029 · 12872 op/s · total p50 0.274</sub> | 0.014<br><sub>context: p90 0.019 · p95 0.022 · p99 0.027 · 13252 op/s · total p50 0.279</sub> | +17.1% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.018<br><sub>context: p90 0.026 · p95 0.029 · p99 0.035 · 50305 op/s · total p50 0.468</sub> | 0.017<br><sub>context: p90 0.026 · p95 0.029 · p99 0.036 · 52310 op/s · total p50 0.458</sub> | -3.6% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.051<br><sub>context: p90 0.084 · p95 0.090 · p99 0.096 · 6288 op/s · total p50 0.623</sub> | 0.056<br><sub>context: p90 0.088 · p95 0.094 · p99 0.103 · 5374 op/s · total p50 0.715</sub> | +9.8% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.060<br><sub>context: p90 0.096 · p95 0.103 · p99 0.119 · 31753 op/s · total p50 0.925</sub> | 0.056<br><sub>context: p90 0.090 · p95 0.095 · p99 0.105 · 34347 op/s · total p50 0.876</sub> | -6.1% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.052<br><sub>context: p90 0.079 · p95 0.085 · p99 0.100 · 5101 op/s · total p50 0.771</sub> | 0.060<br><sub>context: p90 0.093 · p95 0.101 · p99 0.108 · 4496 op/s · total p50 0.871</sub> | +15.1% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.062<br><sub>context: p90 0.096 · p95 0.102 · p99 0.116 · 29808 op/s · total p50 1.008</sub> | 0.060<br><sub>context: p90 0.096 · p95 0.100 · p99 0.116 · 31387 op/s · total p50 0.957</sub> | -1.8% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.134 · p95 0.138 · p99 0.160 · 3973 op/s · total p50 0.980</sub> | 0.119<br><sub>context: p90 0.146 · p95 0.162 · p99 0.179 · 3280 op/s · total p50 1.188</sub> | +15.5% (+0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.112<br><sub>context: p90 0.147 · p95 0.160 · p99 0.188 · 25015 op/s · total p50 1.191</sub> | 0.113<br><sub>context: p90 0.152 · p95 0.168 · p99 0.196 · 24823 op/s · total p50 1.202</sub> | +0.8% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.144 · p95 0.158 · p99 0.175 · 3747 op/s · total p50 1.034</sub> | 0.107<br><sub>context: p90 0.143 · p95 0.160 · p99 0.170 · 3745 op/s · total p50 1.055</sub> | +3.0% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.120<br><sub>context: p90 0.158 · p95 0.166 · p99 0.184 · 23170 op/s · total p50 1.291</sub> | 0.118<br><sub>context: p90 0.159 · p95 0.171 · p99 0.194 · 21363 op/s · total p50 1.339</sub> | -1.8% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.134<br><sub>context: p90 0.182 · p95 0.193 · p99 0.208 · 3388 op/s · total p50 1.166</sub> | 0.144<br><sub>context: p90 0.182 · p95 0.195 · p99 0.210 · 3101 op/s · total p50 1.261</sub> | +7.6% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.162<br><sub>context: p90 0.213 · p95 0.230 · p99 0.255 · 14517 op/s · total p50 2.059</sub> | 0.157<br><sub>context: p90 0.210 · p95 0.225 · p99 0.250 · 14592 op/s · total p50 2.078</sub> | -2.7% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.142<br><sub>context: p90 0.192 · p95 0.216 · p99 0.228 · 2991 op/s · total p50 1.320</sub> | 0.144<br><sub>context: p90 0.188 · p95 0.205 · p99 0.235 · 2935 op/s · total p50 1.322</sub> | +1.4% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.170<br><sub>context: p90 0.231 · p95 0.251 · p99 0.288 · 12895 op/s · total p50 2.347</sub> | 0.173<br><sub>context: p90 0.230 · p95 0.246 · p99 0.276 · 13385 op/s · total p50 2.253</sub> | +1.2% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.257<br><sub>context: p90 0.345 · p95 0.368 · p99 0.416 · 1797 op/s · total p50 2.049</sub> | 0.275<br><sub>context: p90 0.361 · p95 0.371 · p99 0.391 · 1668 op/s · total p50 2.205</sub> | +6.8% (+0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.282<br><sub>context: p90 0.379 · p95 0.415 · p99 0.465 · 3855 op/s · total p50 8.130</sub> | 0.293<br><sub>context: p90 0.402 · p95 0.434 · p99 0.479 · 3924 op/s · total p50 7.834</sub> | +3.9% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.282<br><sub>context: p90 0.354 · p95 0.374 · p99 0.407 · 1751 op/s · total p50 2.199</sub> | 0.297<br><sub>context: p90 0.389 · p95 0.415 · p99 0.472 · 1691 op/s · total p50 2.335</sub> | +5.4% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.309<br><sub>context: p90 0.409 · p95 0.436 · p99 0.486 · 3800 op/s · total p50 8.010</sub> | 0.300<br><sub>context: p90 0.401 · p95 0.438 · p99 0.499 · 3707 op/s · total p50 8.334</sub> | -2.8% (-0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.335<br><sub>context: p90 0.509 · p95 0.568 · p99 0.660 · 1949 op/s · total p50 2.037</sub> | 0.340<br><sub>context: p90 0.515 · p95 0.539 · p99 0.679 · 1885 op/s · total p50 2.149</sub> | +1.4% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.667 · p95 0.741 · p99 0.891 · 11129 op/s · total p50 2.710</sub> | 0.438<br><sub>context: p90 0.679 · p95 0.762 · p99 0.934 · 11014 op/s · total p50 2.781</sub> | +1.1% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.075<br><sub>context: p90 0.111 · p95 0.116 · p99 0.147 · 5024 op/s · total p50 0.783</sub> | 0.096<br><sub>context: p90 0.132 · p95 0.142 · p99 0.153 · 3960 op/s · total p50 0.996</sub> | +27.9% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.095<br><sub>context: p90 0.131 · p95 0.139 · p99 0.161 · 26158 op/s · total p50 1.120</sub> | 0.091<br><sub>context: p90 0.127 · p95 0.136 · p99 0.161 · 27002 op/s · total p50 1.079</sub> | -4.1% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.002 · p95 0.003 · p99 0.003 · 12073 op/s · total p50 0.294</sub> | 0.003<br><sub>context: p90 0.006 · p95 0.006 · p99 0.007 · 7538 op/s · total p50 0.511</sub> | +70.9% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 51714 op/s · total p50 0.468</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 50240 op/s · total p50 0.475</sub> | +2.7% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 14900 op/s · total p50 0.249</sub> | 0.002<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 11850 op/s · total p50 0.316</sub> | +12.8% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 53547 op/s · total p50 0.445</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 54275 op/s · total p50 0.437</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.071<br><sub>context: p90 0.097 · p95 0.107 · p99 0.127 · 4168 op/s · total p50 0.926</sub> | 0.080<br><sub>context: p90 0.107 · p95 0.114 · p99 0.126 · 3457 op/s · total p50 1.162</sub> | +12.0% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.085<br><sub>context: p90 0.118 · p95 0.130 · p99 0.147 · 22728 op/s · total p50 1.324</sub> | 0.083<br><sub>context: p90 0.118 · p95 0.127 · p99 0.145 · 23078 op/s · total p50 1.308</sub> | -1.8% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.093<br><sub>context: p90 1.525 · p95 1.618 · p99 1.709 · 799 op/s · total p50 4.968</sub> | 1.076<br><sub>context: p90 1.544 · p95 1.612 · p99 1.773 · 792 op/s · total p50 5.024</sub> | -1.5% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.876<br><sub>context: p90 2.798 · p95 2.991 · p99 3.391 · 3427 op/s · total p50 8.730</sub> | 1.892<br><sub>context: p90 2.809 · p95 3.027 · p99 3.452 · 3412 op/s · total p50 8.747</sub> | +0.8% (+0.015) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.521<br><sub>context: p90 7.402 · p95 7.838 · p99 8.081 · 200 op/s · total p50 19.928</sub> | 4.598<br><sub>context: p90 7.366 · p95 7.861 · p99 8.326 · 196 op/s · total p50 20.156</sub> | +1.7% (+0.077) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.112<br><sub>context: p90 12.671 · p95 13.379 · p99 14.623 · 901 op/s · total p50 34.874</sub> | 8.038<br><sub>context: p90 12.592 · p95 13.371 · p99 14.424 · 907 op/s · total p50 34.647</sub> | -0.9% (-0.074) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.019<br><sub>context: p90 0.023 · p95 0.036 · p99 0.039 · 7950 op/s · total p50 0.473</sub> | 0.019<br><sub>context: p90 0.026 · p95 0.036 · p99 0.043 · 9308 op/s · total p50 0.409</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.020<br><sub>context: p90 0.025 · p95 0.028 · p99 0.032 · 41879 op/s · total p50 0.601</sub> | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.033 · 40760 op/s · total p50 0.662</sub> | -0.8% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.018<br><sub>context: p90 0.021 · p95 0.023 · p99 0.030 · 9272 op/s · total p50 0.405</sub> | 0.019<br><sub>context: p90 0.024 · p95 0.030 · p99 0.036 · 9291 op/s · total p50 0.414</sub> | +6.2% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.019<br><sub>context: p90 0.024 · p95 0.027 · p99 0.030 · 43720 op/s · total p50 0.564</sub> | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.033 · 42308 op/s · total p50 0.613</sub> | +0.9% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.019 · 8404 op/s · total p50 0.463</sub> | 0.010<br><sub>context: p90 0.015 · p95 0.018 · p99 0.020 · 7615 op/s · total p50 0.499</sub> | +5.0% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.011<br><sub>context: p90 0.016 · p95 0.018 · p99 0.022 · 16925 op/s · total p50 1.755</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.017 · p99 0.021 · 17143 op/s · total p50 1.764</sub> | -0.9% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.005<br><sub>context: p90 0.007 · p95 0.007 · p99 0.008 · 12993 op/s · total p50 0.298</sub> | 0.005<br><sub>context: p90 0.008 · p95 0.009 · p99 0.009 · 11237 op/s · total p50 0.327</sub> | -4.7% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.006<br><sub>context: p90 0.009 · p95 0.010 · p99 0.013 · 48833 op/s · total p50 0.502</sub> | 0.006<br><sub>context: p90 0.009 · p95 0.010 · p99 0.012 · 53204 op/s · total p50 0.469</sub> | -3.6% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.143<br><sub>context: p90 0.188 · p95 0.195 · p99 0.213 · 3636 op/s · total p50 1.093</sub> | 0.140<br><sub>context: p90 0.185 · p95 0.193 · p99 0.207 · 3912 op/s · total p50 1.011</sub> | -2.5% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.156<br><sub>context: p90 0.201 · p95 0.218 · p99 0.248 · 24879 op/s · total p50 1.181</sub> | 0.155<br><sub>context: p90 0.203 · p95 0.222 · p99 0.252 · 25123 op/s · total p50 1.162</sub> | -0.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.067<br><sub>context: p90 0.095 · p95 0.102 · p99 0.116 · 5669 op/s · total p50 0.705</sub> | 0.073<br><sub>context: p90 0.107 · p95 0.113 · p99 0.129 · 5150 op/s · total p50 0.759</sub> | +10.1% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.077<br><sub>context: p90 0.114 · p95 0.123 · p99 0.156 · 33653 op/s · total p50 0.880</sub> | 0.078<br><sub>context: p90 0.117 · p95 0.131 · p99 0.159 · 34677 op/s · total p50 0.878</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.096<br><sub>context: p90 0.130 · p95 0.141 · p99 0.171 · 4345 op/s · total p50 0.913</sub> | 0.100<br><sub>context: p90 0.138 · p95 0.149 · p99 0.159 · 4005 op/s · total p50 0.981</sub> | +4.4% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.105<br><sub>context: p90 0.141 · p95 0.151 · p99 0.170 · 27228 op/s · total p50 1.079</sub> | 0.109<br><sub>context: p90 0.148 · p95 0.159 · p99 0.193 · 23498 op/s · total p50 1.238</sub> | +3.6% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.082<br><sub>context: p90 0.119 · p95 0.128 · p99 0.149 · 4082 op/s · total p50 0.960</sub> | 0.111<br><sub>context: p90 0.149 · p95 0.161 · p99 0.175 · 3113 op/s · total p50 1.276</sub> | +34.7% (+0.029) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.111<br><sub>context: p90 0.152 · p95 0.163 · p99 0.190 · 14341 op/s · total p50 2.063</sub> | 0.112<br><sub>context: p90 0.153 · p95 0.165 · p99 0.178 · 13744 op/s · total p50 2.154</sub> | +1.1% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.145 · p95 0.158 · p99 0.175 · 3450 op/s · total p50 1.158</sub> | 0.104<br><sub>context: p90 0.132 · p95 0.147 · p99 0.156 · 3588 op/s · total p50 1.105</sub> | -1.9% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.118<br><sub>context: p90 0.159 · p95 0.172 · p99 0.194 · 15575 op/s · total p50 1.985</sub> | 0.116<br><sub>context: p90 0.155 · p95 0.167 · p99 0.187 · 16154 op/s · total p50 1.849</sub> | -1.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.094<br><sub>context: p90 0.125 · p95 0.134 · p99 0.141 · 4017 op/s · total p50 0.969</sub> | 0.114<br><sub>context: p90 0.145 · p95 0.151 · p99 0.165 · 3277 op/s · total p50 1.214</sub> | +21.1% (+0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.115<br><sub>context: p90 0.154 · p95 0.165 · p99 0.191 · 21539 op/s · total p50 1.415</sub> | 0.113<br><sub>context: p90 0.152 · p95 0.164 · p99 0.188 · 22418 op/s · total p50 1.364</sub> | -2.0% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.075<br><sub>context: p90 0.106 · p95 0.112 · p99 0.141 · 5365 op/s · total p50 0.715</sub> | 0.086<br><sub>context: p90 0.117 · p95 0.126 · p99 0.145 · 4482 op/s · total p50 0.871</sub> | +14.3% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.091<br><sub>context: p90 0.128 · p95 0.137 · p99 0.160 · 28374 op/s · total p50 1.052</sub> | 0.088<br><sub>context: p90 0.126 · p95 0.136 · p99 0.174 · 29292 op/s · total p50 1.021</sub> | -3.4% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.919<br><sub>context: p90 4.270 · p95 4.370 · p99 4.458 · 87 op/s · total p50 45.357</sub> | 2.859<br><sub>context: p90 2.923 · p95 2.934 · p99 2.969 · 92 op/s · total p50 43.653</sub> | -2.1% (-0.060) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.911<br><sub>context: p90 3.963 · p95 4.071 · p99 4.212 · 150 op/s · total p50 206.424</sub> | 2.885<br><sub>context: p90 2.993 · p95 3.044 · p99 3.289 · 159 op/s · total p50 186.694</sub> | -0.9% (-0.027) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.144 · p95 0.158 · p99 0.174 · 3642 op/s · total p50 1.096</sub> | 0.107<br><sub>context: p90 0.139 · p95 0.157 · p99 0.168 · 3618 op/s · total p50 1.101</sub> | +0.9% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.119<br><sub>context: p90 0.157 · p95 0.166 · p99 0.188 · 20907 op/s · total p50 1.487</sub> | 0.117<br><sub>context: p90 0.155 · p95 0.164 · p99 0.191 · 21769 op/s · total p50 1.378</sub> | -2.0% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.258<br><sub>context: p90 0.324 · p95 0.346 · p99 0.387 · 1191 op/s · total p50 3.118</sub> | 0.235<br><sub>context: p90 0.317 · p95 0.342 · p99 0.412 · 1099 op/s · total p50 3.522</sub> | -8.9% (-0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.347 · p95 0.376 · p99 0.454 · 2572 op/s · total p50 11.747</sub> | 0.271<br><sub>context: p90 0.358 · p95 0.391 · p99 0.453 · 2645 op/s · total p50 11.510</sub> | +1.8% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.110<br><sub>context: p90 0.143 · p95 0.157 · p99 0.170 · 3769 op/s · total p50 1.029</sub> | 0.106<br><sub>context: p90 0.146 · p95 0.157 · p99 0.166 · 3993 op/s · total p50 0.974</sub> | -3.6% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.116<br><sub>context: p90 0.154 · p95 0.163 · p99 0.191 · 24548 op/s · total p50 1.192</sub> | 0.121<br><sub>context: p90 0.157 · p95 0.167 · p99 0.188 · 21519 op/s · total p50 1.428</sub> | +4.1% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.119<br><sub>context: p90 0.165 · p95 0.184 · p99 0.218 · 3810 op/s · total p50 1.026</sub> | 0.123<br><sub>context: p90 0.165 · p95 0.182 · p99 0.227 · 3685 op/s · total p50 1.062</sub> | +3.2% (+0.004) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.133<br><sub>context: p90 0.185 · p95 0.209 · p99 0.260 · 23251 op/s · total p50 1.323</sub> | 0.131<br><sub>context: p90 0.181 · p95 0.203 · p99 0.266 · 24938 op/s · total p50 1.179</sub> | -1.4% (-0.002) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.117<br><sub>context: p90 0.166 · p95 0.181 · p99 0.193 · 3585 op/s · total p50 1.105</sub> | 0.110<br><sub>context: p90 0.162 · p95 0.183 · p99 0.210 · 3743 op/s · total p50 1.032</sub> | -5.8% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.129<br><sub>context: p90 0.184 · p95 0.201 · p99 0.232 · 23629 op/s · total p50 1.260</sub> | 0.132<br><sub>context: p90 0.184 · p95 0.204 · p99 0.232 · 22000 op/s · total p50 1.392</sub> | +2.0% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.040<br><sub>context: p90 0.072 · p95 0.076 · p99 0.079 · 7192 op/s · total p50 0.529</sub> | 0.041<br><sub>context: p90 0.076 · p95 0.078 · p99 0.090 · 6518 op/s · total p50 0.606</sub> | +0.7% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.040<br><sub>context: p90 0.074 · p95 0.078 · p99 0.084 · 41388 op/s · total p50 0.634</sub> | 0.040<br><sub>context: p90 0.075 · p95 0.079 · p99 0.089 · 41846 op/s · total p50 0.586</sub> | +0.9% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 8445 op/s · total p50 0.447</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 9277 op/s · total p50 0.398</sub> | +1.7% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.004<br><sub>context: p90 0.006 · p95 0.006 · p99 0.008 · 43617 op/s · total p50 0.559</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.008 · 43764 op/s · total p50 0.551</sub> | -2.7% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.047<br><sub>context: p90 0.074 · p95 0.078 · p99 0.093 · 5576 op/s · total p50 0.689</sub> | 0.047<br><sub>context: p90 0.080 · p95 0.083 · p99 0.095 · 5504 op/s · total p50 0.701</sub> | -1.2% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.052<br><sub>context: p90 0.087 · p95 0.091 · p99 0.101 · 33491 op/s · total p50 0.906</sub> | 0.053<br><sub>context: p90 0.088 · p95 0.093 · p99 0.103 · 33090 op/s · total p50 0.917</sub> | +2.4% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.067<br><sub>context: p90 0.124 · p95 0.133 · p99 0.142 · 5254 op/s · total p50 0.759</sub> | 0.067<br><sub>context: p90 0.126 · p95 0.131 · p99 0.147 · 5240 op/s · total p50 0.723</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.080<br><sub>context: p90 0.148 · p95 0.156 · p99 0.170 · 30154 op/s · total p50 1.011</sub> | 0.080<br><sub>context: p90 0.150 · p95 0.157 · p99 0.172 · 29986 op/s · total p50 0.990</sub> | -0.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.037<br><sub>context: p90 0.068 · p95 0.073 · p99 0.080 · 7171 op/s · total p50 0.538</sub> | 0.041<br><sub>context: p90 0.075 · p95 0.079 · p99 0.083 · 6204 op/s · total p50 0.625</sub> | +11.1% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.045<br><sub>context: p90 0.080 · p95 0.085 · p99 0.093 · 36868 op/s · total p50 0.828</sub> | 0.044<br><sub>context: p90 0.080 · p95 0.085 · p99 0.096 · 35818 op/s · total p50 0.847</sub> | -1.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.574<br><sub>context: p90 0.630 · p95 0.642 · p99 0.659 · 1307 op/s · total p50 3.052</sub> | 0.585<br><sub>context: p90 0.634 · p95 0.647 · p99 0.663 · 1276 op/s · total p50 3.095</sub> | +1.9% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.651<br><sub>context: p90 0.782 · p95 0.816 · p99 0.919 · 7824 op/s · total p50 3.804</sub> | 0.665<br><sub>context: p90 0.783 · p95 0.821 · p99 0.912 · 8396 op/s · total p50 3.640</sub> | +2.2% (+0.014) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.570<br><sub>context: p90 0.612 · p95 0.621 · p99 0.645 · 1345 op/s · total p50 2.973</sub> | 0.578<br><sub>context: p90 0.616 · p95 0.631 · p99 0.645 · 1314 op/s · total p50 3.029</sub> | +1.5% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.618<br><sub>context: p90 0.687 · p95 0.709 · p99 0.765 · 8149 op/s · total p50 3.619</sub> | 0.614<br><sub>context: p90 0.680 · p95 0.705 · p99 0.753 · 8529 op/s · total p50 3.627</sub> | -0.7% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.054<br><sub>context: p90 0.082 · p95 0.088 · p99 0.123 · 5974 op/s · total p50 0.638</sub> | 0.066<br><sub>context: p90 0.098 · p95 0.108 · p99 0.113 · 4870 op/s · total p50 0.813</sub> | +20.6% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.068<br><sub>context: p90 0.103 · p95 0.110 · p99 0.123 · 30821 op/s · total p50 0.951</sub> | 0.069<br><sub>context: p90 0.105 · p95 0.113 · p99 0.125 · 31436 op/s · total p50 0.938</sub> | +1.8% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.138<br><sub>context: p90 0.218 · p95 0.232 · p99 0.246 · 3387 op/s · total p50 1.137</sub> | 0.150<br><sub>context: p90 0.228 · p95 0.243 · p99 0.276 · 3099 op/s · total p50 1.284</sub> | +9.3% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.176<br><sub>context: p90 0.264 · p95 0.298 · p99 0.356 · 18630 op/s · total p50 1.637</sub> | 0.183<br><sub>context: p90 0.279 · p95 0.319 · p99 0.384 · 17663 op/s · total p50 1.734</sub> | +4.3% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.022<br><sub>context: p90 0.029 · p95 0.030 · p99 0.033 · 8005 op/s · total p50 0.471</sub> | 0.024<br><sub>context: p90 0.048 · p95 0.053 · p99 0.057 · 6862 op/s · total p50 0.558</sub> | +8.1% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.028<br><sub>context: p90 0.038 · p95 0.042 · p99 0.050 · 36197 op/s · total p50 0.806</sub> | 0.027<br><sub>context: p90 0.035 · p95 0.039 · p99 0.045 · 38959 op/s · total p50 0.702</sub> | -4.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.038<br><sub>context: p90 0.067 · p95 0.072 · p99 0.078 · 7442 op/s · total p50 0.513</sub> | 0.039<br><sub>context: p90 0.071 · p95 0.073 · p99 0.080 · 6598 op/s · total p50 0.579</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.039<br><sub>context: p90 0.070 · p95 0.074 · p99 0.078 · 42632 op/s · total p50 0.588</sub> | 0.038<br><sub>context: p90 0.060 · p95 0.070 · p99 0.073 · 43591 op/s · total p50 0.551</sub> | -3.0% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.034<br><sub>context: p90 0.063 · p95 0.076 · p99 0.083 · 8611 op/s · total p50 0.449</sub> | 0.034<br><sub>context: p90 0.064 · p95 0.071 · p99 0.079 · 7727 op/s · total p50 0.511</sub> | -1.6% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.040<br><sub>context: p90 0.076 · p95 0.080 · p99 0.091 · 40284 op/s · total p50 0.619</sub> | 0.040<br><sub>context: p90 0.075 · p95 0.078 · p99 0.086 · 43274 op/s · total p50 0.602</sub> | -0.9% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.205<br><sub>context: p90 0.229 · p95 0.238 · p99 0.262 · 2989 op/s · total p50 1.322</sub> | 0.198<br><sub>context: p90 0.227 · p95 0.233 · p99 0.242 · 3239 op/s · total p50 1.212</sub> | -3.6% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.240<br><sub>context: p90 0.267 · p95 0.275 · p99 0.313 · 20516 op/s · total p50 1.453</sub> | 0.241<br><sub>context: p90 0.271 · p95 0.277 · p99 0.299 · 20496 op/s · total p50 1.460</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>
