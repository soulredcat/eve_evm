// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::BorrowedJournalOperation;
use crate::{Bytes, JournalOperation, StateError, state::encoding::decode_system_record};

pub(crate) fn materialize_operation(
    operation: BorrowedJournalOperation<'_>,
) -> Result<JournalOperation, StateError> {
    Ok(match operation {
        BorrowedJournalOperation::PutAccount {
            address,
            nonce,
            balance,
            code_hash,
        } => JournalOperation::PutAccount {
            address,
            nonce,
            balance,
            code_hash,
        },
        BorrowedJournalOperation::DeleteAccount { address } => {
            JournalOperation::DeleteAccount { address }
        }
        BorrowedJournalOperation::PutStorage {
            address,
            slot,
            value,
        } => JournalOperation::PutStorage {
            address,
            slot,
            value,
        },
        BorrowedJournalOperation::DeleteStorage { address, slot } => {
            JournalOperation::DeleteStorage { address, slot }
        }
        BorrowedJournalOperation::ClearStorage { address } => {
            JournalOperation::ClearStorage { address }
        }
        BorrowedJournalOperation::PutCode { code_hash, code } => JournalOperation::PutCode {
            code_hash,
            code: Bytes::copy_from_slice(code),
        },
        BorrowedJournalOperation::DeleteCode { code_hash } => {
            JournalOperation::DeleteCode { code_hash }
        }
        BorrowedJournalOperation::PutSystem { key, record } => JournalOperation::PutSystem {
            key,
            record: decode_system_record(record)?,
        },
        BorrowedJournalOperation::DeleteSystem { key } => JournalOperation::DeleteSystem { key },
        BorrowedJournalOperation::SetExecutionBlockHash { height, hash } => {
            JournalOperation::SetExecutionBlockHash { height, hash }
        }
    })
}
