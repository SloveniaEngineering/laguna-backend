use std::path::PathBuf;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=migrations");
    println!("cargo:rerun-if-changed=assets");
    println!(
        "cargo:warning=Workspace root is {}",
        PathBuf::from(env::var("WORKSPACE_ROOT").unwrap())
            .canonicalize()
            .unwrap()
            .display()
    );
    if let Ok(_) = env::var("CARGO_FEATURE_DOX") {
        println!("cargo:warning=Copying logos to documentation. CARGO_FEATURE_DOX is set.");
        fs::copy("assets/logo.png", "target/doc/logo.png")
            .expect("Failed to copy crate logo when building documentation.");
        fs::copy("assets/favicon.ico", "target/doc/favicon.ico")
            .expect("Failed to copy crate favicon when building documentation.");
    }
}
