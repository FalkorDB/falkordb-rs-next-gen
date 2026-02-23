# /build - Build and Lint

Build the project and run all linting checks.

## Usage

- `/build` - Full build with format and lint checks
- `/build quick` - Just compile, skip lint
- `/build lint` - Only run lint checks

## Instructions

When the user invokes `/build`:

1. **Full build** (default, no argument):
   Run these commands in sequence:
   ```bash
   cargo fmt --all -- --check
   ```
   ```bash
   cargo build
   ```
   ```bash
   CXX=clang++ cargo clippy --all-targets
   ```

2. **Quick build** (`quick` argument):
   ```bash
   cargo build
   ```

3. **Lint only** (`lint` argument):
   ```bash
   cargo fmt --all -- --check && CXX=clang++ cargo clippy --all-targets
   ```

## Notes

- `CXX=clang++` is required for clippy due to GraphBLAS FFI bindings
- In the devcontainer, `CXX=clang++-21` is automatically set
- Format check uses the project's `rustfmt.toml` configuration
- For release builds, use `cargo build --release`
- This skill works both inside and outside the devcontainer

If any step fails, report the errors clearly and help fix them.
