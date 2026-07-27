### 🧪 Synthetic per-op regression — main vs c-engine

| field | c-engine | main |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c | ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 150% | 2 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**main vs c-engine** — 🔴 1 of 10 comparable cell(s) over budget

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: falkordb/falkordb@sha256:e47e0fb112ff29764965a1c25e2f983dd269de33367ca3f2fba61368b735f38c → ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28

🟢 = faster or within budget · 🔴 = slower than budget · ⚠ = results differ (advisory — the engines did different work, so perf is N/A) · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🔴 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.330<br><sub>context: p90 1.426 · p95 1.450 · p99 1.504 · 712 op/s</sub> | 5.539<br><sub>context: p90 5.918 · p95 6.086 · p99 6.321 · 178 op/s</sub> | +316.6% (+4.210) | 150% AND 2 ms | 🔴 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.549<br><sub>context: p90 0.650 · p95 0.677 · p99 0.760 · 1607 op/s</sub> | 0.505<br><sub>context: p90 0.726 · p95 0.838 · p99 0.878 · 1896 op/s</sub> | -8.1% (-0.045) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.854<br><sub>context: p90 0.953 · p95 0.976 · p99 1.076 · 1077 op/s</sub> | 1.482<br><sub>context: p90 1.795 · p95 1.869 · p99 1.921 · 654 op/s</sub> | +73.4% (+0.627) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.828<br><sub>context: p90 0.956 · p95 0.996 · p99 1.025 · 1080 op/s</sub> | 0.668<br><sub>context: p90 0.940 · p95 0.976 · p99 0.997 · 1403 op/s</sub> | -19.3% (-0.159) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.590<br><sub>context: p90 0.677 · p95 0.705 · p99 0.722 · 1486 op/s</sub> | 0.543<br><sub>context: p90 0.780 · p95 0.946 · p99 1.002 · 1679 op/s</sub> | -8.0% (-0.047) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.585<br><sub>context: p90 0.674 · p95 0.707 · p99 0.748 · 1526 op/s</sub> | 0.653<br><sub>context: p90 0.958 · p95 1.060 · p99 1.187 · 1432 op/s</sub> | +11.6% (+0.068) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 27.182<br><sub>context: p90 27.742 · p95 27.912 · p99 28.026 · 37 op/s</sub> | 15.341<br><sub>context: p90 15.623 · p95 15.713 · p99 15.910 · 65 op/s</sub> | -43.6% (-11.842) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.797<br><sub>context: p90 0.891 · p95 0.928 · p99 0.979 · 1153 op/s</sub> | 2.579<br><sub>context: p90 2.729 · p95 2.886 · p99 3.028 · 376 op/s</sub> | +223.5% (+1.782) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.415<br><sub>context: p90 0.531 · p95 0.577 · p99 0.618 · 2074 op/s</sub> | 0.514<br><sub>context: p90 0.684 · p95 0.779 · p99 0.840 · 1840 op/s</sub> | +24.0% (+0.099) | 150% AND 2 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | c-engine p50 (ms) | main p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.318<br><sub>context: p90 0.451 · p95 0.481 · p99 0.501 · 2488 op/s</sub> | 0.789<br><sub>context: p90 0.922 · p95 1.044 · p99 1.127 · 1215 op/s</sub> | +148.6% (+0.472) | 150% AND 2 ms | 🟢 |

</details>
