// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{BorrowedJournalOperation, take_borrowed_bytes, take_encoded_list};
use crate::{
    ExecutionBlockHash, StateBudget, StateError,
    state::encoding::{decode_value, take_list},
};

pub(crate) fn decode_borrowed_operation<'a>(
    input: &mut &'a [u8],
    budget: &StateBudget,
) -> Result<BorrowedJournalOperation<'a>, StateError> {
    let mut fields = take_list(input)?;
    let operation = match decode_value::<u8>(&mut fields)? {
        1 => BorrowedJournalOperation::PutAccount {
            address: decode_value(&mut fields)?,
            nonce: decode_value(&mut fields)?,
            balance: decode_value(&mut fields)?,
            code_hash: decode_value(&mut fields)?,
        },
        2 => BorrowedJournalOperation::DeleteAccount {
            address: decode_value(&mut fields)?,
        },
        3 => BorrowedJournalOperation::PutStorage {
            address: decode_value(&mut fields)?,
            slot: decode_value(&mut fields)?,
            value: decode_value(&mut fields)?,
        },
        4 => BorrowedJournalOperation::DeleteStorage {
            address: decode_value(&mut fields)?,
            slot: decode_value(&mut fields)?,
        },
        5 => BorrowedJournalOperation::ClearStorage {
            address: decode_value(&mut fields)?,
        },
        6 => BorrowedJournalOperation::PutCode {
            code_hash: decode_value(&mut fields)?,
            code: take_borrowed_bytes(&mut fields, budget.maximum_code_bytes)?,
        },
        7 => BorrowedJournalOperation::DeleteCode {
            code_hash: decode_value(&mut fields)?,
        },
        8 => BorrowedJournalOperation::PutSystem {
            key: decode_value(&mut fields)?,
            record: take_encoded_list(&mut fields, 4_096)?,
        },
        9 => BorrowedJournalOperation::DeleteSystem {
            key: decode_value(&mut fields)?,
        },
        10 => BorrowedJournalOperation::SetExecutionBlockHash {
            height: decode_value(&mut fields)?,
            hash: ExecutionBlockHash(decode_value(&mut fields)?),
        },
        _ => return Err(StateError::MalformedEncoding),
    };
    if !fields.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    Ok(operation)
}
