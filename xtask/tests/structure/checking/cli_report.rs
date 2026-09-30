use crate::support::Fixture;
use std::{fs, process::Command};

#[test]
fn cli_returns_nonzero_and_writes_machine_readable_negative_report() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/encode_height.rs",
        "fn encode_height() {} fn decode_height() {}\n",
    );
    let reports = tempfile::tempdir().unwrap();
    let report_path = reports.path().join("negative.json");
    let result = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("check-structure")
        .arg("--root")
        .arg(fixture.root())
        .arg("--report")
        .arg(&report_path)
        .output()
        .unwrap();
    assert!(!result.status.success());
    let saved: serde_json::Value = serde_json::from_slice(&fs::read(report_path).unwrap()).unwrap();
    let printed: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(printed, saved);
    assert_eq!(saved["policy_version"], 1);
    assert_eq!(saved["current_bulk"], 0);
    assert!(!saved["violations"].as_array().unwrap().is_empty());
    let file = saved["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|file| file["path"] == "validator/src/encode_height.rs")
        .unwrap();
    assert_eq!(file["kind"], "behavior");
    assert_eq!(file["physical_lines"], 1);
    assert_eq!(
        file["operations"],
        serde_json::json!(["encode_height", "decode_height"])
    );
    assert!(!file["violations"].as_array().unwrap().is_empty());
}

#[test]
fn cli_positive_report_contains_all_first_party_files_and_exclusions() {
    let fixture = Fixture::new();
    fixture.write("validator/src/encode_height.rs", "fn encode_height() {}\n");
    fixture.write("Cargo.lock", "# Cargo-generated fixture.\n");
    fixture.policy("[[exclusions]]\npath = \"Cargo.lock\"\nkind = \"generated\"\nsource = \"Cargo\"\ngenerator = \"Cargo 1.97.1\"\nreason = \"Generated dependency fixture.\"\n");
    let result = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("check-structure")
        .arg("--root")
        .arg(fixture.root())
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert!(report["violations"].as_array().unwrap().is_empty());
    assert_eq!(report["exclusions"], serde_json::json!(["Cargo.lock"]));
    assert!(
        report["files"]
            .as_array()
            .unwrap()
            .iter()
            .any(|file| file["path"] == "validator/src/encode_height.rs")
    );
}

#[test]
fn missing_policy_cannot_return_success() {
    let fixture = Fixture::new();
    let result = Command::new(env!("CARGO_BIN_EXE_xtask"))
        .arg("check-structure")
        .arg("--root")
        .arg(fixture.root())
        .args(["--policy", "config/missing.toml"])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("missing.toml"));
}
