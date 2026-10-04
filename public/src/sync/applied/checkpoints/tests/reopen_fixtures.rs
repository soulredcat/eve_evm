// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    fixtures::{limits, stage_artifacts, witnesses},
};
use crate::sync::applied::{
    AppliedOwner, AppliedPublication, AppliedReader, capture_applied_state,
    finish_applied_state_service, open_segmented_applied_state_service, poll_applied_durability,
    segmented::tests::fixtures::{configuration, logical_chain},
    tests::import_fixtures::ImportChain,
    try_apply_recovery_bytes,
};
use std::{
    fs::File,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

pub(super) struct ReopenFixture {
    pub(super) database: tempfile::TempDir,
    pub(super) archive: tempfile::TempDir,
    pub(super) chain: ImportChain,
    pub(super) expected: Arc<AppliedPublication>,
    pub(super) content_id: [u8; 32],
    pub(super) proof_id: [u8; 32],
}

/// Match the production root contract explicitly; tempfile otherwise uses platform defaults.
pub(super) fn private_archive() -> tempfile::TempDir {
    use std::os::unix::fs::PermissionsExt;
    tempfile::Builder::new()
        .permissions(std::fs::Permissions::from_mode(0o700))
        .tempdir()
        .unwrap()
}

pub(super) fn recovery_config(archive: &Path) -> CheckpointRecoveryConfig {
    CheckpointRecoveryConfig {
        content_root: archive.to_owned(),
        proof_root: archive.to_owned(),
        limits: limits(),
        maximum_scan_records: 1_000,
    }
}

pub(super) fn activate(owner: &mut AppliedOwner) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_checkpoint_activation(owner).unwrap().is_none() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}

pub(super) fn append_tail(owner: &mut AppliedOwner, chain: &ImportChain) {
    try_apply_recovery_bytes(owner, &chain.records[1]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_durability(owner).unwrap().durable_recovery.0 != 2 {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}

pub(super) fn completed_checkpoint(with_tail: bool) -> ReopenFixture {
    let database = tempfile::tempdir().unwrap();
    let archive = private_archive();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&database.path().join("store"), &chain),
        &chain.genesis,
    )
    .unwrap();
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[1],
        &witnesses(&chain, 1),
    )
    .unwrap();
    let content_id = artifacts.content.content_id;
    let proof_id = artifacts.proof_id;
    let prepared = prepare_applied_checkpoint(artifacts, &chain.genesis).unwrap();
    start_applied_checkpoint_activation(&mut owner, prepared).unwrap();
    activate(&mut owner);
    if with_tail {
        append_tail(&mut owner, &chain);
    }
    let expected = capture_applied_state(&reader).unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.checkpoint_error.is_none());
    assert!(shutdown.acknowledgement_error.is_none());
    drop(shutdown.repository.unwrap());
    ReopenFixture {
        database,
        archive,
        chain,
        expected,
        content_id,
        proof_id,
    }
}

pub(super) fn reopen(
    fixture: &ReopenFixture,
) -> Result<(AppliedOwner, AppliedReader), CheckpointAppliedError> {
    open_segmented_applied_state_service_with_checkpoints(
        configuration(&fixture.database.path().join("store"), &fixture.chain),
        recovery_config(fixture.archive.path()),
        &fixture.chain.genesis,
    )
}

/// Copy only bounded files from private generated fixture namespaces; preserve the original bytes.
pub(super) fn copy_namespace(source: &Path, destination: &Path, id: &[u8; 32]) {
    let name = hex::encode(id);
    let source = source.join(&name);
    let target = destination.join(&name);
    std::fs::create_dir(&target).unwrap();
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let entries: Vec<_> = std::fs::read_dir(source)
        .unwrap()
        .map(Result::unwrap)
        .collect();
    assert!(entries.len() <= 64);
    for entry in entries {
        assert!(entry.file_type().unwrap().is_file());
        assert!(entry.metadata().unwrap().len() <= 262_144);
        std::fs::copy(entry.path(), target.join(entry.file_name())).unwrap();
    }
}
