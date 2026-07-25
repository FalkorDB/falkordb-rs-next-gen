#!/usr/bin/env bash
# Publish ONE synthetic-benchmark view into the gh-pages checkout at $GH_PAGES_DIR, under an
# ISOLATED `synthetic-benchmark/` subtree that no other publisher touches.
#
# Why a separate subtree: the A/B `publish.sh` owns all of `benchmark/` and does `rm -rf` +
# `git add -A -- benchmark` on its canonical publish — so anything nested under `benchmark/` would be
# deleted by it. We therefore live in a sibling tree and only ever `rm -rf` OUR OWN leaf:
#
#   synthetic-benchmark/latest/            <- nightly/on-demand canonical (IS_CANONICAL=true)
#   synthetic-benchmark/branch/<view>/     <- per-PR (pr-<N> / pr-<N>-arm) + dispatch views
#
# Each view dir is self-contained and COPY-ONLY — this job never assembles or recomputes anything
# (design §B2). It hosts:
#   index.html      <- the committed static template (.github/scripts/benchmark/synthetic-report.html)
#   data.json       <- assembled by the MEASURE job (assemble-synthetic-data.py); the page fetches it
#   report-*.md     <- the raw per-comparison Markdown reports (main-pr, c-pr, c-main)
#   summary-*.json  <- the lean per-comparison summaries (v2) the PR comment renders from
#   run-meta.json   <- wall-clock / arch / image refs+digests for the run
#
# Reuses gh-pages-push.sh for the (shared) bot identity + token-remote + .nojekyll + rebase-retry
# push. No npm/Next build and no Markdown rendering — static files only, fast + cheap.
set -euo pipefail

: "${REPO:?REPO is required (owner/name)}"
: "${GH_TOKEN:?GH_TOKEN is required (contents:write token)}"
: "${VIEW:?VIEW is required (e.g. pr-123, pr-123-arm, or a dispatch slug)}"
: "${IS_CANONICAL:?IS_CANONICAL is required (true|false)}"
: "${SYNTHETIC_OUT:?SYNTHETIC_OUT is required (dir with data.json, report-*.md, summary-*.json, run-meta.json)}"
: "${GH_PAGES_DIR:?GH_PAGES_DIR (checkout of the gh-pages branch) is required}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE="${TEMPLATE:-$SCRIPT_DIR/synthetic-report.html}"
REPO_NAME="${REPO#*/}"

# Harden the view slug: it becomes a path segment, so refuse anything that could escape the subtree
# (slashes, `..`, absolute, empty). view-slug.sh already emits `[a-z0-9-]+`; PR views are `pr-<N>`.
case "$VIEW" in
  ''|*/*|*..*|.*) echo "::error::synthetic-publish: unsafe VIEW '$VIEW'" >&2; exit 1 ;;
esac
if ! printf '%s' "$VIEW" | grep -qE '^[A-Za-z0-9._-]+$'; then
  echo "::error::synthetic-publish: VIEW '$VIEW' is not a safe slug ([A-Za-z0-9._-]+)" >&2
  exit 1
fi

# data.json is the page's single input — without it there is nothing to publish.
if [ ! -s "$SYNTHETIC_OUT/data.json" ]; then
  echo "::error::synthetic-publish: '$SYNTHETIC_OUT/data.json' is missing or empty — nothing to publish" >&2
  exit 1
fi
if [ ! -s "$TEMPLATE" ]; then
  echo "::error::synthetic-publish: page template '$TEMPLATE' is missing" >&2
  exit 1
fi

if [ "$IS_CANONICAL" = "true" ]; then
  TARGET_REL="synthetic-benchmark/latest"
else
  TARGET_REL="synthetic-benchmark/branch/${VIEW}"
fi
TARGET_DIR="$GH_PAGES_DIR/$TARGET_REL"

echo "::group::Staging synthetic view into ${TARGET_REL} (canonical=${IS_CANONICAL})"
# Only ever remove OUR leaf — never a parent shared with another publisher.
rm -rf "$TARGET_DIR"
mkdir -p "$TARGET_DIR"
cp "$TEMPLATE" "$TARGET_DIR/index.html"
cp "$SYNTHETIC_OUT/data.json" "$TARGET_DIR/data.json"
copied_reports=0
for f in "$SYNTHETIC_OUT"/report-*.md; do
  [ -e "$f" ] || continue
  cp "$f" "$TARGET_DIR/"
  copied_reports=$((copied_reports + 1))
done
if [ "$copied_reports" -eq 0 ]; then
  echo "::warning::synthetic-publish: no report-*.md found in '$SYNTHETIC_OUT'" >&2
fi
for f in "$SYNTHETIC_OUT"/summary-*.json "$SYNTHETIC_OUT/run-meta.json"; do
  [ -e "$f" ] || continue
  cp "$f" "$TARGET_DIR/"
done
echo "::endgroup::"

PUBLISHED_URL="https://$(printf '%s' "${REPO%%/*}" | tr '[:upper:]' '[:lower:]').github.io/${REPO_NAME}/${TARGET_REL}/"

echo "::group::Committing + pushing gh-pages (${TARGET_REL})"
(
  cd "$GH_PAGES_DIR"
  # Scoped add — never `-A`, so we can't stage another publisher's concurrent change.
  git add -- "$TARGET_REL"
  "$SCRIPT_DIR/gh-pages-push.sh" "synthetic: update ${VIEW} report"
)
echo "::endgroup::"

# Surface the URL for the calling workflow (redirect stdout to GITHUB_OUTPUT, or grep this line).
echo "published_url=${PUBLISHED_URL}"
echo "RESULT: published_url=${PUBLISHED_URL}"
