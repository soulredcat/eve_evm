// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::*,
    reopen_fixtures::{completed_checkpoint, private_archive, recovery_config, reopen},
};
use crate::sync::applied::{
    finish_applied_state_service, segmented::tests::fixtures::configuration,
};
use std::os::unix::fs::PermissionsExt;

#[test]
fn missing_existing_completion_is_not_recreated_and_the_original_completed_archive_is_preserved() {
    let fixture = completed_checkpoint(false);
    for missing_content_completion in [true, false] {
        let copied = private_archive();
        let id = if missing_content_completion {
            fixture.content_id
        } else {
            fixture.proof_id
        };
        let name = hex::encode(id);
        let source = fixture.archive.path().join(&name);
        let target = copied.path().join(&name);
        std::fs::create_dir(&target).unwrap();
        std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o700)).unwrap();
        let mut count = 0;
        for entry in std::fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            count += 1;
            assert!(count <= 64);
            if entry.file_name() == "complete.bin" {
                continue;
            }
            assert!(entry.file_type().unwrap().is_file());
            assert!(entry.metadata().unwrap().len() <= 262_144);
            std::fs::copy(entry.path(), target.join(entry.file_name())).unwrap();
        }
        let mut recovery = recovery_config(fixture.archive.path());
        if missing_content_completion {
            recovery.content_root = copied.path().to_owned();
        } else {
            recovery.proof_root = copied.path().to_owned();
        }
        assert!(matches!(
            open_segmented_applied_state_service_with_checkpoints(
                configuration(&fixture.database.path().join("store"), &fixture.chain),
                recovery,
                &fixture.chain.genesis
            ),
            Err(CheckpointAppliedError::Storage(_))
        ));
        assert!(!target.join("complete.bin").exists());
    }
    let (owner, _) = reopen(&fixture).unwrap();
    drop(finish_applied_state_service(owner));
}
