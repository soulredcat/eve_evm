// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{StateCommitDecodeStats, scan_commit_byte_list::scan_commit_byte_list};
use crate::{
    Bytes, StateBudget, StateError,
    state::encoding::{
        journal_decoding::{take_borrowed_bytes, take_encoded_list},
        take_list,
    },
};

/// Header topology/size only: maintained alloy decoder owns header field semantics.
pub(super) fn scan_commit_block(
    input: &mut &[u8],
    budget: &StateBudget,
    stats: &mut StateCommitDecodeStats,
) -> Result<(), StateError> {
    let mut block = take_list(input)?;
    let header = take_encoded_list(&mut block, 4_096.min(budget.maximum_commit_bytes))?;
    let mut header_remaining = header;
    let mut fields = take_list(&mut header_remaining)?;
    stats.header_encoded_bytes = header.len();
    while !fields.is_empty() {
        if stats.header_leaf_count >= 23 {
            return Err(StateError::MalformedEncoding);
        }
        let field = take_borrowed_bytes(&mut fields, 4_096)?;
        stats.header_payload_bytes = stats
            .header_payload_bytes
            .checked_add(field.len())
            .ok_or(StateError::ArithmeticOverflow)?;
        stats.header_leaf_count = stats
            .header_leaf_count
            .checked_add(1)
            .ok_or(StateError::ArithmeticOverflow)?;
    }
    if !header_remaining.is_empty() || stats.header_leaf_count < 15 {
        return Err(StateError::MalformedEncoding);
    }
    let transactions = take_list(&mut block)?;
    let receipts = take_list(&mut block)?;
    if !block.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    let raw_limit = budget.maximum_commit_bytes.min(8_388_608);
    let (transaction_count, transaction_bytes) = scan_commit_byte_list(
        transactions,
        budget.maximum_journal_operations,
        131_072,
        raw_limit,
    )?;
    let (receipt_count, receipt_bytes) = scan_commit_byte_list(
        receipts,
        transaction_count,
        4_194_304,
        raw_limit
            .checked_sub(transaction_bytes)
            .ok_or(StateError::ArithmeticOverflow)?,
    )?;
    if transaction_count != receipt_count {
        return Err(StateError::NonCanonicalEncoding);
    }
    stats.transaction_count = transaction_count;
    stats.transaction_bytes = transaction_bytes;
    stats.receipt_count = receipt_count;
    stats.receipt_bytes = receipt_bytes;
    stats.block_vector_allocation_bytes = transaction_count
        .checked_add(receipt_count)
        .and_then(|count| count.checked_mul(2 * std::mem::size_of::<Bytes>()))
        .ok_or(StateError::ArithmeticOverflow)?;
    Ok(())
}
