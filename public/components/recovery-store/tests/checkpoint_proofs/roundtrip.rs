// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::{CheckpointError, proofs::*};

#[test]
fn synced_ordered_opaque_witnesses_reopen_and_read_one_exact_charged_blob() {
    let fixture = fixture();
    let (_directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert_eq!(
        checkpoint_proof_initial_resume_observation(&transfer).missing_chunks,
        3
    );
    write_all(&mut transfer, &fixture);
    let completed = complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap();
    assert_eq!(
        checkpoint_proof_store_manifest_bytes(&completed),
        fixture.manifest
    );
    for index in 0..3 {
        let witness = read_checkpoint_proof_witness(
            &completed,
            index,
            required_checkpoint_proof_witness_reservation(&completed, index).unwrap(),
        )
        .unwrap();
        assert_eq!(
            checkpoint_proof_witness_bytes(&witness),
            fixture.blobs[index]
        );
        assert_eq!(
            checkpoint_proof_witness_reference(&witness),
            fixture.references[index]
        );
    }
    drop(completed);
    let transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert_eq!(
        checkpoint_proof_initial_resume_observation(&transfer).verified_chunks,
        3
    );
    drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
}

#[test]
fn complete_proof_witnesses_cannot_be_rewritten_or_repaired() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
    let before = std::fs::read(witness_path(&directory, &fixture, 0)).unwrap();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert!(matches!(
        write_checkpoint_proof_witness(&mut transfer, 0, &fixture.blobs[0], fixture.io),
        Err(CheckpointError::AlreadyComplete)
    ));
    assert!(matches!(
        repair_invalid_checkpoint_proof_witness(&mut transfer, 0, fixture.io),
        Err(CheckpointError::AlreadyComplete)
    ));
    assert_eq!(
        std::fs::read(witness_path(&directory, &fixture, 0)).unwrap(),
        before
    );
}

#[test]
fn stream_checksum_is_transactional_for_wrong_order_and_detects_incomplete_sequence() {
    let fixture = fixture();
    let mut stream = begin_checkpoint_proof_stream_hash(
        &fixture.snapshot,
        &fixture.body,
        &fixture.target,
        &fixture.limits,
        fixture.metadata,
    )
    .unwrap();
    assert!(
        update_checkpoint_proof_stream_hash(&mut stream, &fixture.references[1], &fixture.blobs[1])
            .is_err()
    );
    update_checkpoint_proof_stream_hash(&mut stream, &fixture.references[0], &fixture.blobs[0])
        .unwrap();
    assert!(matches!(
        finish_checkpoint_proof_stream_hash(stream),
        Err(CheckpointError::MissingChunk)
    ));
}
