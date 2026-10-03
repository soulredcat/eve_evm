// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{DevelopmentGenesis, StateBudget, StateError};
use eve_protocol_config::{genesis::validate_development_genesis, network::LaunchMode};

/// Conservative logical initialization charge from actual borrowed genesis inputs.
/// Caller reserves bounded codec/validation scratch before sizing and retains the
/// input separately. This does not claim exact allocator/RSS or final-state sizes.
pub fn estimate_genesis_initialization_reservation(
    genesis: &DevelopmentGenesis,
    budget: &StateBudget,
) -> Result<usize, StateError> {
    crate::state::limits::validate_state_budget(budget)?;
    validate_development_genesis(LaunchMode::Development, genesis).map_err(StateError::Genesis)?;
    let accounts = genesis
        .accounts
        .len()
        .checked_add(1)
        .ok_or(StateError::ArithmeticOverflow)?;
    // The canonical initializer has 17 parameter records, one fee record and one
    // record per validator. Scheduled upgrades remain in the bounded genesis spec.
    let system = genesis
        .validators
        .len()
        .checked_add(18)
        .ok_or(StateError::ArithmeticOverflow)?;
    if accounts > budget.maximum_accounts || system > budget.maximum_system_records {
        return Err(StateError::BudgetExceeded);
    }
    let mut codes = 0_usize;
    let mut code_bytes = 0_usize;
    for account in &genesis.accounts {
        if account.code.len() > budget.maximum_code_bytes {
            return Err(StateError::BudgetExceeded);
        }
        if !account.code.is_empty() {
            codes = codes.checked_add(1).ok_or(StateError::ArithmeticOverflow)?;
            code_bytes = code_bytes
                .checked_add(account.code.len())
                .ok_or(StateError::ArithmeticOverflow)?;
        }
    }
    // Repeated code counts stay conservative rather than assuming deduplication
    // before the canonical initializer hashes and materializes the actual state.
    let serialized = accounts
        .checked_mul(256)
        .and_then(|bytes| bytes.checked_add(code_bytes))
        .and_then(|bytes| {
            system
                .checked_mul(4_096)
                .and_then(|records| bytes.checked_add(records))
        })
        .and_then(|bytes| {
            genesis
                .upgrades
                .len()
                .checked_mul(512)
                .and_then(|upgrades| bytes.checked_add(upgrades))
        })
        .and_then(|bytes| bytes.checked_add(4_096))
        .ok_or(StateError::ArithmeticOverflow)?;
    [
        (accounts, 512),
        (codes, 256),
        (code_bytes, 4),
        (system, 512),
        (serialized, 8),
    ]
    .into_iter()
    .try_fold(2_097_152_usize, |sum, (count, factor)| {
        sum.checked_add(count.checked_mul(factor)?)
    })
    .ok_or(StateError::ArithmeticOverflow)
}
