// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::{CheckpointChunkStatus, CheckpointError, proofs::*};

#[test]
fn partial_witness_resume_repairs_only_invalid_entry_and_keeps_valid_prefix() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_checkpoint_proof_witness(&mut transfer, 0, &fixture.blobs[0], fixture.io).unwrap();
    drop(transfer);
    // Explicit local partial staging fault, not evidence of hardware power loss.
    std::fs::write(witness_path(&directory, &fixture, 1), b"partial").unwrap();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let observed = checkpoint_proof_initial_resume_observation(&transfer);
    assert_eq!(
        (
            observed.verified_chunks,
            observed.corrupt_chunks,
            observed.missing_chunks
        ),
        (1, 1, 1)
    );
    assert!(matches!(
        repair_invalid_checkpoint_proof_witness(&mut transfer, 0, fixture.io),
        Err(CheckpointError::ValidEntry)
    ));
    repair_invalid_checkpoint_proof_witness(&mut transfer, 1, fixture.io).unwrap();
    assert_eq!(
        observe_checkpoint_proof_witness(&transfer, 1, fixture.io).unwrap(),
        CheckpointChunkStatus::Missing
    );
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
}

#[test]
fn missing_or_corrupt_witness_and_changed_stream_digest_cannot_publish_complete_marker() {
    let mut fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_checkpoint_proof_witness(&mut transfer, 0, &fixture.blobs[0], fixture.io).unwrap();
    assert!(complete_checkpoint_proof_transfer(transfer, fixture.io).is_err());
    assert!(
        !namespace(&directory, &fixture)
            .join("complete.bin")
            .exists()
    );
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    corrupt_witness(&directory, &fixture, 1);
    assert!(matches!(
        complete_checkpoint_proof_transfer(transfer, fixture.io),
        Err(CheckpointError::CorruptChunk)
    ));
    fixture.manifest[112] ^= 1;
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    assert!(matches!(
        complete_checkpoint_proof_transfer(transfer, fixture.io),
        Err(CheckpointError::CorruptChunk)
    ));
    assert!(
        !namespace(&directory, &fixture)
            .join("complete.bin")
            .exists()
    );
}

#[test]
fn explicit_pending_repairs_remove_partial_entries_but_preserve_valid_manifest() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let pending = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&pending, &fixture.manifest[..24]).unwrap();
    assert!(begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    repair_invalid_checkpoint_proof_pending(
        &root,
        &manifest(&fixture),
        CheckpointProofPendingKind::Manifest,
        fixture.metadata,
    )
    .unwrap();
    std::fs::write(&pending, &fixture.manifest).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_proof_pending(
            &root,
            &manifest(&fixture),
            CheckpointProofPendingKind::Manifest,
            fixture.metadata
        ),
        Err(CheckpointError::ValidEntry)
    ));
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(transfer);
    let complete_pending = namespace(&directory, &fixture).join("complete.pending");
    std::fs::write(&complete_pending, b"EVE_PROOF").unwrap();
    repair_invalid_checkpoint_proof_pending(
        &root,
        &manifest(&fixture),
        CheckpointProofPendingKind::Completion,
        fixture.metadata,
    )
    .unwrap();
    let transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
    assert!(matches!(
        repair_invalid_checkpoint_proof_pending(
            &root,
            &manifest(&fixture),
            CheckpointProofPendingKind::Completion,
            fixture.metadata
        ),
        Err(CheckpointError::AlreadyComplete)
    ));
}

#[test]
fn completed_corruption_is_detected_without_authorizing_repair() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    let completed = complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap();
    corrupt_witness(&directory, &fixture, 0);
    assert!(matches!(
        read_checkpoint_proof_witness(
            &completed,
            0,
            required_checkpoint_proof_witness_reservation(&completed, 0).unwrap()
        ),
        Err(CheckpointError::CorruptChunk)
    ));
    drop(completed);
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_proof_witness(&mut transfer, 0, fixture.io),
        Err(CheckpointError::AlreadyComplete)
    ));
}

#[test]
fn foreign_manifest_outside_receiver_resource_policy_is_preserved_by_repair() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let mut foreign = fixture.manifest.clone();
    let reference = 144 + fixture.version.len() + 56;
    foreign[reference + 16..reference + 24].copy_from_slice(&4_096_u64.to_be_bytes());
    foreign[40..48].copy_from_slice(&(128_u64 + 4_096 + 1_024).to_be_bytes());
    let path = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&path, &foreign).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_proof_pending(
            &root,
            &manifest(&fixture),
            CheckpointProofPendingKind::Manifest,
            fixture.metadata
        ),
        Err(CheckpointError::ValidEntry)
    ));
    assert_eq!(std::fs::read(path).unwrap(), foreign);
}
