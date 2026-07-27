### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 | ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff |
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

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 → ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.035<br><sub>context: p90 5.346 · p95 5.416 · p99 5.751 · 173 op/s · total p50 5.725</sub> | 4.888<br><sub>context: p90 5.243 · p95 5.296 · p99 5.439 · 178 op/s · total p50 5.530</sub> | -2.9% (-0.148) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.073<br><sub>context: p90 0.097 · p95 0.105 · p99 0.107 · 1846 op/s · total p50 0.524</sub> | 0.077<br><sub>context: p90 0.095 · p95 0.104 · p99 0.108 · 1799 op/s · total p50 0.513</sub> | +5.7% (+0.004) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.833<br><sub>context: p90 0.946 · p95 0.981 · p99 1.036 · 607 op/s · total p50 1.589</sub> | 0.799<br><sub>context: p90 0.920 · p95 0.954 · p99 1.025 · 636 op/s · total p50 1.492</sub> | -4.2% (-0.035) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.209 · p95 0.216 · p99 0.238 · 1268 op/s · total p50 0.729</sub> | 0.164<br><sub>context: p90 0.182 · p95 0.185 · p99 0.199 · 1345 op/s · total p50 0.702</sub> | -12.3% (-0.023) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.102<br><sub>context: p90 0.133 · p95 0.138 · p99 0.151 · 1629 op/s · total p50 0.574</sub> | 0.085<br><sub>context: p90 0.113 · p95 0.118 · p99 0.126 · 1476 op/s · total p50 0.646</sub> | -16.0% (-0.016) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.210<br><sub>context: p90 0.243 · p95 0.247 · p99 0.254 · 1368 op/s · total p50 0.675</sub> | 0.211<br><sub>context: p90 0.247 · p95 0.252 · p99 0.287 · 1429 op/s · total p50 0.661</sub> | +0.2% (+0.001) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.134<br><sub>context: p90 15.791 · p95 15.935 · p99 16.211 · 62 op/s · total p50 15.970</sub> | 14.850<br><sub>context: p90 15.375 · p95 15.458 · p99 15.813 · 64 op/s · total p50 15.638</sub> | -1.9% (-0.284) | 25% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.819<br><sub>context: p90 0.910 · p95 0.988 · p99 1.043 · 386 op/s · total p50 2.486</sub> | 0.840<br><sub>context: p90 0.931 · p95 0.952 · p99 1.035 · 372 op/s · total p50 2.534</sub> | +2.6% (+0.021) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.070<br><sub>context: p90 0.101 · p95 0.104 · p99 0.108 · 1727 op/s · total p50 0.532</sub> | 0.067<br><sub>context: p90 0.099 · p95 0.104 · p99 0.118 · 1762 op/s · total p50 0.541</sub> | -3.6% (-0.003) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.101<br><sub>context: p90 0.123 · p95 0.135 · p99 0.146 · 1140 op/s · total p50 0.837</sub> | 0.074<br><sub>context: p90 0.089 · p95 0.096 · p99 0.107 · 1291 op/s · total p50 0.739</sub> | -27.3% (-0.028) | 10% AND 0.5 ms | 🟢 |

</details>
