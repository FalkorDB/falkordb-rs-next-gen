#!/bin/bash
# Run the numeric-index benchmark against one module build.
#
# Starts a throwaway redis-server with the given .so, waits for it, runs the
# Python harness (passing through any extra args), then shuts it down.
#
# Usage:
#   SO=target/release/libfalkordb.dylib LABEL=native PORT=7901 \
#     ./bench/run_bench.sh --nodes 500000 --edges-per-node 3 --graph g
#
# Run once per build (native feature-on vs feature-off) and diff the JSON.
set -euo pipefail

SO="${SO:?set SO=path/to/libfalkordb.dylib}"
PORT="${PORT:-7901}"
LABEL="${LABEL:-bench}"
HERE="$(cd "$(dirname "$0")" && pwd)"
LOG="/tmp/redis-bench-${LABEL}.log"

redis-server --port "$PORT" --loadmodule "$SO" --save '' --maxmemory 0 \
    --appendonly no &> "$LOG" &
RPID=$!
cleanup() { redis-cli -p "$PORT" shutdown nosave 2>/dev/null || true; kill "$RPID" 2>/dev/null || true; }
trap cleanup EXIT

# Wait for the server to accept connections (module load can take a moment).
for _ in $(seq 1 80); do
    if redis-cli -p "$PORT" ping 2>/dev/null | grep -q PONG; then break; fi
    sleep 0.5
done
if ! redis-cli -p "$PORT" ping 2>/dev/null | grep -q PONG; then
    echo "redis did not come up; see $LOG" >&2
    tail -20 "$LOG" >&2
    exit 1
fi

python3 "$HERE/numeric_index_bench.py" --port "$PORT" --label "$LABEL" "$@"
