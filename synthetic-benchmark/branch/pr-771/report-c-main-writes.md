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
| 1 | 0.865<br><sub>context: p90 0.926 · p95 0.952 · p99 1.070 · 822 op/s · total p50 1.109</sub> | 4.304<br><sub>context: p90 4.420 · p95 4.463 · p99 4.552 · 206 op/s · total p50 4.806</sub> | +397.7% (+3.439) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.224<br><sub>context: p90 0.297 · p95 0.317 · p99 0.329 · 2054 op/s · total p50 0.389</sub> | 0.066<br><sub>context: p90 0.086 · p95 0.090 · p99 0.103 · 2110 op/s · total p50 0.423</sub> | -70.5% (-0.158) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.540<br><sub>context: p90 0.616 · p95 0.644 · p99 0.725 · 1057 op/s · total p50 0.830</sub> | 0.768<br><sub>context: p90 0.829 · p95 0.845 · p99 0.866 · 720 op/s · total p50 1.345</sub> | +42.3% (+0.229) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.463<br><sub>context: p90 0.523 · p95 0.531 · p99 0.569 · 1263 op/s · total p50 0.682</sub> | 0.170<br><sub>context: p90 0.190 · p95 0.195 · p99 0.200 · 1651 op/s · total p50 0.547</sub> | -63.2% (-0.293) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.252<br><sub>context: p90 0.327 · p95 0.334 · p99 0.500 · 1633 op/s · total p50 0.449</sub> | 0.097<br><sub>context: p90 0.126 · p95 0.133 · p99 0.140 · 1706 op/s · total p50 0.537</sub> | -61.4% (-0.155) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.356<br><sub>context: p90 0.399 · p95 0.411 · p99 0.440 · 1490 op/s · total p50 0.571</sub> | 0.177<br><sub>context: p90 0.198 · p95 0.211 · p99 0.225 · 1713 op/s · total p50 0.527</sub> | -50.3% (-0.179) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 23.736<br><sub>context: p90 24.746 · p95 25.185 · p99 26.506 · 40 op/s · total p50 24.578</sub> | 12.615<br><sub>context: p90 12.743 · p95 12.783 · p99 12.834 · 73 op/s · total p50 13.533</sub> | -46.9% (-11.121) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.478<br><sub>context: p90 0.538 · p95 0.546 · p99 0.575 · 1222 op/s · total p50 0.690</sub> | 0.762<br><sub>context: p90 0.846 · p95 0.871 · p99 0.912 · 458 op/s · total p50 2.157</sub> | +59.4% (+0.284) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.260 · p95 0.265 · p99 0.293 · 2384 op/s · total p50 0.338</sub> | 0.062<br><sub>context: p90 0.084 · p95 0.087 · p99 0.111 · 2290 op/s · total p50 0.386</sub> | -67.0% (-0.126) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.152 · p95 0.177 · p99 0.185 · 2834 op/s · total p50 0.264</sub> | 0.092<br><sub>context: p90 0.113 · p95 0.117 · p99 0.125 · 1581 op/s · total p50 0.587</sub> | -12.9% (-0.014) | 150% AND 2 ms | 🟢 |

</details>
