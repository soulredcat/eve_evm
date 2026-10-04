// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::*;
use std::{fs::File, process::Command};

#[test]
fn checkpoint_child_exits_without_destructors() {
    let Some(path) = std::env::var_os("EVE_CHECKPOINT_TRANSFER_CHILD_ROOT") else {
        return;
    };
    let fixture = fixture();
    let root = File::open(&path).unwrap();
    let mut transfer =
        begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_checkpoint_chunk(&mut transfer, 0, &fixture.body[..512], fixture.io).unwrap();
    if std::env::var_os("EVE_CHECKPOINT_TRANSFER_CHILD_COMPLETE").is_some() {
        write_all(&mut transfer, &fixture);
        let _completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
        std::process::exit(42);
    }
    std::process::exit(42);
}

#[test]
fn actual_child_exit_releases_owner_and_retains_synced_partial_and_complete_data() {
    for complete in [false, true] {
        let fixture = fixture();
        let (directory, root) = temporary_root();
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "process::checkpoint_child_exits_without_destructors",
                "--test-threads=1",
            ])
            .env("EVE_CHECKPOINT_TRANSFER_CHILD_ROOT", directory.path())
            .env_remove("EVE_CHECKPOINT_TRANSFER_CHILD_COMPLETE");
        if complete {
            command.env("EVE_CHECKPOINT_TRANSFER_CHILD_COMPLETE", "1");
        }
        assert_eq!(command.status().unwrap().code(), Some(42));
        let mut transfer =
            begin_checkpoint_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
        let observed = checkpoint_initial_resume_observation(&transfer);
        assert_eq!(
            observed.verified_chunks,
            if complete {
                fixture.body.len().div_ceil(512)
            } else {
                1
            }
        );
        assert_eq!(observed.corrupt_chunks, 0);
        if !complete {
            write_all(&mut transfer, &fixture);
        }
        let completed = complete_checkpoint_transfer(transfer, fixture.io).unwrap();
        let body = read_checkpoint_body(
            &completed,
            required_checkpoint_body_reservation(&completed).unwrap(),
        )
        .unwrap();
        assert_eq!(checkpoint_body_bytes(&body), fixture.body);
    }
}
