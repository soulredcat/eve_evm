// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod state_support;
use eve_state::{B256, development_state_budget, encode_state_commit};
use eve_storage::state::{
    HistoryReadBudget, capture_history_snapshot, commit_state, development_state_storage_budget,
    ensure_history_index, lookup_execution_hash, lookup_transaction, open_state_repository,
    read_history_block, state_reader,
};
fn limits() -> HistoryReadBudget {
    HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: 1,
        maximum_index_batch_bytes: 1_048_576,
    }
}

#[test]
fn old_namespace_bootstrap_is_bounded_resumable_and_preserves_canonical_bytes() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let second = state_support::next(&first, 2);
    let budget = development_state_storage_budget();
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    commit_state(&mut store, &first).unwrap();
    commit_state(&mut store, &second).unwrap();
    let reader = state_reader(&store);
    assert!(capture_history_snapshot(&reader, limits()).is_err());
    let status = ensure_history_index(&mut store, limits()).unwrap();
    assert_eq!(status.indexed_height, Some(0));
    assert!(!status.complete);
    drop(reader);
    drop(store);
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    assert_eq!(
        ensure_history_index(&mut store, limits())
            .unwrap()
            .indexed_height,
        Some(1)
    );
    assert!(ensure_history_index(&mut store, limits()).unwrap().complete);
    let reader = state_reader(&store);
    let history = capture_history_snapshot(&reader, limits()).unwrap();
    assert_eq!(history.version(), &second.target);
    for commit in [&genesis, &first, &second] {
        let projection = read_history_block(&history, commit.target.height)
            .unwrap()
            .unwrap();
        assert_eq!(projection.version, commit.target);
        assert_eq!(projection.block, commit.block);
        assert_eq!(
            lookup_execution_hash(&history, commit.target.execution_hash.0).unwrap(),
            Some(commit.target.height)
        );
    }
    assert_eq!(
        lookup_execution_hash(&history, B256::repeat_byte(99)).unwrap(),
        None
    );
    assert_eq!(
        lookup_transaction(&history, B256::repeat_byte(99)).unwrap(),
        None
    );
    drop(history);
    drop(reader);
    drop(store);
    let raw = rocksdb::DB::open_default(directory.path()).unwrap();
    assert_eq!(
        raw.get(b"eve/state/v1/schema").unwrap().unwrap(),
        b"EVESTATE01"
    );
    for commit in [&genesis, &first, &second] {
        let key = [
            b"eve/state/v1/commit/".as_slice(),
            &commit.target.height.to_be_bytes(),
        ]
        .concat();
        assert_eq!(
            raw.get(key).unwrap().unwrap(),
            encode_state_commit(commit, &development_state_budget())
                .unwrap()
                .to_vec()
        );
    }
}

#[test]
fn atomic_new_commit_extends_index_without_invalidating_captured_history() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let mut store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    assert!(ensure_history_index(&mut store, limits()).unwrap().complete);
    let reader = state_reader(&store);
    let before = capture_history_snapshot(&reader, limits()).unwrap();
    let ack = commit_state(&mut store, &first).unwrap();
    let after = capture_history_snapshot(&reader, limits()).unwrap();
    assert_eq!(after.sequence(), ack.database_sequence);
    assert_eq!(before.version().height, 0);
    assert_eq!(after.version().height, 1);
    assert!(read_history_block(&before, 1).unwrap().is_none());
    assert_eq!(
        lookup_execution_hash(&before, first.target.execution_hash.0).unwrap(),
        None
    );
    assert_eq!(
        lookup_execution_hash(&after, first.target.execution_hash.0).unwrap(),
        Some(1)
    );
    assert_eq!(
        read_history_block(&after, 1).unwrap().unwrap().block,
        first.block
    );
}

#[test]
fn snapshot_and_projection_byte_budgets_are_enforced() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let mut budget = development_state_storage_budget();
    budget.maximum_snapshots = 1;
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    ensure_history_index(&mut store, limits()).unwrap();
    let reader = state_reader(&store);
    let mut narrow = limits();
    narrow.maximum_block_bytes = 1;
    let snapshot = capture_history_snapshot(&reader, narrow).unwrap();
    assert!(capture_history_snapshot(&reader, limits()).is_err());
    assert!(read_history_block(&snapshot, 0).is_err());
    drop(snapshot);
    assert!(capture_history_snapshot(&reader, limits()).is_ok());
}

#[test]
fn bootstrap_commits_a_bounded_byte_prefix_and_resumes_without_a_count_limit() {
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
    let bounded = HistoryReadBudget {
        maximum_rebuild_blocks: 128,
        maximum_index_batch_bytes: 1024,
        ..limits()
    };
    let first_pass = ensure_history_index(&mut store, bounded).unwrap();
    assert!(first_pass.indexed_height.is_some() && !first_pass.complete);
    for _ in 0..3 {
        if ensure_history_index(&mut store, bounded).unwrap().complete {
            return;
        }
    }
    panic!("three retained source blocks must finish within three bounded passes");
}
