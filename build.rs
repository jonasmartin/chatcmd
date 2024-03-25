use std::{env, path::Path};

fn main() {
    let dotenv_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(".env");
    
    if dotenv_path.exists() {
        // Emit a custom flag if .env exists
        println!("cargo:rustc-cfg=dotenv_available");

    }
    println!("cargo:rerun-if-changed=.env");
}
