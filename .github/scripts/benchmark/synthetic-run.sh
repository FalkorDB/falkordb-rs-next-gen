#!/usr/bin/env bash
# Per-PR SYNTHETIC benchmark regression check (see .github/workflows/benchmark.yml).
#
# Records the FalkorDB/benchmark `synthetic` workload ONCE (offline, deterministic), then measures
# each engine image — the PR (`rc-pr-<N>`) and main (`edge-rs`), plus the last release when one
# exists — SEQUENTIALLY on THIS one VM (one container at a time), and diffs each baseline against
# the PR with `report --diff … --regression`, producing a colored 🟢/🔴/N-A Markdown report.
#
# Same-machine, one-at-a-time: runner speed and neighbour noise cancel, and the latency comparison
# is meaningful. Images are pinned to immutable digests up front so a moving tag can't swap a build
# mid-run. The check is informational: this script is strict, but the workflow job is non-blocking
# and always publishes (a failure here surfaces as a "benchmark unavailable" note).
set -euo pipefail

: "${BENCHMARK_DIR:?BENCHMARK_DIR (checkout of FalkorDB/benchmark) is required}"
: "${CONFIG_DIR:?CONFIG_DIR (this repo checkout, for .github/synthetic-*.toml) is required}"
: "${IMAGE_PR:?IMAGE_PR (ghcr.io/falkordb/falkordb-server:rc-pr-<N>) is required}"
: "${IMAGE_MAIN:?IMAGE_MAIN (ghcr.io/falkordb/falkordb-server:edge-rs) is required}"
IMAGE_RELEASE="${IMAGE_RELEASE:-}"          # optional; when empty, only PR-vs-main is reported
RELEASE_LABEL="${RELEASE_LABEL:-release}"   # e.g. "release 1.2.3"
ARCH="${ARCH:-x86}"

THRESHOLDS="${CONFIG_DIR}/.github/synthetic-thresholds.toml"
WORKLOAD="${CONFIG_DIR}/.github/synthetic-workload.toml"
OUT_MD="${OUT_MD:-${CONFIG_DIR}/synthetic-report.md}"

# Pinned measurement knobs (kept in step with .github/synthetic-workload.toml's doc comment).
SAMPLES="${SAMPLES:-200}"
WARMUP="${WARMUP:-50}"
SWEEP="${SWEEP:-1,2,4,8,16,32}"
DB_PORT="${DB_PORT:-6379}"
DB_CPUS="${DB_CPUS:-$(nproc)}"
DB_MEMORY="${DB_MEMORY:-12g}"
# The uncached sweep at C=32 trips FalkorDB's default queued-query limit; raise it (as the tool's
# own synthetic-verify recipe does) so every image is measured under identical, headroom-y settings.
MAX_QUEUED_QUERIES="${MAX_QUEUED_QUERIES:-1000}"

WORKDIR="$(mktemp -d)"
CONTAINER="synthetic-db-$$"
cleanup() { docker rm -f "$CONTAINER" >/dev/null 2>&1 || true; rm -rf "$WORKDIR"; }
trap cleanup EXIT

bench() { ( cd "$BENCHMARK_DIR" && cargo run --release --quiet --bin benchmark -- "$@" ); }

# Resolve a (mutable) tag to an immutable `repo@sha256:…` digest reference. Pull first, then read
# the repo digest from the local image (`docker inspect` — no `buildx` dependency, which the A/B
# GCE image doesn't guarantee). `measure()` reuses the pulled layers by running the digest ref.
resolve_digest() {
  local image="$1"
  local repo="${image%:*}" digest
  docker pull -q "$image" >/dev/null
  digest="$(docker inspect --format '{{range .RepoDigests}}{{println .}}{{end}}' "$image" 2>/dev/null | grep "^${repo}@" | head -1)"
  if [ -z "$digest" ]; then
    echo "::error::synthetic: could not resolve a repo digest for ${image}" >&2
    return 1
  fi
  printf '%s' "$digest"
}

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
    --concurrency "$SWEEP" --cache both --samples "$SAMPLES" --warmup "$WARMUP" \
    --out "$out"
  docker rm -f "$CONTAINER" >/dev/null 2>&1 || true
  echo "::endgroup::"
}

echo "synthetic: resolving image digests"
PR_DIGEST="$(resolve_digest "$IMAGE_PR")"
MAIN_DIGEST="$(resolve_digest "$IMAGE_MAIN")"
RELEASE_DIGEST=""
if [ -n "$IMAGE_RELEASE" ]; then RELEASE_DIGEST="$(resolve_digest "$IMAGE_RELEASE")"; fi

# FalkorDB/benchmark is checked out INSIDE this repo's Cargo workspace; give it its own workspace
# root so `cargo run` there doesn't fail with "believes it's in a workspace when it's not"
# (mirrors run-variant.sh / generate-queries.sh / profile.sh).
grep -q '^\[workspace\]' "$BENCHMARK_DIR/Cargo.toml" || printf '\n[workspace]\n' >> "$BENCHMARK_DIR/Cargo.toml"

# Record the workload once (offline; identical bundle replayed into every image).
bench synthetic record --config "$WORKLOAD" --op all --out-dir "$WORKDIR/rec"

# Measure each build back-to-back (one container at a time).
measure "pr" "$PR_DIGEST" "$WORKDIR/pr.json"
measure "main" "$MAIN_DIGEST" "$WORKDIR/main.json"
[ -n "$RELEASE_DIGEST" ] && measure "$RELEASE_LABEL" "$RELEASE_DIGEST" "$WORKDIR/release.json"

# Diff each baseline against the PR (candidate = pr, the second report). Non-fatal + colored.
bench synthetic report --diff "$WORKDIR/main.json" "$WORKDIR/pr.json" \
  --regression --thresholds "$THRESHOLDS" --out "$WORKDIR/reg-main.md" >/dev/null
if [ -n "$RELEASE_DIGEST" ]; then
  bench synthetic report --diff "$WORKDIR/release.json" "$WORKDIR/pr.json" \
    --regression --thresholds "$THRESHOLDS" --out "$WORKDIR/reg-release.md" >/dev/null
fi

# Assemble the sticky-comment body (arch-specific marker, mirroring the A/B comment).
marker='<!-- synthetic-benchmark -->'
[ "$ARCH" = "arm" ] && marker='<!-- synthetic-benchmark-arm -->'
{
  echo "$marker"
  echo "## 🧪 Synthetic per-op regression — PR vs main${IMAGE_RELEASE:+ / release} (\`$ARCH\`)"
  echo
  echo "Identical recorded workload replayed into each engine image, measured **back-to-back on one runner**, one container at a time. 🟢 faster or within budget · 🔴 slower than budget **or** results differ · N/A no perf verdict. **Non-blocking.**"
  echo
  cat "$WORKDIR/reg-main.md"
  if [ -n "$RELEASE_DIGEST" ]; then echo; echo "---"; echo; cat "$WORKDIR/reg-release.md"; fi
} > "$OUT_MD"

echo "synthetic: wrote report to $OUT_MD"
