#!/usr/bin/env python3
"""Render the LEAN three-way synthetic-benchmark PR comment from `report --summary` v2 JSONs.

The full Markdown reports are too big for a PR comment (>65 KB), so CI hosts them (plus the
interactive page) on GitHub Pages and posts THIS compact comment instead: one verdict line per
comparison (main-pr / c-pr / c-main), the total wall-clock from run-meta.json, worst offenders
for the gating main-pr comparison only, and a single link to the interactive page. Consumes the
`SyntheticSummary` schema_version 2 or 3 emitted by `benchmark synthetic report --regression …
--summary <file>` (v3 adds the optional ⏭ skipped bucket); any other schema_version
warns-and-skips that line (never mis-renders).

Pure stdlib, offline, deterministic. Never raises on a missing/unreadable summary — it degrades
to an honest "no summary produced" line so the caller can still post a sticky comment (the check
is informational / non-blocking).
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from typing import Any

# v2 is the original three-way summary; v3 (benchmark Phase 6) adds the ⏭ skipped bucket
# (totals.skipped + headline suffix). Tolerant-forward: both accepted, anything else skipped.
SUPPORTED_SCHEMAS = ("2", "3")
VERDICT_EMOJI = {"pass": "🟢", "regressed": "🔴", "advisory": "⚠", "not_comparable": "⚠"}

# Comparison IDs are the design §1 stable identifiers (baseline→candidate). Rendering order and
# human labels are fixed here; the workflow passes whichever summaries the run produced.
COMPARISONS = [
    ("main-pr", "PR vs main"),
    ("c-pr", "PR vs C engine"),
    ("c-main", "main vs C engine"),
]
COMPARISON_IDS = [cid for cid, _ in COMPARISONS]
CROSS_ENGINE_IDS = {"c-pr", "c-main"}


def _safe_int(value: Any, default: int = 0) -> int:
    """int() that never raises — a malformed/None/schema-changed field must not block the comment."""
    try:
        return int(value)
    except (TypeError, ValueError):
        return default


def _load(path: str) -> dict[str, Any] | None:
    try:
        with open(path, encoding="utf-8") as fh:
            data = json.load(fh)
        if not isinstance(data, dict):
            return None
        return data
    except (OSError, ValueError):
        return None


def _fmt_duration(secs: Any) -> str | None:
    try:
        total = int(float(secs))
    except (TypeError, ValueError, OverflowError):
        return None
    if total < 0:
        return None
    h, rem = divmod(total, 3600)
    m, s = divmod(rem, 60)
    if h:
        return f"{h}h {m}m {s}s"
    if m:
        return f"{m}m {s}s"
    return f"{s}s"


def _offenders_line(offenders: list[Any]) -> str:
    parts: list[str] = []
    for off in offenders:
        if not isinstance(off, dict):
            continue
        op = str(off.get("op", "?"))
        if off.get("diverged"):
            parts.append(f"`{op}` (results differ)")
        else:
            n = _safe_int(off.get("regressed_cells", 0))
            cells = "cell" if n == 1 else "cells"
            parts.append(f"`{op}` ({n} {cells} over budget)")
    return ", ".join(parts)


def verdict_line(cid: str, label: str, summary: dict[str, Any] | None) -> str:
    """One comparison's verdict line (or its honest degraded form)."""
    if summary is None:
        return f"⚠ **{label}** — no summary produced for this run (see the workflow run logs)"

    ver = str(summary.get("schema_version", ""))
    if ver not in SUPPORTED_SCHEMAS:
        # Forward/backward-incompatible producer — surface it rather than mis-render.
        return (
            f"⚠ **{label}** — summary schema v{ver or '?'} is not the expected "
            f"v{'/v'.join(SUPPORTED_SCHEMAS)}; skipping (see the full report)"
        )

    verdict = str(summary.get("overall_verdict", "")).lower()
    emoji = VERDICT_EMOJI.get(verdict, "•")
    headline = str(summary.get("headline", "")).strip() or verdict or "no verdict"
    totals = summary.get("totals")

    line = f"{emoji} **{label}** — {headline}"
    if verdict == "not_comparable":
        reason = str(summary.get("not_comparable_reason", "") or "workloads/configs differ")
        if reason not in headline:
            line += f" ({reason})"
    elif cid in CROSS_ENGINE_IDS:
        diverged = _safe_int(totals.get("diverged", 0)) if isinstance(totals, dict) else 0
        # The tool's Advisory headline already carries the count ("pass, 3 diverged — …");
        # only suppress the explicit suffix when a count is really there, so the guarantee
        # holds even if a future headline mentions divergence without quantifying it.
        if diverged and not re.search(r"\b\d+\s+diverged\b", headline):
            ops = "op" if diverged == 1 else "ops"
            line += f" · ⚠ {diverged} {ops} returned different results (advisory)"
    # v3 skipped bucket (absent in v2): surface the count unless the headline already
    # quantifies it — same suppression pattern as the diverged suffix above.
    skipped = _safe_int(totals.get("skipped", 0)) if isinstance(totals, dict) else 0
    if skipped and not re.search(r"\b\d+\s+skipped\b", headline):
        ops = "op" if skipped == 1 else "ops"
        line += f" · ⏭ {skipped} {ops} skipped (capability)"
    return line


