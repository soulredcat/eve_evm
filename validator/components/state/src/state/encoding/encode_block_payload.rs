// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_list;
use crate::{BlockPayload, StateBudget, StateError};
use alloy_rlp::Encodable;

/// Canonical bounded wire encoding; this does not validate execution, roots or finality.
pub fn encode_block_payload(
    block: &BlockPayload,
    budget: &StateBudget,
) -> Result<Vec<u8>, StateError> {
    if block.transactions.len() != block.receipts.len() {
        return Err(StateError::CommitMismatch);
    }
    if block.transactions.len() > budget.maximum_journal_operations {
        return Err(StateError::BudgetExceeded);
    }
    let mut raw_bytes = 0_usize;
    let mut transaction_bytes = 0_usize;
    let mut receipt_bytes = 0_usize;
    for (transaction, receipt) in block.transactions.iter().zip(&block.receipts) {
        raw_bytes = raw_bytes
            .checked_add(transaction.len())
            .and_then(|length| length.checked_add(receipt.len()))
            .ok_or(StateError::ArithmeticOverflow)?;
        if transaction.len() > 131_072
            || receipt.len() > 4_194_304
            || raw_bytes > 8_388_608
            || raw_bytes > budget.maximum_commit_bytes
        {
            return Err(StateError::BudgetExceeded);
        }
        transaction_bytes = transaction_bytes
            .checked_add(transaction.as_ref().length())
            .ok_or(StateError::ArithmeticOverflow)?;
        receipt_bytes = receipt_bytes
            .checked_add(receipt.as_ref().length())
            .ok_or(StateError::ArithmeticOverflow)?;
    }
    let payload_length = [
        block.header.length(),
        alloy_rlp::Header {
            list: true,
            payload_length: transaction_bytes,
        }
        .length_with_payload(),
        alloy_rlp::Header {
            list: true,
            payload_length: receipt_bytes,
        }
        .length_with_payload(),
    ]
    .into_iter()
    .try_fold(0_usize, |length, field| {
        length
            .checked_add(field)
            .ok_or(StateError::ArithmeticOverflow)
    })?;
    let encoded_length = payload_length
        .checked_add(
            alloy_rlp::Header {
                list: true,
                payload_length,
            }
            .length(),
        )
        .ok_or(StateError::ArithmeticOverflow)?;
    if encoded_length > budget.maximum_commit_bytes {
        return Err(StateError::BudgetExceeded);
    }
    let transactions = block
        .transactions
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    let receipts = block
        .receipts
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    Ok(encode_list(&[
        alloy_rlp::encode(&block.header),
        encode_list(&transactions),
        encode_list(&receipts),
    ]))
}
