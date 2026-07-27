### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
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

**pr vs c-engine** — 🔴 1 of 10 comparable cell(s) over budget

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:311a2361cd17511cdb2811d161733cbf9db13a1877be706f1c591e809db51c43

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.865<br><sub>context: p90 0.926 · p95 0.952 · p99 1.070 · 822 op/s · total p50 1.109</sub> | 4.297<br><sub>context: p90 4.422 · p95 4.458 · p99 4.534 · 206 op/s · total p50 4.808</sub> | +396.8% (+3.432) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.224<br><sub>context: p90 0.297 · p95 0.317 · p99 0.329 · 2054 op/s · total p50 0.389</sub> | 0.062<br><sub>context: p90 0.077 · p95 0.089 · p99 0.108 · 2304 op/s · total p50 0.387</sub> | -72.3% (-0.162) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.540<br><sub>context: p90 0.616 · p95 0.644 · p99 0.725 · 1057 op/s · total p50 0.830</sub> | 0.735<br><sub>context: p90 0.791 · p95 0.801 · p99 0.818 · 765 op/s · total p50 1.253</sub> | +36.3% (+0.196) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.463<br><sub>context: p90 0.523 · p95 0.531 · p99 0.569 · 1263 op/s · total p50 0.682</sub> | 0.167<br><sub>context: p90 0.191 · p95 0.194 · p99 0.199 · 1692 op/s · total p50 0.541</sub> | -63.9% (-0.296) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.252<br><sub>context: p90 0.327 · p95 0.334 · p99 0.500 · 1633 op/s · total p50 0.449</sub> | 0.078<br><sub>context: p90 0.102 · p95 0.111 · p99 0.117 · 2177 op/s · total p50 0.413</sub> | -69.0% (-0.174) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.356<br><sub>context: p90 0.399 · p95 0.411 · p99 0.440 · 1490 op/s · total p50 0.571</sub> | 0.179<br><sub>context: p90 0.213 · p95 0.218 · p99 0.235 · 1677 op/s · total p50 0.551</sub> | -49.8% (-0.177) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 23.736<br><sub>context: p90 24.746 · p95 25.185 · p99 26.506 · 40 op/s · total p50 24.578</sub> | 12.581<br><sub>context: p90 12.705 · p95 12.756 · p99 12.942 · 74 op/s · total p50 13.444</sub> | -47.0% (-11.155) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.478<br><sub>context: p90 0.538 · p95 0.546 · p99 0.575 · 1222 op/s · total p50 0.690</sub> | 0.779<br><sub>context: p90 0.851 · p95 0.865 · p99 0.888 · 447 op/s · total p50 2.211</sub> | +63.0% (+0.301) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.187<br><sub>context: p90 0.260 · p95 0.265 · p99 0.293 · 2384 op/s · total p50 0.338</sub> | 0.057<br><sub>context: p90 0.078 · p95 0.086 · p99 0.093 · 2365 op/s · total p50 0.376</sub> | -69.7% (-0.131) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.106<br><sub>context: p90 0.152 · p95 0.177 · p99 0.185 · 2834 op/s · total p50 0.264</sub> | 0.099<br><sub>context: p90 0.116 · p95 0.121 · p99 0.130 · 1546 op/s · total p50 0.608</sub> | -6.8% (-0.007) | 150% AND 2 ms | 🟢 |

</details>
