### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:61abfbbb28bfb890ba13e79520bf54fd58b91832c2bb667213fd3e6d3e9659ff

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.992<br><sub>context: p90 1.036 · p95 1.043 · p99 1.071 · 768 op/s · total p50 1.217</sub> | 4.888<br><sub>context: p90 5.243 · p95 5.296 · p99 5.439 · 178 op/s · total p50 5.530</sub> | +392.6% (+3.895) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.295<br><sub>context: p90 0.323 · p95 0.335 · p99 0.341 · 1468 op/s · total p50 0.603</sub> | 0.077<br><sub>context: p90 0.095 · p95 0.104 · p99 0.108 · 1799 op/s · total p50 0.513</sub> | -73.8% (-0.217) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.602<br><sub>context: p90 0.651 · p95 0.655 · p99 0.685 · 1069 op/s · total p50 0.843</sub> | 0.799<br><sub>context: p90 0.920 · p95 0.954 · p99 1.025 · 636 op/s · total p50 1.492</sub> | +32.7% (+0.197) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.508<br><sub>context: p90 0.553 · p95 0.564 · p99 0.618 · 1120 op/s · total p50 0.822</sub> | 0.164<br><sub>context: p90 0.182 · p95 0.185 · p99 0.199 · 1345 op/s · total p50 0.702</sub> | -67.8% (-0.344) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.275<br><sub>context: p90 0.314 · p95 0.320 · p99 0.345 · 1616 op/s · total p50 0.533</sub> | 0.085<br><sub>context: p90 0.113 · p95 0.118 · p99 0.126 · 1476 op/s · total p50 0.646</sub> | -69.0% (-0.190) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.347<br><sub>context: p90 0.384 · p95 0.394 · p99 0.406 · 1543 op/s · total p50 0.568</sub> | 0.211<br><sub>context: p90 0.247 · p95 0.252 · p99 0.287 · 1429 op/s · total p50 0.661</sub> | -39.2% (-0.136) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 27.338<br><sub>context: p90 27.848 · p95 28.129 · p99 28.621 · 36 op/s · total p50 27.802</sub> | 14.850<br><sub>context: p90 15.375 · p95 15.458 · p99 15.813 · 64 op/s · total p50 15.638</sub> | -45.7% (-12.488) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.521<br><sub>context: p90 0.580 · p95 0.601 · p99 0.617 · 1120 op/s · total p50 0.823</sub> | 0.840<br><sub>context: p90 0.931 · p95 0.952 · p99 1.035 · 372 op/s · total p50 2.534</sub> | +61.1% (+0.318) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.246 · p95 0.253 · p99 0.277 · 1878 op/s · total p50 0.455</sub> | 0.067<br><sub>context: p90 0.099 · p95 0.104 · p99 0.118 · 1762 op/s · total p50 0.541</sub> | -64.1% (-0.120) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.146<br><sub>context: p90 0.166 · p95 0.175 · p99 0.180 · 1893 op/s · total p50 0.488</sub> | 0.074<br><sub>context: p90 0.089 · p95 0.096 · p99 0.107 · 1291 op/s · total p50 0.739</sub> | -49.6% (-0.073) | 150% AND 2 ms | 🟢 |

</details>
