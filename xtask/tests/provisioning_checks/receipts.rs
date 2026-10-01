// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    artifacts::compute_artifact_digest,
    receipts::validate_receipt_record,
    types::{ArtifactPin, ProvisionedArtifact},
};
use std::path::Path;

#[test]
fn existing_receipt_rejects_changed_binary_source_recipe_and_path() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::create_dir(directory.path().join("tool")).unwrap();
    let executable = directory.path().join("tool/engine");
    std::fs::write(&executable, b"public test tool bytes").unwrap();
    let executable = executable.canonicalize().unwrap();
    let pin = ArtifactPin {
        version: "1".into(),
        source_identity: "fixture".into(),
        url: "https://go.dev/fixture".into(),
        sha256: "a".repeat(64),
        archive_root: None,
        executable: "engine".into(),
        binary_sha256: None,
        expected_version: "1".into(),
        license: "test-only".into(),
    };
    let receipt = ProvisionedArtifact {
        name: "tool".into(),
        executable: executable.clone(),
        source_sha256: pin.sha256.clone(),
        executable_sha256: compute_artifact_digest(&executable).unwrap(),
        version_output: "1".into(),
        recipe_identity: "fixed-recipe".into(),
    };
    assert!(
        validate_receipt_record(directory.path(), "tool", &pin, "fixed-recipe", &receipt).is_ok()
    );
    assert!(
        validate_receipt_record(directory.path(), "tool", &pin, "wrong-recipe", &receipt).is_err()
    );
    let mut changed = receipt.clone();
    changed.source_sha256 = "b".repeat(64);
    assert!(
        validate_receipt_record(directory.path(), "tool", &pin, "fixed-recipe", &changed).is_err()
    );
    changed = receipt.clone();
    changed.executable = Path::new("/outside/engine").to_owned();
    assert!(
        validate_receipt_record(directory.path(), "tool", &pin, "fixed-recipe", &changed).is_err()
    );
    std::fs::write(&executable, b"tampered tool bytes").unwrap();
    assert!(
        validate_receipt_record(directory.path(), "tool", &pin, "fixed-recipe", &receipt).is_err()
    );
}
