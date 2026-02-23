# /coverage - Code Coverage Analysis

Run comprehensive code coverage analysis including Rust unit tests, Python E2E tests, and flow tests.

## Usage

- `/coverage` - Run full coverage analysis with all tests
- `/coverage unit` - Run coverage for Rust unit tests only
- `/coverage quick` - Run coverage without flow tests (faster)

## Instructions

When the user invokes `/coverage`, the skill automatically detects whether it's running inside or outside a devcontainer:

### Environment Detection

First, check if running inside a devcontainer:
```bash
if [ -f /.dockerenv ] || [ -n "$REMOTE_CONTAINERS" ]; then
  echo "Running inside devcontainer"
  INSIDE_CONTAINER=true
else
  echo "Running outside devcontainer"
  INSIDE_CONTAINER=false
fi
```

### Coverage Execution

1. **Full coverage** (default, no argument):

   **If inside devcontainer:**
   ```bash
   # Clean up old coverage data
   find . -name "*.profraw" -delete
   rm -f cov.profdata codecov.txt codecov.txt.all

   # Build with instrumentation
   RUSTFLAGS="-C instrument-coverage" cargo build

   # Run Rust unit tests with coverage
   RUSTFLAGS="-C instrument-coverage" cargo test -p graph

   # Activate Python venv and run Python tests
   source /data/venv/bin/activate
   pytest tests/test_e2e.py tests/test_functions.py tests/test_mvcc.py tests/test_concurrency.py -vv

   # Run flow tests
   ./flow.sh

   # Run TCK tests
   TCK_DONE=tck_done.txt pytest tests/tck/test_tck.py -s

   # Merge and export coverage data
   llvm-profdata-21 merge --sparse $(find . -name "*.profraw") -o cov.profdata
   llvm-cov-21 export --format=lcov --instr-profile cov.profdata target/debug/libfalkordb.so > codecov.txt.all
   lcov --ignore-errors unused -r codecov.txt.all -o codecov.txt

   # Display summary
   llvm-cov-21 report --instr-profile cov.profdata target/debug/libfalkordb.so
   ```

   **If outside devcontainer:**
   ```bash
   # Use docker to run coverage inside the devcontainer
   docker build -t falkordb-dev -f .devcontainer/Dockerfile .
   docker run --rm -v $(pwd):/workspace -w /workspace falkordb-dev bash -c "
     rm -f dump.rdb
     find . -name '*.profraw' -delete
     rm -f cov.profdata codecov.txt codecov.txt.all
     RUSTFLAGS='-C instrument-coverage' cargo build
     RUSTFLAGS='-C instrument-coverage' cargo test -p graph
     source /data/venv/bin/activate
     pytest tests/test_e2e.py tests/test_functions.py tests/test_mvcc.py tests/test_concurrency.py -vv
     ./flow.sh
     TCK_DONE=tck_done.txt pytest tests/tck/test_tck.py -s
     llvm-profdata-21 merge --sparse \$(find . -name '*.profraw') -o cov.profdata
     llvm-cov-21 export --format=lcov --instr-profile cov.profdata target/debug/libfalkordb.so > codecov.txt.all
     lcov --ignore-errors unused -r codecov.txt.all -o codecov.txt
     llvm-cov-21 report --instr-profile cov.profdata target/debug/libfalkordb.so
   "
   ```

