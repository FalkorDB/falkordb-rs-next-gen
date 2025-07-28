fn main() {
    println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/lib");
    println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/lib/c++");
    println!("cargo:rustc-link-search=/usr/lib/llvm-20/lib");
    println!("cargo:rustc-link-search=/usr/lib/llvm-20/lib/c++");

    println!("cargo:rustc-link-lib=omp");

    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=graphblas");

    println!(
        "cargo:rustc-link-search=native=redisearch/RediSearch/bin/macos-arm64v8-release/search-static"
    );
    println!(
        "cargo:rustc-link-search=native=redisearch/RediSearch/bin/macos-arm64v8-release/search-static/deps/VectorSimilarity/src/VecSim"
    );
    println!(
        "cargo:rustc-link-search=native=redisearch/RediSearch/bin/macos-arm64v8-release/search-static/deps/VectorSimilarity/src/VecSim/spaces"
    );
    println!(
        "cargo:rustc-link-search=native=redisearch/RediSearch/bin/linux-x64-release/search-static"
    );
    println!(
        "cargo:rustc-link-search=native=redisearch/RediSearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim"
    );
    println!(
        "cargo:rustc-link-search=native=redisearch/RediSearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim/spaces"
    );
    println!("cargo:rustc-link-lib=static=c++");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-lib=static=VectorSimilarity");
    println!("cargo:rustc-link-lib=static=VectorSimilaritySpaces");
    println!("cargo:rustc-link-lib=static=VectorSimilaritySpaces_no_optimization");
    println!("cargo:rustc-link-lib=static=redisearch-static");
}
