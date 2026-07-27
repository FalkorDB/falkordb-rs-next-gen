### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 | ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
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

**pr vs main** — 🟢 no p50 regression beyond budget across 10 comparable cell(s)

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 → ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.539<br><sub>context: p90 5.918 · p95 6.086 · p99 6.321 · 178 op/s</sub> | 5.359<br><sub>context: p90 5.521 · p95 5.629 · p99 5.828 · 184 op/s</sub> | -3.3% (-0.181) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.505<br><sub>context: p90 0.726 · p95 0.838 · p99 0.878 · 1896 op/s</sub> | 0.538<br><sub>context: p90 0.716 · p95 0.860 · p99 0.930 · 1762 op/s</sub> | +6.6% (+0.033) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.482<br><sub>context: p90 1.795 · p95 1.869 · p99 1.921 · 654 op/s</sub> | 1.464<br><sub>context: p90 1.750 · p95 1.853 · p99 1.923 · 658 op/s</sub> | -1.2% (-0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.668<br><sub>context: p90 0.940 · p95 0.976 · p99 0.997 · 1403 op/s</sub> | 0.646<br><sub>context: p90 0.902 · p95 0.937 · p99 1.024 · 1451 op/s</sub> | -3.4% (-0.022) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.543<br><sub>context: p90 0.780 · p95 0.946 · p99 1.002 · 1679 op/s</sub> | 0.550<br><sub>context: p90 0.769 · p95 0.846 · p99 0.905 · 1714 op/s</sub> | +1.2% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.653<br><sub>context: p90 0.958 · p95 1.060 · p99 1.187 · 1432 op/s</sub> | 0.645<br><sub>context: p90 0.920 · p95 1.011 · p99 1.105 · 1451 op/s</sub> | -1.2% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.341<br><sub>context: p90 15.623 · p95 15.713 · p99 15.910 · 65 op/s</sub> | 15.267<br><sub>context: p90 15.513 · p95 15.622 · p99 15.734 · 65 op/s</sub> | -0.5% (-0.074) | 25% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.579<br><sub>context: p90 2.729 · p95 2.886 · p99 3.028 · 376 op/s</sub> | 2.494<br><sub>context: p90 2.613 · p95 2.772 · p99 2.880 · 391 op/s</sub> | -3.3% (-0.085) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.514<br><sub>context: p90 0.684 · p95 0.779 · p99 0.840 · 1840 op/s</sub> | 0.485<br><sub>context: p90 0.562 · p95 0.610 · p99 0.712 · 1927 op/s</sub> | -5.6% (-0.029) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.789<br><sub>context: p90 0.922 · p95 1.044 · p99 1.127 · 1215 op/s</sub> | 0.786<br><sub>context: p90 0.983 · p95 1.131 · p99 1.199 · 1205 op/s</sub> | -0.5% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>
