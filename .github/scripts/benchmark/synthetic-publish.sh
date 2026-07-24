#!/usr/bin/env bash
# Publish ONE synthetic-benchmark report into the gh-pages checkout at $GH_PAGES_DIR, under an
# ISOLATED `synthetic-benchmark/` subtree that no other publisher touches.
#
# Why a separate subtree: the A/B `publish.sh` owns all of `benchmark/` and does `rm -rf` +
# `git add -A -- benchmark` on its canonical publish — so anything nested under `benchmark/` would be
# deleted by it. We therefore live in a sibling tree and only ever `rm -rf` OUR OWN leaf:
#
#   synthetic-benchmark/latest/            <- nightly/on-demand canonical (IS_CANONICAL=true)
#   synthetic-benchmark/branch/<view>/     <- per-PR (pr-<N> / pr-<N>-arm) + dispatch views
#
# Each view dir is self-contained: report.md (raw), summary-*.json (the lean summaries), and
# index.html (a pre-rendered, dependency-free view of report.md). The lean PR comment links here.
#
# Reuses gh-pages-push.sh for the (shared) bot identity + token-remote + .nojekyll + rebase-retry
# push. Unlike publish.sh there is NO npm/Next build — we host Markdown, so this is fast + cheap.
set -euo pipefail

: "${REPO:?REPO is required (owner/name)}"
: "${GH_TOKEN:?GH_TOKEN is required (contents:write token)}"
: "${VIEW:?VIEW is required (e.g. pr-123, pr-123-arm, or a dispatch slug)}"
: "${IS_CANONICAL:?IS_CANONICAL is required (true|false)}"
: "${REPORT_MD:?REPORT_MD is required (path to the full Markdown report)}"
: "${GH_PAGES_DIR:?GH_PAGES_DIR (checkout of the gh-pages branch) is required}"
SUMMARY_DIR="${SUMMARY_DIR:-}"                 # optional dir holding summary-*.json
TITLE="${TITLE:-Synthetic benchmark report — ${VIEW}}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
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

if [ ! -s "$REPORT_MD" ]; then
  echo "::error::synthetic-publish: REPORT_MD '$REPORT_MD' is missing or empty — nothing to publish" >&2
  exit 1
fi

if [ "$IS_CANONICAL" = "true" ]; then
  TARGET_REL="synthetic-benchmark/latest"
else
  TARGET_REL="synthetic-benchmark/branch/${VIEW}"
fi
TARGET_DIR="$GH_PAGES_DIR/$TARGET_REL"

echo "::group::Staging synthetic report into ${TARGET_REL} (canonical=${IS_CANONICAL})"
# Only ever remove OUR leaf — never a parent shared with another publisher.
rm -rf "$TARGET_DIR"
mkdir -p "$TARGET_DIR"
cp "$REPORT_MD" "$TARGET_DIR/report.md"
if [ -n "$SUMMARY_DIR" ] && ls "$SUMMARY_DIR"/summary-*.json >/dev/null 2>&1; then
  cp "$SUMMARY_DIR"/summary-*.json "$TARGET_DIR/"
fi
python3 "$SCRIPT_DIR/render-report-html.py" \
  --in "$TARGET_DIR/report.md" --out "$TARGET_DIR/index.html" --title "$TITLE"
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
