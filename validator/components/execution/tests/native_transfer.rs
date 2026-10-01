// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
#[path = "support/build_typed_transaction.rs"]
mod typed_fixture;

use alloy_eips::eip2930::{AccessList, AccessListItem};
use alloy_primitives::{Bytes, TxKind, U256, address};
use eve_evm::{compute_state_root, execute_serial_block, split_collected_fees};

#[test]
fn transfer_charges_exact_fees_without_extra_proposer_credit() {
    let parent = support::state();
    let recipient = address!("00000000000000000000000000000000000000bb");
    let block = support::environment();
    let before_root = compute_state_root(&parent);
    let raw = support::legacy(
        0,
        TxKind::Call(recipient),
        U256::from(42),
        21_000,
        Bytes::new(),
    );
    let result = execute_serial_block(&parent, &block, &[raw]).unwrap();
    let collected = U256::from(42_000_000_000_000_u64);
    assert_eq!(result.gas_used, 21_000);
    assert_eq!(result.fees, split_collected_fees(collected));
    assert_eq!(result.fees.burn, U256::from(16_800_000_000_000_u64));
    assert_eq!(
        result.state.cache.accounts[&recipient].info.balance,
        U256::from(42)
    );
    assert_eq!(
        result.state.cache.accounts[&support::sender()].info.nonce,
        1
    );
    assert_eq!(
        result.state.cache.accounts[&support::sender()].info.balance,
        parent.cache.accounts[&support::sender()].info.balance - collected - U256::from(42)
    );
    assert_eq!(
        result.state.cache.accounts[&block.fee_pools.node_pool]
            .info
            .balance,
        U256::from(12_600_000_000_000_u64)
    );
    assert_eq!(
        result.state.cache.accounts[&block.fee_pools.validator_pool]
            .info
            .balance,
        U256::from(12_600_000_000_000_u64)
    );
    assert!(!result.state.cache.accounts.contains_key(&block.proposer));
    assert_eq!(compute_state_root(&parent), before_root);
}

#[test]
fn accepted_typed_envelopes_preserve_pricing_and_access_list_gas() {
    let recipient = address!("00000000000000000000000000000000000000bb");
    let access_list = AccessList(vec![AccessListItem {
        address: recipient,
        storage_keys: vec![],
    }]);
    let raw_type_one = typed_fixture::build_typed_transaction(1, access_list, 0, recipient);
    let raw_type_two =
        typed_fixture::build_typed_transaction(2, AccessList::default(), 1, recipient);
    let result = execute_serial_block(
        &support::state(),
        &support::environment(),
        &[raw_type_one, raw_type_two],
    )
    .unwrap();
    assert_eq!(result.outcomes[0].execution.tx_gas_used(), 23_400);
    assert_eq!(result.outcomes[1].execution.tx_gas_used(), 21_000);
    assert_eq!(result.outcomes[1].effective_gas_price, 1_500_000_000);
    assert_eq!(result.gas_used, 44_400);
    assert_eq!(
        result.state.cache.accounts[&recipient].info.balance,
        U256::from(84)
    );
    assert_eq!(
        result.state.cache.accounts[&support::sender()].info.nonce,
        2
    );
    assert_eq!(result.fees.collected, U256::from(78_300_000_000_000_u64));
}

#[test]
fn repeated_execution_is_deterministic() {
    let parent = support::state();
    let block = support::environment();
    let raw = support::legacy(
        0,
        TxKind::Call(block.proposer),
        U256::from(9),
        21_000,
        Bytes::new(),
    );
    let a = execute_serial_block(&parent, &block, std::slice::from_ref(&raw)).unwrap();
    let b = execute_serial_block(&parent, &block, &[raw]).unwrap();
    assert_eq!(a.state_root, b.state_root);
    assert_eq!(a.transactions_root, b.transactions_root);
    assert_eq!(a.receipts_root, b.receipts_root);
    assert_eq!(a.receipts, b.receipts);
    assert_eq!(
        a.state.cache.accounts[&block.proposer].info.balance,
        U256::from(9)
    );
}

#[test]
fn fee_rounding_conserves_even_the_maximum_256_bit_amount() {
    for collected in [U256::ZERO, U256::from(1), U256::from(7), U256::MAX] {
        let fees = split_collected_fees(collected);
        assert_eq!(fees.burn + fees.node_pool + fees.validator_pool, collected);
    }
    let odd = split_collected_fees(U256::from(7));
    assert_eq!(
        (odd.burn, odd.node_pool, odd.validator_pool),
        (U256::from(2), U256::from(2), U256::from(3))
    );
}
