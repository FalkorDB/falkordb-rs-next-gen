### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 → ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 4.225<br><sub>context: p90 4.437 · p95 4.451 · p99 4.541 · 208 op/s · total p50 4.744</sub> | 4.206<br><sub>context: p90 4.369 · p95 4.433 · p99 4.504 · 203 op/s · total p50 4.905</sub> | -0.4% (-0.018) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.075<br><sub>context: p90 0.093 · p95 0.102 · p99 0.117 · 1994 op/s · total p50 0.474</sub> | 0.074<br><sub>context: p90 0.094 · p95 0.102 · p99 0.106 · 1720 op/s · total p50 0.546</sub> | -0.0% (-0.000) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.728<br><sub>context: p90 0.806 · p95 0.829 · p99 0.875 · 730 op/s · total p50 1.320</sub> | 0.724<br><sub>context: p90 0.802 · p95 0.847 · p99 0.865 · 722 op/s · total p50 1.360</sub> | -0.5% (-0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.182<br><sub>context: p90 0.209 · p95 0.217 · p99 0.224 · 1579 op/s · total p50 0.602</sub> | 0.185<br><sub>context: p90 0.210 · p95 0.217 · p99 0.226 · 1403 op/s · total p50 0.679</sub> | +1.7% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.102<br><sub>context: p90 0.124 · p95 0.134 · p99 0.146 · 1733 op/s · total p50 0.552</sub> | 0.100<br><sub>context: p90 0.135 · p95 0.138 · p99 0.147 · 1772 op/s · total p50 0.537</sub> | -2.1% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.174<br><sub>context: p90 0.207 · p95 0.212 · p99 0.225 · 1606 op/s · total p50 0.594</sub> | 0.190<br><sub>context: p90 0.221 · p95 0.228 · p99 0.248 · 1416 op/s · total p50 0.677</sub> | +9.6% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 12.572<br><sub>context: p90 12.760 · p95 12.833 · p99 12.891 · 74 op/s · total p50 13.430</sub> | 12.653<br><sub>context: p90 13.111 · p95 13.337 · p99 13.397 · 73 op/s · total p50 13.617</sub> | +0.6% (+0.081) | 25% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.730<br><sub>context: p90 0.802 · p95 0.839 · p99 0.898 · 449 op/s · total p50 2.225</sub> | 0.789<br><sub>context: p90 0.849 · p95 0.859 · p99 0.896 · 442 op/s · total p50 2.243</sub> | +8.1% (+0.059) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.067<br><sub>context: p90 0.093 · p95 0.098 · p99 0.110 · 2132 op/s · total p50 0.445</sub> | 0.068<br><sub>context: p90 0.096 · p95 0.101 · p99 0.107 · 2023 op/s · total p50 0.473</sub> | +1.6% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.104<br><sub>context: p90 0.122 · p95 0.126 · p99 0.129 · 1462 op/s · total p50 0.650</sub> | 0.112<br><sub>context: p90 0.129 · p95 0.133 · p99 0.142 · 1289 op/s · total p50 0.756</sub> | +7.7% (+0.008) | 10% AND 0.5 ms | 🟢 |

</details>
