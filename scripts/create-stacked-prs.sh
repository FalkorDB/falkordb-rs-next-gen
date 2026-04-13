#!/usr/bin/env bash
# create-stacked-prs.sh
#
# Creates 8 stacked branches and GitHub pull requests that split the original
# rdb branch (PR #359) into reviewable slices.
#
# Each branch corresponds to one commit on `copilot/split-prs-for-review`.
# The commits are stacked: each one builds on the previous, so the diff
# between adjacent branches is exactly what that PR introduces.
#
# Prerequisites:
#   - gh CLI authenticated: gh auth login
#   - This repo checked out locally
#   - Internet access to github.com
#
# Usage:
#   cd <repo-root>
#   bash scripts/create-stacked-prs.sh
#
set -euo pipefail

REPO="FalkorDB/falkordb-rs-next-gen"
BASE_BRANCH="main"

# ─── Commit SHAs (ordered oldest → newest) ──────────────────────────────────
# These are the commits on copilot/split-prs-for-review that represent each PR.
# Update these if the branch is rebased.
declare -a SHAS=(
  "a935a24"   # PR1: GraphBLAS serialization primitives
  "b9b117b"   # PR2: Value serialization + temporal types
  "6c589c5"   # PR3: Graph/AttributeStore/Indexer serialization
  "2e8fe88"   # PR4: Effects buffer (Pending mutations)
  "40ec48c"   # PR5: Planner non-determinism detection
  "d3ed112"   # PR6: GRAPH.EFFECT, GRAPH.DEBUG commands + config
  "392f10b"   # PR7: RDB encoder/decoder + Redis type integration
  "00d1795"   # PR8: Tests + infrastructure
)

declare -a BRANCH_NAMES=(
  "rdb/pr1-graphblas-serialization"
  "rdb/pr2-value-serialization"
  "rdb/pr3-graph-serialization"
  "rdb/pr4-effects-buffer"
  "rdb/pr5-planner-nondeterminism"
  "rdb/pr6-commands"
  "rdb/pr7-rdb-serializers"
  "rdb/pr8-tests"
)

declare -a PR_TITLES=(
  "feat(rdb): GraphBLAS serialization primitives (Encode/Decode traits)"
  "feat(rdb): Value serialization – Encode/Decode for all Value variants + temporal types"
  "feat(rdb): Graph, AttributeStore, and Indexer serialization"
  "feat(rdb): Effects buffer – Pending mutations serialization for replication"
  "feat(rdb): Planner non-determinism detection and schema version validation"
  "feat(rdb): GRAPH.EFFECT + GRAPH.DEBUG commands, EFFECTS_THRESHOLD config"
  "feat(rdb): RDB encoder/decoder, buffered IO, GRAPHMETA_TYPE virtual key"
  "feat(rdb): Tests, flow tests, and infrastructure updates"
)

