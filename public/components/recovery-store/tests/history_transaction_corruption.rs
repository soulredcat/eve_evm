// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod history_transaction_support;
use alloy_primitives::{B256, Bytes};
use eve_state::{
    StateCommit, compute_commit_identity, decode_state_commit, development_state_budget,
};
use eve_storage::state::{
    HistoryReadBudget, capture_history_snapshot, commit_state, development_state_storage_budget,
    ensure_history_index, open_state_repository, state_reader,
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
fn malformed_missing_wrong_index_hash_and_commit_identity_history_rows_reject_reopen() {
    for defect in ["length", "index", "height", "missing", "extra", "identity"] {
        let directory = tempfile::tempdir().unwrap();
        let (genesis, first, _, hash) = source_fixture();
        let budget = development_state_storage_budget();
        let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
        commit_state(&mut store, &first).unwrap();
        ensure_history_index(&mut store, limits()).unwrap();
        drop(store);
        let raw = rocksdb::DB::open_default(directory.path()).unwrap();
        let key = [
            b"eve/state/history-index/v1/transaction/".as_slice(),
            hash.as_slice(),
        ]
        .concat();
        let location = [
            1_u64.to_be_bytes().as_slice(),
            0_u32.to_be_bytes().as_slice(),
        ]
        .concat();
        match defect {
            "length" => raw.put(&key, [0_u8; 11]).unwrap(),
            "index" => raw
                .put(
                    &key,
                    [
                        1_u64.to_be_bytes().as_slice(),
                        1_u32.to_be_bytes().as_slice(),
                    ]
                    .concat(),
                )
                .unwrap(),
            "height" => raw
                .put(
                    &key,
                    [
                        0_u64.to_be_bytes().as_slice(),
                        0_u32.to_be_bytes().as_slice(),
                    ]
                    .concat(),
                )
                .unwrap(),
            "missing" => raw.delete(&key).unwrap(),
            "extra" => raw
                .put(
                    [
                        b"eve/state/history-index/v1/transaction/".as_slice(),
                        B256::repeat_byte(99).as_slice(),
                    ]
                    .concat(),
                    location,
                )
                .unwrap(),
            "identity" => raw
                .put(
                    [b"eve/state/v1/commit-id/".as_slice(), &1_u64.to_be_bytes()].concat(),
                    B256::repeat_byte(99).as_slice(),
                )
                .unwrap(),
            _ => unreachable!(),
        }
        raw.flush_wal(true).unwrap();
        let canonical = raw
            .get([b"eve/state/v1/commit/".as_slice(), &1_u64.to_be_bytes()].concat())
            .unwrap()
            .unwrap();
        assert_eq!(
            decode_state_commit(&canonical, &development_state_budget()).unwrap(),
            first
        );
        drop(raw);
        assert!(
            open_state_repository(directory.path(), &genesis, budget).is_err(),
            "{defect}"
        );
    }
}
#[test]
fn duplicate_transaction_identity_within_block_rejects_the_bootstrap_atomically() {
    use alloy_consensus::ReceiptEnvelope;
    use alloy_eips::eip2718::{Decodable2718, Encodable2718};
    let directory = tempfile::tempdir().unwrap();
    let (genesis, first, _, hash) = source_fixture();
    let budget = development_state_storage_budget();
    let mut block = first.block.clone();
    let mut receipt_bytes = block.receipts[0].as_ref();
    let receipt = ReceiptEnvelope::decode_2718(&mut receipt_bytes).unwrap();
    let mut repeated = receipt.clone();
    let cumulative = receipt.cumulative_gas_used() * 2;
    match &mut repeated {
        ReceiptEnvelope::Legacy(receipt) => receipt.receipt.cumulative_gas_used = cumulative,
        _ => unreachable!(),
    }
    block.transactions.push(block.transactions[0].clone());
    block.receipts.push(Bytes::from(repeated.encoded_2718()));
    block.header.gas_used = cumulative;
    // Duplicate source is intentionally structurally valid but not execution-valid; canonical roots use the maintained trie library.
    let mut transaction_bytes = block.transactions[0].as_ref();
    let transaction = alloy_consensus::TxEnvelope::decode_2718(&mut transaction_bytes).unwrap();
    block.header.transactions_root =
        alloy_consensus::proofs::calculate_transaction_root(&[transaction.clone(), transaction]);
    block.header.receipts_root =
        alloy_consensus::proofs::calculate_receipt_root(&[receipt, repeated]);
    let source: StateCommit = eve_state::build_state_commit(
        Some(genesis.target.clone()),
        first.state.clone(),
        block,
        &budget.logical,
    )
    .unwrap();
    let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
    commit_state(&mut store, &source).unwrap();
    assert!(
        ensure_history_index(&mut store, limits())
            .unwrap_err()
            .to_string()
            .contains("duplicate transaction identity")
    );
    let reader = state_reader(&store);
    assert!(capture_history_snapshot(&reader, limits()).is_err());
    assert!(!hash.is_zero());
    assert_eq!(
        compute_commit_identity(&source, &budget.logical).unwrap(),
        compute_commit_identity(
            eve_storage::state::read_state_service(&eve_storage::state::create_state_service(
                state_reader(&store)
            ))
            .unwrap()
            .commit(),
            &budget.logical
        )
        .unwrap()
    );
}
