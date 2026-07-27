### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**main vs c-engine** — 🔴 1 of 10 comparable cell(s) over budget

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.784<br><sub>context: p90 0.872 · p95 0.887 · p99 0.908 · 954 op/s · total p50 0.963</sub> | 4.225<br><sub>context: p90 4.437 · p95 4.451 · p99 4.541 · 208 op/s · total p50 4.744</sub> | +438.5% (+3.440) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.183<br><sub>context: p90 0.298 · p95 0.308 · p99 0.625 · 2309 op/s · total p50 0.337</sub> | 0.075<br><sub>context: p90 0.093 · p95 0.102 · p99 0.117 · 1994 op/s · total p50 0.474</sub> | -59.3% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.502<br><sub>context: p90 0.625 · p95 0.822 · p99 0.909 · 1295 op/s · total p50 0.705</sub> | 0.728<br><sub>context: p90 0.806 · p95 0.829 · p99 0.875 · 730 op/s · total p50 1.320</sub> | +45.0% (+0.226) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.414<br><sub>context: p90 0.482 · p95 0.514 · p99 0.563 · 1425 op/s · total p50 0.630</sub> | 0.182<br><sub>context: p90 0.209 · p95 0.217 · p99 0.224 · 1579 op/s · total p50 0.602</sub> | -56.0% (-0.232) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.190<br><sub>context: p90 0.291 · p95 0.319 · p99 0.610 · 2308 op/s · total p50 0.337</sub> | 0.102<br><sub>context: p90 0.124 · p95 0.134 · p99 0.146 · 1733 op/s · total p50 0.552</sub> | -46.4% (-0.088) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.256<br><sub>context: p90 0.342 · p95 0.370 · p99 0.401 · 1945 op/s · total p50 0.412</sub> | 0.174<br><sub>context: p90 0.207 · p95 0.212 · p99 0.225 · 1606 op/s · total p50 0.594</sub> | -32.3% (-0.083) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 23.597<br><sub>context: p90 24.202 · p95 24.303 · p99 24.585 · 41 op/s · total p50 24.204</sub> | 12.572<br><sub>context: p90 12.760 · p95 12.833 · p99 12.891 · 74 op/s · total p50 13.430</sub> | -46.7% (-11.025) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.475<br><sub>context: p90 0.555 · p95 0.568 · p99 0.815 · 1334 op/s · total p50 0.693</sub> | 0.730<br><sub>context: p90 0.802 · p95 0.839 · p99 0.898 · 449 op/s · total p50 2.225</sub> | +53.7% (+0.255) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.265 · p95 0.273 · p99 0.370 · 2233 op/s · total p50 0.348</sub> | 0.067<br><sub>context: p90 0.093 · p95 0.098 · p99 0.110 · 2132 op/s · total p50 0.445</sub> | -62.9% (-0.113) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.089<br><sub>context: p90 0.160 · p95 0.168 · p99 0.201 · 3141 op/s · total p50 0.230</sub> | 0.104<br><sub>context: p90 0.122 · p95 0.126 · p99 0.129 · 1462 op/s · total p50 0.650</sub> | +17.0% (+0.015) | 150% AND 2 ms | 🟢 |

</details>
