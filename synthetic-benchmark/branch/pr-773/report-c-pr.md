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
| 1 | 1.241<br><sub>context: p90 1.289 · p95 1.311 · p99 1.365 · 748 op/s · total p50 5.343</sub> | 0.601<br><sub>context: p90 0.631 · p95 0.641 · p99 0.657 · 1364 op/s · total p50 2.894</sub> | -51.6% (-0.641) | 150% AND 2 ms | 🟢 |
| 8 | 1.373<br><sub>context: p90 1.946 · p95 2.193 · p99 2.515 · 4895 op/s · total p50 6.112</sub> | 0.634<br><sub>context: p90 0.699 · p95 0.727 · p99 0.770 · 8903 op/s · total p50 3.508</sub> | -53.8% (-0.739) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.923<br><sub>context: p90 1.971 · p95 1.985 · p99 2.012 · 490 op/s · total p50 8.167</sub> | 0.622<br><sub>context: p90 0.656 · p95 0.671 · p99 0.688 · 1350 op/s · total p50 2.952</sub> | -67.6% (-1.300) | 150% AND 2 ms | 🟢 |
| 8 | 2.078<br><sub>context: p90 2.523 · p95 2.741 · p99 3.124 · 3398 op/s · total p50 8.915</sub> | 0.677<br><sub>context: p90 0.735 · p95 0.763 · p99 0.820 · 8436 op/s · total p50 3.722</sub> | -67.4% (-1.401) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.889<br><sub>context: p90 1.975 · p95 1.999 · p99 2.059 · 494 op/s · total p50 8.080</sub> | 1.115<br><sub>context: p90 1.150 · p95 1.161 · p99 1.175 · 773 op/s · total p50 5.166</sub> | -41.0% (-0.774) | 150% AND 2 ms | 🟢 |
| 8 | 2.107<br><sub>context: p90 3.026 · p95 3.302 · p99 3.812 · 3230 op/s · total p50 9.295</sub> | 1.944<br><sub>context: p90 2.728 · p95 2.919 · p99 3.344 · 3578 op/s · total p50 8.819</sub> | -7.7% (-0.163) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.495<br><sub>context: p90 2.556 · p95 2.596 · p99 2.686 · 381 op/s · total p50 10.479</sub> | 1.191<br><sub>context: p90 1.232 · p95 1.242 · p99 1.286 · 736 op/s · total p50 5.423</sub> | -52.3% (-1.304) | 150% AND 2 ms | 🟢 |
| 8 | 2.819<br><sub>context: p90 3.751 · p95 4.030 · p99 4.831 · 2532 op/s · total p50 12.075</sub> | 1.257<br><sub>context: p90 1.413 · p95 1.455 · p99 1.547 · 5288 op/s · total p50 5.876</sub> | -55.4% (-1.563) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.100<br><sub>context: p90 0.134 · p95 0.140 · p99 0.146 · 6388 op/s · total p50 0.620</sub> | 0.014<br><sub>context: p90 0.019 · p95 0.022 · p99 0.027 · 13252 op/s · total p50 0.279</sub> | -85.7% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.196<br><sub>context: p90 0.386 · p95 0.454 · p99 0.609 · 27031 op/s · total p50 1.087</sub> | 0.017<br><sub>context: p90 0.026 · p95 0.029 · p99 0.036 · 52310 op/s · total p50 0.458</sub> | -91.3% (-0.179) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.218<br><sub>context: p90 0.271 · p95 0.286 · p99 0.308 · 3431 op/s · total p50 1.141</sub> | 0.056<br><sub>context: p90 0.088 · p95 0.094 · p99 0.103 · 5374 op/s · total p50 0.715</sub> | -74.2% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.321<br><sub>context: p90 0.492 · p95 0.557 · p99 0.681 · 17296 op/s · total p50 1.730</sub> | 0.056<br><sub>context: p90 0.090 · p95 0.095 · p99 0.105 · 34347 op/s · total p50 0.876</sub> | -82.4% (-0.264) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.292<br><sub>context: p90 0.333 · p95 0.342 · p99 0.358 · 2536 op/s · total p50 1.570</sub> | 0.060<br><sub>context: p90 0.093 · p95 0.101 · p99 0.108 · 4496 op/s · total p50 0.871</sub> | -79.4% (-0.232) | 150% AND 2 ms | 🟢 |
| 8 | 0.352<br><sub>context: p90 0.526 · p95 0.607 · p99 0.734 · 16323 op/s · total p50 1.859</sub> | 0.060<br><sub>context: p90 0.096 · p95 0.100 · p99 0.116 · 31387 op/s · total p50 0.957</sub> | -82.8% (-0.291) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.297<br><sub>context: p90 0.329 · p95 0.338 · p99 0.352 · 2601 op/s · total p50 1.493</sub> | 0.119<br><sub>context: p90 0.146 · p95 0.162 · p99 0.179 · 3280 op/s · total p50 1.188</sub> | -60.1% (-0.179) | 150% AND 2 ms | 🟢 |
| 8 | 0.428<br><sub>context: p90 0.679 · p95 0.789 · p99 0.964 · 12988 op/s · total p50 2.341</sub> | 0.113<br><sub>context: p90 0.152 · p95 0.168 · p99 0.196 · 24823 op/s · total p50 1.202</sub> | -73.7% (-0.315) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.357 · p95 0.370 · p99 0.394 · 2543 op/s · total p50 1.553</sub> | 0.107<br><sub>context: p90 0.143 · p95 0.160 · p99 0.170 · 3745 op/s · total p50 1.055</sub> | -66.2% (-0.209) | 150% AND 2 ms | 🟢 |
| 8 | 0.402<br><sub>context: p90 0.585 · p95 0.666 · p99 0.797 · 14320 op/s · total p50 2.118</sub> | 0.118<br><sub>context: p90 0.159 · p95 0.171 · p99 0.194 · 21363 op/s · total p50 1.339</sub> | -70.7% (-0.284) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.345<br><sub>context: p90 0.424 · p95 0.433 · p99 0.468 · 2363 op/s · total p50 1.675</sub> | 0.144<br><sub>context: p90 0.182 · p95 0.195 · p99 0.210 · 3101 op/s · total p50 1.261</sub> | -58.3% (-0.201) | 150% AND 2 ms | 🟢 |
| 8 | 0.455<br><sub>context: p90 0.641 · p95 0.713 · p99 0.868 · 12271 op/s · total p50 2.474</sub> | 0.157<br><sub>context: p90 0.210 · p95 0.225 · p99 0.250 · 14592 op/s · total p50 2.078</sub> | -65.5% (-0.298) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.355<br><sub>context: p90 0.452 · p95 0.471 · p99 0.525 · 2192 op/s · total p50 1.812</sub> | 0.144<br><sub>context: p90 0.188 · p95 0.205 · p99 0.235 · 2935 op/s · total p50 1.322</sub> | -59.4% (-0.211) | 150% AND 2 ms | 🟢 |
| 8 | 0.503<br><sub>context: p90 0.700 · p95 0.774 · p99 0.947 · 10761 op/s · total p50 2.773</sub> | 0.173<br><sub>context: p90 0.230 · p95 0.246 · p99 0.276 · 13385 op/s · total p50 2.253</sub> | -65.7% (-0.331) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.589<br><sub>context: p90 0.781 · p95 0.818 · p99 0.910 · 1380 op/s · total p50 2.842</sub> | 0.275<br><sub>context: p90 0.361 · p95 0.371 · p99 0.391 · 1668 op/s · total p50 2.205</sub> | -53.4% (-0.314) | 150% AND 2 ms | 🟢 |
| 8 | 0.658<br><sub>context: p90 0.893 · p95 0.977 · p99 1.123 · 3793 op/s · total p50 8.029</sub> | 0.293<br><sub>context: p90 0.402 · p95 0.434 · p99 0.479 · 3924 op/s · total p50 7.834</sub> | -55.5% (-0.365) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.678<br><sub>context: p90 0.824 · p95 0.871 · p99 0.987 · 1225 op/s · total p50 3.266</sub> | 0.297<br><sub>context: p90 0.389 · p95 0.415 · p99 0.472 · 1691 op/s · total p50 2.335</sub> | -56.1% (-0.381) | 150% AND 2 ms | 🟢 |
| 8 | 0.724<br><sub>context: p90 0.975 · p95 1.050 · p99 1.201 · 3687 op/s · total p50 8.269</sub> | 0.300<br><sub>context: p90 0.401 · p95 0.438 · p99 0.499 · 3707 op/s · total p50 8.334</sub> | -58.6% (-0.424) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.507<br><sub>context: p90 0.578 · p95 0.601 · p99 0.641 · 1656 op/s · total p50 2.384</sub> | 0.340<br><sub>context: p90 0.515 · p95 0.539 · p99 0.679 · 1885 op/s · total p50 2.149</sub> | -33.0% (-0.167) | 150% AND 2 ms | 🟢 |
| 8 | 0.628<br><sub>context: p90 0.854 · p95 0.940 · p99 1.137 · 9267 op/s · total p50 3.335</sub> | 0.438<br><sub>context: p90 0.679 · p95 0.762 · p99 0.934 · 11014 op/s · total p50 2.781</sub> | -30.3% (-0.190) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.341<br><sub>context: p90 0.384 · p95 0.394 · p99 0.423 · 2300 op/s · total p50 1.736</sub> | 0.096<br><sub>context: p90 0.132 · p95 0.142 · p99 0.153 · 3960 op/s · total p50 0.996</sub> | -71.8% (-0.245) | 150% AND 2 ms | 🟢 |
| 8 | 0.468<br><sub>context: p90 0.734 · p95 0.833 · p99 1.049 · 12533 op/s · total p50 2.442</sub> | 0.091<br><sub>context: p90 0.127 · p95 0.136 · p99 0.161 · 27002 op/s · total p50 1.079</sub> | -80.5% (-0.376) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.191<br><sub>context: p90 0.221 · p95 0.233 · p99 0.259 · 3613 op/s · total p50 1.072</sub> | 0.003<br><sub>context: p90 0.006 · p95 0.006 · p99 0.007 · 7538 op/s · total p50 0.511</sub> | -98.4% (-0.188) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.509 · p95 0.594 · p99 0.795 · 19036 op/s · total p50 1.529</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 50240 op/s · total p50 0.475</sub> | -99.2% (-0.288) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.139<br><sub>context: p90 0.168 · p95 0.171 · p99 0.183 · 4883 op/s · total p50 0.804</sub> | 0.002<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 11850 op/s · total p50 0.316</sub> | -98.5% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.218<br><sub>context: p90 0.386 · p95 0.462 · p99 0.601 · 26312 op/s · total p50 1.124</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 54275 op/s · total p50 0.437</sub> | -98.9% (-0.215) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.406<br><sub>context: p90 0.451 · p95 0.473 · p99 0.503 · 1895 op/s · total p50 2.089</sub> | 0.080<br><sub>context: p90 0.107 · p95 0.114 · p99 0.126 · 3457 op/s · total p50 1.162</sub> | -80.4% (-0.326) | 150% AND 2 ms | 🟢 |
| 8 | 0.557<br><sub>context: p90 0.940 · p95 1.093 · p99 1.392 · 10158 op/s · total p50 2.917</sub> | 0.083<br><sub>context: p90 0.118 · p95 0.127 · p99 0.145 · 23078 op/s · total p50 1.308</sub> | -85.0% (-0.474) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.712<br><sub>context: p90 0.892 · p95 0.947 · p99 1.023 · 1207 op/s · total p50 3.273</sub> | 1.076<br><sub>context: p90 1.544 · p95 1.612 · p99 1.773 · 792 op/s · total p50 5.024</sub> | +51.1% (+0.364) | 150% AND 2 ms | 🟢 |
| 8 | 0.851<br><sub>context: p90 1.219 · p95 1.347 · p99 1.652 · 6628 op/s · total p50 4.493</sub> | 1.892<br><sub>context: p90 2.809 · p95 3.027 · p99 3.452 · 3412 op/s · total p50 8.747</sub> | +122.3% (+1.041) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.329<br><sub>context: p90 4.007 · p95 4.295 · p99 4.437 · 374 op/s · total p50 10.381</sub> | 4.598<br><sub>context: p90 7.366 · p95 7.861 · p99 8.326 · 196 op/s · total p50 20.156</sub> | +97.4% (+2.268) | 150% AND 2 ms | 🟢 |
| 8 | 2.555<br><sub>context: p90 4.428 · p95 4.881 · p99 6.122 · 2618 op/s · total p50 11.537</sub> | 8.038<br><sub>context: p90 12.592 · p95 13.371 · p99 14.424 · 907 op/s · total p50 34.647</sub> | +214.6% (+5.483) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.189<br><sub>context: p90 0.209 · p95 0.220 · p99 0.235 · 3523 op/s · total p50 1.114</sub> | 0.019<br><sub>context: p90 0.026 · p95 0.036 · p99 0.043 · 9308 op/s · total p50 0.409</sub> | -90.1% (-0.171) | 150% AND 2 ms | 🟢 |
| 8 | 0.283<br><sub>context: p90 0.496 · p95 0.573 · p99 0.761 · 18993 op/s · total p50 1.557</sub> | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.033 · 40760 op/s · total p50 0.662</sub> | -93.1% (-0.264) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.203<br><sub>context: p90 0.237 · p95 0.246 · p99 0.277 · 3143 op/s · total p50 1.251</sub> | 0.019<br><sub>context: p90 0.024 · p95 0.030 · p99 0.036 · 9291 op/s · total p50 0.414</sub> | -90.6% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.256<br><sub>context: p90 0.424 · p95 0.499 · p99 0.628 · 21328 op/s · total p50 1.396</sub> | 0.019<br><sub>context: p90 0.025 · p95 0.028 · p99 0.033 · 42308 op/s · total p50 0.613</sub> | -92.7% (-0.238) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.192<br><sub>context: p90 0.231 · p95 0.247 · p99 0.259 · 3664 op/s · total p50 1.068</sub> | 0.010<br><sub>context: p90 0.015 · p95 0.018 · p99 0.020 · 7615 op/s · total p50 0.499</sub> | -94.6% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.212<br><sub>context: p90 0.296 · p95 0.329 · p99 0.412 · 15576 op/s · total p50 1.922</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.017 · p99 0.021 · 17143 op/s · total p50 1.764</sub> | -94.8% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.150<br><sub>context: p90 0.189 · p95 0.194 · p99 0.206 · 4774 op/s · total p50 0.821</sub> | 0.005<br><sub>context: p90 0.008 · p95 0.009 · p99 0.009 · 11237 op/s · total p50 0.327</sub> | -96.8% (-0.145) | 150% AND 2 ms | 🟢 |
| 8 | 0.224<br><sub>context: p90 0.397 · p95 0.454 · p99 0.615 · 24633 op/s · total p50 1.210</sub> | 0.006<br><sub>context: p90 0.009 · p95 0.010 · p99 0.012 · 53204 op/s · total p50 0.469</sub> | -97.4% (-0.218) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.275<br><sub>context: p90 0.304 · p95 0.310 · p99 0.332 · 2927 op/s · total p50 1.357</sub> | 0.140<br><sub>context: p90 0.185 · p95 0.193 · p99 0.207 · 3912 op/s · total p50 1.011</sub> | -49.2% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.347<br><sub>context: p90 0.544 · p95 0.621 · p99 0.751 · 15638 op/s · total p50 1.913</sub> | 0.155<br><sub>context: p90 0.203 · p95 0.222 · p99 0.252 · 25123 op/s · total p50 1.162</sub> | -55.2% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.230<br><sub>context: p90 0.259 · p95 0.274 · p99 0.299 · 2902 op/s · total p50 1.365</sub> | 0.073<br><sub>context: p90 0.107 · p95 0.113 · p99 0.129 · 5150 op/s · total p50 0.759</sub> | -68.0% (-0.156) | 150% AND 2 ms | 🟢 |
| 8 | 0.313<br><sub>context: p90 0.546 · p95 0.639 · p99 0.821 · 17290 op/s · total p50 1.775</sub> | 0.078<br><sub>context: p90 0.117 · p95 0.131 · p99 0.159 · 34677 op/s · total p50 0.878</sub> | -75.2% (-0.235) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.293<br><sub>context: p90 0.334 · p95 0.348 · p99 0.369 · 2664 op/s · total p50 1.453</sub> | 0.100<br><sub>context: p90 0.138 · p95 0.149 · p99 0.159 · 4005 op/s · total p50 0.981</sub> | -65.8% (-0.193) | 150% AND 2 ms | 🟢 |
| 8 | 0.362<br><sub>context: p90 0.528 · p95 0.592 · p99 0.707 · 15769 op/s · total p50 1.919</sub> | 0.109<br><sub>context: p90 0.148 · p95 0.159 · p99 0.193 · 23498 op/s · total p50 1.238</sub> | -69.8% (-0.253) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.264<br><sub>context: p90 0.306 · p95 0.316 · p99 0.339 · 2853 op/s · total p50 1.397</sub> | 0.111<br><sub>context: p90 0.149 · p95 0.161 · p99 0.175 · 3113 op/s · total p50 1.276</sub> | -58.1% (-0.153) | 150% AND 2 ms | 🟢 |
| 8 | 0.371<br><sub>context: p90 0.540 · p95 0.612 · p99 0.767 · 14496 op/s · total p50 2.107</sub> | 0.112<br><sub>context: p90 0.153 · p95 0.165 · p99 0.178 · 13744 op/s · total p50 2.154</sub> | -69.8% (-0.259) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.326<br><sub>context: p90 0.372 · p95 0.381 · p99 0.397 · 2303 op/s · total p50 1.722</sub> | 0.104<br><sub>context: p90 0.132 · p95 0.147 · p99 0.156 · 3588 op/s · total p50 1.105</sub> | -68.2% (-0.222) | 150% AND 2 ms | 🟢 |
| 8 | 0.407<br><sub>context: p90 0.606 · p95 0.680 · p99 0.852 · 12744 op/s · total p50 2.364</sub> | 0.116<br><sub>context: p90 0.155 · p95 0.167 · p99 0.187 · 16154 op/s · total p50 1.849</sub> | -71.6% (-0.291) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.312<br><sub>context: p90 0.346 · p95 0.359 · p99 0.376 · 2597 op/s · total p50 1.515</sub> | 0.114<br><sub>context: p90 0.145 · p95 0.151 · p99 0.165 · 3277 op/s · total p50 1.214</sub> | -63.6% (-0.198) | 150% AND 2 ms | 🟢 |
| 8 | 0.394<br><sub>context: p90 0.566 · p95 0.647 · p99 0.790 · 14972 op/s · total p50 2.069</sub> | 0.113<br><sub>context: p90 0.152 · p95 0.164 · p99 0.188 · 22418 op/s · total p50 1.364</sub> | -71.3% (-0.281) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.333<br><sub>context: p90 0.377 · p95 0.386 · p99 0.437 · 2416 op/s · total p50 1.628</sub> | 0.086<br><sub>context: p90 0.117 · p95 0.126 · p99 0.145 · 4482 op/s · total p50 0.871</sub> | -74.2% (-0.248) | 150% AND 2 ms | 🟢 |
| 8 | 0.420<br><sub>context: p90 0.619 · p95 0.723 · p99 0.884 · 13641 op/s · total p50 2.160</sub> | 0.088<br><sub>context: p90 0.126 · p95 0.136 · p99 0.174 · 29292 op/s · total p50 1.021</sub> | -79.1% (-0.332) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 6.313<br><sub>context: p90 6.774 · p95 6.881 · p99 7.215 · 88 op/s · total p50 43.825</sub> | 2.859<br><sub>context: p90 2.923 · p95 2.934 · p99 2.969 · 92 op/s · total p50 43.653</sub> | -54.7% (-3.454) | 150% AND 2 ms | 🟢 |
| 8 | 6.792<br><sub>context: p90 8.885 · p95 9.329 · p99 10.156 · 148 op/s · total p50 210.202</sub> | 2.885<br><sub>context: p90 2.993 · p95 3.044 · p99 3.289 · 159 op/s · total p50 186.694</sub> | -57.5% (-3.908) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.401<br><sub>context: p90 0.453 · p95 0.465 · p99 0.502 · 1984 op/s · total p50 1.993</sub> | 0.107<br><sub>context: p90 0.139 · p95 0.157 · p99 0.168 · 3618 op/s · total p50 1.101</sub> | -73.4% (-0.294) | 150% AND 2 ms | 🟢 |
| 8 | 0.525<br><sub>context: p90 0.749 · p95 0.855 · p99 1.071 · 10886 op/s · total p50 2.703</sub> | 0.117<br><sub>context: p90 0.155 · p95 0.164 · p99 0.191 · 21769 op/s · total p50 1.378</sub> | -77.7% (-0.408) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.547<br><sub>context: p90 0.654 · p95 0.684 · p99 0.753 · 1173 op/s · total p50 2.889</sub> | 0.235<br><sub>context: p90 0.317 · p95 0.342 · p99 0.412 · 1099 op/s · total p50 3.522</sub> | -57.0% (-0.312) | 150% AND 2 ms | 🟢 |
| 8 | 0.621<br><sub>context: p90 0.783 · p95 0.848 · p99 0.928 · 2642 op/s · total p50 11.355</sub> | 0.271<br><sub>context: p90 0.358 · p95 0.391 · p99 0.453 · 2645 op/s · total p50 11.510</sub> | -56.4% (-0.351) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.357 · p95 0.366 · p99 0.392 · 2486 op/s · total p50 1.604</sub> | 0.106<br><sub>context: p90 0.146 · p95 0.157 · p99 0.166 · 3993 op/s · total p50 0.974</sub> | -66.5% (-0.210) | 150% AND 2 ms | 🟢 |
| 8 | 0.410<br><sub>context: p90 0.623 · p95 0.718 · p99 0.890 · 13771 op/s · total p50 2.220</sub> | 0.121<br><sub>context: p90 0.157 · p95 0.167 · p99 0.188 · 21519 op/s · total p50 1.428</sub> | -70.5% (-0.289) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.125<br><sub>context: p90 2.693 · p95 2.763 · p99 2.834 · 460 op/s · total p50 8.877</sub> | 0.123<br><sub>context: p90 0.165 · p95 0.182 · p99 0.227 · 3685 op/s · total p50 1.062</sub> | -94.2% (-2.002) | 150% AND 2 ms | 🟢 |
| 8 | 2.303<br><sub>context: p90 2.926 · p95 3.108 · p99 4.168 · 3271 op/s · total p50 9.601</sub> | 0.131<br><sub>context: p90 0.181 · p95 0.203 · p99 0.266 · 24938 op/s · total p50 1.179</sub> | -94.3% (-2.172) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.031<br><sub>context: p90 2.661 · p95 2.741 · p99 2.839 · 469 op/s · total p50 8.452</sub> | 0.110<br><sub>context: p90 0.162 · p95 0.183 · p99 0.210 · 3743 op/s · total p50 1.032</sub> | -94.6% (-1.921) | 150% AND 2 ms | 🟢 |
| 8 | 2.195<br><sub>context: p90 2.851 · p95 3.043 · p99 4.168 · 3455 op/s · total p50 8.865</sub> | 0.132<br><sub>context: p90 0.184 · p95 0.204 · p99 0.232 · 22000 op/s · total p50 1.392</sub> | -94.0% (-2.064) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.165 · p95 0.170 · p99 0.187 · 5477 op/s · total p50 0.718</sub> | 0.041<br><sub>context: p90 0.076 · p95 0.078 · p99 0.090 · 6518 op/s · total p50 0.606</sub> | -66.7% (-0.082) | 150% AND 2 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.324 · p95 0.369 · p99 0.472 · 27540 op/s · total p50 1.076</sub> | 0.040<br><sub>context: p90 0.075 · p95 0.079 · p99 0.089 · 41846 op/s · total p50 0.586</sub> | -78.9% (-0.150) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.238<br><sub>context: p90 0.274 · p95 0.288 · p99 0.301 · 3245 op/s · total p50 1.209</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 9277 op/s · total p50 0.398</sub> | -98.5% (-0.235) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.319<br><sub>context: p90 0.522 · p95 0.613 · p99 0.725 · 16701 op/s · total p50 1.817</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.008 · 43764 op/s · total p50 0.551</sub> | -98.7% (-0.315) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.243<br><sub>context: p90 0.281 · p95 0.285 · p99 0.300 · 3141 op/s · total p50 1.260</sub> | 0.047<br><sub>context: p90 0.080 · p95 0.083 · p99 0.095 · 5504 op/s · total p50 0.701</sub> | -80.8% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 0.370<br><sub>context: p90 0.604 · p95 0.714 · p99 0.883 · 16096 op/s · total p50 1.897</sub> | 0.053<br><sub>context: p90 0.088 · p95 0.093 · p99 0.103 · 33090 op/s · total p50 0.917</sub> | -85.5% (-0.316) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.253<br><sub>context: p90 0.289 · p95 0.302 · p99 0.331 · 2892 op/s · total p50 1.323</sub> | 0.067<br><sub>context: p90 0.126 · p95 0.131 · p99 0.147 · 5240 op/s · total p50 0.723</sub> | -73.4% (-0.186) | 150% AND 2 ms | 🟢 |
| 8 | 0.366<br><sub>context: p90 0.582 · p95 0.660 · p99 0.812 · 16590 op/s · total p50 1.848</sub> | 0.080<br><sub>context: p90 0.150 · p95 0.157 · p99 0.172 · 29986 op/s · total p50 0.990</sub> | -78.1% (-0.286) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.224<br><sub>context: p90 0.259 · p95 0.274 · p99 0.289 · 3428 op/s · total p50 1.151</sub> | 0.041<br><sub>context: p90 0.075 · p95 0.079 · p99 0.083 · 6204 op/s · total p50 0.625</sub> | -81.8% (-0.183) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.481 · p95 0.552 · p99 0.669 · 18864 op/s · total p50 1.628</sub> | 0.044<br><sub>context: p90 0.080 · p95 0.085 · p99 0.096 · 35818 op/s · total p50 0.847</sub> | -85.5% (-0.260) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.137<br><sub>context: p90 1.212 · p95 1.235 · p99 1.262 · 793 op/s · total p50 5.035</sub> | 0.585<br><sub>context: p90 0.634 · p95 0.647 · p99 0.663 · 1276 op/s · total p50 3.095</sub> | -48.6% (-0.552) | 150% AND 2 ms | 🟢 |
| 8 | 1.308<br><sub>context: p90 1.601 · p95 1.769 · p99 2.098 · 5178 op/s · total p50 5.784</sub> | 0.665<br><sub>context: p90 0.783 · p95 0.821 · p99 0.912 · 8396 op/s · total p50 3.640</sub> | -49.2% (-0.643) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.173<br><sub>context: p90 1.244 · p95 1.261 · p99 1.293 · 784 op/s · total p50 5.076</sub> | 0.578<br><sub>context: p90 0.616 · p95 0.631 · p99 0.645 · 1314 op/s · total p50 3.029</sub> | -50.7% (-0.595) | 150% AND 2 ms | 🟢 |
| 8 | 1.248<br><sub>context: p90 1.526 · p95 1.660 · p99 2.027 · 5538 op/s · total p50 5.493</sub> | 0.614<br><sub>context: p90 0.680 · p95 0.705 · p99 0.753 · 8529 op/s · total p50 3.627</sub> | -50.8% (-0.634) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.298 · p95 0.310 · p99 0.358 · 2768 op/s · total p50 1.402</sub> | 0.066<br><sub>context: p90 0.098 · p95 0.108 · p99 0.113 · 4870 op/s · total p50 0.813</sub> | -74.9% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 0.349<br><sub>context: p90 0.548 · p95 0.618 · p99 0.778 · 15793 op/s · total p50 1.896</sub> | 0.069<br><sub>context: p90 0.105 · p95 0.113 · p99 0.125 · 31436 op/s · total p50 0.938</sub> | -80.1% (-0.280) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.347<br><sub>context: p90 0.433 · p95 0.458 · p99 0.470 · 2341 op/s · total p50 1.690</sub> | 0.150<br><sub>context: p90 0.228 · p95 0.243 · p99 0.276 · 3099 op/s · total p50 1.284</sub> | -56.6% (-0.196) | 150% AND 2 ms | 🟢 |
| 8 | 0.467<br><sub>context: p90 0.696 · p95 0.793 · p99 0.995 · 11738 op/s · total p50 2.557</sub> | 0.183<br><sub>context: p90 0.279 · p95 0.319 · p99 0.384 · 17663 op/s · total p50 1.734</sub> | -60.7% (-0.284) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.280 · p95 0.289 · p99 0.314 · 2816 op/s · total p50 1.397</sub> | 0.024<br><sub>context: p90 0.048 · p95 0.053 · p99 0.057 · 6862 op/s · total p50 0.558</sub> | -89.8% (-0.213) | 150% AND 2 ms | 🟢 |
| 8 | 0.348<br><sub>context: p90 0.660 · p95 0.783 · p99 0.986 · 14749 op/s · total p50 2.036</sub> | 0.027<br><sub>context: p90 0.035 · p95 0.039 · p99 0.045 · 38959 op/s · total p50 0.702</sub> | -92.3% (-0.321) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.171<br><sub>context: p90 0.206 · p95 0.214 · p99 0.239 · 3896 op/s · total p50 1.015</sub> | 0.039<br><sub>context: p90 0.071 · p95 0.073 · p99 0.080 · 6598 op/s · total p50 0.579</sub> | -77.4% (-0.133) | 150% AND 2 ms | 🟢 |
| 8 | 0.199<br><sub>context: p90 0.329 · p95 0.384 · p99 0.486 · 26845 op/s · total p50 1.105</sub> | 0.038<br><sub>context: p90 0.060 · p95 0.070 · p99 0.073 · 43591 op/s · total p50 0.551</sub> | -81.0% (-0.161) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.161<br><sub>context: p90 0.187 · p95 0.197 · p99 0.207 · 4414 op/s · total p50 0.871</sub> | 0.034<br><sub>context: p90 0.064 · p95 0.071 · p99 0.079 · 7727 op/s · total p50 0.511</sub> | -79.1% (-0.128) | 150% AND 2 ms | 🟢 |
| 8 | 0.197<br><sub>context: p90 0.348 · p95 0.407 · p99 0.498 · 26858 op/s · total p50 1.119</sub> | 0.040<br><sub>context: p90 0.075 · p95 0.078 · p99 0.086 · 43274 op/s · total p50 0.602</sub> | -79.8% (-0.157) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.875<br><sub>context: p90 0.920 · p95 0.932 · p99 0.946 · 1012 op/s · total p50 3.936</sub> | 0.198<br><sub>context: p90 0.227 · p95 0.233 · p99 0.242 · 3239 op/s · total p50 1.212</sub> | -77.4% (-0.677) | 150% AND 2 ms | 🟢 |
| 8 | 0.974<br><sub>context: p90 1.686 · p95 1.828 · p99 2.072 · 5712 op/s · total p50 5.424</sub> | 0.241<br><sub>context: p90 0.271 · p95 0.277 · p99 0.299 · 20496 op/s · total p50 1.460</sub> | -75.3% (-0.733) | 150% AND 2 ms | 🟢 |

</details>
