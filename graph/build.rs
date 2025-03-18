fn main() {
    println!("cargo:rustc-link-search=/opt/homebrew/opt/llvm/lib");
    println!("cargo:rustc-link-search=/usr/lib/llvm-19/lib");
    println!("cargo:rustc-link-lib=omp");

    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=graphblas");
}
