#!/usr/bin/env bash
# Per-PR SYNTHETIC benchmark regression check (see the `synthetic-*` jobs in
# .github/workflows/_benchmark.yml, invoked by benchmark.yml).
#
# Records the FalkorDB/benchmark `synthetic` workload ONCE (offline, deterministic), then measures
# three engine images SEQUENTIALLY on THIS one VM (one container at a time): the PR (`rc-pr-<N>`),
# Rust main (`edge-rs`), and the production C engine (`falkordb/falkordb:edge`, run with BROWSER=0).
# Three comparisons are reported (stable IDs used in filenames, JSON and the page):
#
#   main-pr — main → PR      (did this PR regress the Rust engine?)   strict budgets, gate policy
#   c-pr    — C engine → PR  (how does the PR stand vs C?)            cross-engine, advisory
#   c-main  — C engine → main (was main already red vs C?)            cross-engine, advisory
#
# Failure isolation: `main-pr` is the gating signal — it is measured and its artifacts persisted
# FIRST, before the C leg runs as a separate, time-bounded child script (synthetic-c-leg.sh). A C
# hiccup can never cost the main-pr signal: on C failure two stub summaries record WHY, both C
# comparisons are marked unavailable in data.json, and this script still exits 0.
#
# Same-machine, one-at-a-time: runner speed and neighbour noise cancel, and the latency comparison
# is meaningful. Images are pinned to immutable digests up front so a moving tag can't swap a build
# mid-run. The check is informational: the workflow job is non-blocking and always publishes.
#
# Self-test: `synthetic-run.sh --self-test` runs the pure-bash digest-normalization tests (no
# Docker, no env needed) and exits — used by CI and local validation.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# digest_candidates / match_digest / resolve_digest — shared with synthetic-c-leg.sh.
# shellcheck source=.github/scripts/benchmark/synthetic-digest-lib.sh
. "$SCRIPT_DIR/synthetic-digest-lib.sh"

self_test() {
  local failures=0
  expect_candidates() {
    local image="$1" want="$2" got
    got="$(digest_candidates "$image" | tr '\n' ' ' | sed 's/ $//')"
    if [ "$got" != "$want" ]; then
      echo "FAIL digest_candidates($image): got '$got', want '$want'" >&2
      failures=$((failures + 1))
    else
      echo "ok   digest_candidates($image) -> $got"
    fi
  }
  # Docker Hub two-segment ref (the C-engine default; the G5 bug case).
  expect_candidates "docker.io/falkordb/falkordb:edge" \
    "docker.io/falkordb/falkordb falkordb/falkordb"
  # GHCR (unchanged, matches as-given).
  expect_candidates "ghcr.io/falkordb/falkordb-server:rc-pr-1" \
    "ghcr.io/falkordb/falkordb-server"
  # Port-qualified registry, no tag: the ':' is in the host segment, never stripped.
  expect_candidates "localhost:5000/repo" "localhost:5000/repo"
  expect_candidates "localhost:5000/repo:tag" "localhost:5000/repo"
  # Official single-name spellings.
  expect_candidates "alpine:3.20" "alpine library/alpine"
  expect_candidates "docker.io/library/alpine:3.20" \
    "docker.io/library/alpine library/alpine alpine"
  expect_candidates "docker.io/alpine" "docker.io/alpine alpine library/alpine"

  # Pre-pinned digest refs short-circuit resolve_digest before any Docker call.
  local pinned="ghcr.io/falkordb/falkordb-server@sha256:0000000000000000000000000000000000000000000000000000000000000000"
  local got
  got="$(resolve_digest "$pinned")"
  if [ "$got" != "$pinned" ]; then
    echo "FAIL resolve_digest(pinned): got '$got', want '$pinned'" >&2
    failures=$((failures + 1))
  else
    echo "ok   resolve_digest(pinned) passes through verbatim"
  fi

  # match_digest picks the normalized Docker Hub entry and prefers the first candidate that hits.
  local digests=$'falkordb/falkordb@sha256:aaa\nother/repo@sha256:bbb'
  got="$(match_digest "$digests" "docker.io/falkordb/falkordb" "falkordb/falkordb")" || true
  if [ "$got" != "falkordb/falkordb@sha256:aaa" ]; then
    echo "FAIL match_digest(hub): got '$got'" >&2
    failures=$((failures + 1))
  else
    echo "ok   match_digest resolves the docker.io-normalized entry"
  fi
  if match_digest "$digests" "ghcr.io/nope" >/dev/null; then
    echo "FAIL match_digest(no hit) should fail" >&2
    failures=$((failures + 1))
  else
    echo "ok   match_digest fails cleanly on no match"
  fi

  if [ "$failures" -gt 0 ]; then
    echo "self-test: $failures failure(s)" >&2
    return 1
  fi
  echo "self-test: all digest-normalization checks passed"
}

