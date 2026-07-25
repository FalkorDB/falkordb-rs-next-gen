"""Playwright DOM tests for the synthetic report page.

Covers the two properties the design demands of the page:
1. hostile op/engine labels (script-shaped strings in data.json) render as
   inert TEXT — nothing executes, nothing is parsed as markup;
2. the comparison/metric/cache selectors and the matrix filter chips and
   free-text op filter actually switch what is rendered.
"""


XSS_OP_SCRIPT = "<script>window.__pwned=1</script>"
XSS_OP_IMG = '<img src=x onerror="window.__pwned=2">'


def wait_ready(page):
    page.wait_for_selector("#cmpSeg button[data-cmp]")


# --- XSS inertness -----------------------------------------------------------


def test_script_shaped_labels_render_inert(serve_page, page):
    page.goto(serve_page("data-xss.json"))
    wait_ready(page)

    # Nothing executed: none of the payload sentinels may exist.
    assert page.evaluate("window.__pwned") is None

    # The op labels appear verbatim as text in the matrix rows.
    rows = page.locator("#view tr[data-op]")
    ops = [rows.nth(i).get_attribute("data-op") for i in range(rows.count())]
    assert XSS_OP_SCRIPT in ops and XSS_OP_IMG in ops
    assert page.locator("#view td", has_text="window.__pwned=1").count() > 0

    # No <script> or <img> node was created inside the rendered view.
    assert page.locator("#view script").count() == 0
    assert page.locator("#view img").count() == 0
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
    # Matrix ignores cache mode; its controls are visible, cache seg hidden.
    assert page.locator("#matrixControls").is_visible()
    assert not page.locator("#cacheSeg").is_visible()


def test_matrix_cells_show_worst_gated_delta(serve_page, page):
    """Matrix cells carry the worst gated Δ% next to the emoji (emoji-only without one)."""
    page.goto(serve_page("data.json"))
    wait_ready(page)
    # main-pr aggregate_age: worst gated cell is C=8 uncached, delta_pct ≈ -4.14.
    cell = page.locator('#view tr[data-op="aggregate_age"] td').nth(1)
    text = cell.inner_text()
    assert "🟢" in text and "-4.1%" in text
    title = cell.get_attribute("title")
    assert "worst gated Δ -4.1%" in title
    assert "0.654 ms → 0.627 ms" in title and "C=8" in title and "uncached" in title
    # c-pr distinct_labels is a cell-less diverged op: emoji only, no value, plain title.
    cell = page.locator('#view tr[data-op="distinct_labels"] td').nth(2)
    assert cell.inner_text().strip() == "⚠"
    assert "%" not in cell.get_attribute("title")
    # c-pr aggregate_age: cells exist but none carries a p50 verdict (advisory divergence
    # makes them not_applicable) — emoji only.
    cell = page.locator('#view tr[data-op="aggregate_age"] td').nth(2)
    assert "%" not in cell.inner_text()


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

    # The sample fixture has one diverged_advisory op with cells and one
    # cell-less diverged op (both ⚠ in c-pr) and no regressed ops, so:
    # any-red -> 0 rows;  all-green -> total - 2 rows (both diverged ops are
    # non-green in an available comparison).
    page.click('#chips [data-chip="any-red"]')
    assert page.locator("#view tr[data-op]").count() == 0

    page.click('#chips [data-chip="all-green"]')
    assert page.locator("#view tr[data-op]").count() == total - 2

    page.click('#chips [data-chip="red-vs-c"]')
    assert page.locator("#view tr[data-op]").count() == 0

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