def build_comment(
    marker: str,
    arch: str,
    summaries: dict[str, str],
    url: str | None,
    run_meta_path: str | None,
) -> str:
    header_arch = f" (`{arch}`)" if arch else ""
    out: list[str] = [marker]
    out.append(f"## 🧪 Synthetic per-op benchmark{header_arch}")
    out.append("")
    out.append(
        "Identical recorded workload replayed into each engine image, measured **back-to-back on "
        "one runner**. `PR vs main` gates on strict p50 budgets (result divergence fails it); the "
        "C-engine comparisons use looser cross-engine budgets and are **advisory** — divergence "
        "never gates them. **Non-blocking.**"
    )
    out.append("")

    loaded: dict[str, dict[str, Any] | None] = {
        cid: _load(summaries[cid]) if cid in summaries else None for cid in COMPARISON_IDS
    }
    for cid, label in COMPARISONS:
        out.append(verdict_line(cid, label, loaded[cid]))
    out.append("")

    main_pr = loaded.get("main-pr")
    if main_pr is not None and str(main_pr.get("schema_version", "")) in SUPPORTED_SCHEMAS:
        offenders = main_pr.get("worst_offenders") or []
        if isinstance(offenders, list) and offenders:
            out.append(f"**Worst offenders (PR vs main):** {_offenders_line(offenders)}")
            out.append("")

    tail_parts: list[str] = []
    elapsed = None
    if run_meta_path:
        meta = _load(run_meta_path)
        if meta is not None:
            elapsed = _fmt_duration(meta.get("elapsed_secs"))
    if elapsed:
        tail_parts.append(f"⏱ total wall-clock {elapsed}")
    if url:
        tail_parts.append(f"📄 **[Interactive report →]({url})**")
    else:
        tail_parts.append("📄 _report hosting unavailable — see the job log_")
    out.append(" · ".join(tail_parts))

    return "\n".join(out).rstrip() + "\n"


def _parse_summary_arg(value: str) -> tuple[str, str]:
    cid, sep, path = value.partition("=")
    if not sep or not path or cid not in COMPARISON_IDS:
        raise argparse.ArgumentTypeError(
            f"--summary must be <id>=<path> with id one of {', '.join(COMPARISON_IDS)} (got {value!r})"
        )
    return cid, path


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--summary", action="append", default=[], metavar="ID=FILE", type=_parse_summary_arg,
        help="a report --summary v2 JSON, tagged with its comparison id "
             "(main-pr|c-pr|c-main); repeatable",
    )
    ap.add_argument("--run-meta", default="", metavar="FILE",
                    help="run-meta.json with the run's elapsed_secs (optional)")
    ap.add_argument("--url", default="", help="URL of the interactive report page (empty = unavailable)")
    ap.add_argument("--arch", default="", help="arch tag for the header (x86|arm)")
    ap.add_argument(
        "--marker", default="<!-- synthetic-benchmark -->",
        help="sticky-comment HTML marker (use the -arm variant on arm runs)",
    )
    ap.add_argument("--out", default="", help="write to this file instead of stdout")
    args = ap.parse_args(argv)

    summaries: dict[str, str] = {}
    for cid, path in args.summary:
        if cid in summaries:
            ap.error(f"duplicate --summary id {cid!r}")
        summaries[cid] = path

    body = build_comment(args.marker, args.arch, summaries, args.url or None, args.run_meta or None)
    if args.out:
        with open(args.out, "w", encoding="utf-8") as fh:
            fh.write(body)
    else:
        sys.stdout.write(body)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
