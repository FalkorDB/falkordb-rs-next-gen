# /lint - Advanced Clippy Linting

Run Clippy with pedantic, nursery, and cargo lints enabled for comprehensive code quality checks.

## Usage

- `/lint` - Run clippy with advanced lint settings on all targets
- `/lint fix` - Automatically fix clippy warnings where possible

## Instructions

When the user invokes `/lint`:

1. **Standard clippy run** (default, no argument):
   Run clippy with comprehensive lint settings:
   ```bash
   CXX=clang++ cargo clippy --all-targets -- -W clippy::pedantic -W clippy::nursery -W clippy::cargo -A clippy::missing-errors-doc -A clippy::missing-panics-doc
   ```

2. **Auto-fix mode** (`fix` argument):
   Automatically apply clippy suggestions where possible:
   ```bash
   CXX=clang++ cargo clippy --all-targets --fix --allow-dirty --allow-staged -- -W clippy::pedantic -W clippy::nursery -W clippy::cargo -A clippy::missing-errors-doc -A clippy::missing-panics-doc
   ```

## Lint Configuration

The following clippy lints are enabled:

- **`clippy::pedantic`**: Extra pedantic lints for catching more potential issues
- **`clippy::nursery`**: Experimental lints that may find additional problems
- **`clippy::cargo`**: Lints specific to Cargo.toml and package management

The following lints are explicitly allowed (disabled):

- **`clippy::missing-errors-doc`**: Allow missing documentation for error cases
- **`clippy::missing-panics-doc`**: Allow missing documentation for panic cases

## Notes

- `CXX=clang++` is required for clippy due to GraphBLAS FFI bindings
- These lint settings are more strict than the default clippy configuration
- The `fix` mode will modify your code automatically, so review changes carefully
- Some nursery lints may produce false positives as they are experimental

If any lints fail, report the errors clearly and help fix them.
