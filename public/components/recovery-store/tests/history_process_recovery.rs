// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod state_support;
use eve_state::{
    StateCommit, compute_commit_identity, development_state_budget, encode_state_commit,
};
use eve_storage::state::{
    HistoryReadBudget, capture_history_snapshot, commit_state, create_state_service,
    development_state_storage_budget, ensure_history_index, lookup_execution_hash,
    open_state_repository, read_history_block, read_state_service, state_reader,
};

fn limits(complete: bool) -> HistoryReadBudget {
    HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: if complete { 128 } else { 1 },
        maximum_index_batch_bytes: 16 * 1_048_576,
    }
}
#[test]
fn auxiliary_synced_partial_and_complete_passes_survive_child_exit_without_destructors() {
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let second = state_support::next(&first, 2);
    let budget = development_state_storage_budget();
    if let Some(path) = std::env::var_os("EVE_HISTORY_INDEX_CHILD_PATH") {
        let complete = std::env::var_os("EVE_HISTORY_INDEX_CHILD_COMPLETE").is_some();
        let mut store =
            open_state_repository(std::path::Path::new(&path), &genesis, budget).unwrap();
        let status = ensure_history_index(&mut store, limits(complete)).unwrap();
        assert_eq!(status.complete, complete);
        assert_eq!(status.indexed_height, Some(if complete { 2 } else { 0 }));
        // Actual exit bypasses Rust/DB destructors after acknowledged sync; hardware power loss remains untested.
        std::process::exit(86);
    }
    for complete in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
        commit_state(&mut store, &first).unwrap();
        commit_state(&mut store, &second).unwrap();
        drop(store);
        let mut child = std::process::Command::new(std::env::current_exe().unwrap());
        child.args(["--exact","auxiliary_synced_partial_and_complete_passes_survive_child_exit_without_destructors"])
            .env("EVE_HISTORY_INDEX_CHILD_PATH",directory.path());
        if complete {
            child.env("EVE_HISTORY_INDEX_CHILD_COMPLETE", "1");
        }
        assert_eq!(child.status().unwrap().code(), Some(86));
        let raw = rocksdb::DB::open_default(directory.path()).unwrap();
        let mut cursor = (if complete { 2_u64 } else { 0_u64 })
            .to_be_bytes()
            .to_vec();
        cursor.extend_from_slice(
            compute_commit_identity(if complete { &second } else { &genesis }, &budget.logical)
                .unwrap()
                .as_slice(),
        );
        assert_eq!(
            raw.get(b"eve/state/history-index/v1/cursor")
                .unwrap()
                .unwrap(),
            cursor
        );
        assert_eq!(
            raw.get(b"eve/state/history-index/v1/schema")
                .unwrap()
                .is_some(),
            complete
        );
        assert!(
            raw.get(
                [
                    b"eve/state/history-index/v1/version/".as_slice(),
                    &0_u64.to_be_bytes()
                ]
                .concat()
            )
            .unwrap()
            .is_some()
        );
        assert_eq!(
            raw.get(
                [
                    b"eve/state/history-index/v1/version/".as_slice(),
                    &1_u64.to_be_bytes()
                ]
                .concat()
            )
            .unwrap()
            .is_some(),
            complete
        );
        for source in [&genesis, &first, &second] {
            assert_canonical_bytes(&raw, source);
        }
        let before_marker = raw.get(b"eve/state/v1/durable-head").unwrap().unwrap();
        drop(raw);
        let mut reopened = open_state_repository(directory.path(), &genesis, budget).unwrap();
        let reader = state_reader(&reopened);
        assert_eq!(
            capture_history_snapshot(&reader, limits(true)).is_ok(),
            complete
        );
        let service = create_state_service(state_reader(&reopened));
        assert_eq!(read_state_service(&service).unwrap().commit(), &second);
        let next = ensure_history_index(&mut reopened, limits(false)).unwrap();
        assert_eq!(next.indexed_height, Some(if complete { 2 } else { 1 }));
        if !next.complete {
            assert!(
                ensure_history_index(&mut reopened, limits(false))
                    .unwrap()
                    .complete
            );
        }
        let snapshot = capture_history_snapshot(&reader, limits(true)).unwrap();
        let sequence = snapshot.sequence();
        for source in [&genesis, &first, &second] {
            assert_eq!(
                lookup_execution_hash(&snapshot, source.target.execution_hash.0).unwrap(),
                Some(source.target.height)
            );
            assert_eq!(
                read_history_block(&snapshot, source.target.height)
                    .unwrap()
                    .unwrap()
                    .block,
                source.block
            );
        }
        assert!(
            ensure_history_index(&mut reopened, limits(true))
                .unwrap()
                .complete
        );
        assert_eq!(
            capture_history_snapshot(&reader, limits(true))
                .unwrap()
                .sequence(),
            sequence,
            "complete replay must not write again"
        );
        drop(snapshot);
        drop(reader);
        drop(service);
        drop(reopened);
        let raw = rocksdb::DB::open_default(directory.path()).unwrap();
        assert_eq!(
            raw.get(b"eve/state/v1/durable-head").unwrap().unwrap(),
            before_marker
        );
        for source in [&genesis, &first, &second] {
            assert_canonical_bytes(&raw, source);
        }
    }
}
fn assert_canonical_bytes(raw: &rocksdb::DB, source: &StateCommit) {
    assert_eq!(
        raw.get(
            [
                b"eve/state/v1/commit/".as_slice(),
                &source.target.height.to_be_bytes()
            ]
            .concat()
        )
        .unwrap()
        .unwrap(),
        encode_state_commit(source, &development_state_budget())
            .unwrap()
            .to_vec()
    );
}