declare -a PR_BODIES=(
"Add \`Encode\`/\`Decode\` trait infrastructure and RDB v19 serialize/deserialize support
to the low-level GraphBLAS matrix/vector/tensor types.

**Files changed:**
- \`graph/src/graph/graphblas/serialization.rs\` (new — traits + type-tag constants)
- \`graph/src/graph/graphblas/matrix.rs\` (Encode/Decode for sparse Matrix)
- \`graph/src/graph/graphblas/tensor.rs\` (Encode/Decode for Tensor)
- \`graph/src/graph/graphblas/vector.rs\` (Encode/Decode for Vector)
- \`graph/src/graph/graphblas/versioned_matrix.rs\` (Encode/Decode for VersionedMatrix)
- \`graph/src/graph/graphblas/mod.rs\` (re-export serialization module)

Part of stacked PR series splitting #359. Base: \`main\`."

"Implement \`Encode\`/\`Decode\` for all \`Value\` variants (Bool, Int, Float, String, List,
Point, VecF32, Datetime, Date, Time, Duration) and extend temporal type functions.

**Files changed:**
- \`graph/src/runtime/value.rs\` (Encode/Decode for all Value variants)
- \`graph/src/runtime/functions/temporal.rs\` (temporal type improvements)
- \`graph/src/runtime/functions/{aggregation,conversion,math,mod,procedures}.rs\`

Part of stacked PR series splitting #359. Base: \`rdb/pr1-graphblas-serialization\`."

"Add \`name\` and \`schema_version\` fields to \`Graph\`, expose \`get_database()\`,
implement Encode/Decode for \`AttributeStore\` and \`Graph\`, and add synchronous
index population for RDB restore.

**Files changed:**
- \`graph/src/graph/attribute_store.rs\` (Encode/Decode)
- \`graph/src/graph/graph.rs\` (name, schema_version, get_database, serialization)
- \`graph/src/graph/mvcc_graph.rs\` (serialization pass-through)
- \`graph/src/index/indexer.rs\` (synchronous index population)

Part of stacked PR series splitting #359. Base: \`rdb/pr2-value-serialization\`."

"Encode the full mutation log in \`Pending\` as a binary effects buffer suitable
for replication. Adds schema-baseline tracking, all \`EFFECT_*\` constants, and
helper encode/decode functions.

**Files changed:**
- \`graph/src/runtime/pending.rs\` (build_effects_buffer, EFFECT_* constants, schema baseline)
- \`graph/src/runtime/ops/commit.rs\` (emit effects buffer post-commit)
- \`graph/src/runtime/ops/set.rs\` (schema version bump)
- \`graph/src/runtime/runtime.rs\` (effects threshold wiring)

Part of stacked PR series splitting #359. Base: \`rdb/pr3-graph-serialization\`."

"Add \`plan_is_non_deterministic()\` to detect non-deterministic queries
(so they are not replicated via the effects path). Also adds schema version
validation in the node-by-ID optimizer.

**Files changed:**
- \`graph/src/planner/mod.rs\` (plan_is_non_deterministic + helpers)
- \`graph/src/planner/optimizer/utilize_node_by_id.rs\` (version guard)

Part of stacked PR series splitting #359. Base: \`rdb/pr4-effects-buffer\`."

"New Redis commands and config changes:
- \`GRAPH.EFFECT\` — apply serialized effects buffer on replicas
- \`GRAPH.DEBUG\` — persistence debugging helpers
- Make \`EFFECTS_THRESHOLD\` runtime-settable via \`GRAPH.CONFIG SET\`
- Add optional \`version\` argument to \`GRAPH.QUERY\` for schema version check
- Add \`ThreadedGraph::from_mvcc()\` and \`name()\` helpers

**Files changed:**
- \`src/commands/effect.rs\` (new)
- \`src/commands/debug.rs\` (new)
- \`src/commands/{mod,query,config_cmd}.rs\`, \`src/config.rs\`, \`src/graph_core.rs\`

Part of stacked PR series splitting #359. Base: \`rdb/pr5-planner-nondeterminism\`."

"Main RDB persistence layer:
- Buffered IO layer (\`buffered_io.rs\`) for RDB v19 format
- Encoder (\`encoder/mod.rs\`) — single-key and multi-key payload distribution
- Decoder (\`decoder/mod.rs\`) — load graphs from RDB streams
- \`serializers/mod.rs\` — virtual key state, schema encode/decode
- \`GRAPHMETA_TYPE\` virtual-key Redis type, \`rdb_save\`/\`rdb_load\` hooks
- Subscribe to persistence events for virtual key orchestration

**Files changed:**
- \`src/serializers/{buffered_io,encoder/mod,decoder/mod,mod}.rs\` (new)
- \`src/redis_type.rs\`, \`src/lib.rs\`, \`src/module_init.rs\`

Part of stacked PR series splitting #359. Base: \`rdb/pr6-commands\`."

"Update flow tests for effects replication, persistency, RDB load, and previous-version
RDB decoding. Remove obsolete \`test_prev_rdb_decode.py\`. Update Dockerfiles, flow
scripts, and Python test requirements.

**Files changed:**
- \`tests/flow/{test_effects,test_persistency,test_rdb_load,test_replication,graph_utils}.py\`
- \`tests/flow/dumps/*.dump\` removed (replaced by new RDB v19 format)
- \`tests/requirements.txt\`, \`.devcontainer/Dockerfile\`, \`build/Dockerfile\`
- \`flow.sh\`, \`flow_tests_done.txt\`, \`flow_tests_todo.txt\`, \`Cargo.{toml,lock}\`

Part of stacked PR series splitting #359. Base: \`rdb/pr7-rdb-serializers\`."
)

# ─── Step 1: fetch the split-prs branch so we have all SHAs ─────────────────
echo "Fetching copilot/split-prs-for-review..."
git fetch origin copilot/split-prs-for-review

# ─── Step 2: create branches and push ───────────────────────────────────────
echo ""
echo "Creating branches..."
for i in "${!BRANCH_NAMES[@]}"; do
  branch="${BRANCH_NAMES[$i]}"
  sha="${SHAS[$i]}"
  echo "  $branch @ $sha"
  git branch -f "$branch" "$sha"
  git push origin "$branch"
done

# ─── Step 3: create PRs ─────────────────────────────────────────────────────
echo ""
echo "Creating pull requests..."
for i in "${!BRANCH_NAMES[@]}"; do
  branch="${BRANCH_NAMES[$i]}"
  title="${PR_TITLES[$i]}"
  body="${PR_BODIES[$i]}"

  # Each PR targets the previous PR's branch (stacked), except PR1 targets main
  if [ "$i" -eq 0 ]; then
    base="$BASE_BRANCH"
  else
    base="${BRANCH_NAMES[$((i-1))]}"
  fi

  echo "  Creating PR: $title (base: $base)"
  gh pr create \
    --repo "$REPO" \
    --base "$base" \
    --head "$branch" \
    --title "$title" \
    --body "$body" \
    --draft \
    || echo "  WARNING: PR creation failed for $branch (may already exist)"
done

echo ""
echo "Done! Created ${#BRANCH_NAMES[@]} stacked PRs."
echo ""
echo "Stack order (each PR targets the one above it):"
for i in "${!BRANCH_NAMES[@]}"; do
  echo "  PR$((i+1)): ${BRANCH_NAMES[$i]}"
done
