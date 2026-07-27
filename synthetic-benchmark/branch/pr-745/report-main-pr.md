### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 → ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.480<br><sub>context: p90 0.523 · p95 0.533 · p99 0.546 · 1382 op/s · total p50 0.709</sub> | 0.505<br><sub>context: p90 0.531 · p95 0.540 · p99 0.562 · 1135 op/s · total p50 0.883</sub> | +5.1% (+0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.496<br><sub>context: p90 0.563 · p95 0.591 · p99 0.631 · 9098 op/s · total p50 0.798</sub> | 0.509<br><sub>context: p90 0.587 · p95 0.613 · p99 0.657 · 9504 op/s · total p50 0.797</sub> | +2.5% (+0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.547<br><sub>context: p90 0.579 · p95 0.589 · p99 0.607 · 1081 op/s · total p50 0.916</sub> | 0.560<br><sub>context: p90 0.587 · p95 0.595 · p99 0.618 · 1047 op/s · total p50 0.944</sub> | +2.4% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.559<br><sub>context: p90 0.634 · p95 0.669 · p99 0.728 · 8798 op/s · total p50 0.856</sub> | 0.558<br><sub>context: p90 0.648 · p95 0.676 · p99 0.725 · 8531 op/s · total p50 0.865</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.941<br><sub>context: p90 0.982 · p95 0.989 · p99 1.009 · 738 op/s · total p50 1.360</sub> | 0.944<br><sub>context: p90 0.977 · p95 0.983 · p99 1.005 · 750 op/s · total p50 1.333</sub> | +0.3% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.679<br><sub>context: p90 2.721 · p95 3.035 · p99 3.528 · 3462 op/s · total p50 2.111</sub> | 1.647<br><sub>context: p90 2.676 · p95 2.998 · p99 3.625 · 3522 op/s · total p50 2.060</sub> | -1.9% (-0.033) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.990<br><sub>context: p90 1.015 · p95 1.026 · p99 1.045 · 725 op/s · total p50 1.371</sub> | 0.994<br><sub>context: p90 1.023 · p95 1.031 · p99 1.054 · 695 op/s · total p50 1.430</sub> | +0.4% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.986<br><sub>context: p90 1.063 · p95 1.092 · p99 1.132 · 5450 op/s · total p50 1.343</sub> | 0.991<br><sub>context: p90 1.071 · p95 1.104 · p99 1.152 · 5340 op/s · total p50 1.360</sub> | +0.5% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.019<br><sub>context: p90 0.045 · p95 0.048 · p99 0.051 · 5466 op/s · total p50 0.163</sub> | 0.017<br><sub>context: p90 0.043 · p95 0.047 · p99 0.053 · 5495 op/s · total p50 0.157</sub> | -11.2% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.016<br><sub>context: p90 0.026 · p95 0.029 · p99 0.038 · 33690 op/s · total p50 0.223</sub> | 0.017<br><sub>context: p90 0.027 · p95 0.030 · p99 0.041 · 32719 op/s · total p50 0.229</sub> | +2.6% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.062<br><sub>context: p90 0.096 · p95 0.102 · p99 0.119 · 3042 op/s · total p50 0.328</sub> | 0.049<br><sub>context: p90 0.079 · p95 0.087 · p99 0.101 · 3503 op/s · total p50 0.251</sub> | -21.0% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.050<br><sub>context: p90 0.079 · p95 0.084 · p99 0.094 · 24082 op/s · total p50 0.313</sub> | 0.051<br><sub>context: p90 0.078 · p95 0.084 · p99 0.098 · 23011 op/s · total p50 0.331</sub> | +1.4% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.059<br><sub>context: p90 0.083 · p95 0.096 · p99 0.109 · 3323 op/s · total p50 0.274</sub> | 0.065<br><sub>context: p90 0.092 · p95 0.101 · p99 0.111 · 2818 op/s · total p50 0.349</sub> | +11.3% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.052<br><sub>context: p90 0.080 · p95 0.086 · p99 0.096 · 23670 op/s · total p50 0.327</sub> | 0.053<br><sub>context: p90 0.079 · p95 0.086 · p99 0.097 · 22889 op/s · total p50 0.333</sub> | +0.5% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.150 · p95 0.162 · p99 0.174 · 2564 op/s · total p50 0.371</sub> | 0.124<br><sub>context: p90 0.157 · p95 0.171 · p99 0.183 · 2074 op/s · total p50 0.479</sub> | +20.3% (+0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.106<br><sub>context: p90 0.141 · p95 0.151 · p99 0.171 · 17021 op/s · total p50 0.448</sub> | 0.104<br><sub>context: p90 0.138 · p95 0.150 · p99 0.173 · 17649 op/s · total p50 0.430</sub> | -1.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.146 · p95 0.155 · p99 0.181 · 2344 op/s · total p50 0.406</sub> | 0.114<br><sub>context: p90 0.161 · p95 0.168 · p99 0.178 · 2254 op/s · total p50 0.446</sub> | +10.1% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.109<br><sub>context: p90 0.144 · p95 0.153 · p99 0.173 · 16435 op/s · total p50 0.465</sub> | 0.108<br><sub>context: p90 0.144 · p95 0.156 · p99 0.175 · 16342 op/s · total p50 0.474</sub> | -0.5% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.157<br><sub>context: p90 0.199 · p95 0.208 · p99 0.227 · 1613 op/s · total p50 0.608</sub> | 0.153<br><sub>context: p90 0.200 · p95 0.212 · p99 0.232 · 1523 op/s · total p50 0.650</sub> | -2.6% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.149<br><sub>context: p90 0.194 · p95 0.206 · p99 0.234 · 12743 op/s · total p50 0.608</sub> | 0.151<br><sub>context: p90 0.195 · p95 0.210 · p99 0.239 · 12335 op/s · total p50 0.626</sub> | +2.0% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.218 · p95 0.227 · p99 0.242 · 1746 op/s · total p50 0.564</sub> | 0.163<br><sub>context: p90 0.207 · p95 0.217 · p99 0.254 · 1553 op/s · total p50 0.639</sub> | +4.5% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.155<br><sub>context: p90 0.205 · p95 0.218 · p99 0.240 · 11686 op/s · total p50 0.654</sub> | 0.155<br><sub>context: p90 0.202 · p95 0.219 · p99 0.252 · 11940 op/s · total p50 0.638</sub> | +0.0% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.286<br><sub>context: p90 0.363 · p95 0.374 · p99 0.393 · 892 op/s · total p50 1.090</sub> | 0.289<br><sub>context: p90 0.358 · p95 0.370 · p99 0.406 · 827 op/s · total p50 1.196</sub> | +1.0% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.285<br><sub>context: p90 0.378 · p95 0.397 · p99 0.442 · 6305 op/s · total p50 1.221</sub> | 0.284<br><sub>context: p90 0.371 · p95 0.396 · p99 0.431 · 6406 op/s · total p50 1.213</sub> | -0.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.255<br><sub>context: p90 0.341 · p95 0.366 · p99 0.426 · 869 op/s · total p50 1.121</sub> | 0.310<br><sub>context: p90 0.385 · p95 0.410 · p99 0.439 · 819 op/s · total p50 1.198</sub> | +21.4% (+0.055) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.386 · p95 0.419 · p99 0.475 · 6197 op/s · total p50 1.236</sub> | 0.301<br><sub>context: p90 0.396 · p95 0.422 · p99 0.487 · 6200 op/s · total p50 1.230</sub> | +1.7% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.329<br><sub>context: p90 0.484 · p95 0.546 · p99 0.593 · 1461 op/s · total p50 0.680</sub> | 0.327<br><sub>context: p90 0.487 · p95 0.553 · p99 0.611 · 1423 op/s · total p50 0.677</sub> | -0.6% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.500<br><sub>context: p90 0.798 · p95 0.877 · p99 1.030 · 9228 op/s · total p50 0.830</sub> | 0.491<br><sub>context: p90 0.758 · p95 0.842 · p99 1.039 · 9487 op/s · total p50 0.811</sub> | -1.9% (-0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.092<br><sub>context: p90 0.128 · p95 0.138 · p99 0.150 · 2757 op/s · total p50 0.339</sub> | 0.084<br><sub>context: p90 0.127 · p95 0.141 · p99 0.154 · 2741 op/s · total p50 0.338</sub> | -9.6% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.085<br><sub>context: p90 0.117 · p95 0.127 · p99 0.145 · 19912 op/s · total p50 0.377</sub> | 0.088<br><sub>context: p90 0.119 · p95 0.128 · p99 0.151 · 18616 op/s · total p50 0.413</sub> | +3.4% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.006 · p95 0.006 · p99 0.007 · 5730 op/s · total p50 0.150</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 6434 op/s · total p50 0.133</sub> | -28.6% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 33991 op/s · total p50 0.223</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 33656 op/s · total p50 0.226</sub> | -0.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.004 · 7802 op/s · total p50 0.110</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 6782 op/s · total p50 0.134</sub> | +20.7% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.004 · 37223 op/s · total p50 0.204</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 37779 op/s · total p50 0.203</sub> | -4.3% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.084<br><sub>context: p90 0.112 · p95 0.127 · p99 0.135 · 2137 op/s · total p50 0.467</sub> | 0.081<br><sub>context: p90 0.111 · p95 0.119 · p99 0.144 · 2347 op/s · total p50 0.425</sub> | -3.9% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.073<br><sub>context: p90 0.105 · p95 0.113 · p99 0.128 · 18544 op/s · total p50 0.414</sub> | 0.076<br><sub>context: p90 0.106 · p95 0.114 · p99 0.130 · 17052 op/s · total p50 0.447</sub> | +3.7% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.031<br><sub>context: p90 1.594 · p95 1.775 · p99 2.063 · 686 op/s · total p50 1.409</sub> | 1.063<br><sub>context: p90 1.632 · p95 1.878 · p99 2.164 · 658 op/s · total p50 1.483</sub> | +3.0% (+0.031) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.723<br><sub>context: p90 4.546 · p95 5.117 · p99 5.987 · 2398 op/s · total p50 3.154</sub> | 2.896<br><sub>context: p90 4.830 · p95 5.488 · p99 6.545 · 2294 op/s · total p50 3.311</sub> | +6.3% (+0.173) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.122<br><sub>context: p90 6.285 · p95 7.114 · p99 7.463 · 208 op/s · total p50 4.684</sub> | 4.076<br><sub>context: p90 6.299 · p95 7.061 · p99 7.479 · 210 op/s · total p50 4.625</sub> | -1.1% (-0.046) | 10% AND 0.5 ms | 🟢 |
| 8 | 12.800<br><sub>context: p90 19.330 · p95 21.470 · p99 23.513 · 584 op/s · total p50 13.254</sub> | 12.997<br><sub>context: p90 19.913 · p95 22.040 · p99 23.834 · 578 op/s · total p50 13.448</sub> | +1.5% (+0.197) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.018<br><sub>context: p90 0.041 · p95 0.045 · p99 0.050 · 4927 op/s · total p50 0.187</sub> | 0.018<br><sub>context: p90 0.043 · p95 0.046 · p99 0.051 · 4924 op/s · total p50 0.175</sub> | -0.7% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.018<br><sub>context: p90 0.024 · p95 0.028 · p99 0.034 · 28755 op/s · total p50 0.262</sub> | 0.018<br><sub>context: p90 0.024 · p95 0.027 · p99 0.034 · 29691 op/s · total p50 0.257</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.013<br><sub>context: p90 0.042 · p95 0.045 · p99 0.052 · 4417 op/s · total p50 0.196</sub> | 0.016<br><sub>context: p90 0.039 · p95 0.043 · p99 0.046 · 4679 op/s · total p50 0.190</sub> | +28.3% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.017<br><sub>context: p90 0.023 · p95 0.026 · p99 0.033 · 29437 op/s · total p50 0.257</sub> | 0.018<br><sub>context: p90 0.024 · p95 0.027 · p99 0.034 · 28282 op/s · total p50 0.267</sub> | +4.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.012<br><sub>context: p90 0.020 · p95 0.021 · p99 0.025 · 3563 op/s · total p50 0.268</sub> | 0.009<br><sub>context: p90 0.012 · p95 0.013 · p99 0.016 · 3736 op/s · total p50 0.255</sub> | -27.5% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.011<br><sub>context: p90 0.016 · p95 0.018 · p99 0.022 · 21718 op/s · total p50 0.343</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.017 · p99 0.022 · 22098 op/s · total p50 0.344</sub> | -2.2% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.005<br><sub>context: p90 0.006 · p95 0.007 · p99 0.007 · 7272 op/s · total p50 0.136</sub> | 0.006<br><sub>context: p90 0.015 · p95 0.016 · p99 0.018 · 5720 op/s · total p50 0.160</sub> | +12.8% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.006<br><sub>context: p90 0.008 · p95 0.009 · p99 0.012 · 34125 op/s · total p50 0.222</sub> | 0.005<br><sub>context: p90 0.008 · p95 0.009 · p99 0.011 · 34681 op/s · total p50 0.221</sub> | -4.8% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.175 · p95 0.184 · p99 0.204 · 2233 op/s · total p50 0.441</sub> | 0.133<br><sub>context: p90 0.172 · p95 0.184 · p99 0.209 · 2305 op/s · total p50 0.429</sub> | +1.8% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.128<br><sub>context: p90 0.165 · p95 0.178 · p99 0.200 · 18651 op/s · total p50 0.411</sub> | 0.129<br><sub>context: p90 0.167 · p95 0.178 · p99 0.196 · 20136 op/s · total p50 0.383</sub> | +0.7% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.067<br><sub>context: p90 0.100 · p95 0.110 · p99 0.121 · 3795 op/s · total p50 0.246</sub> | 0.060<br><sub>context: p90 0.088 · p95 0.103 · p99 0.108 · 3980 op/s · total p50 0.234</sub> | -10.0% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.066<br><sub>context: p90 0.095 · p95 0.104 · p99 0.123 · 24662 op/s · total p50 0.311</sub> | 0.066<br><sub>context: p90 0.095 · p95 0.102 · p99 0.123 · 26045 op/s · total p50 0.295</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.152 · p95 0.166 · p99 0.173 · 1946 op/s · total p50 0.508</sub> | 0.102<br><sub>context: p90 0.145 · p95 0.156 · p99 0.170 · 2277 op/s · total p50 0.431</sub> | -16.1% (-0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.097<br><sub>context: p90 0.129 · p95 0.139 · p99 0.161 · 18456 op/s · total p50 0.418</sub> | 0.100<br><sub>context: p90 0.136 · p95 0.146 · p99 0.168 · 17792 op/s · total p50 0.431</sub> | +3.0% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.118<br><sub>context: p90 0.151 · p95 0.158 · p99 0.178 · 1746 op/s · total p50 0.559</sub> | 0.123<br><sub>context: p90 0.160 · p95 0.163 · p99 0.172 · 1570 op/s · total p50 0.619</sub> | +4.2% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.108<br><sub>context: p90 0.142 · p95 0.153 · p99 0.173 · 13206 op/s · total p50 0.581</sub> | 0.108<br><sub>context: p90 0.146 · p95 0.157 · p99 0.177 · 13243 op/s · total p50 0.578</sub> | -0.4% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.127<br><sub>context: p90 0.156 · p95 0.163 · p99 0.179 · 1655 op/s · total p50 0.593</sub> | 0.126<br><sub>context: p90 0.153 · p95 0.164 · p99 0.178 · 1802 op/s · total p50 0.549</sub> | -0.6% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.114<br><sub>context: p90 0.152 · p95 0.164 · p99 0.188 · 12532 op/s · total p50 0.606</sub> | 0.114<br><sub>context: p90 0.149 · p95 0.159 · p99 0.180 · 12775 op/s · total p50 0.604</sub> | -0.5% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.091<br><sub>context: p90 0.152 · p95 0.161 · p99 0.178 · 2414 op/s · total p50 0.392</sub> | 0.110<br><sub>context: p90 0.150 · p95 0.157 · p99 0.172 · 2304 op/s · total p50 0.426</sub> | +21.5% (+0.020) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.104<br><sub>context: p90 0.136 · p95 0.145 · p99 0.165 · 16208 op/s · total p50 0.474</sub> | 0.104<br><sub>context: p90 0.137 · p95 0.148 · p99 0.171 · 16161 op/s · total p50 0.475</sub> | -0.5% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.060<br><sub>context: p90 0.121 · p95 0.135 · p99 0.151 · 3272 op/s · total p50 0.281</sub> | 0.083<br><sub>context: p90 0.124 · p95 0.135 · p99 0.145 · 2860 op/s · total p50 0.334</sub> | +38.7% (+0.023) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.081<br><sub>context: p90 0.114 · p95 0.123 · p99 0.139 · 20405 op/s · total p50 0.372</sub> | 0.084<br><sub>context: p90 0.116 · p95 0.126 · p99 0.144 · 19124 op/s · total p50 0.401</sub> | +3.5% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.524<br><sub>context: p90 2.570 · p95 2.581 · p99 2.608 · 73 op/s · total p50 13.681</sub> | 2.517<br><sub>context: p90 2.560 · p95 2.585 · p99 2.620 · 74 op/s · total p50 13.522</sub> | -0.3% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 3.155<br><sub>context: p90 3.436 · p95 3.470 · p99 3.629 · 465 op/s · total p50 16.187</sub> | 3.162<br><sub>context: p90 3.510 · p95 3.712 · p99 3.835 · 470 op/s · total p50 15.996</sub> | +0.2% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.153 · p95 0.165 · p99 0.182 · 2532 op/s · total p50 0.367</sub> | 0.117<br><sub>context: p90 0.162 · p95 0.170 · p99 0.183 · 2115 op/s · total p50 0.459</sub> | +12.9% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.112<br><sub>context: p90 0.145 · p95 0.156 · p99 0.181 · 16488 op/s · total p50 0.465</sub> | 0.111<br><sub>context: p90 0.142 · p95 0.153 · p99 0.179 · 16682 op/s · total p50 0.462</sub> | -1.4% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.255<br><sub>context: p90 0.324 · p95 0.336 · p99 0.362 · 744 op/s · total p50 1.320</sub> | 0.269<br><sub>context: p90 0.331 · p95 0.345 · p99 0.370 · 697 op/s · total p50 1.419</sub> | +5.4% (+0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.268<br><sub>context: p90 0.351 · p95 0.374 · p99 0.414 · 5338 op/s · total p50 1.439</sub> | 0.264<br><sub>context: p90 0.344 · p95 0.365 · p99 0.395 · 5413 op/s · total p50 1.437</sub> | -1.6% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.107<br><sub>context: p90 0.151 · p95 0.161 · p99 0.171 · 2139 op/s · total p50 0.461</sub> | 0.098<br><sub>context: p90 0.153 · p95 0.163 · p99 0.171 · 2589 op/s · total p50 0.371</sub> | -8.5% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.105<br><sub>context: p90 0.139 · p95 0.149 · p99 0.174 · 16704 op/s · total p50 0.460</sub> | 0.104<br><sub>context: p90 0.139 · p95 0.148 · p99 0.168 · 16364 op/s · total p50 0.469</sub> | -1.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.138<br><sub>context: p90 0.191 · p95 0.207 · p99 0.244 · 2162 op/s · total p50 0.456</sub> | 0.133<br><sub>context: p90 0.183 · p95 0.210 · p99 0.234 · 2076 op/s · total p50 0.472</sub> | -3.3% (-0.005) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.116<br><sub>context: p90 0.168 · p95 0.182 · p99 0.213 · 18771 op/s · total p50 0.403</sub> | 0.116<br><sub>context: p90 0.167 · p95 0.182 · p99 0.210 · 19414 op/s · total p50 0.391</sub> | -0.0% (-0.000) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.129<br><sub>context: p90 0.187 · p95 0.206 · p99 0.219 · 2277 op/s · total p50 0.433</sub> | 0.128<br><sub>context: p90 0.188 · p95 0.201 · p99 0.245 · 2366 op/s · total p50 0.409</sub> | -0.7% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.123<br><sub>context: p90 0.180 · p95 0.198 · p99 0.236 · 16600 op/s · total p50 0.454</sub> | 0.122<br><sub>context: p90 0.178 · p95 0.199 · p99 0.229 · 17484 op/s · total p50 0.430</sub> | -0.8% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.045<br><sub>context: p90 0.074 · p95 0.077 · p99 0.084 · 3220 op/s · total p50 0.310</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.070 · p99 0.077 · 4050 op/s · total p50 0.217</sub> | -24.6% (-0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.034<br><sub>context: p90 0.063 · p95 0.066 · p99 0.073 · 29272 op/s · total p50 0.261</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.067 · p99 0.074 · 29323 op/s · total p50 0.260</sub> | -0.5% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.003<br><sub>context: p90 0.005 · p95 0.008 · p99 0.009 · 5252 op/s · total p50 0.165</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.005 · p99 0.007 · 5362 op/s · total p50 0.172</sub> | +10.2% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 30043 op/s · total p50 0.252</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.008 · 28619 op/s · total p50 0.267</sub> | +3.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.046<br><sub>context: p90 0.073 · p95 0.083 · p99 0.091 · 3381 op/s · total p50 0.268</sub> | 0.041<br><sub>context: p90 0.068 · p95 0.072 · p99 0.094 · 3804 op/s · total p50 0.229</sub> | -10.8% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.044<br><sub>context: p90 0.073 · p95 0.077 · p99 0.087 · 24185 op/s · total p50 0.314</sub> | 0.045<br><sub>context: p90 0.074 · p95 0.078 · p99 0.089 · 25222 op/s · total p50 0.304</sub> | +1.5% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.061<br><sub>context: p90 0.114 · p95 0.121 · p99 0.134 · 3599 op/s · total p50 0.256</sub> | 0.064<br><sub>context: p90 0.113 · p95 0.117 · p99 0.125 · 3433 op/s · total p50 0.278</sub> | +5.5% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.065<br><sub>context: p90 0.125 · p95 0.132 · p99 0.141 · 24405 op/s · total p50 0.318</sub> | 0.066<br><sub>context: p90 0.125 · p95 0.132 · p99 0.140 · 23876 op/s · total p50 0.320</sub> | +1.1% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.033<br><sub>context: p90 0.061 · p95 0.075 · p99 0.084 · 4326 op/s · total p50 0.201</sub> | 0.035<br><sub>context: p90 0.062 · p95 0.075 · p99 0.082 · 4478 op/s · total p50 0.208</sub> | +4.9% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.037<br><sub>context: p90 0.067 · p95 0.070 · p99 0.078 · 28649 op/s · total p50 0.268</sub> | 0.037<br><sub>context: p90 0.067 · p95 0.070 · p99 0.078 · 27351 op/s · total p50 0.279</sub> | +0.7% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.532<br><sub>context: p90 0.571 · p95 0.587 · p99 0.602 · 1022 op/s · total p50 0.982</sub> | 0.530<br><sub>context: p90 0.560 · p95 0.571 · p99 0.588 · 960 op/s · total p50 1.032</sub> | -0.4% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.558<br><sub>context: p90 0.644 · p95 0.680 · p99 0.741 · 6309 op/s · total p50 1.194</sub> | 0.532<br><sub>context: p90 0.611 · p95 0.640 · p99 0.678 · 6754 op/s · total p50 1.126</sub> | -4.7% (-0.026) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.557<br><sub>context: p90 0.597 · p95 0.613 · p99 0.665 · 1072 op/s · total p50 0.923</sub> | 0.518<br><sub>context: p90 0.569 · p95 0.579 · p99 0.609 · 1115 op/s · total p50 0.898</sub> | -6.9% (-0.039) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.545<br><sub>context: p90 0.620 · p95 0.641 · p99 0.688 · 8122 op/s · total p50 0.922</sub> | 0.551<br><sub>context: p90 0.670 · p95 0.707 · p99 0.765 · 8052 op/s · total p50 0.923</sub> | +1.1% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.071<br><sub>context: p90 0.098 · p95 0.108 · p99 0.120 · 2705 op/s · total p50 0.359</sub> | 0.061<br><sub>context: p90 0.095 · p95 0.107 · p99 0.124 · 3073 op/s · total p50 0.294</sub> | -14.4% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.064<br><sub>context: p90 0.090 · p95 0.097 · p99 0.112 · 21080 op/s · total p50 0.366</sub> | 0.063<br><sub>context: p90 0.092 · p95 0.099 · p99 0.111 · 20492 op/s · total p50 0.376</sub> | -0.5% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.177<br><sub>context: p90 0.268 · p95 0.296 · p99 0.362 · 2084 op/s · total p50 0.464</sub> | 0.195<br><sub>context: p90 0.282 · p95 0.307 · p99 0.344 · 1909 op/s · total p50 0.521</sub> | +10.5% (+0.019) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.207<br><sub>context: p90 0.338 · p95 0.375 · p99 0.435 · 15005 op/s · total p50 0.509</sub> | 0.201<br><sub>context: p90 0.307 · p95 0.340 · p99 0.398 · 14743 op/s · total p50 0.519</sub> | -3.1% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.030<br><sub>context: p90 0.060 · p95 0.063 · p99 0.070 · 3833 op/s · total p50 0.233</sub> | 0.021<br><sub>context: p90 0.030 · p95 0.038 · p99 0.043 · 4325 op/s · total p50 0.211</sub> | -31.8% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.027<br><sub>context: p90 0.036 · p95 0.040 · p99 0.048 · 25214 op/s · total p50 0.302</sub> | 0.027<br><sub>context: p90 0.037 · p95 0.042 · p99 0.050 · 24541 op/s · total p50 0.311</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.034<br><sub>context: p90 0.059 · p95 0.070 · p99 0.081 · 4224 op/s · total p50 0.195</sub> | 0.036<br><sub>context: p90 0.065 · p95 0.077 · p99 0.089 · 4419 op/s · total p50 0.203</sub> | +7.6% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.034<br><sub>context: p90 0.062 · p95 0.066 · p99 0.071 · 29698 op/s · total p50 0.257</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.066 · p99 0.073 · 30212 op/s · total p50 0.254</sub> | -0.5% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.032<br><sub>context: p90 0.058 · p95 0.061 · p99 0.065 · 5036 op/s · total p50 0.182</sub> | 0.032<br><sub>context: p90 0.058 · p95 0.062 · p99 0.066 · 5190 op/s · total p50 0.183</sub> | -0.6% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.033<br><sub>context: p90 0.063 · p95 0.065 · p99 0.073 · 30666 op/s · total p50 0.252</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.065 · p99 0.071 · 30248 op/s · total p50 0.254</sub> | +1.8% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.209 · p95 0.214 · p99 0.219 · 2131 op/s · total p50 0.464</sub> | 0.176<br><sub>context: p90 0.200 · p95 0.205 · p99 0.214 · 2500 op/s · total p50 0.382</sub> | -4.6% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.219 · p95 0.227 · p99 0.245 · 17084 op/s · total p50 0.449</sub> | 0.187<br><sub>context: p90 0.217 · p95 0.224 · p99 0.237 · 18082 op/s · total p50 0.429</sub> | -1.5% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>
