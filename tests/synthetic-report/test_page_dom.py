"""Playwright DOM tests for the synthetic report page.

Covers the two properties the design demands of the page:
1. hostile op/engine labels (script-shaped strings in data.json) render as
   inert TEXT — nothing executes, nothing is parsed as markup;
2. the comparison/metric/cache selectors and the matrix filter chips (kind +
   verdict) and free-text op filter actually switch what is rendered.

data.json carries both kinds: reads on every comparison plus a writes slot on
main-pr (ok: one pass + one regressed op) and c-pr (unavailable); c-main has
no writes slot at all.
"""


XSS_OP_SCRIPT = "<script>window.__pwned=1</script>"
XSS_OP_IMG = '<img src=x onerror="window.__pwned=2">'
XSS_OP_SVG_WRITE = '<svg onload="window.__pwned=3">write'


def wait_ready(page):
    page.wait_for_selector("#cmpSeg button[data-cmp]")


# --- XSS inertness -----------------------------------------------------------


def test_script_shaped_labels_render_inert(serve_page, page):
    page.goto(serve_page("data-xss.json"))
    wait_ready(page)

    # Nothing executed: none of the payload sentinels may exist.
    assert page.evaluate("window.__pwned") is None

    # The op labels appear verbatim as text in the matrix rows (reads AND writes).
    rows = page.locator("#view tr[data-op]")
    ops = [rows.nth(i).get_attribute("data-op") for i in range(rows.count())]
    assert XSS_OP_SCRIPT in ops and XSS_OP_IMG in ops
    assert XSS_OP_SVG_WRITE in ops
    assert page.locator("#view td", has_text="window.__pwned=1").count() > 0

    # No <script>, <img> or <svg> node was created inside the rendered view.
    assert page.locator("#view script").count() == 0
    assert page.locator("#view img").count() == 0
    assert page.locator("#view svg").count() == 0
    assert page.locator("#metaGrid script, #metaGrid img").count() == 0


def test_script_shaped_labels_inert_in_comparison_view(serve_page, page):
    page.goto(serve_page("data-xss.json"))
    wait_ready(page)
    page.click('#cmpSeg button[data-cmp="main-pr"]')
    page.wait_for_selector("#view details")
    assert page.evaluate("window.__pwned") is None
    assert page.locator("#view script").count() == 0
    assert page.locator("#view img").count() == 0
    # The XSS-shaped warning from the analysis renders as text too.
    assert "onerror=" in page.locator("#view").inner_text()


# --- selectors / chips / filter behavior ------------------------------------


