### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.287<br><sub>context: p90 1.348 · p95 1.369 · p99 1.408 · 618 op/s · total p50 1.615</sub> | 0.578<br><sub>context: p90 0.604 · p95 0.609 · p99 0.630 · 1079 op/s · total p50 0.917</sub> | -55.1% (-0.710) | 150% AND 2 ms | 🟢 |
| 8 | 1.778<br><sub>context: p90 2.342 · p95 2.565 · p99 2.988 · 3892 op/s · total p50 1.989</sub> | 0.698<br><sub>context: p90 0.820 · p95 0.840 · p99 0.878 · 8540 op/s · total p50 0.927</sub> | -60.8% (-1.081) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.989<br><sub>context: p90 2.057 · p95 2.091 · p99 2.152 · 441 op/s · total p50 2.268</sub> | 0.630<br><sub>context: p90 0.665 · p95 0.673 · p99 0.694 · 1041 op/s · total p50 0.946</sub> | -68.3% (-1.359) | 150% AND 2 ms | 🟢 |
| 8 | 2.583<br><sub>context: p90 3.345 · p95 3.496 · p99 3.959 · 2738 op/s · total p50 2.819</sub> | 0.799<br><sub>context: p90 0.973 · p95 0.995 · p99 1.049 · 7592 op/s · total p50 1.038</sub> | -69.1% (-1.784) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.922<br><sub>context: p90 2.003 · p95 2.032 · p99 2.063 · 439 op/s · total p50 2.262</sub> | 1.116<br><sub>context: p90 1.163 · p95 1.177 · p99 1.214 · 644 op/s · total p50 1.552</sub> | -42.0% (-0.806) | 150% AND 2 ms | 🟢 |
| 8 | 2.638<br><sub>context: p90 3.401 · p95 3.595 · p99 3.930 · 2714 op/s · total p50 2.882</sub> | 1.577<br><sub>context: p90 2.065 · p95 2.230 · p99 2.531 · 4102 op/s · total p50 1.850</sub> | -40.2% (-1.061) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.565<br><sub>context: p90 2.663 · p95 2.685 · p99 2.749 · 336 op/s · total p50 2.969</sub> | 1.182<br><sub>context: p90 1.241 · p95 1.262 · p99 1.316 · 642 op/s · total p50 1.549</sub> | -53.9% (-1.383) | 150% AND 2 ms | 🟢 |
| 8 | 3.572<br><sub>context: p90 4.525 · p95 4.792 · p99 5.176 · 2032 op/s · total p50 3.809</sub> | 1.399<br><sub>context: p90 1.714 · p95 1.752 · p99 1.802 · 4630 op/s · total p50 1.689</sub> | -60.8% (-2.173) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.128 · p95 0.132 · p99 0.145 · 3976 op/s · total p50 0.242</sub> | 0.025<br><sub>context: p90 0.046 · p95 0.050 · p99 0.052 · 5061 op/s · total p50 0.177</sub> | -75.6% (-0.078) | 150% AND 2 ms | 🟢 |
| 8 | 0.126<br><sub>context: p90 0.177 · p95 0.197 · p99 0.243 · 28979 op/s · total p50 0.256</sub> | 0.025<br><sub>context: p90 0.035 · p95 0.039 · p99 0.046 · 33283 op/s · total p50 0.230</sub> | -79.8% (-0.101) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.266<br><sub>context: p90 0.286 · p95 0.295 · p99 0.305 · 1760 op/s · total p50 0.556</sub> | 0.069<br><sub>context: p90 0.098 · p95 0.102 · p99 0.118 · 2557 op/s · total p50 0.372</sub> | -74.1% (-0.197) | 150% AND 2 ms | 🟢 |
| 8 | 0.324<br><sub>context: p90 0.430 · p95 0.480 · p99 0.574 · 14408 op/s · total p50 0.522</sub> | 0.086<br><sub>context: p90 0.161 · p95 0.195 · p99 0.258 · 21542 op/s · total p50 0.344</sub> | -73.3% (-0.238) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.310 · p95 0.316 · p99 0.334 · 1716 op/s · total p50 0.582</sub> | 0.074<br><sub>context: p90 0.101 · p95 0.108 · p99 0.116 · 2092 op/s · total p50 0.465</sub> | -74.2% (-0.211) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.422 · p95 0.454 · p99 0.527 · 14519 op/s · total p50 0.515</sub> | 0.085<br><sub>context: p90 0.153 · p95 0.179 · p99 0.225 · 20717 op/s · total p50 0.367</sub> | -74.7% (-0.252) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.303<br><sub>context: p90 0.327 · p95 0.336 · p99 0.343 · 1620 op/s · total p50 0.608</sub> | 0.141<br><sub>context: p90 0.175 · p95 0.185 · p99 0.197 · 1873 op/s · total p50 0.526</sub> | -53.6% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.347<br><sub>context: p90 0.427 · p95 0.448 · p99 0.507 · 14119 op/s · total p50 0.547</sub> | 0.147<br><sub>context: p90 0.204 · p95 0.230 · p99 0.279 · 16216 op/s · total p50 0.478</sub> | -57.5% (-0.199) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.308<br><sub>context: p90 0.350 · p95 0.362 · p99 0.388 · 1645 op/s · total p50 0.608</sub> | 0.148<br><sub>context: p90 0.176 · p95 0.183 · p99 0.191 · 1734 op/s · total p50 0.563</sub> | -51.9% (-0.160) | 150% AND 2 ms | 🟢 |
| 8 | 0.387<br><sub>context: p90 0.469 · p95 0.500 · p99 0.566 · 13103 op/s · total p50 0.594</sub> | 0.152<br><sub>context: p90 0.211 · p95 0.235 · p99 0.287 · 15790 op/s · total p50 0.491</sub> | -60.8% (-0.235) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.369<br><sub>context: p90 0.421 · p95 0.437 · p99 0.461 · 1257 op/s · total p50 0.781</sub> | 0.176<br><sub>context: p90 0.216 · p95 0.227 · p99 0.242 · 1412 op/s · total p50 0.695</sub> | -52.4% (-0.193) | 150% AND 2 ms | 🟢 |
| 8 | 0.434<br><sub>context: p90 0.535 · p95 0.565 · p99 0.664 · 9932 op/s · total p50 0.779</sub> | 0.195<br><sub>context: p90 0.252 · p95 0.273 · p99 0.329 · 11581 op/s · total p50 0.668</sub> | -55.1% (-0.239) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.391<br><sub>context: p90 0.452 · p95 0.474 · p99 0.507 · 1256 op/s · total p50 0.785</sub> | 0.183<br><sub>context: p90 0.230 · p95 0.244 · p99 0.254 · 1523 op/s · total p50 0.645</sub> | -53.1% (-0.207) | 150% AND 2 ms | 🟢 |
| 8 | 0.481<br><sub>context: p90 0.595 · p95 0.628 · p99 0.700 · 9419 op/s · total p50 0.827</sub> | 0.201<br><sub>context: p90 0.271 · p95 0.292 · p99 0.347 · 11029 op/s · total p50 0.698</sub> | -58.3% (-0.280) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.599<br><sub>context: p90 0.737 · p95 0.796 · p99 0.879 · 699 op/s · total p50 1.391</sub> | 0.322<br><sub>context: p90 0.403 · p95 0.425 · p99 0.480 · 794 op/s · total p50 1.231</sub> | -46.2% (-0.277) | 150% AND 2 ms | 🟢 |
| 8 | 0.713<br><sub>context: p90 0.946 · p95 1.021 · p99 1.167 · 4722 op/s · total p50 1.620</sub> | 0.340<br><sub>context: p90 0.448 · p95 0.478 · p99 0.549 · 5628 op/s · total p50 1.377</sub> | -52.3% (-0.373) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.674<br><sub>context: p90 0.850 · p95 0.918 · p99 1.024 · 652 op/s · total p50 1.504</sub> | 0.334<br><sub>context: p90 0.428 · p95 0.450 · p99 0.509 · 736 op/s · total p50 1.330</sub> | -50.4% (-0.340) | 150% AND 2 ms | 🟢 |
| 8 | 0.802<br><sub>context: p90 1.076 · p95 1.168 · p99 1.349 · 4435 op/s · total p50 1.702</sub> | 0.360<br><sub>context: p90 0.481 · p95 0.514 · p99 0.613 · 5356 op/s · total p50 1.428</sub> | -55.1% (-0.442) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.517<br><sub>context: p90 0.591 · p95 0.612 · p99 0.649 · 1304 op/s · total p50 0.764</sub> | 0.380<br><sub>context: p90 0.568 · p95 0.626 · p99 0.669 · 1261 op/s · total p50 0.782</sub> | -26.5% (-0.137) | 150% AND 2 ms | 🟢 |
| 8 | 0.630<br><sub>context: p90 0.766 · p95 0.808 · p99 0.883 · 9654 op/s · total p50 0.811</sub> | 0.511<br><sub>context: p90 0.773 · p95 0.838 · p99 0.950 · 9389 op/s · total p50 0.837</sub> | -18.9% (-0.119) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.354 · p95 0.363 · p99 0.384 · 1796 op/s · total p50 0.554</sub> | 0.123<br><sub>context: p90 0.152 · p95 0.157 · p99 0.162 · 1877 op/s · total p50 0.530</sub> | -61.9% (-0.200) | 150% AND 2 ms | 🟢 |
| 8 | 0.412<br><sub>context: p90 0.503 · p95 0.539 · p99 0.609 · 13280 op/s · total p50 0.581</sub> | 0.135<br><sub>context: p90 0.215 · p95 0.249 · p99 0.300 · 15039 op/s · total p50 0.479</sub> | -67.3% (-0.277) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.181<br><sub>context: p90 0.203 · p95 0.209 · p99 0.219 · 2578 op/s · total p50 0.383</sub> | 0.005<br><sub>context: p90 0.005 · p95 0.006 · p99 0.009 · 3664 op/s · total p50 0.263</sub> | -97.3% (-0.176) | 150% AND 2 ms | 🟢 |
| 8 | 0.213<br><sub>context: p90 0.295 · p95 0.324 · p99 0.398 · 19618 op/s · total p50 0.363</sub> | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.006 · 32132 op/s · total p50 0.237</sub> | -98.6% (-0.210) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.128<br><sub>context: p90 0.149 · p95 0.152 · p99 0.167 · 3413 op/s · total p50 0.283</sub> | 0.005<br><sub>context: p90 0.006 · p95 0.006 · p99 0.009 · 4309 op/s · total p50 0.209</sub> | -96.1% (-0.123) | 150% AND 2 ms | 🟢 |
| 8 | 0.155<br><sub>context: p90 0.217 · p95 0.241 · p99 0.295 · 24721 op/s · total p50 0.294</sub> | 0.003<br><sub>context: p90 0.005 · p95 0.006 · p99 0.007 · 30828 op/s · total p50 0.237</sub> | -98.1% (-0.152) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.399<br><sub>context: p90 0.434 · p95 0.444 · p99 0.454 · 1338 op/s · total p50 0.747</sub> | 0.095<br><sub>context: p90 0.123 · p95 0.133 · p99 0.147 · 1586 op/s · total p50 0.622</sub> | -76.1% (-0.304) | 150% AND 2 ms | 🟢 |
| 8 | 0.495<br><sub>context: p90 0.603 · p95 0.637 · p99 0.719 · 11067 op/s · total p50 0.700</sub> | 0.106<br><sub>context: p90 0.177 · p95 0.208 · p99 0.266 · 15910 op/s · total p50 0.482</sub> | -78.6% (-0.389) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.729<br><sub>context: p90 1.041 · p95 1.154 · p99 1.238 · 895 op/s · total p50 1.085</sub> | 1.246<br><sub>context: p90 1.900 · p95 2.166 · p99 2.479 · 585 op/s · total p50 1.691</sub> | +70.9% (+0.517) | 150% AND 2 ms | 🟢 |
| 8 | 0.949<br><sub>context: p90 1.452 · p95 1.647 · p99 1.969 · 6277 op/s · total p50 1.183</sub> | 1.982<br><sub>context: p90 3.260 · p95 3.684 · p99 4.241 · 3277 op/s · total p50 2.338</sub> | +108.9% (+1.033) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.389<br><sub>context: p90 3.850 · p95 4.392 · p99 4.609 · 345 op/s · total p50 2.790</sub> | 4.768<br><sub>context: p90 7.402 · p95 8.075 · p99 9.001 · 183 op/s · total p50 5.253</sub> | +99.6% (+2.379) | 150% AND 2 ms | 🟢 |
| 8 | 3.172<br><sub>context: p90 5.606 · p95 6.422 · p99 8.170 · 2125 op/s · total p50 3.452</sub> | 8.161<br><sub>context: p90 13.047 · p95 14.316 · p99 16.110 · 889 op/s · total p50 8.545</sub> | +157.3% (+4.989) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.179<br><sub>context: p90 0.203 · p95 0.209 · p99 0.218 · 2331 op/s · total p50 0.428</sub> | 0.034<br><sub>context: p90 0.040 · p95 0.042 · p99 0.044 · 3278 op/s · total p50 0.295</sub> | -80.9% (-0.145) | 150% AND 2 ms | 🟢 |
| 8 | 0.208<br><sub>context: p90 0.284 · p95 0.317 · p99 0.372 · 18466 op/s · total p50 0.400</sub> | 0.022<br><sub>context: p90 0.031 · p95 0.035 · p99 0.042 · 23719 op/s · total p50 0.317</sub> | -89.4% (-0.186) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.207 · p95 0.210 · p99 0.225 · 2286 op/s · total p50 0.431</sub> | 0.030<br><sub>context: p90 0.037 · p95 0.039 · p99 0.042 · 4049 op/s · total p50 0.245</sub> | -83.8% (-0.155) | 150% AND 2 ms | 🟢 |
| 8 | 0.210<br><sub>context: p90 0.279 · p95 0.306 · p99 0.365 · 19115 op/s · total p50 0.393</sub> | 0.022<br><sub>context: p90 0.031 · p95 0.035 · p99 0.043 · 23328 op/s · total p50 0.320</sub> | -89.6% (-0.189) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.216 · p95 0.221 · p99 0.230 · 1674 op/s · total p50 0.605</sub> | 0.016<br><sub>context: p90 0.019 · p95 0.020 · p99 0.024 · 2158 op/s · total p50 0.448</sub> | -91.6% (-0.178) | 150% AND 2 ms | 🟢 |
| 8 | 0.219<br><sub>context: p90 0.287 · p95 0.307 · p99 0.346 · 14557 op/s · total p50 0.527</sub> | 0.013<br><sub>context: p90 0.018 · p95 0.021 · p99 0.024 · 19763 op/s · total p50 0.383</sub> | -94.0% (-0.206) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.159<br><sub>context: p90 0.175 · p95 0.180 · p99 0.192 · 2579 op/s · total p50 0.379</sub> | 0.007<br><sub>context: p90 0.012 · p95 0.013 · p99 0.015 · 5768 op/s · total p50 0.160</sub> | -95.5% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.174<br><sub>context: p90 0.237 · p95 0.261 · p99 0.310 · 24080 op/s · total p50 0.313</sub> | 0.007<br><sub>context: p90 0.010 · p95 0.012 · p99 0.014 · 32281 op/s · total p50 0.235</sub> | -96.0% (-0.167) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.306 · p95 0.315 · p99 0.330 · 1620 op/s · total p50 0.608</sub> | 0.157<br><sub>context: p90 0.199 · p95 0.207 · p99 0.250 · 1824 op/s · total p50 0.547</sub> | -45.0% (-0.128) | 150% AND 2 ms | 🟢 |
| 8 | 0.329<br><sub>context: p90 0.413 · p95 0.438 · p99 0.502 · 14910 op/s · total p50 0.505</sub> | 0.174<br><sub>context: p90 0.244 · p95 0.269 · p99 0.329 · 17405 op/s · total p50 0.436</sub> | -47.1% (-0.155) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.221<br><sub>context: p90 0.244 · p95 0.253 · p99 0.274 · 1913 op/s · total p50 0.502</sub> | 0.087<br><sub>context: p90 0.116 · p95 0.121 · p99 0.139 · 2380 op/s · total p50 0.421</sub> | -60.9% (-0.135) | 150% AND 2 ms | 🟢 |
| 8 | 0.247<br><sub>context: p90 0.308 · p95 0.328 · p99 0.377 · 19293 op/s · total p50 0.396</sub> | 0.108<br><sub>context: p90 0.182 · p95 0.214 · p99 0.276 · 21030 op/s · total p50 0.356</sub> | -56.5% (-0.140) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.294<br><sub>context: p90 0.322 · p95 0.336 · p99 0.348 · 1640 op/s · total p50 0.616</sub> | 0.131<br><sub>context: p90 0.160 · p95 0.171 · p99 0.194 · 1882 op/s · total p50 0.523</sub> | -55.3% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.344<br><sub>context: p90 0.441 · p95 0.481 · p99 0.558 · 13170 op/s · total p50 0.567</sub> | 0.144<br><sub>context: p90 0.205 · p95 0.227 · p99 0.269 · 16492 op/s · total p50 0.468</sub> | -58.1% (-0.200) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.286<br><sub>context: p90 0.313 · p95 0.320 · p99 0.337 · 1468 op/s · total p50 0.675</sub> | 0.132<br><sub>context: p90 0.165 · p95 0.172 · p99 0.184 · 1462 op/s · total p50 0.673</sub> | -53.9% (-0.154) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.408 · p95 0.429 · p99 0.487 · 11224 op/s · total p50 0.696</sub> | 0.148<br><sub>context: p90 0.223 · p95 0.251 · p99 0.310 · 11477 op/s · total p50 0.659</sub> | -55.5% (-0.185) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.351 · p95 0.357 · p99 0.376 · 1355 op/s · total p50 0.737</sub> | 0.133<br><sub>context: p90 0.170 · p95 0.174 · p99 0.188 · 1590 op/s · total p50 0.626</sub> | -58.3% (-0.186) | 150% AND 2 ms | 🟢 |
| 8 | 0.364<br><sub>context: p90 0.446 · p95 0.473 · p99 0.544 · 10763 op/s · total p50 0.714</sub> | 0.156<br><sub>context: p90 0.227 · p95 0.259 · p99 0.314 · 11719 op/s · total p50 0.660</sub> | -57.3% (-0.209) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.343 · p95 0.354 · p99 0.365 · 1657 op/s · total p50 0.596</sub> | 0.134<br><sub>context: p90 0.168 · p95 0.177 · p99 0.199 · 2080 op/s · total p50 0.476</sub> | -57.8% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.370<br><sub>context: p90 0.459 · p95 0.486 · p99 0.558 · 13385 op/s · total p50 0.577</sub> | 0.146<br><sub>context: p90 0.209 · p95 0.236 · p99 0.292 · 15400 op/s · total p50 0.499</sub> | -60.4% (-0.224) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.329<br><sub>context: p90 0.358 · p95 0.370 · p99 0.396 · 1587 op/s · total p50 0.624</sub> | 0.118<br><sub>context: p90 0.150 · p95 0.153 · p99 0.160 · 1826 op/s · total p50 0.538</sub> | -64.2% (-0.211) | 150% AND 2 ms | 🟢 |
| 8 | 0.396<br><sub>context: p90 0.489 · p95 0.524 · p99 0.606 · 13360 op/s · total p50 0.579</sub> | 0.124<br><sub>context: p90 0.196 · p95 0.219 · p99 0.293 · 18835 op/s · total p50 0.403</sub> | -68.8% (-0.273) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 6.362<br><sub>context: p90 6.657 · p95 6.739 · p99 6.869 · 52 op/s · total p50 19.184</sub> | 3.070<br><sub>context: p90 3.233 · p95 3.303 · p99 3.428 · 63 op/s · total p50 15.778</sub> | -51.7% (-3.292) | 150% AND 2 ms | 🟢 |
| 8 | 7.806<br><sub>context: p90 9.954 · p95 10.312 · p99 11.054 · 297 op/s · total p50 24.726</sub> | 3.338<br><sub>context: p90 4.258 · p95 4.363 · p99 4.552 · 368 op/s · total p50 19.613</sub> | -57.2% (-4.468) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.403<br><sub>context: p90 0.446 · p95 0.454 · p99 0.477 · 1489 op/s · total p50 0.667</sub> | 0.138<br><sub>context: p90 0.168 · p95 0.174 · p99 0.192 · 2035 op/s · total p50 0.486</sub> | -65.7% (-0.265) | 150% AND 2 ms | 🟢 |
| 8 | 0.512<br><sub>context: p90 0.622 · p95 0.659 · p99 0.734 · 11335 op/s · total p50 0.688</sub> | 0.147<br><sub>context: p90 0.206 · p95 0.234 · p99 0.283 · 16300 op/s · total p50 0.472</sub> | -71.2% (-0.365) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.595<br><sub>context: p90 0.694 · p95 0.729 · p99 0.768 · 574 op/s · total p50 1.711</sub> | 0.302<br><sub>context: p90 0.371 · p95 0.398 · p99 0.419 · 667 op/s · total p50 1.460</sub> | -49.2% (-0.293) | 150% AND 2 ms | 🟢 |
| 8 | 0.665<br><sub>context: p90 0.847 · p95 0.897 · p99 1.007 · 4034 op/s · total p50 1.934</sub> | 0.323<br><sub>context: p90 0.426 · p95 0.457 · p99 0.528 · 4524 op/s · total p50 1.708</sub> | -51.5% (-0.342) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.344 · p95 0.351 · p99 0.365 · 1551 op/s · total p50 0.643</sub> | 0.132<br><sub>context: p90 0.164 · p95 0.172 · p99 0.186 · 1937 op/s · total p50 0.497</sub> | -58.3% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.438 · p95 0.460 · p99 0.520 · 13338 op/s · total p50 0.582</sub> | 0.154<br><sub>context: p90 0.223 · p95 0.250 · p99 0.301 · 14853 op/s · total p50 0.520</sub> | -57.2% (-0.205) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.073<br><sub>context: p90 2.671 · p95 2.744 · p99 2.892 · 423 op/s · total p50 2.421</sub> | 0.142<br><sub>context: p90 0.189 · p95 0.198 · p99 0.225 · 1838 op/s · total p50 0.543</sub> | -93.2% (-1.931) | 150% AND 2 ms | 🟢 |
| 8 | 2.376<br><sub>context: p90 3.156 · p95 3.345 · p99 3.565 · 3049 op/s · total p50 2.625</sub> | 0.176<br><sub>context: p90 0.285 · p95 0.318 · p99 0.392 · 17108 op/s · total p50 0.451</sub> | -92.6% (-2.200) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.134<br><sub>context: p90 2.732 · p95 2.833 · p99 2.888 · 416 op/s · total p50 2.483</sub> | 0.142<br><sub>context: p90 0.200 · p95 0.214 · p99 0.248 · 1962 op/s · total p50 0.504</sub> | -93.3% (-1.992) | 150% AND 2 ms | 🟢 |
| 8 | 2.411<br><sub>context: p90 3.253 · p95 3.409 · p99 3.704 · 3031 op/s · total p50 2.663</sub> | 0.183<br><sub>context: p90 0.291 · p95 0.333 · p99 0.412 · 16138 op/s · total p50 0.471</sub> | -92.4% (-2.228) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.163<br><sub>context: p90 0.181 · p95 0.189 · p99 0.201 · 2743 op/s · total p50 0.350</sub> | 0.047<br><sub>context: p90 0.077 · p95 0.082 · p99 0.087 · 3367 op/s · total p50 0.287</sub> | -71.4% (-0.117) | 150% AND 2 ms | 🟢 |
| 8 | 0.177<br><sub>context: p90 0.248 · p95 0.278 · p99 0.339 · 21336 op/s · total p50 0.347</sub> | 0.079<br><sub>context: p90 0.167 · p95 0.205 · p99 0.280 · 24205 op/s · total p50 0.311</sub> | -55.5% (-0.098) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.235<br><sub>context: p90 0.255 · p95 0.261 · p99 0.271 · 1864 op/s · total p50 0.529</sub> | 0.004<br><sub>context: p90 0.009 · p95 0.009 · p99 0.011 · 4978 op/s · total p50 0.179</sub> | -98.2% (-0.231) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.289<br><sub>context: p90 0.352 · p95 0.375 · p99 0.442 · 16937 op/s · total p50 0.452</sub> | 0.005<br><sub>context: p90 0.006 · p95 0.007 · p99 0.010 · 30281 op/s · total p50 0.251</sub> | -98.4% (-0.284) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.265 · p95 0.274 · p99 0.293 · 2162 op/s · total p50 0.464</sub> | 0.064<br><sub>context: p90 0.094 · p95 0.098 · p99 0.103 · 2641 op/s · total p50 0.373</sub> | -73.4% (-0.176) | 150% AND 2 ms | 🟢 |
| 8 | 0.306<br><sub>context: p90 0.395 · p95 0.435 · p99 0.492 · 16370 op/s · total p50 0.469</sub> | 0.105<br><sub>context: p90 0.201 · p95 0.244 · p99 0.306 · 19740 op/s · total p50 0.391</sub> | -65.6% (-0.201) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.248<br><sub>context: p90 0.275 · p95 0.279 · p99 0.304 · 1994 op/s · total p50 0.488</sub> | 0.085<br><sub>context: p90 0.142 · p95 0.150 · p99 0.154 · 2423 op/s · total p50 0.406</sub> | -65.7% (-0.163) | 150% AND 2 ms | 🟢 |
| 8 | 0.318<br><sub>context: p90 0.412 · p95 0.457 · p99 0.545 · 15697 op/s · total p50 0.470</sub> | 0.140<br><sub>context: p90 0.240 · p95 0.277 · p99 0.367 · 18917 op/s · total p50 0.399</sub> | -55.9% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.227<br><sub>context: p90 0.252 · p95 0.256 · p99 0.283 · 1830 op/s · total p50 0.543</sub> | 0.054<br><sub>context: p90 0.083 · p95 0.086 · p99 0.096 · 2443 op/s · total p50 0.404</sub> | -76.3% (-0.173) | 150% AND 2 ms | 🟢 |
| 8 | 0.271<br><sub>context: p90 0.350 · p95 0.378 · p99 0.439 · 17481 op/s · total p50 0.430</sub> | 0.074<br><sub>context: p90 0.144 · p95 0.171 · p99 0.231 · 22767 op/s · total p50 0.328</sub> | -72.7% (-0.197) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.151<br><sub>context: p90 1.213 · p95 1.230 · p99 1.248 · 616 op/s · total p50 1.616</sub> | 0.603<br><sub>context: p90 0.644 · p95 0.657 · p99 0.675 · 853 op/s · total p50 1.167</sub> | -47.6% (-0.548) | 150% AND 2 ms | 🟢 |
| 8 | 1.480<br><sub>context: p90 1.843 · p95 1.974 · p99 2.316 · 4023 op/s · total p50 1.934</sub> | 0.706<br><sub>context: p90 0.810 · p95 0.834 · p99 0.913 · 6351 op/s · total p50 1.239</sub> | -52.3% (-0.774) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.161<br><sub>context: p90 1.247 · p95 1.267 · p99 1.320 · 686 op/s · total p50 1.448</sub> | 0.727<br><sub>context: p90 0.771 · p95 0.789 · p99 0.820 · 841 op/s · total p50 1.186</sub> | -37.4% (-0.434) | 150% AND 2 ms | 🟢 |
| 8 | 1.511<br><sub>context: p90 1.863 · p95 1.984 · p99 2.303 · 4531 op/s · total p50 1.720</sub> | 0.829<br><sub>context: p90 0.942 · p95 0.970 · p99 1.030 · 6905 op/s · total p50 1.135</sub> | -45.2% (-0.683) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.286 · p95 0.292 · p99 0.307 · 1732 op/s · total p50 0.570</sub> | 0.088<br><sub>context: p90 0.117 · p95 0.123 · p99 0.136 · 1861 op/s · total p50 0.539</sub> | -66.4% (-0.174) | 150% AND 2 ms | 🟢 |
| 8 | 0.292<br><sub>context: p90 0.368 · p95 0.398 · p99 0.456 · 15044 op/s · total p50 0.505</sub> | 0.098<br><sub>context: p90 0.169 · p95 0.201 · p99 0.257 · 16580 op/s · total p50 0.443</sub> | -66.4% (-0.194) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.380<br><sub>context: p90 0.448 · p95 0.476 · p99 0.506 · 1457 op/s · total p50 0.665</sub> | 0.202<br><sub>context: p90 0.281 · p95 0.322 · p99 0.358 · 1523 op/s · total p50 0.641</sub> | -46.8% (-0.178) | 150% AND 2 ms | 🟢 |
| 8 | 0.483<br><sub>context: p90 0.648 · p95 0.710 · p99 0.811 · 10557 op/s · total p50 0.719</sub> | 0.215<br><sub>context: p90 0.324 · p95 0.364 · p99 0.422 · 14515 op/s · total p50 0.530</sub> | -55.5% (-0.268) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.215<br><sub>context: p90 0.244 · p95 0.249 · p99 0.264 · 1904 op/s · total p50 0.524</sub> | 0.047<br><sub>context: p90 0.055 · p95 0.059 · p99 0.063 · 3203 op/s · total p50 0.300</sub> | -77.9% (-0.167) | 150% AND 2 ms | 🟢 |
| 8 | 0.252<br><sub>context: p90 0.324 · p95 0.350 · p99 0.404 · 17014 op/s · total p50 0.449</sub> | 0.054<br><sub>context: p90 0.133 · p95 0.168 · p99 0.218 · 22695 op/s · total p50 0.336</sub> | -78.5% (-0.198) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.178 · p95 0.188 · p99 0.203 · 2833 op/s · total p50 0.337</sub> | 0.050<br><sub>context: p90 0.080 · p95 0.087 · p99 0.094 · 3135 op/s · total p50 0.313</sub> | -67.0% (-0.103) | 150% AND 2 ms | 🟢 |
| 8 | 0.165<br><sub>context: p90 0.215 · p95 0.233 · p99 0.276 · 24442 op/s · total p50 0.312</sub> | 0.078<br><sub>context: p90 0.182 · p95 0.218 · p99 0.298 · 23197 op/s · total p50 0.322</sub> | -52.4% (-0.086) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.146<br><sub>context: p90 0.177 · p95 0.183 · p99 0.194 · 3023 op/s · total p50 0.322</sub> | 0.051<br><sub>context: p90 0.080 · p95 0.084 · p99 0.089 · 2607 op/s · total p50 0.370</sub> | -65.4% (-0.096) | 150% AND 2 ms | 🟢 |
| 8 | 0.163<br><sub>context: p90 0.211 · p95 0.231 · p99 0.281 · 24733 op/s · total p50 0.312</sub> | 0.078<br><sub>context: p90 0.181 · p95 0.222 · p99 0.292 · 22304 op/s · total p50 0.334</sub> | -52.1% (-0.085) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.854<br><sub>context: p90 0.912 · p95 0.929 · p99 0.946 · 872 op/s · total p50 1.145</sub> | 0.218<br><sub>context: p90 0.238 · p95 0.241 · p99 0.256 · 1649 op/s · total p50 0.599</sub> | -74.4% (-0.635) | 150% AND 2 ms | 🟢 |
| 8 | 1.158<br><sub>context: p90 1.665 · p95 1.902 · p99 2.175 · 5535 op/s · total p50 1.352</sub> | 0.255<br><sub>context: p90 0.296 · p95 0.303 · p99 0.319 · 15662 op/s · total p50 0.492</sub> | -78.0% (-0.904) | 150% AND 2 ms | 🟢 |

</details>
