// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::Header;
use alloy_eips::eip1559::BaseFeeParams;

use super::HeaderError;

/// Checked EVE development EIP-1559 update: elasticity 2, denominator 8, floor 1.
pub fn derive_next_base_fee(parent: &Header) -> Result<u64, HeaderError> {
    let fee = parent
        .base_fee_per_gas
        .filter(|fee| *fee > 0)
        .ok_or(HeaderError::InvalidEnvironment)?;
    let target = parent.gas_limit / 2;
    if target == 0 || parent.gas_used > parent.gas_limit {
        return Err(HeaderError::InvalidEnvironment);
    }
    // The maintained helper adds in u64. Check its exact rise before calling it.
    if parent.gas_used > target {
        let increase =
            (u128::from(fee) * u128::from(parent.gas_used - target) / u128::from(target) / 8)
                .max(1);
        let increase = u64::try_from(increase).map_err(|_| HeaderError::ArithmeticOverflow)?;
        fee.checked_add(increase)
            .ok_or(HeaderError::ArithmeticOverflow)?;
    }
    Ok(parent
        .next_block_base_fee(BaseFeeParams::ethereum())
        .ok_or(HeaderError::InvalidEnvironment)?
        .max(1))
}
