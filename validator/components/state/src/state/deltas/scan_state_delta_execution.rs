// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    scan_state_delta_byte_list::scan_state_delta_byte_list,
    types::{MAXIMUM_EXECUTION_BYTES, StateDeltaError, StateDeltaExecutionStats},
};
use crate::StateBudget;

/// Borrowed state-owned RLP topology admission; maintained block decoder owns fields.
pub(super) fn scan_state_delta_execution(
    bytes: &[u8],
    budget: &StateBudget,
) -> Result<StateDeltaExecutionStats, StateDeltaError> {
    if bytes.len() > budget.maximum_commit_bytes.min(MAXIMUM_EXECUTION_BYTES) {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let mut fields = alloy_rlp::Header::decode_bytes(&mut remaining, true)
        .map_err(|_| StateDeltaError::MalformedEncoding)?;
    let before = fields.len();
    alloy_rlp::Header::decode_bytes(&mut fields, true)
        .map_err(|_| StateDeltaError::MalformedEncoding)?;
    let header_bytes = before - fields.len();
    if header_bytes > 4_096 {
        return Err(StateDeltaError::BudgetExceeded);
    }
    let transactions = alloy_rlp::Header::decode_bytes(&mut fields, true)
        .map_err(|_| StateDeltaError::MalformedEncoding)?;
    let receipts = alloy_rlp::Header::decode_bytes(&mut fields, true)
        .map_err(|_| StateDeltaError::MalformedEncoding)?;
    if !remaining.is_empty() || !fields.is_empty() {
        return Err(StateDeltaError::NonCanonicalEncoding);
    }
    let (transaction_count, transaction_bytes) = scan_state_delta_byte_list(
        transactions,
        1_428.min(budget.maximum_journal_operations),
        131_072,
        4_194_304,
    )?;
    let (receipt_count, receipt_bytes) = scan_state_delta_byte_list(
        receipts,
        transaction_count,
        4_194_304,
        budget
            .maximum_commit_bytes
            .min(8_388_608)
            .checked_sub(transaction_bytes)
            .ok_or(StateDeltaError::BudgetExceeded)?,
    )?;
    if transaction_count != receipt_count {
        return Err(StateDeltaError::MalformedEncoding);
    }
    Ok(StateDeltaExecutionStats {
        encoded_bytes: bytes.len(),
        header_bytes,
        transaction_count,
        transaction_bytes,
        receipt_count,
        receipt_bytes,
    })
}
