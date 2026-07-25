"""Shared fixtures for the synthetic-report page tests.

Serves the committed single-file template plus a chosen data.json fixture
over a real (loopback) HTTP server, because the page fetches its sibling
data.json — file:// URLs would be blocked by CORS in Chromium.
"""

import http.server
import json
import shutil
import threading
from pathlib import Path

import pytest

HERE = Path(__file__).resolve().parent
REPO_ROOT = HERE.parent.parent
SCRIPTS_DIR = REPO_ROOT / ".github" / "scripts" / "benchmark"
TEMPLATE = SCRIPTS_DIR / "synthetic-report.html"
FIXTURES = HERE / "fixtures"


def load_fixture(name):
    with open(FIXTURES / name, encoding="utf-8") as fh:
        return json.load(fh)


class _QuietHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, *args):  # noqa: D102 - silence per-request logging
        pass


@pytest.fixture
def serve_page(tmp_path):
    """Return a factory: serve_page(fixture_name) -> URL of the page.

    Copies the template as index.html next to the chosen fixture (renamed
    data.json) into a directory OWNED BY THAT CALL and serves it on an
    ephemeral port. Per-call docroots keep multiple servers in one test
    (e.g. data.json then data-xss.json) from mutating each other's files
    while still serving — shared state here would be a flake.
    """
    servers = []

    def _serve(fixture_name):
        docroot = tmp_path / f"site-{len(servers)}"
        docroot.mkdir()
        shutil.copy(TEMPLATE, docroot / "index.html")
        shutil.copy(FIXTURES / fixture_name, docroot / "data.json")

        def handler(*args, **kwargs):
            return _QuietHandler(*args, directory=str(docroot), **kwargs)

        httpd = http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler)
        thread = threading.Thread(target=httpd.serve_forever, daemon=True)
        thread.start()
        servers.append((httpd, thread))
        return f"http://127.0.0.1:{httpd.server_address[1]}/index.html"

    yield _serve

    for httpd, thread in servers:
        httpd.shutdown()
        httpd.server_close()
        thread.join(timeout=5)