if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit 0
fi

: "${BENCHMARK_DIR:?BENCHMARK_DIR (checkout of FalkorDB/benchmark) is required}"
: "${CONFIG_DIR:?CONFIG_DIR (this repo checkout, for .github/synthetic-*.toml) is required}"
: "${IMAGE_PR:?IMAGE_PR (ghcr.io/falkordb/falkordb-server:rc-pr-<N>) is required}"
: "${IMAGE_MAIN:?IMAGE_MAIN (ghcr.io/falkordb/falkordb-server:edge-rs) is required}"
# The production C engine (third leg). The bundle image's entrypoint starts an in-container Node
# browser server unless BROWSER=0, so synthetic-c-leg.sh hardcodes `--env BROWSER=0` in its
# `docker run` line — the measured container stays server-only.
IMAGE_CENGINE="${IMAGE_CENGINE:-docker.io/falkordb/falkordb:edge}"
ARCH="${ARCH:-x86}"
PR_NUMBER="${PR_NUMBER:-}"   # recorded in run-meta.json (page/comment header)
HEAD_SHA="${HEAD_SHA:-}"     # recorded in run-meta.json (page/comment header)

THRESHOLDS="${CONFIG_DIR}/.github/synthetic-thresholds.toml"
WORKLOAD="${CONFIG_DIR}/.github/synthetic-workload.toml"

# Pinned measurement knobs (kept in step with .github/synthetic-workload.toml's doc comment).
SAMPLES="${SAMPLES:-200}"
WARMUP="${WARMUP:-50}"
# Per-PR default is the CHEAP matrix — every read shape but only concurrency 1 & 8, uncached — so a
# PR check covers all shapes fast. Nightly/on-demand overrides SWEEP/CACHE to the full sweep + both
# cache modes. REPO_READS selects the A/B baseline read shapes (`full` = all ~46; `core` = subset).
SWEEP="${SWEEP:-1,8}"
CACHE="${CACHE:-uncached}"
REPO_READS="${REPO_READS:-full}"
# ONE persistent output dir (survives the $WORKDIR trap) owning every data artifact of the run:
# report-{main-pr,c-pr,c-main}.md, summary-*.json, cells-*.json, run-meta.json and data.json (the
# interactive page's single input). The publish job only copies from here — it never assembles.
SYNTHETIC_OUT="${SYNTHETIC_OUT:-${CONFIG_DIR}/synthetic-out}"
# Host port for the DB container (mapped to the container's 6379). Default 16379 to match the other
# benchmark scripts (run-variant.sh / profile.sh) and avoid colliding with any host Redis on 6379.
DB_PORT="${DB_PORT:-16379}"
DB_CPUS="${DB_CPUS:-$(nproc)}"
DB_MEMORY="${DB_MEMORY:-12g}"
# The uncached sweep at C=32 trips FalkorDB's default queued-query limit; raise it (as the tool's
# own synthetic-verify recipe does) so every image is measured under identical, headroom-y settings.
MAX_QUEUED_QUERIES="${MAX_QUEUED_QUERIES:-1000}"
# Upper bound (seconds) for the WHOLE C leg (pull + measure + two reports), enforced with
# `timeout` so a C-engine hang can never eat the job's margin or the artifact upload. Well below
# the job's timeout-minutes: 90.
C_LEG_TIMEOUT="${C_LEG_TIMEOUT:-2700}"

WORKDIR="$(mktemp -d)"
CONTAINER="synthetic-db-$$"
cleanup() { docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; rm -rf "$WORKDIR"; }
trap cleanup EXIT

