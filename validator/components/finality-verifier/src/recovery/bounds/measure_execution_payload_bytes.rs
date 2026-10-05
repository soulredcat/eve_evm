// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    measure_transaction_list_bytes::measure_transaction_list_bytes,
    types::{MAXIMUM_EXECUTION_DATA_BYTES, MAXIMUM_EXECUTION_HEADER_BYTES, MAXIMUM_RECEIPT_BYTES},
};
use crate::recovery::RecoveryError;
use alloy_rlp::Encodable;
use eve_state::BlockPayload;

/// Measures the existing canonical RLP payload; this function serializes no data.
pub(crate) fn measure_execution_payload_bytes(
    block: &BlockPayload,
) -> Result<usize, RecoveryError> {
    measure_transaction_list_bytes(&block.transactions)?;
    if block.receipts.len() != block.transactions.len() {
        return Err(RecoveryError::MalformedEncoding);
    }
    let header = block.header.length();
    if header > MAXIMUM_EXECUTION_HEADER_BYTES {
        return Err(RecoveryError::BudgetExceeded);
    }
    let mut raw_total = 0_usize;
    let mut transactions = 0_usize;
    let mut receipts = 0_usize;
    for (transaction, receipt) in block.transactions.iter().zip(&block.receipts) {
        if receipt.len() > MAXIMUM_RECEIPT_BYTES {
            return Err(RecoveryError::BudgetExceeded);
        }
        raw_total = raw_total
            .checked_add(transaction.len())
            .and_then(|size| size.checked_add(receipt.len()))
            .ok_or(RecoveryError::BudgetExceeded)?;
        if raw_total > MAXIMUM_EXECUTION_DATA_BYTES {
            return Err(RecoveryError::BudgetExceeded);
        }
        transactions += transaction.as_ref().length();
        receipts += receipt.as_ref().length();
    }
    let body = header
        + alloy_rlp::Header {
            list: true,
            payload_length: transactions,
        }
        .length()
        + transactions
        + alloy_rlp::Header {
            list: true,
            payload_length: receipts,
        }
        .length()
        + receipts;
    Ok(alloy_rlp::Header {
        list: true,
        payload_length: body,
    }
    .length()
        + body)
}
