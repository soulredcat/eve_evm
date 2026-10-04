// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::reopen_support::{captured_files, expected};
use crate::support::*;
use eve_storage::checkpoints::{CheckpointError, proofs::*};
use std::os::unix::fs::PermissionsExt;

#[test]
fn wrong_anchor_stream_target_zero_id_or_short_lease_refuses_valid_store_unchanged() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
    let path = namespace(&directory, &fixture);
    let before = captured_files(&path);
    for selector in 0..5 {
        let mut expected = expected(&fixture);
        match selector {
            0 => expected.snapshot_manifest_id[0] ^= 1,
            1 => expected.snapshot_body_sha256[0] ^= 1,
            2 => expected.stream_sha256[0] ^= 1,
            3 => expected.manifest_id = [0; 32],
            _ => expected.snapshot_manifest_id = [0; 32],
        }
        assert!(
            open_completed_checkpoint_proof_store(
                &root,
                &expected,
                &fixture.target,
                &fixture.limits,
                fixture.metadata,
                fixture.io
            )
            .is_err()
        );
    }
    let mut target = fixture.target.clone();
    target.timestamp += 1;
    assert!(matches!(
        open_completed_checkpoint_proof_store(
            &root,
            &expected(&fixture),
            &target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        ),
        Err(CheckpointError::TargetMismatch)
    ));
    assert!(matches!(
        open_completed_checkpoint_proof_store(
            &root,
            &expected(&fixture),
            &fixture.target,
            &fixture.limits,
            fixture.metadata - 1,
            fixture.io
        ),
        Err(CheckpointError::ResourceReservation)
    ));
    assert!(matches!(
        open_completed_checkpoint_proof_store(
            &root,
            &expected(&fixture),
            &fixture.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io - 1
        ),
        Err(CheckpointError::ResourceReservation)
    ));
    assert_eq!(captured_files(&path), before);
}

#[test]
fn copied_valid_proof_bytes_under_foreign_id_fail_without_any_mutation() {
    let fixture = fixture();
    let (directory, root) = temporary_root();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_all(&mut transfer, &fixture);
    drop(complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap());
    let mut identity = expected(&fixture);
    identity.manifest_id[0] ^= 1;
    let name: String = identity
        .manifest_id
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let foreign = directory.path().join(name);
    std::fs::create_dir(&foreign).unwrap();
    std::fs::set_permissions(&foreign, std::fs::Permissions::from_mode(0o700)).unwrap();
    for (name, bytes) in captured_files(&namespace(&directory, &fixture)) {
        std::fs::write(foreign.join(name), bytes).unwrap();
    }
    let before = captured_files(&foreign);
    assert!(matches!(
        open_completed_checkpoint_proof_store(
            &root,
            &identity,
            &fixture.target,
            &fixture.limits,
            fixture.metadata,
            fixture.io
        ),
        Err(CheckpointError::TargetMismatch)
    ));
    assert_eq!(captured_files(&foreign), before);
}
