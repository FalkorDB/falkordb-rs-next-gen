#!/usr/bin/env bash
# Synthetic three-way benchmark for PR CI (reads + writes) — record deterministic bundles once,
# measure them against the PR image, the main image and the production C engine on one runner,
# and report per (comparison, kind):
#   main-pr reads+writes — strict budgets, gate divergence policy (the signal; same engine)
#   c-pr / c-main reads+writes — cross-engine budgets, advisory divergence (context only)
# The reads bundle records every A/B read shape (--repo-reads); the writes bundle records the
# repo's write shapes (--repo-writes) — single-kind by design, latency-tier only (no determinism
# oracle; correctness shows `not_gated`), recorded per-op budgets pin C=1/samples/warmup so the
# same CLI flag set serves both bundles verbatim. See docs/design/synthetic-three-way-writes.md
# in FalkorDB/benchmark. REPO_WRITES=0/false/no/off disables the write legs (slots omitted).
#
# Runs inside the `synthetic` job of _benchmark.yml on a dedicated bare-metal runner. Publishing
# happens in a separate trusted job; this script only fills $SYNTHETIC_OUT.
#
# Failure isolation: the pr/main READS leg is the point of the check — if it fails, the job
# fails (set -e). Everything after that signal is a guarded, time-bounded child script
# (synthetic-writes-leg.sh, synthetic-c-leg.sh, synthetic-c-writes-leg.sh): on failure the
# affected comparisons degrade to honest not_comparable/unavailable stubs and the job stays
# green. run-meta.json + data.json are (re)assembled INCREMENTALLY after every phase, so even a
# hard job timeout leaves the last consistent snapshot for the `if: always()` artifact upload.
# Container names are PARENT-assigned and swept before each leg and on EXIT — `timeout
# --kill-after` SIGKILLs a child without running its EXIT trap, and an orphan holding $DB_PORT
# would cascade-fail every later leg.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# digest_candidates / match_digest / resolve_digest — shared with the C-leg children.
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
# browser server unless BROWSER=0, so the C-leg children hardcode `--env BROWSER=0` in their
# `docker run` lines — the measured container stays server-only.
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
# The write bundle ignores these globals by construction: its recorded per-op budgets pin C=1 and
# their own samples/warmup, so a nightly override changes the READ legs only.
SWEEP="${SWEEP:-1,8}"
CACHE="${CACHE:-uncached}"
REPO_READS="${REPO_READS:-full}"
# Write legs are DEFAULT-ON; only an explicit 0/false/no/off (any case) disables them — anything
# else, including unset and "", means enabled. Deliberately env-only, NOT a workflow input: an
# unset workflow input renders as "" and any ""-is-falsy rule would silently disable the legs.
# (tr, not ${var,,}: macOS ships bash 3.2 and local runs matter.)
REPO_WRITES="${REPO_WRITES-}"
case "$(printf '%s' "$REPO_WRITES" | tr '[:upper:]' '[:lower:]')" in
  0|false|no|off) WRITES_ENABLED=0 ;;
  *) WRITES_ENABLED=1 ;;
esac
echo "synthetic: writes pass: $([ "$WRITES_ENABLED" -eq 1 ] && echo enabled || echo disabled)"
# ONE persistent output dir (survives the $WORKDIR trap) owning every data artifact of the run:
# report-*.md, summary-*.json, cells-*.json, run-meta.json and data.json (the interactive page's
# single input). The publish job only copies from here — it never assembles.
SYNTHETIC_OUT="${SYNTHETIC_OUT:-${CONFIG_DIR}/synthetic-out}"
# Host port for the DB container (mapped to the container's 6379). Default 16379 to match the other
# benchmark scripts (run-variant.sh / profile.sh) and avoid colliding with any host Redis on 6379.
DB_PORT="${DB_PORT:-16379}"
DB_CPUS="${DB_CPUS:-$(nproc)}"
DB_MEMORY="${DB_MEMORY:-12g}"
# CPU partitioning between the measured server container (docker --cpuset-cpus) and the
# closed-loop client (taskset -c) is DEFAULT-ON with >= 4 cpus — the sets are derived inside
# synthetic-measure-lib.sh (identically in this process and every leg child, which inherit the
# SYNTH_* env). SYNTH_CPU_PARTITION=0 disables it; SYNTH_SERVER_CPUS/SYNTH_CLIENT_CPUS pin
# explicit sets. Policy + fallbacks (few cores, no taskset): synthetic-cpu-lib.sh. Every image
# in a run is measured under the SAME partitioning, so the comparisons stay apples-to-apples;
# absolute latencies are not comparable with pre-partitioning reports.
# The uncached sweep at C=32 trips FalkorDB's default queued-query limit; raise it (as the tool's
# own synthetic-verify recipe does) so every image is measured under identical, headroom-y settings.
MAX_QUEUED_QUERIES="${MAX_QUEUED_QUERIES:-1000}"
# Upper bounds (seconds) per guarded child leg, enforced with `timeout` so a hang can never eat
# the job's margin or the artifact upload. Reads C leg: pull + full-matrix measure + two reports.
# Writes legs replay C=1-only (~1 min per engine measured in the design), so 900 is generous.
C_LEG_TIMEOUT="${C_LEG_TIMEOUT:-2700}"
WRITES_LEG_TIMEOUT="${WRITES_LEG_TIMEOUT:-900}"
C_WRITES_LEG_TIMEOUT="${C_WRITES_LEG_TIMEOUT:-900}"

