// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::supervision::bind_application_listener;

#[test]
fn owned_listener_cleanup_preserves_replacement_and_foreign_file() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("application.sock");
    let listener = bind_application_listener(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    std::fs::write(&path, b"foreign replacement").unwrap();
    drop(listener);
    assert_eq!(std::fs::read(&path).unwrap(), b"foreign replacement");
    assert!(bind_application_listener(&path).is_err());
}

#[test]
fn owned_listener_drop_removes_only_its_original_inode() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("application.sock");
    drop(bind_application_listener(&path).unwrap());
    assert!(!path.exists());
}
