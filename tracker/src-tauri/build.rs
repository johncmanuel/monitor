use std::env;

fn main() {
    dotenvy::dotenv().ok();

    println!("cargo:rerun-if-changed=.env");

    for (key, value) in env::vars() {
        println!("cargo:rustc-env={}={}", key, value);
    }

    tauri_build::build()
}
