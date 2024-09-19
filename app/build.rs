use std::{env, fs};
use std::process::Command;
use serde_json::Value;
fn main() {
    let relative = "../style";
    let mut style = fs::canonicalize(env::current_dir().unwrap().join(relative)).unwrap();
    style.push("emu_lib_ui");

    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .expect("Failed to execute cargo metadata");

    if !output.status.success() {
        panic!("cargo metadata command failed");
    }

    // Convert the output to a string
    let metadata_json = String::from_utf8(output.stdout).expect("Invalid UTF-8 in cargo metadata output");

    // Parse the JSON
    let metadata: Value = serde_json::from_str(&metadata_json).expect("Failed to parse JSON");
    // Extract dependencies and their paths
    if let Some(packages) = metadata["packages"].as_array() {
        for package in packages {
            let name = package["name"].as_str().unwrap_or("Unknown");
            if name == "emu_lib_ui" {
                let manifest_path = package["manifest_path"].as_str().unwrap_or("Unknown path");
                let manifest_dir = std::path::Path::new(manifest_path).parent().unwrap();
                let _output = Command::new("stylance")
                    .arg(".")
                    .arg("--output-dir")
                    .arg(style.clone())
                    .current_dir(manifest_dir)
                    .output()
                    .expect("Failed to execute stylance");
                println!("cargo:rerun-if-changed={}/stylance", style.to_str().unwrap());
                break;
            }
        }
    }
}