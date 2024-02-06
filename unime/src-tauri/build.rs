use std::io::Read;

fn main() {
    // Get the latest version of tauri-build
    let output = std::process::Command::new("cargo")
        .args(&["search", "tauri-build", "--limit", "1"])
        .output()
        .expect("Failed to execute cargo search");

    let latest_version = String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .map(|s| s.trim())
        .unwrap_or_default()
        .split('#')
        .next()
        .unwrap()
        .split('=')
        .last()
        .unwrap()
        .trim()
        .trim_matches('"')
        .to_string();

    // Specify the expected version
    let mut toml_file = std::fs::File::open("Cargo.toml").expect("Failed to open Cargo.toml");

    let mut toml_content = String::new();
    toml_file
        .read_to_string(&mut toml_content)
        .expect("Failed to read Cargo.toml");

    // Parse the TOML content
    let toml_table: toml::Value = toml::from_str(&toml_content).expect("Failed to parse Cargo.toml");

    let expected_version = toml_table["build-dependencies"]["tauri-build"]["version"]
        .as_str()
        .unwrap()
        .split('=')
        .last()
        .unwrap();

    println!("cargo:warning={}", latest_version);
    println!("cargo:warning={}", expected_version);

    // Check if the specified version is outdated
    if expected_version != latest_version {
        println!(
            "cargo:warning=Your specified version {} is outdated. Latest version is {}.",
            expected_version, latest_version
        );
        panic!("Please update your specified version to the latest version.");
    } else {
        println!(
            "cargo:warning=Your specified version {} is up-to-date.",
            expected_version
        );
    }

    tauri_build::build()
}
