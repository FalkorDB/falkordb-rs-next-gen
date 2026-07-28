### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 | ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43 |
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

> ⚠ server image changed: falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.811<br><sub>context: p90 0.881 · p95 0.900 · p99 0.975 · 938 op/s · total p50 0.973</sub> | 4.104<br><sub>context: p90 4.360 · p95 4.427 · p99 4.465 · 221 op/s · total p50 4.543</sub> | +406.0% (+3.293) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.231 · p95 0.246 · p99 0.293 · 2778 op/s · total p50 0.278</sub> | 0.048<br><sub>context: p90 0.064 · p95 0.070 · p99 0.082 · 3125 op/s · total p50 0.268</sub> | -68.7% (-0.105) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.440<br><sub>context: p90 0.500 · p95 0.550 · p99 0.586 · 1528 op/s · total p50 0.569</sub> | 0.635<br><sub>context: p90 0.713 · p95 0.725 · p99 0.759 · 926 op/s · total p50 1.032</sub> | +44.4% (+0.195) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.376<br><sub>context: p90 0.422 · p95 0.442 · p99 0.478 · 1727 op/s · total p50 0.513</sub> | 0.120<br><sub>context: p90 0.137 · p95 0.140 · p99 0.142 · 2336 op/s · total p50 0.370</sub> | -68.0% (-0.256) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.149<br><sub>context: p90 0.181 · p95 0.196 · p99 0.215 · 2936 op/s · total p50 0.262</sub> | 0.069<br><sub>context: p90 0.098 · p95 0.101 · p99 0.106 · 2515 op/s · total p50 0.344</sub> | -53.4% (-0.080) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.249<br><sub>context: p90 0.272 · p95 0.276 · p99 0.304 · 2067 op/s · total p50 0.404</sub> | 0.158<br><sub>context: p90 0.195 · p95 0.203 · p99 0.206 · 2089 op/s · total p50 0.440</sub> | -36.5% (-0.091) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 24.161<br><sub>context: p90 25.202 · p95 25.487 · p99 25.733 · 41 op/s · total p50 24.505</sub> | 13.167<br><sub>context: p90 13.922 · p95 14.244 · p99 14.766 · 72 op/s · total p50 13.759</sub> | -45.5% (-10.994) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.381<br><sub>context: p90 0.458 · p95 0.501 · p99 0.539 · 1662 op/s · total p50 0.527</sub> | 0.616<br><sub>context: p90 0.672 · p95 0.698 · p99 0.727 · 524 op/s · total p50 1.921</sub> | +61.8% (+0.235) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.154 · p95 0.197 · p99 0.211 · 3589 op/s · total p50 0.209</sub> | 0.047<br><sub>context: p90 0.073 · p95 0.074 · p99 0.078 · 3392 op/s · total p50 0.256</sub> | -61.4% (-0.075) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.058<br><sub>context: p90 0.080 · p95 0.096 · p99 0.105 · 4654 op/s · total p50 0.143</sub> | 0.062<br><sub>context: p90 0.068 · p95 0.071 · p99 0.079 · 2014 op/s · total p50 0.466</sub> | +6.8% (+0.004) | 150% AND 2 ms | 🟢 |

</details>
