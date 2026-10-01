// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod state_support;
use eve_storage::state::{
    CommitDisposition, commit_state, create_state_service, development_state_storage_budget,
    open_state_repository, read_state_service, state_reader,
};
use std::sync::Arc;

#[test]
fn full_state_domains_payload_roots_and_marker_reopen_together() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let second = state_support::next(&first, 2);
    let mut store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    let ack = commit_state(&mut store, &first).unwrap();
    assert_eq!(ack.disposition, CommitDisposition::NewlySynced);
    assert_eq!(ack.committed, first.target);
    commit_state(&mut store, &second).unwrap();
    drop(store);
    let reopened = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    let service = create_state_service(state_reader(&reopened));
    let view = read_state_service(&service).unwrap();
    assert_eq!(view.commit(), &second);
    assert_eq!(view.commit().state.accounts, second.state.accounts);
    assert_eq!(view.commit().state.system, second.state.system);
}

#[test]
fn whole_commit_replay_and_stale_parent_are_distinct() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let second = state_support::next(&first, 2);
    let mut store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    commit_state(&mut store, &first).unwrap();
    commit_state(&mut store, &second).unwrap();
    let replay = commit_state(&mut store, &first).unwrap();
    assert_eq!(replay.disposition, CommitDisposition::ExactReplay);
    assert_eq!(replay.committed.height, 1);
    assert_eq!(replay.store_head.height, 2);
    let altered_first = state_support::next(&genesis, 99);
    assert!(commit_state(&mut store, &altered_first).is_err());
    let altered_second = state_support::next(&altered_first, 3);
    let stale_third = state_support::next(&altered_second, 4);
    assert!(commit_state(&mut store, &stale_third).is_err());
}

#[test]
fn cache_is_checked_first_and_refreshes_after_durable_publication() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let mut store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    let service = create_state_service(state_reader(&store));
    let initial = read_state_service(&service).unwrap();
    assert!(Arc::ptr_eq(
        &initial,
        &read_state_service(&service).unwrap()
    ));
    commit_state(&mut store, &first).unwrap();
    let refreshed = read_state_service(&service).unwrap();
    assert!(!Arc::ptr_eq(&initial, &refreshed));
    assert_eq!(initial.commit(), &genesis);
    assert_eq!(refreshed.commit(), &first);
    assert!(Arc::ptr_eq(
        &refreshed,
        &read_state_service(&service).unwrap()
    ));
}
