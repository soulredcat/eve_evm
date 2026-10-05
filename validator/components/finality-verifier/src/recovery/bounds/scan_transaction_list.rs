// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MAXIMUM_NATIVE_DATA_BYTES, MAXIMUM_TRANSACTION_BYTES, MAXIMUM_TRANSACTIONS};
use crate::recovery::{
    RecoveryError,
    decoding::{take_length_prefixed::take_length_prefixed, take_u32::take_u32},
};

pub(crate) fn scan_transaction_list(mut input: &[u8]) -> Result<(), RecoveryError> {
    let count = take_u32(&mut input)?;
    if count > MAXIMUM_TRANSACTIONS {
        return Err(RecoveryError::BudgetExceeded);
    }
    let mut total = 0_usize;
    for _ in 0..count {
        let transaction = take_length_prefixed(&mut input, MAXIMUM_TRANSACTION_BYTES)?;
        total = total
            .checked_add(transaction.len())
            .ok_or(RecoveryError::BudgetExceeded)?;
        if total > MAXIMUM_NATIVE_DATA_BYTES {
            return Err(RecoveryError::BudgetExceeded);
        }
    }
    if !input.is_empty() {
        return Err(RecoveryError::NonCanonicalEncoding);
    }
    Ok(())
}
