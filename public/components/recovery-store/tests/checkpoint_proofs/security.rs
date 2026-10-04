// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::{CheckpointError, proofs::*};
use std::os::unix::fs::symlink;

#[test]
fn proof_namespace_has_one_exclusive_owner_and_reopens_after_drop() {
    let fixture = fixture();
    let (_directory, root) = temporary_root();
    let transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert!(begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    drop(transfer);
    assert!(begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).is_ok());
}

#[test]
fn witness_symlink_is_not_followed_and_safe_staging_repair_preserves_target() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let outside = directory.path().join("external.bin");
    std::fs::write(&outside, b"preserve outside").unwrap();
    symlink(&outside, witness_path(&directory, &fixture, 0)).unwrap();
    assert!(observe_checkpoint_proof_witness(&transfer, 0, fixture.io).is_err());
    repair_invalid_checkpoint_proof_witness(&mut transfer, 0, fixture.io).unwrap();
    assert_eq!(std::fs::read(&outside).unwrap(), b"preserve outside");
    write_checkpoint_proof_witness(&mut transfer, 0, &fixture.blobs[0], fixture.io).unwrap();
}

#[test]
fn hardlinked_witness_and_metadata_are_preserved_and_refused() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    let outside = directory.path().join("external.bin");
    std::fs::write(&outside, &fixture.blobs[0]).unwrap();
    std::fs::hard_link(&outside, witness_path(&directory, &fixture, 0)).unwrap();
    assert!(matches!(
        observe_checkpoint_proof_witness(&transfer, 0, fixture.io),
        Err(CheckpointError::UnsafeEntry)
    ));
    assert!(matches!(
        repair_invalid_checkpoint_proof_witness(&mut transfer, 0, fixture.io),
        Err(CheckpointError::UnsafeEntry)
    ));
    assert_eq!(std::fs::read(&outside).unwrap(), fixture.blobs[0]);
    drop(transfer);
    std::fs::hard_link(
        &outside,
        namespace(&directory, &fixture).join("complete.pending"),
    )
    .unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_proof_pending(
            &root,
            &manifest(&fixture),
            CheckpointProofPendingKind::Completion,
            fixture.metadata
        ),
        Err(CheckpointError::UnsafeEntry)
    ));
}

#[test]
fn foreign_manifest_and_unknown_entry_never_mutate_preserved_data() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.bin");
    let mut wrong = fixture.manifest.clone();
    wrong[112] ^= 1;
    std::fs::write(&path, &wrong).unwrap();
    assert!(matches!(
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::NamespaceOccupied)
    ));
    assert_eq!(std::fs::read(&path).unwrap(), wrong);
    std::fs::remove_file(&path).unwrap();
    std::fs::write(
        namespace(&directory, &fixture).join("unknown.bin"),
        b"retain",
    )
    .unwrap();
    assert!(matches!(
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::NamespaceOccupied)
    ));
    assert_eq!(
        std::fs::read(namespace(&directory, &fixture).join("unknown.bin")).unwrap(),
        b"retain"
    );
}

#[test]
fn namespace_symlink_cannot_redirect_proof_store_into_external_directory() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let outside = directory.path().join("external");
    std::fs::create_dir(&outside).unwrap();
    std::fs::write(outside.join("sentinel"), b"retain").unwrap();
    symlink(&outside, namespace(&directory, &fixture)).unwrap();
    assert!(begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    assert_eq!(std::fs::read(outside.join("sentinel")).unwrap(), b"retain");
}
