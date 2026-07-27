### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 | ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3 |
| workload_hash | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` | `sha256:bf459bd5494d130caeb98ff1e0d1d30ac272348b5f991dc405f9878096d885fe` |
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

**pr vs main** — 🟢 no p50 regression beyond budget across 100 comparable cell(s)

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 → ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>aggregate_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.750<br><sub>context: p90 0.825 · p95 0.853 · p99 0.885 · 1309 op/s</sub> | 0.759<br><sub>context: p90 0.824 · p95 0.860 · p99 0.901 · 1294 op/s</sub> | +1.2% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.894<br><sub>context: p90 1.040 · p95 1.074 · p99 1.144 · 8750 op/s</sub> | 0.889<br><sub>context: p90 1.022 · p95 1.056 · p99 1.155 · 8888 op/s</sub> | -0.6% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_distinct</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.809<br><sub>context: p90 0.877 · p95 0.908 · p99 0.963 · 1213 op/s</sub> | 0.817<br><sub>context: p90 0.882 · p95 0.907 · p99 0.973 · 1208 op/s</sub> | +1.0% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.986<br><sub>context: p90 1.159 · p95 1.190 · p99 1.258 · 7967 op/s</sub> | 0.983<br><sub>context: p90 1.161 · p95 1.193 · p99 1.266 · 8003 op/s</sub> | -0.4% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_filtered</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.312<br><sub>context: p90 1.368 · p95 1.385 · p99 1.419 · 756 op/s</sub> | 1.309<br><sub>context: p90 1.388 · p95 1.406 · p99 1.462 · 756 op/s</sub> | -0.2% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.793<br><sub>context: p90 2.327 · p95 2.520 · p99 2.881 · 4277 op/s</sub> | 1.810<br><sub>context: p90 2.267 · p95 2.491 · p99 2.805 · 4289 op/s</sub> | +0.9% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_age_min_max_avg</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.415<br><sub>context: p90 1.503 · p95 1.520 · p99 1.572 · 704 op/s</sub> | 1.410<br><sub>context: p90 1.508 · p95 1.530 · p99 1.595 · 702 op/s</sub> | -0.3% (-0.005) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.635<br><sub>context: p90 1.953 · p95 2.007 · p99 2.101 · 4732 op/s</sub> | 1.608<br><sub>context: p90 1.942 · p95 1.983 · p99 2.053 · 4791 op/s</sub> | -1.7% (-0.027) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_count_users</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.148<br><sub>context: p90 0.175 · p95 0.182 · p99 0.190 · 6596 op/s</sub> | 0.129<br><sub>context: p90 0.160 · p95 0.167 · p99 0.192 · 7197 op/s</sub> | -12.4% (-0.018) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.220<br><sub>context: p90 0.284 · p95 0.299 · p99 0.335 · 34688 op/s</sub> | 0.219<br><sub>context: p90 0.284 · p95 0.305 · p99 0.347 · 35030 op/s</sub> | -0.3% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.214<br><sub>context: p90 0.249 · p95 0.259 · p99 0.284 · 4480 op/s</sub> | 0.216<br><sub>context: p90 0.262 · p95 0.282 · p99 0.295 · 4418 op/s</sub> | +1.2% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.328<br><sub>context: p90 0.418 · p95 0.449 · p99 0.497 · 23445 op/s</sub> | 0.335<br><sub>context: p90 0.419 · p95 0.451 · p99 0.505 · 22904 op/s</sub> | +2.2% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_1_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.243<br><sub>context: p90 0.306 · p95 0.337 · p99 0.391 · 3921 op/s</sub> | 0.258<br><sub>context: p90 0.337 · p95 0.354 · p99 0.397 · 3566 op/s</sub> | +6.1% (+0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.354<br><sub>context: p90 0.454 · p95 0.482 · p99 0.542 · 21481 op/s</sub> | 0.349<br><sub>context: p90 0.446 · p95 0.476 · p99 0.536 · 21459 op/s</sub> | -1.3% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.356<br><sub>context: p90 0.406 · p95 0.412 · p99 0.449 · 2768 op/s</sub> | 0.347<br><sub>context: p90 0.419 · p95 0.445 · p99 0.467 · 2824 op/s</sub> | -2.4% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.451<br><sub>context: p90 0.546 · p95 0.581 · p99 0.642 · 17114 op/s</sub> | 0.456<br><sub>context: p90 0.557 · p95 0.585 · p99 0.649 · 17076 op/s</sub> | +1.1% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.389<br><sub>context: p90 0.458 · p95 0.474 · p99 0.510 · 2538 op/s</sub> | 0.379<br><sub>context: p90 0.463 · p95 0.486 · p99 0.525 · 2557 op/s</sub> | -2.6% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.479<br><sub>context: p90 0.585 · p95 0.620 · p99 0.703 · 16193 op/s</sub> | 0.479<br><sub>context: p90 0.584 · p95 0.618 · p99 0.674 · 16216 op/s</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.482<br><sub>context: p90 0.597 · p95 0.623 · p99 0.722 · 2008 op/s</sub> | 0.522<br><sub>context: p90 0.622 · p95 0.669 · p99 0.759 · 1900 op/s</sub> | +8.2% (+0.040) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.640<br><sub>context: p90 0.820 · p95 0.884 · p99 1.043 · 11997 op/s</sub> | 0.637<br><sub>context: p90 0.797 · p95 0.853 · p99 0.966 · 12141 op/s</sub> | -0.5% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_3_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.539<br><sub>context: p90 0.673 · p95 0.704 · p99 0.774 · 1798 op/s</sub> | 0.576<br><sub>context: p90 0.719 · p95 0.769 · p99 0.864 · 1708 op/s</sub> | +6.8% (+0.037) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.670<br><sub>context: p90 0.848 · p95 0.901 · p99 1.032 · 11537 op/s</sub> | 0.670<br><sub>context: p90 0.848 · p95 0.915 · p99 1.012 · 11480 op/s</sub> | +0.1% (+0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.999<br><sub>context: p90 1.327 · p95 1.475 · p99 1.577 · 990 op/s</sub> | 1.064<br><sub>context: p90 1.441 · p95 1.538 · p99 1.768 · 932 op/s</sub> | +6.4% (+0.064) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.304<br><sub>context: p90 1.841 · p95 2.013 · p99 2.299 · 5880 op/s</sub> | 1.303<br><sub>context: p90 1.798 · p95 1.964 · p99 2.306 · 5932 op/s</sub> | -0.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>aggregate_expansion_4_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.059<br><sub>context: p90 1.418 · p95 1.612 · p99 1.789 · 919 op/s</sub> | 1.045<br><sub>context: p90 1.389 · p95 1.494 · p99 1.731 · 926 op/s</sub> | -1.3% (-0.014) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.319<br><sub>context: p90 1.863 · p95 2.033 · p99 2.334 · 5724 op/s</sub> | 1.327<br><sub>context: p90 1.826 · p95 1.989 · p99 2.275 · 5799 op/s</sub> | +0.6% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>all_shortest_paths_len</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.587<br><sub>context: p90 0.770 · p95 0.804 · p99 0.948 · 1659 op/s</sub> | 0.573<br><sub>context: p90 0.747 · p95 0.791 · p99 0.859 · 1699 op/s</sub> | -2.5% (-0.015) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.783<br><sub>context: p90 1.035 · p95 1.102 · p99 1.246 · 10036 op/s</sub> | 0.782<br><sub>context: p90 1.054 · p95 1.134 · p99 1.305 · 9876 op/s</sub> | -0.2% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>call_subquery</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.299<br><sub>context: p90 0.363 · p95 0.382 · p99 0.430 · 3222 op/s</sub> | 0.342<br><sub>context: p90 0.421 · p95 0.439 · p99 0.550 · 2813 op/s</sub> | +14.5% (+0.043) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.407<br><sub>context: p90 0.508 · p95 0.538 · p99 0.579 · 19129 op/s</sub> | 0.403<br><sub>context: p90 0.505 · p95 0.537 · p99 0.593 · 19018 op/s</sub> | -1.0% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_friend_edges_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.152<br><sub>context: p90 0.184 · p95 0.199 · p99 0.243 · 6114 op/s</sub> | 0.155<br><sub>context: p90 0.196 · p95 0.211 · p99 0.230 · 5676 op/s</sub> | +2.1% (+0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.222<br><sub>context: p90 0.290 · p95 0.310 · p99 0.360 · 34463 op/s</sub> | 0.224<br><sub>context: p90 0.298 · p95 0.318 · p99 0.363 · 34302 op/s</sub> | +0.9% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>count_users_plain</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.119<br><sub>context: p90 0.140 · p95 0.144 · p99 0.161 · 8156 op/s</sub> | 0.116<br><sub>context: p90 0.138 · p95 0.144 · p99 0.165 · 8331 op/s</sub> | -2.3% (-0.003) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.197<br><sub>context: p90 0.270 · p95 0.295 · p99 0.369 · 38090 op/s</sub> | 0.203<br><sub>context: p90 0.282 · p95 0.302 · p99 0.338 · 37127 op/s</sub> | +3.2% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>entity_path_introspection</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.357<br><sub>context: p90 0.406 · p95 0.431 · p99 0.468 · 2796 op/s</sub> | 0.361<br><sub>context: p90 0.422 · p95 0.443 · p99 0.465 · 2721 op/s</sub> | +1.1% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.441<br><sub>context: p90 0.545 · p95 0.576 · p99 0.630 · 17430 op/s</sub> | 0.442<br><sub>context: p90 0.548 · p95 0.581 · p99 0.647 · 17280 op/s</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_5_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.570<br><sub>context: p90 2.313 · p95 2.415 · p99 2.649 · 623 op/s</sub> | 1.414<br><sub>context: p90 2.069 · p95 2.314 · p99 2.627 · 684 op/s</sub> | -9.9% (-0.155) | 10% AND 0.5 ms | 🟢 |
| 8 | 2.278<br><sub>context: p90 3.572 · p95 3.959 · p99 4.630 · 3375 op/s</sub> | 2.270<br><sub>context: p90 3.517 · p95 3.950 · p99 4.492 · 3375 op/s</sub> | -0.3% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>exact_6_hop_traverse_count</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.932<br><sub>context: p90 7.523 · p95 8.248 · p99 8.710 · 196 op/s</sub> | 4.903<br><sub>context: p90 7.356 · p95 8.238 · p99 8.868 · 196 op/s</sub> | -0.6% (-0.029) | 10% AND 0.5 ms | 🟢 |
| 8 | 8.702<br><sub>context: p90 13.267 · p95 14.489 · p99 16.345 · 890 op/s</sub> | 8.452<br><sub>context: p90 13.028 · p95 14.329 · p99 15.939 · 912 op/s</sub> | -2.9% (-0.250) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.170<br><sub>context: p90 0.196 · p95 0.202 · p99 0.216 · 5693 op/s</sub> | 0.177<br><sub>context: p90 0.205 · p95 0.220 · p99 0.252 · 5235 op/s</sub> | +4.6% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.299<br><sub>context: p90 0.402 · p95 0.434 · p99 0.517 · 25341 op/s</sub> | 0.296<br><sub>context: p90 0.409 · p95 0.442 · p99 0.508 · 25438 op/s</sub> | -0.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>fulltext_query_relationships_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.189<br><sub>context: p90 0.213 · p95 0.229 · p99 0.238 · 5082 op/s</sub> | 0.178<br><sub>context: p90 0.206 · p95 0.211 · p99 0.226 · 5400 op/s</sub> | -5.7% (-0.011) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.296<br><sub>context: p90 0.388 · p95 0.419 · p99 0.478 · 25870 op/s</sub> | 0.298<br><sub>context: p90 0.414 · p95 0.452 · p99 0.533 · 25128 op/s</sub> | +0.4% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_range_scan</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.252<br><sub>context: p90 0.306 · p95 0.317 · p99 0.359 · 3760 op/s</sub> | 0.236<br><sub>context: p90 0.267 · p95 0.276 · p99 0.296 · 4129 op/s</sub> | -6.5% (-0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.362<br><sub>context: p90 0.465 · p95 0.513 · p99 0.590 · 20880 op/s</sub> | 0.357<br><sub>context: p90 0.446 · p95 0.472 · p99 0.539 · 21557 op/s</sub> | -1.5% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>id_seek</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.135<br><sub>context: p90 0.165 · p95 0.172 · p99 0.180 · 6994 op/s</sub> | 0.161<br><sub>context: p90 0.184 · p95 0.191 · p99 0.217 · 5916 op/s</sub> | +19.2% (+0.026) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.223<br><sub>context: p90 0.292 · p95 0.311 · p99 0.348 · 34679 op/s</sub> | 0.225<br><sub>context: p90 0.292 · p95 0.311 · p99 0.355 · 34252 op/s</sub> | +0.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_in_list_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.338<br><sub>context: p90 0.387 · p95 0.401 · p99 0.421 · 2934 op/s</sub> | 0.340<br><sub>context: p90 0.395 · p95 0.433 · p99 0.489 · 2875 op/s</sub> | +0.6% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.387<br><sub>context: p90 0.470 · p95 0.499 · p99 0.552 · 20011 op/s</sub> | 0.397<br><sub>context: p90 0.492 · p95 0.523 · p99 0.576 · 19336 op/s</sub> | +2.7% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>indexed_or_predicate</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.234<br><sub>context: p90 0.271 · p95 0.289 · p99 0.329 · 4108 op/s</sub> | 0.246<br><sub>context: p90 0.301 · p95 0.324 · p99 0.358 · 3887 op/s</sub> | +5.1% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.327<br><sub>context: p90 0.413 · p95 0.439 · p99 0.500 · 23443 op/s</sub> | 0.324<br><sub>context: p90 0.417 · p95 0.450 · p99 0.505 · 23484 op/s</sub> | -0.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.346<br><sub>context: p90 0.397 · p95 0.419 · p99 0.455 · 2855 op/s</sub> | 0.355<br><sub>context: p90 0.400 · p95 0.415 · p99 0.461 · 2794 op/s</sub> | +2.6% (+0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.436<br><sub>context: p90 0.538 · p95 0.568 · p99 0.639 · 17595 op/s</sub> | 0.446<br><sub>context: p90 0.551 · p95 0.584 · p99 0.647 · 17381 op/s</sub> | +2.2% (+0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.490<br><sub>context: p90 0.595 · p95 0.626 · p99 0.689 · 2014 op/s</sub> | 0.475<br><sub>context: p90 0.593 · p95 0.626 · p99 0.708 · 2046 op/s</sub> | -3.2% (-0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.620<br><sub>context: p90 0.798 · p95 0.843 · p99 0.935 · 12453 op/s</sub> | 0.620<br><sub>context: p90 0.803 · p95 0.865 · p99 0.971 · 12373 op/s</sub> | -0.1% (-0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_data_and_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.485<br><sub>context: p90 0.591 · p95 0.618 · p99 0.684 · 2010 op/s</sub> | 0.521<br><sub>context: p90 0.623 · p95 0.647 · p99 0.735 · 1883 op/s</sub> | +7.4% (+0.036) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.634<br><sub>context: p90 0.791 · p95 0.839 · p99 0.959 · 12184 op/s</sub> | 0.628<br><sub>context: p90 0.786 · p95 0.839 · p99 0.938 · 12313 op/s</sub> | -1.0% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>neighbours_2_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.361<br><sub>context: p90 0.410 · p95 0.417 · p99 0.443 · 2752 op/s</sub> | 0.362<br><sub>context: p90 0.443 · p95 0.469 · p99 0.504 · 2668 op/s</sub> | +0.3% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.463<br><sub>context: p90 0.568 · p95 0.604 · p99 0.660 · 16536 op/s</sub> | 0.472<br><sub>context: p90 0.578 · p95 0.612 · p99 0.665 · 16503 op/s</sub> | +1.9% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>optional_friend</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.300<br><sub>context: p90 0.359 · p95 0.384 · p99 0.470 · 3244 op/s</sub> | 0.288<br><sub>context: p90 0.356 · p95 0.375 · p99 0.405 · 3347 op/s</sub> | -3.9% (-0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.389<br><sub>context: p90 0.483 · p95 0.509 · p99 0.562 · 19784 op/s</sub> | 0.393<br><sub>context: p90 0.493 · p95 0.523 · p99 0.579 · 19494 op/s</sub> | +0.8% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>order_by_age</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.252<br><sub>context: p90 15.419 · p95 15.521 · p99 15.700 · 66 op/s</sub> | 15.167<br><sub>context: p90 15.329 · p95 15.369 · p99 15.493 · 66 op/s</sub> | -0.6% (-0.086) | 10% AND 0.5 ms | 🟢 |
| 8 | 18.231<br><sub>context: p90 24.498 · p95 27.285 · p99 31.201 · 393 op/s</sub> | 18.840<br><sub>context: p90 25.917 · p95 29.593 · p99 32.751 · 381 op/s</sub> | +3.3% (+0.609) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_cycle</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.381<br><sub>context: p90 0.472 · p95 0.497 · p99 0.537 · 2526 op/s</sub> | 0.372<br><sub>context: p90 0.414 · p95 0.437 · p99 0.470 · 2653 op/s</sub> | -2.5% (-0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.449<br><sub>context: p90 0.545 · p95 0.575 · p99 0.637 · 17271 op/s</sub> | 0.458<br><sub>context: p90 0.557 · p95 0.589 · p99 0.655 · 17076 op/s</sub> | +2.0% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_long</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.276<br><sub>context: p90 1.730 · p95 1.850 · p99 2.028 · 765 op/s</sub> | 1.292<br><sub>context: p90 1.732 · p95 1.872 · p99 2.101 · 771 op/s</sub> | +1.2% (+0.016) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.597<br><sub>context: p90 2.248 · p95 2.482 · p99 2.968 · 4841 op/s</sub> | 1.595<br><sub>context: p90 2.293 · p95 2.493 · p99 3.055 · 4817 op/s</sub> | -0.1% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>pattern_short</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.382<br><sub>context: p90 0.485 · p95 0.520 · p99 0.559 · 2510 op/s</sub> | 0.404<br><sub>context: p90 0.496 · p95 0.529 · p99 0.618 · 2415 op/s</sub> | +5.7% (+0.022) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.490<br><sub>context: p90 0.606 · p95 0.638 · p99 0.729 · 15935 op/s</sub> | 0.477<br><sub>context: p90 0.597 · p95 0.634 · p99 0.691 · 16057 op/s</sub> | -2.6% (-0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.366<br><sub>context: p90 0.455 · p95 0.481 · p99 0.538 · 2665 op/s</sub> | 0.369<br><sub>context: p90 0.460 · p95 0.485 · p99 0.595 · 2625 op/s</sub> | +0.8% (+0.003) | 12% AND 0.5 ms | 🟢 |
| 8 | 0.433<br><sub>context: p90 0.541 · p95 0.576 · p99 0.651 · 17816 op/s</sub> | 0.440<br><sub>context: p90 0.557 · p95 0.599 · p99 0.683 · 17534 op/s</sub> | +1.8% (+0.008) | 12% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>shortest_path_with_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.367<br><sub>context: p90 0.434 · p95 0.454 · p99 0.482 · 2701 op/s</sub> | 0.369<br><sub>context: p90 0.455 · p95 0.490 · p99 0.558 · 2617 op/s</sub> | +0.4% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.447<br><sub>context: p90 0.557 · p95 0.596 · p99 0.680 · 17256 op/s</sub> | 0.458<br><sub>context: p90 0.583 · p95 0.618 · p99 0.715 · 16725 op/s</sub> | +2.5% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_read</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.178<br><sub>context: p90 0.214 · p95 0.222 · p99 0.251 · 5440 op/s</sub> | 0.187<br><sub>context: p90 0.236 · p95 0.277 · p99 0.290 · 5022 op/s</sub> | +5.5% (+0.010) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.300<br><sub>context: p90 0.410 · p95 0.445 · p99 0.527 · 25159 op/s</sub> | 0.309<br><sub>context: p90 0.429 · p95 0.467 · p99 0.549 · 24331 op/s</sub> | +3.1% (+0.009) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>temporal_spatial_roundtrip</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.168<br><sub>context: p90 0.190 · p95 0.196 · p99 0.216 · 5686 op/s</sub> | 0.151<br><sub>context: p90 0.179 · p95 0.184 · p99 0.212 · 6211 op/s</sub> | -10.1% (-0.017) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.243<br><sub>context: p90 0.304 · p95 0.323 · p99 0.368 · 31838 op/s</sub> | 0.247<br><sub>context: p90 0.317 · p95 0.338 · p99 0.382 · 31285 op/s</sub> | +1.9% (+0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_all_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.217<br><sub>context: p90 0.251 · p95 0.269 · p99 0.308 · 4410 op/s</sub> | 0.224<br><sub>context: p90 0.276 · p95 0.302 · p99 0.337 · 4214 op/s</sub> | +3.5% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.363<br><sub>context: p90 0.465 · p95 0.501 · p99 0.575 · 21079 op/s</sub> | 0.372<br><sub>context: p90 0.485 · p95 0.529 · p99 0.587 · 20519 op/s</sub> | +2.3% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>union_distinct_ids</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.253<br><sub>context: p90 0.324 · p95 0.341 · p99 0.384 · 3728 op/s</sub> | 0.265<br><sub>context: p90 0.342 · p95 0.371 · p99 0.405 · 3559 op/s</sub> | +4.8% (+0.012) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.369<br><sub>context: p90 0.483 · p95 0.520 · p99 0.587 · 20544 op/s</sub> | 0.382<br><sub>context: p90 0.498 · p95 0.534 · p99 0.618 · 20099 op/s</sub> | +3.5% (+0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>unwind_rows</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.216<br><sub>context: p90 0.257 · p95 0.267 · p99 0.319 · 4581 op/s</sub> | 0.207<br><sub>context: p90 0.311 · p95 0.360 · p99 0.396 · 4382 op/s</sub> | -3.9% (-0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.311<br><sub>context: p90 0.403 · p95 0.436 · p99 0.513 · 24386 op/s</sub> | 0.318<br><sub>context: p90 0.417 · p95 0.458 · p99 0.511 · 23940 op/s</sub> | +2.3% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.935<br><sub>context: p90 0.989 · p95 0.999 · p99 1.026 · 1062 op/s</sub> | 0.936<br><sub>context: p90 1.027 · p95 1.048 · p99 1.127 · 1052 op/s</sub> | +0.2% (+0.001) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.185<br><sub>context: p90 1.345 · p95 1.402 · p99 1.507 · 6648 op/s</sub> | 1.168<br><sub>context: p90 1.326 · p95 1.380 · p99 1.483 · 6738 op/s</sub> | -1.4% (-0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>value_join_cnt</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.939<br><sub>context: p90 0.977 · p95 0.990 · p99 1.018 · 1061 op/s</sub> | 0.947<br><sub>context: p90 1.004 · p95 1.029 · p99 1.060 · 1049 op/s</sub> | +0.9% (+0.008) | 10% AND 0.5 ms | 🟢 |
| 8 | 1.094<br><sub>context: p90 1.219 · p95 1.261 · p99 1.377 · 7258 op/s</sub> | 1.100<br><sub>context: p90 1.254 · p95 1.304 · p99 1.410 · 7159 op/s</sub> | +0.5% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_friends</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.252<br><sub>context: p90 0.310 · p95 0.342 · p99 0.363 · 3835 op/s</sub> | 0.256<br><sub>context: p90 0.290 · p95 0.307 · p99 0.343 · 3895 op/s</sub> | +1.7% (+0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.381<br><sub>context: p90 0.484 · p95 0.515 · p99 0.585 · 20044 op/s</sub> | 0.392<br><sub>context: p90 0.494 · p95 0.525 · p99 0.598 · 19729 op/s</sub> | +3.0% (+0.011) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>var_len_with_edge_where_filter</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.386<br><sub>context: p90 0.479 · p95 0.506 · p99 0.562 · 2515 op/s</sub> | 0.378<br><sub>context: p90 0.460 · p95 0.484 · p99 0.521 · 2580 op/s</sub> | -2.3% (-0.009) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.486<br><sub>context: p90 0.612 · p95 0.647 · p99 0.722 · 15953 op/s</sub> | 0.475<br><sub>context: p90 0.591 · p95 0.625 · p99 0.705 · 16250 op/s</sub> | -2.4% (-0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vector_query_nodes_smoke</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.212 · p95 0.216 · p99 0.224 · 5323 op/s</sub> | 0.182<br><sub>context: p90 0.223 · p95 0.228 · p99 0.242 · 5237 op/s</sub> | +1.0% (+0.002) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.320<br><sub>context: p90 0.418 · p95 0.454 · p99 0.513 · 23358 op/s</sub> | 0.324<br><sub>context: p90 0.427 · p95 0.460 · p99 0.519 · 23394 op/s</sub> | +1.0% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.169<br><sub>context: p90 0.204 · p95 0.213 · p99 0.252 · 5718 op/s</sub> | 0.203<br><sub>context: p90 0.288 · p95 0.319 · p99 0.358 · 4530 op/s</sub> | +19.7% (+0.033) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.303<br><sub>context: p90 0.410 · p95 0.455 · p99 0.529 · 24873 op/s</sub> | 0.298<br><sub>context: p90 0.388 · p95 0.424 · p99 0.481 · 25258 op/s</sub> | -1.6% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_label_property_index</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.218 · p95 0.232 · p99 0.287 · 5369 op/s</sub> | 0.177<br><sub>context: p90 0.219 · p95 0.241 · p99 0.272 · 5408 op/s</sub> | -2.0% (-0.004) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.302<br><sub>context: p90 0.408 · p95 0.440 · p99 0.506 · 24868 op/s</sub> | 0.304<br><sub>context: p90 0.402 · p95 0.438 · p99 0.523 · 24896 op/s</sub> | +0.7% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>vertex_on_property</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.388<br><sub>context: p90 0.462 · p95 0.484 · p99 0.512 · 2464 op/s</sub> | 0.422<br><sub>context: p90 0.483 · p95 0.503 · p99 0.553 · 2283 op/s</sub> | +8.7% (+0.034) | 10% AND 0.5 ms | 🟢 |
| 8 | 0.463<br><sub>context: p90 0.536 · p95 0.562 · p99 0.624 · 16856 op/s</sub> | 0.469<br><sub>context: p90 0.544 · p95 0.568 · p99 0.623 · 16726 op/s</sub> | +1.4% (+0.006) | 10% AND 0.5 ms | 🟢 |

</details>