bench() { ( cd "$BENCHMARK_DIR" && cargo run --release --quiet --bin benchmark -- "$@" ); }

wait_for_redis() {
  local tries=60
  until docker exec "$CONTAINER" redis-cli PING 2>/dev/null | grep -q PONG; do
    tries=$((tries - 1))
    [ "$tries" -le 0 ] && { echo "::error::synthetic: DB container never became ready" >&2; docker logs "$CONTAINER" 2>&1 | tail -100 || true; return 1; }
    sleep 2
  done
}

# measure <label> <digest> <out.json> — start the image, prep it, run the recorded workload, stop.
measure() {
  local label="$1" digest="$2" out="$3"
  echo "::group::synthetic: measuring ${label} (${digest})"
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
  docker run -d --name "$CONTAINER" --cpus="$DB_CPUS" --memory="$DB_MEMORY" -p "${DB_PORT}:6379" "$digest" >/dev/null
  wait_for_redis
  # Throwaway container: don't let a background save abort the load; give the sweep queue headroom.
  docker exec "$CONTAINER" redis-cli CONFIG SET save "" >/dev/null 2>&1 || true
  docker exec "$CONTAINER" redis-cli CONFIG SET stop-writes-on-bgsave-error no >/dev/null 2>&1 || true
  docker exec "$CONTAINER" redis-cli GRAPH.CONFIG SET MAX_QUEUED_QUERIES "$MAX_QUEUED_QUERIES" >/dev/null
  bench synthetic run --recording "$WORKDIR/rec" \
    --endpoint "falkor://127.0.0.1:${DB_PORT}" \
    --label "$label" --server-image "$digest" \
    --concurrency "$SWEEP" --cache "$CACHE" --samples "$SAMPLES" --warmup "$WARMUP" \
    --out "$out"
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
  echo "::endgroup::"
}

# write_stub_summary <path> <baseline_label> <candidate_label> <slug> <reason> — a v2-shaped
# SyntheticSummary recording WHY a comparison is unavailable, so the comment renderer can show an
# honest not_comparable line instead of silence.
write_stub_summary() {
  local path="$1" baseline="$2" candidate="$3" slug="$4" reason="$5"
  BASELINE="$baseline" CANDIDATE="$candidate" SLUG="$slug" REASON="$reason" python3 - "$path" <<'PY'
import json, os, sys
reason = os.environ["REASON"]
stub = {
    "schema_version": 2,
    "baseline_label": os.environ["BASELINE"],
    "candidate_label": os.environ["CANDIDATE"],
    "slug": os.environ["SLUG"],
    "budget_profile": "cross-engine",
    "divergence_policy": "advisory",
    "gated_metric": "total_ms.p50",
    "elapsed_secs": None,
    "overall_verdict": "not_comparable",
    "headline": f"C-engine leg unavailable — {reason}",
    "not_comparable_reason": reason,
    "comparable_cells": 0,
    "regressed_cells": 0,
    "diverged_ops": [],
    "totals": {"pass": 0, "regressed": 0, "diverged": 0, "not_applicable": 0},
    "per_tier": [],
    "worst_offenders": [],
}
with open(sys.argv[1], "w", encoding="utf-8") as fh:
    json.dump(stub, fh, indent=2)
    fh.write("\n")
PY
}

# Start the wall-clock for the "benchmark + reporting" time shown in the report header (covers the
# digest pulls, the record, all three measurements and the reports — everything in this script).
SYNTHETIC_START_TS="$(date +%s)"
mkdir -p "$SYNTHETIC_OUT"

echo "synthetic: resolving image digests"
PR_DIGEST="$(resolve_digest "$IMAGE_PR")"
MAIN_DIGEST="$(resolve_digest "$IMAGE_MAIN")"

# FalkorDB/benchmark is checked out INSIDE this repo's Cargo workspace; give it its own workspace
# root so `cargo run` there doesn't fail with "believes it's in a workspace when it's not"
# (mirrors run-variant.sh / generate-queries.sh / profile.sh).
grep -q '^\[workspace\]' "$BENCHMARK_DIR/Cargo.toml" || printf '\n[workspace]\n' >> "$BENCHMARK_DIR/Cargo.toml"

