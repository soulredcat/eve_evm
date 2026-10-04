// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn actual_synced_checkpoint_reopens_exact_bytes_and_uses_sealed_canonical_decode() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let initial = checkpoint_initial_resume_observation(&transfer);
    assert_eq!(initial.missing_chunks, fixture.body.len().div_ceil(512));
    write_all(&mut transfer, &fixture);
    let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
    assert_eq!(
        checkpoint_target_encoding(&completed),
        fixture.version.as_ref()
    );
    let body = read_checkpoint_body(
        &completed,
        required_checkpoint_body_reservation(&completed).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint_body_bytes(&body), fixture.body);
    assert_eq!(
        eve_state::decode_preflight_state_commit(&preflight_checkpoint_body(&body).unwrap())
            .unwrap(),
        fixture.commit
    );
    drop(body);
    drop(completed);
    let transfer = begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert_eq!(
        checkpoint_initial_resume_observation(&transfer).verified_chunks,
        initial.missing_chunks
    );
    let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
    let body = read_checkpoint_body(
        &completed,
        required_checkpoint_body_reservation(&completed).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint_body_bytes(&body), fixture.body);
    assert!(
        namespace(&directory, &fixture)
            .join("complete.bin")
            .is_file()
    );
}

#[test]
fn completed_files_are_immutable_to_write_and_repair_apis() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
    let before = std::fs::read(chunk_path(&directory, &fixture, 0)).unwrap();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert!(matches!(
        write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io),
        Err(CheckpointError::AlreadyComplete)
    ));
    assert!(matches!(
        repair_invalid_checkpoint_chunk(&mut transfer, 0, fixture.io),
        Err(CheckpointError::AlreadyComplete)
    ));
    assert_eq!(
        std::fs::read(chunk_path(&directory, &fixture, 0)).unwrap(),
        before
    );
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
}
