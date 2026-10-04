// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::checkpoints::open_development_checkpoint_directories;
use std::os::unix::fs::{MetadataExt, PermissionsExt};

#[test]
fn ordinary_startup_creates_no_checkpoint_directory_and_explicit_bootstrap_creates_private_synced_roots()
 {
    let data = tempfile::tempdir().unwrap();
    assert!(
        open_development_checkpoint_directories(data.path(), false, None)
            .unwrap()
            .is_none()
    );
    assert!(!data.path().join(".checkpoints").exists());
    let directories = open_development_checkpoint_directories(data.path(), true, None)
        .unwrap()
        .unwrap();
    for file in [&directories.content, &directories.proofs] {
        assert_eq!(file.metadata().unwrap().mode() & 0o777, 0o700);
        assert_eq!(
            file.metadata().unwrap().uid(),
            rustix::process::geteuid().as_raw()
        );
    }
    assert_eq!(
        std::fs::metadata(data.path().join(".checkpoints"))
            .unwrap()
            .mode()
            & 0o777,
        0o700
    );
    assert!(
        open_development_checkpoint_directories(data.path(), false, None)
            .unwrap()
            .is_some()
    );
}

#[test]
fn a_public_or_symlinked_existing_checkpoint_container_is_refused_without_chmod_or_following() {
    let data = tempfile::tempdir().unwrap();
    let container = data.path().join(".checkpoints");
    std::fs::create_dir(&container).unwrap();
    std::fs::set_permissions(&container, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(open_development_checkpoint_directories(data.path(), true, None).is_err());
    assert_eq!(std::fs::metadata(&container).unwrap().mode() & 0o777, 0o755);
    let linked = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(data.path(), linked.path().join(".checkpoints")).unwrap();
    assert!(open_development_checkpoint_directories(linked.path(), false, None).is_err());
}
