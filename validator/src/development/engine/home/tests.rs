// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[test]
fn lease_drop_explicitly_unlocks_even_when_the_open_file_description_is_duplicated() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let lease = super::acquire_engine_lease(directory.path()).unwrap();
    let duplicate = lease.file.try_clone().unwrap();
    assert!(super::acquire_engine_lease(directory.path()).is_err());
    drop(lease);
    let reacquired = super::acquire_engine_lease(directory.path()).unwrap();
    drop(duplicate);
    assert!(super::acquire_engine_lease(directory.path()).is_err());
    drop(reacquired);
    assert!(super::acquire_engine_lease(directory.path()).is_ok());
}
