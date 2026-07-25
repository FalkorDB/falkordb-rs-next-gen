#!/usr/bin/env bash
# Digest-resolution helpers shared by synthetic-run.sh and synthetic-c-leg.sh. Source this file —
# it only defines functions (no side effects). Self-tested via `synthetic-run.sh --self-test`.
#
# `docker inspect .RepoDigests` stores NORMALIZED repo names: `docker.io/` is stripped and official
# single-name images lose their `library/` namespace (`docker.io/falkordb/falkordb:edge` pulls as
# `falkordb/falkordb@sha256:…`, `docker.io/library/alpine` as `alpine@sha256:…`). A literal match
# on the ref as-given therefore fails for every Docker Hub spelling (the G5 bug this fixes).

# digest_candidates <image> — echo the repo names to try against RepoDigests, one per line,
# most-specific first: the repo as-given (GHCR / port-qualified registries match here), the
# `docker.io/`-stripped form, and the `library/` add/strip variants for official single-name images.
digest_candidates() {
  local image="$1" repo hubrepo
  # Strip a trailing :tag, but only when the ':' is in the LAST path segment — otherwise a registry
  # port (e.g. localhost:5000/repo with no tag) would be mangled to the host.
  case "${image##*/}" in
    *:*) repo="${image%:*}" ;;
    *) repo="$image" ;;
  esac
  printf '%s\n' "$repo"
  case "$repo" in
    docker.io/*)
      hubrepo="${repo#docker.io/}"
      printf '%s\n' "$hubrepo"
      case "$hubrepo" in
        library/*) printf '%s\n' "${hubrepo#library/}" ;;
        */*) : ;;
        *) printf 'library/%s\n' "$hubrepo" ;;
      esac
      ;;
    */*) : ;;
    *)
      # Bare official image (`alpine:3.20`): older daemons/mirrors may keep the namespace.
      printf 'library/%s\n' "$repo"
      ;;
  esac
}

# match_digest <newline-separated RepoDigests> <candidate>... — print the first RepoDigests entry
# whose repo equals a candidate (literal, non-regex prefix match on `candidate@`).
match_digest() {
  local digests="$1"; shift
  local cand hit
  for cand in "$@"; do
    hit="$(printf '%s\n' "$digests" | awk -v r="${cand}@" 'index($0, r) == 1 { print; exit }')"
    if [ -n "$hit" ]; then printf '%s' "$hit"; return 0; fi
  done
  return 1
}

# resolve_digest <image> — resolve a (mutable) tag to an immutable `repo@sha256:…` digest
# reference. Pull first, then read the repo digest from the local image (`docker inspect` — no
# `buildx` dependency, which the A/B GCE image doesn't guarantee). Callers reuse the pulled layers
# by running the digest ref. Pre-pinned `@sha256:` refs pass through verbatim without any pull.
resolve_digest() {
  local image="$1"
  case "$image" in *@sha256:*) printf '%s' "$image"; return 0 ;; esac
  docker pull -q "$image" >/dev/null
  local digests
  digests="$(docker inspect --format '{{range .RepoDigests}}{{println .}}{{end}}' "$image" 2>/dev/null)" || true
  local -a cands=()
  local line
  while IFS= read -r line; do [ -n "$line" ] && cands+=("$line"); done < <(digest_candidates "$image")
  local digest
  if ! digest="$(match_digest "${digests:-}" "${cands[@]}")"; then
    echo "::error::synthetic: could not resolve a repo digest for ${image} (tried: ${cands[*]})" >&2
    return 1
  fi
  printf '%s' "$digest"
}
