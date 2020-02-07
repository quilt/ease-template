use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let status = Command::new("cargo").args(&["build", "--target=wasm32-unknown-unknown", "--manifest-path=../ee/Cargo.toml"])
                    .arg(&format!("--target-dir={}",out_dir))
                    .status().unwrap();

    if status.success() {
        println!("eWasm build successful");
    } else {
        println!("eWasm build failed..\n{}", status);
    }
}