WORKDIR="$(mktemp -d)"
# Parent-assigned container names — one per leg, handed to each child via CONTAINER. Children
# remove their own container on EXIT, but `timeout --kill-after` SIGKILLs a child WITHOUT running
# its trap, so the parent sweeps every name it handed out before each leg and on its own EXIT
# (an orphan would hold $DB_PORT and cascade-fail every later `docker run -p`).
CONTAINER="synthetic-db-$$"
W_CONTAINER="synthetic-writes-$$"
C_CONTAINER="synthetic-cengine-$$"
CW_CONTAINER="synthetic-cwrites-$$"
sweep_containers() {
  docker rm -f "$CONTAINER" "$W_CONTAINER" "$C_CONTAINER" "$CW_CONTAINER" >/dev/null 2>&1 || true
}
cleanup() { sweep_containers; rm -rf "$WORKDIR"; }
trap cleanup EXIT

# bench / wait_for_redis / measure_recording / report_comparison — shared with the leg children
# (the parent's own measurements run in $CONTAINER).
# shellcheck source=.github/scripts/benchmark/synthetic-measure-lib.sh
. "$SCRIPT_DIR/synthetic-measure-lib.sh"

# Every guarded child needs coreutils `timeout` (a child script keeps `set -e` live where an
# if-tested function would disable it, and `timeout` cannot run a shell function). Probe ONCE so
# all legs degrade consistently: without a bound, one hung engine could eat the whole job budget
# and sink the already-measured main-pr artifacts. CI (Debian) always has it; only ad-hoc local
# runs land in the else branch — their legs become honest stubs instead of unbounded runs.
if command -v timeout >/dev/null 2>&1; then
  HAVE_TIMEOUT=1
else
  HAVE_TIMEOUT=0
  echo "::warning::synthetic: no timeout binary on this runner — all guarded legs will be stubbed" >&2
fi

