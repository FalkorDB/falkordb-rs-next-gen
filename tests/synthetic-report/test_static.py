"""Static (no-browser) checks for the synthetic report page and its data flow.

Guards two contracts:
1. the committed template never uses HTML-string injection sinks — all
   rendering must go through createElement/textContent so op names, image
   refs and reasons stay inert text;
2. the committed data.json fixtures match the schema the page consumes
   (the shape assemble-synthetic-data.py emits), covering every status the
   page must handle: ok, ok-with-diverged-advisory-op, unavailable, and the
   v2 (comparison, kind) slot model with reads and writes.
"""

import json
import re
import subprocess
import sys

import pytest

from support import FIXTURES, SCRIPTS_DIR, TEMPLATE, load_fixture

ASSEMBLER = SCRIPTS_DIR / "assemble-synthetic-data.py"

COMPARISON_IDS = {"main-pr", "c-pr", "c-main"}
SLOT_KINDS = {"reads", "writes"}
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
    # Formatting-resilient: any quoting, attribute order, or extra attributes.
    icon_links = re.findall(
        r"<link\b[^>]*\brel\s*=\s*[\"']?(?:icon|apple-touch-icon)[\"']?[^>]*>",
        text, re.IGNORECASE)
    assert icon_links, "icon links missing"
    for link in icon_links:
        href = re.search(r"\bhref\s*=\s*[\"']?([^\"'\s>]+)", link, re.IGNORECASE)
        assert href, link
        assert href.group(1).startswith("data:image/svg+xml,"), link


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
        # Optional per-op example query (benchmark ≥ v2.7) — a non-empty string when present.
        if "example_query" in entry:
            assert isinstance(entry["example_query"], str) and entry["example_query"], (
                f"{op} example_query must be a non-empty string"
            )
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
                    # Optional within-run dispersion stats of server_ms (benchmark ≥
                    # v2.7): the sample count is a non-negative int, σ/CV are numbers
                    # — when present.
                    stats = cell["context"][side]
                    if "server_n" in stats:
                        assert isinstance(stats["server_n"], int) and stats["server_n"] >= 0, (
                            f"{op} context.{side}.server_n"
                        )
                    for key in ("server_stddev_ms", "server_cv_pct"):
                        if key in stats:
                            assert isinstance(stats[key], (int, float)), (
                                f"{op} context.{side}.{key}"
                            )


def _assert_data_shape(data):
    assert data["schema_version"] == 2
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
        assert set(entry) <= SLOT_KINDS, f"{cid}: unknown slot kind"
        assert entry, f"{cid}: comparison without any kind slot"
        for kind, slot in entry.items():
            if slot["status"] == "ok":
                _assert_analysis_shape(slot["analysis"])
            elif slot["status"] == "unavailable":
                assert slot["reason"], f"{cid}/{kind} unavailable without reason"
            else:
                raise AssertionError(f"{cid}/{kind}: unknown status {slot['status']!r}")
        # The page's matrix keys rows by bare op name and resolves each to ONE kind — the
        # assembler rejects reads/writes op-name clashes, so fixtures must stay disjoint too.
        op_sets = [
            set(slot["analysis"]["ops"])
            for slot in entry.values()
            if slot["status"] == "ok"
        ]
        if len(op_sets) == 2:
            assert not (op_sets[0] & op_sets[1]), f"{cid}: reads/writes op names overlap"


@pytest.mark.parametrize(
    "fixture",
    ["data.json", "data-xss.json", "data-not-comparable.json", "data-cache-modes.json",
     "data-v2-skipped.json"],
)
def test_fixture_matches_page_schema(fixture):
    _assert_data_shape(load_fixture(fixture))


