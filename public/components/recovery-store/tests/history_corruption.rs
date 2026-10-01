// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod state_support;
use eve_storage::state::{
    HistoryReadBudget, development_state_storage_budget, ensure_history_index,
    open_state_repository,
};
fn limits() -> HistoryReadBudget {
    HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: 1,
        maximum_index_batch_bytes: 1_048_576,
    }
}

#[test]
fn completed_index_missing_conflicting_or_unexpected_entries_fail_startup() {
    for defect in [
        "missing", "conflict", "extra", "cursor", "schema", "version",
    ] {
        let directory = tempfile::tempdir().unwrap();
        let genesis = state_support::genesis();
        let budget = development_state_storage_budget();
        let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
        assert!(ensure_history_index(&mut store, limits()).unwrap().complete);
        drop(store);
        let raw = rocksdb::DB::open_default(directory.path()).unwrap();
        let block_key = [
            b"eve/state/history-index/v1/block-hash/".as_slice(),
            genesis.target.execution_hash.0.as_slice(),
        ]
        .concat();
        match defect {
            "missing" => raw.delete(block_key).unwrap(),
            "conflict" => raw.put(block_key, 1_u64.to_be_bytes()).unwrap(),
            "extra" => raw
                .put(
                    [
                        b"eve/state/history-index/v1/transaction/".as_slice(),
                        &[99; 32],
                    ]
                    .concat(),
                    [0; 12],
                )
                .unwrap(),
            "cursor" => raw
                .put(b"eve/state/history-index/v1/cursor", [0; 40])
                .unwrap(),
            "schema" => raw
                .put(b"eve/state/history-index/v1/schema", b"other")
                .unwrap(),
            "version" => raw
                .put(
                    [b"eve/state/history-index/v1/version/".as_slice(), &[0; 8]].concat(),
                    b"wrong",
                )
                .unwrap(),
            _ => unreachable!(),
        }
        raw.flush_wal(true).unwrap();
        drop(raw);
        assert!(
            open_state_repository(directory.path(), &genesis, budget).is_err(),
            "{defect}"
        );
    }
}

#[test]
fn oversized_bootstrap_batch_rejects_without_activating_or_modifying_source() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let budget = development_state_storage_budget();
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    let mut tiny = limits();
    tiny.maximum_index_batch_bytes = 1;
    assert!(ensure_history_index(&mut store, tiny).is_err());
    assert!(ensure_history_index(&mut store, limits()).unwrap().complete);
    drop(store);
    assert!(open_state_repository(directory.path(), &genesis, budget).is_ok());
}

#[test]
fn bootstrap_budget_above_repository_physical_cap_rejects_before_write() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let budget = development_state_storage_budget();
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    let excessive = HistoryReadBudget {
        maximum_index_batch_bytes: budget.maximum_commit_bytes + 1,
        ..limits()
    };
    assert!(
        ensure_history_index(&mut store, excessive)
            .unwrap_err()
            .to_string()
            .contains("physical batch bound")
    );
    assert!(ensure_history_index(&mut store, limits()).unwrap().complete);
}
