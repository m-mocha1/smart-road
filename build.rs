fn main() {
    println!("cargo:rustc-link-search=native=target/debug"); // Or the absolute path if needed
    println!("cargo:rustc-link-lib=SDL2");
}
