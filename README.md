# The Rust implementation of falkordb

[![Rust](https://github.com/FalkorDB/falkordb-rs-next-gen/actions/workflows/rust-push.yml/badge.svg)](https://github.com/FalkorDB/falkordb-rs-next-gen/actions/workflows/rust-push.yml)
[![codecov](https://codecov.io/gh/FalkorDB/falkordb-rs-next-gen/branch/main/graph/badge.svg)](https://codecov.io/gh/FalkorDB/falkordb-rs-next-gen)
[![license](https://img.shields.io/badge/license-Server_Side_Public_License-green)](https://github.com/FalkorDB/falkordb-rs-next-gen/blob/main/LICENSE)

## Developer Guide

### Quick Start with Dev Container (Recommended)

The easiest way to get started is using the development container, which includes all dependencies pre-installed:

1. Install [Docker](https://docs.docker.com/get-docker/) and [VS Code](https://code.visualstudio.com/)
2. Install the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
3. Open this project in VS Code
4. Click "Reopen in Container" when prompted (or press F1 and select "Dev Containers: Reopen in Container")
5. Wait for the container to build (first time takes ~10-15 minutes)
6. Start developing! All dependencies are ready to use.

See [.devcontainer/README.md](.devcontainer/README.md) for more details.

### Manual Setup

If you prefer to set up the environment manually:

#### Build

```
cargo build
```

#### Dependencies:

GraphBLAS & RediSearch must be built and installed before building this project.

- building [GraphBLAS](https://github.com/DrTimothyAldenDavis/GraphBLAS.git)

```bash
  ./graphblas.sh
``

or

```bash
  make static CMAKE_OPTIONS='-DGRAPHBLAS_COMPACT=1 -DCMAKE_POSITION_INDEPENDENT_CODE=on'
  sudo make install
```

- building [RediSearch](https://github.com/RediSearch/RediSearch.git)

```bash
  ./redisearch.sh
```

- pytest - create virtualenv and install tests/requirements.txt

The virtual environment should be activated before running tests.

```bash
  python3 -m venv venv
  source venv/bin/activate
  pip install -r  tests/requirements.txt
```

### Testing

- run e2e tests with `pytest tests/test_e2e.py tests/test_functions.py -vv`
- run tck tests with `pytest tests/tck/test_tck.py -s`

There is an option to run only part of the TCK tests and stop on the first fail

```bash
TCK_INCLUDE=tests/tck/features/expressions/list pytest tests/tck/test_tck.py -s
```

To run all passing TCK tests use:

```bash
TCK_DONE=tck_done.txt pytest tests/tck/test_tck.py -s
```

- run unit tests with `cargo test -p graph`

- [benchmark](https://falkordb.github.io/falkordb-rs-next-gen/dev/bench/)
