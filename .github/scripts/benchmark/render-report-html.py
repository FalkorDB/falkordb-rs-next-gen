#!/usr/bin/env python3
"""Render a synthetic-benchmark Markdown report into a self-contained, styled HTML page for hosting
on GitHub Pages.

The full ~46-shape report is too big for a PR comment, so CI hosts it here and links it from the
lean sticky comment. We PRE-RENDER the Markdown to HTML at publish time (rather than shipping a
client-side renderer) so the hosted page is static, dependency-free at view time, and carries no CDN
or client-JS surface. The report is our own tool's trusted output (op names are already
HTML-escaped by `report::md_cell`), and this script additionally strips `<script>`/`<style>` and
`on*=`/`javascript:` handlers as defense-in-depth.

Uses python-`markdown` when available (tables, fenced code, and `md_in_html` so the report's
collapsible `<details>` blocks render); if the import fails it degrades to the raw Markdown inside a
`<pre>` so publishing never hard-fails.
"""
from __future__ import annotations

import argparse
import html
import re
import sys

_PAGE = """<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="robots" content="noindex">
<title>{title}</title>
<style>
  :root {{ color-scheme: light dark; }}
  body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    max-width: 1100px; margin: 2rem auto; padding: 0 1rem; line-height: 1.5; }}
  table {{ border-collapse: collapse; margin: 1rem 0; font-variant-numeric: tabular-nums; }}
  th, td {{ border: 1px solid #8884; padding: 4px 10px; text-align: left; }}
  code, pre {{ font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }}
  pre {{ background: #8881; padding: 1rem; overflow-x: auto; border-radius: 6px; }}
  details {{ margin: 0.5rem 0; }}
  summary {{ cursor: pointer; }}
  h1, h2, h3 {{ line-height: 1.25; }}
</style>
</head>
<body>
{body}
</body>
</html>
"""

# Defense-in-depth: the input is our own trusted report, but never emit active content.
_SCRIPT_STYLE = re.compile(r"<\s*(script|style)\b.*?<\s*/\s*\1\s*>", re.IGNORECASE | re.DOTALL)
_ON_ATTR = re.compile(r"\son\w+\s*=\s*(\"[^\"]*\"|'[^']*'|[^\s>]+)", re.IGNORECASE)
_JS_URI = re.compile(r"(href|src)\s*=\s*(\"|')\s*javascript:[^\"']*(\2)", re.IGNORECASE)


def _sanitize(markup: str) -> str:
    markup = _SCRIPT_STYLE.sub("", markup)
    markup = _ON_ATTR.sub("", markup)
    markup = _JS_URI.sub(r"\1=\2#\2", markup)
    return markup


def render(md_text: str, title: str) -> str:
    try:
        import markdown  # type: ignore

        # md_in_html only parses Markdown *inside* a raw HTML block when that block is marked
        # `markdown="1"`. The report wraps each per-op table in a bare <details>, so without this the
        # tables render as literal text. Add the marker (idempotently) before converting.
        prepared = re.sub(r'<details(?![^>]*\bmarkdown=)', '<details markdown="1"', md_text)
        body = markdown.markdown(
            prepared,
            extensions=["tables", "fenced_code", "md_in_html", "sane_lists", "toc"],
            output_format="html5",
        )
    except Exception:  # noqa: BLE001 - any failure degrades to a readable raw fallback
        body = "<pre>" + html.escape(md_text) + "</pre>"
    return _PAGE.format(title=html.escape(title), body=_sanitize(body))


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--in", dest="inp", required=True, help="input Markdown report")
    ap.add_argument("--out", required=True, help="output HTML file")
    ap.add_argument("--title", default="Synthetic benchmark report", help="page <title>")
    args = ap.parse_args(argv)

    try:
        with open(args.inp, encoding="utf-8") as fh:
            md_text = fh.read()
    except OSError as exc:
        print(f"render-report-html: cannot read {args.inp}: {exc}", file=sys.stderr)
        return 1

    with open(args.out, "w", encoding="utf-8") as fh:
        fh.write(render(md_text, args.title))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
