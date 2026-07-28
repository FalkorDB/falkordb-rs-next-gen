### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 → ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.539<br><sub>context: p90 0.569 · p95 0.577 · p99 0.586 · 1613 op/s · total p50 2.486</sub> | 0.530<br><sub>context: p90 0.564 · p95 0.574 · p99 0.585 · 1634 op/s · total p50 2.437</sub> | -1.6% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.596<br><sub>context: p90 0.659 · p95 0.682 · p99 0.729 · 9697 op/s · total p50 3.157</sub> | 0.587<br><sub>context: p90 0.653 · p95 0.682 · p99 0.725 · 9694 op/s · total p50 3.179</sub> | -1.6% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.563<br><sub>context: p90 0.595 · p95 0.604 · p99 0.624 · 1543 op/s · total p50 2.588</sub> | 0.573<br><sub>context: p90 0.615 · p95 0.624 · p99 0.643 · 1481 op/s · total p50 2.708</sub> | +1.8% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.598<br><sub>context: p90 0.643 · p95 0.661 · p99 0.708 · 9790 op/s · total p50 3.144</sub> | 0.608<br><sub>context: p90 0.660 · p95 0.682 · p99 0.731 · 9572 op/s · total p50 3.224</sub> | +1.7% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.022<br><sub>context: p90 1.068 · p95 1.080 · p99 1.102 · 873 op/s · total p50 4.605</sub> | 0.978<br><sub>context: p90 1.051 · p95 1.070 · p99 1.087 · 916 op/s · total p50 4.376</sub> | -4.3% (-0.044) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.625<br><sub>context: p90 2.339 · p95 2.503 · p99 2.820 · 4121 op/s · total p50 7.580</sub> | 1.798<br><sub>context: p90 2.468 · p95 2.640 · p99 2.955 · 3891 op/s · total p50 7.988</sub> | +10.6% (+0.172) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.065<br><sub>context: p90 1.129 · p95 1.153 · p99 1.181 · 866 op/s · total p50 4.636</sub> | 1.114<br><sub>context: p90 1.167 · p95 1.178 · p99 1.189 · 805 op/s · total p50 4.969</sub> | +4.5% (+0.048) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.137<br><sub>context: p90 1.287 · p95 1.328 · p99 1.409 · 6045 op/s · total p50 5.215</sub> | 1.175<br><sub>context: p90 1.308 · p95 1.346 · p99 1.431 · 5606 op/s · total p50 5.543</sub> | +3.3% (+0.038) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.013<br><sub>context: p90 0.015 · p95 0.016 · p99 0.022 · 15797 op/s · total p50 0.249</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.016 · p99 0.020 · 16475 op/s · total p50 0.228</sub> | -13.0% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.015<br><sub>context: p90 0.022 · p95 0.024 · p99 0.029 · 61782 op/s · total p50 0.393</sub> | 0.015<br><sub>context: p90 0.022 · p95 0.024 · p99 0.029 · 56127 op/s · total p50 0.410</sub> | +2.9% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.043<br><sub>context: p90 0.068 · p95 0.072 · p99 0.082 · 7683 op/s · total p50 0.513</sub> | 0.045<br><sub>context: p90 0.073 · p95 0.082 · p99 0.089 · 6401 op/s · total p50 0.612</sub> | +6.4% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.052<br><sub>context: p90 0.085 · p95 0.089 · p99 0.096 · 38411 op/s · total p50 0.796</sub> | 0.048<br><sub>context: p90 0.078 · p95 0.083 · p99 0.090 · 41809 op/s · total p50 0.724</sub> | -8.8% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.043<br><sub>context: p90 0.069 · p95 0.072 · p99 0.077 · 6977 op/s · total p50 0.567</sub> | 0.047<br><sub>context: p90 0.073 · p95 0.077 · p99 0.090 · 6193 op/s · total p50 0.620</sub> | +10.5% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.052<br><sub>context: p90 0.083 · p95 0.087 · p99 0.098 · 36952 op/s · total p50 0.806</sub> | 0.055<br><sub>context: p90 0.087 · p95 0.091 · p99 0.106 · 35434 op/s · total p50 0.838</sub> | +5.0% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.079<br><sub>context: p90 0.105 · p95 0.117 · p99 0.134 · 5262 op/s · total p50 0.745</sub> | 0.093<br><sub>context: p90 0.122 · p95 0.126 · p99 0.141 · 4289 op/s · total p50 0.898</sub> | +17.7% (+0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.097<br><sub>context: p90 0.129 · p95 0.137 · p99 0.152 · 29588 op/s · total p50 1.006</sub> | 0.099<br><sub>context: p90 0.131 · p95 0.141 · p99 0.157 · 28740 op/s · total p50 1.011</sub> | +1.8% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.093<br><sub>context: p90 0.122 · p95 0.128 · p99 0.144 · 4443 op/s · total p50 0.900</sub> | 0.088<br><sub>context: p90 0.122 · p95 0.128 · p99 0.152 · 4352 op/s · total p50 0.916</sub> | -5.9% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.104<br><sub>context: p90 0.138 · p95 0.147 · p99 0.164 · 25496 op/s · total p50 1.115</sub> | 0.103<br><sub>context: p90 0.137 · p95 0.146 · p99 0.162 · 25934 op/s · total p50 1.153</sub> | -0.9% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.112<br><sub>context: p90 0.149 · p95 0.158 · p99 0.181 · 4126 op/s · total p50 0.954</sub> | 0.107<br><sub>context: p90 0.141 · p95 0.147 · p99 0.170 · 4173 op/s · total p50 0.954</sub> | -4.6% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.137<br><sub>context: p90 0.187 · p95 0.201 · p99 0.224 · 16049 op/s · total p50 1.875</sub> | 0.129<br><sub>context: p90 0.176 · p95 0.189 · p99 0.214 · 16914 op/s · total p50 1.783</sub> | -5.7% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.129<br><sub>context: p90 0.172 · p95 0.179 · p99 0.205 · 3485 op/s · total p50 1.133</sub> | 0.136<br><sub>context: p90 0.177 · p95 0.183 · p99 0.193 · 3321 op/s · total p50 1.187</sub> | +5.1% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.150<br><sub>context: p90 0.205 · p95 0.221 · p99 0.246 · 14456 op/s · total p50 2.090</sub> | 0.147<br><sub>context: p90 0.202 · p95 0.218 · p99 0.243 · 14482 op/s · total p50 2.096</sub> | -2.0% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.301 · p95 0.323 · p99 0.351 · 2037 op/s · total p50 1.836</sub> | 0.246<br><sub>context: p90 0.326 · p95 0.337 · p99 0.426 · 1966 op/s · total p50 1.935</sub> | +9.1% (+0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.255<br><sub>context: p90 0.349 · p95 0.384 · p99 0.425 · 4138 op/s · total p50 7.360</sub> | 0.247<br><sub>context: p90 0.351 · p95 0.383 · p99 0.419 · 4185 op/s · total p50 7.320</sub> | -3.1% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.321 · p95 0.367 · p99 0.433 · 2050 op/s · total p50 1.833</sub> | 0.259<br><sub>context: p90 0.333 · p95 0.370 · p99 0.389 · 1911 op/s · total p50 2.037</sub> | +8.2% (+0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.280<br><sub>context: p90 0.375 · p95 0.405 · p99 0.463 · 3867 op/s · total p50 8.002</sub> | 0.267<br><sub>context: p90 0.358 · p95 0.390 · p99 0.441 · 3831 op/s · total p50 7.870</sub> | -4.7% (-0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.307<br><sub>context: p90 0.467 · p95 0.499 · p99 0.575 · 2235 op/s · total p50 1.780</sub> | 0.304<br><sub>context: p90 0.460 · p95 0.489 · p99 0.617 · 2179 op/s · total p50 1.809</sub> | -1.1% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.375<br><sub>context: p90 0.580 · p95 0.659 · p99 0.767 · 13050 op/s · total p50 2.276</sub> | 0.384<br><sub>context: p90 0.593 · p95 0.667 · p99 0.817 · 12355 op/s · total p50 2.469</sub> | +2.3% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.063<br><sub>context: p90 0.089 · p95 0.100 · p99 0.105 · 5786 op/s · total p50 0.679</sub> | 0.066<br><sub>context: p90 0.090 · p95 0.096 · p99 0.105 · 5664 op/s · total p50 0.689</sub> | +4.0% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.077<br><sub>context: p90 0.109 · p95 0.116 · p99 0.134 · 32904 op/s · total p50 0.930</sub> | 0.081<br><sub>context: p90 0.115 · p95 0.122 · p99 0.144 · 30808 op/s · total p50 0.960</sub> | +5.5% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.001<br><sub>context: p90 0.002 · p95 0.002 · p99 0.002 · 14063 op/s · total p50 0.279</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.003 · 13860 op/s · total p50 0.284</sub> | +43.4% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 60949 op/s · total p50 0.383</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 58677 op/s · total p50 0.415</sub> | +6.0% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.003 · 18780 op/s · total p50 0.209</sub> | 0.002<br><sub>context: p90 0.002 · p95 0.002 · p99 0.002 · 15995 op/s · total p50 0.246</sub> | -1.4% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 61614 op/s · total p50 0.370</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.003 · p99 0.004 · 61638 op/s · total p50 0.403</sub> | +7.7% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.065<br><sub>context: p90 0.093 · p95 0.097 · p99 0.111 · 4557 op/s · total p50 0.862</sub> | 0.059<br><sub>context: p90 0.089 · p95 0.092 · p99 0.097 · 4920 op/s · total p50 0.802</sub> | -9.0% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.076<br><sub>context: p90 0.105 · p95 0.114 · p99 0.129 · 25103 op/s · total p50 1.206</sub> | 0.072<br><sub>context: p90 0.100 · p95 0.109 · p99 0.125 · 27475 op/s · total p50 1.059</sub> | -5.9% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.003<br><sub>context: p90 1.421 · p95 1.501 · p99 1.665 · 860 op/s · total p50 4.651</sub> | 1.009<br><sub>context: p90 1.426 · p95 1.493 · p99 1.669 · 871 op/s · total p50 4.581</sub> | +0.6% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.585<br><sub>context: p90 2.376 · p95 2.542 · p99 2.883 · 4083 op/s · total p50 7.328</sub> | 1.687<br><sub>context: p90 2.509 · p95 2.704 · p99 3.076 · 3866 op/s · total p50 7.789</sub> | +6.4% (+0.102) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.264<br><sub>context: p90 6.675 · p95 7.163 · p99 7.537 · 216 op/s · total p50 18.028</sub> | 4.236<br><sub>context: p90 6.566 · p95 7.168 · p99 7.646 · 218 op/s · total p50 18.448</sub> | -0.7% (-0.028) | 10% AND 0.5 ms | 🟢 |
| 8 | 6.600<br><sub>context: p90 10.311 · p95 11.026 · p99 12.317 · 1098 op/s · total p50 28.550</sub> | 7.072<br><sub>context: p90 11.151 · p95 11.908 · p99 13.003 · 1030 op/s · total p50 30.363</sub> | +7.2% (+0.472) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.014<br><sub>context: p90 0.021 · p95 0.022 · p99 0.028 · 11012 op/s · total p50 0.350</sub> | 0.013<br><sub>context: p90 0.017 · p95 0.018 · p99 0.018 · 12833 op/s · total p50 0.305</sub> | -8.6% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.018<br><sub>context: p90 0.023 · p95 0.025 · p99 0.029 · 45166 op/s · total p50 0.633</sub> | 0.017<br><sub>context: p90 0.021 · p95 0.024 · p99 0.027 · 49630 op/s · total p50 0.567</sub> | -7.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.012<br><sub>context: p90 0.020 · p95 0.021 · p99 0.024 · 10785 op/s · total p50 0.366</sub> | 0.012<br><sub>context: p90 0.019 · p95 0.019 · p99 0.021 · 12182 op/s · total p50 0.317</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.017<br><sub>context: p90 0.023 · p95 0.025 · p99 0.031 · 45999 op/s · total p50 0.579</sub> | 0.017<br><sub>context: p90 0.021 · p95 0.023 · p99 0.028 · 49205 op/s · total p50 0.538</sub> | -2.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.008<br><sub>context: p90 0.011 · p95 0.012 · p99 0.014 · 10486 op/s · total p50 0.376</sub> | 0.009<br><sub>context: p90 0.010 · p95 0.011 · p99 0.012 · 9743 op/s · total p50 0.392</sub> | +19.1% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.010<br><sub>context: p90 0.013 · p95 0.014 · p99 0.017 · 18010 op/s · total p50 1.697</sub> | 0.009<br><sub>context: p90 0.013 · p95 0.014 · p99 0.017 · 19823 op/s · total p50 1.593</sub> | -4.5% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.006 · 13093 op/s · total p50 0.302</sub> | 0.005<br><sub>context: p90 0.006 · p95 0.007 · p99 0.008 · 13721 op/s · total p50 0.281</sub> | +10.4% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.005<br><sub>context: p90 0.007 · p95 0.008 · p99 0.010 · 60066 op/s · total p50 0.406</sub> | 0.005<br><sub>context: p90 0.007 · p95 0.008 · p99 0.010 · 58149 op/s · total p50 0.401</sub> | -4.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.164 · p95 0.174 · p99 0.200 · 4518 op/s · total p50 0.858</sub> | 0.124<br><sub>context: p90 0.158 · p95 0.164 · p99 0.187 · 4733 op/s · total p50 0.843</sub> | -4.5% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.139<br><sub>context: p90 0.181 · p95 0.193 · p99 0.221 · 29148 op/s · total p50 1.031</sub> | 0.146<br><sub>context: p90 0.186 · p95 0.196 · p99 0.225 · 27987 op/s · total p50 1.061</sub> | +4.9% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.059<br><sub>context: p90 0.087 · p95 0.095 · p99 0.104 · 7003 op/s · total p50 0.559</sub> | 0.058<br><sub>context: p90 0.084 · p95 0.087 · p99 0.093 · 6970 op/s · total p50 0.570</sub> | -1.5% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.070<br><sub>context: p90 0.102 · p95 0.110 · p99 0.139 · 40079 op/s · total p50 0.757</sub> | 0.072<br><sub>context: p90 0.105 · p95 0.114 · p99 0.145 · 36980 op/s · total p50 0.796</sub> | +3.3% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.070<br><sub>context: p90 0.100 · p95 0.107 · p99 0.117 · 5961 op/s · total p50 0.663</sub> | 0.073<br><sub>context: p90 0.103 · p95 0.109 · p99 0.128 · 5395 op/s · total p50 0.733</sub> | +3.9% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.093<br><sub>context: p90 0.126 · p95 0.134 · p99 0.159 · 28460 op/s · total p50 1.063</sub> | 0.099<br><sub>context: p90 0.131 · p95 0.140 · p99 0.155 · 28700 op/s · total p50 1.010</sub> | +6.5% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.067<br><sub>context: p90 0.096 · p95 0.103 · p99 0.118 · 5402 op/s · total p50 0.726</sub> | 0.078<br><sub>context: p90 0.111 · p95 0.123 · p99 0.148 · 4559 op/s · total p50 0.835</sub> | +16.1% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.096<br><sub>context: p90 0.132 · p95 0.142 · p99 0.160 · 15421 op/s · total p50 1.945</sub> | 0.101<br><sub>context: p90 0.137 · p95 0.148 · p99 0.171 · 15087 op/s · total p50 1.991</sub> | +5.4% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.094<br><sub>context: p90 0.129 · p95 0.136 · p99 0.154 · 4122 op/s · total p50 0.943</sub> | 0.100<br><sub>context: p90 0.136 · p95 0.143 · p99 0.163 · 3725 op/s · total p50 1.073</sub> | +6.1% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.106<br><sub>context: p90 0.144 · p95 0.153 · p99 0.171 · 17311 op/s · total p50 1.734</sub> | 0.107<br><sub>context: p90 0.146 · p95 0.156 · p99 0.174 · 17307 op/s · total p50 1.766</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.083<br><sub>context: p90 0.111 · p95 0.118 · p99 0.128 · 4631 op/s · total p50 0.843</sub> | 0.079<br><sub>context: p90 0.103 · p95 0.114 · p99 0.122 · 4656 op/s · total p50 0.786</sub> | -4.5% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.100<br><sub>context: p90 0.134 · p95 0.144 · p99 0.160 · 25860 op/s · total p50 1.193</sub> | 0.103<br><sub>context: p90 0.136 · p95 0.147 · p99 0.172 · 24656 op/s · total p50 1.259</sub> | +2.3% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.069<br><sub>context: p90 0.089 · p95 0.100 · p99 0.107 · 5748 op/s · total p50 0.686</sub> | 0.062<br><sub>context: p90 0.089 · p95 0.095 · p99 0.100 · 6585 op/s · total p50 0.604</sub> | -10.4% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.077<br><sub>context: p90 0.112 · p95 0.120 · p99 0.144 · 32443 op/s · total p50 0.927</sub> | 0.076<br><sub>context: p90 0.109 · p95 0.116 · p99 0.141 · 34413 op/s · total p50 0.849</sub> | -1.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.546<br><sub>context: p90 2.681 · p95 2.723 · p99 2.753 · 101 op/s · total p50 39.298</sub> | 2.542<br><sub>context: p90 2.917 · p95 3.281 · p99 3.462 · 103 op/s · total p50 37.528</sub> | -0.1% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.571<br><sub>context: p90 2.732 · p95 2.885 · p99 3.283 · 168 op/s · total p50 176.270</sub> | 2.593<br><sub>context: p90 2.727 · p95 2.776 · p99 3.059 · 171 op/s · total p50 179.566</sub> | +0.9% (+0.022) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.093<br><sub>context: p90 0.118 · p95 0.129 · p99 0.145 · 4267 op/s · total p50 0.922</sub> | 0.101<br><sub>context: p90 0.128 · p95 0.136 · p99 0.158 · 3834 op/s · total p50 1.031</sub> | +8.2% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.103<br><sub>context: p90 0.135 · p95 0.143 · p99 0.163 · 24730 op/s · total p50 1.239</sub> | 0.108<br><sub>context: p90 0.143 · p95 0.152 · p99 0.172 · 22948 op/s · total p50 1.365</sub> | +5.7% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.218<br><sub>context: p90 0.291 · p95 0.317 · p99 0.343 · 1325 op/s · total p50 2.862</sub> | 0.218<br><sub>context: p90 0.282 · p95 0.304 · p99 0.349 · 1267 op/s · total p50 3.198</sub> | -0.4% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.236<br><sub>context: p90 0.327 · p95 0.355 · p99 0.418 · 2857 op/s · total p50 10.537</sub> | 0.237<br><sub>context: p90 0.323 · p95 0.345 · p99 0.408 · 2671 op/s · total p50 11.621</sub> | +0.4% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.094<br><sub>context: p90 0.118 · p95 0.125 · p99 0.145 · 4691 op/s · total p50 0.838</sub> | 0.093<br><sub>context: p90 0.126 · p95 0.136 · p99 0.154 · 4693 op/s · total p50 0.815</sub> | -1.2% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.103<br><sub>context: p90 0.141 · p95 0.149 · p99 0.168 · 26278 op/s · total p50 1.141</sub> | 0.107<br><sub>context: p90 0.143 · p95 0.152 · p99 0.174 · 25106 op/s · total p50 1.176</sub> | +3.3% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.140 · p95 0.153 · p99 0.187 · 4649 op/s · total p50 0.855</sub> | 0.101<br><sub>context: p90 0.141 · p95 0.152 · p99 0.194 · 4797 op/s · total p50 0.832</sub> | -1.9% (-0.002) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.117<br><sub>context: p90 0.161 · p95 0.182 · p99 0.231 · 27109 op/s · total p50 1.128</sub> | 0.121<br><sub>context: p90 0.168 · p95 0.186 · p99 0.234 · 26470 op/s · total p50 1.128</sub> | +3.3% (+0.004) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.147 · p95 0.169 · p99 0.182 · 4330 op/s · total p50 0.915</sub> | 0.104<br><sub>context: p90 0.150 · p95 0.164 · p99 0.181 · 4286 op/s · total p50 0.924</sub> | +1.0% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.111<br><sub>context: p90 0.160 · p95 0.174 · p99 0.194 · 26366 op/s · total p50 1.075</sub> | 0.123<br><sub>context: p90 0.176 · p95 0.196 · p99 0.229 · 24734 op/s · total p50 1.178</sub> | +11.0% (+0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.032<br><sub>context: p90 0.063 · p95 0.066 · p99 0.068 · 8269 op/s · total p50 0.466</sub> | 0.035<br><sub>context: p90 0.068 · p95 0.069 · p99 0.074 · 8151 op/s · total p50 0.480</sub> | +7.4% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.034<br><sub>context: p90 0.065 · p95 0.067 · p99 0.072 · 50617 op/s · total p50 0.516</sub> | 0.037<br><sub>context: p90 0.070 · p95 0.072 · p99 0.079 · 47117 op/s · total p50 0.572</sub> | +9.1% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 11722 op/s · total p50 0.315</sub> | 0.004<br><sub>context: p90 0.004 · p95 0.004 · p99 0.005 · 10989 op/s · total p50 0.359</sub> | +30.9% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 49315 op/s · total p50 0.481</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.005 · p99 0.006 · 50743 op/s · total p50 0.458</sub> | -0.7% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.041<br><sub>context: p90 0.063 · p95 0.066 · p99 0.073 · 7255 op/s · total p50 0.536</sub> | 0.039<br><sub>context: p90 0.064 · p95 0.070 · p99 0.076 · 7102 op/s · total p50 0.552</sub> | -6.7% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.046<br><sub>context: p90 0.077 · p95 0.081 · p99 0.091 · 38986 op/s · total p50 0.768</sub> | 0.047<br><sub>context: p90 0.077 · p95 0.081 · p99 0.089 · 38860 op/s · total p50 0.778</sub> | +1.0% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.060<br><sub>context: p90 0.111 · p95 0.115 · p99 0.131 · 6391 op/s · total p50 0.618</sub> | 0.061<br><sub>context: p90 0.114 · p95 0.118 · p99 0.121 · 6266 op/s · total p50 0.630</sub> | +2.2% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.069<br><sub>context: p90 0.130 · p95 0.136 · p99 0.145 · 35779 op/s · total p50 0.832</sub> | 0.066<br><sub>context: p90 0.124 · p95 0.132 · p99 0.142 · 37201 op/s · total p50 0.810</sub> | -3.7% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.032<br><sub>context: p90 0.060 · p95 0.061 · p99 0.070 · 8003 op/s · total p50 0.496</sub> | 0.033<br><sub>context: p90 0.062 · p95 0.068 · p99 0.073 · 8578 op/s · total p50 0.461</sub> | +2.7% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.039<br><sub>context: p90 0.070 · p95 0.073 · p99 0.083 · 42173 op/s · total p50 0.722</sub> | 0.040<br><sub>context: p90 0.072 · p95 0.074 · p99 0.084 · 41248 op/s · total p50 0.735</sub> | +1.0% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.509<br><sub>context: p90 0.544 · p95 0.561 · p99 0.571 · 1533 op/s · total p50 2.600</sub> | 0.521<br><sub>context: p90 0.562 · p95 0.575 · p99 0.588 · 1508 op/s · total p50 2.637</sub> | +2.3% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.576<br><sub>context: p90 0.709 · p95 0.741 · p99 0.821 · 8502 op/s · total p50 3.611</sub> | 0.593<br><sub>context: p90 0.725 · p95 0.759 · p99 0.865 · 8242 op/s · total p50 3.690</sub> | +3.0% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.511<br><sub>context: p90 0.558 · p95 0.570 · p99 0.579 · 1565 op/s · total p50 2.560</sub> | 0.516<br><sub>context: p90 0.583 · p95 0.603 · p99 0.636 · 1532 op/s · total p50 2.595</sub> | +0.9% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.553<br><sub>context: p90 0.612 · p95 0.632 · p99 0.675 · 9751 op/s · total p50 3.128</sub> | 0.573<br><sub>context: p90 0.661 · p95 0.691 · p99 0.758 · 9241 op/s · total p50 3.276</sub> | +3.7% (+0.020) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.052<br><sub>context: p90 0.081 · p95 0.095 · p99 0.102 · 6499 op/s · total p50 0.598</sub> | 0.052<br><sub>context: p90 0.078 · p95 0.087 · p99 0.093 · 6972 op/s · total p50 0.566</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.061<br><sub>context: p90 0.095 · p95 0.101 · p99 0.114 · 34903 op/s · total p50 0.825</sub> | 0.059<br><sub>context: p90 0.091 · p95 0.098 · p99 0.109 · 36613 op/s · total p50 0.813</sub> | -4.6% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.125<br><sub>context: p90 0.200 · p95 0.227 · p99 0.249 · 3822 op/s · total p50 1.009</sub> | 0.131<br><sub>context: p90 0.201 · p95 0.226 · p99 0.251 · 3652 op/s · total p50 1.056</sub> | +4.3% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.157<br><sub>context: p90 0.240 · p95 0.272 · p99 0.320 · 20970 op/s · total p50 1.472</sub> | 0.161<br><sub>context: p90 0.247 · p95 0.274 · p99 0.331 · 21230 op/s · total p50 1.401</sub> | +2.9% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.017<br><sub>context: p90 0.020 · p95 0.021 · p99 0.024 · 9811 op/s · total p50 0.387</sub> | 0.020<br><sub>context: p90 0.031 · p95 0.037 · p99 0.049 · 8121 op/s · total p50 0.470</sub> | +20.5% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.024<br><sub>context: p90 0.032 · p95 0.036 · p99 0.044 · 39391 op/s · total p50 0.732</sub> | 0.023<br><sub>context: p90 0.030 · p95 0.033 · p99 0.039 · 45889 op/s · total p50 0.615</sub> | -3.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.036<br><sub>context: p90 0.068 · p95 0.070 · p99 0.074 · 7726 op/s · total p50 0.502</sub> | 0.031<br><sub>context: p90 0.057 · p95 0.068 · p99 0.069 · 8729 op/s · total p50 0.448</sub> | -13.6% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.034<br><sub>context: p90 0.044 · p95 0.063 · p99 0.067 · 50484 op/s · total p50 0.478</sub> | 0.033<br><sub>context: p90 0.041 · p95 0.059 · p99 0.065 · 50204 op/s · total p50 0.491</sub> | -1.7% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.031<br><sub>context: p90 0.058 · p95 0.065 · p99 0.068 · 9797 op/s · total p50 0.400</sub> | 0.032<br><sub>context: p90 0.065 · p95 0.067 · p99 0.071 · 9336 op/s · total p50 0.419</sub> | +3.7% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.037<br><sub>context: p90 0.069 · p95 0.071 · p99 0.079 · 48826 op/s · total p50 0.518</sub> | 0.035<br><sub>context: p90 0.066 · p95 0.068 · p99 0.075 · 48656 op/s · total p50 0.545</sub> | -4.8% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.197 · p95 0.199 · p99 0.208 · 3781 op/s · total p50 1.055</sub> | 0.183<br><sub>context: p90 0.189 · p95 0.192 · p99 0.201 · 3801 op/s · total p50 1.035</sub> | -1.0% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.220<br><sub>context: p90 0.243 · p95 0.251 · p99 0.290 · 20738 op/s · total p50 1.464</sub> | 0.215<br><sub>context: p90 0.240 · p95 0.246 · p99 0.268 · 21950 op/s · total p50 1.399</sub> | -2.2% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>
