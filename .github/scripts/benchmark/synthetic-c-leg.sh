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

CONTAINER="synthetic-cengine-$$"
cleanup() { docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; }
trap cleanup EXIT

stage() { printf '%s' "$1" > "$WORKDIR/c-stage"; echo "synthetic-c-leg: $1"; }

bench() { ( cd "$BENCHMARK_DIR" && cargo run --release --quiet --bin benchmark -- "$@" ); }

wait_for_redis() {
  local tries=60
  until docker exec "$CONTAINER" redis-cli PING 2>/dev/null | grep -q PONG; do
    tries=$((tries - 1))
    [ "$tries" -le 0 ] && { echo "::error::synthetic-c-leg: C container never became ready" >&2; docker logs "$CONTAINER" 2>&1 | tail -100 || true; return 1; }
    sleep 2
  done
}

stage "resolving the C-engine image digest (${IMAGE_CENGINE})"
C_DIGEST="$(resolve_digest "$IMAGE_CENGINE")"
printf '%s' "$C_DIGEST" > "$WORKDIR/c-digest"

stage "measuring the C engine (${C_DIGEST})"
echo "::group::synthetic: measuring c-engine (${C_DIGEST})"
docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
# BROWSER=0 is deliberately hardcoded (no pass-through env), per the three-way design: the bundle
# image otherwise starts a Node browser server inside the measured container.
docker run -d --name "$CONTAINER" --env BROWSER=0 \
  --cpus="$DB_CPUS" --memory="$DB_MEMORY" -p "${DB_PORT}:6379" "$C_DIGEST" >/dev/null
wait_for_redis
docker exec "$CONTAINER" redis-cli CONFIG SET save "" >/dev/null 2>&1 || true
docker exec "$CONTAINER" redis-cli CONFIG SET stop-writes-on-bgsave-error no >/dev/null 2>&1 || true
docker exec "$CONTAINER" redis-cli GRAPH.CONFIG SET MAX_QUEUED_QUERIES "$MAX_QUEUED_QUERIES" >/dev/null
bench synthetic run --recording "$WORKDIR/rec" \
  --endpoint "falkor://127.0.0.1:${DB_PORT}" \
  --label "c-engine" --server-image "$C_DIGEST" \
  --concurrency "$SWEEP" --cache "$CACHE" --samples "$SAMPLES" --warmup "$WARMUP" \
  --out "$WORKDIR/c.json"
docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
echo "::endgroup::"

# Cross-engine comparisons: looser budgets ([cross-engine] TOML profile) and the `advisory`
# divergence policy — engines legitimately differ, so a diverged op is ⚠ (needs a look), not 🔴.
stage "reporting c-pr (C engine → PR)"
echo "::group::synthetic: report c-pr (cross-engine, advisory)"
bench synthetic report --diff "$WORKDIR/c.json" "$WORKDIR/pr.json" \
  --regression --thresholds "$THRESHOLDS" \
  --budget-profile cross-engine --divergence-policy advisory \
  --out "$SYNTHETIC_OUT/report-c-pr.md" \
  --summary "$SYNTHETIC_OUT/summary-c-pr.json" \
  --cells "$SYNTHETIC_OUT/cells-c-pr.json" >/dev/null
echo "::endgroup::"

stage "reporting c-main (C engine → main)"
echo "::group::synthetic: report c-main (cross-engine, advisory)"
bench synthetic report --diff "$WORKDIR/c.json" "$WORKDIR/main.json" \
  --regression --thresholds "$THRESHOLDS" \
  --budget-profile cross-engine --divergence-policy advisory \
  --out "$SYNTHETIC_OUT/report-c-main.md" \
  --summary "$SYNTHETIC_OUT/summary-c-main.json" \
  --cells "$SYNTHETIC_OUT/cells-c-main.json" >/dev/null
echo "::endgroup::"

stage "done"
