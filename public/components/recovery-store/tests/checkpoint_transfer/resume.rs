// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn bounded_partial_chunk_is_explicitly_repaired_after_identical_manifest_resume() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io).unwrap();
    drop(transfer);
    // An intentionally incomplete local write models interrupted staging, not power loss.
    std::fs::write(chunk_path(&directory, &fixture, 1), [0x77_u8; 3]).unwrap();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let observed = checkpoint_initial_resume_observation(&transfer);
    assert_eq!((observed.verified_chunks, observed.corrupt_chunks), (1, 1));
    assert!(matches!(
        write_checkpoint_chunk(&mut transfer, 1, &fixture.body[512..1_024], fixture.io),
        Err(CheckpointError::CorruptChunk)
    ));
    repair_invalid_checkpoint_chunk(&mut transfer, 1, fixture.io).unwrap();
    assert_eq!(
        observe_checkpoint_chunk(&transfer, 1, fixture.io).unwrap(),
        CheckpointChunkStatus::Missing
    );
    assert!(matches!(
        repair_invalid_checkpoint_chunk(&mut transfer, 0, fixture.io),
        Err(CheckpointError::ValidEntry)
    ));
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
}

#[test]
fn missing_chunk_prevents_completion_and_retains_synced_prefix_for_reopen() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io).unwrap();
    assert!(complete_checkpoint_transfer(transfer, fixture.io).is_err());
    assert!(
        !namespace(&directory, &fixture)
            .join("complete.bin")
            .exists()
    );
    let transfer = begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert_eq!(
        checkpoint_initial_resume_observation(&transfer).verified_chunks,
        1
    );
    assert_eq!(
        observe_checkpoint_chunk(&transfer, 1, fixture.io).unwrap(),
        CheckpointChunkStatus::Missing
    );
}

#[test]
fn real_chunk_corruption_is_detected_and_completed_corruption_cannot_be_repaired() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    corrupt_chunk(&directory, &fixture, 0);
    assert_eq!(
        observe_checkpoint_chunk(&transfer, 0, fixture.io).unwrap(),
        CheckpointChunkStatus::Corrupt
    );
    repair_invalid_checkpoint_chunk(&mut transfer, 0, fixture.io).unwrap();
    write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io).unwrap();
    let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
    corrupt_chunk(&directory, &fixture, 0);
    assert!(
        read_checkpoint_body(
            &completed,
            required_checkpoint_body_reservation(&completed).unwrap()
        )
        .is_err()
    );
    drop(completed);
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert_eq!(
        checkpoint_initial_resume_observation(&transfer).corrupt_chunks,
        1
    );
    assert!(matches!(
        repair_invalid_checkpoint_chunk(&mut transfer, 0, fixture.io),
        Err(CheckpointError::AlreadyComplete)
    ));
    assert!(complete_checkpoint_transfer(transfer, fixture.io).is_err());
}
