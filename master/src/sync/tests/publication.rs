// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::fixture;
use crate::sync::{
    current_master_commit, import_master_wire, master_sync_status, open_master_follower,
};
use eve_state::encode_state_commit;

#[test]
fn actual_signed_imports_archive_exact_complete_commits_and_reverify_every_height_on_restart() {
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    let genesis_status = master_sync_status(&follower);
    assert_eq!(genesis_status.role, "MASTER_SYNC_ONLY");
    assert!(!genesis_status.authenticated_validator_finality);
    assert!(!genesis_status.ready);
    assert_eq!(genesis_status.peer_head, None);
    assert_eq!(genesis_status.lag, None);
    let genesis_bytes =
        encode_state_commit(&fixture.chain.commits[0], &fixture.config.storage.logical)
            .unwrap()
            .len() as u64;
    let mut commit_bytes = genesis_bytes;
    let mut proof_bytes = 0;
    for height in 1..=2 {
        let status = import_master_wire(
            &mut follower,
            &fixture.wires[height - 1],
            &fixture.chain.commits[height].target,
        )
        .unwrap();
        commit_bytes += encode_state_commit(
            &fixture.chain.commits[height],
            &fixture.config.storage.logical,
        )
        .unwrap()
        .len() as u64;
        proof_bytes += fixture.wires[height - 1].len() as u64;
        assert_eq!(
            current_master_commit(&follower),
            &fixture.chain.commits[height]
        );
        assert_eq!(status.finalized_height, height as u64);
        assert_eq!(status.applied_height, height as u64);
        assert_eq!(status.durable_height, height as u64);
        assert_eq!(status.authenticated_height, height as u64);
        assert!(status.authenticated_validator_finality);
        assert_eq!(status.retained_canonical_commit_bytes, commit_bytes);
        assert_eq!(status.retained_archive_bytes, proof_bytes);
        assert_eq!(status.retained_proof_files, height as u64);
        assert!(!status.storage_outcome_unknown);
        assert!(!status.ready);
    }
    drop(follower);
    let reopened = open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    assert_eq!(current_master_commit(&reopened), &fixture.chain.commits[2]);
    assert_eq!(
        master_sync_status(&reopened).retained_canonical_commit_bytes,
        commit_bytes
    );
}

#[test]
fn actual_exclusive_state_namespace_rejects_a_second_owner_without_rewriting_history() {
    let fixture = fixture();
    let mut follower =
        open_master_follower(fixture.config.clone(), &fixture.chain.genesis).unwrap();
    import_master_wire(
        &mut follower,
        &fixture.wires[0],
        &fixture.chain.commits[1].target,
    )
    .unwrap();
    assert!(open_master_follower(fixture.config.clone(), &fixture.chain.genesis).is_err());
    assert_eq!(current_master_commit(&follower), &fixture.chain.commits[1]);
    assert!(!master_sync_status(&follower).fenced);
}
