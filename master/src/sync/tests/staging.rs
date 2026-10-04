// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{fixture, proof_directory};
use crate::sync::{
    archive::write_staged_proof, current_master_commit, import_master_wire, master_sync_status,
    open_master_follower, types::MasterSyncFault,
};

#[test]
fn complete_staging_is_reauthenticated_synced_promoted_and_committed_on_restart() {
    let fixture = fixture();
    let follower = open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    write_staged_proof(&follower.directory, &fixture.wires[0]).unwrap();
    drop(follower);
    let reopened = open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    assert_eq!(current_master_commit(&reopened), &fixture.chain.commits[1]);
    assert!(master_sync_status(&reopened).authenticated_validator_finality);
    assert!(!proof_directory(&fixture).join("pending.proof").exists());
    assert_eq!(
        std::fs::read(proof_directory(&fixture).join("00000000000000000001.proof")).unwrap(),
        fixture.wires[0]
    );
}

#[test]
fn partial_uncommitted_staging_is_preserved_once_and_new_valid_attempt_can_finish() {
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    follower.fault = Some(MasterSyncFault::PartialStaging);
    assert!(
        import_master_wire(
            &mut follower,
            &fixture.wires[0],
            &fixture.chain.commits[1].target
        )
        .is_err()
    );
    drop(follower);
    let mut reopened =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    assert_eq!(current_master_commit(&reopened), &fixture.chain.commits[0]);
    assert!(master_sync_status(&reopened).rejected_staging_retained);
    assert_eq!(
        std::fs::read(proof_directory(&fixture).join("rejected-staging.proof")).unwrap(),
        fixture.wires[0][..fixture.wires[0].len() / 2]
    );
    import_master_wire(
        &mut reopened,
        &fixture.wires[0],
        &fixture.chain.commits[1].target,
    )
    .unwrap();
    assert_eq!(current_master_commit(&reopened), &fixture.chain.commits[1]);
    assert!(
        proof_directory(&fixture)
            .join("rejected-staging.proof")
            .exists()
    );
    assert_eq!(
        master_sync_status(&reopened).retained_archive_bytes,
        (fixture.wires[0].len() + fixture.wires[0].len() / 2) as u64
    );
}

#[test]
fn occupied_rejected_slot_refuses_another_malformed_staging_without_replacing_either_body() {
    let fixture = fixture();
    let follower = open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    write_staged_proof(&follower.directory, b"first malformed body").unwrap();
    drop(follower);
    let reopened = open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    write_staged_proof(&reopened.directory, b"second malformed body").unwrap();
    drop(reopened);
    let error = open_master_follower(fixture.config.clone(), &fixture.chain.genesis)
        .err()
        .unwrap()
        .to_string();
    assert!(
        error.contains("MASTER_REJECTED_STAGING_SLOT_OCCUPIED"),
        "{error}"
    );
    assert_eq!(
        std::fs::read(proof_directory(&fixture).join("rejected-staging.proof")).unwrap(),
        b"first malformed body"
    );
    assert_eq!(
        std::fs::read(proof_directory(&fixture).join("pending.proof")).unwrap(),
        b"second malformed body"
    );
}

#[test]
fn unexplained_completed_proof_ahead_plus_next_staging_preserves_both_and_refuses_startup() {
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    follower.fault = Some(MasterSyncFault::AfterProofSync);
    assert!(
        import_master_wire(
            &mut follower,
            &fixture.wires[0],
            &fixture.chain.commits[1].target
        )
        .is_err()
    );
    write_staged_proof(&follower.directory, &fixture.wires[1]).unwrap();
    drop(follower);
    let error = open_master_follower(fixture.config.clone(), &fixture.chain.genesis)
        .err()
        .unwrap()
        .to_string();
    assert!(
        error.contains("MASTER_UNEXPLAINED_PROOF_AND_STAGING_SUFFIX"),
        "{error}"
    );
    assert_eq!(
        std::fs::read(proof_directory(&fixture).join("00000000000000000001.proof")).unwrap(),
        fixture.wires[0]
    );
    assert_eq!(
        std::fs::read(proof_directory(&fixture).join("pending.proof")).unwrap(),
        fixture.wires[1]
    );
}
