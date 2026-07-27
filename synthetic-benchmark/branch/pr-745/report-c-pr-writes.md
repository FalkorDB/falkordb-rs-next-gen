### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:146c29a9bcf897899002be990d694ebcc09d6420236f949320553cf9e294993f

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** of `server_ms` (server-reported execution time) is gated — the `context:` line (p90/p95/p99 · throughput · client-observed total p50) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.784<br><sub>context: p90 0.872 · p95 0.887 · p99 0.908 · 954 op/s · total p50 0.963</sub> | 4.206<br><sub>context: p90 4.369 · p95 4.433 · p99 4.504 · 203 op/s · total p50 4.905</sub> | +436.2% (+3.422) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.183<br><sub>context: p90 0.298 · p95 0.308 · p99 0.625 · 2309 op/s · total p50 0.337</sub> | 0.074<br><sub>context: p90 0.094 · p95 0.102 · p99 0.106 · 1720 op/s · total p50 0.546</sub> | -59.3% (-0.109) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.502<br><sub>context: p90 0.625 · p95 0.822 · p99 0.909 · 1295 op/s · total p50 0.705</sub> | 0.724<br><sub>context: p90 0.802 · p95 0.847 · p99 0.865 · 722 op/s · total p50 1.360</sub> | +44.3% (+0.222) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.414<br><sub>context: p90 0.482 · p95 0.514 · p99 0.563 · 1425 op/s · total p50 0.630</sub> | 0.185<br><sub>context: p90 0.210 · p95 0.217 · p99 0.226 · 1403 op/s · total p50 0.679</sub> | -55.2% (-0.229) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.190<br><sub>context: p90 0.291 · p95 0.319 · p99 0.610 · 2308 op/s · total p50 0.337</sub> | 0.100<br><sub>context: p90 0.135 · p95 0.138 · p99 0.147 · 1772 op/s · total p50 0.537</sub> | -47.6% (-0.090) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.256<br><sub>context: p90 0.342 · p95 0.370 · p99 0.401 · 1945 op/s · total p50 0.412</sub> | 0.190<br><sub>context: p90 0.221 · p95 0.228 · p99 0.248 · 1416 op/s · total p50 0.677</sub> | -25.8% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 23.597<br><sub>context: p90 24.202 · p95 24.303 · p99 24.585 · 41 op/s · total p50 24.204</sub> | 12.653<br><sub>context: p90 13.111 · p95 13.337 · p99 13.397 · 73 op/s · total p50 13.617</sub> | -46.4% (-10.944) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.475<br><sub>context: p90 0.555 · p95 0.568 · p99 0.815 · 1334 op/s · total p50 0.693</sub> | 0.789<br><sub>context: p90 0.849 · p95 0.859 · p99 0.896 · 442 op/s · total p50 2.243</sub> | +66.2% (+0.314) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.180<br><sub>context: p90 0.265 · p95 0.273 · p99 0.370 · 2233 op/s · total p50 0.348</sub> | 0.068<br><sub>context: p90 0.096 · p95 0.101 · p99 0.107 · 2023 op/s · total p50 0.473</sub> | -62.3% (-0.112) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.089<br><sub>context: p90 0.160 · p95 0.168 · p99 0.201 · 3141 op/s · total p50 0.230</sub> | 0.112<br><sub>context: p90 0.129 · p95 0.133 · p99 0.142 · 1289 op/s · total p50 0.756</sub> | +26.0% (+0.023) | 150% AND 2 ms | 🟢 |

</details>
