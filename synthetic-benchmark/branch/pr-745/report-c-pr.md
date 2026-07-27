### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.129<br><sub>context: p90 1.199 · p95 1.212 · p99 1.232 · 734 op/s · total p50 1.353</sub> | 0.505<br><sub>context: p90 0.531 · p95 0.540 · p99 0.562 · 1135 op/s · total p50 0.883</sub> | -55.3% (-0.624) | 150% AND 2 ms | 🟢 |
| 8 | 1.203<br><sub>context: p90 1.693 · p95 1.897 · p99 2.109 · 4523 op/s · total p50 1.455</sub> | 0.509<br><sub>context: p90 0.587 · p95 0.613 · p99 0.657 · 9504 op/s · total p50 0.797</sub> | -57.7% (-0.694) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.693<br><sub>context: p90 1.756 · p95 1.776 · p99 1.834 · 511 op/s · total p50 1.947</sub> | 0.560<br><sub>context: p90 0.587 · p95 0.595 · p99 0.618 · 1047 op/s · total p50 0.944</sub> | -66.9% (-1.133) | 150% AND 2 ms | 🟢 |
| 8 | 1.808<br><sub>context: p90 2.208 · p95 2.446 · p99 2.731 · 3610 op/s · total p50 2.039</sub> | 0.558<br><sub>context: p90 0.648 · p95 0.676 · p99 0.725 · 8531 op/s · total p50 0.865</sub> | -69.1% (-1.250) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.690<br><sub>context: p90 1.762 · p95 1.772 · p99 1.800 · 502 op/s · total p50 1.988</sub> | 0.944<br><sub>context: p90 0.977 · p95 0.983 · p99 1.005 · 750 op/s · total p50 1.333</sub> | -44.2% (-0.746) | 150% AND 2 ms | 🟢 |
| 8 | 1.793<br><sub>context: p90 2.726 · p95 2.989 · p99 3.293 · 3267 op/s · total p50 2.098</sub> | 1.647<br><sub>context: p90 2.676 · p95 2.998 · p99 3.625 · 3522 op/s · total p50 2.060</sub> | -8.2% (-0.147) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.250<br><sub>context: p90 2.320 · p95 2.347 · p99 2.395 · 387 op/s · total p50 2.583</sub> | 0.994<br><sub>context: p90 1.023 · p95 1.031 · p99 1.054 · 695 op/s · total p50 1.430</sub> | -55.8% (-1.256) | 150% AND 2 ms | 🟢 |
| 8 | 2.517<br><sub>context: p90 3.806 · p95 4.242 · p99 5.063 · 2512 op/s · total p50 2.800</sub> | 0.991<br><sub>context: p90 1.071 · p95 1.104 · p99 1.152 · 5340 op/s · total p50 1.360</sub> | -60.6% (-1.526) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.060<br><sub>context: p90 0.096 · p95 0.109 · p99 0.119 · 5541 op/s · total p50 0.158</sub> | 0.017<br><sub>context: p90 0.043 · p95 0.047 · p99 0.053 · 5495 op/s · total p50 0.157</sub> | -71.7% (-0.043) | 150% AND 2 ms | 🟢 |
| 8 | 0.123<br><sub>context: p90 0.193 · p95 0.224 · p99 0.269 · 28773 op/s · total p50 0.250</sub> | 0.017<br><sub>context: p90 0.027 · p95 0.030 · p99 0.041 · 32719 op/s · total p50 0.229</sub> | -86.3% (-0.106) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.233<br><sub>context: p90 0.285 · p95 0.300 · p99 0.313 · 2332 op/s · total p50 0.415</sub> | 0.049<br><sub>context: p90 0.079 · p95 0.087 · p99 0.101 · 3503 op/s · total p50 0.251</sub> | -78.9% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.266<br><sub>context: p90 0.347 · p95 0.377 · p99 0.453 · 16959 op/s · total p50 0.440</sub> | 0.051<br><sub>context: p90 0.078 · p95 0.084 · p99 0.098 · 23011 op/s · total p50 0.331</sub> | -80.8% (-0.215) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.310 · p95 0.314 · p99 0.332 · 2315 op/s · total p50 0.417</sub> | 0.065<br><sub>context: p90 0.092 · p95 0.101 · p99 0.111 · 2818 op/s · total p50 0.349</sub> | -70.5% (-0.156) | 150% AND 2 ms | 🟢 |
| 8 | 0.290<br><sub>context: p90 0.383 · p95 0.418 · p99 0.486 · 16295 op/s · total p50 0.459</sub> | 0.053<br><sub>context: p90 0.079 · p95 0.086 · p99 0.097 · 22889 op/s · total p50 0.333</sub> | -81.9% (-0.238) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.330 · p95 0.338 · p99 0.345 · 2041 op/s · total p50 0.472</sub> | 0.124<br><sub>context: p90 0.157 · p95 0.171 · p99 0.183 · 2074 op/s · total p50 0.479</sub> | -52.5% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.395 · p95 0.431 · p99 0.506 · 14933 op/s · total p50 0.505</sub> | 0.104<br><sub>context: p90 0.138 · p95 0.150 · p99 0.173 · 17649 op/s · total p50 0.430</sub> | -65.8% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.329<br><sub>context: p90 0.370 · p95 0.379 · p99 0.392 · 1820 op/s · total p50 0.553</sub> | 0.114<br><sub>context: p90 0.161 · p95 0.168 · p99 0.178 · 2254 op/s · total p50 0.446</sub> | -65.2% (-0.214) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.431 · p95 0.466 · p99 0.543 · 14216 op/s · total p50 0.539</sub> | 0.108<br><sub>context: p90 0.144 · p95 0.156 · p99 0.175 · 16342 op/s · total p50 0.474</sub> | -68.0% (-0.230) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.365<br><sub>context: p90 0.421 · p95 0.439 · p99 0.474 · 1418 op/s · total p50 0.701</sub> | 0.153<br><sub>context: p90 0.200 · p95 0.212 · p99 0.232 · 1523 op/s · total p50 0.650</sub> | -58.0% (-0.212) | 150% AND 2 ms | 🟢 |
| 8 | 0.386<br><sub>context: p90 0.492 · p95 0.527 · p99 0.596 · 10751 op/s · total p50 0.702</sub> | 0.151<br><sub>context: p90 0.195 · p95 0.210 · p99 0.239 · 12335 op/s · total p50 0.626</sub> | -60.8% (-0.235) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.419<br><sub>context: p90 0.481 · p95 0.496 · p99 0.539 · 1272 op/s · total p50 0.770</sub> | 0.163<br><sub>context: p90 0.207 · p95 0.217 · p99 0.254 · 1553 op/s · total p50 0.639</sub> | -61.1% (-0.256) | 150% AND 2 ms | 🟢 |
| 8 | 0.437<br><sub>context: p90 0.546 · p95 0.584 · p99 0.649 · 10071 op/s · total p50 0.768</sub> | 0.155<br><sub>context: p90 0.202 · p95 0.219 · p99 0.252 · 11940 op/s · total p50 0.638</sub> | -64.6% (-0.282) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.626<br><sub>context: p90 0.748 · p95 0.785 · p99 0.893 · 757 op/s · total p50 1.301</sub> | 0.289<br><sub>context: p90 0.358 · p95 0.370 · p99 0.406 · 827 op/s · total p50 1.196</sub> | -53.9% (-0.338) | 150% AND 2 ms | 🟢 |
| 8 | 0.655<br><sub>context: p90 0.856 · p95 0.932 · p99 1.054 · 5261 op/s · total p50 1.463</sub> | 0.284<br><sub>context: p90 0.371 · p95 0.396 · p99 0.431 · 6406 op/s · total p50 1.213</sub> | -56.6% (-0.371) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.622<br><sub>context: p90 0.790 · p95 0.843 · p99 0.910 · 745 op/s · total p50 1.307</sub> | 0.310<br><sub>context: p90 0.385 · p95 0.410 · p99 0.439 · 819 op/s · total p50 1.198</sub> | -50.1% (-0.312) | 150% AND 2 ms | 🟢 |
| 8 | 0.725<br><sub>context: p90 0.959 · p95 1.027 · p99 1.170 · 5013 op/s · total p50 1.524</sub> | 0.301<br><sub>context: p90 0.396 · p95 0.422 · p99 0.487 · 6200 op/s · total p50 1.230</sub> | -58.5% (-0.424) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.475<br><sub>context: p90 0.557 · p95 0.579 · p99 0.613 · 1430 op/s · total p50 0.691</sub> | 0.327<br><sub>context: p90 0.487 · p95 0.553 · p99 0.611 · 1423 op/s · total p50 0.677</sub> | -31.1% (-0.148) | 150% AND 2 ms | 🟢 |
| 8 | 0.555<br><sub>context: p90 0.707 · p95 0.753 · p99 0.877 · 9864 op/s · total p50 0.765</sub> | 0.491<br><sub>context: p90 0.758 · p95 0.842 · p99 1.039 · 9487 op/s · total p50 0.811</sub> | -11.6% (-0.064) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.283<br><sub>context: p90 0.358 · p95 0.367 · p99 0.378 · 2000 op/s · total p50 0.487</sub> | 0.084<br><sub>context: p90 0.127 · p95 0.141 · p99 0.154 · 2741 op/s · total p50 0.338</sub> | -70.5% (-0.200) | 150% AND 2 ms | 🟢 |
| 8 | 0.372<br><sub>context: p90 0.496 · p95 0.540 · p99 0.657 · 13663 op/s · total p50 0.553</sub> | 0.088<br><sub>context: p90 0.119 · p95 0.128 · p99 0.151 · 18616 op/s · total p50 0.413</sub> | -76.4% (-0.284) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.194 · p95 0.215 · p99 0.227 · 3885 op/s · total p50 0.240</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 6434 op/s · total p50 0.133</sub> | -98.7% (-0.121) | 150% AND 2 ms | 🟢 |
| 8 | 0.194<br><sub>context: p90 0.274 · p95 0.304 · p99 0.385 · 23186 op/s · total p50 0.326</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 33656 op/s · total p50 0.226</sub> | -98.8% (-0.191) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.092<br><sub>context: p90 0.155 · p95 0.161 · p99 0.172 · 3996 op/s · total p50 0.225</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 6782 op/s · total p50 0.134</sub> | -97.9% (-0.090) | 150% AND 2 ms | 🟢 |
| 8 | 0.143<br><sub>context: p90 0.201 · p95 0.222 · p99 0.273 · 27426 op/s · total p50 0.272</sub> | 0.002<br><sub>context: p90 0.003 · p95 0.004 · p99 0.005 · 37779 op/s · total p50 0.203</sub> | -98.7% (-0.141) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.336<br><sub>context: p90 0.414 · p95 0.424 · p99 0.440 · 1831 op/s · total p50 0.525</sub> | 0.081<br><sub>context: p90 0.111 · p95 0.119 · p99 0.144 · 2347 op/s · total p50 0.425</sub> | -76.0% (-0.255) | 150% AND 2 ms | 🟢 |
| 8 | 0.425<br><sub>context: p90 0.541 · p95 0.594 · p99 0.704 · 12316 op/s · total p50 0.621</sub> | 0.076<br><sub>context: p90 0.106 · p95 0.114 · p99 0.130 · 17052 op/s · total p50 0.447</sub> | -82.1% (-0.349) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.670<br><sub>context: p90 0.942 · p95 1.016 · p99 1.114 · 1105 op/s · total p50 0.901</sub> | 1.063<br><sub>context: p90 1.632 · p95 1.878 · p99 2.164 · 658 op/s · total p50 1.483</sub> | +58.6% (+0.393) | 150% AND 2 ms | 🟢 |
| 8 | 0.724<br><sub>context: p90 1.086 · p95 1.197 · p99 1.445 · 7647 op/s · total p50 0.979</sub> | 2.896<br><sub>context: p90 4.830 · p95 5.488 · p99 6.545 · 2294 op/s · total p50 3.311</sub> | +300.2% (+2.173) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.099<br><sub>context: p90 3.231 · p95 3.678 · p99 4.055 · 408 op/s · total p50 2.381</sub> | 4.076<br><sub>context: p90 6.299 · p95 7.061 · p99 7.479 · 210 op/s · total p50 4.625</sub> | +94.2% (+1.977) | 150% AND 2 ms | 🟢 |
| 8 | 2.190<br><sub>context: p90 3.653 · p95 4.064 · p99 4.708 · 3021 op/s · total p50 2.432</sub> | 12.997<br><sub>context: p90 19.913 · p95 22.040 · p99 23.834 · 578 op/s · total p50 13.448</sub> | +493.4% (+10.807) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.186 · p95 0.208 · p99 0.221 · 3517 op/s · total p50 0.250</sub> | 0.018<br><sub>context: p90 0.043 · p95 0.046 · p99 0.051 · 4924 op/s · total p50 0.175</sub> | -83.0% (-0.086) | 150% AND 2 ms | 🟢 |
| 8 | 0.177<br><sub>context: p90 0.239 · p95 0.265 · p99 0.318 · 21694 op/s · total p50 0.337</sub> | 0.018<br><sub>context: p90 0.024 · p95 0.027 · p99 0.034 · 29691 op/s · total p50 0.257</sub> | -89.9% (-0.160) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.127<br><sub>context: p90 0.204 · p95 0.211 · p99 0.220 · 3440 op/s · total p50 0.269</sub> | 0.016<br><sub>context: p90 0.039 · p95 0.043 · p99 0.046 · 4679 op/s · total p50 0.190</sub> | -87.2% (-0.111) | 150% AND 2 ms | 🟢 |
| 8 | 0.179<br><sub>context: p90 0.238 · p95 0.268 · p99 0.324 · 22042 op/s · total p50 0.341</sub> | 0.018<br><sub>context: p90 0.024 · p95 0.027 · p99 0.034 · 28282 op/s · total p50 0.267</sub> | -90.2% (-0.161) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.142<br><sub>context: p90 0.212 · p95 0.222 · p99 0.231 · 2674 op/s · total p50 0.360</sub> | 0.009<br><sub>context: p90 0.012 · p95 0.013 · p99 0.016 · 3736 op/s · total p50 0.255</sub> | -93.8% (-0.134) | 150% AND 2 ms | 🟢 |
| 8 | 0.189<br><sub>context: p90 0.251 · p95 0.275 · p99 0.319 · 16694 op/s · total p50 0.457</sub> | 0.011<br><sub>context: p90 0.015 · p95 0.017 · p99 0.022 · 22098 op/s · total p50 0.344</sub> | -94.4% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.149<br><sub>context: p90 0.181 · p95 0.187 · p99 0.200 · 2642 op/s · total p50 0.354</sub> | 0.006<br><sub>context: p90 0.015 · p95 0.016 · p99 0.018 · 5720 op/s · total p50 0.160</sub> | -96.0% (-0.143) | 150% AND 2 ms | 🟢 |
| 8 | 0.152<br><sub>context: p90 0.211 · p95 0.238 · p99 0.290 · 26627 op/s · total p50 0.282</sub> | 0.005<br><sub>context: p90 0.008 · p95 0.009 · p99 0.011 · 34681 op/s · total p50 0.221</sub> | -96.4% (-0.146) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.226<br><sub>context: p90 0.288 · p95 0.297 · p99 0.330 · 2511 op/s · total p50 0.386</sub> | 0.133<br><sub>context: p90 0.172 · p95 0.184 · p99 0.209 · 2305 op/s · total p50 0.429</sub> | -41.2% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.273<br><sub>context: p90 0.348 · p95 0.374 · p99 0.429 · 17435 op/s · total p50 0.436</sub> | 0.129<br><sub>context: p90 0.167 · p95 0.178 · p99 0.196 · 20136 op/s · total p50 0.383</sub> | -52.6% (-0.144) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.226 · p95 0.239 · p99 0.255 · 3357 op/s · total p50 0.274</sub> | 0.060<br><sub>context: p90 0.088 · p95 0.103 · p99 0.108 · 3980 op/s · total p50 0.234</sub> | -60.5% (-0.093) | 150% AND 2 ms | 🟢 |
| 8 | 0.220<br><sub>context: p90 0.289 · p95 0.319 · p99 0.375 · 20816 op/s · total p50 0.365</sub> | 0.066<br><sub>context: p90 0.095 · p95 0.102 · p99 0.123 · 26045 op/s · total p50 0.295</sub> | -70.0% (-0.154) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.261<br><sub>context: p90 0.323 · p95 0.327 · p99 0.345 · 2151 op/s · total p50 0.451</sub> | 0.102<br><sub>context: p90 0.145 · p95 0.156 · p99 0.170 · 2277 op/s · total p50 0.431</sub> | -60.8% (-0.159) | 150% AND 2 ms | 🟢 |
| 8 | 0.297<br><sub>context: p90 0.385 · p95 0.417 · p99 0.497 · 15134 op/s · total p50 0.500</sub> | 0.100<br><sub>context: p90 0.136 · p95 0.146 · p99 0.168 · 17792 op/s · total p50 0.431</sub> | -66.2% (-0.196) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.264<br><sub>context: p90 0.318 · p95 0.324 · p99 0.338 · 1672 op/s · total p50 0.588</sub> | 0.123<br><sub>context: p90 0.160 · p95 0.163 · p99 0.172 · 1570 op/s · total p50 0.619</sub> | -53.4% (-0.141) | 150% AND 2 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.384 · p95 0.410 · p99 0.453 · 11703 op/s · total p50 0.642</sub> | 0.108<br><sub>context: p90 0.146 · p95 0.157 · p99 0.177 · 13243 op/s · total p50 0.578</sub> | -64.0% (-0.192) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.309<br><sub>context: p90 0.353 · p95 0.359 · p99 0.381 · 1658 op/s · total p50 0.596</sub> | 0.126<br><sub>context: p90 0.153 · p95 0.164 · p99 0.178 · 1802 op/s · total p50 0.549</sub> | -59.2% (-0.183) | 150% AND 2 ms | 🟢 |
| 8 | 0.330<br><sub>context: p90 0.419 · p95 0.453 · p99 0.522 · 11667 op/s · total p50 0.658</sub> | 0.114<br><sub>context: p90 0.149 · p95 0.159 · p99 0.180 · 12775 op/s · total p50 0.604</sub> | -65.5% (-0.216) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.298<br><sub>context: p90 0.356 · p95 0.365 · p99 0.386 · 1759 op/s · total p50 0.557</sub> | 0.110<br><sub>context: p90 0.150 · p95 0.157 · p99 0.172 · 2304 op/s · total p50 0.426</sub> | -63.0% (-0.188) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.436 · p95 0.477 · p99 0.558 · 13595 op/s · total p50 0.550</sub> | 0.104<br><sub>context: p90 0.137 · p95 0.148 · p99 0.171 · 16161 op/s · total p50 0.475</sub> | -68.8% (-0.229) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.271<br><sub>context: p90 0.354 · p95 0.365 · p99 0.385 · 2217 op/s · total p50 0.438</sub> | 0.083<br><sub>context: p90 0.124 · p95 0.135 · p99 0.145 · 2860 op/s · total p50 0.334</sub> | -69.2% (-0.188) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.442 · p95 0.486 · p99 0.578 · 14905 op/s · total p50 0.511</sub> | 0.084<br><sub>context: p90 0.116 · p95 0.126 · p99 0.144 · 19124 op/s · total p50 0.401</sub> | -75.1% (-0.254) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.781<br><sub>context: p90 5.986 · p95 6.032 · p99 6.250 · 60 op/s · total p50 16.734</sub> | 2.517<br><sub>context: p90 2.560 · p95 2.585 · p99 2.620 · 74 op/s · total p50 13.522</sub> | -56.5% (-3.264) | 150% AND 2 ms | 🟢 |
| 8 | 7.237<br><sub>context: p90 8.850 · p95 9.272 · p99 10.230 · 372 op/s · total p50 20.842</sub> | 3.162<br><sub>context: p90 3.510 · p95 3.712 · p99 3.835 · 470 op/s · total p50 15.996</sub> | -56.3% (-4.075) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.332<br><sub>context: p90 0.422 · p95 0.441 · p99 0.449 · 1797 op/s · total p50 0.531</sub> | 0.117<br><sub>context: p90 0.162 · p95 0.170 · p99 0.183 · 2115 op/s · total p50 0.459</sub> | -64.6% (-0.215) | 150% AND 2 ms | 🟢 |
| 8 | 0.428<br><sub>context: p90 0.545 · p95 0.585 · p99 0.674 · 12601 op/s · total p50 0.601</sub> | 0.111<br><sub>context: p90 0.142 · p95 0.153 · p99 0.179 · 16682 op/s · total p50 0.462</sub> | -74.2% (-0.318) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.570<br><sub>context: p90 0.668 · p95 0.691 · p99 0.741 · 656 op/s · total p50 1.501</sub> | 0.269<br><sub>context: p90 0.331 · p95 0.345 · p99 0.370 · 697 op/s · total p50 1.419</sub> | -52.8% (-0.301) | 150% AND 2 ms | 🟢 |
| 8 | 0.614<br><sub>context: p90 0.782 · p95 0.836 · p99 0.925 · 4632 op/s · total p50 1.672</sub> | 0.264<br><sub>context: p90 0.344 · p95 0.365 · p99 0.395 · 5413 op/s · total p50 1.437</sub> | -57.1% (-0.350) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.345 · p95 0.354 · p99 0.375 · 1811 op/s · total p50 0.554</sub> | 0.098<br><sub>context: p90 0.153 · p95 0.163 · p99 0.171 · 2589 op/s · total p50 0.371</sub> | -67.2% (-0.201) | 150% AND 2 ms | 🟢 |
| 8 | 0.315<br><sub>context: p90 0.405 · p95 0.432 · p99 0.510 · 14090 op/s · total p50 0.534</sub> | 0.104<br><sub>context: p90 0.139 · p95 0.148 · p99 0.168 · 16364 op/s · total p50 0.469</sub> | -67.1% (-0.211) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.741<br><sub>context: p90 2.194 · p95 2.242 · p99 2.330 · 490 op/s · total p50 2.085</sub> | 0.133<br><sub>context: p90 0.183 · p95 0.210 · p99 0.234 · 2076 op/s · total p50 0.472</sub> | -92.3% (-1.608) | 150% AND 2 ms | 🟢 |
| 8 | 1.778<br><sub>context: p90 2.293 · p95 2.389 · p99 2.547 · 3938 op/s · total p50 2.015</sub> | 0.116<br><sub>context: p90 0.167 · p95 0.182 · p99 0.210 · 19414 op/s · total p50 0.391</sub> | -93.5% (-1.662) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.772<br><sub>context: p90 2.278 · p95 2.316 · p99 2.394 · 482 op/s · total p50 2.123</sub> | 0.128<br><sub>context: p90 0.188 · p95 0.201 · p99 0.245 · 2366 op/s · total p50 0.409</sub> | -92.8% (-1.643) | 150% AND 2 ms | 🟢 |
| 8 | 1.837<br><sub>context: p90 2.355 · p95 2.446 · p99 2.612 · 3439 op/s · total p50 2.238</sub> | 0.122<br><sub>context: p90 0.178 · p95 0.199 · p99 0.229 · 17484 op/s · total p50 0.430</sub> | -93.4% (-1.715) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.130<br><sub>context: p90 0.183 · p95 0.188 · p99 0.195 · 3546 op/s · total p50 0.270</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.070 · p99 0.077 · 4050 op/s · total p50 0.217</sub> | -73.7% (-0.096) | 150% AND 2 ms | 🟢 |
| 8 | 0.150<br><sub>context: p90 0.203 · p95 0.223 · p99 0.273 · 25565 op/s · total p50 0.296</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.067 · p99 0.074 · 29323 op/s · total p50 0.260</sub> | -77.2% (-0.116) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.248 · p95 0.254 · p99 0.267 · 2995 op/s · total p50 0.317</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.005 · p99 0.007 · 5362 op/s · total p50 0.172</sub> | -98.0% (-0.150) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.235<br><sub>context: p90 0.309 · p95 0.337 · p99 0.418 · 18850 op/s · total p50 0.400</sub> | 0.004<br><sub>context: p90 0.005 · p95 0.006 · p99 0.008 · 28619 op/s · total p50 0.267</sub> | -98.4% (-0.231) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.260 · p95 0.264 · p99 0.272 · 2556 op/s · total p50 0.381</sub> | 0.041<br><sub>context: p90 0.068 · p95 0.072 · p99 0.094 · 3804 op/s · total p50 0.229</sub> | -78.1% (-0.145) | 150% AND 2 ms | 🟢 |
| 8 | 0.279<br><sub>context: p90 0.371 · p95 0.409 · p99 0.485 · 17548 op/s · total p50 0.434</sub> | 0.045<br><sub>context: p90 0.074 · p95 0.078 · p99 0.089 · 25222 op/s · total p50 0.304</sub> | -83.8% (-0.234) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.182<br><sub>context: p90 0.271 · p95 0.278 · p99 0.297 · 2682 op/s · total p50 0.349</sub> | 0.064<br><sub>context: p90 0.113 · p95 0.117 · p99 0.125 · 3433 op/s · total p50 0.278</sub> | -64.7% (-0.118) | 150% AND 2 ms | 🟢 |
| 8 | 0.295<br><sub>context: p90 0.401 · p95 0.449 · p99 0.547 · 17252 op/s · total p50 0.441</sub> | 0.066<br><sub>context: p90 0.125 · p95 0.132 · p99 0.140 · 23876 op/s · total p50 0.320</sub> | -77.8% (-0.230) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.236 · p95 0.248 · p99 0.259 · 3078 op/s · total p50 0.296</sub> | 0.035<br><sub>context: p90 0.062 · p95 0.075 · p99 0.082 · 4478 op/s · total p50 0.208</sub> | -77.5% (-0.121) | 150% AND 2 ms | 🟢 |
| 8 | 0.230<br><sub>context: p90 0.312 · p95 0.339 · p99 0.417 · 20373 op/s · total p50 0.374</sub> | 0.037<br><sub>context: p90 0.067 · p95 0.070 · p99 0.078 · 27351 op/s · total p50 0.279</sub> | -83.9% (-0.193) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.000<br><sub>context: p90 1.067 · p95 1.095 · p99 1.114 · 721 op/s · total p50 1.371</sub> | 0.530<br><sub>context: p90 0.560 · p95 0.571 · p99 0.588 · 960 op/s · total p50 1.032</sub> | -47.0% (-0.470) | 150% AND 2 ms | 🟢 |
| 8 | 1.083<br><sub>context: p90 1.332 · p95 1.418 · p99 1.756 · 4953 op/s · total p50 1.512</sub> | 0.532<br><sub>context: p90 0.611 · p95 0.640 · p99 0.678 · 6754 op/s · total p50 1.126</sub> | -50.9% (-0.551) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.014<br><sub>context: p90 1.091 · p95 1.110 · p99 1.123 · 786 op/s · total p50 1.272</sub> | 0.518<br><sub>context: p90 0.569 · p95 0.579 · p99 0.609 · 1115 op/s · total p50 0.898</sub> | -48.9% (-0.496) | 150% AND 2 ms | 🟢 |
| 8 | 1.067<br><sub>context: p90 1.243 · p95 1.308 · p99 1.501 · 5576 op/s · total p50 1.290</sub> | 0.551<br><sub>context: p90 0.670 · p95 0.707 · p99 0.765 · 8052 op/s · total p50 0.923</sub> | -48.4% (-0.516) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.201<br><sub>context: p90 0.280 · p95 0.288 · p99 0.304 · 2680 op/s · total p50 0.362</sub> | 0.061<br><sub>context: p90 0.095 · p95 0.107 · p99 0.124 · 3073 op/s · total p50 0.294</sub> | -69.8% (-0.140) | 150% AND 2 ms | 🟢 |
| 8 | 0.250<br><sub>context: p90 0.333 · p95 0.363 · p99 0.434 · 16655 op/s · total p50 0.455</sub> | 0.063<br><sub>context: p90 0.092 · p95 0.099 · p99 0.111 · 20492 op/s · total p50 0.376</sub> | -74.7% (-0.187) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.370<br><sub>context: p90 0.449 · p95 0.472 · p99 0.500 · 1774 op/s · total p50 0.560</sub> | 0.195<br><sub>context: p90 0.282 · p95 0.307 · p99 0.344 · 1909 op/s · total p50 0.521</sub> | -47.2% (-0.175) | 150% AND 2 ms | 🟢 |
| 8 | 0.391<br><sub>context: p90 0.506 · p95 0.544 · p99 0.604 · 13361 op/s · total p50 0.574</sub> | 0.201<br><sub>context: p90 0.307 · p95 0.340 · p99 0.398 · 14743 op/s · total p50 0.519</sub> | -48.7% (-0.190) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.186<br><sub>context: p90 0.245 · p95 0.254 · p99 0.265 · 2739 op/s · total p50 0.363</sub> | 0.021<br><sub>context: p90 0.030 · p95 0.038 · p99 0.043 · 4325 op/s · total p50 0.211</sub> | -88.9% (-0.165) | 150% AND 2 ms | 🟢 |
| 8 | 0.208<br><sub>context: p90 0.274 · p95 0.305 · p99 0.369 · 20416 op/s · total p50 0.368</sub> | 0.027<br><sub>context: p90 0.037 · p95 0.042 · p99 0.050 · 24541 op/s · total p50 0.311</sub> | -87.0% (-0.181) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.132<br><sub>context: p90 0.187 · p95 0.192 · p99 0.207 · 3212 op/s · total p50 0.301</sub> | 0.036<br><sub>context: p90 0.065 · p95 0.077 · p99 0.089 · 4419 op/s · total p50 0.203</sub> | -72.5% (-0.096) | 150% AND 2 ms | 🟢 |
| 8 | 0.155<br><sub>context: p90 0.218 · p95 0.243 · p99 0.288 · 23397 op/s · total p50 0.311</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.066 · p99 0.073 · 30212 op/s · total p50 0.254</sub> | -78.1% (-0.121) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.122<br><sub>context: p90 0.180 · p95 0.186 · p99 0.192 · 3772 op/s · total p50 0.248</sub> | 0.032<br><sub>context: p90 0.058 · p95 0.062 · p99 0.066 · 5190 op/s · total p50 0.183</sub> | -74.0% (-0.090) | 150% AND 2 ms | 🟢 |
| 8 | 0.151<br><sub>context: p90 0.199 · p95 0.222 · p99 0.262 · 25779 op/s · total p50 0.295</sub> | 0.034<br><sub>context: p90 0.063 · p95 0.065 · p99 0.071 · 30248 op/s · total p50 0.254</sub> | -77.4% (-0.117) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.754<br><sub>context: p90 0.777 · p95 0.783 · p99 0.817 · 968 op/s · total p50 1.007</sub> | 0.176<br><sub>context: p90 0.200 · p95 0.205 · p99 0.214 · 2500 op/s · total p50 0.382</sub> | -76.6% (-0.578) | 150% AND 2 ms | 🟢 |
| 8 | 0.791<br><sub>context: p90 1.172 · p95 1.342 · p99 1.515 · 6616 op/s · total p50 1.020</sub> | 0.187<br><sub>context: p90 0.217 · p95 0.224 · p99 0.237 · 18082 op/s · total p50 0.429</sub> | -76.4% (-0.604) | 150% AND 2 ms | 🟢 |

</details>
