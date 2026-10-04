// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;
use std::os::unix::fs::{PermissionsExt, symlink};

#[test]
fn namespace_symlink_and_open_permissions_are_rejected_without_target_changes() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let outside = directory.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    std::fs::write(outside.join("sentinel"), b"preserve").unwrap();
    symlink(&outside, namespace(&directory, &fixture)).unwrap();
    assert!(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    assert_eq!(
        std::fs::read(outside.join("sentinel")).unwrap(),
        b"preserve"
    );
    std::fs::remove_file(namespace(&directory, &fixture)).unwrap();
    create_namespace(&directory, &fixture);
    std::fs::set_permissions(
        namespace(&directory, &fixture),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    assert!(matches!(
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::UnsafeEntry)
    ));
}

#[test]
fn exclusive_owner_blocks_second_transfer_until_first_handle_is_dropped() {
    let fixture = fixture();
    let (_directory, root) = temporary_root();
    let transfer = begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert!(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    drop(transfer);
    assert!(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).is_ok());
}

#[test]
fn staging_chunk_symlink_is_unlinked_without_following_or_mutating_target() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let outside = directory.path().join("outside.bin");
    std::fs::write(&outside, b"external contents").unwrap();
    symlink(&outside, chunk_path(&directory, &fixture, 0)).unwrap();
    assert!(observe_checkpoint_chunk(&transfer, 0, fixture.io).is_err());
    repair_invalid_checkpoint_chunk(&mut transfer, 0, fixture.io).unwrap();
    assert_eq!(std::fs::read(&outside).unwrap(), b"external contents");
    assert_eq!(
        observe_checkpoint_chunk(&transfer, 0, fixture.io).unwrap(),
        CheckpointChunkStatus::Missing
    );
    write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io).unwrap();
}

#[test]
fn hardlinked_chunk_cannot_be_read_written_or_repaired() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let outside = directory.path().join("outside.bin");
    std::fs::write(&outside, &fixture.body[..512]).unwrap();
    std::fs::hard_link(&outside, chunk_path(&directory, &fixture, 0)).unwrap();
    assert!(matches!(
        observe_checkpoint_chunk(&transfer, 0, fixture.io),
        Err(CheckpointError::UnsafeEntry)
    ));
    assert!(matches!(
        repair_invalid_checkpoint_chunk(&mut transfer, 0, fixture.io),
        Err(CheckpointError::UnsafeEntry)
    ));
    assert!(matches!(
        write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io),
        Err(CheckpointError::UnsafeEntry)
    ));
    assert_eq!(std::fs::read(&outside).unwrap(), fixture.body[..512]);
}

#[test]
fn unsafe_metadata_symlink_hardlink_and_unknown_names_are_preserved_and_rejected() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let outside = directory.path().join("outside.bin");
    std::fs::write(&outside, &fixture.manifest).unwrap();
    let path = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::hard_link(&outside, &path).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::UnsafeEntry)
    ));
    assert!(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    std::fs::remove_file(&path).unwrap();
    symlink(&outside, &path).unwrap();
    repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata)
        .unwrap();
    assert_eq!(std::fs::read(&outside).unwrap(), fixture.manifest);
    std::fs::write(
        namespace(&directory, &fixture).join("unexpected.bin"),
        b"preserve",
    )
    .unwrap();
    assert!(matches!(
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::NamespaceOccupied)
    ));
    assert_eq!(
        std::fs::read(namespace(&directory, &fixture).join("unexpected.bin")).unwrap(),
        b"preserve"
    );
}

#[test]
fn foreign_published_manifest_is_preserved_and_cannot_rebind_requested_transfer() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let mut wrong = fixture.manifest.clone();
    wrong[44] ^= 1;
    let path = namespace(&directory, &fixture).join("manifest.bin");
    std::fs::write(&path, &wrong).unwrap();
    assert!(matches!(
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::NamespaceOccupied)
    ));
    assert_eq!(std::fs::read(path).unwrap(), wrong);
}
