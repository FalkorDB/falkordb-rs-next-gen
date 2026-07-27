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
| 1 | 0.868<br><sub>context: p90 0.930 · p95 0.943 · p99 1.015 · 890 op/s · total p50 1.060</sub> | 4.467<br><sub>context: p90 4.571 · p95 4.598 · p99 4.628 · 205 op/s · total p50 4.827</sub> | +414.6% (+3.599) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.207<br><sub>context: p90 0.278 · p95 0.289 · p99 0.352 · 2363 op/s · total p50 0.356</sub> | 0.053<br><sub>context: p90 0.075 · p95 0.079 · p99 0.104 · 2719 op/s · total p50 0.310</sub> | -74.5% (-0.154) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.502<br><sub>context: p90 0.570 · p95 0.578 · p99 0.650 · 1344 op/s · total p50 0.669</sub> | 0.692<br><sub>context: p90 0.749 · p95 0.761 · p99 0.777 · 858 op/s · total p50 1.122</sub> | +37.8% (+0.190) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.457<br><sub>context: p90 0.503 · p95 0.509 · p99 0.526 · 1379 op/s · total p50 0.655</sub> | 0.146<br><sub>context: p90 0.165 · p95 0.171 · p99 0.188 · 1958 op/s · total p50 0.461</sub> | -68.1% (-0.312) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.203<br><sub>context: p90 0.277 · p95 0.296 · p99 0.378 · 2241 op/s · total p50 0.364</sub> | 0.077<br><sub>context: p90 0.113 · p95 0.118 · p99 0.133 · 2262 op/s · total p50 0.409</sub> | -62.3% (-0.126) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.281<br><sub>context: p90 0.335 · p95 0.344 · p99 0.435 · 1892 op/s · total p50 0.466</sub> | 0.176<br><sub>context: p90 0.203 · p95 0.211 · p99 0.221 · 1917 op/s · total p50 0.488</sub> | -37.5% (-0.105) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 26.356<br><sub>context: p90 26.813 · p95 27.099 · p99 27.206 · 37 op/s · total p50 26.673</sub> | 14.617<br><sub>context: p90 15.260 · p95 15.308 · p99 15.465 · 65 op/s · total p50 15.195</sub> | -44.5% (-11.739) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.432<br><sub>context: p90 0.526 · p95 0.547 · p99 0.582 · 1470 op/s · total p50 0.598</sub> | 0.672<br><sub>context: p90 0.718 · p95 0.741 · p99 0.755 · 482 op/s · total p50 2.044</sub> | +55.5% (+0.240) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.157<br><sub>context: p90 0.207 · p95 0.232 · p99 0.290 · 2719 op/s · total p50 0.294</sub> | 0.058<br><sub>context: p90 0.086 · p95 0.090 · p99 0.102 · 2575 op/s · total p50 0.356</sub> | -62.8% (-0.099) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.077<br><sub>context: p90 0.134 · p95 0.145 · p99 0.162 · 3593 op/s · total p50 0.201</sub> | 0.070<br><sub>context: p90 0.086 · p95 0.090 · p99 0.096 · 1631 op/s · total p50 0.579</sub> | -9.3% (-0.007) | 150% AND 2 ms | 🟢 |

</details>
