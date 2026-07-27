### 🧪 Synthetic per-op regression — pr vs main

| field | main | pr |
|---|---|---|
| FalkorDB module | 99.99.99 | 99.99.99 |
| server image | ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 | ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3 |
| workload_hash | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` | `sha256:3563fbd87df39d75b1b4c8f6116a27c1b4b835a6a2dfdeddfd0086f266f2133b` |
| samples / warmup | 200 / 50 | 200 / 50 |
| outcome oracle | — | — |

**Thresholds**

| scope | budget (slower than baseline) | floor (min Δ) |
|---|---|---|
| _default_ | 10% | 0.5 ms |
| `expand_hops_5` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `match_by_index` | 15% | 0.5 ms |
| `property_projection` | 15% | 0.5 ms |
| `return_const` | 15% | 0.5 ms |
| `shortest_path` | 12% (c16 18%, c32 25%) | 0.5 ms |
| `single_edge_update` | 25% | 0.5 ms |

_Metric `p50`. A cell is 🔴 only when the candidate is **slower** than the baseline by **more than** its budget **and** the absolute p50 increase exceeds the floor; faster (or slower within either bound) is 🟢 (N/A if the baseline is missing or ≤ 0). Budget precedence: per-op×concurrency > per-op > default._

**pr vs main** — 🟢 no p50 regression beyond budget across 10 comparable cell(s)

> ⚠ both runs measured oracle-eligible write op(s) (detach_delete_user, foreach_loop_mutation, merge_friend_edge_upsert, merge_user_insert_path, merge_user_upsert_existing, remove_user_property_and_label, single_edge_write, single_vertex_update, single_vertex_write) with no outcome oracle — latencies were compared WITHOUT the §6.3 correctness tier. Re-record with --oracle and replay with --require-oracle to enforce it

> ⚠ a FalkorDB module version is the dev placeholder — use tagged release images for a meaningful version comparison

> ⚠ server image changed: ghcr.io/falkordb/falkordb-server@sha256:885151f4b5a5d99ced693d75f663274c38bcfb329008e6af5433eaf7643bdf28 → ghcr.io/falkordb/falkordb-server@sha256:ac2ece579016801dc9192d33780b26c3dc22319ad0bb03374bf8ffb1a584a7f3

🟢 = faster or within budget · 🔴 = slower than budget **or** results differ · N/A = no perf verdict. Only **p50** is gated — the `context:` line (p90/p95/p99 · throughput) and `Δms` are informational, never part of the verdict. Non-blocking.

<details><summary>🟢 <code>detach_delete_user</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 5.400<br><sub>context: p90 5.704 · p95 5.842 · p99 6.039 · 183 op/s</sub> | 5.262<br><sub>context: p90 5.564 · p95 5.658 · p99 5.761 · 188 op/s</sub> | -2.5% (-0.138) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>foreach_loop_mutation</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.356<br><sub>context: p90 0.408 · p95 0.446 · p99 0.469 · 2452 op/s</sub> | 0.416<br><sub>context: p90 0.675 · p95 0.752 · p99 0.782 · 2192 op/s</sub> | +17.0% (+0.060) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_friend_edge_upsert</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 1.260<br><sub>context: p90 1.342 · p95 1.389 · p99 1.529 · 752 op/s</sub> | 1.250<br><sub>context: p90 1.418 · p95 1.632 · p99 1.658 · 758 op/s</sub> | -0.8% (-0.010) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_insert_path</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.499<br><sub>context: p90 0.543 · p95 0.561 · p99 0.591 · 1816 op/s</sub> | 0.525<br><sub>context: p90 0.728 · p95 0.859 · p99 0.889 · 1723 op/s</sub> | +5.2% (+0.026) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>merge_user_upsert_existing</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.461<br><sub>context: p90 0.659 · p95 0.791 · p99 0.849 · 1901 op/s</sub> | 0.400<br><sub>context: p90 0.475 · p95 0.492 · p99 0.535 · 2143 op/s</sub> | -13.4% (-0.062) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>remove_user_property_and_label</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.528<br><sub>context: p90 0.586 · p95 0.601 · p99 0.654 · 1730 op/s</sub> | 0.515<br><sub>context: p90 0.583 · p95 0.609 · p99 0.680 · 1763 op/s</sub> | -2.5% (-0.013) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 15.147<br><sub>context: p90 15.435 · p95 15.495 · p99 15.775 · 66 op/s</sub> | 15.313<br><sub>context: p90 15.572 · p95 15.657 · p99 15.885 · 65 op/s</sub> | +1.1% (+0.166) | 25% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_edge_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 2.372<br><sub>context: p90 2.521 · p95 2.654 · p99 2.777 · 410 op/s</sub> | 2.260<br><sub>context: p90 2.478 · p95 2.639 · p99 2.754 · 413 op/s</sub> | -4.7% (-0.112) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_update</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.328<br><sub>context: p90 0.376 · p95 0.389 · p99 0.424 · 2697 op/s</sub> | 0.345<br><sub>context: p90 0.397 · p95 0.420 · p99 0.456 · 2597 op/s</sub> | +5.2% (+0.017) | 10% AND 0.5 ms | 🟢 |

</details>

<details><summary>🟢 <code>single_vertex_write</code></summary>

_uncached (forced plan-cache miss — execution + compilation)_

| C | main p50 (ms) | pr p50 (ms) | Δp50 (Δms) | p50 guard (>% AND >ms) | verdict |
|---:|---:|---:|---:|:--:|:--:|
| 1 | 0.626<br><sub>context: p90 0.698 · p95 0.731 · p99 0.865 · 1508 op/s</sub> | 0.597<br><sub>context: p90 0.656 · p95 0.674 · p99 0.696 · 1565 op/s</sub> | -4.7% (-0.029) | 10% AND 0.5 ms | 🟢 |

</details>
