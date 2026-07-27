### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
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

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**pr vs main** — 🟢 no p50 regression beyond budget across 10 comparable cell(s)

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.817<br><sub>context: p90 4.895 · p95 4.940 · p99 4.998 · 182 op/s · total p50 5.464</sub> | 4.805<br><sub>context: p90 4.895 · p95 4.925 · p99 4.997 · 182 op/s · total p50 5.449</sub> | -0.3% (-0.012) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.071<br><sub>context: p90 0.087 · p95 0.096 · p99 0.110 · 1871 op/s · total p50 0.482</sub> | 0.064<br><sub>context: p90 0.084 · p95 0.091 · p99 0.098 · 2043 op/s · total p50 0.471</sub> | -10.0% (-0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.750<br><sub>context: p90 0.838 · p95 0.860 · p99 0.888 · 721 op/s · total p50 1.339</sub> | 0.757<br><sub>context: p90 0.832 · p95 0.840 · p99 0.880 · 691 op/s · total p50 1.405</sub> | +0.9% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.171<br><sub>context: p90 0.188 · p95 0.198 · p99 0.207 · 1367 op/s · total p50 0.715</sub> | 0.168<br><sub>context: p90 0.187 · p95 0.194 · p99 0.210 · 1506 op/s · total p50 0.629</sub> | -1.8% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.097<br><sub>context: p90 0.128 · p95 0.134 · p99 0.150 · 1625 op/s · total p50 0.572</sub> | 0.091<br><sub>context: p90 0.118 · p95 0.132 · p99 0.144 · 1794 op/s · total p50 0.524</sub> | -5.8% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.196<br><sub>context: p90 0.224 · p95 0.230 · p99 0.255 · 1457 op/s · total p50 0.667</sub> | 0.199<br><sub>context: p90 0.229 · p95 0.238 · p99 0.255 · 1443 op/s · total p50 0.647</sub> | +1.7% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 14.691<br><sub>context: p90 15.290 · p95 15.447 · p99 15.534 · 63 op/s · total p50 15.668</sub> | 14.654<br><sub>context: p90 15.152 · p95 15.385 · p99 15.508 · 63 op/s · total p50 15.645</sub> | -0.3% (-0.037) | 25% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.818<br><sub>context: p90 0.867 · p95 0.875 · p99 0.941 · 396 op/s · total p50 2.507</sub> | 0.814<br><sub>context: p90 0.863 · p95 0.882 · p99 0.931 · 397 op/s · total p50 2.476</sub> | -0.5% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.067<br><sub>context: p90 0.093 · p95 0.099 · p99 0.110 · 2022 op/s · total p50 0.451</sub> | 0.063<br><sub>context: p90 0.092 · p95 0.095 · p99 0.101 · 2036 op/s · total p50 0.455</sub> | -5.1% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.091<br><sub>context: p90 0.108 · p95 0.111 · p99 0.119 · 1260 op/s · total p50 0.756</sub> | 0.089<br><sub>context: p90 0.106 · p95 0.113 · p99 0.128 · 1413 op/s · total p50 0.665</sub> | -1.7% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>
