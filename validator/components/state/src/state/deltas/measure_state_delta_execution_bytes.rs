// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MAXIMUM_EXECUTION_BYTES, StateDeltaError};
use crate::{BlockPayload, StateBudget, state::limits::list_length};
use alloy_rlp::Encodable;

/// Exact maintained state block payload shape; no execution or finality validation.
pub fn measure_state_delta_execution_bytes(
    block: &BlockPayload,
    budget: &StateBudget,
) -> Result<usize, StateDeltaError> {
    if block.transactions.len() != block.receipts.len()
        || block.transactions.len() > 1_428.min(budget.maximum_journal_operations)
        || block.header.length() > 4_096
    {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let mut transaction_raw = 0_usize;
    let mut receipt_raw = 0_usize;
    let mut transaction_encoded = 0_usize;
    let mut receipt_encoded = 0_usize;
    for (transaction, receipt) in block.transactions.iter().zip(&block.receipts) {
        if transaction.len() > 131_072 || receipt.len() > 4_194_304 {
            return Err(StateDeltaError::BudgetExceeded);
        }
        transaction_raw = transaction_raw
            .checked_add(transaction.len())
            .ok_or(StateDeltaError::BudgetExceeded)?;
        receipt_raw = receipt_raw
            .checked_add(receipt.len())
            .ok_or(StateDeltaError::BudgetExceeded)?;
        transaction_encoded = transaction_encoded
            .checked_add(transaction.as_ref().length())
            .ok_or(StateDeltaError::BudgetExceeded)?;
        receipt_encoded = receipt_encoded
            .checked_add(receipt.as_ref().length())
            .ok_or(StateDeltaError::BudgetExceeded)?;
    }
    if transaction_raw > 4_194_304
        || transaction_raw
            .checked_add(receipt_raw)
            .is_none_or(|bytes| bytes > budget.maximum_commit_bytes.min(8_388_608))
    {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let transaction_list = list_length(transaction_encoded).map_err(StateDeltaError::State)?;
    let receipt_list = list_length(receipt_encoded).map_err(StateDeltaError::State)?;
    let payload = block
        .header
        .length()
        .checked_add(transaction_list)
        .and_then(|bytes| bytes.checked_add(receipt_list))
        .ok_or(StateDeltaError::BudgetExceeded)?;
    let size = list_length(payload).map_err(StateDeltaError::State)?;
    if size > budget.maximum_commit_bytes.min(MAXIMUM_EXECUTION_BYTES) {
        return Err(StateDeltaError::BudgetExceeded);
    }
    Ok(size)
}
