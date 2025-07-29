use std::fs;

fn main() {
    println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/lib");
    println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/lib/c++");
    println!("cargo:rustc-link-search=/usr/lib/llvm-20/lib");
    println!("cargo:rustc-link-search=/usr/lib/llvm-20/lib/c++");

    println!("cargo:rustc-link-lib=omp");

    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=graphblas");

    println!("cargo:rustc-link-lib=static=c++");
    println!("cargo:rustc-link-lib=static=c++abi");

    #[cfg(target_os = "macos")]
    {
        println!(
            "cargo:rustc-link-search=native=redisearch/RediSearch/bin/macos-arm64v8-release/search-static"
        );
        println!(
            "cargo:rustc-link-search=native=redisearch/RediSearch/bin/macos-arm64v8-release/search-static/deps/VectorSimilarity/src/VecSim"
        );
        println!(
            "cargo:rustc-link-search=native=redisearch/RediSearch/bin/macos-arm64v8-release/search-static/deps/VectorSimilarity/src/VecSim/spaces"
        );
    }

    #[cfg(target_os = "linux")]
    {
        println!(
            "cargo:rustc-link-search=native=redisearch/RediSearch/bin/linux-x64-release/search-static"
        );
        println!(
            "cargo:rustc-link-search=native=redisearch/RediSearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim"
        );
        println!(
            "cargo:rustc-link-search=native=redisearch/RediSearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim/spaces"
        );
        println!(
            "cargo:rustc-link-search=native=/data/redisearch/bin/linux-x64-release/search-static"
        );
        println!(
            "cargo:rustc-link-search=native=/data/redisearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim"
        );
        println!(
            "cargo:rustc-link-search=native=/data/redisearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim/spaces"
        );
    }

    #[cfg(target_os = "linux")]
    let paths = fs::read_dir("../redisearch/RediSearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim/spaces").unwrap_or_else(|_| {
        fs::read_dir("/data/redisearch/bin/linux-x64-release/search-static/deps/VectorSimilarity/src/VecSim/spaces").unwrap()
    });

    #[cfg(target_os = "macos")]
    let paths = fs::read_dir("../redisearch/RediSearch/bin/macos-arm64v8-release/search-static/deps/VectorSimilarity/src/VecSim/spaces").unwrap();

    for path in paths.flatten() {
        let path = path.path();
        let name = &path.file_name().unwrap().to_str().unwrap();
        let name = &name[3..&name.len() - 2];
        let extention = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if extention.eq_ignore_ascii_case("a") {
            println!("cargo:rustc-link-lib=static={name}");
        }
    }
    println!("cargo:rustc-link-lib=static=VectorSimilarity");
    println!("cargo:rustc-link-lib=static=redisearch-static");
}
