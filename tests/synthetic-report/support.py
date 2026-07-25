"""Shared paths + helpers for the synthetic-report page tests.

Deliberately NOT in conftest.py: importing `conftest` by bare module name is
ambiguous under a wider pytest invocation (tests/conftest.py may already be
bound as the `conftest` module), so test modules import from here instead and
conftest.py stays fixtures-only.
"""

import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO_ROOT = HERE.parent.parent
SCRIPTS_DIR = REPO_ROOT / ".github" / "scripts" / "benchmark"
TEMPLATE = SCRIPTS_DIR / "synthetic-report.html"
FIXTURES = HERE / "fixtures"


def load_fixture(name):
    with open(FIXTURES / name, encoding="utf-8") as fh:
        return json.load(fh)