2. **Unit tests only** (`unit` argument):

   **If inside devcontainer:**
   ```bash
   find . -name "*.profraw" -delete
   rm -f cov.profdata codecov.txt codecov.txt.all
   RUSTFLAGS="-C instrument-coverage" cargo build
   RUSTFLAGS="-C instrument-coverage" cargo test -p graph
   llvm-profdata-21 merge --sparse $(find . -name "*.profraw") -o cov.profdata
   llvm-cov-21 export --format=lcov --instr-profile cov.profdata target/debug/libfalkordb.so > codecov.txt.all
   lcov --ignore-errors unused -r codecov.txt.all -o codecov.txt
   llvm-cov-21 report --instr-profile cov.profdata target/debug/libfalkordb.so
   ```

   **If outside devcontainer:**
   ```bash
   docker build -t falkordb-dev -f .devcontainer/Dockerfile .
   docker run --rm -v $(pwd):/workspace -w /workspace falkordb-dev bash -c "
     find . -name '*.profraw' -delete
     rm -f cov.profdata codecov.txt codecov.txt.all
     RUSTFLAGS='-C instrument-coverage' cargo build
     RUSTFLAGS='-C instrument-coverage' cargo test -p graph
     llvm-profdata-21 merge --sparse \$(find . -name '*.profraw') -o cov.profdata
     llvm-cov-21 export --format=lcov --instr-profile cov.profdata target/debug/libfalkordb.so > codecov.txt.all
     lcov --ignore-errors unused -r codecov.txt.all -o codecov.txt
     llvm-cov-21 report --instr-profile cov.profdata target/debug/libfalkordb.so
   "
   ```

3. **Quick coverage** (`quick` argument) - Same as full but skip flow tests:

   **If inside devcontainer:**
   ```bash
   find . -name "*.profraw" -delete
   rm -f cov.profdata codecov.txt codecov.txt.all
   RUSTFLAGS="-C instrument-coverage" cargo build
   RUSTFLAGS="-C instrument-coverage" cargo test -p graph
   source /data/venv/bin/activate
   pytest tests/test_e2e.py tests/test_functions.py tests/test_mvcc.py tests/test_concurrency.py -vv
   llvm-profdata-21 merge --sparse $(find . -name "*.profraw") -o cov.profdata
   llvm-cov-21 export --format=lcov --instr-profile cov.profdata target/debug/libfalkordb.so > codecov.txt.all
   lcov --ignore-errors unused -r codecov.txt.all -o codecov.txt
   llvm-cov-21 report --instr-profile cov.profdata target/debug/libfalkordb.so
   ```

   **If outside devcontainer:**
   ```bash
   docker build -t falkordb-dev -f .devcontainer/Dockerfile .
   docker run --rm -v $(pwd):/workspace -w /workspace falkordb-dev bash -c "
     rm -f dump.rdb
     find . -name '*.profraw' -delete
     rm -f cov.profdata codecov.txt codecov.txt.all
     RUSTFLAGS='-C instrument-coverage' cargo build
     RUSTFLAGS='-C instrument-coverage' cargo test -p graph
     source /data/venv/bin/activate
     pytest tests/test_e2e.py tests/test_functions.py tests/test_mvcc.py tests/test_concurrency.py -vv
     llvm-profdata-21 merge --sparse \$(find . -name '*.profraw') -o cov.profdata
     llvm-cov-21 export --format=lcov --instr-profile cov.profdata target/debug/libfalkordb.so > codecov.txt.all
     lcov --ignore-errors unused -r codecov.txt.all -o codecov.txt
     llvm-cov-21 report --instr-profile cov.profdata target/debug/libfalkordb.so
   "
   ```

## Output Files

After running coverage, the following files will be generated:

- `cov.profdata` - Merged LLVM profile data
- `codecov.txt.all` - Raw LCOV coverage data
- `codecov.txt` - Filtered LCOV coverage data (recommended for upload to Codecov)
- `*.profraw` - Individual profile data files (can be cleaned up)

## Notes

- Code coverage uses LLVM's source-based code coverage instrumentation
- `llvm-profdata-21` and `llvm-cov-21` are required (provided in devcontainer)
- Flow tests require the debug build to be completed first
- Coverage data includes both Rust and integration test coverage
- When running outside devcontainer, the first run may take longer as it builds the container image
- The coverage report shows line coverage, region coverage, and function coverage

If coverage analysis fails, report the errors clearly and help diagnose the issue.
