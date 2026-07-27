#!/usr/bin/env bash
# C-engine leg of the synthetic three-way check — a SEPARATE child script so synthetic-run.sh can
# guard it with an explicit `if` + `timeout` while `set -e` stays live in here (Bash disables
# errexit inside any function/compound command tested by `if`, and `timeout` cannot run a shell
# function). Invoked ONLY by synthetic-run.sh, which passes everything via env.
#
# Does, in order (writing the current stage to $WORKDIR/c-stage so a failure is attributable):
#   1. resolve the C-engine image (IMAGE_CENGINE) to an immutable digest;
#   2. measure it against the recorded workload ($WORKDIR/rec) — the `docker run` line hardcodes
#      `--env BROWSER=0`: the bundle image's entrypoint otherwise starts an in-container Node
#      browser server (FalkorDB/FalkorDB build/docker/run.sh), which would contaminate latencies;
#   3. report c-pr  (C engine → PR)   with --budget-profile cross-engine --divergence-policy advisory;
#   4. report c-main (C engine → main), same profile/policy.
#
# On ANY failure the parent discards partial C artifacts and writes two not_comparable stub
# summaries instead — this script only has to fail loudly and early (set -euo pipefail).
set -euo pipefail

: "${BENCHMARK_DIR:?BENCHMARK_DIR is required}"
: "${WORKDIR:?WORKDIR (parent scratch dir with rec/, pr.json, main.json) is required}"
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
# digest_candidates / match_digest / resolve_digest — shared with synthetic-run.sh.
# shellcheck source=.github/scripts/benchmark/synthetic-digest-lib.sh
. "$SCRIPT_DIR/synthetic-digest-lib.sh"
# bench / wait_for_redis / measure_recording / report_comparison — shared with the other legs.
# CONTAINER is parent-assigned so a SIGKILLed child (timeout --kill-after: the EXIT trap never
# runs) can never leak a container holding DB_PORT — the parent sweeps every name it handed out.
# shellcheck source=.github/scripts/benchmark/synthetic-measure-lib.sh
. "$SCRIPT_DIR/synthetic-measure-lib.sh"

cleanup() { docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; }
trap cleanup EXIT

stage() { printf '%s' "$1" > "$WORKDIR/c-stage"; echo "synthetic-c-leg: $1"; }

stage "resolving the C-engine image digest (${IMAGE_CENGINE})"
C_DIGEST="$(resolve_digest "$IMAGE_CENGINE")"
printf '%s' "$C_DIGEST" > "$WORKDIR/c-digest"

stage "measuring the C engine (${C_DIGEST})"
# BROWSER=0 is deliberately hardcoded (no pass-through env), per the three-way design: the bundle
# image otherwise starts a Node browser server inside the measured container.
measure_recording "$WORKDIR/rec" "c-engine" "$C_DIGEST" "$WORKDIR/c.json" \
  --env BROWSER=0

# Cross-engine comparisons: looser budgets ([cross-engine] TOML profile) and the `advisory`
# divergence policy — engines legitimately differ, so a diverged op is ⚠ (needs a look), not 🔴.
stage "reporting c-pr (C engine → PR)"
report_comparison "$WORKDIR/c.json" "$WORKDIR/pr.json" \
  cross-engine advisory c-pr

stage "reporting c-main (C engine → main)"
report_comparison "$WORKDIR/c.json" "$WORKDIR/main.json" \
  cross-engine advisory c-main

stage "done"
