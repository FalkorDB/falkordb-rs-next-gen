#!/usr/bin/env python3
"""Assemble `data.json` — the interactive synthetic report page's single input — from
`run-meta.json` plus, per comparison, either a `--cells` analysis file (ok) or an unavailability
reason. Invoked by synthetic-run.sh (the MEASURE job is the only assembler; the publish job just
copies the result next to the page).

    assemble-synthetic-data.py --meta run-meta.json --out data.json \
        --ok main-pr=cells-main-pr.json \
        --unavailable c-pr='C leg failed …' --unavailable c-main='C leg failed …'

Output schema (schema_version 1):

    { "schema_version": 1,
      "meta": { …run-meta.json fields… },
      "comparisons": {
        "main-pr": {"status": "ok", "analysis": { …cells JSON… }},
        "c-pr":    {"status": "unavailable", "reason": "…"},
        "c-main":  {"status": "unavailable", "reason": "…"} } }

Pure stdlib, offline, deterministic. Unknown comparison IDs are rejected (typo guard); the page
renders any SUBSET of the three defensively, so absent comparisons are simply omitted.
"""
from __future__ import annotations

import argparse
import json
import sys

COMPARISON_IDS = ("main-pr", "c-pr", "c-main")
SCHEMA_VERSION = 1


def _split_kv(spec: str, flag: str) -> tuple[str, str]:
    key, sep, value = spec.partition("=")
    if not sep or not key or not value:
        raise SystemExit(f"{flag} expects ID=VALUE, got {spec!r}")
    if key not in COMPARISON_IDS:
        raise SystemExit(f"{flag}: unknown comparison id {key!r} (expected one of {COMPARISON_IDS})")
    return key, value


def build(meta_path: str, ok: list[str], unavailable: list[str]) -> dict:
    with open(meta_path, encoding="utf-8") as fh:
        meta = json.load(fh)
    if not isinstance(meta, dict):
        raise SystemExit(f"--meta {meta_path}: expected a JSON object")

    comparisons: dict[str, dict] = {}
    for spec in ok:
        cid, cells_path = _split_kv(spec, "--ok")
        if cid in comparisons:
            raise SystemExit(f"duplicate comparison id {cid!r}")
        with open(cells_path, encoding="utf-8") as fh:
            analysis = json.load(fh)
        if not isinstance(analysis, dict) or "ops" not in analysis:
            raise SystemExit(f"--ok {cid}: {cells_path} does not look like a --cells analysis file")
        comparisons[cid] = {"status": "ok", "analysis": analysis}
    for spec in unavailable:
        cid, reason = _split_kv(spec, "--unavailable")
        if cid in comparisons:
            raise SystemExit(f"duplicate comparison id {cid!r}")
        comparisons[cid] = {"status": "unavailable", "reason": reason}

    if not comparisons:
        raise SystemExit("at least one --ok or --unavailable comparison is required")
    return {"schema_version": SCHEMA_VERSION, "meta": meta, "comparisons": comparisons}


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--meta", required=True, help="run-meta.json written by synthetic-run.sh")
    ap.add_argument("--out", required=True, help="where to write data.json")
    ap.add_argument(
        "--ok", action="append", default=[], metavar="ID=CELLS_JSON",
        help="comparison ID with its report --cells analysis file (repeatable)",
    )
    ap.add_argument(
        "--unavailable", action="append", default=[], metavar="ID=REASON",
        help="comparison ID that could not be produced, with the human-readable reason (repeatable)",
    )
    args = ap.parse_args(argv)

    data = build(args.meta, args.ok, args.unavailable)
    with open(args.out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1)
        fh.write("\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
