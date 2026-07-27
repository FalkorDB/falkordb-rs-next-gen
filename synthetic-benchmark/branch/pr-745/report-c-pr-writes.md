### 🧪 Synthetic per-op regression — pr vs c-engine

| field | c-engine | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb |
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

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:3a8a2508bf220131681807ac603325f29afd9ef4c4c4efbafa140859db2b08cb

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.330<br><sub>context: p90 1.426 · p95 1.450 · p99 1.504 · 712 op/s</sub> | 5.359<br><sub>context: p90 5.521 · p95 5.629 · p99 5.828 · 184 op/s</sub> | +303.0% (+4.029) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.549<br><sub>context: p90 0.650 · p95 0.677 · p99 0.760 · 1607 op/s</sub> | 0.538<br><sub>context: p90 0.716 · p95 0.860 · p99 0.930 · 1762 op/s</sub> | -2.0% (-0.011) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.854<br><sub>context: p90 0.953 · p95 0.976 · p99 1.076 · 1077 op/s</sub> | 1.464<br><sub>context: p90 1.750 · p95 1.853 · p99 1.923 · 658 op/s</sub> | +71.4% (+0.610) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.828<br><sub>context: p90 0.956 · p95 0.996 · p99 1.025 · 1080 op/s</sub> | 0.646<br><sub>context: p90 0.902 · p95 0.937 · p99 1.024 · 1451 op/s</sub> | -22.0% (-0.182) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.590<br><sub>context: p90 0.677 · p95 0.705 · p99 0.722 · 1486 op/s</sub> | 0.550<br><sub>context: p90 0.769 · p95 0.846 · p99 0.905 · 1714 op/s</sub> | -6.8% (-0.040) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.585<br><sub>context: p90 0.674 · p95 0.707 · p99 0.748 · 1526 op/s</sub> | 0.645<br><sub>context: p90 0.920 · p95 1.011 · p99 1.105 · 1451 op/s</sub> | +10.2% (+0.060) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 27.182<br><sub>context: p90 27.742 · p95 27.912 · p99 28.026 · 37 op/s</sub> | 15.267<br><sub>context: p90 15.513 · p95 15.622 · p99 15.734 · 65 op/s</sub> | -43.8% (-11.916) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.797<br><sub>context: p90 0.891 · p95 0.928 · p99 0.979 · 1153 op/s</sub> | 2.494<br><sub>context: p90 2.613 · p95 2.772 · p99 2.880 · 391 op/s</sub> | +212.8% (+1.697) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.415<br><sub>context: p90 0.531 · p95 0.577 · p99 0.618 · 2074 op/s</sub> | 0.485<br><sub>context: p90 0.562 · p95 0.610 · p99 0.712 · 1927 op/s</sub> | +17.0% (+0.071) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.451 · p95 0.481 · p99 0.501 · 2488 op/s</sub> | 0.786<br><sub>context: p90 0.983 · p95 1.131 · p99 1.199 · 1205 op/s</sub> | +147.5% (+0.468) | 150% AND 2 ms | 🟢 |

</details>
