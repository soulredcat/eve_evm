// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod state_support;
use eve_storage::state::{
    HistoryReadBudget, capture_history_snapshot, commit_state, development_state_storage_budget,
    ensure_history_index, open_state_repository, read_history_block, state_reader,
};
fn limits() -> HistoryReadBudget {
    HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: 1,
        maximum_index_batch_bytes: 1_048_576,
    }
}
#[test]
fn projection_budget_includes_parent_version_roots_and_commit_identity_at_exact_boundary() {
    let directory = tempfile::tempdir().unwrap();
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let mut store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    commit_state(&mut store, &first).unwrap();
    while !ensure_history_index(&mut store, limits()).unwrap().complete {}
    drop(store);
    let raw = rocksdb::DB::open_default(directory.path()).unwrap();
    let mut exact = 0_usize;
    for prefix in [
        b"eve/state/history-index/v1/version/".as_slice(),
        b"eve/state/v1/header/",
        b"eve/state/v1/transactions/",
        b"eve/state/v1/receipts/",
        b"eve/state/v1/roots/",
        b"eve/state/v1/commit-id/",
    ] {
        exact += raw
            .get([prefix, &1_u64.to_be_bytes()].concat())
            .unwrap()
            .unwrap()
            .len();
    }
    exact += raw
        .get(
            [
                b"eve/state/history-index/v1/version/".as_slice(),
                &0_u64.to_be_bytes(),
            ]
            .concat(),
        )
        .unwrap()
        .unwrap()
        .len();
    drop(raw);
    let store = open_state_repository(
        directory.path(),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    let reader = state_reader(&store);
    let fits = HistoryReadBudget {
        maximum_block_bytes: exact,
        ..limits()
    };
    assert_eq!(
        read_history_block(&capture_history_snapshot(&reader, fits).unwrap(), 1)
            .unwrap()
            .unwrap()
            .block,
        first.block
    );
    let too_small = HistoryReadBudget {
        maximum_block_bytes: exact - 1,
        ..fits
    };
    assert!(
        read_history_block(&capture_history_snapshot(&reader, too_small).unwrap(), 1)
            .unwrap_err()
            .to_string()
            .contains("byte capacity")
    );
}
