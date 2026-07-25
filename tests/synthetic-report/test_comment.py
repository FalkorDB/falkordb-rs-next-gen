"""Snapshot + behavior tests for render-synthetic-comment.py (the sticky PR comment).

The comment is the design's B4 deliverable: three verdict lines (one per comparison ID),
diverged counts on the cross-engine lines, not_comparable reasons on unavailable ones,
the run wall-clock from run-meta.json, worst offenders for the gating main-pr comparison
only, and a single link to the interactive page.
"""

import subprocess
import sys

from conftest import FIXTURES, SCRIPTS_DIR

RENDERER = SCRIPTS_DIR / "render-synthetic-comment.py"
URL = "https://falkordb.github.io/falkordb-rs-next-gen/synthetic-benchmark/branch/pr-745/"

INTRO = (
    "Identical recorded workload replayed into each engine image, measured **back-to-back on "
    "one runner**. `PR vs main` gates on strict p50 budgets (result divergence fails it); the "
    "C-engine comparisons use looser cross-engine budgets and are **advisory** — divergence "
    "never gates them. **Non-blocking.**"
)


def render(*args):
    proc = subprocess.run(
        [sys.executable, str(RENDERER), *args], capture_output=True, text=True,
    )
    assert proc.returncode == 0, proc.stderr
    return proc.stdout


def fixture_arg(cid, name):
    return f"{cid}={FIXTURES / name}"


def test_full_three_way_snapshot():
    """Golden output for a complete healthy run (all three summaries + run-meta + URL)."""
    out = render(
        "--summary", fixture_arg("main-pr", "summary-main-pr.json"),
        "--summary", fixture_arg("c-pr", "summary-c-pr.json"),
        "--summary", fixture_arg("c-main", "summary-c-main.json"),
        "--run-meta", str(FIXTURES / "run-meta.json"),
        "--url", URL,
        "--arch", "x86",
    )
    expected = f"""<!-- synthetic-benchmark -->
## 🧪 Synthetic per-op benchmark (`x86`)

{INTRO}

🟢 **PR vs main** — no p50 regression beyond budget across 26 comparable cell(s)
🟢 **PR vs C engine** — no p50 regression beyond budget across 26 comparable cell(s)
🟢 **main vs C engine** — no p50 regression beyond budget across 26 comparable cell(s)

⏱ total wall-clock 20m 34s · 📄 **[Interactive report →]({URL})**
"""
    assert out == expected


def test_unavailable_c_leg_snapshot():
    """Golden output when the C leg failed: stub summaries render honest ⚠ lines."""
    out = render(
        "--summary", fixture_arg("main-pr", "summary-main-pr.json"),
        "--summary", fixture_arg("c-pr", "summary-c-pr-stub.json"),
        "--summary", fixture_arg("c-main", "summary-c-main-stub.json"),
        "--run-meta", str(FIXTURES / "run-meta.json"),
        "--url", URL,
        "--arch", "arm",
        "--marker", "<!-- synthetic-benchmark-arm -->",
    )
    reason = "C leg timed out after 2700s during: measuring the C engine"
    expected = f"""<!-- synthetic-benchmark-arm -->
## 🧪 Synthetic per-op benchmark (`arm`)

{INTRO}

🟢 **PR vs main** — no p50 regression beyond budget across 26 comparable cell(s)
⚠ **PR vs C engine** — C-engine leg unavailable — {reason}
⚠ **main vs C engine** — C-engine leg unavailable — {reason}

⏱ total wall-clock 20m 34s · 📄 **[Interactive report →]({URL})**
"""
    assert out == expected


def test_cross_engine_diverged_count_on_line():
    # Real tool wording: the Advisory headline itself carries "2 diverged", so the
    # renderer must NOT stutter an extra suffix.
    out = render(
        "--summary", fixture_arg("c-pr", "summary-c-pr-diverged.json"),
        "--url", URL,
    )
    line = next(l for l in out.splitlines() if "**PR vs C engine**" in l)
    assert line.startswith("⚠")
    assert "2 diverged" in line
    assert "returned different results" not in line


def test_cross_engine_diverged_suffix_when_headline_lacks_it(tmp_path):
    # If a summary's headline doesn't mention divergence, the explicit ⚠ count is added.
    import json
    d = json.loads((FIXTURES / "summary-c-pr-diverged.json").read_text())
    d["headline"] = "custom headline without the d-word"
    p = tmp_path / "s.json"
    p.write_text(json.dumps(d))
    out = render("--summary", f"c-pr={p}", "--url", URL)
    line = next(l for l in out.splitlines() if "**PR vs C engine**" in l)
    assert "· ⚠ 2 ops returned different results (advisory)" in line


