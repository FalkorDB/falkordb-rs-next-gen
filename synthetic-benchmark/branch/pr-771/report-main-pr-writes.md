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
| 1 | 4.304<br><sub>context: p90 4.420 · p95 4.463 · p99 4.552 · 206 op/s · total p50 4.806</sub> | 4.297<br><sub>context: p90 4.422 · p95 4.458 · p99 4.534 · 206 op/s · total p50 4.808</sub> | -0.2% (-0.008) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.066<br><sub>context: p90 0.086 · p95 0.090 · p99 0.103 · 2110 op/s · total p50 0.423</sub> | 0.062<br><sub>context: p90 0.077 · p95 0.089 · p99 0.108 · 2304 op/s · total p50 0.387</sub> | -6.3% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.768<br><sub>context: p90 0.829 · p95 0.845 · p99 0.866 · 720 op/s · total p50 1.345</sub> | 0.735<br><sub>context: p90 0.791 · p95 0.801 · p99 0.818 · 765 op/s · total p50 1.253</sub> | -4.3% (-0.033) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.170<br><sub>context: p90 0.190 · p95 0.195 · p99 0.200 · 1651 op/s · total p50 0.547</sub> | 0.167<br><sub>context: p90 0.191 · p95 0.194 · p99 0.199 · 1692 op/s · total p50 0.541</sub> | -1.7% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.097<br><sub>context: p90 0.126 · p95 0.133 · p99 0.140 · 1706 op/s · total p50 0.537</sub> | 0.078<br><sub>context: p90 0.102 · p95 0.111 · p99 0.117 · 2177 op/s · total p50 0.413</sub> | -19.9% (-0.019) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.177<br><sub>context: p90 0.198 · p95 0.211 · p99 0.225 · 1713 op/s · total p50 0.527</sub> | 0.179<br><sub>context: p90 0.213 · p95 0.218 · p99 0.235 · 1677 op/s · total p50 0.551</sub> | +1.0% (+0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 12.615<br><sub>context: p90 12.743 · p95 12.783 · p99 12.834 · 73 op/s · total p50 13.533</sub> | 12.581<br><sub>context: p90 12.705 · p95 12.756 · p99 12.942 · 74 op/s · total p50 13.444</sub> | -0.3% (-0.034) | 25% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.762<br><sub>context: p90 0.846 · p95 0.871 · p99 0.912 · 458 op/s · total p50 2.157</sub> | 0.779<br><sub>context: p90 0.851 · p95 0.865 · p99 0.888 · 447 op/s · total p50 2.211</sub> | +2.3% (+0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.062<br><sub>context: p90 0.084 · p95 0.087 · p99 0.111 · 2290 op/s · total p50 0.386</sub> | 0.057<br><sub>context: p90 0.078 · p95 0.086 · p99 0.093 · 2365 op/s · total p50 0.376</sub> | -8.2% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.092<br><sub>context: p90 0.113 · p95 0.117 · p99 0.125 · 1581 op/s · total p50 0.587</sub> | 0.099<br><sub>context: p90 0.116 · p95 0.121 · p99 0.130 · 1546 op/s · total p50 0.608</sub> | +7.1% (+0.007) | 10% AND 0.5 ms | 🟢 |

</details>
