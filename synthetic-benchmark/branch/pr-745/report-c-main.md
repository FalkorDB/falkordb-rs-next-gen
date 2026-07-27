### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
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

**main vs c-engine** — 🔴 1 of 98 comparable cell(s) over budget

_⚠ 1 op(s) with differing results (perf N/A): temporal_spatial_roundtrip_

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.287<br><sub>context: p90 1.348 · p95 1.369 · p99 1.408 · 618 op/s · total p50 1.615</sub> | 0.612<br><sub>context: p90 0.650 · p95 0.661 · p99 0.670 · 981 op/s · total p50 1.012</sub> | -52.5% (-0.675) | 150% AND 2 ms | 🟢 |
| 8 | 1.778<br><sub>context: p90 2.342 · p95 2.565 · p99 2.988 · 3892 op/s · total p50 1.989</sub> | 0.731<br><sub>context: p90 0.864 · p95 0.885 · p99 0.955 · 7900 op/s · total p50 1.000</sub> | -58.9% (-1.048) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.989<br><sub>context: p90 2.057 · p95 2.091 · p99 2.152 · 441 op/s · total p50 2.268</sub> | 0.648<br><sub>context: p90 0.687 · p95 0.696 · p99 0.712 · 961 op/s · total p50 1.038</sub> | -67.4% (-1.341) | 150% AND 2 ms | 🟢 |
| 8 | 2.583<br><sub>context: p90 3.345 · p95 3.496 · p99 3.959 · 2738 op/s · total p50 2.819</sub> | 0.811<br><sub>context: p90 0.995 · p95 1.019 · p99 1.085 · 7384 op/s · total p50 1.067</sub> | -68.6% (-1.772) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.922<br><sub>context: p90 2.003 · p95 2.032 · p99 2.063 · 439 op/s · total p50 2.262</sub> | 1.140<br><sub>context: p90 1.195 · p95 1.204 · p99 1.224 · 650 op/s · total p50 1.525</sub> | -40.7% (-0.782) | 150% AND 2 ms | 🟢 |
| 8 | 2.638<br><sub>context: p90 3.401 · p95 3.595 · p99 3.930 · 2714 op/s · total p50 2.882</sub> | 1.601<br><sub>context: p90 2.082 · p95 2.244 · p99 2.617 · 4080 op/s · total p50 1.895</sub> | -39.3% (-1.036) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.565<br><sub>context: p90 2.663 · p95 2.685 · p99 2.749 · 336 op/s · total p50 2.969</sub> | 1.235<br><sub>context: p90 1.292 · p95 1.313 · p99 1.352 · 580 op/s · total p50 1.713</sub> | -51.9% (-1.330) | 150% AND 2 ms | 🟢 |
| 8 | 3.572<br><sub>context: p90 4.525 · p95 4.792 · p99 5.176 · 2032 op/s · total p50 3.809</sub> | 1.461<br><sub>context: p90 1.775 · p95 1.808 · p99 1.883 · 4386 op/s · total p50 1.772</sub> | -59.1% (-2.111) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.103<br><sub>context: p90 0.128 · p95 0.132 · p99 0.145 · 3976 op/s · total p50 0.242</sub> | 0.038<br><sub>context: p90 0.048 · p95 0.051 · p99 0.058 · 4467 op/s · total p50 0.221</sub> | -62.9% (-0.065) | 150% AND 2 ms | 🟢 |
| 8 | 0.126<br><sub>context: p90 0.177 · p95 0.197 · p99 0.243 · 28979 op/s · total p50 0.256</sub> | 0.026<br><sub>context: p90 0.037 · p95 0.042 · p99 0.051 · 33425 op/s · total p50 0.231</sub> | -79.4% (-0.100) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.266<br><sub>context: p90 0.286 · p95 0.295 · p99 0.305 · 1760 op/s · total p50 0.556</sub> | 0.072<br><sub>context: p90 0.097 · p95 0.104 · p99 0.110 · 2107 op/s · total p50 0.459</sub> | -73.1% (-0.194) | 150% AND 2 ms | 🟢 |
| 8 | 0.324<br><sub>context: p90 0.430 · p95 0.480 · p99 0.574 · 14408 op/s · total p50 0.522</sub> | 0.064<br><sub>context: p90 0.097 · p95 0.103 · p99 0.122 · 20216 op/s · total p50 0.362</sub> | -80.2% (-0.260) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.310 · p95 0.316 · p99 0.334 · 1716 op/s · total p50 0.582</sub> | 0.073<br><sub>context: p90 0.101 · p95 0.105 · p99 0.124 · 1859 op/s · total p50 0.517</sub> | -74.5% (-0.212) | 150% AND 2 ms | 🟢 |
| 8 | 0.338<br><sub>context: p90 0.422 · p95 0.454 · p99 0.527 · 14519 op/s · total p50 0.515</sub> | 0.064<br><sub>context: p90 0.098 · p95 0.105 · p99 0.116 · 21420 op/s · total p50 0.354</sub> | -81.0% (-0.274) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.303<br><sub>context: p90 0.327 · p95 0.336 · p99 0.343 · 1620 op/s · total p50 0.608</sub> | 0.140<br><sub>context: p90 0.171 · p95 0.179 · p99 0.197 · 1798 op/s · total p50 0.553</sub> | -53.6% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.347<br><sub>context: p90 0.427 · p95 0.448 · p99 0.507 · 14119 op/s · total p50 0.547</sub> | 0.142<br><sub>context: p90 0.180 · p95 0.194 · p99 0.220 · 14553 op/s · total p50 0.519</sub> | -59.1% (-0.205) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.308<br><sub>context: p90 0.350 · p95 0.362 · p99 0.388 · 1645 op/s · total p50 0.608</sub> | 0.148<br><sub>context: p90 0.180 · p95 0.189 · p99 0.198 · 1482 op/s · total p50 0.671</sub> | -52.1% (-0.161) | 150% AND 2 ms | 🟢 |
| 8 | 0.387<br><sub>context: p90 0.469 · p95 0.500 · p99 0.566 · 13103 op/s · total p50 0.594</sub> | 0.153<br><sub>context: p90 0.192 · p95 0.204 · p99 0.259 · 12165 op/s · total p50 0.613</sub> | -60.4% (-0.234) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.369<br><sub>context: p90 0.421 · p95 0.437 · p99 0.461 · 1257 op/s · total p50 0.781</sub> | 0.186<br><sub>context: p90 0.221 · p95 0.235 · p99 0.255 · 1333 op/s · total p50 0.724</sub> | -49.6% (-0.183) | 150% AND 2 ms | 🟢 |
| 8 | 0.434<br><sub>context: p90 0.535 · p95 0.565 · p99 0.664 · 9932 op/s · total p50 0.779</sub> | 0.185<br><sub>context: p90 0.234 · p95 0.250 · p99 0.281 · 11512 op/s · total p50 0.667</sub> | -57.3% (-0.249) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.391<br><sub>context: p90 0.452 · p95 0.474 · p99 0.507 · 1256 op/s · total p50 0.785</sub> | 0.190<br><sub>context: p90 0.231 · p95 0.246 · p99 0.278 · 1243 op/s · total p50 0.789</sub> | -51.3% (-0.201) | 150% AND 2 ms | 🟢 |
| 8 | 0.481<br><sub>context: p90 0.595 · p95 0.628 · p99 0.700 · 9419 op/s · total p50 0.827</sub> | 0.190<br><sub>context: p90 0.245 · p95 0.262 · p99 0.290 · 10916 op/s · total p50 0.711</sub> | -60.5% (-0.291) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.599<br><sub>context: p90 0.737 · p95 0.796 · p99 0.879 · 699 op/s · total p50 1.391</sub> | 0.322<br><sub>context: p90 0.382 · p95 0.397 · p99 0.417 · 741 op/s · total p50 1.341</sub> | -46.2% (-0.277) | 150% AND 2 ms | 🟢 |
| 8 | 0.713<br><sub>context: p90 0.946 · p95 1.021 · p99 1.167 · 4722 op/s · total p50 1.620</sub> | 0.336<br><sub>context: p90 0.442 · p95 0.470 · p99 0.530 · 5344 op/s · total p50 1.429</sub> | -52.9% (-0.377) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.674<br><sub>context: p90 0.850 · p95 0.918 · p99 1.024 · 652 op/s · total p50 1.504</sub> | 0.330<br><sub>context: p90 0.405 · p95 0.422 · p99 0.473 · 742 op/s · total p50 1.318</sub> | -51.0% (-0.344) | 150% AND 2 ms | 🟢 |
| 8 | 0.802<br><sub>context: p90 1.076 · p95 1.168 · p99 1.349 · 4435 op/s · total p50 1.702</sub> | 0.339<br><sub>context: p90 0.449 · p95 0.479 · p99 0.543 · 5522 op/s · total p50 1.384</sub> | -57.7% (-0.463) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.517<br><sub>context: p90 0.591 · p95 0.612 · p99 0.649 · 1304 op/s · total p50 0.764</sub> | 0.366<br><sub>context: p90 0.537 · p95 0.570 · p99 0.637 · 1344 op/s · total p50 0.754</sub> | -29.3% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.630<br><sub>context: p90 0.766 · p95 0.808 · p99 0.883 · 9654 op/s · total p50 0.811</sub> | 0.477<br><sub>context: p90 0.719 · p95 0.785 · p99 0.917 · 9806 op/s · total p50 0.795</sub> | -24.4% (-0.153) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.323<br><sub>context: p90 0.354 · p95 0.363 · p99 0.384 · 1796 op/s · total p50 0.554</sub> | 0.114<br><sub>context: p90 0.142 · p95 0.144 · p99 0.158 · 1950 op/s · total p50 0.505</sub> | -64.6% (-0.208) | 150% AND 2 ms | 🟢 |
| 8 | 0.412<br><sub>context: p90 0.503 · p95 0.539 · p99 0.609 · 13280 op/s · total p50 0.581</sub> | 0.108<br><sub>context: p90 0.142 · p95 0.152 · p99 0.168 · 18575 op/s · total p50 0.420</sub> | -73.8% (-0.304) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.181<br><sub>context: p90 0.203 · p95 0.209 · p99 0.219 · 2578 op/s · total p50 0.383</sub> | 0.004<br><sub>context: p90 0.006 · p95 0.006 · p99 0.008 · 4930 op/s · total p50 0.195</sub> | -97.9% (-0.177) | 150% AND 2 ms | 🟢 |
| 8 | 0.213<br><sub>context: p90 0.295 · p95 0.324 · p99 0.398 · 19618 op/s · total p50 0.363</sub> | 0.003<br><sub>context: p90 0.004 · p95 0.005 · p99 0.006 · 32061 op/s · total p50 0.236</sub> | -98.6% (-0.210) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.128<br><sub>context: p90 0.149 · p95 0.152 · p99 0.167 · 3413 op/s · total p50 0.283</sub> | 0.003<br><sub>context: p90 0.006 · p95 0.006 · p99 0.008 · 6438 op/s · total p50 0.145</sub> | -97.6% (-0.125) | 150% AND 2 ms | 🟢 |
| 8 | 0.155<br><sub>context: p90 0.217 · p95 0.241 · p99 0.295 · 24721 op/s · total p50 0.294</sub> | 0.003<br><sub>context: p90 0.004 · p95 0.004 · p99 0.006 · 35438 op/s · total p50 0.214</sub> | -98.2% (-0.152) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.399<br><sub>context: p90 0.434 · p95 0.444 · p99 0.454 · 1338 op/s · total p50 0.747</sub> | 0.086<br><sub>context: p90 0.117 · p95 0.121 · p99 0.133 · 1971 op/s · total p50 0.492</sub> | -78.4% (-0.313) | 150% AND 2 ms | 🟢 |
| 8 | 0.495<br><sub>context: p90 0.603 · p95 0.637 · p99 0.719 · 11067 op/s · total p50 0.700</sub> | 0.085<br><sub>context: p90 0.121 · p95 0.130 · p99 0.153 · 16773 op/s · total p50 0.456</sub> | -82.9% (-0.410) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.729<br><sub>context: p90 1.041 · p95 1.154 · p99 1.238 · 895 op/s · total p50 1.085</sub> | 1.183<br><sub>context: p90 1.904 · p95 2.118 · p99 2.397 · 600 op/s · total p50 1.625</sub> | +62.2% (+0.454) | 150% AND 2 ms | 🟢 |
| 8 | 0.949<br><sub>context: p90 1.452 · p95 1.647 · p99 1.969 · 6277 op/s · total p50 1.183</sub> | 2.025<br><sub>context: p90 3.322 · p95 3.801 · p99 4.555 · 3210 op/s · total p50 2.361</sub> | +113.4% (+1.076) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🔴 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.389<br><sub>context: p90 3.850 · p95 4.392 · p99 4.609 · 345 op/s · total p50 2.790</sub> | 4.805<br><sub>context: p90 7.359 · p95 8.273 · p99 9.065 · 181 op/s · total p50 5.325</sub> | +101.1% (+2.416) | 150% AND 2 ms | 🟢 |
| 8 | 3.172<br><sub>context: p90 5.606 · p95 6.422 · p99 8.170 · 2125 op/s · total p50 3.452</sub> | 8.385<br><sub>context: p90 13.215 · p95 14.481 · p99 16.630 · 869 op/s · total p50 8.792</sub> | +164.3% (+5.213) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.179<br><sub>context: p90 0.203 · p95 0.209 · p99 0.218 · 2331 op/s · total p50 0.428</sub> | 0.026<br><sub>context: p90 0.042 · p95 0.045 · p99 0.048 · 3880 op/s · total p50 0.244</sub> | -85.3% (-0.153) | 150% AND 2 ms | 🟢 |
| 8 | 0.208<br><sub>context: p90 0.284 · p95 0.317 · p99 0.372 · 18466 op/s · total p50 0.400</sub> | 0.022<br><sub>context: p90 0.030 · p95 0.033 · p99 0.042 · 28639 op/s · total p50 0.266</sub> | -89.4% (-0.186) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.207 · p95 0.210 · p99 0.225 · 2286 op/s · total p50 0.431</sub> | 0.035<br><sub>context: p90 0.041 · p95 0.042 · p99 0.048 · 3398 op/s · total p50 0.284</sub> | -81.3% (-0.150) | 150% AND 2 ms | 🟢 |
| 8 | 0.210<br><sub>context: p90 0.279 · p95 0.306 · p99 0.365 · 19115 op/s · total p50 0.393</sub> | 0.022<br><sub>context: p90 0.030 · p95 0.033 · p99 0.041 · 28143 op/s · total p50 0.272</sub> | -89.7% (-0.189) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.194<br><sub>context: p90 0.216 · p95 0.221 · p99 0.230 · 1674 op/s · total p50 0.605</sub> | 0.010<br><sub>context: p90 0.014 · p95 0.016 · p99 0.017 · 3900 op/s · total p50 0.241</sub> | -94.9% (-0.185) | 150% AND 2 ms | 🟢 |
| 8 | 0.219<br><sub>context: p90 0.287 · p95 0.307 · p99 0.346 · 14557 op/s · total p50 0.527</sub> | 0.013<br><sub>context: p90 0.017 · p95 0.019 · p99 0.022 · 20217 op/s · total p50 0.373</sub> | -94.2% (-0.206) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.159<br><sub>context: p90 0.175 · p95 0.180 · p99 0.192 · 2579 op/s · total p50 0.379</sub> | 0.007<br><sub>context: p90 0.013 · p95 0.014 · p99 0.019 · 5608 op/s · total p50 0.163</sub> | -95.6% (-0.152) | 150% AND 2 ms | 🟢 |
| 8 | 0.174<br><sub>context: p90 0.237 · p95 0.261 · p99 0.310 · 24080 op/s · total p50 0.313</sub> | 0.007<br><sub>context: p90 0.010 · p95 0.011 · p99 0.014 · 33906 op/s · total p50 0.228</sub> | -96.0% (-0.168) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.285<br><sub>context: p90 0.306 · p95 0.315 · p99 0.330 · 1620 op/s · total p50 0.608</sub> | 0.155<br><sub>context: p90 0.193 · p95 0.203 · p99 0.224 · 1980 op/s · total p50 0.501</sub> | -45.8% (-0.130) | 150% AND 2 ms | 🟢 |
| 8 | 0.329<br><sub>context: p90 0.413 · p95 0.438 · p99 0.502 · 14910 op/s · total p50 0.505</sub> | 0.152<br><sub>context: p90 0.202 · p95 0.212 · p99 0.241 · 19318 op/s · total p50 0.403</sub> | -53.7% (-0.177) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.221<br><sub>context: p90 0.244 · p95 0.253 · p99 0.274 · 1913 op/s · total p50 0.502</sub> | 0.081<br><sub>context: p90 0.112 · p95 0.119 · p99 0.146 · 3116 op/s · total p50 0.320</sub> | -63.3% (-0.140) | 150% AND 2 ms | 🟢 |
| 8 | 0.247<br><sub>context: p90 0.308 · p95 0.328 · p99 0.377 · 19293 op/s · total p50 0.396</sub> | 0.080<br><sub>context: p90 0.114 · p95 0.122 · p99 0.144 · 23631 op/s · total p50 0.319</sub> | -67.8% (-0.168) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.294<br><sub>context: p90 0.322 · p95 0.336 · p99 0.348 · 1640 op/s · total p50 0.616</sub> | 0.132<br><sub>context: p90 0.160 · p95 0.167 · p99 0.191 · 1775 op/s · total p50 0.559</sub> | -55.0% (-0.162) | 150% AND 2 ms | 🟢 |
| 8 | 0.344<br><sub>context: p90 0.441 · p95 0.481 · p99 0.558 · 13170 op/s · total p50 0.567</sub> | 0.127<br><sub>context: p90 0.164 · p95 0.176 · p99 0.197 · 16664 op/s · total p50 0.457</sub> | -63.0% (-0.217) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.286<br><sub>context: p90 0.313 · p95 0.320 · p99 0.337 · 1468 op/s · total p50 0.675</sub> | 0.133<br><sub>context: p90 0.165 · p95 0.172 · p99 0.180 · 1504 op/s · total p50 0.652</sub> | -53.6% (-0.153) | 150% AND 2 ms | 🟢 |
| 8 | 0.333<br><sub>context: p90 0.408 · p95 0.429 · p99 0.487 · 11224 op/s · total p50 0.696</sub> | 0.131<br><sub>context: p90 0.169 · p95 0.180 · p99 0.199 · 12379 op/s · total p50 0.621</sub> | -60.7% (-0.202) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.351 · p95 0.357 · p99 0.376 · 1355 op/s · total p50 0.737</sub> | 0.135<br><sub>context: p90 0.170 · p95 0.176 · p99 0.182 · 1365 op/s · total p50 0.723</sub> | -57.4% (-0.183) | 150% AND 2 ms | 🟢 |
| 8 | 0.364<br><sub>context: p90 0.446 · p95 0.473 · p99 0.544 · 10763 op/s · total p50 0.714</sub> | 0.136<br><sub>context: p90 0.173 · p95 0.184 · p99 0.206 · 12106 op/s · total p50 0.641</sub> | -62.6% (-0.228) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.343 · p95 0.354 · p99 0.365 · 1657 op/s · total p50 0.596</sub> | 0.134<br><sub>context: p90 0.162 · p95 0.170 · p99 0.186 · 1751 op/s · total p50 0.562</sub> | -58.0% (-0.184) | 150% AND 2 ms | 🟢 |
| 8 | 0.370<br><sub>context: p90 0.459 · p95 0.486 · p99 0.558 · 13385 op/s · total p50 0.577</sub> | 0.135<br><sub>context: p90 0.173 · p95 0.184 · p99 0.207 · 15563 op/s · total p50 0.497</sub> | -63.6% (-0.236) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.329<br><sub>context: p90 0.358 · p95 0.370 · p99 0.396 · 1587 op/s · total p50 0.624</sub> | 0.110<br><sub>context: p90 0.143 · p95 0.148 · p99 0.154 · 1968 op/s · total p50 0.506</sub> | -66.7% (-0.219) | 150% AND 2 ms | 🟢 |
| 8 | 0.396<br><sub>context: p90 0.489 · p95 0.524 · p99 0.606 · 13360 op/s · total p50 0.579</sub> | 0.103<br><sub>context: p90 0.138 · p95 0.147 · p99 0.171 · 19620 op/s · total p50 0.393</sub> | -74.0% (-0.293) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 6.362<br><sub>context: p90 6.657 · p95 6.739 · p99 6.869 · 52 op/s · total p50 19.184</sub> | 2.969<br><sub>context: p90 3.101 · p95 3.140 · p99 3.190 · 63 op/s · total p50 15.744</sub> | -53.3% (-3.392) | 150% AND 2 ms | 🟢 |
| 8 | 7.806<br><sub>context: p90 9.954 · p95 10.312 · p99 11.054 · 297 op/s · total p50 24.726</sub> | 3.201<br><sub>context: p90 4.234 · p95 4.363 · p99 4.617 · 373 op/s · total p50 19.478</sub> | -59.0% (-4.605) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.403<br><sub>context: p90 0.446 · p95 0.454 · p99 0.477 · 1489 op/s · total p50 0.667</sub> | 0.137<br><sub>context: p90 0.168 · p95 0.174 · p99 0.179 · 1900 op/s · total p50 0.515</sub> | -66.0% (-0.266) | 150% AND 2 ms | 🟢 |
| 8 | 0.512<br><sub>context: p90 0.622 · p95 0.659 · p99 0.734 · 11335 op/s · total p50 0.688</sub> | 0.138<br><sub>context: p90 0.174 · p95 0.186 · p99 0.211 · 15314 op/s · total p50 0.503</sub> | -73.0% (-0.374) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.595<br><sub>context: p90 0.694 · p95 0.729 · p99 0.768 · 574 op/s · total p50 1.711</sub> | 0.299<br><sub>context: p90 0.362 · p95 0.377 · p99 0.405 · 660 op/s · total p50 1.484</sub> | -49.7% (-0.295) | 150% AND 2 ms | 🟢 |
| 8 | 0.665<br><sub>context: p90 0.847 · p95 0.897 · p99 1.007 · 4034 op/s · total p50 1.934</sub> | 0.314<br><sub>context: p90 0.411 · p95 0.438 · p99 0.499 · 4515 op/s · total p50 1.714</sub> | -52.9% (-0.351) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.316<br><sub>context: p90 0.344 · p95 0.351 · p99 0.365 · 1551 op/s · total p50 0.643</sub> | 0.138<br><sub>context: p90 0.169 · p95 0.176 · p99 0.186 · 1668 op/s · total p50 0.596</sub> | -56.4% (-0.178) | 150% AND 2 ms | 🟢 |
| 8 | 0.359<br><sub>context: p90 0.438 · p95 0.460 · p99 0.520 · 13338 op/s · total p50 0.582</sub> | 0.129<br><sub>context: p90 0.168 · p95 0.179 · p99 0.199 · 16571 op/s · total p50 0.459</sub> | -64.0% (-0.230) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.073<br><sub>context: p90 2.671 · p95 2.744 · p99 2.892 · 423 op/s · total p50 2.421</sub> | 0.134<br><sub>context: p90 0.182 · p95 0.198 · p99 0.218 · 2061 op/s · total p50 0.479</sub> | -93.6% (-1.940) | 150% AND 2 ms | 🟢 |
| 8 | 2.376<br><sub>context: p90 3.156 · p95 3.345 · p99 3.565 · 3049 op/s · total p50 2.625</sub> | 0.136<br><sub>context: p90 0.192 · p95 0.208 · p99 0.242 · 18407 op/s · total p50 0.411</sub> | -94.3% (-2.241) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.134<br><sub>context: p90 2.732 · p95 2.833 · p99 2.888 · 416 op/s · total p50 2.483</sub> | 0.151<br><sub>context: p90 0.201 · p95 0.218 · p99 0.249 · 1658 op/s · total p50 0.603</sub> | -92.9% (-1.983) | 150% AND 2 ms | 🟢 |
| 8 | 2.411<br><sub>context: p90 3.253 · p95 3.409 · p99 3.704 · 3031 op/s · total p50 2.663</sub> | 0.143<br><sub>context: p90 0.207 · p95 0.227 · p99 0.266 · 16579 op/s · total p50 0.466</sub> | -94.1% (-2.268) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.163<br><sub>context: p90 0.181 · p95 0.189 · p99 0.201 · 2743 op/s · total p50 0.350</sub> | 0.048<br><sub>context: p90 0.077 · p95 0.080 · p99 0.091 · 2395 op/s · total p50 0.387</sub> | -70.8% (-0.115) | 150% AND 2 ms | 🟢 |
| 8 | 0.177<br><sub>context: p90 0.248 · p95 0.278 · p99 0.339 · 21336 op/s · total p50 0.347</sub> | 0.042<br><sub>context: p90 0.076 · p95 0.081 · p99 0.091 · 25783 op/s · total p50 0.295</sub> | -76.0% (-0.135) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>⚠ <code>temporal_spatial_roundtrip</code> — ⚠ results differ (advisory; perf verdict N/A)</summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.235<br><sub>context: p90 0.255 · p95 0.261 · p99 0.271 · 1864 op/s · total p50 0.529</sub> | 0.006<br><sub>context: p90 0.009 · p95 0.011 · p99 0.013 · 4441 op/s · total p50 0.213</sub> | -97.4% (-0.229) | 150% AND 2 ms | ⚠ N/A |
| 8 | 0.289<br><sub>context: p90 0.352 · p95 0.375 · p99 0.442 · 16937 op/s · total p50 0.452</sub> | 0.005<br><sub>context: p90 0.008 · p95 0.010 · p99 0.011 · 27561 op/s · total p50 0.267</sub> | -98.3% (-0.284) | 150% AND 2 ms | ⚠ N/A |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.240<br><sub>context: p90 0.265 · p95 0.274 · p99 0.293 · 2162 op/s · total p50 0.464</sub> | 0.059<br><sub>context: p90 0.084 · p95 0.092 · p99 0.102 · 2974 op/s · total p50 0.335</sub> | -75.4% (-0.181) | 150% AND 2 ms | 🟢 |
| 8 | 0.306<br><sub>context: p90 0.395 · p95 0.435 · p99 0.492 · 16370 op/s · total p50 0.469</sub> | 0.053<br><sub>context: p90 0.087 · p95 0.092 · p99 0.102 · 24170 op/s · total p50 0.322</sub> | -82.6% (-0.253) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.248<br><sub>context: p90 0.275 · p95 0.279 · p99 0.304 · 1994 op/s · total p50 0.488</sub> | 0.078<br><sub>context: p90 0.136 · p95 0.145 · p99 0.162 · 2573 op/s · total p50 0.373</sub> | -68.6% (-0.170) | 150% AND 2 ms | 🟢 |
| 8 | 0.318<br><sub>context: p90 0.412 · p95 0.457 · p99 0.545 · 15697 op/s · total p50 0.470</sub> | 0.078<br><sub>context: p90 0.148 · p95 0.155 · p99 0.165 · 23768 op/s · total p50 0.320</sub> | -75.6% (-0.240) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.227<br><sub>context: p90 0.252 · p95 0.256 · p99 0.283 · 1830 op/s · total p50 0.543</sub> | 0.049<br><sub>context: p90 0.079 · p95 0.082 · p99 0.092 · 2703 op/s · total p50 0.355</sub> | -78.5% (-0.178) | 150% AND 2 ms | 🟢 |
| 8 | 0.271<br><sub>context: p90 0.350 · p95 0.378 · p99 0.439 · 17481 op/s · total p50 0.430</sub> | 0.044<br><sub>context: p90 0.079 · p95 0.082 · p99 0.089 · 26806 op/s · total p50 0.290</sub> | -83.9% (-0.227) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.151<br><sub>context: p90 1.213 · p95 1.230 · p99 1.248 · 616 op/s · total p50 1.616</sub> | 0.624<br><sub>context: p90 0.665 · p95 0.676 · p99 0.696 · 779 op/s · total p50 1.291</sub> | -45.8% (-0.527) | 150% AND 2 ms | 🟢 |
| 8 | 1.480<br><sub>context: p90 1.843 · p95 1.974 · p99 2.316 · 4023 op/s · total p50 1.934</sub> | 0.714<br><sub>context: p90 0.836 · p95 0.869 · p99 1.004 · 5966 op/s · total p50 1.303</sub> | -51.8% (-0.766) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.161<br><sub>context: p90 1.247 · p95 1.267 · p99 1.320 · 686 op/s · total p50 1.448</sub> | 0.614<br><sub>context: p90 0.649 · p95 0.663 · p99 0.677 · 925 op/s · total p50 1.080</sub> | -47.1% (-0.547) | 150% AND 2 ms | 🟢 |
| 8 | 1.511<br><sub>context: p90 1.863 · p95 1.984 · p99 2.303 · 4531 op/s · total p50 1.720</sub> | 0.719<br><sub>context: p90 0.844 · p95 0.876 · p99 0.967 · 7445 op/s · total p50 1.052</sub> | -52.5% (-0.793) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.262<br><sub>context: p90 0.286 · p95 0.292 · p99 0.307 · 1732 op/s · total p50 0.570</sub> | 0.080<br><sub>context: p90 0.108 · p95 0.117 · p99 0.130 · 2221 op/s · total p50 0.450</sub> | -69.5% (-0.182) | 150% AND 2 ms | 🟢 |
| 8 | 0.292<br><sub>context: p90 0.368 · p95 0.398 · p99 0.456 · 15044 op/s · total p50 0.505</sub> | 0.073<br><sub>context: p90 0.105 · p95 0.114 · p99 0.130 · 20421 op/s · total p50 0.376</sub> | -74.8% (-0.218) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.380<br><sub>context: p90 0.448 · p95 0.476 · p99 0.506 · 1457 op/s · total p50 0.665</sub> | 0.177<br><sub>context: p90 0.246 · p95 0.278 · p99 0.298 · 1876 op/s · total p50 0.519</sub> | -53.4% (-0.203) | 150% AND 2 ms | 🟢 |
| 8 | 0.483<br><sub>context: p90 0.648 · p95 0.710 · p99 0.811 · 10557 op/s · total p50 0.719</sub> | 0.196<br><sub>context: p90 0.295 · p95 0.328 · p99 0.377 · 15868 op/s · total p50 0.494</sub> | -59.3% (-0.286) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.215<br><sub>context: p90 0.244 · p95 0.249 · p99 0.264 · 1904 op/s · total p50 0.524</sub> | 0.044<br><sub>context: p90 0.053 · p95 0.055 · p99 0.059 · 3109 op/s · total p50 0.320</sub> | -79.4% (-0.171) | 150% AND 2 ms | 🟢 |
| 8 | 0.252<br><sub>context: p90 0.324 · p95 0.350 · p99 0.404 · 17014 op/s · total p50 0.449</sub> | 0.030<br><sub>context: p90 0.044 · p95 0.049 · p99 0.058 · 26242 op/s · total p50 0.291</sub> | -88.0% (-0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.178 · p95 0.188 · p99 0.203 · 2833 op/s · total p50 0.337</sub> | 0.043<br><sub>context: p90 0.074 · p95 0.076 · p99 0.081 · 3586 op/s · total p50 0.270</sub> | -71.6% (-0.110) | 150% AND 2 ms | 🟢 |
| 8 | 0.165<br><sub>context: p90 0.215 · p95 0.233 · p99 0.276 · 24442 op/s · total p50 0.312</sub> | 0.041<br><sub>context: p90 0.074 · p95 0.079 · p99 0.086 · 27843 op/s · total p50 0.276</sub> | -75.4% (-0.124) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.146<br><sub>context: p90 0.177 · p95 0.183 · p99 0.194 · 3023 op/s · total p50 0.322</sub> | 0.044<br><sub>context: p90 0.074 · p95 0.079 · p99 0.085 · 3282 op/s · total p50 0.292</sub> | -69.7% (-0.102) | 150% AND 2 ms | 🟢 |
| 8 | 0.163<br><sub>context: p90 0.211 · p95 0.231 · p99 0.281 · 24733 op/s · total p50 0.312</sub> | 0.041<br><sub>context: p90 0.076 · p95 0.080 · p99 0.088 · 27835 op/s · total p50 0.274</sub> | -74.8% (-0.122) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.854<br><sub>context: p90 0.912 · p95 0.929 · p99 0.946 · 872 op/s · total p50 1.145</sub> | 0.219<br><sub>context: p90 0.235 · p95 0.240 · p99 0.244 · 1604 op/s · total p50 0.622</sub> | -74.4% (-0.635) | 150% AND 2 ms | 🟢 |
| 8 | 1.158<br><sub>context: p90 1.665 · p95 1.902 · p99 2.175 · 5535 op/s · total p50 1.352</sub> | 0.253<br><sub>context: p90 0.292 · p95 0.301 · p99 0.322 · 15298 op/s · total p50 0.494</sub> | -78.1% (-0.905) | 150% AND 2 ms | 🟢 |

</details>
