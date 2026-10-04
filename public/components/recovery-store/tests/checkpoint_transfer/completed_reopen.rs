// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::reopen_support::captured_files;
use crate::support::*;
use eve_storage::checkpoints::*;
use std::os::unix::fs::PermissionsExt;

#[test]
fn completed_reopen_verifies_all_actual_bytes_and_never_changes_files() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
    let path = namespace(&directory, &fixture);
    let before = captured_files(&path);
    let id = checkpoint_manifest_id(&manifest(&fixture));
    let completed = open_completed_checkpoint_store(
        &root,
        &id,
        &fixture.commit.target,
        &fixture.limits,
        fixture.metadata,
        fixture.io,
    )
    .unwrap();
    let body = read_checkpoint_body(
        &completed,
        required_checkpoint_body_reservation(&completed).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint_body_bytes(&body), fixture.body);
    assert_eq!(captured_files(&path), before);
}

#[test]
fn absent_namespace_or_completion_never_creates_or_republishes_files() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let id = checkpoint_manifest_id(&manifest(&fixture));
    assert!(
        open_completed_checkpoint_store(
            &root,
            &id,
            &fixture.commit.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        )
        .is_err()
    );
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(transfer);
    let path = namespace(&directory, &fixture);
    let before = captured_files(&path);
    assert!(
        open_completed_checkpoint_store(
            &root,
            &id,
            &fixture.commit.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        )
        .is_err()
    );
    assert_eq!(captured_files(&path), before);
    assert!(!path.join("complete.bin").exists());
    assert!(!path.join("complete.pending").exists());
}

#[test]
fn missing_corrupt_chunk_or_changed_completion_is_rejected_without_repair() {
    for fault in 0..3 {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        let mut transfer =
            begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
        write_all(&mut transfer, &fixture);
        drop(complete_checkpoint_transfer(transfer, fixture.io).unwrap());
        let path = namespace(&directory, &fixture);
        match fault {
            0 => std::fs::remove_file(chunk_path(&directory, &fixture, 0)).unwrap(),
            1 => corrupt_chunk(&directory, &fixture, 0),
            _ => {
                let marker = path.join("complete.bin");
                std::fs::set_permissions(&marker, std::fs::Permissions::from_mode(0o600)).unwrap();
                let mut bytes = std::fs::read(&marker).unwrap();
                bytes[16] ^= 1;
                std::fs::write(marker, bytes).unwrap();
            }
        }
        let before = captured_files(&path);
        let id = checkpoint_manifest_id(&manifest(&fixture));
        assert!(
            open_completed_checkpoint_store(
                &root,
                &id,
                &fixture.commit.target,
                &fixture.limits,
                fixture.metadata,
                fixture.io
            )
            .is_err()
        );
        assert_eq!(captured_files(&path), before);
    }
}
