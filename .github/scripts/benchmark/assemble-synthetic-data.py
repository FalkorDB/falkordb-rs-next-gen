#!/usr/bin/env python3
"""Assemble `data.json` — the interactive synthetic report page's single input — from
`run-meta.json` plus, per (comparison, kind) slot, either a `--cells` analysis file (ok) or an
unavailability reason. Invoked by synthetic-run.sh after EVERY phase (incremental assembly: a
job that dies mid-run still leaves an honest data.json for the phases that completed); the
publish job just copies the result next to the page.

    assemble-synthetic-data.py --meta run-meta.json --out data.json \
        --ok main-pr/reads=cells-main-pr.json \
        --ok main-pr/writes=cells-main-pr-writes.json \
        --unavailable 'c-pr/reads=C leg failed …' --unavailable 'c-pr/writes=…' \
        --unavailable 'c-main/reads=…' --unavailable 'c-main/writes=…'

Output schema (schema_version 2 — v1 had one un-kinded slot per comparison):

    { "schema_version": 2,
      "meta": { …run-meta.json fields… },
      "comparisons": {
        "main-pr": {
          "reads":  {"status": "ok", "analysis": { …cells JSON… }},
          "writes": {"status": "unavailable", "reason": "…"} },
        "c-pr":  { …same shape… },
        "c-main": { …same shape… } } }

Pure stdlib, offline, deterministic. Unknown comparison IDs / kinds are rejected (typo guard);
an op name appearing in BOTH kinds of one comparison is rejected too — the page's matrix keys
rows by bare op name and resolves each to its kind, so a silent reads/writes collision would
merge two different ops into one row. The page renders any SUBSET of the slots defensively, so
absent comparisons/kinds are simply omitted (e.g. a REPO_WRITES-disabled run has no writes keys).
"""
from __future__ import annotations

import argparse
import json
import sys

COMPARISON_IDS = ("main-pr", "c-pr", "c-main")
KINDS = ("reads", "writes")
SCHEMA_VERSION = 2


def _split_slot(spec: str, flag: str) -> tuple[str, str, str]:
    slot, sep, value = spec.partition("=")
    if not sep or not slot or not value:
        raise SystemExit(f"{flag} expects ID/KIND=VALUE, got {spec!r}")
    cid, sep, kind = slot.partition("/")
    if not sep:
        raise SystemExit(f"{flag}: slot {slot!r} must be ID/KIND (e.g. main-pr/reads)")
    if cid not in COMPARISON_IDS:
        raise SystemExit(f"{flag}: unknown comparison id {cid!r} (expected one of {COMPARISON_IDS})")
    if kind not in KINDS:
        raise SystemExit(f"{flag}: unknown kind {kind!r} (expected one of {KINDS})")
    return cid, kind, value


def build(meta_path: str, ok: list[str], unavailable: list[str]) -> dict:
    with open(meta_path, encoding="utf-8") as fh:
        meta = json.load(fh)
    if not isinstance(meta, dict):
        raise SystemExit(f"--meta {meta_path}: expected a JSON object")

    comparisons: dict[str, dict] = {}

    def claim(cid: str, kind: str) -> dict:
        entry = comparisons.setdefault(cid, {})
        if kind in entry:
            raise SystemExit(f"duplicate slot {cid}/{kind}")
        return entry

    for spec in ok:
        cid, kind, cells_path = _split_slot(spec, "--ok")
        entry = claim(cid, kind)
        with open(cells_path, encoding="utf-8") as fh:
            analysis = json.load(fh)
        if not isinstance(analysis, dict) or "ops" not in analysis:
            raise SystemExit(f"--ok {cid}/{kind}: {cells_path} does not look like a --cells analysis file")
        entry[kind] = {"status": "ok", "analysis": analysis}
    for spec in unavailable:
        cid, kind, reason = _split_slot(spec, "--unavailable")
        entry = claim(cid, kind)
        entry[kind] = {"status": "unavailable", "reason": reason}

    if not comparisons:
        raise SystemExit("at least one --ok or --unavailable slot is required")

    # Reads/writes op names must stay disjoint within a comparison (matrix rows key on bare name).
    for cid, entry in comparisons.items():
        op_sets = [
            set(entry[kind]["analysis"]["ops"])
            for kind in KINDS
            if kind in entry and entry[kind]["status"] == "ok"
            and isinstance(entry[kind]["analysis"].get("ops"), dict)
        ]
        if len(op_sets) == 2:
            clash = sorted(op_sets[0] & op_sets[1])
            if clash:
                raise SystemExit(
                    f"{cid}: op name(s) present in BOTH reads and writes: {', '.join(clash)}"
                )

    return {"schema_version": SCHEMA_VERSION, "meta": meta, "comparisons": comparisons}


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--meta", required=True, help="run-meta.json written by synthetic-run.sh")
    ap.add_argument("--out", required=True, help="where to write data.json")
    ap.add_argument(
        "--ok", action="append", default=[], metavar="ID/KIND=CELLS_JSON",
        help="slot with its report --cells analysis file (repeatable)",
    )
    ap.add_argument(
        "--unavailable", action="append", default=[], metavar="ID/KIND=REASON",
        help="slot that could not be produced, with the human-readable reason (repeatable)",
    )
    args = ap.parse_args(argv)

    data = build(args.meta, args.ok, args.unavailable)
    with open(args.out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1)
        fh.write("\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
