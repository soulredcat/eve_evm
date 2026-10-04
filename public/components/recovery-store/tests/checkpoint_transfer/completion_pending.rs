// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn truncated_completion_pending_is_explicitly_repaired_then_exact_chunks_resume() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(transfer);
    let first = std::fs::read(chunk_path(&directory, &fixture, 0)).unwrap();
    let pending = namespace(&directory, &fixture).join("complete.pending");
    std::fs::write(&pending, b"EVE_CKPT").unwrap();
    assert!(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    repair_invalid_checkpoint_completion_pending(&root, &manifest(&fixture), fixture.metadata)
        .unwrap();
    assert!(!pending.exists());
    assert_eq!(
        std::fs::read(chunk_path(&directory, &fixture, 0)).unwrap(),
        first
    );
    let transfer = begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
    let body = read_checkpoint_body(
        &completed,
        required_checkpoint_body_reservation(&completed).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint_body_bytes(&body), fixture.body);
}

#[test]
fn valid_completion_pending_is_preserved_then_promoted_without_rewrite() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(transfer);
    let stats = checkpoint_manifest_stats(&manifest(&fixture));
    let mut marker = Vec::from(b"EVE_CKPT_DONE_V1".as_slice());
    marker.extend_from_slice(&checkpoint_manifest_id(&manifest(&fixture)));
    marker.extend_from_slice(&stats.body_sha256);
    let pending = namespace(&directory, &fixture).join("complete.pending");
    std::fs::write(&pending, &marker).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_completion_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    let transfer = begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
    assert!(!pending.exists());
    assert_eq!(
        std::fs::read(namespace(&directory, &fixture).join("complete.bin")).unwrap(),
        marker
    );
    assert!(matches!(
        repair_invalid_checkpoint_completion_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::AlreadyComplete)
    ));
}

#[test]
fn foreign_valid_or_hardlinked_completion_staging_is_preserved() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    drop(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap());
    let pending = namespace(&directory, &fixture).join("complete.pending");
    let mut foreign = vec![0x71_u8; 80];
    foreign[..16].copy_from_slice(b"EVE_CKPT_DONE_V1");
    std::fs::write(&pending, &foreign).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_completion_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    assert_eq!(std::fs::read(&pending).unwrap(), foreign);
    std::fs::remove_file(&pending).unwrap();
    let outside = directory.path().join("outside.bin");
    std::fs::write(&outside, b"invalid but externally linked").unwrap();
    std::fs::hard_link(&outside, &pending).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_completion_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::UnsafeEntry)
    ));
    assert_eq!(
        std::fs::read(&outside).unwrap(),
        b"invalid but externally linked"
    );
}
