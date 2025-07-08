# The Rust implemenation of falkordb

[![Rust](https://github.com/FalkorDB/falkordb-rs-next-gen/actions/workflows/rust-push.yml/badge.svg)](https://github.com/FalkorDB/falkordb-rs-next-gen/actions/workflows/rust-push.yml)
[![codecov](https://codecov.io/gh/FalkorDB/falkordb-rs-next-gen/branch/main/graph/badge.svg)](https://codecov.io/gh/FalkorDB/falkordb-rs-next-gen)
[![license](https://img.shields.io/badge/license-Server_Side_Public_License-green)](https://github.com/FalkorDB/falkordb-rs-next-gen/blob/main/LICENSE)

#### Developer Guide

##### Build

dependencies:

- building [GraphBLAS](https://github.com/DrTimothyAldenDavis/GraphBLAS.git)

```bash
  make static CMAKE_OPTIONS='-DGRAPHBLAS_COMPACT=1 -DCMAKE_POSITION_INDEPENDENT_CODE=on'
  sudo make install
 ```

- pytest - create virtualenv and install tests/requirements.txt

```bash
  python3 -m venv venv
  source venv/bin/activate
  pip install -r  tests/requierments.txt
```

- build with `cargo build`
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

##### Query record

To be able to understand how query run in the database run the server
then use the record.py utility
```
python3 tests/record.py
```

It will show a UI with 3 parts
1. execution plan tree
   use the left and right keys to move between the query steps
2. progress of the query
3. input that you can record new query and visualize