# write_stub_summary <path> <baseline_label> <candidate_label> <slug> <reason> <profile> <policy>
#   <leg_desc> — a v2-shaped SyntheticSummary recording WHY a comparison is unavailable, so the
# comment renderer shows an honest not_comparable line instead of silence. profile/policy must
# echo what the real leg would have used (strict/gate for main-pr writes; cross-engine/advisory
# for C legs); leg_desc opens the headline (e.g. "writes leg unavailable — …").
write_stub_summary() {
  local path="$1" baseline="$2" candidate="$3" slug="$4" reason="$5" profile="$6" policy="$7" leg_desc="$8"
  BASELINE="$baseline" CANDIDATE="$candidate" SLUG="$slug" REASON="$reason" \
  PROFILE="$profile" POLICY="$policy" LEG_DESC="$leg_desc" python3 - "$path" <<'PY'
import json, os, sys
env = os.environ
reason = env["REASON"]
stub = {
    "schema_version": 2,
    "baseline_label": env["BASELINE"],
    "candidate_label": env["CANDIDATE"],
    "slug": env["SLUG"],
    "budget_profile": env["PROFILE"],
    "divergence_policy": env["POLICY"],
    "gated_metric": "total_ms.p50",
    "elapsed_secs": None,
    "overall_verdict": "not_comparable",
    "headline": f"{env['LEG_DESC']} unavailable — {reason}",
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

# ------------------------------------------------------------------------------------------------
# Per-phase slot status + incremental assembly. Status values per guarded phase:
#   pending     — not reached yet → assembled as unavailable ("not reached — …") so a hard job
#                 timeout still leaves an honest data.json snapshot for the phases that DID run
#   ok          — the phase's cells files exist in $SYNTHETIC_OUT
#   unavailable — the phase failed/was skipped; reason in the matching *_REASON
#   disabled    — kind switched off by REPO_WRITES → the slot is OMITTED from data.json entirely
# main-pr/reads has no status variable: it is fatal (set -e), so the first assembly only ever
# happens after it succeeded.
# ------------------------------------------------------------------------------------------------
C_STATUS="pending";  C_REASON=""
if [ "$WRITES_ENABLED" -eq 1 ]; then
  W_STATUS="pending";  W_REASON=""
  CW_STATUS="pending"; CW_REASON=""
else
  W_STATUS="disabled";  W_REASON=""
  CW_STATUS="disabled"; CW_REASON=""
fi
NOT_REACHED="not reached — the run ended before this phase"

# Start the wall-clock for the "benchmark + reporting" time shown in the report header (covers the
# digest pulls, the records, every measurement and report — everything in this script).
SYNTHETIC_START_TS="$(date +%s)"
mkdir -p "$SYNTHETIC_OUT"

# (Re)write run-meta.json and (re)assemble data.json from everything produced SO FAR. Called after
# EVERY phase (cheap, pure-python, offline): the artifact dir always holds the newest consistent
# snapshot even if the job is hard-killed later; the last call of a full run is the final word.
emit_meta_and_assemble() {
  local elapsed c_digest
  elapsed=$(( $(date +%s) - SYNTHETIC_START_TS ))
  c_digest="$(cat "$WORKDIR/c-digest" 2>/dev/null || true)"
  ELAPSED_SECS="$elapsed" ARCH="$ARCH" PR_NUMBER="$PR_NUMBER" HEAD_SHA="$HEAD_SHA" \
    IMAGE_PR="$IMAGE_PR" PR_DIGEST="$PR_DIGEST" \
    IMAGE_MAIN="$IMAGE_MAIN" MAIN_DIGEST="$MAIN_DIGEST" \
    IMAGE_CENGINE="$IMAGE_CENGINE" C_DIGEST="$c_digest" \
    WRITES_ENABLED="$WRITES_ENABLED" \
    python3 - "$SYNTHETIC_OUT/run-meta.json" <<'PY'
import json, os, sys
env = os.environ
comparisons = {
    "main-pr": {"budget_profile": "strict", "divergence_policy": "gate"},
    "c-pr": {"budget_profile": "cross-engine", "divergence_policy": "advisory"},
    "c-main": {"budget_profile": "cross-engine", "divergence_policy": "advisory"},
}
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
    "comparisons": comparisons,
}
with open(sys.argv[1], "w", encoding="utf-8") as fh:
    json.dump(meta, fh, indent=2)
    fh.write("\n")
PY

  local assemble=(python3 "$SCRIPT_DIR/assemble-synthetic-data.py"
    --meta "$SYNTHETIC_OUT/run-meta.json" --out "$SYNTHETIC_OUT/data.json"
    --ok "main-pr/reads=$SYNTHETIC_OUT/cells-main-pr.json")
  case "$C_STATUS" in
    ok) assemble+=(--ok "c-pr/reads=$SYNTHETIC_OUT/cells-c-pr.json"
                   --ok "c-main/reads=$SYNTHETIC_OUT/cells-c-main.json") ;;
    pending) assemble+=(--unavailable "c-pr/reads=$NOT_REACHED"
                        --unavailable "c-main/reads=$NOT_REACHED") ;;
    *) assemble+=(--unavailable "c-pr/reads=$C_REASON"
                  --unavailable "c-main/reads=$C_REASON") ;;
  esac
  case "$W_STATUS" in
    ok)          assemble+=(--ok "main-pr/writes=$SYNTHETIC_OUT/cells-main-pr-writes.json") ;;
    pending)     assemble+=(--unavailable "main-pr/writes=$NOT_REACHED") ;;
    unavailable) assemble+=(--unavailable "main-pr/writes=$W_REASON") ;;
    disabled)    : ;;
  esac
  case "$CW_STATUS" in
    ok) assemble+=(--ok "c-pr/writes=$SYNTHETIC_OUT/cells-c-pr-writes.json"
                   --ok "c-main/writes=$SYNTHETIC_OUT/cells-c-main-writes.json") ;;
    pending) assemble+=(--unavailable "c-pr/writes=$NOT_REACHED"
                        --unavailable "c-main/writes=$NOT_REACHED") ;;
    unavailable) assemble+=(--unavailable "c-pr/writes=$CW_REASON"
                            --unavailable "c-main/writes=$CW_REASON") ;;
    disabled) : ;;
  esac
  "${assemble[@]}"
}

