// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::corpus::{
    adapt_expected_fee_state::adapt_expected_fee_state,
    decode_reference_transaction::decode_reference_transaction,
    load_pinned_cases::load_pinned_cases, materialize_state::materialize_state,
    project_state::project_state, run_reference_case::run_reference_case,
};
use alloy_eips::eip2718::Encodable2718;
use eve_evm::{BlockEnvironment, FeePoolAddresses, compute_state_root, execute_serial_block};
use eve_protocol_config::native::{NODE_POOL_ADDRESS, VALIDATOR_POOL_ADDRESS};

#[test]
fn te02_eve_compatible_corpus_matches_exact_gas_receipts_and_declared_fee_delta() {
    let mut compared = 0;
    let mut unprotected = 0;
    let mut invalid = 0;
    for (name, case) in load_pinned_cases() {
        let post = &case.post["Shanghai"][0];
        let parent = materialize_state(&case.pre);
        let parent_root = compute_state_root(&parent);
        let environment = BlockEnvironment {
            chain_id: case.config.chainid.to::<u64>(),
            number: case.env.number.to::<u64>(),
            timestamp: case.env.timestamp.to::<u64>(),
            gas_limit: case.env.gas_limit.to::<u64>(),
            base_fee: case.env.base_fee.to::<u64>(),
            proposer: case.env.coinbase,
            previous_consensus_hash: case.env.random,
            maximum_transaction_bytes: 128 * 1024,
            fee_pools: FeePoolAddresses {
                node_pool: NODE_POOL_ADDRESS,
                validator_pool: VALIDATOR_POOL_ADDRESS,
            },
        };
        let result =
            execute_serial_block(&parent, &environment, std::slice::from_ref(&post.txbytes));
        assert_eq!(
            compute_state_root(&parent),
            parent_root,
            "Canonical parent mutated {name}"
        );
        if let Some(exception) = &post.expected_exception {
            assert!(
                result.is_err(),
                "Expected upstream-invalid transaction was accepted: {name} {exception}"
            );
            invalid += 1;
        } else {
            let (transaction, _) = decode_reference_transaction(&post.txbytes).unwrap();
            if transaction.chain_id.is_none() {
                assert!(
                    result.is_err(),
                    "Unprotected Ethereum transaction was accepted by EVE: {name}"
                );
                unprotected += 1;
            } else {
                let (_, reference, reference_receipt, effective_price) =
                    run_reference_case(&case).unwrap();
                assert_eq!(
                    reference_receipt.encoded_2718(),
                    post.receipt.as_ref().unwrap().rlp,
                    "Unchecked reference {name}"
                );
                let actual = result
                    .unwrap_or_else(|error| panic!("EVE compatible execution {name}: {error:?}"));
                let (expected, fees) =
                    adapt_expected_fee_state(&case, reference.tx_gas_used(), effective_price);
                assert_eq!(
                    project_state(&actual.state),
                    expected,
                    "EVE fee-adjusted complete state {name}"
                );
                assert_eq!(actual.gas_used, reference.tx_gas_used(), "Gas {name}");
                assert_eq!(
                    actual.receipts[0].encoded_2718(),
                    post.receipt.as_ref().unwrap().rlp,
                    "Receipt {name}"
                );
                assert_eq!(
                    [
                        actual.fees.burn,
                        actual.fees.node_pool,
                        actual.fees.validator_pool
                    ],
                    fees,
                    "Fee identities {name}"
                );
                assert_eq!(
                    actual.state_root,
                    compute_state_root(&materialize_state(&expected)),
                    "Adjusted root {name}"
                );
                compared += 1;
            }
        }
    }
    assert_eq!((compared, unprotected, invalid), (3118, 84, 293));
    println!(
        "EVE comparisons: 3118 compatible executions with explicit economic delta, 84 Ethereum-valid unprotected policy divergences, 293 expected invalid. Original Ethereum state-root equivalence is deliberately not claimed."
    );
}
