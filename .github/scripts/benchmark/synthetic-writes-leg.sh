#!/usr/bin/env bash
# Writes leg of the synthetic three-way check — a SEPARATE child script (like synthetic-c-leg.sh)
# so synthetic-run.sh can guard it with an explicit `if` + `timeout` while `set -e` stays live in
# here. Runs AFTER the main-pr reads signal is persisted and BEFORE the C legs: main-pr writes
# (same-engine, gate policy) outranks the advisory cross-engine comparisons. Invoked ONLY by
# synthetic-run.sh, which passes everything via env (including CONTAINER — parent-assigned so a
# SIGKILLed child can never leak a container holding DB_PORT; the parent sweeps the name).
#
# Does, in order (writing the current stage to $WORKDIR/writes-stage so a failure is attributable):
#   1. record the WRITE bundle offline (`--repo-writes`, same workload TOML as the reads bundle —
#      write bundles are single-kind by design, so this is a second recording with its own
#      workload_hash; the recorded per-op budget pins C=1/samples/warmup and overrides the
#      global SWEEP/SAMPLES/WARMUP at replay);
#   2. measure the PR image, then main (one container at a time, own lifecycle);
#   3. report main-pr writes with --budget-profile strict --divergence-policy gate
#      (same-engine: writes are latency-tier only, correctness `not_gated`).
#
# On ANY failure the parent discards partial write artifacts and writes stub summaries instead —
# this script only has to fail loudly and early (set -euo pipefail).
set -euo pipefail

: "${BENCHMARK_DIR:?BENCHMARK_DIR is required}"
: "${WORKDIR:?WORKDIR (parent scratch dir) is required}"
: "${SYNTHETIC_OUT:?SYNTHETIC_OUT (persistent artifact dir) is required}"
: "${WORKLOAD:?WORKLOAD (synthetic-workload.toml path) is required}"
: "${THRESHOLDS:?THRESHOLDS (synthetic-thresholds.toml path) is required}"
: "${PR_DIGEST:?PR_DIGEST is required}"
: "${MAIN_DIGEST:?MAIN_DIGEST is required}"
: "${CONTAINER:?CONTAINER (parent-assigned container name) is required}"
: "${DB_PORT:?DB_PORT is required}"
: "${DB_CPUS:?DB_CPUS is required}"
: "${DB_MEMORY:?DB_MEMORY is required}"
: "${MAX_QUEUED_QUERIES:?MAX_QUEUED_QUERIES is required}"
: "${SWEEP:?SWEEP is required}"
: "${CACHE:?CACHE is required}"
: "${SAMPLES:?SAMPLES is required}"
: "${WARMUP:?WARMUP is required}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# bench / wait_for_redis / measure_recording / report_comparison — shared with the other legs.
# shellcheck source=.github/scripts/benchmark/synthetic-measure-lib.sh
. "$SCRIPT_DIR/synthetic-measure-lib.sh"

cleanup() { docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; }
trap cleanup EXIT

stage() { printf '%s' "$1" > "$WORKDIR/writes-stage"; echo "synthetic-writes-leg: $1"; }

stage "recording the write bundle (offline)"
bench synthetic record --config "$WORKLOAD" --repo-writes --out-dir "$WORKDIR/rec-writes"

stage "measuring pr writes (${PR_DIGEST})"
measure_recording "$WORKDIR/rec-writes" "pr" "$PR_DIGEST" "$WORKDIR/pr-writes.json"

stage "measuring main writes (${MAIN_DIGEST})"
measure_recording "$WORKDIR/rec-writes" "main" "$MAIN_DIGEST" "$WORKDIR/main-writes.json"

stage "reporting main-pr writes (strict, gate)"
report_comparison "$WORKDIR/main-writes.json" "$WORKDIR/pr-writes.json" \
  strict gate main-pr-writes

stage "done"
