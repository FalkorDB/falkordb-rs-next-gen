#!/usr/bin/env bash
# Unit tests for the server/client CPU-partition policy (synthetic-cpu-lib.sh) and its
# source-time wiring in synthetic-measure-lib.sh. Plain bash, no framework — run directly:
#   bash .github/scripts/benchmark/tests/test-cpu-partition.sh
# Exits 0 when every check passes, 1 otherwise (mirrors `synthetic-run.sh --self-test`).
# Wired into CI by .github/workflows/synthetic-report-tests.yml.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BENCH_SCRIPTS="$(cd "$SCRIPT_DIR/.." && pwd)"
# shellcheck source=.github/scripts/benchmark/synthetic-cpu-lib.sh
. "$BENCH_SCRIPTS/synthetic-cpu-lib.sh"

failures=0

# expect_eq <label> <got> <want>
expect_eq() {
  local label="$1" got="$2" want="$3"
  if [ "$got" != "$want" ]; then
    echo "FAIL ${label}: got '${got}', want '${want}'" >&2
    failures=$((failures + 1))
  else
    echo "ok   ${label} -> '${got}'"
  fi
}

# expand_cpuset <set> — expand a cpuset string ("0-3", "8", "0-3,8-11") to one index per line.
# Fails on any token that neither --cpuset-cpus nor taskset -c would accept.
expand_cpuset() {
  local token lo hi i
  local IFS=','
  for token in $1; do
    case "$token" in
      *-*) lo="${token%-*}" hi="${token#*-}" ;;
      *) lo="$token" hi="$token" ;;
    esac
    case "$lo$hi" in '' | *[!0-9]*)
      echo "FAIL expand_cpuset: malformed token '${token}' in '$1'" >&2
      return 1
      ;;
    esac
    for ((i = lo; i <= hi; i++)); do echo "$i"; done
  done
}

# expect_valid_split <nproc> — auto-split invariants for one host size: both sets are plain
# "a-b" ranges, disjoint, within 0..nproc-1, and together cover every cpu exactly once.
expect_valid_split() {
  local n="$1" server client fmt='^[0-9]+-[0-9]+$'
  server="$(cpu_partition_server "$n")"
  client="$(cpu_partition_client "$n")"
  if ! printf '%s' "$server" | grep -Eq "$fmt" || ! printf '%s' "$client" | grep -Eq "$fmt"; then
    echo "FAIL split(nproc=$n): not plain ranges: server='${server}' client='${client}'" >&2
    failures=$((failures + 1))
    return 0
  fi
  local expanded
  expanded="$(
    expand_cpuset "$server"
    expand_cpuset "$client"
  )" || { failures=$((failures + 1)); return 0; }
  local want got
  want="$(for ((i = 0; i < n; i++)); do echo "$i"; done)"
  got="$(printf '%s\n' "$expanded" | sort -n)"
  # Sorted equality to the full 0..n-1 sequence == disjoint + in-range + complete in one shot
  # (an overlap would duplicate an index, an out-of-range or missing index would break the set).
  if [ "$got" != "$want" ]; then
    echo "FAIL split(nproc=$n): server='${server}' client='${client}' does not partition 0..$((n - 1))" >&2
    failures=$((failures + 1))
  else
    echo "ok   split(nproc=$n) -> server '${server}', client '${client}' (disjoint, complete)"
  fi
}

echo "--- auto-split policy"
expect_eq "server(nproc=4)" "$(cpu_partition_server 4)" "0-1"
expect_eq "client(nproc=4)" "$(cpu_partition_client 4)" "2-3"
expect_eq "server(nproc=8)" "$(cpu_partition_server 8)" "0-3"
expect_eq "client(nproc=8)" "$(cpu_partition_client 8)" "4-7"
expect_eq "server(nproc=16)" "$(cpu_partition_server 16)" "0-7"
expect_eq "client(nproc=16)" "$(cpu_partition_client 16)" "8-15"
expect_eq "server(nproc=64)" "$(cpu_partition_server 64)" "0-31"
expect_eq "client(nproc=64)" "$(cpu_partition_client 64)" "32-63"
# Odd count: the server (the measured side) gets the extra cpu.
expect_eq "server(nproc=5)" "$(cpu_partition_server 5)" "0-2"
expect_eq "client(nproc=5)" "$(cpu_partition_client 5)" "3-4"
for n in 4 5 8 16 64; do expect_valid_split "$n"; done