def test_default_view_is_matrix_all_comparisons(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    assert "on" in page.locator('#cmpSeg button[data-cmp="matrix"]').get_attribute(
        "class").split()
    # Matrix table has one column per comparison plus the op column.
    assert page.locator("#view th.cmp-col").count() == 3
    assert page.locator("#view tr[data-op]").count() > 0
    # Cache seg hides only because this fixture is single-mode (nothing to switch);
    # with two modes it shows in matrix view too (see the cache-selector test).
    assert page.locator("#matrixControls").is_visible()
    assert not page.locator("#cacheSeg").is_visible()


def test_matrix_cells_show_worst_gated_delta(serve_page, page):
    """Matrix cells carry the worst Δ% for the selected metric (emoji-only without one)."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    # main-pr aggregate_age: worst gated p50 cell is C=8 uncached, delta_pct ≈ -4.14.
    cell = page.locator('#view tr[data-op="aggregate_age"] td').nth(1)
    text = cell.inner_text()
    assert "🟢" in text and "-4.1%" in text
    title = cell.get_attribute("title")
    assert "worst p50 Δ -4.1%" in title
    assert "0.654 ms → 0.627 ms" in title and "C=8" in title and "uncached" in title
    # Compact raw line under the Δ%: baseline → candidate, unit once.
    assert cell.locator("span.mraw").inner_text() == "0.654 → 0.627 ms"
    # The gated p50 value carries no neutral marker.
    assert cell.locator("span.mdelta.delta-neu").count() == 0
    # c-pr distinct_labels is a cell-less diverged op: emoji only, no value, plain title.
    cell = page.locator('#view tr[data-op="distinct_labels"] td').nth(2)
    assert cell.inner_text().strip() == "⚠"
    assert "%" not in cell.get_attribute("title")
    # c-pr aggregate_age: advisory divergence makes its cells N/A-gated, but the p50
    # deltas exist and render — same as the comparison table (worst = max(-1.4, -4.3)).
    cell = page.locator('#view tr[data-op="aggregate_age"] td').nth(2)
    assert "-1.4%" in cell.inner_text()


def test_matrix_values_follow_metric_selector(serve_page, page):
    """The matrix value column follows the metric selector; emoji stays the p50 rollup."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    cell = page.locator('#view tr[data-op="aggregate_count_users"] td').nth(1)
    assert "+23.4%" in cell.inner_text()  # gated p50, worst at C=1
    page.click('#metricSeg [data-metric="p90"]')
    cell = page.locator('#view tr[data-op="aggregate_count_users"] td').nth(1)
    assert "+27.4%" in cell.inner_text()  # context p90, worst at C=1
    assert "🟢" in cell.inner_text()      # emoji unchanged
    # Non-p50 values are informational: neutral marker + metric named in the tooltip.
    assert cell.locator("span.mdelta.delta-neu").count() == 1
    assert "worst p90 Δ +27.4%" in cell.get_attribute("title")
    # aggregate_age has no cell with a complete p90 context pair -> emoji-only, no raw line.
    cell = page.locator('#view tr[data-op="aggregate_age"] td').nth(1)
    assert "%" not in cell.inner_text() and "→" not in cell.inner_text()
    # Throughput: higher is better, so the WORST delta is the minimum (C=1: -19.4%).
    page.click('#metricSeg [data-metric="throughput"]')
    cell = page.locator('#view tr[data-op="aggregate_count_users"] td').nth(1)
    assert "-19.4%" in cell.inner_text()
    assert "ops/s" in cell.get_attribute("title")
    assert cell.locator("span.mraw").inner_text() == "3194.3 → 2574.3 ops/s"


def test_matrix_values_follow_cache_selector(serve_page, page):
    """With two cache modes the selector shows in matrix view and switches the values."""
    page.goto(serve_page("data-cache-modes.json"))
    wait_ready(page)
    assert page.locator("#cacheSeg").is_visible()
    cell = page.locator('#view tr[data-op="aggregate_count_users"] td').nth(1)
    assert "+23.4%" in cell.inner_text()  # default: uncached
    page.click('#cacheSeg [data-cache="cached"]')
    cell = page.locator('#view tr[data-op="aggregate_count_users"] td').nth(1)
    assert "-5.0%" in cell.inner_text()   # cached worst = max(-10.0, -5.0)
    assert "cached" in cell.get_attribute("title")


def test_unavailable_comparison_greyed_with_reason(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    btn = page.locator('#cmpSeg button[data-cmp="c-main"]')
    assert "unavail" in btn.get_attribute("class")
    assert "C leg failed" in btn.get_attribute("title")
    # Visibly greyed, announced via title/aria-label — NOT aria-disabled (the button
    # is genuinely clickable: it reveals the reason card).
    assert btn.get_attribute("aria-disabled") is None
    aria = btn.get_attribute("aria-label")
    assert "unavailable" in aria and "click for reason" in aria
    opacity = btn.evaluate("el => getComputedStyle(el).opacity")
    assert float(opacity) < 1
    assert btn.evaluate("el => getComputedStyle(el).cursor") == "not-allowed"
    # Matrix column header for the unavailable comparison is greyed + reasoned.
    col = page.locator("#view th.cmp-col.unavail")
    assert col.count() == 1
    assert "C leg failed" in col.inner_text()
    # Selecting it shows the reason card instead of tables.
    btn.click()
    assert "C leg failed" in page.locator("#view").inner_text()
    assert page.locator("#view table").count() == 0


def test_comparison_selector_switches_to_card_view(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    page.click('#cmpSeg button[data-cmp="main-pr"]')
    page.wait_for_selector("#view .verdict-badge")
    assert page.locator("#view details").count() > 0
    # Filter controls stay available in comparison view (scoped chips + text search).
    assert page.locator("#matrixControls").is_visible()
    # Back to matrix.
    page.click('#cmpSeg button[data-cmp="matrix"]')
    page.wait_for_selector("#view th.cmp-col")
    assert page.locator("#matrixControls").is_visible()


def test_metric_selector_gated_vs_informational(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    page.click('#cmpSeg button[data-cmp="main-pr"]')
    page.wait_for_selector("#view .verdict-badge")
    # p50 (default): gated, no informational note.
    note = page.locator("#metricNote")
    assert "not gated" not in note.inner_text()
    # p95: informational note appears.
    page.click('#metricSeg button[data-metric="p95"]')
    assert "informational" in note.inner_text()
    assert "not gated" in note.inner_text()
    # throughput: also states higher-is-better.
    page.click('#metricSeg button[data-metric="throughput"]')
    assert "higher is better" in note.inner_text().lower()


def test_cache_selector_hidden_for_single_mode(serve_page, page):
    # Fixtures were measured uncached-only, so the cache selector must stay
    # hidden even in comparison view.
    page.goto(serve_page("data.json"))
    wait_ready(page)
    page.click('#cmpSeg button[data-cmp="main-pr"]')
    page.wait_for_selector("#view .verdict-badge")
    assert not page.locator("#cacheSeg").is_visible()


def test_matrix_chips_filter_rows(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    total = page.locator("#view tr[data-op]").count()
    assert total > 1

    # Reads: one diverged_advisory op with cells and one cell-less diverged op
    # (both ⚠ in c-pr), no regressed reads. Writes (main-pr only): one pass op
    # and one regressed op. So:
    # any-red -> 1 row (the regressed write op);
    # all-green -> total - 3 (two diverged reads + the regressed write drop out).
    page.click('#chips [data-chip="any-red"]')
    rows = page.locator("#view tr[data-op]")
    assert rows.count() == 1
    assert rows.first.get_attribute("data-op") == "single_edge_update"

    page.click('#chips [data-chip="all-green"]')
    assert page.locator("#view tr[data-op]").count() == total - 3

    # No red vs the C engine anywhere (c-pr writes are unavailable, reads have no reds).
    page.click('#chips [data-chip="red-vs-c"]')
    assert page.locator("#view tr[data-op]").count() == 0

    page.click('#chips [data-chip="red-vs-main"]')
    rows = page.locator("#view tr[data-op]")
    assert rows.count() == 1
    assert rows.first.get_attribute("data-op") == "single_edge_update"

    page.click('#chips [data-chip="all"]')
    assert page.locator("#view tr[data-op]").count() == total


def test_matrix_text_filter_composes_and(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    total = page.locator("#view tr[data-op]").count()
    page.fill("#opFilter", "aggregate")
    filtered = page.locator("#view tr[data-op]").count()
    assert 0 < filtered < total
    for i in range(filtered):
        assert "aggregate" in page.locator("#view tr[data-op]").nth(i).get_attribute("data-op")
    # Composes AND with the all-green chip: the diverged aggregate_age op
    # is excluded, leaving strictly fewer rows.
    page.click('#chips [data-chip="all-green"]')
    assert page.locator("#view tr[data-op]").count() < filtered
    page.fill("#opFilter", "zz-no-such-op")
    assert page.locator("#view tr[data-op]").count() == 0


def test_comparison_view_has_op_filters(serve_page, page):
    """Chips + text search work in comparison view too, scoped to its own outcomes."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    page.click('#cmpSeg button[data-cmp="c-pr"]')
    # The filter controls stay visible outside the matrix.
    assert page.locator("#matrixControls").is_visible()
    total = page.locator("#view details[data-op]").count()
    assert total > 2
    # Text search narrows the op list (same substring match as the matrix).
    page.fill("#opFilter", "aggregate_age")
    assert page.locator('#view details[data-op="aggregate_age"]').count() == 1
    assert page.locator("#view details[data-op]").count() == 1
    page.fill("#opFilter", "zz-no-such-op")
    assert page.locator("#view details[data-op]").count() == 0
    assert f"0 of {total}" in page.locator("#view p.count-note").inner_text()
    page.fill("#opFilter", "")
    # all-green is scoped to THIS comparison: c-pr's two diverged_advisory ops drop out.
    page.click('#chips [data-chip="all-green"]')
    assert page.locator("#view details[data-op]").count() == total - 2
    assert page.locator('#view details[data-op="aggregate_age"]').count() == 0
    assert "(this comparison)" in page.locator("#view p.count-note").inner_text()
    # any-red scoped: c-pr has no regressed ops.
    page.click('#chips [data-chip="any-red"]')
    assert page.locator("#view details[data-op]").count() == 0
    # red-vs-main keeps its global meaning in every view, and says so.
    page.click('#chips [data-chip="red-vs-main"]')
    assert "(global)" in page.locator("#view p.count-note").inner_text()
    page.click('#chips [data-chip="all"]')
    assert page.locator("#view details[data-op]").count() == total
    # An unavailable comparison renders only its reason card — no filter controls.
    page.click('#cmpSeg button[data-cmp="c-main"]')
    assert not page.locator("#matrixControls").is_visible()


def test_meta_header_renders_run_facts(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    meta = page.locator("#metaGrid").inner_text()
    assert "745" in meta          # PR number
    assert "x86" in meta          # arch
    assert "20m 34s" in meta      # 1234s wall-clock, humanized
    assert "falkordb/falkordb:edge" in meta
    # Per-comparison profile/policy is stated.
    assert "strict" in meta and "cross-engine" in meta
    assert "gate" in meta and "advisory" in meta


# --- header warnings strip + non-comparable status kinds ---------------------


def test_header_surfaces_tool_warnings(serve_page, page):
    page.goto(serve_page("data.json"))
    wait_ready(page)
    strip = page.locator("#warnStrip")
    assert strip.is_visible()
    text = strip.inner_text()
    # Real tool-emitted advisory warnings, prefixed with the comparison label.
    assert "module version" in text or "server image changed" in text
    assert "PR vs main:" in text
    # Warning strings render inert as text (data-xss carries hostile warnings).
    page.goto(serve_page("data-xss.json"))
    wait_ready(page)
    assert page.evaluate("window.__pwned") is None
    assert page.locator("#warnStrip script, #warnStrip img").count() == 0
    assert "onerror=" in page.locator("#warnStrip").inner_text()


def test_not_comparable_kinds_render_kind_and_reason(serve_page, page):
    page.goto(serve_page("data-not-comparable.json"))
    wait_ready(page)

    page.click('#cmpSeg button[data-cmp="c-pr"]')
    page.wait_for_selector("#view .banner.bad")
    banner = page.locator("#view .banner.bad").inner_text()
    assert "not comparable (workload mismatch)" in banner
    assert "workload_hash" in banner  # the tool's reason is shown verbatim

    page.click('#cmpSeg button[data-cmp="c-main"]')
    page.wait_for_selector("#view .banner.bad")
    banner = page.locator("#view .banner.bad").inner_text()
    assert "not comparable (no common ops)" in banner
    assert "share no operation names" in banner


def test_cellless_and_contextless_ops_render_without_errors(serve_page, page):
    errors = []
    page.on("pageerror", lambda e: errors.append(str(e)))
    page.goto(serve_page("data.json"))
    wait_ready(page)

    # The cell-less diverged op appears in the matrix as ⚠ for c-pr.
    row = page.locator('#view tr[data-op="distinct_labels"]')
    assert row.count() == 1

    # Card view: the op renders with an honest correctness-only note, no table.
    page.click('#cmpSeg button[data-cmp="c-pr"]')
    page.wait_for_selector("#view details")
    details = page.locator('#view details', has_text="distinct_labels")
    details.locator("summary").click()
    body = details.locator(".body").inner_text()
    assert "correctness-only" in body

    # aggregate_age carries the producer edge shapes: C=1 has context {}, C=8 a
    # baseline-only context, C=16 omits candidate p50 + deltas (one-sided cell).
    page.click('#cmpSeg button[data-cmp="main-pr"]')
    page.wait_for_selector("#view details")
    agg = page.locator('#view details', has_text="aggregate_age")
    agg.locator("summary").click()
    rows = agg.locator("tbody tr")
    assert rows.count() == 3

    # (a) p50 view, one-sided C=16 row: candidate/deltas em-dash, verdict N/A.
    c16 = rows.nth(2)
    tds = [c16.locator("td").nth(i).inner_text() for i in range(7)]
    assert tds[0] == "16"
    assert "ms" in tds[1]              # baseline p50 present
    assert tds[2] == "—" and tds[3] == "—" and tds[4] == "—"
    assert tds[6] == "N/A"

    # Informational metric: values come from the per-side context.
    page.click('#metricSeg [data-metric="p90"]')
    page.wait_for_selector("#view details")
    agg = page.locator('#view details', has_text="aggregate_age")
    agg.locator("summary").click()
    rows = agg.locator("tbody tr")

    # (b) context {}: both sides em-dash.
    c1 = [rows.nth(0).locator("td").nth(i).inner_text() for i in range(4)]
    assert c1[0] == "1" and c1[1] == "—" and c1[2] == "—"

    # (c) one-sided context: only the side that has it shows a value.
    c8 = [rows.nth(1).locator("td").nth(i).inner_text() for i in range(4)]
    assert c8[0] == "8" and "ms" in c8[1] and c8[2] == "—"

    assert errors == []


def test_cells_v2_skipped_ops_render_neutral(serve_page, page):
    """Phase 6 cells v2: op_outcome "skipped" is its own neutral state — ⏭ in the
    matrix, excluded from both red and green chip rollups, side-attributed note +
    ⏭ totals bucket in the comparison view, tier "full" chip rendered as-is."""
    errors = []
    page.on("pageerror", lambda e: errors.append(str(e)))
    page.goto(serve_page("data-v2-skipped.json"))
    wait_ready(page)

    # Matrix: both skipped ops render ⏭ (emoji-only, no value line) in the c-pr column.
    for op in ("algo_bfs", "algo_pagerank"):
        cell = page.locator(f'#view tr[data-op="{op}"] td').nth(2)
        assert cell.inner_text().strip() == "⏭", op
        assert cell.locator("span.mdelta").count() == 0, op

    # Chip rollups: skipped is neither red nor green.
    page.click('#chips [data-chip="any-red"]')
    assert page.locator("#view tr[data-op]").count() == 0
    page.click('#chips [data-chip="all-green"]')
    green_ops = page.locator("#view tr[data-op]")
    ops = [green_ops.nth(i).get_attribute("data-op") for i in range(green_ops.count())]
    assert "algo_bfs" not in ops and "algo_pagerank" not in ops
    page.click('#chips [data-chip="all"]')

    # Comparison view: ⏭ bucket in the totals line, per-op note names the skipped sides.
    page.click('#cmpSeg button[data-cmp="c-pr"]')
    page.wait_for_selector("#view details")
    assert "⏭ 2" in page.locator("#view .cmp-sub").first.inner_text()
    bfs = page.locator("#view details", has_text="algo_bfs")
    assert bfs.locator("summary .tier").inner_text().lower() == "full"
    bfs.locator("summary").click()
    body = bfs.locator(".body").inner_text()
    assert "op skipped on baseline + candidate" in body
    assert "excluded from pass/regressed rollups" in body
    pr = page.locator("#view details", has_text="algo_pagerank")
    pr.locator("summary").click()
    assert "op skipped on candidate —" in pr.locator(".body").inner_text()

    # main-pr carries skipped: 0 — must not surface a ⏭ bucket.
    page.click('#cmpSeg button[data-cmp="main-pr"]')
    page.wait_for_selector("#view details")
    assert "⏭" not in page.locator("#view .cmp-sub").first.inner_text()

    assert errors == []


# --- writes kind: chips, matrix slots, per-kind cards ------------------------


def test_kind_chips_filter_matrix_rows(serve_page, page):
    """Kind chips appear only for multi-kind runs and compose with the other filters."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    chips = page.locator('#chips button[data-kind]')
    assert [chips.nth(i).inner_text() for i in range(chips.count())] == [
        "all kinds", "reads", "writes"]
    total = page.locator("#view tr[data-op]").count()
    reads = page.locator('#view tr[data-op][data-kind="reads"]').count()
    writes = page.locator('#view tr[data-op][data-kind="writes"]').count()
    assert writes == 2 and reads + writes == total

    page.click('#chips button[data-kind="writes"]')
    rows = page.locator("#view tr[data-op]")
    ops = sorted(rows.nth(i).get_attribute("data-op") for i in range(rows.count()))
    assert ops == ["single_edge_update", "single_vertex_write"]

    # Composes AND with the verdict chip and the text filter.
    page.click('#chips [data-chip="all-green"]')
    rows = page.locator("#view tr[data-op]")
    assert rows.count() == 1 and rows.first.get_attribute("data-op") == "single_vertex_write"
    page.fill("#opFilter", "zz-no-such-op")
    assert page.locator("#view tr[data-op]").count() == 0
    page.fill("#opFilter", "")
    page.click('#chips [data-chip="all"]')

    page.click('#chips button[data-kind="reads"]')
    assert page.locator("#view tr[data-op]").count() == reads
    page.click('#chips button[data-kind="all"]')
    assert page.locator("#view tr[data-op]").count() == total


def test_kind_chips_absent_for_reads_only_run(serve_page, page):
    """A run without writes slots must render exactly as before — no kind chips."""
    page.goto(serve_page("data-v2-skipped.json"))
    wait_ready(page)
    assert page.locator('#chips button[data-kind]').count() == 0


def test_matrix_write_rows_mark_unavailable_and_absent_slots(serve_page, page):
    """A write-op row shows '—' with an honest per-kind tooltip where the writes slot
    is unavailable (c-pr) or absent entirely (c-main), while reads stay rendered."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    row = page.locator('#view tr[data-op="single_edge_update"]')
    # main-pr column: real regressed cell with the write op's Δ%.
    main_pr = row.locator("td").nth(1)
    assert "🔴" in main_pr.inner_text() and "+35.7%" in main_pr.inner_text()
    assert "19.842 ms → 26.917 ms" in main_pr.get_attribute("title")
    # c-pr column: writes slot unavailable — em-dash + kind-scoped reason.
    c_pr = row.locator("td").nth(2)
    assert c_pr.inner_text().strip() == "—"
    assert c_pr.get_attribute("title") == (
        "unavailable (writes) — C writes leg failed (exit 1) during: "
        "measuring C-engine writes")
    # c-main column: no writes slot at all.
    c_main = row.locator("td").nth(3)
    assert c_main.inner_text().strip() == "—"
    assert c_main.get_attribute("title") == "unavailable (writes) — not part of this run"
    # A reads row keeps its real cells in c-pr (kind slots are independent).
    assert "%" in page.locator('#view tr[data-op="aggregate_age"] td').nth(2).inner_text()


def test_comparison_view_renders_per_kind_cards(serve_page, page):
    """main-pr renders one card per kind (reads first), writes card carries its own
    verdict and the not_gated correctness note; the kind chip hides other kinds."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    page.click('#cmpSeg button[data-cmp="main-pr"]')
    page.wait_for_selector("#view .cmp-card")
    cards = page.locator("#view .cmp-card")
    assert cards.count() == 2
    assert [cards.nth(i).get_attribute("data-kind") for i in range(2)] == ["reads", "writes"]
    titles = page.locator("#view .cmp-title")
    assert titles.nth(0).inner_text().endswith("— reads")
    assert titles.nth(1).inner_text().endswith("— writes")
    writes_card = cards.nth(1)
    assert writes_card.locator(".verdict-badge").inner_text() == "regressed"
    # Op details render per kind (as siblings following each card): the regressed
    # write op is present, carries the not-gated chip, and has its C=1 cell row.
    d = page.locator('#view details[data-op="single_edge_update"]')
    assert d.count() == 1
    assert d.locator("summary .corr").inner_text() == "correctness not gated"
    d.locator("summary").click()
    assert d.locator("tbody tr").count() == 1
    total_details = page.locator("#view details[data-op]").count()
    # Kind chip scopes the card list AND the op details.
    page.click('#chips button[data-kind="reads"]')
    cards = page.locator("#view .cmp-card")
    assert cards.count() == 1 and cards.first.get_attribute("data-kind") == "reads"
    assert page.locator("#view details[data-op]").count() == total_details - 2
    page.click('#chips button[data-kind="writes"]')
    cards = page.locator("#view .cmp-card")
    assert cards.count() == 1 and cards.first.get_attribute("data-kind") == "writes"
    assert page.locator("#view details[data-op]").count() == 2
    page.click('#chips button[data-kind="all"]')
    assert page.locator("#view .cmp-card").count() == 2


def test_unavailable_writes_kind_renders_banner_card(serve_page, page):
    """c-pr: reads render normally, the writes card degrades to an honest banner."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    page.click('#cmpSeg button[data-cmp="c-pr"]')
    page.wait_for_selector("#view .cmp-card")
    cards = page.locator("#view .cmp-card")
    assert cards.count() == 2
    writes_card = cards.nth(1)
    assert writes_card.get_attribute("data-kind") == "writes"
    banner = writes_card.locator(".banner.warn").inner_text()
    assert "unavailable — C writes leg failed (exit 1)" in banner
    # Reads details still render; no write-op details exist (details are card siblings).
    details = page.locator("#view details[data-op]")
    assert details.count() > 0
    ops = {details.nth(i).get_attribute("data-op") for i in range(details.count())}
    assert "single_edge_update" not in ops and "single_vertex_write" not in ops


def test_writes_warning_prefixed_in_header_strip(serve_page, page):
    """Warnings from a writes analysis carry the '(writes)' prefix in the strip."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    text = page.locator("#warnStrip").inner_text()
    assert "PR vs main: (writes)" in text
    assert "latency-only" in text
