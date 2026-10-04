// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    reopen_fixtures::{
        completed_checkpoint, copy_namespace, private_archive, recovery_config, reopen,
    },
};
use crate::sync::applied::{
    finish_applied_state_service, segmented::tests::fixtures::configuration,
};
use std::io::{Read, Seek, SeekFrom, Write};

#[test]
fn an_existing_base_with_missing_content_or_proofs_fails_without_recreating_files() {
    let fixture = completed_checkpoint(false);
    let missing = private_archive();
    for missing_content in [true, false] {
        let mut recovery = recovery_config(fixture.archive.path());
        if missing_content {
            recovery.content_root = missing.path().to_owned();
        } else {
            recovery.proof_root = missing.path().to_owned();
        }
        assert!(matches!(
            open_segmented_applied_state_service_with_checkpoints(
                configuration(&fixture.database.path().join("store"), &fixture.chain),
                recovery,
                &fixture.chain.genesis
            ),
            Err(CheckpointAppliedError::Storage(_))
        ));
        assert_eq!(std::fs::read_dir(missing.path()).unwrap().count(), 0);
    }
    let (owner, _) = reopen(&fixture).unwrap();
    drop(finish_applied_state_service(owner));
}

#[test]
fn corrupt_copied_content_and_proof_files_are_rejected_and_the_original_archive_remains_usable() {
    let fixture = completed_checkpoint(false);
    for corrupt_content in [true, false] {
        let copy = private_archive();
        let (id, filename) = if corrupt_content {
            (fixture.content_id, "chunk-0000.bin")
        } else {
            (fixture.proof_id, "witness-00000.bin")
        };
        copy_namespace(fixture.archive.path(), copy.path(), &id);
        let file_path = copy.path().join(hex::encode(id)).join(filename);
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&file_path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&file_path)
            .unwrap();
        let mut byte = [0_u8; 1];
        file.read_exact(&mut byte).unwrap();
        byte[0] ^= 1;
        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(&byte).unwrap();
        file.sync_all().unwrap();
        {
            use std::os::unix::fs::PermissionsExt;
            file.set_permissions(std::fs::Permissions::from_mode(0o400))
                .unwrap();
        }
        drop(file);
        let mut recovery = recovery_config(fixture.archive.path());
        if corrupt_content {
            recovery.content_root = copy.path().to_owned();
        } else {
            recovery.proof_root = copy.path().to_owned();
        }
        assert!(matches!(
            open_segmented_applied_state_service_with_checkpoints(
                configuration(&fixture.database.path().join("store"), &fixture.chain),
                recovery,
                &fixture.chain.genesis
            ),
            Err(CheckpointAppliedError::Storage(_))
        ));
    }
    let (owner, _) = reopen(&fixture).unwrap();
    drop(finish_applied_state_service(owner));
}

#[test]
fn scan_cap_refuses_a_larger_actual_history_before_artifact_or_ram_publication() {
    let fixture = completed_checkpoint(true);
    let mut recovery = recovery_config(fixture.archive.path());
    recovery.maximum_scan_records = 1;
    assert!(matches!(
        open_segmented_applied_state_service_with_checkpoints(
            configuration(&fixture.database.path().join("store"), &fixture.chain),
            recovery,
            &fixture.chain.genesis
        ),
        Err(CheckpointAppliedError::ScanLimit)
    ));
    let (owner, _) = reopen(&fixture).unwrap();
    drop(finish_applied_state_service(owner));
}

#[test]
fn a_symlinked_configured_root_is_refused_without_following_it() {
    let fixture = completed_checkpoint(false);
    let links = private_archive();
    let link = links.path().join("linked-root");
    std::os::unix::fs::symlink(fixture.archive.path(), &link).unwrap();
    let mut recovery = recovery_config(fixture.archive.path());
    recovery.content_root = link;
    assert!(matches!(
        open_segmented_applied_state_service_with_checkpoints(
            configuration(&fixture.database.path().join("store"), &fixture.chain),
            recovery,
            &fixture.chain.genesis
        ),
        Err(CheckpointAppliedError::Storage(_))
    ));
    let (owner, _) = reopen(&fixture).unwrap();
    drop(finish_applied_state_service(owner));
}
