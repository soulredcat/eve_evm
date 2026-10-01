// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::FeeAllocation;
use alloy_primitives::U256;

/// Exact 40/30/30 without overflowing a 256-bit multiplication.
pub fn split_collected_fees(collected: U256) -> FeeAllocation {
    // Dividing first bounds the products; split the remainder separately.
    let divisor = U256::from(10_000);
    let quotient = collected / divisor;
    let remainder = collected % divisor;
    let burn = quotient * U256::from(4_000) + remainder * U256::from(4_000) / divisor;
    let node_pool = quotient * U256::from(3_000) + remainder * U256::from(3_000) / divisor;
    let validator_pool = collected - burn - node_pool;
    FeeAllocation {
        collected,
        burn,
        node_pool,
        validator_pool,
    }
}