# Record the workload once (offline; identical bundle replayed into every image). `--repo-reads`
# records the A/B benchmark's baseline non-algorithm READ shapes (deterministic, record-once →
# replay-verbatim) instead of the legacy catalog ops.
bench synthetic record --config "$WORKLOAD" --repo-reads "$REPO_READS" --out-dir "$WORKDIR/rec"

# Measure the two Rust builds back-to-back (one container at a time). Any failure here is fatal
# (set -e): without the PR and main runs there is nothing worth reporting.
measure "pr" "$PR_DIGEST" "$WORKDIR/pr.json"
measure "main" "$MAIN_DIGEST" "$WORKDIR/main.json"

# Persist EVERY main-pr artifact immediately — before the C leg gets a chance to fail — so the
# gating PR-vs-main signal can never be lost to a C-engine hiccup. Strict budgets + gate policy:
# same-engine, so a result divergence is a correctness failure. (Wall-clock is not passed to the
# per-comparison reports — run-meta.json is the authoritative, single-emission record; see B2.)
echo "::group::synthetic: report main-pr (strict, gate)"
bench synthetic report --diff "$WORKDIR/main.json" "$WORKDIR/pr.json" \
  --regression --thresholds "$THRESHOLDS" \
  --budget-profile strict --divergence-policy gate \
  --out "$SYNTHETIC_OUT/report-main-pr.md" \
  --summary "$SYNTHETIC_OUT/summary-main-pr.json" \
  --cells "$SYNTHETIC_OUT/cells-main-pr.json" >/dev/null
echo "::endgroup::"

# --- C-engine leg: separate child script, explicitly time-bounded, never fatal. -----------------
# A child script (not a function) keeps `set -e` live inside the guarded code — Bash disables
# errexit in any function/compound command tested by `if`, which would silently mask mid-stage
# failures — and `timeout` (which cannot run a shell function) bounds the leg well below the job
# timeout so a hang cannot prevent the artifact upload. The child writes its current stage to
# $WORKDIR/c-stage so a failure is attributed to the stage that broke.
C_STATUS="ok"
C_REASON=""
if ! command -v timeout >/dev/null 2>&1; then
  # Fail closed: without a bound, a hung C engine would eat the job's whole budget and could
  # sink the already-measured main-pr artifacts. CI (Debian) always has coreutils `timeout`;
  # only ad-hoc local runs land here — they get honest stubs instead of an unbounded run.
  C_STATUS="unavailable"
  C_REASON="C leg skipped: no timeout binary available on this runner"
  echo "::warning::synthetic: ${C_REASON} — publishing main-pr only" >&2
elif env \
  BENCHMARK_DIR="$BENCHMARK_DIR" WORKDIR="$WORKDIR" SYNTHETIC_OUT="$SYNTHETIC_OUT" \
  IMAGE_CENGINE="$IMAGE_CENGINE" THRESHOLDS="$THRESHOLDS" \
  DB_PORT="$DB_PORT" DB_CPUS="$DB_CPUS" DB_MEMORY="$DB_MEMORY" \
  MAX_QUEUED_QUERIES="$MAX_QUEUED_QUERIES" \
  SWEEP="$SWEEP" CACHE="$CACHE" SAMPLES="$SAMPLES" WARMUP="$WARMUP" \
  timeout --kill-after=30s "$C_LEG_TIMEOUT" bash "$SCRIPT_DIR/synthetic-c-leg.sh"; then
  echo "synthetic: C-engine leg completed"
else
  c_rc=$?
  C_STATUS="unavailable"
  stage="$(cat "$WORKDIR/c-stage" 2>/dev/null || echo "starting the C leg")"
  if [ "$c_rc" -eq 124 ]; then
    C_REASON="C leg timed out after ${C_LEG_TIMEOUT}s during: ${stage}"
  elif [ "$c_rc" -eq 137 ]; then
    C_REASON="C leg timed out after ${C_LEG_TIMEOUT}s (ignored SIGTERM, killed after 30s grace) during: ${stage}"
  else
    C_REASON="C leg failed (exit ${c_rc}) during: ${stage}"
  fi
  echo "::warning::synthetic: ${C_REASON} — publishing main-pr only" >&2