# leg_failure_reason <stage-file> <rc> <leg-name> <timeout-secs> — attribute a guarded child's
# failure to its last-reported stage (mirrors the reason wording the C leg has always used).
leg_failure_reason() {
  local stage_file="$1" rc="$2" leg="$3" budget="$4" stage
  stage="$(cat "$stage_file" 2>/dev/null || echo "starting the $leg")"
  if [ "$rc" -eq 124 ]; then
    echo "$leg timed out after ${budget}s during: ${stage}"
  elif [ "$rc" -eq 137 ]; then
    echo "$leg timed out after ${budget}s (ignored SIGTERM, killed after 30s grace) during: ${stage}"
  else
    echo "$leg failed (exit ${rc}) during: ${stage}"
  fi
}

if [ "$WRITES_ENABLED" -eq 1 ]; then
  echo "synthetic: write legs ENABLED (REPO_WRITES='${REPO_WRITES}')"
else
  echo "synthetic: write legs DISABLED (REPO_WRITES='${REPO_WRITES}')"
fi

echo "synthetic: resolving image digests"
PR_DIGEST="$(resolve_digest "$IMAGE_PR")"
MAIN_DIGEST="$(resolve_digest "$IMAGE_MAIN")"

# FalkorDB/benchmark is checked out INSIDE this repo's Cargo workspace; give it its own workspace
# root so `cargo run` there doesn't fail with "believes it's in a workspace when it's not"
# (mirrors run-variant.sh / generate-queries.sh / profile.sh).
grep -q '^\[workspace\]' "$BENCHMARK_DIR/Cargo.toml" || printf '\n[workspace]\n' >> "$BENCHMARK_DIR/Cargo.toml"

# --- Phase 1 (fatal): the reads signal — record once, measure pr + main, report main-pr. --------
# `--repo-reads` records the A/B benchmark's baseline non-algorithm READ shapes (deterministic,
# record-once → replay-verbatim) instead of the legacy catalog ops. Identical bundle replayed
# into every image. Any failure here is fatal (set -e): without the pr and main runs there is
# nothing worth reporting.
bench synthetic record --config "$WORKLOAD" --repo-reads "$REPO_READS" --out-dir "$WORKDIR/rec"

measure_recording "$WORKDIR/rec" "pr" "$PR_DIGEST" "$WORKDIR/pr.json"
measure_recording "$WORKDIR/rec" "main" "$MAIN_DIGEST" "$WORKDIR/main.json"

# Persist EVERY main-pr artifact immediately — before any guarded leg gets a chance to fail — so
# the gating PR-vs-main signal can never be lost to a later hiccup. Strict budgets + gate policy:
# same-engine, so a result divergence is a correctness failure. (Wall-clock is not passed to the
# per-comparison reports — run-meta.json is the authoritative, single-emission record; see B2.)
report_comparison "$WORKDIR/main.json" "$WORKDIR/pr.json" strict gate "main-pr"

emit_meta_and_assemble

