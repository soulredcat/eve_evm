// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{TestFixture, transaction_signer::signed_transaction};
use alloy_primitives::{Address, B256, Bytes, keccak256};
use eve_evm::{ExecutionBlockInput, PreparedStateBlock, execute_state_block};
use eve_state::{development_state_budget, initialize_development_state};
use k256::ecdsa::SigningKey;

pub(in crate::development::acceptance::enabled::tests) fn transition_input(action: u8) -> Bytes {
    let mut input = keccak256(b"transition(uint8)")[..4].to_vec();
    input.extend_from_slice(&[0; 31]);
    input.push(action);
    input.into()
}

pub(in crate::development::acceptance::enabled::tests) fn execute_fixture(
    fixture: &TestFixture,
    key: &SigningKey,
    inputs: &[Bytes],
) -> PreparedStateBlock {
    let budget = development_state_budget();
    let genesis = initialize_development_state(&fixture.genesis, &budget).unwrap();
    let transactions = inputs
        .iter()
        .enumerate()
        .map(|(nonce, input)| {
            signed_transaction(
                key,
                u64::try_from(nonce).unwrap(),
                fixture.contract,
                200_000,
                input.clone(),
            )
        })
        .collect::<Vec<_>>();
    execute_state_block(
        &genesis,
        &ExecutionBlockInput {
            timestamp: 2,
            proposer: Address::repeat_byte(4),
            previous_consensus_hash: B256::ZERO,
        },
        &transactions,
        &budget,
        64 * 1_048_576,
    )
    .unwrap()
}
