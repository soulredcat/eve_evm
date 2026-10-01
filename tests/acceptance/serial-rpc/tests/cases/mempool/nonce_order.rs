// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{development_fixture::development_fixture, sign_type_two::sign_type_two};
use alloy_primitives::{B256, Bytes, U256};
use eve_evm::{
    ExecutionBlockInput, decode_signed_transaction, estimate_clone_reservation, execute_state_block,
};
use eve_public::mempool::{MempoolLimits, start_mempool};
use std::sync::Arc;

#[tokio::test]
async fn ta04_future_nonce_duplicate_and_committed_revalidation_preserve_sender_order() {
    let fixture = development_fixture(Bytes::new());
    let pool = start_mempool(Arc::new(fixture.parent.clone()), MempoolLimits::default());
    let future = sign_type_two(
        &fixture.key,
        1,
        fixture.contract,
        2_000_000_000,
        1000,
        U256::ZERO,
    );
    let validated = decode_signed_transaction(&future, 31_337, 131_072).unwrap();
    let future_hash = validated.hash();
    assert_eq!(
        pool.admit(future.clone(), validated.clone()).await.unwrap(),
        future_hash
    );
    assert_eq!(
        pool.admit(future.clone(), validated).await.unwrap(),
        future_hash
    );
    assert_eq!(pool.pending_nonce(fixture.sender).await.unwrap(), 0);
    assert!(
        pool.select().await.unwrap().is_empty(),
        "A nonce gap cannot be proposed"
    );
    assert!(pool.find(future_hash).await.unwrap().is_some());
    let current = sign_type_two(
        &fixture.key,
        0,
        fixture.contract,
        2_000_000_000,
        1000,
        U256::ZERO,
    );
    let current_validated = decode_signed_transaction(&current, 31_337, 131_072).unwrap();
    let current_hash = current_validated.hash();
    pool.admit(current.clone(), current_validated)
        .await
        .unwrap();
    assert_eq!(pool.pending_nonce(fixture.sender).await.unwrap(), 2);
    let selected = pool.select().await.unwrap();
    assert_eq!(
        selected
            .iter()
            .map(|entry| entry.admission.nonce)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    assert_eq!(
        selected
            .iter()
            .map(|entry| entry.admission.hash)
            .collect::<Vec<_>>(),
        [current_hash, future_hash]
    );
    let reservation = estimate_clone_reservation(&fixture.parent.state).unwrap();
    let committed = execute_state_block(
        &fixture.parent,
        &ExecutionBlockInput {
            timestamp: fixture.parent.target.timestamp + 1,
            proposer: fixture.contract,
            previous_consensus_hash: B256::ZERO,
        },
        &[current, future.clone()],
        &fixture.budget,
        reservation,
    )
    .unwrap();
    pool.committed(Arc::new(committed.commit)).await.unwrap();
    assert_eq!(pool.pending_nonce(fixture.sender).await.unwrap(), 2);
    assert!(pool.find(current_hash).await.unwrap().is_none());
    assert!(pool.find(future_hash).await.unwrap().is_none());
    assert!(pool.select().await.unwrap().is_empty());
    assert!(
        pool.admit(
            future.clone(),
            decode_signed_transaction(&future, 31_337, 131_072).unwrap()
        )
        .await
        .is_err(),
        "An included predecessor cannot be admitted again after revalidation"
    );
}
