### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 | ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**Gated metric: `server_ms.p50`** (default) — the server-reported execution time; client-observed total latency is demoted to the `context:` line and is not part of any verdict in this comparison.

**pr vs c-engine** — 🔴 1 of 10 comparable cell(s) over budget

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb-server@sha256:7a40d2ef964c51fcdb069d7b06d2b3b8c62d54cca9721c1118d036614988d369 → ghcr.io/falkordb/falkordb-server@sha256:64943502b6ccb1eb99b46e9566a302d9efb5a306bc9ea0af08bfadd60cdcfaad

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.811<br><sub>context: p90 0.881 · p95 0.900 · p99 0.975 · 938 op/s · total p50 0.973</sub> | 4.112<br><sub>context: p90 4.286 · p95 4.326 · p99 4.434 · 223 op/s · total p50 4.481</sub> | +406.8% (+3.300) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.153<br><sub>context: p90 0.231 · p95 0.246 · p99 0.293 · 2778 op/s · total p50 0.278</sub> | 0.046<br><sub>context: p90 0.070 · p95 0.080 · p99 0.089 · 3213 op/s · total p50 0.246</sub> | -70.1% (-0.107) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.440<br><sub>context: p90 0.500 · p95 0.550 · p99 0.586 · 1528 op/s · total p50 0.569</sub> | 0.649<br><sub>context: p90 0.694 · p95 0.706 · p99 0.727 · 919 op/s · total p50 1.072</sub> | +47.4% (+0.209) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.376<br><sub>context: p90 0.422 · p95 0.442 · p99 0.478 · 1727 op/s · total p50 0.513</sub> | 0.131<br><sub>context: p90 0.145 · p95 0.151 · p99 0.156 · 2104 op/s · total p50 0.416</sub> | -65.3% (-0.246) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.149<br><sub>context: p90 0.181 · p95 0.196 · p99 0.215 · 2936 op/s · total p50 0.262</sub> | 0.072<br><sub>context: p90 0.100 · p95 0.105 · p99 0.116 · 2535 op/s · total p50 0.360</sub> | -51.3% (-0.076) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.249<br><sub>context: p90 0.272 · p95 0.276 · p99 0.304 · 2067 op/s · total p50 0.404</sub> | 0.152<br><sub>context: p90 0.186 · p95 0.194 · p99 0.200 · 2240 op/s · total p50 0.397</sub> | -39.0% (-0.097) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 24.161<br><sub>context: p90 25.202 · p95 25.487 · p99 25.733 · 41 op/s · total p50 24.505</sub> | 13.350<br><sub>context: p90 14.328 · p95 14.469 · p99 14.679 · 72 op/s · total p50 13.915</sub> | -44.7% (-10.812) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.381<br><sub>context: p90 0.458 · p95 0.501 · p99 0.539 · 1662 op/s · total p50 0.527</sub> | 0.655<br><sub>context: p90 0.710 · p95 0.720 · p99 0.748 · 501 op/s · total p50 1.978</sub> | +71.9% (+0.274) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.123<br><sub>context: p90 0.154 · p95 0.197 · p99 0.211 · 3589 op/s · total p50 0.209</sub> | 0.051<br><sub>context: p90 0.078 · p95 0.080 · p99 0.087 · 3031 op/s · total p50 0.293</sub> | -58.6% (-0.072) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.058<br><sub>context: p90 0.080 · p95 0.096 · p99 0.105 · 4654 op/s · total p50 0.143</sub> | 0.070<br><sub>context: p90 0.082 · p95 0.087 · p99 0.093 · 1795 op/s · total p50 0.522</sub> | +19.9% (+0.012) | 150% AND 2 ms | 🟢 |

</details>
