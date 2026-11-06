fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-arg=-T{dir}/ereader.ld");
    println!("cargo:rerun-if-changed=ereader.ld");
}
