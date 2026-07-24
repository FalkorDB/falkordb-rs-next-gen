#!/usr/bin/env python3
"""Render the LEAN synthetic-regression PR comment from one or more `report --summary` JSON files.

The full ~46-shape Markdown report is too big for a PR comment (>65 KB), so CI hosts it on GitHub
Pages and posts THIS compact comment instead: overall verdict + per-tier 🟢/🔴/N-A counts + worst
offenders + a link to the hosted full report. Consumes the `SyntheticSummary` schema (schema_version
1) emitted by `benchmark synthetic report --diff … --regression --summary <file>`.

Pure stdlib, offline, deterministic. One `--summary` per baseline (e.g. main, release); each renders
its own section and the sticky marker + header are emitted once. Writes the comment body to stdout
(or --out). Never raises on a missing/unreadable summary — it degrades to an honest "unavailable"
line so the caller can still post a sticky comment (the check is informational / non-blocking).
"""
from __future__ import annotations

import argparse
import json
import sys
from typing import Any

SCHEMA_VERSION = 1
VERDICT_EMOJI = {"pass": "🟢", "regressed": "🔴", "not_comparable": "⚠"}


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


def _counts_row(label: str, counts: Any) -> str:
    if not isinstance(counts, dict):
        counts = {}
    p = _safe_int(counts.get("pass", 0))
    r = _safe_int(counts.get("regressed", 0))
    na = _safe_int(counts.get("not_applicable", 0))
    return f"| {label} | {p} | {r} | {na} |"


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


def render_section(summary: dict[str, Any], url: str | None) -> str:
    """Render one baseline's section from a parsed SyntheticSummary dict."""
    ver = str(summary.get("schema_version", ""))
    if str(SCHEMA_VERSION) != ver:
        # Forward/backward-incompatible producer — surface it rather than mis-render.
        return (
            f"> ⚠ summary schema v{ver or '?'} is not the expected v{SCHEMA_VERSION}; "
            "skipping the compact render (see the full report).\n"
        )

    verdict = str(summary.get("verdict", "")).lower()
    emoji = VERDICT_EMOJI.get(verdict, "•")
    headline = str(summary.get("headline", "")).strip()
    base = str(summary.get("baseline_label", "baseline"))
    cand = str(summary.get("candidate_label", "candidate"))

    lines: list[str] = []
    lines.append(f"### {cand} vs {base}")
    lines.append("")
    lines.append(f"{emoji} **{headline}**" if headline else f"{emoji} **{verdict or 'no verdict'}**")
    lines.append("")

    if verdict == "not_comparable":
        reason = str(summary.get("not_comparable_reason", "") or "workloads/configs differ")
        lines.append(f"> Not comparable: {reason}")
        lines.append("")

    per_tier = summary.get("per_tier") or []
    if isinstance(per_tier, list) and per_tier:
        lines.append("| tier | 🟢 | 🔴 | N/A |")
        lines.append("| --- | --- | --- | --- |")
        for t in per_tier:
            if isinstance(t, dict):
                lines.append(_counts_row(str(t.get("tier", "?")), t.get("counts") or {}))
        totals = summary.get("totals") or {}
        lines.append(_counts_row("**all**", totals))
        lines.append("")

    offenders = summary.get("worst_offenders") or []
    if isinstance(offenders, list) and offenders:
        lines.append(f"**Worst offenders:** {_offenders_line(offenders)}")
        lines.append("")

    comparable = _safe_int(summary.get("comparable_cells", 0))
    tail = f"{comparable} comparable p50 cell(s)"
    if url:
        lines.append(f"📄 **[Full report →]({url})** · {tail}")
    else:
        lines.append(f"📄 _full report hosting unavailable — see the job log_ · {tail}")
    lines.append("")
    return "\n".join(lines)


def build_comment(marker: str, arch: str, summaries: list[str], url: str | None) -> str:
    header_arch = f" (`{arch}`)" if arch else ""
    out: list[str] = [marker]
    out.append(f"## 🧪 Synthetic per-op regression{header_arch}")
    out.append("")
    out.append(
        "Identical recorded workload replayed into each engine image, measured **back-to-back on "
        "one runner**. 🟢 within budget · 🔴 slower than budget **or** results differ · N/A no perf "
        "verdict. **Non-blocking.**"
    )
    out.append("")

    rendered_any = False
    for path in summaries:
        data = _load(path)
        if data is None:
            continue
        out.append(render_section(data, url))
        rendered_any = True

    if not rendered_any:
        out.append(
            "⚠ No summary was produced for this run (build/measurement error, stock-out, or no PR "
            "image). See the `synthetic` job logs. Informational; never blocks the PR."
        )
        out.append("")

    return "\n".join(out).rstrip() + "\n"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--summary", action="append", default=[], metavar="FILE",
        help="a report --summary JSON file (repeat per baseline: main, release)",
    )
    ap.add_argument("--url", default="", help="URL of the hosted full report (empty = unavailable)")
    ap.add_argument("--arch", default="", help="arch tag for the header (x86|arm)")
    ap.add_argument(
        "--marker", default="<!-- synthetic-benchmark -->",
        help="sticky-comment HTML marker (use the -arm variant on arm runs)",
    )
    ap.add_argument("--out", default="", help="write to this file instead of stdout")
    args = ap.parse_args(argv)

    body = build_comment(args.marker, args.arch, args.summary, args.url or None)
    if args.out:
        with open(args.out, "w", encoding="utf-8") as fh:
            fh.write(body)
    else:
        sys.stdout.write(body)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
