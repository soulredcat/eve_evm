// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::{path::Path, process::Command};

#[test]
fn t_l06_actual_component_packages_keep_their_canonical_role_owner() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let result = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--offline",
            "--locked",
            "--no-deps",
            "--format-version",
            "1",
        ])
        .current_dir(repository)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    let packages = metadata["packages"].as_array().unwrap();
    for (name, manifest) in [
        ("eve-storage", "public/components/recovery-store/Cargo.toml"),
        (
            "eve-crypto",
            "validator/components/authentication/Cargo.toml",
        ),
        ("eve-evm", "validator/components/execution/Cargo.toml"),
    ] {
        let package = packages
            .iter()
            .find(|package| package["name"] == name)
            .unwrap();
        assert_eq!(
            Path::new(package["manifest_path"].as_str().unwrap())
                .canonicalize()
                .unwrap(),
            repository.join(manifest).canonicalize().unwrap(),
        );
        for dependency in package["dependencies"].as_array().unwrap() {
            if let Some(path) = dependency["path"].as_str() {
                let resolved = Path::new(path).canonicalize().unwrap();
                assert!(!resolved.starts_with(repository.join("master")));
            }
        }
    }
}
