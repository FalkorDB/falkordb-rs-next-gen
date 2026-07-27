### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 → ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.612<br><sub>context: p90 0.650 · p95 0.661 · p99 0.670 · 981 op/s · total p50 1.012</sub> | 0.578<br><sub>context: p90 0.604 · p95 0.609 · p99 0.630 · 1079 op/s · total p50 0.917</sub> | -5.6% (-0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.731<br><sub>context: p90 0.864 · p95 0.885 · p99 0.955 · 7900 op/s · total p50 1.000</sub> | 0.698<br><sub>context: p90 0.820 · p95 0.840 · p99 0.878 · 8540 op/s · total p50 0.927</sub> | -4.5% (-0.033) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.648<br><sub>context: p90 0.687 · p95 0.696 · p99 0.712 · 961 op/s · total p50 1.038</sub> | 0.630<br><sub>context: p90 0.665 · p95 0.673 · p99 0.694 · 1041 op/s · total p50 0.946</sub> | -2.8% (-0.018) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.811<br><sub>context: p90 0.995 · p95 1.019 · p99 1.085 · 7384 op/s · total p50 1.067</sub> | 0.799<br><sub>context: p90 0.973 · p95 0.995 · p99 1.049 · 7592 op/s · total p50 1.038</sub> | -1.5% (-0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.140<br><sub>context: p90 1.195 · p95 1.204 · p99 1.224 · 650 op/s · total p50 1.525</sub> | 1.116<br><sub>context: p90 1.163 · p95 1.177 · p99 1.214 · 644 op/s · total p50 1.552</sub> | -2.1% (-0.024) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.601<br><sub>context: p90 2.082 · p95 2.244 · p99 2.617 · 4080 op/s · total p50 1.895</sub> | 1.577<br><sub>context: p90 2.065 · p95 2.230 · p99 2.531 · 4102 op/s · total p50 1.850</sub> | -1.6% (-0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.235<br><sub>context: p90 1.292 · p95 1.313 · p99 1.352 · 580 op/s · total p50 1.713</sub> | 1.182<br><sub>context: p90 1.241 · p95 1.262 · p99 1.316 · 642 op/s · total p50 1.549</sub> | -4.2% (-0.052) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.461<br><sub>context: p90 1.775 · p95 1.808 · p99 1.883 · 4386 op/s · total p50 1.772</sub> | 1.399<br><sub>context: p90 1.714 · p95 1.752 · p99 1.802 · 4630 op/s · total p50 1.689</sub> | -4.3% (-0.062) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.038<br><sub>context: p90 0.048 · p95 0.051 · p99 0.058 · 4467 op/s · total p50 0.221</sub> | 0.025<br><sub>context: p90 0.046 · p95 0.050 · p99 0.052 · 5061 op/s · total p50 0.177</sub> | -34.3% (-0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.026<br><sub>context: p90 0.037 · p95 0.042 · p99 0.051 · 33425 op/s · total p50 0.231</sub> | 0.025<br><sub>context: p90 0.035 · p95 0.039 · p99 0.046 · 33283 op/s · total p50 0.230</sub> | -2.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.072<br><sub>context: p90 0.097 · p95 0.104 · p99 0.110 · 2107 op/s · total p50 0.459</sub> | 0.069<br><sub>context: p90 0.098 · p95 0.102 · p99 0.118 · 2557 op/s · total p50 0.372</sub> | -3.9% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.064<br><sub>context: p90 0.097 · p95 0.103 · p99 0.122 · 20216 op/s · total p50 0.362</sub> | 0.086<br><sub>context: p90 0.161 · p95 0.195 · p99 0.258 · 21542 op/s · total p50 0.344</sub> | +34.7% (+0.022) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.073<br><sub>context: p90 0.101 · p95 0.105 · p99 0.124 · 1859 op/s · total p50 0.517</sub> | 0.074<br><sub>context: p90 0.101 · p95 0.108 · p99 0.116 · 2092 op/s · total p50 0.465</sub> | +1.1% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.064<br><sub>context: p90 0.098 · p95 0.105 · p99 0.116 · 21420 op/s · total p50 0.354</sub> | 0.085<br><sub>context: p90 0.153 · p95 0.179 · p99 0.225 · 20717 op/s · total p50 0.367</sub> | +33.0% (+0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.140<br><sub>context: p90 0.171 · p95 0.179 · p99 0.197 · 1798 op/s · total p50 0.553</sub> | 0.141<br><sub>context: p90 0.175 · p95 0.185 · p99 0.197 · 1873 op/s · total p50 0.526</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.142<br><sub>context: p90 0.180 · p95 0.194 · p99 0.220 · 14553 op/s · total p50 0.519</sub> | 0.147<br><sub>context: p90 0.204 · p95 0.230 · p99 0.279 · 16216 op/s · total p50 0.478</sub> | +3.9% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.148<br><sub>context: p90 0.180 · p95 0.189 · p99 0.198 · 1482 op/s · total p50 0.671</sub> | 0.148<br><sub>context: p90 0.176 · p95 0.183 · p99 0.191 · 1734 op/s · total p50 0.563</sub> | +0.3% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.153<br><sub>context: p90 0.192 · p95 0.204 · p99 0.259 · 12165 op/s · total p50 0.613</sub> | 0.152<br><sub>context: p90 0.211 · p95 0.235 · p99 0.287 · 15790 op/s · total p50 0.491</sub> | -1.1% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.221 · p95 0.235 · p99 0.255 · 1333 op/s · total p50 0.724</sub> | 0.176<br><sub>context: p90 0.216 · p95 0.227 · p99 0.242 · 1412 op/s · total p50 0.695</sub> | -5.6% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.185<br><sub>context: p90 0.234 · p95 0.250 · p99 0.281 · 11512 op/s · total p50 0.667</sub> | 0.195<br><sub>context: p90 0.252 · p95 0.273 · p99 0.329 · 11581 op/s · total p50 0.668</sub> | +5.2% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.190<br><sub>context: p90 0.231 · p95 0.246 · p99 0.278 · 1243 op/s · total p50 0.789</sub> | 0.183<br><sub>context: p90 0.230 · p95 0.244 · p99 0.254 · 1523 op/s · total p50 0.645</sub> | -3.6% (-0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.190<br><sub>context: p90 0.245 · p95 0.262 · p99 0.290 · 10916 op/s · total p50 0.711</sub> | 0.201<br><sub>context: p90 0.271 · p95 0.292 · p99 0.347 · 11029 op/s · total p50 0.698</sub> | +5.6% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.322<br><sub>context: p90 0.382 · p95 0.397 · p99 0.417 · 741 op/s · total p50 1.341</sub> | 0.322<br><sub>context: p90 0.403 · p95 0.425 · p99 0.480 · 794 op/s · total p50 1.231</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.336<br><sub>context: p90 0.442 · p95 0.470 · p99 0.530 · 5344 op/s · total p50 1.429</sub> | 0.340<br><sub>context: p90 0.448 · p95 0.478 · p99 0.549 · 5628 op/s · total p50 1.377</sub> | +1.2% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.330<br><sub>context: p90 0.405 · p95 0.422 · p99 0.473 · 742 op/s · total p50 1.318</sub> | 0.334<br><sub>context: p90 0.428 · p95 0.450 · p99 0.509 · 736 op/s · total p50 1.330</sub> | +1.2% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.339<br><sub>context: p90 0.449 · p95 0.479 · p99 0.543 · 5522 op/s · total p50 1.384</sub> | 0.360<br><sub>context: p90 0.481 · p95 0.514 · p99 0.613 · 5356 op/s · total p50 1.428</sub> | +6.2% (+0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.366<br><sub>context: p90 0.537 · p95 0.570 · p99 0.637 · 1344 op/s · total p50 0.754</sub> | 0.380<br><sub>context: p90 0.568 · p95 0.626 · p99 0.669 · 1261 op/s · total p50 0.782</sub> | +4.1% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.477<br><sub>context: p90 0.719 · p95 0.785 · p99 0.917 · 9806 op/s · total p50 0.795</sub> | 0.511<br><sub>context: p90 0.773 · p95 0.838 · p99 0.950 · 9389 op/s · total p50 0.837</sub> | +7.2% (+0.034) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.114<br><sub>context: p90 0.142 · p95 0.144 · p99 0.158 · 1950 op/s · total p50 0.505</sub> | 0.123<br><sub>context: p90 0.152 · p95 0.157 · p99 0.162 · 1877 op/s · total p50 0.530</sub> | +7.5% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.108<br><sub>context: p90 0.142 · p95 0.152 · p99 0.168 · 18575 op/s · total p50 0.420</sub> | 0.135<br><sub>context: p90 0.215 · p95 0.249 · p99 0.300 · 15039 op/s · total p50 0.479</sub> | +25.2% (+0.027) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.004<br><sub>context: p90 0.006 · p95 0.006 · p99 0.008 · 4930 op/s · total p50 0.195</sub> | 0.005<br><sub>context: p90 0.005 · p95 0.006 · p99 0.009 · 3664 op/s · total p50 0.263</sub> | +32.2% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.003<br><sub>context: p90 0.004 · p95 0.005 · p99 0.006 · 32061 op/s · total p50 0.236</sub> | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.006 · 32132 op/s · total p50 0.237</sub> | -3.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.003<br><sub>context: p90 0.006 · p95 0.006 · p99 0.008 · 6438 op/s · total p50 0.145</sub> | 0.005<br><sub>context: p90 0.006 · p95 0.006 · p99 0.009 · 4309 op/s · total p50 0.209</sub> | +63.2% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.006 · 35438 op/s · total p50 0.214</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 30828 op/s · total p50 0.237</sub> | +8.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.086<br><sub>context: p90 0.117 · p95 0.121 · p99 0.133 · 1971 op/s · total p50 0.492</sub> | 0.095<br><sub>context: p90 0.123 · p95 0.133 · p99 0.147 · 1586 op/s · total p50 0.622</sub> | +10.3% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.085<br><sub>context: p90 0.121 · p95 0.130 · p99 0.153 · 16773 op/s · total p50 0.456</sub> | 0.106<br><sub>context: p90 0.177 · p95 0.208 · p99 0.266 · 15910 op/s · total p50 0.482</sub> | +24.6% (+0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.183<br><sub>context: p90 1.904 · p95 2.118 · p99 2.397 · 600 op/s · total p50 1.625</sub> | 1.246<br><sub>context: p90 1.900 · p95 2.166 · p99 2.479 · 585 op/s · total p50 1.691</sub> | +5.3% (+0.063) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.025<br><sub>context: p90 3.322 · p95 3.801 · p99 4.555 · 3210 op/s · total p50 2.361</sub> | 1.982<br><sub>context: p90 3.260 · p95 3.684 · p99 4.241 · 3277 op/s · total p50 2.338</sub> | -2.1% (-0.043) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.805<br><sub>context: p90 7.359 · p95 8.273 · p99 9.065 · 181 op/s · total p50 5.325</sub> | 4.768<br><sub>context: p90 7.402 · p95 8.075 · p99 9.001 · 183 op/s · total p50 5.253</sub> | -0.8% (-0.037) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.385<br><sub>context: p90 13.215 · p95 14.481 · p99 16.630 · 869 op/s · total p50 8.792</sub> | 8.161<br><sub>context: p90 13.047 · p95 14.316 · p99 16.110 · 889 op/s · total p50 8.545</sub> | -2.7% (-0.224) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.026<br><sub>context: p90 0.042 · p95 0.045 · p99 0.048 · 3880 op/s · total p50 0.244</sub> | 0.034<br><sub>context: p90 0.040 · p95 0.042 · p99 0.044 · 3278 op/s · total p50 0.295</sub> | +29.4% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.022<br><sub>context: p90 0.030 · p95 0.033 · p99 0.042 · 28639 op/s · total p50 0.266</sub> | 0.022<br><sub>context: p90 0.031 · p95 0.035 · p99 0.042 · 23719 op/s · total p50 0.317</sub> | +0.2% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.035<br><sub>context: p90 0.041 · p95 0.042 · p99 0.048 · 3398 op/s · total p50 0.284</sub> | 0.030<br><sub>context: p90 0.037 · p95 0.039 · p99 0.042 · 4049 op/s · total p50 0.245</sub> | -13.6% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.022<br><sub>context: p90 0.030 · p95 0.033 · p99 0.041 · 28143 op/s · total p50 0.272</sub> | 0.022<br><sub>context: p90 0.031 · p95 0.035 · p99 0.043 · 23328 op/s · total p50 0.320</sub> | +0.5% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.010<br><sub>context: p90 0.014 · p95 0.016 · p99 0.017 · 3900 op/s · total p50 0.241</sub> | 0.016<br><sub>context: p90 0.019 · p95 0.020 · p99 0.024 · 2158 op/s · total p50 0.448</sub> | +65.5% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.013<br><sub>context: p90 0.017 · p95 0.019 · p99 0.022 · 20217 op/s · total p50 0.373</sub> | 0.013<br><sub>context: p90 0.018 · p95 0.021 · p99 0.024 · 19763 op/s · total p50 0.383</sub> | +4.0% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.007<br><sub>context: p90 0.013 · p95 0.014 · p99 0.019 · 5608 op/s · total p50 0.163</sub> | 0.007<br><sub>context: p90 0.012 · p95 0.013 · p99 0.015 · 5768 op/s · total p50 0.160</sub> | +2.0% (+0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.007<br><sub>context: p90 0.010 · p95 0.011 · p99 0.014 · 33906 op/s · total p50 0.228</sub> | 0.007<br><sub>context: p90 0.010 · p95 0.012 · p99 0.014 · 32281 op/s · total p50 0.235</sub> | +0.4% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.155<br><sub>context: p90 0.193 · p95 0.203 · p99 0.224 · 1980 op/s · total p50 0.501</sub> | 0.157<br><sub>context: p90 0.199 · p95 0.207 · p99 0.250 · 1824 op/s · total p50 0.547</sub> | +1.3% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.152<br><sub>context: p90 0.202 · p95 0.212 · p99 0.241 · 19318 op/s · total p50 0.403</sub> | 0.174<br><sub>context: p90 0.244 · p95 0.269 · p99 0.329 · 17405 op/s · total p50 0.436</sub> | +14.2% (+0.022) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.081<br><sub>context: p90 0.112 · p95 0.119 · p99 0.146 · 3116 op/s · total p50 0.320</sub> | 0.087<br><sub>context: p90 0.116 · p95 0.121 · p99 0.139 · 2380 op/s · total p50 0.421</sub> | +6.5% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.080<br><sub>context: p90 0.114 · p95 0.122 · p99 0.144 · 23631 op/s · total p50 0.319</sub> | 0.108<br><sub>context: p90 0.182 · p95 0.214 · p99 0.276 · 21030 op/s · total p50 0.356</sub> | +34.8% (+0.028) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.132<br><sub>context: p90 0.160 · p95 0.167 · p99 0.191 · 1775 op/s · total p50 0.559</sub> | 0.131<br><sub>context: p90 0.160 · p95 0.171 · p99 0.194 · 1882 op/s · total p50 0.523</sub> | -0.5% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.127<br><sub>context: p90 0.164 · p95 0.176 · p99 0.197 · 16664 op/s · total p50 0.457</sub> | 0.144<br><sub>context: p90 0.205 · p95 0.227 · p99 0.269 · 16492 op/s · total p50 0.468</sub> | +13.0% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.133<br><sub>context: p90 0.165 · p95 0.172 · p99 0.180 · 1504 op/s · total p50 0.652</sub> | 0.132<br><sub>context: p90 0.165 · p95 0.172 · p99 0.184 · 1462 op/s · total p50 0.673</sub> | -0.6% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.131<br><sub>context: p90 0.169 · p95 0.180 · p99 0.199 · 12379 op/s · total p50 0.621</sub> | 0.148<br><sub>context: p90 0.223 · p95 0.251 · p99 0.310 · 11477 op/s · total p50 0.659</sub> | +13.3% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.135<br><sub>context: p90 0.170 · p95 0.176 · p99 0.182 · 1365 op/s · total p50 0.723</sub> | 0.133<br><sub>context: p90 0.170 · p95 0.174 · p99 0.188 · 1590 op/s · total p50 0.626</sub> | -2.1% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.136<br><sub>context: p90 0.173 · p95 0.184 · p99 0.206 · 12106 op/s · total p50 0.641</sub> | 0.156<br><sub>context: p90 0.227 · p95 0.259 · p99 0.314 · 11719 op/s · total p50 0.660</sub> | +14.3% (+0.019) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.134<br><sub>context: p90 0.162 · p95 0.170 · p99 0.186 · 1751 op/s · total p50 0.562</sub> | 0.134<br><sub>context: p90 0.168 · p95 0.177 · p99 0.199 · 2080 op/s · total p50 0.476</sub> | +0.5% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.135<br><sub>context: p90 0.173 · p95 0.184 · p99 0.207 · 15563 op/s · total p50 0.497</sub> | 0.146<br><sub>context: p90 0.209 · p95 0.236 · p99 0.292 · 15400 op/s · total p50 0.499</sub> | +8.8% (+0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.110<br><sub>context: p90 0.143 · p95 0.148 · p99 0.154 · 1968 op/s · total p50 0.506</sub> | 0.118<br><sub>context: p90 0.150 · p95 0.153 · p99 0.160 · 1826 op/s · total p50 0.538</sub> | +7.4% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.103<br><sub>context: p90 0.138 · p95 0.147 · p99 0.171 · 19620 op/s · total p50 0.393</sub> | 0.124<br><sub>context: p90 0.196 · p95 0.219 · p99 0.293 · 18835 op/s · total p50 0.403</sub> | +20.2% (+0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.969<br><sub>context: p90 3.101 · p95 3.140 · p99 3.190 · 63 op/s · total p50 15.744</sub> | 3.070<br><sub>context: p90 3.233 · p95 3.303 · p99 3.428 · 63 op/s · total p50 15.778</sub> | +3.4% (+0.100) | 10% AND 0.5 ms | 🟢 |
| 8 | 3.201<br><sub>context: p90 4.234 · p95 4.363 · p99 4.617 · 373 op/s · total p50 19.478</sub> | 3.338<br><sub>context: p90 4.258 · p95 4.363 · p99 4.552 · 368 op/s · total p50 19.613</sub> | +4.3% (+0.137) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.137<br><sub>context: p90 0.168 · p95 0.174 · p99 0.179 · 1900 op/s · total p50 0.515</sub> | 0.138<br><sub>context: p90 0.168 · p95 0.174 · p99 0.192 · 2035 op/s · total p50 0.486</sub> | +1.0% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.138<br><sub>context: p90 0.174 · p95 0.186 · p99 0.211 · 15314 op/s · total p50 0.503</sub> | 0.147<br><sub>context: p90 0.206 · p95 0.234 · p99 0.283 · 16300 op/s · total p50 0.472</sub> | +6.6% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.362 · p95 0.377 · p99 0.405 · 660 op/s · total p50 1.484</sub> | 0.302<br><sub>context: p90 0.371 · p95 0.398 · p99 0.419 · 667 op/s · total p50 1.460</sub> | +0.9% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.314<br><sub>context: p90 0.411 · p95 0.438 · p99 0.499 · 4515 op/s · total p50 1.714</sub> | 0.323<br><sub>context: p90 0.426 · p95 0.457 · p99 0.528 · 4524 op/s · total p50 1.708</sub> | +2.9% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.138<br><sub>context: p90 0.169 · p95 0.176 · p99 0.186 · 1668 op/s · total p50 0.596</sub> | 0.132<br><sub>context: p90 0.164 · p95 0.172 · p99 0.186 · 1937 op/s · total p50 0.497</sub> | -4.4% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.129<br><sub>context: p90 0.168 · p95 0.179 · p99 0.199 · 16571 op/s · total p50 0.459</sub> | 0.154<br><sub>context: p90 0.223 · p95 0.250 · p99 0.301 · 14853 op/s · total p50 0.520</sub> | +19.0% (+0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.134<br><sub>context: p90 0.182 · p95 0.198 · p99 0.218 · 2061 op/s · total p50 0.479</sub> | 0.142<br><sub>context: p90 0.189 · p95 0.198 · p99 0.225 · 1838 op/s · total p50 0.543</sub> | +6.2% (+0.008) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.136<br><sub>context: p90 0.192 · p95 0.208 · p99 0.242 · 18407 op/s · total p50 0.411</sub> | 0.176<br><sub>context: p90 0.285 · p95 0.318 · p99 0.392 · 17108 op/s · total p50 0.451</sub> | +29.8% (+0.040) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.151<br><sub>context: p90 0.201 · p95 0.218 · p99 0.249 · 1658 op/s · total p50 0.603</sub> | 0.142<br><sub>context: p90 0.200 · p95 0.214 · p99 0.248 · 1962 op/s · total p50 0.504</sub> | -5.6% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.143<br><sub>context: p90 0.207 · p95 0.227 · p99 0.266 · 16579 op/s · total p50 0.466</sub> | 0.183<br><sub>context: p90 0.291 · p95 0.333 · p99 0.412 · 16138 op/s · total p50 0.471</sub> | +27.9% (+0.040) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.048<br><sub>context: p90 0.077 · p95 0.080 · p99 0.091 · 2395 op/s · total p50 0.387</sub> | 0.047<br><sub>context: p90 0.077 · p95 0.082 · p99 0.087 · 3367 op/s · total p50 0.287</sub> | -2.3% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.042<br><sub>context: p90 0.076 · p95 0.081 · p99 0.091 · 25783 op/s · total p50 0.295</sub> | 0.079<br><sub>context: p90 0.167 · p95 0.205 · p99 0.280 · 24205 op/s · total p50 0.311</sub> | +85.7% (+0.036) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.006<br><sub>context: p90 0.009 · p95 0.011 · p99 0.013 · 4441 op/s · total p50 0.213</sub> | 0.004<br><sub>context: p90 0.009 · p95 0.009 · p99 0.011 · 4978 op/s · total p50 0.179</sub> | -30.8% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.005<br><sub>context: p90 0.008 · p95 0.010 · p99 0.011 · 27561 op/s · total p50 0.267</sub> | 0.005<br><sub>context: p90 0.006 · p95 0.007 · p99 0.010 · 30281 op/s · total p50 0.251</sub> | -8.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.059<br><sub>context: p90 0.084 · p95 0.092 · p99 0.102 · 2974 op/s · total p50 0.335</sub> | 0.064<br><sub>context: p90 0.094 · p95 0.098 · p99 0.103 · 2641 op/s · total p50 0.373</sub> | +8.5% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.053<br><sub>context: p90 0.087 · p95 0.092 · p99 0.102 · 24170 op/s · total p50 0.322</sub> | 0.105<br><sub>context: p90 0.201 · p95 0.244 · p99 0.306 · 19740 op/s · total p50 0.391</sub> | +97.7% (+0.052) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.078<br><sub>context: p90 0.136 · p95 0.145 · p99 0.162 · 2573 op/s · total p50 0.373</sub> | 0.085<br><sub>context: p90 0.142 · p95 0.150 · p99 0.154 · 2423 op/s · total p50 0.406</sub> | +9.3% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.078<br><sub>context: p90 0.148 · p95 0.155 · p99 0.165 · 23768 op/s · total p50 0.320</sub> | 0.140<br><sub>context: p90 0.240 · p95 0.277 · p99 0.367 · 18917 op/s · total p50 0.399</sub> | +80.5% (+0.062) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.049<br><sub>context: p90 0.079 · p95 0.082 · p99 0.092 · 2703 op/s · total p50 0.355</sub> | 0.054<br><sub>context: p90 0.083 · p95 0.086 · p99 0.096 · 2443 op/s · total p50 0.404</sub> | +10.0% (+0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.044<br><sub>context: p90 0.079 · p95 0.082 · p99 0.089 · 26806 op/s · total p50 0.290</sub> | 0.074<br><sub>context: p90 0.144 · p95 0.171 · p99 0.231 · 22767 op/s · total p50 0.328</sub> | +69.8% (+0.030) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.624<br><sub>context: p90 0.665 · p95 0.676 · p99 0.696 · 779 op/s · total p50 1.291</sub> | 0.603<br><sub>context: p90 0.644 · p95 0.657 · p99 0.675 · 853 op/s · total p50 1.167</sub> | -3.4% (-0.021) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.714<br><sub>context: p90 0.836 · p95 0.869 · p99 1.004 · 5966 op/s · total p50 1.303</sub> | 0.706<br><sub>context: p90 0.810 · p95 0.834 · p99 0.913 · 6351 op/s · total p50 1.239</sub> | -1.1% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.614<br><sub>context: p90 0.649 · p95 0.663 · p99 0.677 · 925 op/s · total p50 1.080</sub> | 0.727<br><sub>context: p90 0.771 · p95 0.789 · p99 0.820 · 841 op/s · total p50 1.186</sub> | +18.4% (+0.113) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.719<br><sub>context: p90 0.844 · p95 0.876 · p99 0.967 · 7445 op/s · total p50 1.052</sub> | 0.829<br><sub>context: p90 0.942 · p95 0.970 · p99 1.030 · 6905 op/s · total p50 1.135</sub> | +15.3% (+0.110) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.080<br><sub>context: p90 0.108 · p95 0.117 · p99 0.130 · 2221 op/s · total p50 0.450</sub> | 0.088<br><sub>context: p90 0.117 · p95 0.123 · p99 0.136 · 1861 op/s · total p50 0.539</sub> | +10.4% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.073<br><sub>context: p90 0.105 · p95 0.114 · p99 0.130 · 20421 op/s · total p50 0.376</sub> | 0.098<br><sub>context: p90 0.169 · p95 0.201 · p99 0.257 · 16580 op/s · total p50 0.443</sub> | +33.4% (+0.025) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.177<br><sub>context: p90 0.246 · p95 0.278 · p99 0.298 · 1876 op/s · total p50 0.519</sub> | 0.202<br><sub>context: p90 0.281 · p95 0.322 · p99 0.358 · 1523 op/s · total p50 0.641</sub> | +14.2% (+0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.196<br><sub>context: p90 0.295 · p95 0.328 · p99 0.377 · 15868 op/s · total p50 0.494</sub> | 0.215<br><sub>context: p90 0.324 · p95 0.364 · p99 0.422 · 14515 op/s · total p50 0.530</sub> | +9.5% (+0.019) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.044<br><sub>context: p90 0.053 · p95 0.055 · p99 0.059 · 3109 op/s · total p50 0.320</sub> | 0.047<br><sub>context: p90 0.055 · p95 0.059 · p99 0.063 · 3203 op/s · total p50 0.300</sub> | +7.1% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.030<br><sub>context: p90 0.044 · p95 0.049 · p99 0.058 · 26242 op/s · total p50 0.291</sub> | 0.054<br><sub>context: p90 0.133 · p95 0.168 · p99 0.218 · 22695 op/s · total p50 0.336</sub> | +78.7% (+0.024) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.043<br><sub>context: p90 0.074 · p95 0.076 · p99 0.081 · 3586 op/s · total p50 0.270</sub> | 0.050<br><sub>context: p90 0.080 · p95 0.087 · p99 0.094 · 3135 op/s · total p50 0.313</sub> | +16.1% (+0.007) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.041<br><sub>context: p90 0.074 · p95 0.079 · p99 0.086 · 27843 op/s · total p50 0.276</sub> | 0.078<br><sub>context: p90 0.182 · p95 0.218 · p99 0.298 · 23197 op/s · total p50 0.322</sub> | +93.8% (+0.038) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.044<br><sub>context: p90 0.074 · p95 0.079 · p99 0.085 · 3282 op/s · total p50 0.292</sub> | 0.051<br><sub>context: p90 0.080 · p95 0.084 · p99 0.089 · 2607 op/s · total p50 0.370</sub> | +14.2% (+0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.041<br><sub>context: p90 0.076 · p95 0.080 · p99 0.088 · 27835 op/s · total p50 0.274</sub> | 0.078<br><sub>context: p90 0.181 · p95 0.222 · p99 0.292 · 22304 op/s · total p50 0.334</sub> | +90.1% (+0.037) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.219<br><sub>context: p90 0.235 · p95 0.240 · p99 0.244 · 1604 op/s · total p50 0.622</sub> | 0.218<br><sub>context: p90 0.238 · p95 0.241 · p99 0.256 · 1649 op/s · total p50 0.599</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.253<br><sub>context: p90 0.292 · p95 0.301 · p99 0.322 · 15298 op/s · total p50 0.494</sub> | 0.255<br><sub>context: p90 0.296 · p95 0.303 · p99 0.319 · 15662 op/s · total p50 0.492</sub> | +0.5% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>
