// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    AppliedOwner, AppliedReader,
    checkpoints::tests::fixtures::{stage_artifacts, witnesses},
    observe_applied_storage, reserve_applied_snapshot_staging, reserve_applied_storage_read,
    tests::import_fixtures::ImportChain,
};
use std::{fs::File, os::unix::fs::PermissionsExt};

pub(super) fn assert_storage_saturation(
    owner: &AppliedOwner,
    reader: &AppliedReader,
    chain: &ImportChain,
) {
    let directory = tempfile::Builder::new()
        .permissions(std::fs::Permissions::from_mode(0o700))
        .tempdir()
        .unwrap();
    let file = File::open(directory.path()).unwrap();
    let proofs = witnesses(chain, 1);
    let first = reserve_applied_storage_read(reader).unwrap();
    let second = reserve_applied_storage_read(reader).unwrap();
    assert!(stage_artifacts(owner, &file, &chain.commits[1], &proofs).is_err());
    assert_eq!(directory.path().read_dir().unwrap().count(), 0);
    drop(first);
    drop(second);
    let staging = reserve_applied_snapshot_staging(reader, 16 * 1_048_576).unwrap();
    assert!(stage_artifacts(owner, &file, &chain.commits[1], &proofs).is_err());
    assert_eq!(directory.path().read_dir().unwrap().count(), 0);
    drop(staging);
    let observed = observe_applied_storage(reader).unwrap();
    assert_eq!(observed.active_reads, 0);
    assert_eq!(observed.reserved_staging_bytes, 0);
    assert_eq!(observed.peak_reads, 2);
    assert_eq!(observed.staging_limit, 16 * 1_048_576);
}