def test_main_fixture_covers_all_statuses():
    """data.json must keep exercising every status/kind combination the page renders."""
    data = load_fixture("data.json")
    slots = [
        slot
        for c in data["comparisons"].values()
        for slot in c.values()
    ]
    statuses = {slot["status"] for slot in slots}
    assert statuses == {"ok", "unavailable"}
    # Kind-model coverage: an ok writes slot, an unavailable writes slot, and a comparison
    # with NO writes slot at all (kind absent — the page renders '—' / omits the card).
    kinds_by_cid = {cid: set(entry) for cid, entry in data["comparisons"].items()}
    assert kinds_by_cid["main-pr"] == {"reads", "writes"}
    assert data["comparisons"]["main-pr"]["writes"]["status"] == "ok"
    assert data["comparisons"]["c-pr"]["writes"]["status"] == "unavailable"
    assert kinds_by_cid["c-main"] == {"reads"}
    ok_ops_by_kind = {
        kind: [
            entry
            for c in data["comparisons"].values()
            for k, slot in c.items()
            if k == kind and slot["status"] == "ok"
            for entry in slot["analysis"]["ops"].values()
        ]
        for kind in ("reads", "writes")
    }
    outcomes = {entry["op_outcome"] for entry in ok_ops_by_kind["reads"]}
    assert "diverged_advisory" in outcomes, (
        "fixture must include a diverged_advisory op so the page's advisory "
        "path stays covered"
    )
    # Writes ops are latency-only (correctness not_gated by design) and the fixture must
    # exercise BOTH the green and the red write path.
    assert ok_ops_by_kind["writes"], "fixture must include an ok writes slot with ops"
    assert all(e["correctness"] == "not_gated" for e in ok_ops_by_kind["writes"])
    write_outcomes = {e["op_outcome"] for e in ok_ops_by_kind["writes"]}
    assert {"pass", "regressed"} <= write_outcomes
    # Contract edge cases the page must keep rendering: a cell-less diverged op
    # (legal — counts in totals), an empty context object, a one-sided context
    # and a one-sided cell whose candidate p50/deltas are omitted entirely.
    ok_ops = ok_ops_by_kind["reads"]
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
    # Within-run dispersion + example-query enrichment (benchmark ≥ v2.7) must stay
    # covered ALONGSIDE unenriched ops in the same document, so the page's
    # feature-detection keeps both paths exercised in one render pass.
    both_sided_stats = [
        cell for entry in ok_ops for cell in entry["cells"]
        if all(
            "server_n" in cell["context"].get(side, {})
            and "server_stddev_ms" in cell["context"].get(side, {})
            and "server_cv_pct" in cell["context"].get(side, {})
            for side in ("baseline", "candidate")
        )
    ]
    assert both_sided_stats, "fixture must include a cell with two-sided n/σ/CV stats"
    assert any(
        "example_query" in entry for entry in ok_ops
    ), "fixture must include an op with an example_query"
    assert any(
        "example_query" not in entry
        and all("server_n" not in cell["context"].get(side, {})
                for cell in entry["cells"] for side in ("baseline", "candidate"))
        for entry in ok_ops
    ), "fixture must keep an op without any v2.7 enrichment (degradation coverage)"
    all_read_sides = [
        cell["context"][side]
        for c in data["comparisons"].values()
        for k, slot in c.items()
        if k == "reads" and slot["status"] == "ok"
        for entry in slot["analysis"]["ops"].values()
        for cell in entry["cells"]
        for side in ("baseline", "candidate")
        if side in cell["context"]
    ]
    # σ/CV without the server_n gate (a hand-broken shape the page must ignore).
    assert any(
        "server_n" not in s and "server_stddev_ms" in s for s in all_read_sides
    ), "fixture must include a σ/CV-without-server_n side"


def test_xss_fixture_has_script_shaped_labels():
    raw = json.dumps(load_fixture("data-xss.json"))
    assert "<script>" in raw and "onerror=" in raw
    # The example query itself must carry hostile markup so the page's
    # example-query block stays covered by the inertness tests.
    data = load_fixture("data-xss.json")
    examples = [
        entry["example_query"]
        for c in data["comparisons"].values()
        for slot in c.values()
        if slot["status"] == "ok"
        for entry in slot["analysis"]["ops"].values()
        if "example_query" in entry
    ]
    assert any("<script>" in q and "onerror=" in q for q in examples)


# --- 3. assembler CLI behavior ----------------------------------------------


def run_assembler(*args):
    return subprocess.run(
        [sys.executable, str(ASSEMBLER), *args],
        capture_output=True, text=True,
    )


