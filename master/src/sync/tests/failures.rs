// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    fixtures::{fixture, import_input, proof_directory},
    recovery_support,
};
use crate::sync::{
    current_master_commit, import_master_wire, master_sync_status, open_master_follower,
    types::MasterSyncFault,
};
use eve_finality_verifier::encode_logical_import_wire;
use eve_state::{Bytes, build_state_commit};

#[test]
fn proof_ahead_and_lost_state_ack_reopen_exactly_without_reapplying_imported_fees() {
    for fault in [
        MasterSyncFault::AfterProofSync,
        MasterSyncFault::AfterStateSync,
    ] {
        let fixture = fixture();
        let mut follower =
            open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
        follower.fault = Some(fault);
        assert!(
            import_master_wire(
                &mut follower,
                &fixture.wires[0],
                &fixture.chain.commits[1].target
            )
            .is_err()
        );
        assert_eq!(current_master_commit(&follower), &fixture.chain.commits[0]);
        let status = master_sync_status(&follower);
        assert!(status.fenced && status.storage_outcome_unknown);
        assert!(!status.authenticated_validator_finality);
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
        assert_eq!(current_master_commit(&reopened), &fixture.chain.commits[1]);
        import_master_wire(
            &mut reopened,
            &fixture.wires[1],
            &fixture.chain.commits[2].target,
        )
        .unwrap();
        assert_eq!(current_master_commit(&reopened), &fixture.chain.commits[2]);
    }
}

#[test]
fn bad_native_signatures_and_certified_wrong_application_roots_cannot_create_archive_files() {
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    let mut invalid = import_input(&fixture.chain, 1, &fixture.config);
    for signature in &mut invalid.finalized.commit.signatures {
        signature.signature[0] ^= 1;
    }
    let wire = encode_logical_import_wire(&invalid, &fixture.config.storage.logical).unwrap();
    assert!(import_master_wire(&mut follower, &wire, &fixture.chain.commits[1].target).is_err());
    let mut wrong_application = import_input(&fixture.chain, 1, &fixture.config);
    wrong_application.lookahead.frame.header.app_hash[0] ^= 1;
    recovery_support::resign(
        &mut wrong_application.lookahead.frame,
        &fixture.chain.genesis,
    );
    let wire =
        encode_logical_import_wire(&wrong_application, &fixture.config.storage.logical).unwrap();
    assert!(import_master_wire(&mut follower, &wire, &fixture.chain.commits[1].target).is_err());
    assert_eq!(current_master_commit(&follower), &fixture.chain.commits[0]);
    assert_eq!(
        std::fs::read_dir(proof_directory(&fixture))
            .unwrap()
            .count(),
        0
    );
    assert!(!follower.fenced);
}

#[test]
fn exact_whole_database_commit_must_match_proof_derived_auxiliary_state() {
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    let mut state = fixture.chain.commits[1].state.clone();
    let unused = Bytes::from_static(&[0x60, 0x01, 0x60, 0x00]);
    state
        .codes
        .insert(alloy_primitives::keccak256(&unused), unused);
    let local = build_state_commit(
        Some(fixture.chain.commits[0].target.clone()),
        state,
        fixture.chain.commits[1].block.clone(),
        &fixture.config.storage.logical,
    )
    .unwrap();
    assert_eq!(
        local.target.application,
        fixture.chain.commits[1].target.application
    );
    assert_ne!(
        local.target.content_digest,
        fixture.chain.commits[1].target.content_digest
    );
    eve_storage::state::commit_state(&mut follower.repository, &local).unwrap();
    crate::sync::archive::write_staged_proof(&follower.directory, &fixture.wires[0]).unwrap();
    crate::sync::archive::promote_staged_proof(&follower.directory, 1).unwrap();
    drop(follower);
    let error = open_master_follower(fixture.config.clone(), &fixture.chain.genesis)
        .err()
        .unwrap()
        .to_string();
    assert!(
        error.contains("MASTER_RETAINED_WHOLE_COMMIT_MISMATCH"),
        "{error}"
    );
}
