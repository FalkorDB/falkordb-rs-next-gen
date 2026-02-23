# /test - Smart Test Runner

Run the appropriate tests based on what changed or what the user specifies.

## Usage

- `/test` - Run all unit tests
- `/test unit` - Run Rust unit tests only
- `/test e2e` - Run Python E2E tests
- `/test functions` - Run Python function tests
- `/test mvcc` - Run MVCC tests
- `/test concurrency` - Run concurrency tests
- `/test tck` - Run all passing TCK tests
- `/test tck <path>` - Run specific TCK feature tests
- `/test flow` - Run all flow tests

## Instructions

When the user invokes `/test`, determine which tests to run:

**Environment Setup:**
First, determine the correct Python virtual environment path:
- Inside devcontainer: `/data/venv`
- Outside devcontainer: `venv`

Check if `/data/venv` exists to detect devcontainer:
```bash
if [ -d "/data/venv" ]; then
  VENV_PATH="/data/venv"
else
  VENV_PATH="venv"
fi
```

1. **Unit tests (default)**: Run Rust unit tests
   ```bash
   cargo test -p graph
   ```

2. **E2E tests**: Run Python end-to-end tests
   ```bash
   source $VENV_PATH/bin/activate && pytest tests/test_e2e.py -vv
   ```

3. **Function tests**: Run Python function tests
   ```bash
   source $VENV_PATH/bin/activate && pytest tests/test_functions.py -vv
   ```

4. **MVCC tests**: Run MVCC concurrency tests
   ```bash
   source $VENV_PATH/bin/activate && pytest tests/test_mvcc.py -vv
   ```

5. **Concurrency tests**: Run concurrency tests
   ```bash
   source $VENV_PATH/bin/activate && pytest tests/test_concurrency.py -vv
   ```

6. **TCK tests**: Run Technology Compatibility Kit tests for Cypher language compliance
   - **All passing TCK tests** (no path argument):
     ```bash
     source $VENV_PATH/bin/activate && TCK_DONE=tck_done.txt pytest tests/tck/test_tck.py -s
     ```
   - **Specific feature tests** (with path argument):
     ```bash
     source $VENV_PATH/bin/activate && TCK_INCLUDE=<path> pytest tests/tck/test_tck.py -s
     ```
   Common TCK paths:
   - `tests/tck/features/expressions/list` - List expression tests
   - `tests/tck/features/expressions/map` - Map expression tests
   - `tests/tck/features/clauses/match` - MATCH clause tests
   - `tests/tck/features/clauses/return` - RETURN clause tests

   Notes:
   - TCK tests verify Cypher language compliance against the openCypher spec
   - `tck_done.txt` contains the list of passing test scenarios

7. **Flow tests**: Run the flow test suite which tests query flows against the running database
   ```bash
   ./flow.sh
   ```
   **Important**: The project must be compiled in debug mode (`cargo build`) before running flow tests.

   Notes:
   - Flow tests are located in `tests/flow/` directory
   - These tests verify complete query execution flows
   - Progress is tracked in `flow_tests_done.txt` and `flow_tests_todo.txt`
   - This skill automatically detects and uses the correct venv path

If no argument is provided, run the Rust unit tests (`cargo test -p graph`).

If tests fail, analyze the output and help the user understand what went wrong.
