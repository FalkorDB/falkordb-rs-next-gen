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
| 1 | 0.868<br><sub>context: p90 0.930 · p95 0.943 · p99 1.015 · 890 op/s · total p50 1.060</sub> | 4.456<br><sub>context: p90 4.589 · p95 4.664 · p99 4.759 · 203 op/s · total p50 4.851</sub> | +413.4% (+3.588) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.207<br><sub>context: p90 0.278 · p95 0.289 · p99 0.352 · 2363 op/s · total p50 0.356</sub> | 0.060<br><sub>context: p90 0.082 · p95 0.090 · p99 0.100 · 2399 op/s · total p50 0.382</sub> | -71.2% (-0.147) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.502<br><sub>context: p90 0.570 · p95 0.578 · p99 0.650 · 1344 op/s · total p50 0.669</sub> | 0.686<br><sub>context: p90 0.725 · p95 0.736 · p99 0.756 · 859 op/s · total p50 1.126</sub> | +36.6% (+0.184) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.457<br><sub>context: p90 0.503 · p95 0.509 · p99 0.526 · 1379 op/s · total p50 0.655</sub> | 0.151<br><sub>context: p90 0.168 · p95 0.175 · p99 0.182 · 1772 op/s · total p50 0.530</sub> | -67.0% (-0.306) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.203<br><sub>context: p90 0.277 · p95 0.296 · p99 0.378 · 2241 op/s · total p50 0.364</sub> | 0.075<br><sub>context: p90 0.103 · p95 0.110 · p99 0.122 · 2448 op/s · total p50 0.357</sub> | -63.2% (-0.128) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.335 · p95 0.344 · p99 0.435 · 1892 op/s · total p50 0.466</sub> | 0.180<br><sub>context: p90 0.208 · p95 0.214 · p99 0.235 · 1775 op/s · total p50 0.533</sub> | -36.1% (-0.102) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 26.356<br><sub>context: p90 26.813 · p95 27.099 · p99 27.206 · 37 op/s · total p50 26.673</sub> | 14.702<br><sub>context: p90 15.192 · p95 15.544 · p99 15.667 · 65 op/s · total p50 15.306</sub> | -44.2% (-11.655) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.432<br><sub>context: p90 0.526 · p95 0.547 · p99 0.582 · 1470 op/s · total p50 0.598</sub> | 0.693<br><sub>context: p90 0.725 · p95 0.738 · p99 0.749 · 468 op/s · total p50 2.114</sub> | +60.3% (+0.261) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.157<br><sub>context: p90 0.207 · p95 0.232 · p99 0.290 · 2719 op/s · total p50 0.294</sub> | 0.055<br><sub>context: p90 0.086 · p95 0.089 · p99 0.093 · 2541 op/s · total p50 0.364</sub> | -65.3% (-0.103) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.077<br><sub>context: p90 0.134 · p95 0.145 · p99 0.162 · 3593 op/s · total p50 0.201</sub> | 0.078<br><sub>context: p90 0.088 · p95 0.090 · p99 0.101 · 1573 op/s · total p50 0.607</sub> | +1.4% (+0.001) | 150% AND 2 ms | 🟢 |

</details>
