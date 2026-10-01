// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod history_transaction_support;
use eve_storage::state::{
    HistoryReadBudget, capture_history_snapshot, commit_state, development_state_storage_budget,
    ensure_history_index, lookup_execution_hash, lookup_transaction, open_state_repository,
    read_history_block, state_reader,
};
use history_transaction_support::source_fixture;
fn limits() -> HistoryReadBudget {
    HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: 128,
        maximum_index_batch_bytes: 16 * 1_048_576,
    }
}

#[test]
fn actual_transaction_and_receipt_bootstrap_lookups_bind_exact_source_location() {
    let directory = tempfile::tempdir().unwrap();
    let (genesis, first, _, hash) = source_fixture();
    let mut store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    commit_state(&mut store, &first).unwrap();
    let partial = HistoryReadBudget {
        maximum_rebuild_blocks: 1,
        ..limits()
    };
    assert!(!ensure_history_index(&mut store, partial).unwrap().complete);
    assert!(ensure_history_index(&mut store, partial).unwrap().complete);
    let reader = state_reader(&store);
    let snapshot = capture_history_snapshot(&reader, limits()).unwrap();
    let location = lookup_transaction(&snapshot, hash).unwrap().unwrap();
    assert_eq!(location.height, 1);
    assert_eq!(location.transaction_index, 0);
    assert_eq!(
        read_history_block(&snapshot, 1).unwrap().unwrap().block,
        first.block
    );
    assert_eq!(
        lookup_execution_hash(&snapshot, first.target.execution_hash.0).unwrap(),
        Some(1)
    );
    drop(snapshot);
    drop(reader);
    drop(store);
    let reopened = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    let reader = state_reader(&reopened);
    let snapshot = capture_history_snapshot(&reader, limits()).unwrap();
    assert_eq!(
        lookup_transaction(&snapshot, hash).unwrap().unwrap(),
        location
    );
    assert_eq!(
        read_history_block(&snapshot, 1)
            .unwrap()
            .unwrap()
            .block
            .receipts,
        first.block.receipts
    );
}

#[test]
fn duplicate_transaction_in_one_bootstrap_pass_rejects_without_activation_or_source_rewrite() {
    let directory = tempfile::tempdir().unwrap();
    let (genesis, first, second, _) = source_fixture();
    let budget = development_state_storage_budget();
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    commit_state(&mut store, &first).unwrap();
    commit_state(&mut store, &second).unwrap();
    assert!(
        ensure_history_index(&mut store, limits())
            .unwrap_err()
            .to_string()
            .contains("conflicting bootstrap identity")
    );
    let reader = state_reader(&store);
    assert!(capture_history_snapshot(&reader, limits()).is_err());
    drop(reader);
    drop(store);
    let reopened = open_state_repository(directory.path(), &genesis, budget).unwrap();
    let service = eve_storage::state::create_state_service(state_reader(&reopened));
    assert_eq!(
        eve_storage::state::read_state_service(&service)
            .unwrap()
            .commit(),
        &second
    );
}

#[test]
fn duplicate_new_commit_preflight_preserves_the_existing_synced_state_and_index() {
    let directory = tempfile::tempdir().unwrap();
    let (genesis, first, second, hash) = source_fixture();
    let mut store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    ensure_history_index(&mut store, limits()).unwrap();
    commit_state(&mut store, &first).unwrap();
    assert!(commit_state(&mut store, &second).is_err());
    let reader = state_reader(&store);
    let snapshot = capture_history_snapshot(&reader, limits()).unwrap();
    assert_eq!(snapshot.version(), &first.target);
    assert_eq!(
        lookup_transaction(&snapshot, hash).unwrap().unwrap().height,
        1
    );
}
