// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{JournalOperation, StateError, state::limits::list_length};
use alloy_rlp::Encodable;
use eve_protocol_config::records::encode_system_record;

/// Exact tagged-operation length; only system fields use bounded canonical scratch.
pub(crate) fn operation_journal_wire_length(
    operation: &JournalOperation,
) -> Result<usize, StateError> {
    // All existing tags 1..=10 have one byte of canonical RLP encoding.
    let fields = match operation {
        JournalOperation::PutAccount {
            address,
            nonce,
            balance,
            code_hash,
        } => [
            address.length(),
            nonce.length(),
            balance.length(),
            code_hash.length(),
        ],
        JournalOperation::DeleteAccount { address }
        | JournalOperation::ClearStorage { address } => [address.length(), 0, 0, 0],
        JournalOperation::PutStorage {
            address,
            slot,
            value,
        } => [address.length(), slot.length(), value.length(), 0],
        JournalOperation::DeleteStorage { address, slot } => {
            [address.length(), slot.length(), 0, 0]
        }
        JournalOperation::PutCode { code_hash, code } => {
            [code_hash.length(), code.as_ref().length(), 0, 0]
        }
        JournalOperation::DeleteCode { code_hash }
        | JournalOperation::DeleteSystem { key: code_hash } => [code_hash.length(), 0, 0, 0],
        JournalOperation::PutSystem { key, record } => [
            key.length(),
            encode_system_record(record)
                .map_err(StateError::SystemRecord)?
                .len(),
            0,
            0,
        ],
        JournalOperation::SetExecutionBlockHash { height, hash } => {
            [height.length(), hash.0.length(), 0, 0]
        }
    };
    let payload = fields
        .into_iter()
        .try_fold(1_usize, |sum, field| sum.checked_add(field))
        .ok_or(StateError::ArithmeticOverflow)?;
    list_length(payload)
}