# --- Phase 2 (guarded): main-pr WRITES — record + measure pr/main + report, in one child. --------
# Runs before the C legs: the same-engine gate-policy writes signal outranks the advisory
# cross-engine context, so it gets first claim on the remaining job budget.
if [ "$W_STATUS" = "pending" ]; then
  sweep_containers
  if [ "$HAVE_TIMEOUT" -eq 0 ]; then
    W_STATUS="unavailable"
    W_REASON="writes leg skipped: no timeout binary available on this runner"
    echo "::warning::synthetic: ${W_REASON} — publishing reads only" >&2
  elif env \
    BENCHMARK_DIR="$BENCHMARK_DIR" WORKDIR="$WORKDIR" SYNTHETIC_OUT="$SYNTHETIC_OUT" \
    WORKLOAD="$WORKLOAD" THRESHOLDS="$THRESHOLDS" \
    PR_DIGEST="$PR_DIGEST" MAIN_DIGEST="$MAIN_DIGEST" \
    CONTAINER="$W_CONTAINER" \
    DB_PORT="$DB_PORT" DB_CPUS="$DB_CPUS" DB_MEMORY="$DB_MEMORY" \
    MAX_QUEUED_QUERIES="$MAX_QUEUED_QUERIES" \
    SWEEP="$SWEEP" CACHE="$CACHE" SAMPLES="$SAMPLES" WARMUP="$WARMUP" \
    timeout --kill-after=30s "$WRITES_LEG_TIMEOUT" bash "$SCRIPT_DIR/synthetic-writes-leg.sh"; then
    W_STATUS="ok"
    echo "synthetic: writes leg completed"
  else
    w_rc=$?
    W_STATUS="unavailable"
    W_REASON="$(leg_failure_reason "$WORKDIR/writes-stage" "$w_rc" "writes leg" "$WRITES_LEG_TIMEOUT")"
    echo "::warning::synthetic: ${W_REASON} — publishing reads + a writes stub" >&2
    sweep_containers
  fi
  if [ "$W_STATUS" = "unavailable" ]; then
    # Clean slate: drop any partial writes artifacts, then write the stub recording WHY. The page
    # and the comment surface the reason; the job stays green (the reads signal is intact).
    rm -f "$SYNTHETIC_OUT"/report-main-pr-writes.md \
      "$SYNTHETIC_OUT"/cells-main-pr-writes.json "$SYNTHETIC_OUT"/summary-main-pr-writes.json
    write_stub_summary "$SYNTHETIC_OUT/summary-main-pr-writes.json" "main" "pr" \
      "synthetic-pr-vs-main-writes-unavailable" "$W_REASON" strict gate "writes leg"
  fi
  emit_meta_and_assemble
fi

# --- Phase 3 (guarded): C-engine READS — measure the reads bundle, report c-pr + c-main. ---------
# A child script (not a function) keeps `set -e` live inside the guarded code — Bash disables
# errexit in any function/compound command tested by `if`, which would silently mask mid-stage
# failures — and `timeout` (which cannot run a shell function) bounds the leg well below the job
# timeout so a hang cannot prevent the artifact upload. The child writes its current stage to
# $WORKDIR/c-stage so a failure is attributed to the stage that broke.
sweep_containers
if [ "$HAVE_TIMEOUT" -eq 0 ]; then
  C_STATUS="unavailable"
  C_REASON="C leg skipped: no timeout binary available on this runner"
  echo "::warning::synthetic: ${C_REASON} — publishing without C comparisons" >&2
elif env \
  BENCHMARK_DIR="$BENCHMARK_DIR" WORKDIR="$WORKDIR" SYNTHETIC_OUT="$SYNTHETIC_OUT" \
  IMAGE_CENGINE="$IMAGE_CENGINE" THRESHOLDS="$THRESHOLDS" \
  CONTAINER="$C_CONTAINER" \
  DB_PORT="$DB_PORT" DB_CPUS="$DB_CPUS" DB_MEMORY="$DB_MEMORY" \
  MAX_QUEUED_QUERIES="$MAX_QUEUED_QUERIES" \
  SWEEP="$SWEEP" CACHE="$CACHE" SAMPLES="$SAMPLES" WARMUP="$WARMUP" \
  timeout --kill-after=30s "$C_LEG_TIMEOUT" bash "$SCRIPT_DIR/synthetic-c-leg.sh"; then
  C_STATUS="ok"
  echo "synthetic: C-engine leg completed"
else
  c_rc=$?
  C_STATUS="unavailable"
  C_REASON="$(leg_failure_reason "$WORKDIR/c-stage" "$c_rc" "C leg" "$C_LEG_TIMEOUT")"
  echo "::warning::synthetic: ${C_REASON} — publishing without C reads" >&2
  sweep_containers
