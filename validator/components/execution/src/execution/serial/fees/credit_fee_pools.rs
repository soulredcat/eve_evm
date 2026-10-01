// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::FeeAllocation;
use crate::execution::serial::block::{BlockExecutionError, FeePoolAddresses};
use revm::database::{AccountState, InMemoryDB};

/// Credit funded escrows once after the complete ordered block succeeds.
pub(crate) fn credit_fee_pools(
    state: &mut InMemoryDB,
    pools: FeePoolAddresses,
    allocation: FeeAllocation,
) -> Result<(), BlockExecutionError> {
    for (address, amount) in [
        (pools.node_pool, allocation.node_pool),
        (pools.validator_pool, allocation.validator_pool),
    ] {
        if amount.is_zero() {
            continue;
        }
        let account = state
            .load_account(address)
            .expect("the serial oracle is backed by infallible EmptyDB");
        account.info.balance = account
            .info
            .balance
            .checked_add(amount)
            .ok_or(BlockExecutionError::ArithmeticOverflow)?;
        account.account_state = AccountState::Touched;
    }
    Ok(())
}
