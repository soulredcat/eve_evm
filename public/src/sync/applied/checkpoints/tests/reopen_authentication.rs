// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    fixtures::{stage_artifacts, witnesses},
    reopen_fixtures::{private_archive, recovery_config},
};
use crate::{
    persistence::segmented::checkpoints::{
        seal_checkpoint_base_record, try_receive_checkpoint_ack, try_submit_checkpoint_base,
    },
    sync::applied::{
        capture_applied_state, finish_applied_state_service, open_segmented_applied_state_service,
        segmented::{
            bind_local_state_version,
            tests::fixtures::{configuration, logical_chain},
        },
        types::AppliedBackend,
    },
};
use eve_storage::records::segmented::checkpoints::{
    CHECKPOINT_BASE_MAX_PAYLOAD_BYTES, CheckpointBaseLimits, CheckpointBaseMetadata,
    CheckpointBaseMode,
};
use std::fs::File;

#[test]
fn rehashed_complete_proof_bytes_and_a_real_synced_base_cannot_authorize_a_forged_history() {
    let database = tempfile::tempdir().unwrap();
    let archive = private_archive();
    let chain = logical_chain(false);
    let (owner, reader) = open_segmented_applied_state_service(
        configuration(&database.path().join("store"), &chain),
        &chain.genesis,
    )
    .unwrap();
    let mut proofs = witnesses(&chain, 1);
    let eve_finality_verifier::CheckpointWitness::Execution(first) = &mut proofs[0] else {
        panic!("execution")
    };
    first.native.frame.header.app_hash[0] ^= 1;
    // The private test creates a corrupt local archive through storage-only APIs.
    // This intentionally bypasses activation; no production finality capability is manufactured.
    let artifacts = stage_artifacts(
        &owner,
        &File::open(archive.path()).unwrap(),
        &chain.commits[1],
        &proofs,
    )
    .unwrap();
    let parent = capture_applied_state(&reader).unwrap();
    let metadata = CheckpointBaseMetadata {
        mode: CheckpointBaseMode::AuthenticatedImport,
        previous_opaque_cursor: owner.admitted_cursor,
        previous_logical_anchor: parent.segmented_position.unwrap().durable,
        target_height: 1,
        target_state_binding: bind_local_state_version(&chain.commits[1].target).unwrap(),
        snapshot_manifest_id: artifacts.content.content_id,
        snapshot_body_hash: artifacts.content.content.body_sha256,
        proof_manifest_id: artifacts.proof_id,
        proof_root: artifacts.proofs.stream_sha256,
        proof_genesis_height: 0,
        execution_start: 1,
        execution_end: 1,
        lookahead_height: 2,
        retained_start: 0,
        retained_end: 1,
    };
    let AppliedBackend::Segmented { worker, pool, .. } = &owner.backend else {
        panic!("segmented")
    };
    let record = seal_checkpoint_base_record(
        pool,
        metadata,
        &chain.commits[1].target,
        CheckpointBaseLimits {
            maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
        },
    )
    .unwrap();
    let ticket =
        try_submit_checkpoint_base(worker.as_ref().unwrap(), owner.admitted_cursor, record)
            .ok()
            .unwrap();
    let shutdown = finish_applied_state_service(owner);
    assert!(try_receive_checkpoint_ack(&ticket).unwrap().is_some());
    drop(ticket);
    drop(artifacts);
    drop(shutdown.repository.unwrap());
    assert!(matches!(
        open_segmented_applied_state_service_with_checkpoints(
            configuration(&database.path().join("store"), &chain),
            recovery_config(archive.path()),
            &chain.genesis
        ),
        Err(CheckpointAppliedError::Checkpoint(_))
    ));
}
