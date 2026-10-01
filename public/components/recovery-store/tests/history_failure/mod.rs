// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::tests::state_support;
use crate::state::{
    HistoryReadBudget, capture_history_snapshot, create_state_service,
    development_state_storage_budget, ensure_history_index, open_state_repository,
    read_state_service, state_reader, types::SimulatedCommitFailure,
};
#[test]
fn auxiliary_write_or_lost_ack_fences_until_actual_reopen_at_same_execution_version() {
    for after_sync in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let genesis = state_support::genesis();
        let budget = development_state_storage_budget();
        let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
        let reader = state_reader(&store);
        let service = create_state_service(state_reader(&store));
        let limits = HistoryReadBudget {
            maximum_block_bytes: 16 * 1_048_576,
            maximum_rebuild_blocks: 1,
            maximum_index_batch_bytes: 1_048_576,
        };
        let sequence = read_state_service(&service).unwrap().sequence();
        store.simulated_index_failure = Some(if after_sync {
            SimulatedCommitFailure::AfterSuccessfulSync
        } else {
            SimulatedCommitFailure::BeforeWrite
        });
        assert!(
            ensure_history_index(&mut store, limits)
                .unwrap_err()
                .to_string()
                .contains("SIMULATED")
        );
        assert!(capture_history_snapshot(&reader, limits).is_err());
        assert!(read_state_service(&service).is_err());
        drop(service);
        drop(reader);
        drop(store);
        let reopened = open_state_repository(directory.path(), &genesis, budget).unwrap();
        let reader = state_reader(&reopened);
        let service = create_state_service(state_reader(&reopened));
        let head = read_state_service(&service).unwrap();
        assert_eq!(head.commit(), &genesis);
        assert_eq!(
            capture_history_snapshot(&reader, limits).is_ok(),
            after_sync
        );
        if after_sync {
            assert!(head.sequence() > sequence);
        }
    }
}

#[test]
fn schema_inclusive_physical_batch_accepts_exact_budget_and_rejects_one_byte_less() {
    use crate::state::history::{
        indexing::encode_index_entries,
        keys::{INDEX_SCHEMA, INDEX_SCHEMA_KEY},
    };
    let genesis = state_support::genesis();
    let budget = development_state_storage_budget();
    let identity = eve_state::compute_commit_identity(&genesis, &budget.logical).unwrap();
    let mut exact = rocksdb::WriteBatch::default();
    for (key, value) in encode_index_entries(&genesis, identity).unwrap() {
        exact.put(key, value);
    }
    exact.put(INDEX_SCHEMA_KEY, INDEX_SCHEMA);
    let mut limits = HistoryReadBudget {
        maximum_block_bytes: 16 * 1_048_576,
        maximum_rebuild_blocks: 1,
        maximum_index_batch_bytes: exact.size_in_bytes() - 1,
    };
    let directory = tempfile::tempdir().unwrap();
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    assert!(ensure_history_index(&mut store, limits).is_err());
    limits.maximum_index_batch_bytes += 1;
    assert!(ensure_history_index(&mut store, limits).unwrap().complete);
}
