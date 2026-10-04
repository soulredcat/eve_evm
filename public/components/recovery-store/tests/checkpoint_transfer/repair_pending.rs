// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;

#[test]
fn truncated_pending_manifest_requires_explicit_repair_before_resume() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&path, &fixture.manifest[..24]).unwrap();
    assert!(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), fixture.manifest[..24]);
    repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata)
        .unwrap();
    assert!(!path.exists());
    let transfer = begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert_eq!(
        checkpoint_initial_resume_observation(&transfer).verified_chunks,
        0
    );
}

#[test]
fn identical_valid_pending_manifest_is_preserved_and_atomically_promoted() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.pending");
    std::fs::write(&path, &fixture.manifest).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    let transfer = begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    assert_eq!(
        std::fs::read(namespace(&directory, &fixture).join("manifest.bin")).unwrap(),
        fixture.manifest
    );
    assert!(!path.exists());
    drop(transfer);
}

#[test]
fn valid_foreign_pending_manifest_is_preserved_and_cannot_rebind_namespace() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    create_namespace(&directory, &fixture);
    let path = namespace(&directory, &fixture).join("manifest.pending");
    let mut foreign = fixture.manifest.clone();
    foreign[44] ^= 1;
    std::fs::write(&path, &foreign).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::ValidEntry)
    ));
    assert!(begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), foreign);
}

#[test]
fn published_manifest_and_completed_content_are_never_removed_by_pending_repair() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
    let before = std::fs::read(namespace(&directory, &fixture).join("manifest.bin")).unwrap();
    assert!(matches!(
        repair_invalid_checkpoint_manifest_pending(&root, &manifest(&fixture), fixture.metadata),
        Err(CheckpointError::AlreadyComplete)
    ));
    assert_eq!(
        std::fs::read(namespace(&directory, &fixture).join("manifest.bin")).unwrap(),
        before
    );
}
