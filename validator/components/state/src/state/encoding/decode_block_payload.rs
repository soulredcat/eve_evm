// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{take_bytes, take_list};
use crate::{BlockPayload, StateBudget, StateError};
use alloy_consensus::Header;
use alloy_rlp::Decodable;

pub(crate) fn decode_block_payload(
    input: &mut &[u8],
    budget: &StateBudget,
) -> Result<BlockPayload, StateError> {
    let mut list = take_list(input)?;
    let header = Header::decode(&mut list).map_err(|_| StateError::MalformedEncoding)?;
    let mut tx_list = take_list(&mut list)?;
    let mut transactions = Vec::new();
    while !tx_list.is_empty() {
        if transactions.len() >= budget.maximum_journal_operations {
            return Err(StateError::BudgetExceeded);
        }
        transactions.push(take_bytes(&mut tx_list, 131_072)?);
    }
    let mut receipt_list = take_list(&mut list)?;
    let mut receipts = Vec::new();
    while !receipt_list.is_empty() {
        if receipts.len() >= transactions.len() {
            return Err(StateError::BudgetExceeded);
        }
        receipts.push(take_bytes(&mut receipt_list, 4_194_304)?);
    }
    if !list.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(BlockPayload {
        header,
        transactions,
        receipts,
    })
}
