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
| 1 | 1.014<br><sub>context: p90 1.057 · p95 1.070 · p99 1.123 · 651 op/s · total p50 1.469</sub> | 4.557<br><sub>context: p90 4.732 · p95 4.765 · p99 4.858 · 197 op/s · total p50 5.027</sub> | +349.3% (+3.543) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.291<br><sub>context: p90 0.323 · p95 0.334 · p99 0.359 · 1482 op/s · total p50 0.592</sub> | 0.061<br><sub>context: p90 0.082 · p95 0.087 · p99 0.095 · 2348 op/s · total p50 0.376</sub> | -78.9% (-0.230) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.612<br><sub>context: p90 0.644 · p95 0.664 · p99 0.704 · 909 op/s · total p50 1.035</sub> | 0.715<br><sub>context: p90 0.772 · p95 0.780 · p99 0.815 · 761 op/s · total p50 1.235</sub> | +16.8% (+0.103) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.529<br><sub>context: p90 0.566 · p95 0.574 · p99 0.593 · 1000 op/s · total p50 0.891</sub> | 0.156<br><sub>context: p90 0.173 · p95 0.179 · p99 0.189 · 1799 op/s · total p50 0.503</sub> | -70.5% (-0.373) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.305<br><sub>context: p90 0.331 · p95 0.342 · p99 0.397 · 1360 op/s · total p50 0.629</sub> | 0.082<br><sub>context: p90 0.115 · p95 0.124 · p99 0.136 · 2105 op/s · total p50 0.389</sub> | -72.9% (-0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.353<br><sub>context: p90 0.381 · p95 0.392 · p99 0.397 · 1443 op/s · total p50 0.608</sub> | 0.170<br><sub>context: p90 0.199 · p95 0.204 · p99 0.224 · 1892 op/s · total p50 0.482</sub> | -51.9% (-0.183) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 26.852<br><sub>context: p90 27.691 · p95 27.897 · p99 28.825 · 36 op/s · total p50 27.350</sub> | 14.689<br><sub>context: p90 15.129 · p95 15.279 · p99 15.455 · 65 op/s · total p50 15.391</sub> | -45.3% (-12.163) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.569<br><sub>context: p90 0.605 · p95 0.614 · p99 0.635 · 949 op/s · total p50 0.970</sub> | 0.732<br><sub>context: p90 0.786 · p95 0.796 · p99 0.832 · 428 op/s · total p50 2.311</sub> | +28.6% (+0.163) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.234<br><sub>context: p90 0.258 · p95 0.267 · p99 0.281 · 1476 op/s · total p50 0.591</sub> | 0.056<br><sub>context: p90 0.085 · p95 0.091 · p99 0.094 · 2553 op/s · total p50 0.351</sub> | -76.0% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.154<br><sub>context: p90 0.176 · p95 0.179 · p99 0.194 · 1876 op/s · total p50 0.463</sub> | 0.083<br><sub>context: p90 0.096 · p95 0.099 · p99 0.112 · 1449 op/s · total p50 0.655</sub> | -45.9% (-0.071) | 150% AND 2 ms | 🟢 |

</details>
