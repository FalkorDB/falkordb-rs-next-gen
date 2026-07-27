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
| 1 | 1.014<br><sub>context: p90 1.057 · p95 1.070 · p99 1.123 · 651 op/s · total p50 1.469</sub> | 4.673<br><sub>context: p90 4.802 · p95 4.845 · p99 4.893 · 189 op/s · total p50 5.271</sub> | +360.7% (+3.658) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.291<br><sub>context: p90 0.323 · p95 0.334 · p99 0.359 · 1482 op/s · total p50 0.592</sub> | 0.063<br><sub>context: p90 0.080 · p95 0.085 · p99 0.099 · 2257 op/s · total p50 0.397</sub> | -78.4% (-0.228) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.612<br><sub>context: p90 0.644 · p95 0.664 · p99 0.704 · 909 op/s · total p50 1.035</sub> | 0.717<br><sub>context: p90 0.788 · p95 0.800 · p99 0.828 · 778 op/s · total p50 1.236</sub> | +17.2% (+0.105) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.529<br><sub>context: p90 0.566 · p95 0.574 · p99 0.593 · 1000 op/s · total p50 0.891</sub> | 0.158<br><sub>context: p90 0.183 · p95 0.187 · p99 0.191 · 1702 op/s · total p50 0.515</sub> | -70.0% (-0.370) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.305<br><sub>context: p90 0.331 · p95 0.342 · p99 0.397 · 1360 op/s · total p50 0.629</sub> | 0.092<br><sub>context: p90 0.123 · p95 0.130 · p99 0.137 · 1926 op/s · total p50 0.456</sub> | -69.8% (-0.213) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.353<br><sub>context: p90 0.381 · p95 0.392 · p99 0.397 · 1443 op/s · total p50 0.608</sub> | 0.185<br><sub>context: p90 0.211 · p95 0.222 · p99 0.243 · 1649 op/s · total p50 0.567</sub> | -47.6% (-0.168) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 26.852<br><sub>context: p90 27.691 · p95 27.897 · p99 28.825 · 36 op/s · total p50 27.350</sub> | 14.785<br><sub>context: p90 15.306 · p95 15.542 · p99 16.013 · 64 op/s · total p50 15.571</sub> | -44.9% (-12.067) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.569<br><sub>context: p90 0.605 · p95 0.614 · p99 0.635 · 949 op/s · total p50 0.970</sub> | 0.728<br><sub>context: p90 0.795 · p95 0.812 · p99 0.847 · 430 op/s · total p50 2.300</sub> | +28.0% (+0.159) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.234<br><sub>context: p90 0.258 · p95 0.267 · p99 0.281 · 1476 op/s · total p50 0.591</sub> | 0.062<br><sub>context: p90 0.092 · p95 0.095 · p99 0.101 · 2332 op/s · total p50 0.409</sub> | -73.4% (-0.172) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.154<br><sub>context: p90 0.176 · p95 0.179 · p99 0.194 · 1876 op/s · total p50 0.463</sub> | 0.089<br><sub>context: p90 0.104 · p95 0.105 · p99 0.111 · 1379 op/s · total p50 0.701</sub> | -42.4% (-0.065) | 150% AND 2 ms | 🟢 |

</details>
