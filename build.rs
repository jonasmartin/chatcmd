use std::{env, path::Path};

fn main() {
    // Tell Rust that dotenv_available is a valid cfg condition
    println!("cargo::rustc-check-cfg=cfg(dotenv_available)");

    let dotenv_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(".env");

    if dotenv_path.exists() {
        // Emit a custom flag if .env exists
        println!("cargo:rustc-cfg=dotenv_available");
    }
    println!("cargo:rerun-if-changed=.env");
}
