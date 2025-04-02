# matrixdb-rs

[![codecov](https://codecov.io/gh/AviAvni/matrixdb-rs/branch/master/graph/badge.svg?token=WTWEHTBOQF)](https://codecov.io/gh/AviAvni/matrixdb-rs)
[![Rust](https://github.com/AviAvni/matrixdb-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/AviAvni/matrixdb-rs/actions/workflows/rust.yml)

![coverage](https://codecov.io/gh/AviAvni/matrixdb-rs/branch/master/graphs/tree.svg?token=WTWEHTBOQF)


#### Developer Guide

##### Build

dependencies:
- building [GraphBLAS](https://github.com/DrTimothyAldenDavis/GraphBLAS.git)
```bash
  make static CMAKE_OPTIONS='-DGRAPHBLAS_COMPACT=1 -DCMAKE_POSITION_INDEPENDENT_CODE=on'
  sudo make install
 ```
- pytest - create virtualenv and install tests/requirement.txt
```bash
  python3 -m venv venv
  source venv/bin/activate
  pip install -r  tests/requierment.txt
```
- build with `cargo build`
- run e2e tests with `pytest tests/test_e2e.py`
- run tck tests with `pytest tests/tck/test_tck.py`
