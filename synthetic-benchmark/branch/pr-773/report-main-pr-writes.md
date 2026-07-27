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
| 1 | 4.673<br><sub>context: p90 4.802 · p95 4.845 · p99 4.893 · 189 op/s · total p50 5.271</sub> | 4.557<br><sub>context: p90 4.732 · p95 4.765 · p99 4.858 · 197 op/s · total p50 5.027</sub> | -2.5% (-0.115) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.063<br><sub>context: p90 0.080 · p95 0.085 · p99 0.099 · 2257 op/s · total p50 0.397</sub> | 0.061<br><sub>context: p90 0.082 · p95 0.087 · p99 0.095 · 2348 op/s · total p50 0.376</sub> | -2.6% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.717<br><sub>context: p90 0.788 · p95 0.800 · p99 0.828 · 778 op/s · total p50 1.236</sub> | 0.715<br><sub>context: p90 0.772 · p95 0.780 · p99 0.815 · 761 op/s · total p50 1.235</sub> | -0.3% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.158<br><sub>context: p90 0.183 · p95 0.187 · p99 0.191 · 1702 op/s · total p50 0.515</sub> | 0.156<br><sub>context: p90 0.173 · p95 0.179 · p99 0.189 · 1799 op/s · total p50 0.503</sub> | -1.4% (-0.002) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.092<br><sub>context: p90 0.123 · p95 0.130 · p99 0.137 · 1926 op/s · total p50 0.456</sub> | 0.082<br><sub>context: p90 0.115 · p95 0.124 · p99 0.136 · 2105 op/s · total p50 0.389</sub> | -10.3% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.185<br><sub>context: p90 0.211 · p95 0.222 · p99 0.243 · 1649 op/s · total p50 0.567</sub> | 0.170<br><sub>context: p90 0.199 · p95 0.204 · p99 0.224 · 1892 op/s · total p50 0.482</sub> | -8.2% (-0.015) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 14.785<br><sub>context: p90 15.306 · p95 15.542 · p99 16.013 · 64 op/s · total p50 15.571</sub> | 14.689<br><sub>context: p90 15.129 · p95 15.279 · p99 15.455 · 65 op/s · total p50 15.391</sub> | -0.7% (-0.096) | 25% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.728<br><sub>context: p90 0.795 · p95 0.812 · p99 0.847 · 430 op/s · total p50 2.300</sub> | 0.732<br><sub>context: p90 0.786 · p95 0.796 · p99 0.832 · 428 op/s · total p50 2.311</sub> | +0.5% (+0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.062<br><sub>context: p90 0.092 · p95 0.095 · p99 0.101 · 2332 op/s · total p50 0.409</sub> | 0.056<br><sub>context: p90 0.085 · p95 0.091 · p99 0.094 · 2553 op/s · total p50 0.351</sub> | -9.7% (-0.006) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.089<br><sub>context: p90 0.104 · p95 0.105 · p99 0.111 · 1379 op/s · total p50 0.701</sub> | 0.083<br><sub>context: p90 0.096 · p95 0.099 · p99 0.112 · 1449 op/s · total p50 0.655</sub> | -6.0% (-0.005) | 10% AND 0.5 ms | 🟢 |

</details>
