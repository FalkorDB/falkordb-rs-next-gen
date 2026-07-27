### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3 |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs c-engine** — 🔴 1 of 10 comparable cell(s) over budget

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.082<br><sub>context: p90 1.124 · p95 1.136 · p99 1.175 · 864 op/s</sub> | 5.262<br><sub>context: p90 5.564 · p95 5.658 · p99 5.761 · 188 op/s</sub> | +386.3% (+4.180) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.359<br><sub>context: p90 0.443 · p95 0.491 · p99 0.521 · 2256 op/s</sub> | 0.416<br><sub>context: p90 0.675 · p95 0.752 · p99 0.782 · 2192 op/s</sub> | +16.0% (+0.057) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.730<br><sub>context: p90 0.843 · p95 0.874 · p99 0.930 · 1240 op/s</sub> | 1.250<br><sub>context: p90 1.418 · p95 1.632 · p99 1.658 · 758 op/s</sub> | +71.3% (+0.520) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.591<br><sub>context: p90 0.683 · p95 0.730 · p99 0.773 · 1476 op/s</sub> | 0.525<br><sub>context: p90 0.728 · p95 0.859 · p99 0.889 · 1723 op/s</sub> | -11.2% (-0.066) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.343<br><sub>context: p90 0.410 · p95 0.434 · p99 0.456 · 2377 op/s</sub> | 0.400<br><sub>context: p90 0.475 · p95 0.492 · p99 0.535 · 2143 op/s</sub> | +16.6% (+0.057) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.487<br><sub>context: p90 0.576 · p95 0.615 · p99 0.669 · 1768 op/s</sub> | 0.515<br><sub>context: p90 0.583 · p95 0.609 · p99 0.680 · 1763 op/s</sub> | +5.7% (+0.028) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 26.866<br><sub>context: p90 27.198 · p95 27.393 · p99 27.568 · 37 op/s</sub> | 15.313<br><sub>context: p90 15.572 · p95 15.657 · p99 15.885 · 65 op/s</sub> | -43.0% (-11.553) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.677<br><sub>context: p90 0.764 · p95 0.787 · p99 0.853 · 1308 op/s</sub> | 2.260<br><sub>context: p90 2.478 · p95 2.639 · p99 2.754 · 413 op/s</sub> | +233.9% (+1.583) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.237<br><sub>context: p90 0.335 · p95 0.355 · p99 0.440 · 3124 op/s</sub> | 0.345<br><sub>context: p90 0.397 · p95 0.420 · p99 0.456 · 2597 op/s</sub> | +45.7% (+0.108) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.202<br><sub>context: p90 0.327 · p95 0.346 · p99 0.385 · 3390 op/s</sub> | 0.597<br><sub>context: p90 0.656 · p95 0.674 · p99 0.696 · 1565 op/s</sub> | +195.6% (+0.395) | 150% AND 2 ms | 🟢 |

</details>
