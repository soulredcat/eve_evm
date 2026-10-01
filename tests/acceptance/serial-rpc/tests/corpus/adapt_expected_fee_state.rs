// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{FixtureAccount, FixtureCase};
use alloy_primitives::{Address, Bytes, U256};
use eve_protocol_config::native::{NODE_POOL_ADDRESS, VALIDATOR_POOL_ADDRESS};
use std::collections::BTreeMap;

/// Declared economic difference after an independently verified upstream result.
pub fn adapt_expected_fee_state(
    case: &FixtureCase,
    charged_gas: u64,
    effective_price: u128,
) -> (BTreeMap<Address, FixtureAccount>, [U256; 3]) {
    let mut expected = case.post["Shanghai"][0].state.clone();
    for account in expected.values_mut() {
        account.storage.retain(|_, value| !value.is_zero());
    }
    assert!(
        !case.pre.contains_key(&case.env.coinbase),
        "Review beneficiary-prestate collisions explicitly"
    );
    assert!(!expected.contains_key(&NODE_POOL_ADDRESS));
    assert!(!expected.contains_key(&VALIDATOR_POOL_ADDRESS));
    let fee = U256::from(charged_gas) * U256::from(effective_price);
    let tip =
        U256::from(charged_gas) * U256::from(effective_price - case.env.base_fee.to::<u128>());
    if !tip.is_zero() {
        let beneficiary = expected
            .get_mut(&case.env.coinbase)
            .expect("Vanilla priority fee must appear in the expected beneficiary");
        beneficiary.balance = beneficiary
            .balance
            .checked_sub(tip)
            .expect("Expected beneficiary contains its exact vanilla priority fee");
        if beneficiary.balance.is_zero()
            && beneficiary.nonce.is_zero()
            && beneficiary.code.is_empty()
            && beneficiary.storage.is_empty()
        {
            expected.remove(&case.env.coinbase);
        }
    }
    let burn = fee * U256::from(4_000) / U256::from(10_000);
    let node = fee * U256::from(3_000) / U256::from(10_000);
    let validator = fee - burn - node;
    for (address, balance) in [
        (NODE_POOL_ADDRESS, node),
        (VALIDATOR_POOL_ADDRESS, validator),
    ] {
        expected.insert(
            address,
            FixtureAccount {
                nonce: U256::ZERO,
                balance,
                code: Bytes::new(),
                storage: BTreeMap::new(),
            },
        );
    }
    (expected, [burn, node, validator])
}