def test_assembler_mixed_ok_and_unavailable(tmp_path):
    # Use real cells fixtures so the assembled document passes the full
    # page-schema check.
    data = load_fixture("data.json")
    cells = tmp_path / "cells.json"
    cells.write_text(json.dumps(data["comparisons"]["main-pr"]["reads"]["analysis"]))
    wcells = tmp_path / "cells-writes.json"
    wcells.write_text(json.dumps(data["comparisons"]["main-pr"]["writes"]["analysis"]))
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    out = tmp_path / "data.json"
    proc = run_assembler(
        "--meta", str(meta), "--out", str(out),
        "--ok", f"main-pr/reads={cells}",
        "--ok", f"main-pr/writes={wcells}",
        "--unavailable", "c-pr/reads=C leg failed (exit 1)",
        "--unavailable", "c-main/reads=C leg failed (exit 1)",
    )
    assert proc.returncode == 0, proc.stderr
    data = json.loads(out.read_text())
    assert data["comparisons"]["main-pr"]["reads"]["status"] == "ok"
    assert data["comparisons"]["main-pr"]["writes"]["status"] == "ok"
    assert data["comparisons"]["c-pr"] == {
        "reads": {"status": "unavailable", "reason": "C leg failed (exit 1)"}}
    _assert_data_shape(data)


def test_assembler_rejects_unknown_comparison_id(tmp_path):
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--unavailable", "nope/reads=broken",
    )
    assert proc.returncode != 0
    assert "nope" in proc.stderr


def test_assembler_rejects_unknown_kind(tmp_path):
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--unavailable", "c-pr/updates=broken",
    )
    assert proc.returncode != 0
    assert "updates" in proc.stderr


def test_assembler_rejects_bare_comparison_id(tmp_path):
    # The old v1 grammar (no /kind) must be rejected, not silently accepted.
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--unavailable", "c-pr=broken",
    )
    assert proc.returncode != 0


def test_assembler_rejects_duplicate_slot(tmp_path):
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--unavailable", "c-pr/reads=a", "--unavailable", "c-pr/reads=b",
    )
    assert proc.returncode != 0
    assert "duplicate" in proc.stderr.lower()


def test_assembler_allows_reads_and_writes_for_same_comparison(tmp_path):
    data = load_fixture("data.json")
    cells = tmp_path / "cells.json"
    cells.write_text(json.dumps(data["comparisons"]["main-pr"]["reads"]["analysis"]))
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    out = tmp_path / "d.json"
    proc = run_assembler(
        "--meta", str(meta), "--out", str(out),
        "--ok", f"main-pr/reads={cells}",
        "--unavailable", "main-pr/writes=writes leg failed (exit 1)",
    )
    assert proc.returncode == 0, proc.stderr
    slots = json.loads(out.read_text())["comparisons"]["main-pr"]
    assert slots["reads"]["status"] == "ok"
    assert slots["writes"]["status"] == "unavailable"


def test_assembler_rejects_op_name_clash_across_kinds(tmp_path):
    # The page keys matrix rows by bare op name, so a name appearing in both
    # the reads and writes slots of one comparison must be rejected.
    data = load_fixture("data.json")
    cells = tmp_path / "cells.json"
    cells.write_text(json.dumps(data["comparisons"]["main-pr"]["reads"]["analysis"]))
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--ok", f"main-pr/reads={cells}",
        "--ok", f"main-pr/writes={cells}",
    )
    assert proc.returncode != 0
    assert "BOTH reads and writes" in proc.stderr


def test_assembler_rejects_cells_without_ops(tmp_path):
    bad = tmp_path / "bad.json"
    bad.write_text(json.dumps({"schema_version": 1}))
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler(
        "--meta", str(meta), "--out", str(tmp_path / "d.json"),
        "--ok", f"main-pr/reads={bad}",
    )
    assert proc.returncode != 0


def test_assembler_requires_at_least_one_comparison(tmp_path):
    meta = tmp_path / "run-meta.json"
    meta.write_text((FIXTURES / "run-meta.json").read_text())
    proc = run_assembler("--meta", str(meta), "--out", str(tmp_path / "d.json"))
    assert proc.returncode != 0