echo "--- no-pinning fallbacks"
for n in 0 1 2 3; do
  expect_eq "server(nproc=$n)" "$(cpu_partition_server "$n")" ""
  expect_eq "client(nproc=$n)" "$(cpu_partition_client "$n")" ""
done
expect_eq "server(nproc='')" "$(cpu_partition_server "")" ""
expect_eq "server(nproc=garbage)" "$(cpu_partition_server "not-a-number")" ""

echo "--- SYNTH_CPU_PARTITION switch"
for off in 0 false NO Off; do
  expect_eq "server(off=$off)" "$(SYNTH_CPU_PARTITION="$off" cpu_partition_server 16)" ""
  expect_eq "client(off=$off)" "$(SYNTH_CPU_PARTITION="$off" cpu_partition_client 16)" ""
done
# Anything else — including unset and empty — means ON (the REPO_WRITES idiom).
expect_eq "server(on=1)" "$(SYNTH_CPU_PARTITION=1 cpu_partition_server 16)" "0-7"
expect_eq "server(on='')" "$(SYNTH_CPU_PARTITION='' cpu_partition_server 16)" "0-7"
# The off switch outranks explicit sets.
expect_eq "server(off wins)" \
  "$(SYNTH_CPU_PARTITION=off SYNTH_SERVER_CPUS=0-3 SYNTH_CLIENT_CPUS=4-7 cpu_partition_server 16)" ""

echo "--- explicit overrides (verbatim, auto-split disabled)"
expect_eq "server(explicit both)" \
  "$(SYNTH_SERVER_CPUS=0-3,8-11 SYNTH_CLIENT_CPUS=4-7,12-15 cpu_partition_server 16)" "0-3,8-11"
expect_eq "client(explicit both)" \
  "$(SYNTH_SERVER_CPUS=0-3,8-11 SYNTH_CLIENT_CPUS=4-7,12-15 cpu_partition_client 16)" "4-7,12-15"
# Explicit sets are honored even below the auto-split's 4-cpu floor.
expect_eq "server(explicit, nproc=2)" "$(SYNTH_SERVER_CPUS=0 cpu_partition_server 2)" "0"
# One explicit side leaves the other UNPINNED — no half-auto mixing that could overlap.
expect_eq "client(server-only)" "$(SYNTH_SERVER_CPUS=0-5 cpu_partition_client 16)" ""
expect_eq "server(client-only)" "$(SYNTH_CLIENT_CPUS=10-15 cpu_partition_server 16)" ""
expect_eq "client(client-only)" "$(SYNTH_CLIENT_CPUS=10-15 cpu_partition_client 16)" "10-15"

echo "--- determinism"
expect_eq "server(repeat)" "$(cpu_partition_server 16)" "$(cpu_partition_server 16)"
expect_eq "client(repeat)" "$(cpu_partition_client 16)" "$(cpu_partition_client 16)"

echo "--- synthetic-measure-lib.sh source-time wiring"
# Sourcing the measure lib must compute SERVER_CPUS/CLIENT_CPUS from SYNTH_NPROC (the env seam)
# and blank CLIENT_CPUS when taskset is missing — assert per the HOST's taskset availability so
# this test passes on both the Linux CI runner and macOS dev machines.
lib_vars() { # lib_vars <env-assignments…> — echo "SERVER|CLIENT" as the lib computed them
  # shellcheck disable=SC2016 # the $-expansions belong to the inner bash, not this shell
  env "$@" bash -c '
    set -euo pipefail
    . "$1" >/dev/null
    printf "%s|%s" "$SERVER_CPUS" "$CLIENT_CPUS"
  ' _ "$BENCH_SCRIPTS/synthetic-measure-lib.sh"
}
if command -v taskset >/dev/null 2>&1; then want_client="8-15"; else want_client=""; fi
expect_eq "measure-lib(auto, nproc=16)" "$(lib_vars SYNTH_NPROC=16)" "0-7|${want_client}"
expect_eq "measure-lib(off)" "$(lib_vars SYNTH_NPROC=16 SYNTH_CPU_PARTITION=off)" "|"
expect_eq "measure-lib(nproc=2)" "$(lib_vars SYNTH_NPROC=2)" "|"

if [ "$failures" -gt 0 ]; then
  echo "test-cpu-partition: ${failures} failure(s)" >&2
  exit 1
fi
echo "test-cpu-partition: all checks passed"
