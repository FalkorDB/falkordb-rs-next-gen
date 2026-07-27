#!/usr/bin/env bash
# C-engine WRITES leg of the synthetic three-way check — a SEPARATE child script so
# synthetic-run.sh can guard it with an explicit `if` + `timeout` while `set -e` stays live in
# here. Runs ONLY when synthetic-writes-leg.sh succeeded (it needs the write bundle and the
# pr/main write measurements), and INDEPENDENTLY of the C reads leg's outcome — a C-reads hiccup
# doesn't forfeit C-writes, and vice versa. Invoked ONLY by synthetic-run.sh, which passes
# everything via env (including CONTAINER — parent-assigned so a SIGKILLed child can never leak
# a container holding DB_PORT; the parent sweeps the name).
#
# Does, in order (writing the current stage to $WORKDIR/c-writes-stage):
#   1. reuse the C digest the C reads leg resolved ($WORKDIR/c-digest) when present — the C tag
#      is a MOVING tag, and resolving twice could measure C reads and C writes on different
#      builds while run-meta attributes both to one digest; resolve fresh only when absent;
#   2. measure the C engine against the write bundle — `--env BROWSER=0` per the three-way
#      design (the bundle image's entrypoint otherwise starts an in-container Node browser);
#   3. report c-pr writes, then c-main writes, both cross-engine + advisory.
#
# On ANY failure the parent discards partial C-write artifacts and writes stub summaries instead —
# this script only has to fail loudly and early (set -euo pipefail).
set -euo pipefail

: "${BENCHMARK_DIR:?BENCHMARK_DIR is required}"
: "${WORKDIR:?WORKDIR (parent scratch dir with rec-writes/, pr-writes.json, main-writes.json) is required}"
: "${SYNTHETIC_OUT:?SYNTHETIC_OUT (persistent artifact dir) is required}"
: "${IMAGE_CENGINE:?IMAGE_CENGINE is required}"
: "${THRESHOLDS:?THRESHOLDS (synthetic-thresholds.toml path) is required}"
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
# digest_candidates / match_digest / resolve_digest — shared with the other legs.
# shellcheck source=.github/scripts/benchmark/synthetic-digest-lib.sh
. "$SCRIPT_DIR/synthetic-digest-lib.sh"
# bench / wait_for_redis / measure_recording / report_comparison.
# shellcheck source=.github/scripts/benchmark/synthetic-measure-lib.sh
. "$SCRIPT_DIR/synthetic-measure-lib.sh"

cleanup() { docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; }
trap cleanup EXIT

stage() { printf '%s' "$1" > "$WORKDIR/c-writes-stage"; echo "synthetic-c-writes-leg: $1"; }

if [ -s "$WORKDIR/c-digest" ]; then
  stage "reusing the C-engine digest resolved by the C reads leg"
  C_DIGEST="$(cat "$WORKDIR/c-digest")"
else
  stage "resolving the C-engine image digest (${IMAGE_CENGINE})"
  C_DIGEST="$(resolve_digest "$IMAGE_CENGINE")"
  printf '%s' "$C_DIGEST" > "$WORKDIR/c-digest"
fi

stage "measuring C-engine writes (${C_DIGEST})"
measure_recording "$WORKDIR/rec-writes" "c-engine" "$C_DIGEST" "$WORKDIR/c-writes.json" \
  --env BROWSER=0

stage "reporting c-pr writes (cross-engine, advisory)"
report_comparison "$WORKDIR/c-writes.json" "$WORKDIR/pr-writes.json" \
  cross-engine advisory c-pr-writes

stage "reporting c-main writes (cross-engine, advisory)"
report_comparison "$WORKDIR/c-writes.json" "$WORKDIR/main-writes.json" \
  cross-engine advisory c-main-writes

stage "done"
