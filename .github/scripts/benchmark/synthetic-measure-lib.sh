#!/usr/bin/env bash
# Shared measurement plumbing for the synthetic three-way check — sourced by synthetic-run.sh,
# synthetic-c-leg.sh, synthetic-writes-leg.sh and synthetic-c-writes-leg.sh (a fourth copy of
# these helpers was the alternative). Expects the caller to define/export:
#   BENCHMARK_DIR      checkout of FalkorDB/benchmark (cargo workspace-patched by the parent)
#   CONTAINER          the docker container name THIS process may create (parent-assigned for
#                      children: the parent force-removes every name it handed out, so even a
#                      SIGKILLed child — whose EXIT trap never runs — cannot leak a container
#                      holding DB_PORT into the next leg)
#   DB_PORT DB_CPUS DB_MEMORY MAX_QUEUED_QUERIES
#   SWEEP CACHE SAMPLES WARMUP
# Optional: SYNTH_CPU_PARTITION / SYNTH_SERVER_CPUS / SYNTH_CLIENT_CPUS / SYNTH_NPROC — CPU
# partitioning between the measured server and the closed-loop client (see synthetic-cpu-lib.sh
# for the policy; default-on, auto-split at >= 4 cpus, graceful no-pinning fallback otherwise).
#
# shellcheck shell=bash

SYNTHETIC_MEASURE_LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# cpu_partition_server / cpu_partition_client — pure policy helpers.
# shellcheck source=.github/scripts/benchmark/synthetic-cpu-lib.sh
. "$SYNTHETIC_MEASURE_LIB_DIR/synthetic-cpu-lib.sh"

# Partition the host CPUs between server and client ONCE per sourcing process (each leg logs its
# own line — the child re-derives the same sets from the same env). SYNTH_NPROC is an env seam
# for tests; nproc is coreutils (always on the CI runner), getconf covers macOS dev machines.
SYNTH_NPROC="${SYNTH_NPROC:-$(nproc 2>/dev/null || getconf _NPROCESSORS_ONLN 2>/dev/null || echo 0)}"
SERVER_CPUS="$(cpu_partition_server "$SYNTH_NPROC")"
CLIENT_CPUS="$(cpu_partition_client "$SYNTH_NPROC")"
if [ -n "$CLIENT_CPUS" ] && ! command -v taskset >/dev/null 2>&1; then
  echo "synthetic: cpu partition: no taskset on this host — client runs unpinned (macOS/dev fallback)"
  CLIENT_CPUS=""
fi
if [ -n "${SERVER_CPUS}${CLIENT_CPUS}" ]; then
  echo "synthetic: cpu partition: server cpus [${SERVER_CPUS:-unpinned}], client cpus [${CLIENT_CPUS:-unpinned}] (nproc=${SYNTH_NPROC})"
else
  echo "synthetic: cpu partition: off — server and client share all cpus (nproc=${SYNTH_NPROC})"
fi

bench() { ( cd "$BENCHMARK_DIR" && cargo run --release --quiet --bin benchmark -- "$@" ); }

# bench_measured — `bench` for the MEASURED replay only: pinned to CLIENT_CPUS (when computed)
# so the closed-loop workers never contend with the server's cpuset. taskset wraps `cargo run`;
# the spawned benchmark binary inherits the affinity. The offline record/report calls stay on
# plain `bench` — nothing races them, and the first `cargo run` build keeps every core.
bench_measured() {
  if [ -n "$CLIENT_CPUS" ]; then
    ( cd "$BENCHMARK_DIR" && taskset -c "$CLIENT_CPUS" cargo run --release --quiet --bin benchmark -- "$@" )
  else
    bench "$@"
  fi
}

wait_for_redis() {
  local tries=60
  until docker exec "$CONTAINER" redis-cli PING 2>/dev/null | grep -q PONG; do
    tries=$((tries - 1))
    [ "$tries" -le 0 ] && { echo "::error::synthetic: DB container ${CONTAINER} never became ready" >&2; docker logs "$CONTAINER" 2>&1 | tail -100 || true; return 1; }
    sleep 2
  done
}

# measure_recording <recording-dir> <label> <digest> <out.json> [extra docker run args…]
# Start the image, prep it, replay the recorded bundle against it, stop the container.
# Write bundles carry a recorded per-op budget that pins C=1 (and samples/warmup), overriding
# the global SWEEP/SAMPLES/WARMUP flags — so ONE flag set serves both bundle kinds verbatim.
measure_recording() {
  local rec="$1" label="$2" digest="$3" out="$4"
  shift 4
  echo "::group::synthetic: measuring ${label} (${digest})"
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
  # When partitioning is active the server is confined to SERVER_CPUS; the cpuset is then the
  # binding limit (the --cpus quota, sized for the whole host, never engages). run_args is never
  # empty, so the expansion is safe under `set -u` on bash 3.2 (local macOS runs matter).
  local -a run_args=(-d --name "$CONTAINER")
  if [ -n "$SERVER_CPUS" ]; then
    run_args+=(--cpuset-cpus "$SERVER_CPUS")
  fi
  docker run "${run_args[@]}" "$@" \
    --cpus="$DB_CPUS" --memory="$DB_MEMORY" -p "${DB_PORT}:6379" "$digest" >/dev/null
  wait_for_redis
  # Throwaway container: don't let a background save abort the load; give the sweep queue headroom.
  docker exec "$CONTAINER" redis-cli CONFIG SET save "" >/dev/null 2>&1 || true
  docker exec "$CONTAINER" redis-cli CONFIG SET stop-writes-on-bgsave-error no >/dev/null 2>&1 || true
  docker exec "$CONTAINER" redis-cli GRAPH.CONFIG SET MAX_QUEUED_QUERIES "$MAX_QUEUED_QUERIES" >/dev/null
  bench_measured synthetic run --recording "$rec" \
    --endpoint "falkor://127.0.0.1:${DB_PORT}" \
    --label "$label" --server-image "$digest" \
    --concurrency "$SWEEP" --cache "$CACHE" --samples "$SAMPLES" --warmup "$WARMUP" \
    --out "$out"
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
  echo "::endgroup::"
}

# report_comparison <baseline.json> <candidate.json> <profile> <policy> <artifact-stem>
# One `synthetic report --regression` call writing report-/summary-/cells- files for a slot.
report_comparison() {
  local baseline="$1" candidate="$2" profile="$3" policy="$4" stem="$5"
  echo "::group::synthetic: report ${stem} (${profile}, ${policy})"
  bench synthetic report --diff "$baseline" "$candidate" \
    --regression --thresholds "$THRESHOLDS" \
    --budget-profile "$profile" --divergence-policy "$policy" \
    --out "$SYNTHETIC_OUT/report-${stem}.md" \
    --summary "$SYNTHETIC_OUT/summary-${stem}.json" \
    --cells "$SYNTHETIC_OUT/cells-${stem}.json" >/dev/null
  echo "::endgroup::"
}
