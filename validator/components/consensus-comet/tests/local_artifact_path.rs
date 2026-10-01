// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/validate_local_artifact_directory.rs"]
mod validate_local_artifact_directory;

use std::path::PathBuf;

use validate_local_artifact_directory::validate_local_artifact_directory;

#[test]
fn accepts_ignored_local_artifacts_and_rejects_role_source_location() {
    let package = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = package.ancestors().nth(3).unwrap();
    let root = repository.join("local-tests/consensus-b0");
    std::fs::create_dir_all(&root).unwrap();
    let valid = tempfile::tempdir_in(root).unwrap();
    assert!(validate_local_artifact_directory(valid.path(), repository).is_ok());
    assert!(validate_local_artifact_directory(&package, repository).is_err());
}

#[cfg(unix)]
#[test]
fn rejects_symlink_escape_from_ignored_artifact_directory() {
    let package = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repository = package.ancestors().nth(3).unwrap();
    let root = repository.join("local-tests/consensus-b0");
    std::fs::create_dir_all(&root).unwrap();
    let valid = tempfile::tempdir_in(root).unwrap();
    let link = valid.path().join("outside");
    std::os::unix::fs::symlink(&package, &link).unwrap();
    assert!(validate_local_artifact_directory(&link, repository).is_err());
}
