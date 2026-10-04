// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    fixtures::{stage_artifacts, witnesses},
    reopen_fixtures::private_archive,
};
use crate::sync::applied::{
    AppliedOwner, AppliedReader, AppliedWorkingReservation, open_segmented_applied_state_service,
    reserve_applied_working,
    segmented::tests::fixtures::{configuration, logical_chain},
};
use eve_state::Bytes;
use eve_storage::checkpoints::{
    checkpoint_store_manifest_bytes, proofs::checkpoint_proof_store_manifest_bytes,
};
use std::{fs::File, path::PathBuf};

pub(super) struct PendingFixture {
    pub(super) _database: tempfile::TempDir,
    pub(super) archive: tempfile::TempDir,
    pub(super) owner: AppliedOwner,
    pub(super) reader: AppliedReader,
    pub(super) target_encoding: Bytes,
    pub(super) _target_charge: AppliedWorkingReservation,
    pub(super) artifacts: ChargedCheckpointArtifacts,
}

pub(super) fn fixture() -> PendingFixture {
    let database = tempfile::tempdir().unwrap();
    let archive = private_archive();
    let chain = logical_chain(false);
    let (owner, reader) = open_segmented_applied_state_service(
        configuration(&database.path().join("store"), &chain),
        &chain.genesis,
    )
    .unwrap();
    let target_charge = reserve_applied_working(&reader, 4_096).unwrap();
    let target_encoding = eve_state::encode_state_version(&chain.commits[1].target).unwrap();
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[1],
        &witnesses(&chain, 1),
    )
    .unwrap();
    PendingFixture {
        _database: database,
        archive,
        owner,
        reader,
        target_encoding,
        _target_charge: target_charge,
        artifacts,
    }
}

pub(super) fn content_manifest(fixture: &PendingFixture) -> &[u8] {
    checkpoint_store_manifest_bytes(&fixture.artifacts.content.content_store)
}

pub(super) fn proof_manifest(fixture: &PendingFixture) -> &[u8] {
    checkpoint_proof_store_manifest_bytes(&fixture.artifacts.proof_store)
}

pub(super) fn namespace(staging: &tempfile::TempDir, id: &[u8; 32]) -> PathBuf {
    staging.path().join(hex::encode(id))
}

pub(super) fn copied_staging(
    fixture: &PendingFixture,
    proof: bool,
    pending_manifest: bool,
) -> tempfile::TempDir {
    let staging = private_archive();
    let id = if proof {
        &fixture.artifacts.proof_id
    } else {
        &fixture.artifacts.content.content_id
    };
    super::reopen_fixtures::copy_namespace(fixture.archive.path(), staging.path(), id);
    let path = namespace(&staging, id);
    std::fs::remove_file(path.join("complete.bin")).unwrap();
    if pending_manifest {
        std::fs::remove_file(path.join("manifest.bin")).unwrap();
    }
    staging
}

pub(super) fn completed_content_copy(
    fixture: &PendingFixture,
) -> (tempfile::TempDir, ChargedCompletedCheckpoint) {
    let staging = copied_staging(fixture, false, false);
    let transfer = begin_applied_checkpoint_transfer(
        &fixture.owner,
        &File::open(staging.path()).unwrap(),
        content_manifest(fixture),
        &fixture.target_encoding,
        super::fixtures::limits(),
    )
    .unwrap();
    let completed = complete_applied_checkpoint_transfer(transfer).unwrap();
    (staging, completed)
}
