use std::process::Command;

fn main() {
    let status = Command::new("cargo").args(&["build", "--target=wasm32-unknown-unknown", "--manifest-path=../ee/Cargo.toml"])
                    .arg(&format!("--target-dir=../target/ewasm"))
                    .status().unwrap();

    if status.success() {
        println!("eWasm build successful");
    } else {
        println!("eWasm build failed..\n{}", status);
    }
}