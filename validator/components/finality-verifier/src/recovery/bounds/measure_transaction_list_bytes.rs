// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MAXIMUM_NATIVE_DATA_BYTES, MAXIMUM_TRANSACTION_BYTES, MAXIMUM_TRANSACTIONS};
use crate::recovery::RecoveryError;
use eve_state::Bytes;

pub(crate) fn measure_transaction_list_bytes(
    transactions: &[Bytes],
) -> Result<usize, RecoveryError> {
    if transactions.len() > MAXIMUM_TRANSACTIONS {
        return Err(RecoveryError::BudgetExceeded);
    }
    let mut total = 0_usize;
    for transaction in transactions {
        if transaction.len() > MAXIMUM_TRANSACTION_BYTES {
            return Err(RecoveryError::BudgetExceeded);
        }
        total = total
            .checked_add(transaction.len())
            .ok_or(RecoveryError::BudgetExceeded)?;
        if total > MAXIMUM_NATIVE_DATA_BYTES {
            return Err(RecoveryError::BudgetExceeded);
        }
    }
    Ok(4 + transactions.len() * 4 + total)
}
