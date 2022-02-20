use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}/native", project_dir);
    println!("cargo:rustc-link-lib=static=libMinHook");
}