fi
if [ "$C_STATUS" = "unavailable" ]; then
  # Clean slate: drop any partial C artifacts, then write the two stubs recording WHY. The page
  # and the comment surface the reason; the job stays green (the main-pr signal is intact).
  rm -f "$SYNTHETIC_OUT"/report-c-pr.md "$SYNTHETIC_OUT"/report-c-main.md \
    "$SYNTHETIC_OUT"/cells-c-pr.json "$SYNTHETIC_OUT"/cells-c-main.json \
    "$SYNTHETIC_OUT"/summary-c-pr.json "$SYNTHETIC_OUT"/summary-c-main.json
  write_stub_summary "$SYNTHETIC_OUT/summary-c-pr.json" "c-engine" "pr" \
    "synthetic-pr-vs-c-engine-unavailable" "$C_REASON"
  write_stub_summary "$SYNTHETIC_OUT/summary-c-main.json" "c-engine" "main" \
    "synthetic-main-vs-c-engine-unavailable" "$C_REASON"
fi

# Total wall-clock for everything above (pulls + record + all measurements + reports), recorded
# ONCE in run-meta.json — the single authoritative source the comment and page render from.
ELAPSED_SECS=$(( $(date +%s) - SYNTHETIC_START_TS ))

C_DIGEST="$(cat "$WORKDIR/c-digest" 2>/dev/null || true)"
ELAPSED_SECS="$ELAPSED_SECS" ARCH="$ARCH" PR_NUMBER="$PR_NUMBER" HEAD_SHA="$HEAD_SHA" \
  IMAGE_PR="$IMAGE_PR" PR_DIGEST="$PR_DIGEST" \
  IMAGE_MAIN="$IMAGE_MAIN" MAIN_DIGEST="$MAIN_DIGEST" \
  IMAGE_CENGINE="$IMAGE_CENGINE" C_DIGEST="$C_DIGEST" \
  python3 - "$SYNTHETIC_OUT/run-meta.json" <<'PY'
import json, os, sys
env = os.environ
meta = {
    "schema_version": 1,
    "elapsed_secs": int(env["ELAPSED_SECS"]),
    "arch": env["ARCH"],
    "pr_number": env["PR_NUMBER"] or None,
    "head_sha": env["HEAD_SHA"] or None,
    "images": {
        "pr": {"ref": env["IMAGE_PR"], "digest": env["PR_DIGEST"]},
        "main": {"ref": env["IMAGE_MAIN"], "digest": env["MAIN_DIGEST"]},
        "c-engine": {"ref": env["IMAGE_CENGINE"], "digest": env["C_DIGEST"] or None},
    },
    "comparisons": {
        "main-pr": {"budget_profile": "strict", "divergence_policy": "gate"},
        "c-pr": {"budget_profile": "cross-engine", "divergence_policy": "advisory"},
        "c-main": {"budget_profile": "cross-engine", "divergence_policy": "advisory"},
    },
}
with open(sys.argv[1], "w", encoding="utf-8") as fh:
    json.dump(meta, fh, indent=2)
    fh.write("\n")
PY

# Assemble data.json — the interactive page's single input — from run-meta + the per-comparison
# cells files (ok) or the C failure reason (unavailable). The measure job is the only assembler.
assemble=(python3 "$SCRIPT_DIR/assemble-synthetic-data.py"
  --meta "$SYNTHETIC_OUT/run-meta.json" --out "$SYNTHETIC_OUT/data.json"
  --ok "main-pr=$SYNTHETIC_OUT/cells-main-pr.json")
if [ "$C_STATUS" = "ok" ]; then
  assemble+=(--ok "c-pr=$SYNTHETIC_OUT/cells-c-pr.json" --ok "c-main=$SYNTHETIC_OUT/cells-c-main.json")
else
  assemble+=(--unavailable "c-pr=$C_REASON" --unavailable "c-main=$C_REASON")
fi
"${assemble[@]}"

echo "synthetic: artifacts in $SYNTHETIC_OUT"
ls -l "$SYNTHETIC_OUT"
