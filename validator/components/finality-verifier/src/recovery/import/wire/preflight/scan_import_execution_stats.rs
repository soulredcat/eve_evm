// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateBudget;

use super::super::{ImportExecutionWireStats, ImportWireError};
use crate::recovery::{
    bounds::{
        scan_execution_payload::scan_execution_payload,
        scan_rlp_byte_list::scan_rlp_byte_list,
        types::{
            MAXIMUM_EXECUTION_DATA_BYTES, MAXIMUM_EXECUTION_HEADER_BYTES,
            MAXIMUM_NATIVE_DATA_BYTES, MAXIMUM_RECEIPT_BYTES, MAXIMUM_TRANSACTION_BYTES,
            MAXIMUM_TRANSACTIONS,
        },
    },
    decoding::take_bounded_rlp_payload::take_bounded_rlp_payload,
};

/// The existing scanner owns grammar validation; this pass exposes admitted counts.
pub(in crate::recovery::import::wire) fn scan_import_execution_stats(
    bytes: &[u8],
    budget: &StateBudget,
) -> Result<ImportExecutionWireStats, ImportWireError> {
    scan_execution_payload(bytes).map_err(ImportWireError::Recovery)?;
    let mut remaining = bytes;
    let mut fields = take_bounded_rlp_payload(&mut remaining, true, bytes.len())
        .map_err(ImportWireError::Recovery)?;
    let before = fields.len();
    take_bounded_rlp_payload(&mut fields, true, MAXIMUM_EXECUTION_HEADER_BYTES)
        .map_err(ImportWireError::Recovery)?;
    let header_encoded_bytes = before - fields.len();
    if header_encoded_bytes > MAXIMUM_EXECUTION_HEADER_BYTES {
        return Err(ImportWireError::BudgetExceeded);
    }
    let transactions = take_bounded_rlp_payload(&mut fields, true, bytes.len())
        .map_err(ImportWireError::Recovery)?;
    let receipts = take_bounded_rlp_payload(&mut fields, true, bytes.len())
        .map_err(ImportWireError::Recovery)?;
    let (transaction_count, transaction_bytes) = scan_rlp_byte_list(
        transactions,
        MAXIMUM_TRANSACTIONS.min(budget.maximum_journal_operations),
        MAXIMUM_TRANSACTION_BYTES,
        MAXIMUM_NATIVE_DATA_BYTES,
    )
    .map_err(ImportWireError::Recovery)?;
    let (receipt_count, receipt_bytes) = scan_rlp_byte_list(
        receipts,
        transaction_count,
        MAXIMUM_RECEIPT_BYTES,
        MAXIMUM_EXECUTION_DATA_BYTES - transaction_bytes,
    )
    .map_err(ImportWireError::Recovery)?;
    if transaction_bytes
        .checked_add(receipt_bytes)
        .is_none_or(|total| total > budget.maximum_commit_bytes)
    {
        return Err(ImportWireError::BudgetExceeded);
    }
    Ok(ImportExecutionWireStats {
        encoded_bytes: bytes.len(),
        header_encoded_bytes,
        transaction_count,
        transaction_bytes,
        receipt_count,
        receipt_bytes,
    })
}
