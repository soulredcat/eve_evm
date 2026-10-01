// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::Header;
use alloy_primitives::U256;
use eve_evm::split_collected_fees;
use eve_protocol_config::headers::{HeaderError, derive_next_base_fee};

#[test]
fn te04_base_fee_matches_literal_eip1559_rise_fall_target_and_floor() {
    for (base_fee, gas_used, expected) in [
        (1_000_000_000, 0, 875_000_000),
        (1_000_000_000, 15_000_000, 1_000_000_000),
        (1_000_000_000, 30_000_000, 1_125_000_000),
        (1, 0, 1),
        (1, 15_000_000, 1),
        (1, 15_000_001, 2),
        (9, 0, 8),
    ] {
        let parent = Header {
            gas_limit: 30_000_000,
            gas_used,
            base_fee_per_gas: Some(base_fee),
            ..Header::default()
        };
        assert_eq!(derive_next_base_fee(&parent).unwrap(), expected);
    }
}

#[test]
fn te04_invalid_or_overflowing_parent_base_fee_is_rejected() {
    for parent in [
        Header {
            gas_limit: 0,
            base_fee_per_gas: Some(1),
            ..Header::default()
        },
        Header {
            gas_limit: 1,
            base_fee_per_gas: Some(1),
            ..Header::default()
        },
        Header {
            gas_limit: 30_000_000,
            gas_used: 30_000_001,
            base_fee_per_gas: Some(1),
            ..Header::default()
        },
        Header {
            gas_limit: 30_000_000,
            base_fee_per_gas: None,
            ..Header::default()
        },
        Header {
            gas_limit: 30_000_000,
            base_fee_per_gas: Some(0),
            ..Header::default()
        },
    ] {
        assert_eq!(
            derive_next_base_fee(&parent),
            Err(HeaderError::InvalidEnvironment)
        );
    }
    let parent = Header {
        gas_limit: 30_000_000,
        gas_used: 30_000_000,
        base_fee_per_gas: Some(u64::MAX),
        ..Header::default()
    };
    assert_eq!(
        derive_next_base_fee(&parent),
        Err(HeaderError::ArithmeticOverflow)
    );
}

#[test]
fn te04_odd_wei_remainder_belongs_to_validator_and_conserves_collected_fees() {
    for (collected, burn, node, validator) in [
        (0_u64, 0_u64, 0_u64, 0_u64),
        (1, 0, 0, 1),
        (3, 1, 0, 2),
        (10, 4, 3, 3),
        (21_003, 8_401, 6_300, 6_302),
    ] {
        let allocation = split_collected_fees(U256::from(collected));
        assert_eq!(allocation.collected, U256::from(collected));
        assert_eq!(allocation.burn, U256::from(burn));
        assert_eq!(allocation.node_pool, U256::from(node));
        assert_eq!(allocation.validator_pool, U256::from(validator));
        assert_eq!(
            allocation.burn + allocation.node_pool + allocation.validator_pool,
            allocation.collected
        );
    }
}
