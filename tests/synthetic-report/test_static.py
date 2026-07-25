"""Static (no-browser) checks for the synthetic report page and its data flow.

Guards two contracts:
1. the committed template never uses HTML-string injection sinks — all
   rendering must go through createElement/textContent so op names, image
   refs and reasons stay inert text;
2. the committed data.json fixtures match the schema the page consumes
   (the shape assemble-synthetic-data.py emits), covering every status the
   page must handle: ok, ok-with-diverged-advisory-op, and unavailable.
"""

import json
import re
import subprocess
import sys

import pytest

from support import FIXTURES, SCRIPTS_DIR, TEMPLATE, load_fixture

ASSEMBLER = SCRIPTS_DIR / "assemble-synthetic-data.py"

COMPARISON_IDS = {"main-pr", "c-pr", "c-main"}
OP_OUTCOMES = {"pass", "regressed", "diverged_advisory", "not_applicable", "skipped"}
CORRECTNESS = {"match", "diverged", "not_gated"}
PERF_VERDICTS = {"pass", "regressed", "not_applicable"}
STATUS_KINDS = {"comparable", "workload_mismatch", "no_common_ops"}
CACHE_MODES = {"cached", "uncached"}


# --- 1. template forbids HTML-string injection sinks -----------------------

FORBIDDEN_SINKS = [
    "innerHTML",
    "outerHTML",
    "insertAdjacentHTML",
    "document.write",
    "DOMParser",
    "createContextualFragment",
]


@pytest.mark.parametrize("sink", FORBIDDEN_SINKS)
def test_template_has_no_html_injection_sink(sink):
    text = TEMPLATE.read_text(encoding="utf-8")
    assert sink not in text, (
        f"synthetic-report.html must not use {sink}; render via "
        "createElement/textContent only"
    )


def test_template_is_single_file_no_external_resources():
    """One dependency-free file: no external scripts, styles, or imports.

    Regex-based so quoting/whitespace/attribute-order variants can't slip
    through; data: URIs (the inline favicon) stay allowed.
    """
    text = TEMPLATE.read_text(encoding="utf-8")
    patterns = (
        r"<script[^>]*\bsrc\s*=",                        # any external script
        r"<link[^>]*\bhref\s*=(?!\s*[\"']?data:)",       # any link href that is not a data: URI
        r"@import\b",                                    # CSS imports
        r"\bimport\s*\(",                                # dynamic JS import
        r"url\((?!\s*[\"']?(?:data:|#))",                # any CSS url() that is not data:/fragment
    )
    for pattern in patterns:
        match = re.search(pattern, text, re.IGNORECASE)
        assert match is None, (
            f"template must stay dependency-free (pattern {pattern!r} "
            f"matched {match.group(0)!r})"
        )


def test_template_fetches_sibling_data_json():
    text = TEMPLATE.read_text(encoding="utf-8")
    assert "fetch('data.json'" in text or 'fetch("data.json"' in text


def test_template_has_inline_data_uri_favicon():
    """Favicon must be an inline data: URI — zero extra network fetches (B3)."""
    text = TEMPLATE.read_text(encoding="utf-8")
    assert '<link rel="icon" href="data:image/svg+xml,' in text
    icon_hrefs = re.findall(r'<link rel="(?:icon|apple-touch-icon)" href="([^"]+)"', text)
    assert icon_hrefs, "icon links missing"
    for href in icon_hrefs:
        assert href.startswith("data:image/svg+xml,"), href


# --- 2. fixture schema: what the page consumes ------------------------------


def _assert_analysis_shape(analysis):
    # cells v1 (benchmark v2.3) or v2 (Phase 6: adds op_outcome "skipped",
    # totals.skipped, optional per-op skipped_baseline/skipped_candidate).
    assert analysis["schema_version"] in (1, 2)
    for key in ("comparison", "meta", "budget_profile", "divergence_policy",
                "gated_metric", "status", "verdict", "totals", "ops"):
        assert key in analysis, f"cells JSON missing {key}"
    assert analysis["budget_profile"] in {"strict", "cross-engine"}
    assert analysis["divergence_policy"] in {"gate", "advisory"}
    assert analysis["verdict"] in {"pass", "regressed", "advisory", "not_comparable"}
    base_totals = {"pass", "regressed", "diverged", "not_applicable"}
    assert base_totals <= set(analysis["totals"]) <= base_totals | {"skipped"}
    assert isinstance(analysis["status"], dict)
    assert analysis["status"].get("kind") in STATUS_KINDS
    for op, entry in analysis["ops"].items():
        assert entry["correctness"] in CORRECTNESS, op
        assert entry["op_outcome"] in OP_OUTCOMES, op
        # v2 optional per-side skip markers — validate type only when present.
        for key in ("skipped_baseline", "skipped_candidate"):
            if key in entry:
                assert isinstance(entry[key], bool), f"{op} {key}"
        # cells may be EMPTY: a cell-less diverged op is legal (it still counts in totals).
        assert isinstance(entry["cells"], list), op
        for cell in entry["cells"]:
            assert isinstance(cell["concurrency"], int)
            assert cell["cache_mode"] in CACHE_MODES
            assert cell["perf_verdict"] in PERF_VERDICTS
            assert "budget" in cell, f"{op} cell missing budget"
            # p50/delta fields are Option + skip_serializing_if in the producer:
            # OMITTED (not null) when a side lacks a p50 — validate only when present.
            for key in ("baseline_p50_ms", "candidate_p50_ms", "delta_pct", "delta_ms"):
                if key in cell:
                    assert isinstance(cell[key], (int, float)), f"{op} {key}"
            # context is ALWAYS serialized as an object; only its per-side entries
            # (baseline / candidate) are omit-when-absent.
            assert isinstance(cell.get("context"), dict), f"{op} cell missing context"
            for side in ("baseline", "candidate"):
                if side in cell["context"]:
                    assert isinstance(cell["context"][side], dict), f"{op} context.{side}"


