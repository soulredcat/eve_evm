// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::paths::{prepare_output_directory, resolve_contained_path};
use std::path::Path;

#[test]
fn task_paths_reject_parent_absolute_and_nonlocal_output() {
    let root = tempfile::tempdir().unwrap();
    assert!(resolve_contained_path(root.path(), Path::new("../escape")).is_err());
    assert!(resolve_contained_path(root.path(), root.path()).is_err());
    assert!(prepare_output_directory(root.path(), Path::new("target/tools")).is_err());
    assert!(prepare_output_directory(root.path(), Path::new("local-tests")).is_err());
    assert!(prepare_output_directory(root.path(), Path::new("local-tests/tools")).is_ok());
}

#[cfg(unix)]
#[test]
fn task_paths_reject_existing_symlink_prefixes() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(outside.path(), root.path().join("local-tests")).unwrap();
    assert!(prepare_output_directory(root.path(), Path::new("local-tests/tools")).is_err());
}
