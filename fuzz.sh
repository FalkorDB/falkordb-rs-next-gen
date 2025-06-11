# build with fuzz feature
cargo build --features fuzz

# run tck tests to generate the corpus
TCK_DONE=tck_done.txt pytest tests/tck/test_tck.py -v

# run fuzz test
cargo fuzz run fuzz_target_runtime -- -max_total_time=30

# minimize the corpus
cargo fuzz cmin fuzz_target_runtime

# generate coverage data
cargo fuzz coverage fuzz_target_runtime 

# generate the coverage report
llvm-cov show -format=html -instr-profile=fuzz/coverage/fuzz_target_runtime/coverage.profdata -ignore-filename-regex=\.cargo/registry target/aarch64-apple-darwin/coverage/aarch64-apple-darwin/release/fuzz_target_runtime > cov.html