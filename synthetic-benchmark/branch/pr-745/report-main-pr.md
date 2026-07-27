### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:215d05fcfb400f14ccd553f34f1b188d1ffcd9850421cc5c5baab38c49e0c0c5 | ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb |
| workload_hash | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` | `sha256:07fbcd8d1a0f2e2cbb23275ea2b87fd77601ff566d74e99cfe7f7880e8046c88` |
| samples / warmup | 200 / 50 | 200 / 50 |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 10% | 0.5 ms |
| `expand_hops_5` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `match_by_index` | 15% | 0.5 ms |
| `property_projection` | 15% | 0.5 ms |
| `return_const` | 15% | 0.5 ms |
| `shortest_path` | 12% (c16 18%, c32 25%) | 0.5 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs main** — 🟢 no p50 regression beyond budget across 100 comparable cell(s)

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:215d05fcfb400f14ccd553f34f1b188d1ffcd9850421cc5c5baab38c49e0c0c5 → ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.717<br><sub>context: p90 0.772 · p95 0.783 · p99 0.827 · 1367 op/s</sub> | 0.775<br><sub>context: p90 0.872 · p95 0.916 · p99 0.957 · 1272 op/s</sub> | +8.0% (+0.058) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.879<br><sub>context: p90 0.995 · p95 1.025 · p99 1.089 · 9055 op/s</sub> | 0.875<br><sub>context: p90 1.008 · p95 1.042 · p99 1.135 · 9024 op/s</sub> | -0.4% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.766<br><sub>context: p90 0.807 · p95 0.821 · p99 0.852 · 1288 op/s</sub> | 0.834<br><sub>context: p90 0.927 · p95 0.968 · p99 1.009 · 1177 op/s</sub> | +8.8% (+0.068) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.986<br><sub>context: p90 1.149 · p95 1.183 · p99 1.239 · 8060 op/s</sub> | 0.977<br><sub>context: p90 1.161 · p95 1.199 · p99 1.260 · 8065 op/s</sub> | -1.0% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.306<br><sub>context: p90 1.364 · p95 1.384 · p99 1.407 · 761 op/s</sub> | 1.329<br><sub>context: p90 1.406 · p95 1.442 · p99 1.505 · 743 op/s</sub> | +1.7% (+0.022) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.794<br><sub>context: p90 2.347 · p95 2.549 · p99 2.923 · 4267 op/s</sub> | 1.794<br><sub>context: p90 2.326 · p95 2.537 · p99 2.815 · 4305 op/s</sub> | -0.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.354<br><sub>context: p90 1.442 · p95 1.464 · p99 1.516 · 730 op/s</sub> | 1.366<br><sub>context: p90 1.476 · p95 1.510 · p99 1.595 · 719 op/s</sub> | +0.9% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.585<br><sub>context: p90 1.931 · p95 1.979 · p99 2.050 · 4840 op/s</sub> | 1.609<br><sub>context: p90 1.927 · p95 1.973 · p99 2.048 · 4803 op/s</sub> | +1.5% (+0.024) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.156<br><sub>context: p90 0.193 · p95 0.223 · p99 0.247 · 6038 op/s</sub> | 0.146<br><sub>context: p90 0.183 · p95 0.214 · p99 0.244 · 6358 op/s</sub> | -5.9% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.217<br><sub>context: p90 0.286 · p95 0.305 · p99 0.345 · 35343 op/s</sub> | 0.215<br><sub>context: p90 0.280 · p95 0.301 · p99 0.343 · 35816 op/s</sub> | -0.6% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.215<br><sub>context: p90 0.254 · p95 0.276 · p99 0.326 · 4513 op/s</sub> | 0.244<br><sub>context: p90 0.319 · p95 0.349 · p99 0.410 · 3853 op/s</sub> | +13.5% (+0.029) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.324<br><sub>context: p90 0.412 · p95 0.442 · p99 0.507 · 23576 op/s</sub> | 0.328<br><sub>context: p90 0.425 · p95 0.459 · p99 0.519 · 23293 op/s</sub> | +1.2% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.234<br><sub>context: p90 0.279 · p95 0.313 · p99 0.348 · 4085 op/s</sub> | 0.267<br><sub>context: p90 0.364 · p95 0.393 · p99 0.449 · 3572 op/s</sub> | +14.2% (+0.033) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.342<br><sub>context: p90 0.433 · p95 0.464 · p99 0.518 · 22305 op/s</sub> | 0.347<br><sub>context: p90 0.435 · p95 0.466 · p99 0.521 · 21962 op/s</sub> | +1.5% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.337<br><sub>context: p90 0.414 · p95 0.439 · p99 0.488 · 2855 op/s</sub> | 0.353<br><sub>context: p90 0.443 · p95 0.494 · p99 0.555 · 2705 op/s</sub> | +4.7% (+0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.439<br><sub>context: p90 0.540 · p95 0.567 · p99 0.629 · 17610 op/s</sub> | 0.441<br><sub>context: p90 0.540 · p95 0.578 · p99 0.650 · 17588 op/s</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.353<br><sub>context: p90 0.411 · p95 0.424 · p99 0.452 · 2786 op/s</sub> | 0.378<br><sub>context: p90 0.463 · p95 0.504 · p99 0.550 · 2534 op/s</sub> | +7.1% (+0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.466<br><sub>context: p90 0.568 · p95 0.598 · p99 0.659 · 16740 op/s</sub> | 0.475<br><sub>context: p90 0.577 · p95 0.604 · p99 0.670 · 16442 op/s</sub> | +2.1% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.465<br><sub>context: p90 0.554 · p95 0.580 · p99 0.646 · 2115 op/s</sub> | 0.494<br><sub>context: p90 0.609 · p95 0.648 · p99 0.687 · 1959 op/s</sub> | +6.4% (+0.030) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.618<br><sub>context: p90 0.793 · p95 0.844 · p99 0.924 · 12492 op/s</sub> | 0.628<br><sub>context: p90 0.801 · p95 0.854 · p99 0.971 · 12222 op/s</sub> | +1.6% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.473<br><sub>context: p90 0.579 · p95 0.617 · p99 0.657 · 2057 op/s</sub> | 0.566<br><sub>context: p90 0.729 · p95 0.756 · p99 0.849 · 1686 op/s</sub> | +19.7% (+0.093) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.652<br><sub>context: p90 0.822 · p95 0.869 · p99 0.964 · 11852 op/s</sub> | 0.655<br><sub>context: p90 0.832 · p95 0.893 · p99 0.993 · 11733 op/s</sub> | +0.5% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.019<br><sub>context: p90 1.335 · p95 1.515 · p99 1.694 · 969 op/s</sub> | 1.032<br><sub>context: p90 1.391 · p95 1.534 · p99 1.736 · 945 op/s</sub> | +1.2% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.272<br><sub>context: p90 1.771 · p95 1.944 · p99 2.243 · 6056 op/s</sub> | 1.279<br><sub>context: p90 1.805 · p95 1.958 · p99 2.286 · 6032 op/s</sub> | +0.6% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.981<br><sub>context: p90 1.352 · p95 1.443 · p99 1.650 · 986 op/s</sub> | 1.057<br><sub>context: p90 1.400 · p95 1.563 · p99 1.690 · 919 op/s</sub> | +7.7% (+0.075) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.307<br><sub>context: p90 1.802 · p95 1.982 · p99 2.302 · 5844 op/s</sub> | 1.300<br><sub>context: p90 1.784 · p95 1.950 · p99 2.225 · 5870 op/s</sub> | -0.5% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.573<br><sub>context: p90 0.727 · p95 0.769 · p99 0.836 · 1708 op/s</sub> | 0.576<br><sub>context: p90 0.741 · p95 0.795 · p99 0.871 · 1696 op/s</sub> | +0.6% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.771<br><sub>context: p90 1.040 · p95 1.103 · p99 1.222 · 10106 op/s</sub> | 0.776<br><sub>context: p90 1.031 · p95 1.098 · p99 1.235 · 10073 op/s</sub> | +0.6% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.289<br><sub>context: p90 0.337 · p95 0.348 · p99 0.374 · 3406 op/s</sub> | 0.324<br><sub>context: p90 0.408 · p95 0.429 · p99 0.517 · 2978 op/s</sub> | +12.1% (+0.035) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.396<br><sub>context: p90 0.492 · p95 0.521 · p99 0.571 · 19514 op/s</sub> | 0.397<br><sub>context: p90 0.493 · p95 0.525 · p99 0.586 · 19665 op/s</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.172 · p95 0.175 · p99 0.185 · 6337 op/s</sub> | 0.148<br><sub>context: p90 0.187 · p95 0.196 · p99 0.213 · 6032 op/s</sub> | -3.0% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.226<br><sub>context: p90 0.289 · p95 0.312 · p99 0.349 · 33881 op/s</sub> | 0.223<br><sub>context: p90 0.292 · p95 0.316 · p99 0.367 · 34494 op/s</sub> | -1.5% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.149 · p95 0.157 · p99 0.174 · 7577 op/s</sub> | 0.133<br><sub>context: p90 0.174 · p95 0.190 · p99 0.208 · 7052 op/s</sub> | +7.9% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.203<br><sub>context: p90 0.272 · p95 0.300 · p99 0.353 · 37791 op/s</sub> | 0.201<br><sub>context: p90 0.271 · p95 0.294 · p99 0.343 · 38221 op/s</sub> | -1.1% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.354<br><sub>context: p90 0.467 · p95 0.495 · p99 0.539 · 2669 op/s</sub> | 0.328<br><sub>context: p90 0.368 · p95 0.390 · p99 0.442 · 3026 op/s</sub> | -7.1% (-0.025) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.439<br><sub>context: p90 0.531 · p95 0.560 · p99 0.618 · 17681 op/s</sub> | 0.438<br><sub>context: p90 0.537 · p95 0.569 · p99 0.634 · 17664 op/s</sub> | -0.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.313<br><sub>context: p90 2.008 · p95 2.211 · p99 2.498 · 733 op/s</sub> | 1.385<br><sub>context: p90 2.091 · p95 2.304 · p99 2.542 · 693 op/s</sub> | +5.5% (+0.073) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.257<br><sub>context: p90 3.573 · p95 4.029 · p99 4.668 · 3347 op/s</sub> | 2.212<br><sub>context: p90 3.479 · p95 3.903 · p99 4.483 · 3442 op/s</sub> | -2.0% (-0.045) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.796<br><sub>context: p90 7.289 · p95 8.106 · p99 8.687 · 201 op/s</sub> | 4.863<br><sub>context: p90 7.455 · p95 8.038 · p99 8.727 · 197 op/s</sub> | +1.4% (+0.068) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.661<br><sub>context: p90 13.376 · p95 14.699 · p99 16.870 · 890 op/s</sub> | 8.363<br><sub>context: p90 12.756 · p95 14.264 · p99 16.264 · 919 op/s</sub> | -3.4% (-0.297) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.203 · p95 0.218 · p99 0.238 · 5524 op/s</sub> | 0.157<br><sub>context: p90 0.186 · p95 0.189 · p99 0.200 · 6082 op/s</sub> | -10.8% (-0.019) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.398 · p95 0.424 · p99 0.508 · 25101 op/s</sub> | 0.307<br><sub>context: p90 0.421 · p95 0.461 · p99 0.550 · 24626 op/s</sub> | +2.3% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.193 · p95 0.202 · p99 0.213 · 6034 op/s</sub> | 0.167<br><sub>context: p90 0.203 · p95 0.217 · p99 0.249 · 5577 op/s</sub> | +9.7% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.405 · p95 0.434 · p99 0.507 · 25224 op/s</sub> | 0.299<br><sub>context: p90 0.404 · p95 0.443 · p99 0.517 · 25437 op/s</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.222<br><sub>context: p90 0.261 · p95 0.267 · p99 0.279 · 4289 op/s</sub> | 0.232<br><sub>context: p90 0.263 · p95 0.271 · p99 0.302 · 4103 op/s</sub> | +4.8% (+0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.356<br><sub>context: p90 0.458 · p95 0.497 · p99 0.557 · 21179 op/s</sub> | 0.356<br><sub>context: p90 0.455 · p95 0.500 · p99 0.577 · 21201 op/s</sub> | -0.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.128<br><sub>context: p90 0.154 · p95 0.158 · p99 0.163 · 7294 op/s</sub> | 0.141<br><sub>context: p90 0.169 · p95 0.185 · p99 0.198 · 6352 op/s</sub> | +10.1% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.221<br><sub>context: p90 0.285 · p95 0.307 · p99 0.338 · 34789 op/s</sub> | 0.218<br><sub>context: p90 0.286 · p95 0.306 · p99 0.346 · 35282 op/s</sub> | -1.3% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.340<br><sub>context: p90 0.426 · p95 0.458 · p99 0.477 · 2834 op/s</sub> | 0.332<br><sub>context: p90 0.390 · p95 0.415 · p99 0.447 · 2974 op/s</sub> | -2.2% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.395<br><sub>context: p90 0.483 · p95 0.512 · p99 0.561 · 19642 op/s</sub> | 0.395<br><sub>context: p90 0.478 · p95 0.509 · p99 0.549 · 19480 op/s</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.230<br><sub>context: p90 0.319 · p95 0.349 · p99 0.371 · 4096 op/s</sub> | 0.231<br><sub>context: p90 0.283 · p95 0.309 · p99 0.348 · 4161 op/s</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.329<br><sub>context: p90 0.420 · p95 0.459 · p99 0.515 · 23405 op/s</sub> | 0.325<br><sub>context: p90 0.415 · p95 0.446 · p99 0.502 · 23248 op/s</sub> | -1.1% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.353<br><sub>context: p90 0.455 · p95 0.475 · p99 0.546 · 2739 op/s</sub> | 0.334<br><sub>context: p90 0.420 · p95 0.443 · p99 0.493 · 2914 op/s</sub> | -5.3% (-0.019) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.435<br><sub>context: p90 0.538 · p95 0.565 · p99 0.642 · 17532 op/s</sub> | 0.438<br><sub>context: p90 0.540 · p95 0.567 · p99 0.647 · 17406 op/s</sub> | +0.7% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.474<br><sub>context: p90 0.593 · p95 0.615 · p99 0.709 · 2085 op/s</sub> | 0.471<br><sub>context: p90 0.556 · p95 0.593 · p99 0.665 · 2107 op/s</sub> | -0.5% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.614<br><sub>context: p90 0.776 · p95 0.828 · p99 0.932 · 12569 op/s</sub> | 0.610<br><sub>context: p90 0.781 · p95 0.840 · p99 0.945 · 12593 op/s</sub> | -0.7% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.481<br><sub>context: p90 0.585 · p95 0.628 · p99 0.676 · 2044 op/s</sub> | 0.473<br><sub>context: p90 0.583 · p95 0.607 · p99 0.653 · 2082 op/s</sub> | -1.6% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.623<br><sub>context: p90 0.795 · p95 0.855 · p99 0.965 · 12222 op/s</sub> | 0.615<br><sub>context: p90 0.767 · p95 0.823 · p99 0.899 · 12526 op/s</sub> | -1.3% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.379<br><sub>context: p90 0.452 · p95 0.486 · p99 0.602 · 2575 op/s</sub> | 0.352<br><sub>context: p90 0.462 · p95 0.483 · p99 0.520 · 2685 op/s</sub> | -7.0% (-0.027) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.455<br><sub>context: p90 0.552 · p95 0.587 · p99 0.651 · 16911 op/s</sub> | 0.462<br><sub>context: p90 0.564 · p95 0.590 · p99 0.668 · 16802 op/s</sub> | +1.4% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.275<br><sub>context: p90 0.327 · p95 0.337 · p99 0.379 · 3566 op/s</sub> | 0.284<br><sub>context: p90 0.366 · p95 0.390 · p99 0.416 · 3377 op/s</sub> | +3.4% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.377<br><sub>context: p90 0.475 · p95 0.506 · p99 0.574 · 20434 op/s</sub> | 0.386<br><sub>context: p90 0.486 · p95 0.519 · p99 0.566 · 20016 op/s</sub> | +2.6% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.059<br><sub>context: p90 15.472 · p95 15.509 · p99 15.721 · 66 op/s</sub> | 15.117<br><sub>context: p90 15.448 · p95 15.523 · p99 15.639 · 66 op/s</sub> | +0.4% (+0.058) | 10% AND 0.5 ms | 🟢 |
| 8 | 18.044<br><sub>context: p90 23.723 · p95 26.222 · p99 29.648 · 394 op/s</sub> | 18.069<br><sub>context: p90 23.854 · p95 26.310 · p99 30.656 · 393 op/s</sub> | +0.1% (+0.024) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.366<br><sub>context: p90 0.462 · p95 0.488 · p99 0.505 · 2603 op/s</sub> | 0.365<br><sub>context: p90 0.422 · p95 0.439 · p99 0.478 · 2714 op/s</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.446<br><sub>context: p90 0.542 · p95 0.570 · p99 0.633 · 17443 op/s</sub> | 0.446<br><sub>context: p90 0.537 · p95 0.566 · p99 0.621 · 17360 op/s</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.252<br><sub>context: p90 1.762 · p95 1.882 · p99 2.042 · 781 op/s</sub> | 1.291<br><sub>context: p90 1.774 · p95 1.914 · p99 2.283 · 755 op/s</sub> | +3.1% (+0.039) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.568<br><sub>context: p90 2.214 · p95 2.397 · p99 2.913 · 4914 op/s</sub> | 1.576<br><sub>context: p90 2.219 · p95 2.400 · p99 2.788 · 4959 op/s</sub> | +0.5% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.375<br><sub>context: p90 0.455 · p95 0.479 · p99 0.527 · 2612 op/s</sub> | 0.368<br><sub>context: p90 0.447 · p95 0.461 · p99 0.539 · 2685 op/s</sub> | -1.7% (-0.006) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.464<br><sub>context: p90 0.570 · p95 0.605 · p99 0.661 · 16722 op/s</sub> | 0.465<br><sub>context: p90 0.578 · p95 0.610 · p99 0.699 · 16303 op/s</sub> | +0.4% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.342<br><sub>context: p90 0.412 · p95 0.430 · p99 0.452 · 2849 op/s</sub> | 0.344<br><sub>context: p90 0.435 · p95 0.466 · p99 0.542 · 2765 op/s</sub> | +0.6% (+0.002) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.423<br><sub>context: p90 0.536 · p95 0.561 · p99 0.652 · 18190 op/s</sub> | 0.424<br><sub>context: p90 0.531 · p95 0.569 · p99 0.660 · 18204 op/s</sub> | +0.3% (+0.001) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.345<br><sub>context: p90 0.438 · p95 0.466 · p99 0.515 · 2771 op/s</sub> | 0.364<br><sub>context: p90 0.452 · p95 0.484 · p99 0.549 · 2669 op/s</sub> | +5.4% (+0.019) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.444<br><sub>context: p90 0.560 · p95 0.596 · p99 0.669 · 17363 op/s</sub> | 0.438<br><sub>context: p90 0.555 · p95 0.602 · p99 0.670 · 17547 op/s</sub> | -1.2% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.176<br><sub>context: p90 0.218 · p95 0.242 · p99 0.318 · 5442 op/s</sub> | 0.194<br><sub>context: p90 0.246 · p95 0.272 · p99 0.333 · 4873 op/s</sub> | +10.5% (+0.018) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.297<br><sub>context: p90 0.391 · p95 0.428 · p99 0.495 · 25352 op/s</sub> | 0.303<br><sub>context: p90 0.401 · p95 0.442 · p99 0.510 · 24913 op/s</sub> | +1.8% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.148<br><sub>context: p90 0.183 · p95 0.188 · p99 0.194 · 6385 op/s</sub> | 0.165<br><sub>context: p90 0.193 · p95 0.200 · p99 0.219 · 5685 op/s</sub> | +11.6% (+0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.242<br><sub>context: p90 0.307 · p95 0.329 · p99 0.375 · 31518 op/s</sub> | 0.238<br><sub>context: p90 0.311 · p95 0.339 · p99 0.381 · 31848 op/s</sub> | -1.5% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.207<br><sub>context: p90 0.249 · p95 0.261 · p99 0.295 · 4564 op/s</sub> | 0.220<br><sub>context: p90 0.262 · p95 0.272 · p99 0.291 · 4415 op/s</sub> | +6.2% (+0.013) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.356<br><sub>context: p90 0.460 · p95 0.492 · p99 0.569 · 21361 op/s</sub> | 0.362<br><sub>context: p90 0.470 · p95 0.507 · p99 0.591 · 21005 op/s</sub> | +1.8% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.252<br><sub>context: p90 0.319 · p95 0.332 · p99 0.369 · 3777 op/s</sub> | 0.262<br><sub>context: p90 0.342 · p95 0.359 · p99 0.406 · 3651 op/s</sub> | +3.9% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.369<br><sub>context: p90 0.480 · p95 0.512 · p99 0.597 · 20876 op/s</sub> | 0.376<br><sub>context: p90 0.491 · p95 0.527 · p99 0.613 · 20453 op/s</sub> | +1.8% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.215 · p95 0.231 · p99 0.249 · 5288 op/s</sub> | 0.188<br><sub>context: p90 0.241 · p95 0.255 · p99 0.309 · 4992 op/s</sub> | +0.6% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.395 · p95 0.425 · p99 0.498 · 24761 op/s</sub> | 0.309<br><sub>context: p90 0.401 · p95 0.435 · p99 0.503 · 24485 op/s</sub> | +1.4% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.941<br><sub>context: p90 1.018 · p95 1.047 · p99 1.109 · 1053 op/s</sub> | 0.956<br><sub>context: p90 1.064 · p95 1.102 · p99 1.160 · 1030 op/s</sub> | +1.5% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.163<br><sub>context: p90 1.326 · p95 1.374 · p99 1.469 · 6752 op/s</sub> | 1.165<br><sub>context: p90 1.331 · p95 1.392 · p99 1.466 · 6765 op/s</sub> | +0.2% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.970<br><sub>context: p90 1.079 · p95 1.100 · p99 1.143 · 1015 op/s</sub> | 0.945<br><sub>context: p90 1.049 · p95 1.081 · p99 1.139 · 1039 op/s</sub> | -2.7% (-0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.088<br><sub>context: p90 1.213 · p95 1.250 · p99 1.329 · 7307 op/s</sub> | 1.080<br><sub>context: p90 1.205 · p95 1.242 · p99 1.368 · 7327 op/s</sub> | -0.7% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.235<br><sub>context: p90 0.275 · p95 0.295 · p99 0.347 · 4128 op/s</sub> | 0.262<br><sub>context: p90 0.308 · p95 0.323 · p99 0.359 · 3733 op/s</sub> | +11.5% (+0.027) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.373<br><sub>context: p90 0.465 · p95 0.496 · p99 0.560 · 20561 op/s</sub> | 0.381<br><sub>context: p90 0.478 · p95 0.510 · p99 0.600 · 20136 op/s</sub> | +2.3% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.378<br><sub>context: p90 0.451 · p95 0.472 · p99 0.542 · 2578 op/s</sub> | 0.386<br><sub>context: p90 0.477 · p95 0.527 · p99 0.587 · 2508 op/s</sub> | +2.1% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.478<br><sub>context: p90 0.598 · p95 0.630 · p99 0.691 · 16377 op/s</sub> | 0.477<br><sub>context: p90 0.598 · p95 0.636 · p99 0.716 · 16052 op/s</sub> | -0.1% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.206 · p95 0.216 · p99 0.227 · 5384 op/s</sub> | 0.176<br><sub>context: p90 0.207 · p95 0.218 · p99 0.224 · 5490 op/s</sub> | -1.8% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.315<br><sub>context: p90 0.415 · p95 0.448 · p99 0.521 · 24029 op/s</sub> | 0.314<br><sub>context: p90 0.409 · p95 0.440 · p99 0.508 · 23903 op/s</sub> | -0.4% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.191<br><sub>context: p90 0.224 · p95 0.252 · p99 0.282 · 5029 op/s</sub> | 0.189<br><sub>context: p90 0.243 · p95 0.269 · p99 0.315 · 4978 op/s</sub> | -1.4% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.297<br><sub>context: p90 0.404 · p95 0.445 · p99 0.522 · 25406 op/s</sub> | 0.298<br><sub>context: p90 0.398 · p95 0.438 · p99 0.511 · 25366 op/s</sub> | +0.5% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.174<br><sub>context: p90 0.221 · p95 0.243 · p99 0.276 · 5475 op/s</sub> | 0.158<br><sub>context: p90 0.199 · p95 0.209 · p99 0.228 · 6045 op/s</sub> | -9.1% (-0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.305<br><sub>context: p90 0.408 · p95 0.440 · p99 0.515 · 24772 op/s</sub> | 0.300<br><sub>context: p90 0.398 · p95 0.434 · p99 0.518 · 25287 op/s</sub> | -1.6% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.382<br><sub>context: p90 0.456 · p95 0.479 · p99 0.538 · 2523 op/s</sub> | 0.380<br><sub>context: p90 0.452 · p95 0.476 · p99 0.500 · 2496 op/s</sub> | -0.4% (-0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.460<br><sub>context: p90 0.538 · p95 0.562 · p99 0.605 · 16905 op/s</sub> | 0.459<br><sub>context: p90 0.533 · p95 0.557 · p99 0.595 · 16991 op/s</sub> | -0.3% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>
