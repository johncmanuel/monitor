use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "dev".into());

    if profile == "release" {
        dotenvy::from_filename(".env.production").ok();
        println!("cargo:rerun-if-changed=.env.production");
    } else {
        dotenvy::from_filename(".env").ok();
        println!("cargo:rerun-if-changed=.env");
    }

    // Pass all env vars to rustc
    for (key, value) in env::vars() {
        println!("cargo:rustc-env={}={}", key, value);
    }

    tauri_build::build()
}