def test_worst_offenders_only_for_main_pr():
    out = render(
        "--summary", fixture_arg("main-pr", "summary-main-pr-regressed.json"),
        "--summary", fixture_arg("c-pr", "summary-c-pr-diverged.json"),
        "--url", URL,
    )
    assert "**Worst offenders (PR vs main):**" in out
    assert "`expand_friends` (2 cells over budget)" in out
    assert "`shortest_path` (1 cell over budget)" in out
    # c-pr's diverged offenders must NOT surface in the offenders section.
    offenders_line = next(l for l in out.splitlines() if "Worst offenders" in l)
    assert "aggregate_age" not in offenders_line
    # The regressed gating line carries the red emoji.
    line = next(l for l in out.splitlines() if "**PR vs main**" in l)
    assert line.startswith("🔴")


def test_schema_mismatch_warns_and_skips(tmp_path):
    bad = tmp_path / "summary-v1.json"
    bad.write_text('{"schema_version": 1, "verdict": "pass", "headline": "old shape"}')
    out = render("--summary", f"main-pr={bad}", "--url", URL)
    line = next(l for l in out.splitlines() if "**PR vs main**" in l)
    assert "schema v1 is not the expected v2" in line
    assert "old shape" not in out  # v1 content must not be rendered


def test_missing_summary_degrades_to_honest_line():
    out = render(
        "--summary", fixture_arg("main-pr", "summary-main-pr.json"),
        # c-pr and c-main not passed at all (e.g. files never produced).
        "--url", URL,
    )
    for label in ("PR vs C engine", "main vs C engine"):
        line = next(l for l in out.splitlines() if f"**{label}**" in l)
        assert "no summary produced for this run" in line


def test_no_run_meta_omits_wall_clock():
    out = render(
        "--summary", fixture_arg("main-pr", "summary-main-pr.json"),
        "--url", URL,
    )
    assert "total wall-clock" not in out
    assert f"[Interactive report →]({URL})" in out


def test_no_url_states_hosting_unavailable():
    out = render("--summary", fixture_arg("main-pr", "summary-main-pr.json"))
    assert "report hosting unavailable" in out


def test_rejects_unknown_comparison_id():
    proc = subprocess.run(
        [sys.executable, str(RENDERER), "--summary", "bogus=x.json"],
        capture_output=True, text=True,
    )
    assert proc.returncode != 0
    assert "bogus" in proc.stderr


def test_rejects_duplicate_comparison_id():
    proc = subprocess.run(
        [sys.executable, str(RENDERER),
         "--summary", fixture_arg("main-pr", "summary-main-pr.json"),
         "--summary", fixture_arg("main-pr", "summary-main-pr.json")],
        capture_output=True, text=True,
    )
    assert proc.returncode != 0
    assert "duplicate" in proc.stderr.lower()


def test_cross_engine_suffix_added_when_diverged_mentioned_without_count(tmp_path):
    # Only a genuine "<N> diverged" count suppresses the suffix; a headline that
    # merely mentions divergence must still get the explicit count.
    import json
    d = json.loads((FIXTURES / "summary-c-pr-diverged.json").read_text())
    d["headline"] = "pass, results diverged for some ops — see the report"
    p = tmp_path / "s.json"
    p.write_text(json.dumps(d))
    out = render("--summary", f"c-pr={p}", "--url", URL)
    line = next(l for l in out.splitlines() if "**PR vs C engine**" in l)
    assert "· ⚠ 2 ops returned different results (advisory)" in line


def test_malformed_totals_still_renders_line(tmp_path):
    # A summary whose totals is not a dict must degrade (no diverged suffix),
    # never crash — a crash here would silently drop the sticky comment.
    import json
    d = json.loads((FIXTURES / "summary-c-pr-diverged.json").read_text())
    d["totals"] = "garbage"
    p = tmp_path / "s.json"
    p.write_text(json.dumps(d))
    out = render("--summary", f"c-pr={p}", "--url", URL)
    line = next(l for l in out.splitlines() if "**PR vs C engine**" in l)
    assert "returned different results" not in line


def test_non_finite_elapsed_omits_wall_clock(tmp_path):
    # json.loads accepts bare Infinity; int(float("inf")) raises OverflowError,
    # which must be swallowed like any other unusable elapsed value.
    import json
    meta = json.loads((FIXTURES / "run-meta.json").read_text())
    p = tmp_path / "run-meta.json"
    p.write_text(json.dumps(meta).replace(json.dumps(meta["elapsed_secs"]), "Infinity"))
    assert "Infinity" in p.read_text()
    out = render(
        "--summary", fixture_arg("main-pr", "summary-main-pr.json"),
        "--run-meta", str(p), "--url", URL,
    )
    assert "total wall-clock" not in out
