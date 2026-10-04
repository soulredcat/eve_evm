// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::reopen_support::{captured_files, expected};
use crate::support::*;
use eve_storage::checkpoints::proofs::*;
use std::os::unix::fs::PermissionsExt;

#[test]
fn completed_proof_reopen_verifies_actual_ordered_stream_and_preserves_all_files() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
    let path = namespace(&directory, &fixture);
    let before = captured_files(&path);
    let completed = open_completed_checkpoint_proof_store(
        &root,
        &expected(&fixture),
        &fixture.target,
        &fixture.limits,
        fixture.metadata,
        fixture.io,
    )
    .unwrap();
    let witness = read_checkpoint_proof_witness(
        &completed,
        2,
        required_checkpoint_proof_witness_reservation(&completed, 2).unwrap(),
    )
    .unwrap();
    assert_eq!(checkpoint_proof_witness_bytes(&witness), fixture.blobs[2]);
    assert_eq!(captured_files(&path), before);
}

#[test]
fn missing_proof_namespace_or_completion_never_recreates_or_promotes_data() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    assert!(
        open_completed_checkpoint_proof_store(
            &root,
            &expected(&fixture),
            &fixture.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        )
        .is_err()
    );
    assert_eq!(std::fs::read_dir(directory.path()).unwrap().count(), 0);
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(transfer);
    let path = namespace(&directory, &fixture);
    let before = captured_files(&path);
    assert!(
        open_completed_checkpoint_proof_store(
            &root,
            &expected(&fixture),
            &fixture.target,
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
fn corrupt_or_missing_witness_and_changed_marker_fail_without_repairs() {
    for fault in 0..3 {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        let mut transfer =
            begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
        write_all(&mut transfer, &fixture);
        drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
        let path = namespace(&directory, &fixture);
        match fault {
            0 => std::fs::remove_file(witness_path(&directory, &fixture, 0)).unwrap(),
            1 => corrupt_witness(&directory, &fixture, 0),
            _ => {
                let marker = path.join("complete.bin");
                std::fs::set_permissions(&marker, std::fs::Permissions::from_mode(0o600)).unwrap();
                let mut bytes = std::fs::read(&marker).unwrap();
                bytes[48] ^= 1;
                std::fs::write(marker, bytes).unwrap();
            }
        }
        let before = captured_files(&path);
        assert!(
            open_completed_checkpoint_proof_store(
                &root,
                &expected(&fixture),
                &fixture.target,
                &fixture.limits,
                fixture.metadata,
                fixture.io
            )
            .is_err()
        );
        assert_eq!(captured_files(&path), before);
    }
}
