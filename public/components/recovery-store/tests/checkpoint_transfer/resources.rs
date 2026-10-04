// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn metadata_and_io_one_byte_short_refuse_before_namespace_or_chunk_write() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    assert!(matches!(
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert!(matches!(
        write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(!chunk_path(&directory, &fixture, 0).exists());
    assert!(matches!(
        observe_checkpoint_chunk(&transfer, 0, fixture.io - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(matches!(
        repair_invalid_checkpoint_chunk(&mut transfer, 0, fixture.io - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(matches!(
        complete_checkpoint_transfer(transfer, fixture.io - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(
        !namespace(&directory, &fixture)
            .join("complete.bin")
            .exists()
    );
}

#[test]
fn body_one_byte_short_rejects_without_invalidating_completed_store() {
    let fixture = fixture();
    let (_directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
    let required = required_checkpoint_body_reservation(&completed).unwrap();
    assert!(matches!(
        read_checkpoint_body(&completed, required - 1),
        Err(CheckpointError::ResourceReservation)
    ));
    assert_eq!(
        checkpoint_body_bytes(&read_checkpoint_body(&completed, required).unwrap()),
        fixture.body
    );
}

#[test]
fn oversized_body_chunk_count_manifest_and_invalid_limits_fail_before_transfer() {
    let fixture = fixture();
    let mut cases = Vec::new();
    let mut limits = fixture.limits;
    limits.maximum_body_bytes = fixture.body.len() - 1;
    cases.push(limits);
    limits = fixture.limits;
    limits.maximum_chunk_bytes = 511;
    cases.push(limits);
    limits = fixture.limits;
    limits.maximum_chunks = fixture.body.len().div_ceil(512) - 1;
    cases.push(limits);
    limits = fixture.limits;
    limits.maximum_manifest_bytes = fixture.manifest.len() - 1;
    cases.push(limits);
    for limits in cases {
        assert!(
            preflight_checkpoint_manifest(&fixture.manifest, &fixture.version, &limits).is_err()
        );
    }
    for invalid in [0, MAXIMUM_CHECKPOINT_CHUNK_BYTES + 1, usize::MAX] {
        limits = fixture.limits;
        limits.maximum_chunk_bytes = invalid;
        assert!(matches!(
            required_checkpoint_io_reservation(&limits),
            Err(CheckpointError::InvalidLimits)
        ));
    }
}

#[test]
fn malformed_chunks_reject_before_file_creation_and_index_cannot_escape_namespace() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let mut wrong = fixture.body[..512].to_vec();
    wrong[0] ^= 1;
    assert!(matches!(
        write_checkpoint_chunk(&mut transfer, 0, &wrong, fixture.io),
        Err(CheckpointError::CorruptChunk)
    ));
    assert!(matches!(
        write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..511], fixture.io),
        Err(CheckpointError::CorruptChunk)
    ));
    assert!(!chunk_path(&directory, &fixture, 0).exists());
    assert!(matches!(
        write_checkpoint_chunk(&mut transfer, usize::MAX, &[], fixture.io),
        Err(CheckpointError::InvalidManifest)
    ));
}
