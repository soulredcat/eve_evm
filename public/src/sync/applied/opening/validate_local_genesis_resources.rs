// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedError;
use eve_state::{DevelopmentGenesis, StateBudget};

/// Refuse oversized local genesis before canonical initialization materializes its maps.
pub(super) fn validate_local_genesis_resources(
    genesis: &DevelopmentGenesis,
    budget: &StateBudget,
) -> Result<(), AppliedError> {
    let accounts = genesis
        .accounts
        .len()
        .checked_add(1)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let system = genesis
        .validators
        .len()
        .checked_add(18)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let system_bytes = system
        .checked_mul(512)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    let codes = genesis
        .accounts
        .iter()
        .filter(|account| !account.code.is_empty())
        .count();
    let code_bytes = genesis
        .accounts
        .iter()
        .try_fold(0_usize, |sum, account| sum.checked_add(account.code.len()))
        .ok_or(AppliedError::ArithmeticOverflow)?;
    if accounts > budget.maximum_accounts
        || system > budget.maximum_system_records
        || system_bytes > budget.maximum_system_bytes
        || codes > budget.maximum_codes
        || code_bytes > budget.maximum_total_code_bytes
        || genesis
            .accounts
            .iter()
            .any(|account| account.code.len() > budget.maximum_code_bytes)
    {
        return Err(AppliedError::EstimatedCapacity);
    }
    Ok(())
}