def _assert_data_shape(data):
    assert data["schema_version"] == 1
    meta = data["meta"]
    for key in ("elapsed_secs", "arch", "images", "comparisons"):
        assert key in meta, f"run-meta missing {key}"
    assert set(meta["images"]) == {"pr", "main", "c-engine"}
    for side in meta["images"].values():
        assert "ref" in side and "digest" in side
    comparisons = data["comparisons"]
    assert set(comparisons) <= COMPARISON_IDS
    assert comparisons, "at least one comparison required"
    for cid, entry in comparisons.items():
        if entry["status"] == "ok":
            _assert_analysis_shape(entry["analysis"])
        elif entry["status"] == "unavailable":
            assert entry["reason"], f"{cid} unavailable without reason"
        else:
            raise AssertionError(f"{cid}: unknown status {entry['status']!r}")


@pytest.mark.parametrize(
    "fixture",
    ["data.json", "data-xss.json", "data-not-comparable.json", "data-cache-modes.json",
     "data-v2-skipped.json"],
)
def test_fixture_matches_page_schema(fixture):
    _assert_data_shape(load_fixture(fixture))


def test_main_fixture_covers_all_statuses():
    """data.json must keep exercising every status the page renders."""
    data = load_fixture("data.json")
    statuses = {c["status"] for c in data["comparisons"].values()}
    assert statuses == {"ok", "unavailable"}
    outcomes = {
        entry["op_outcome"]
        for c in data["comparisons"].values()
        if c["status"] == "ok"
        for entry in c["analysis"]["ops"].values()
    }
    assert "diverged_advisory" in outcomes, (
        "fixture must include a diverged_advisory op so the page's advisory "
        "path stays covered"
    )
    # Contract edge cases the page must keep rendering: a cell-less diverged op
    # (legal — counts in totals), an empty context object, a one-sided context
    # and a one-sided cell whose candidate p50/deltas are omitted entirely.
    ok_ops = [
        entry
        for c in data["comparisons"].values()
        if c["status"] == "ok"
        for entry in c["analysis"]["ops"].values()
    ]
    assert any(
        entry["correctness"] == "diverged" and entry["cells"] == []
        for entry in ok_ops
    ), "fixture must include a cell-less diverged op"
    all_cells = [cell for entry in ok_ops for cell in entry["cells"]]
    assert any(cell["context"] == {} for cell in all_cells), (
        "fixture must include a cell with an empty context object"
    )
    assert any(
        len(cell["context"]) == 1 for cell in all_cells
    ), "fixture must include a one-sided context"
    assert any(
        "candidate_p50_ms" not in cell and "delta_pct" not in cell
        and cell["perf_verdict"] == "not_applicable"
        for cell in all_cells
    ), "fixture must include a one-sided cell with omitted p50/delta fields"


def test_xss_fixture_has_script_shaped_labels():
    raw = json.dumps(load_fixture("data-xss.json"))
    assert "<script>" in raw and "onerror=" in raw


# --- 3. assembler CLI behavior ----------------------------------------------


def run_assembler(*args):
    return subprocess.run(
        [sys.executable, str(ASSEMBLER), *args],
        capture_output=True, text=True,
    )


def test_assembler_mixed_ok_and_unavailable(tmp_path):
    # Use a real cells fixture so the assembled document passes the full
    # page-schema check.
    data = load_fixture("data.json")
    cells = tmp_path / "cells.json"
    cells.write_text(json.dumps(data["comparisons"]["main-pr"]["analysis"]))
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    out = tmp_path / "data.json"
    proc = run_assembler(
        "--meta", str(meta), "--out", str(out),
        "--ok", f"main-pr={cells}",
        "--unavailable", "c-pr=C leg failed (exit 1)",
        "--unavailable", "c-main=C leg failed (exit 1)",
    )
    assert proc.returncode == 0, proc.stderr
    data = json.loads(out.read_text())
    assert data["comparisons"]["main-pr"]["status"] == "ok"
    assert data["comparisons"]["c-pr"] == {
        "status": "unavailable", "reason": "C leg failed (exit 1)"}
    _assert_data_shape(data)


def test_assembler_rejects_unknown_comparison_id(tmp_path):
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--unavailable", "nope=broken",
    )
    assert proc.returncode != 0
    assert "nope" in proc.stderr


def test_assembler_rejects_duplicate_comparison_id(tmp_path):
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--unavailable", "c-pr=a", "--unavailable", "c-pr=b",
    )
    assert proc.returncode != 0
    assert "duplicate" in proc.stderr.lower()


def test_assembler_rejects_cells_without_ops(tmp_path):
    bad = tmp_path / "bad.json"
    bad.write_text(json.dumps({"schema_version": 1}))
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--ok", f"main-pr={bad}",
    )
    assert proc.returncode != 0


def test_assembler_requires_at_least_one_comparison(tmp_path):
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler("--meta", str(meta), "--out", str(tmp_path / "d.json"))
    assert proc.returncode != 0
