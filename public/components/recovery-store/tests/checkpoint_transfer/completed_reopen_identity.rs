// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::reopen_support::captured_files;
use crate::support::*;
use eve_storage::checkpoints::*;
use std::os::unix::fs::PermissionsExt;

#[test]
fn wrong_target_zero_id_and_short_reservations_refuse_without_touching_valid_store() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
    let path = namespace(&directory, &fixture);
    let before = captured_files(&path);
    let id = checkpoint_manifest_id(&manifest(&fixture));
    let mut target = fixture.commit.target.clone();
    target.timestamp += 1;
    assert!(matches!(
        open_completed_checkpoint_store(
            &root,
            &id,
            &target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        ),
        Err(CheckpointError::TargetMismatch)
    ));
    assert!(matches!(
        open_completed_checkpoint_store(
            &root,
            &[0; 32],
            &fixture.commit.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        ),
        Err(CheckpointError::InvalidManifest)
    ));
    assert!(matches!(
        open_completed_checkpoint_store(
            &root,
            &id,
            &fixture.commit.target,
            &fixture.limits,
            fixture.metadata - 1,
            fixture.io
        ),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(matches!(
        open_completed_checkpoint_store(
            &root,
            &id,
            &fixture.commit.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io - 1
        ),
        Err(CheckpointError::ResourceReservation)
    ));
    assert_eq!(captured_files(&path), before);
}

#[test]
fn copied_valid_bytes_under_foreign_namespace_id_fail_exact_manifest_binding() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
    let mut id = checkpoint_manifest_id(&manifest(&fixture));
    id[0] ^= 1;
    let name: String = id.iter().map(|byte| format!("{byte:02x}")).collect();
    let foreign = directory.path().join(name);
    std::fs::create_dir(&foreign).unwrap();
    std::fs::set_permissions(&foreign, std::fs::Permissions::from_mode(0o700)).unwrap();
    for (name, bytes) in captured_files(&namespace(&directory, &fixture)) {
        std::fs::write(foreign.join(name), bytes).unwrap();
    }
    let before = captured_files(&foreign);
    assert!(matches!(
        open_completed_checkpoint_store(
            &root,
            &id,
            &fixture.commit.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        ),
        Err(CheckpointError::TargetMismatch)
    ));
    assert_eq!(captured_files(&foreign), before);
}
