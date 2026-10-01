// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{ReceiptEnvelope, Transaction, TxEnvelope, TxType};
use alloy_eips::eip2718::{Decodable2718, Encodable2718};
use alloy_primitives::Bloom;

use crate::{BlockPayload, StateBudget, StateError, StateIdentity};

/// Canonical recovery-envelope checks, not nonce/balance execution validation.
pub fn validate_block_payload(
    block: &BlockPayload,
    identity: &StateIdentity,
    budget: &StateBudget,
) -> Result<(), StateError> {
    if block.transactions.len() != block.receipts.len()
        || block.transactions.len() > budget.maximum_journal_operations
    {
        return Err(StateError::CommitMismatch);
    }
    let mut cumulative = 0_u64;
    let mut bloom = Bloom::ZERO;
    let mut total_bytes = 0_usize;
    for (raw_tx, raw_receipt) in block.transactions.iter().zip(&block.receipts) {
        total_bytes = total_bytes
            .checked_add(raw_tx.len())
            .and_then(|size| size.checked_add(raw_receipt.len()))
            .ok_or(StateError::ArithmeticOverflow)?;
        if raw_tx.len() > 131_072
            || raw_receipt.len() > 4_194_304
            || total_bytes > 8_388_608
            || total_bytes > budget.maximum_commit_bytes
        {
            return Err(StateError::BudgetExceeded);
        }
        let mut tx_remaining = raw_tx.as_ref();
        let transaction = TxEnvelope::decode_2718(&mut tx_remaining)
            .map_err(|_| StateError::MalformedEncoding)?;
        if !tx_remaining.is_empty()
            || transaction.encoded_2718() != raw_tx.as_ref()
            || !matches!(
                transaction.tx_type(),
                TxType::Legacy | TxType::Eip2930 | TxType::Eip1559
            )
            || transaction.chain_id() != Some(identity.evm_chain_id)
        {
            return Err(StateError::NonCanonicalEncoding);
        }
        let mut receipt_remaining = raw_receipt.as_ref();
        let receipt = ReceiptEnvelope::decode_2718(&mut receipt_remaining)
            .map_err(|_| StateError::MalformedEncoding)?;
        if !receipt_remaining.is_empty()
            || receipt.encoded_2718() != raw_receipt.as_ref()
            || receipt.tx_type() != transaction.tx_type()
            || receipt
                .as_receipt()
                .is_none_or(|receipt| !receipt.status.is_eip658())
        {
            return Err(StateError::NonCanonicalEncoding);
        }
        let next = receipt.cumulative_gas_used();
        let delta = next
            .checked_sub(cumulative)
            .ok_or(StateError::CommitMismatch)?;
        if delta == 0 || delta > transaction.gas_limit() {
            return Err(StateError::CommitMismatch);
        }
        let mut actual_bloom = Bloom::ZERO;
        if receipt.logs().iter().any(|log| log.topics().len() > 4) {
            return Err(StateError::NonCanonicalEncoding);
        }
        actual_bloom.accrue_logs(receipt.logs());
        if &actual_bloom != receipt.logs_bloom() {
            return Err(StateError::CommitMismatch);
        }
        bloom.accrue_bloom(&actual_bloom);
        cumulative = next;
    }
    if cumulative != block.header.gas_used || bloom != block.header.logs_bloom {
        return Err(StateError::CommitMismatch);
    }
    Ok(())
}
