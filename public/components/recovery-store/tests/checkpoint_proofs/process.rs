// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::proofs::*;
use std::{fs::File, process::Command};

#[test]
fn checkpoint_proof_child_exits_without_destructors() {
    let Some(path) = std::env::var_os("EVE_CHECKPOINT_PROOF_CHILD_ROOT") else {
        return;
    };
    let fixture = fixture();
    let root = File::open(path).unwrap();
    let mut transfer =
        begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
    write_checkpoint_proof_witness(&mut transfer, 0, &fixture.blobs[0], fixture.io).unwrap();
    if std::env::var_os("EVE_CHECKPOINT_PROOF_CHILD_COMPLETE").is_some() {
        write_all(&mut transfer, &fixture);
        let _completed = complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap();
        std::process::exit(42);
    }
    std::process::exit(42);
}

#[test]
fn actual_child_exit_preserves_partial_or_complete_proof_store_and_releases_lock() {
    for complete in [false, true] {
        let fixture = fixture();
        let (_directory, root) = temporary_root();
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "process::checkpoint_proof_child_exits_without_destructors",
                "--test-threads=1",
            ])
            .env("EVE_CHECKPOINT_PROOF_CHILD_ROOT", _directory.path())
            .env_remove("EVE_CHECKPOINT_PROOF_CHILD_COMPLETE");
        if complete {
            command.env("EVE_CHECKPOINT_PROOF_CHILD_COMPLETE", "1");
        }
        assert_eq!(command.status().unwrap().code(), Some(42));
        let mut transfer =
            begin_checkpoint_proof_transfer(&root, &manifest(&fixture), fixture.metadata).unwrap();
        assert_eq!(
            checkpoint_proof_initial_resume_observation(&transfer).verified_chunks,
            if complete { 3 } else { 1 }
        );
        if !complete {
            write_all(&mut transfer, &fixture);
        }
        let completed = complete_checkpoint_proof_transfer(transfer, fixture.io).unwrap();
        let witness = read_checkpoint_proof_witness(
            &completed,
            2,
            required_checkpoint_proof_witness_reservation(&completed, 2).unwrap(),
        )
        .unwrap();
        assert_eq!(
            checkpoint_proof_witness_reference(&witness).kind,
            CheckpointProofKind::ClosingLookahead
        );
        assert_eq!(checkpoint_proof_witness_bytes(&witness), fixture.blobs[2]);
    }
}
