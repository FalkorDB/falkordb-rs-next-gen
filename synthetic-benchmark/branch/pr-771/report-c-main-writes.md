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
| 1 | 0.962<br><sub>context: p90 1.024 · p95 1.045 · p99 1.092 · 750 op/s · total p50 1.260</sub> | 4.817<br><sub>context: p90 4.895 · p95 4.940 · p99 4.998 · 182 op/s · total p50 5.464</sub> | +400.6% (+3.855) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.250<br><sub>context: p90 0.307 · p95 0.318 · p99 0.525 · 1755 op/s · total p50 0.474</sub> | 0.071<br><sub>context: p90 0.087 · p95 0.096 · p99 0.110 · 1871 op/s · total p50 0.482</sub> | -71.6% (-0.179) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.594<br><sub>context: p90 0.631 · p95 0.648 · p99 0.686 · 1062 op/s · total p50 0.861</sub> | 0.750<br><sub>context: p90 0.838 · p95 0.860 · p99 0.888 · 721 op/s · total p50 1.339</sub> | +26.2% (+0.156) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.514<br><sub>context: p90 0.545 · p95 0.554 · p99 0.565 · 1182 op/s · total p50 0.758</sub> | 0.171<br><sub>context: p90 0.188 · p95 0.198 · p99 0.207 · 1367 op/s · total p50 0.715</sub> | -66.7% (-0.343) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.275<br><sub>context: p90 0.313 · p95 0.324 · p99 0.332 · 1746 op/s · total p50 0.474</sub> | 0.097<br><sub>context: p90 0.128 · p95 0.134 · p99 0.150 · 1625 op/s · total p50 0.572</sub> | -64.7% (-0.178) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.347<br><sub>context: p90 0.383 · p95 0.388 · p99 0.396 · 1427 op/s · total p50 0.617</sub> | 0.196<br><sub>context: p90 0.224 · p95 0.230 · p99 0.255 · 1457 op/s · total p50 0.667</sub> | -43.5% (-0.151) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 26.743<br><sub>context: p90 27.372 · p95 27.491 · p99 27.846 · 36 op/s · total p50 27.385</sub> | 14.691<br><sub>context: p90 15.290 · p95 15.447 · p99 15.534 · 63 op/s · total p50 15.668</sub> | -45.1% (-12.052) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.484<br><sub>context: p90 0.570 · p95 0.579 · p99 0.608 · 1287 op/s · total p50 0.689</sub> | 0.818<br><sub>context: p90 0.867 · p95 0.875 · p99 0.941 · 396 op/s · total p50 2.507</sub> | +69.1% (+0.334) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.190<br><sub>context: p90 0.242 · p95 0.246 · p99 0.362 · 2177 op/s · total p50 0.374</sub> | 0.067<br><sub>context: p90 0.093 · p95 0.099 · p99 0.110 · 2022 op/s · total p50 0.451</sub> | -64.9% (-0.123) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.085<br><sub>context: p90 0.154 · p95 0.161 · p99 0.179 · 3171 op/s · total p50 0.219</sub> | 0.091<br><sub>context: p90 0.108 · p95 0.111 · p99 0.119 · 1260 op/s · total p50 0.756</sub> | +7.1% (+0.006) | 150% AND 2 ms | 🟢 |

</details>
