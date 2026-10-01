// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{development_fixture::development_fixture, sign_type_two::sign_type_two};
use alloy_primitives::{B256, Bytes, U256};
use eve_evm::{
    ExecutionBlockInput, decode_signed_transaction, estimate_clone_reservation, execute_state_block,
};
use eve_public::mempool::{MempoolLimits, start_mempool};
use eve_state::encode_state_commit;
use std::sync::Arc;

#[tokio::test]
async fn te05_cumulative_affordability_and_invalid_second_transaction_leave_parent_atomic() {
    let fixture = development_fixture(Bytes::new());
    let before = encode_state_commit(&fixture.parent, &fixture.budget).unwrap();
    let value = U256::from(60_000) * U256::from(10_u64.pow(18));
    let first = sign_type_two(
        &fixture.key,
        0,
        fixture.secondary_sender,
        2_000_000_000,
        1000,
        value,
    );
    let second = sign_type_two(
        &fixture.key,
        1,
        fixture.secondary_sender,
        2_000_000_000,
        1000,
        value,
    );
    let pool = start_mempool(Arc::new(fixture.parent.clone()), MempoolLimits::default());
    pool.admit(
        first.clone(),
        decode_signed_transaction(&first, 31_337, 131_072).unwrap(),
    )
    .await
    .unwrap();
    let rejected = pool
        .admit(
            second.clone(),
            decode_signed_transaction(&second, 31_337, 131_072).unwrap(),
        )
        .await
        .unwrap_err();
    assert!(rejected.0.contains("cumulative sender"));
    assert_eq!(pool.select().await.unwrap().len(), 1);
    let input = ExecutionBlockInput {
        timestamp: fixture.parent.target.timestamp + 1,
        proposer: fixture.contract,
        previous_consensus_hash: B256::ZERO,
    };
    let reservation = estimate_clone_reservation(&fixture.parent.state).unwrap();
    assert!(
        execute_state_block(
            &fixture.parent,
            &input,
            &[first.clone(), second.clone()],
            &fixture.budget,
            reservation
        )
        .is_err(),
        "The second transaction must be validated against its predecessor's actual state"
    );
    assert_eq!(
        encode_state_commit(&fixture.parent, &fixture.budget).unwrap(),
        before
    );
    let one = execute_state_block(
        &fixture.parent,
        &input,
        &[first],
        &fixture.budget,
        reservation,
    )
    .unwrap();
    assert_eq!(one.commit.state.accounts[&fixture.sender].nonce, 1);
    assert!(one.commit.state.accounts[&fixture.sender].balance < value);
    assert!(
        execute_state_block(
            &one.commit,
            &ExecutionBlockInput {
                timestamp: one.commit.target.timestamp + 1,
                ..input
            },
            &[second],
            &fixture.budget,
            estimate_clone_reservation(&one.commit.state).unwrap()
        )
        .is_err()
    );
}
