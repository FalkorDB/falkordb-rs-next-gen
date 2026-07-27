#!/usr/bin/env bash
# CPU-partition helpers shared by the synthetic measurement legs (sourced by
# synthetic-measure-lib.sh). Source this file — it only defines functions (no side effects).
# Unit-tested via tests/test-cpu-partition.sh.
#
# WHY: every leg runs the measured server container and the closed-loop benchmark client on the
# SAME runner. Without partitioning the client's workers (up to 8 in the per-PR matrix, plus the
# tokio runtime threads) compete with the server threads for the same CPUs, adding run-queue
# latency noise — worst at concurrency > 1, where it blurs the per-op regression verdicts. The
# helpers below split the host CPUs into two disjoint sets: the server container is confined to
# SERVER_CPUS (docker --cpuset-cpus) and the client to CLIENT_CPUS (taskset -c), so neither side
# ever preempts the other.
#
# Policy (pure + deterministic — the host CPU count comes in as an ARGUMENT, only SYNTH_* env is
# read, nothing is probed):
#   1. SYNTH_CPU_PARTITION=0/false/no/off (any case) switches partitioning OFF: both sets are
#      empty, even when explicit sets are also given. Anything else (including unset) means ON —
#      same default-on idiom as REPO_WRITES in synthetic-run.sh.
#   2. Explicit SYNTH_SERVER_CPUS / SYNTH_CLIENT_CPUS are honored VERBATIM when either is
#      non-empty, and the auto-split is fully disabled: each side uses exactly its own value, an
#      empty/unset side stays unpinned. This is the escape hatch for topology-aware sets (e.g.
#      keeping SMT siblings on one side: SYNTH_SERVER_CPUS=0-3,8-11 SYNTH_CLIENT_CPUS=4-7,12-15).
#   3. Otherwise auto-split when <nproc> >= 4: the server takes the FIRST half (rounded up — it
#      does the measured work), the client the rest, e.g. 16 -> 0-7 / 8-15. Equal halves keep
#      both sides off each other's run queues on every runner size, stay deterministic, and emit
#      the plain "a-b" range both docker --cpuset-cpus and taskset -c accept. On SMT hosts the
#      two halves may still share physical cores (sibling enumeration differs per platform) —
#      run-queue competition is gone either way, and rule 2 covers exact topologies. Below 4
#      CPUs both sets stay empty (no pinning): halving would starve the server for less noise
#      win than the isolation buys.

# cpu_partition_off — succeed when SYNTH_CPU_PARTITION explicitly disables partitioning.
# (tr, not ${var,,}: macOS ships bash 3.2 and local runs matter.)
cpu_partition_off() {
  case "$(printf '%s' "${SYNTH_CPU_PARTITION-}" | tr '[:upper:]' '[:lower:]')" in
    0 | false | no | off) return 0 ;;
    *) return 1 ;;
  esac
}

# cpu_partition_explicit — succeed when either explicit set is given (auto-split disabled).
cpu_partition_explicit() {
  [ -n "${SYNTH_SERVER_CPUS-}" ] || [ -n "${SYNTH_CLIENT_CPUS-}" ]
}

# cpu_split_bound <nproc> — echo the first CLIENT cpu index of the auto-split (server gets
# 0..bound-1, i.e. ceil(nproc/2) cpus), or nothing when <nproc> is not a number >= 4.
cpu_split_bound() {
  local n="$1"
  case "$n" in '' | *[!0-9]*) return 0 ;; esac
  [ "$n" -ge 4 ] || return 0
  echo $(((n + 1) / 2))
}

# cpu_partition_server <nproc> — echo the server cpu set ("" = leave unpinned).
cpu_partition_server() {
  local bound
  cpu_partition_off && return 0
  if cpu_partition_explicit; then
    printf '%s' "${SYNTH_SERVER_CPUS-}"
    return 0
  fi
  bound="$(cpu_split_bound "$1")"
  [ -n "$bound" ] && echo "0-$((bound - 1))"
  return 0
}

# cpu_partition_client <nproc> — echo the client cpu set ("" = leave unpinned).
cpu_partition_client() {
  local bound
  cpu_partition_off && return 0
  if cpu_partition_explicit; then
    printf '%s' "${SYNTH_CLIENT_CPUS-}"
    return 0
  fi
  bound="$(cpu_split_bound "$1")"
  [ -n "$bound" ] && echo "${bound}-$(($1 - 1))"
  return 0
}
