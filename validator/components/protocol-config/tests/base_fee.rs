// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::Header;
use eve_protocol_config::headers::{HeaderError, derive_next_base_fee};

#[test]
fn base_fee_literal_rise_fall_target_floor_and_rounding_vectors() {
    for (fee, used, expected) in [
        (1_000_000_000, 0, 875_000_000),
        (1_000_000_000, 15_000_000, 1_000_000_000),
        (1_000_000_000, 30_000_000, 1_125_000_000),
        (1_000_000_000, 15_000_001, 1_000_000_008),
        (9, 0, 8),
        (1, 0, 1),
        (1, 30_000_000, 2),
        (u64::MAX, 15_000_000, u64::MAX),
    ] {
        let parent = Header {
            gas_limit: 30_000_000,
            gas_used: used,
            base_fee_per_gas: Some(fee),
            ..Default::default()
        };
        assert_eq!(derive_next_base_fee(&parent), Ok(expected));
    }
}

#[test]
fn checked_base_fee_rejects_overflow_and_malformed_parent_environment() {
    let mut parent = Header {
        gas_limit: 30_000_000,
        gas_used: 30_000_000,
        base_fee_per_gas: Some(u64::MAX),
        ..Default::default()
    };
    assert_eq!(
        derive_next_base_fee(&parent),
        Err(HeaderError::ArithmeticOverflow)
    );
    parent.base_fee_per_gas = None;
    assert_eq!(
        derive_next_base_fee(&parent),
        Err(HeaderError::InvalidEnvironment)
    );
    parent.base_fee_per_gas = Some(1);
    parent.gas_used = 30_000_001;
    assert_eq!(
        derive_next_base_fee(&parent),
        Err(HeaderError::InvalidEnvironment)
    );
    parent.gas_used = 0;
    parent.gas_limit = 1;
    assert_eq!(
        derive_next_base_fee(&parent),
        Err(HeaderError::InvalidEnvironment)
    );
}