fi
if [ "$C_STATUS" = "unavailable" ]; then
  rm -f "$SYNTHETIC_OUT"/report-c-pr.md "$SYNTHETIC_OUT"/report-c-main.md \
    "$SYNTHETIC_OUT"/cells-c-pr.json "$SYNTHETIC_OUT"/cells-c-main.json \
    "$SYNTHETIC_OUT"/summary-c-pr.json "$SYNTHETIC_OUT"/summary-c-main.json
  write_stub_summary "$SYNTHETIC_OUT/summary-c-pr.json" "c-engine" "pr" \
    "synthetic-pr-vs-c-engine-unavailable" "$C_REASON" cross-engine advisory "C-engine leg"
  write_stub_summary "$SYNTHETIC_OUT/summary-c-main.json" "c-engine" "main" \
    "synthetic-main-vs-c-engine-unavailable" "$C_REASON" cross-engine advisory "C-engine leg"
fi
emit_meta_and_assemble

# --- Phase 4 (guarded): C-engine WRITES — needs Phase 2's bundle + measurements. -----------------
# Independent of Phase 3's outcome (a C-reads failure doesn't forfeit C-writes: the child
# re-resolves the digest if $WORKDIR/c-digest is absent, and reuses it — same build for both C
# kinds — when present).
if [ "$CW_STATUS" = "pending" ]; then
  if [ "$W_STATUS" != "ok" ]; then
    CW_STATUS="unavailable"
    CW_REASON="not attempted — the writes leg failed"
  else
    sweep_containers
    if [ "$HAVE_TIMEOUT" -eq 0 ]; then
      CW_STATUS="unavailable"
      CW_REASON="C writes leg skipped: no timeout binary available on this runner"
      echo "::warning::synthetic: ${CW_REASON}" >&2
    elif env \
      BENCHMARK_DIR="$BENCHMARK_DIR" WORKDIR="$WORKDIR" SYNTHETIC_OUT="$SYNTHETIC_OUT" \
      IMAGE_CENGINE="$IMAGE_CENGINE" THRESHOLDS="$THRESHOLDS" \
      CONTAINER="$CW_CONTAINER" \
      DB_PORT="$DB_PORT" DB_CPUS="$DB_CPUS" DB_MEMORY="$DB_MEMORY" \
      MAX_QUEUED_QUERIES="$MAX_QUEUED_QUERIES" \
      SWEEP="$SWEEP" CACHE="$CACHE" SAMPLES="$SAMPLES" WARMUP="$WARMUP" \
      timeout --kill-after=30s "$C_WRITES_LEG_TIMEOUT" bash "$SCRIPT_DIR/synthetic-c-writes-leg.sh"; then
      CW_STATUS="ok"
      echo "synthetic: C-engine writes leg completed"
    else
      cw_rc=$?
      CW_STATUS="unavailable"
      CW_REASON="$(leg_failure_reason "$WORKDIR/c-writes-stage" "$cw_rc" "C writes leg" "$C_WRITES_LEG_TIMEOUT")"
      echo "::warning::synthetic: ${CW_REASON} — publishing without C writes" >&2
      sweep_containers
    fi
  fi
  if [ "$CW_STATUS" = "unavailable" ]; then
    rm -f "$SYNTHETIC_OUT"/report-c-pr-writes.md "$SYNTHETIC_OUT"/report-c-main-writes.md \
      "$SYNTHETIC_OUT"/cells-c-pr-writes.json "$SYNTHETIC_OUT"/cells-c-main-writes.json \
      "$SYNTHETIC_OUT"/summary-c-pr-writes.json "$SYNTHETIC_OUT"/summary-c-main-writes.json
    write_stub_summary "$SYNTHETIC_OUT/summary-c-pr-writes.json" "c-engine" "pr" \
      "synthetic-pr-vs-c-engine-writes-unavailable" "$CW_REASON" cross-engine advisory "C-engine writes leg"
    write_stub_summary "$SYNTHETIC_OUT/summary-c-main-writes.json" "c-engine" "main" \
      "synthetic-main-vs-c-engine-writes-unavailable" "$CW_REASON" cross-engine advisory "C-engine writes leg"
  fi
  emit_meta_and_assemble
fi

echo "synthetic: artifacts in $SYNTHETIC_OUT"
ls -l "$SYNTHETIC_OUT"
