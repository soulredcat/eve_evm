// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    scan_rlp_byte_list::scan_rlp_byte_list,
    types::{
        MAXIMUM_EXECUTION_DATA_BYTES, MAXIMUM_EXECUTION_HEADER_BYTES, MAXIMUM_NATIVE_DATA_BYTES,
        MAXIMUM_RECEIPT_BYTES, MAXIMUM_RECOVERY_BYTES, MAXIMUM_TRANSACTION_BYTES,
        MAXIMUM_TRANSACTIONS,
    },
};
use crate::recovery::{
    RecoveryError, decoding::take_bounded_rlp_payload::take_bounded_rlp_payload,
};

/// Preflights existing state-owned RLP topology before allocating decoded fields.
pub(crate) fn scan_execution_payload(mut input: &[u8]) -> Result<(), RecoveryError> {
    let mut block = take_bounded_rlp_payload(&mut input, true, MAXIMUM_RECOVERY_BYTES)?;
    if !input.is_empty() {
        return Err(RecoveryError::NonCanonicalEncoding);
    }
    take_bounded_rlp_payload(&mut block, true, MAXIMUM_EXECUTION_HEADER_BYTES)?;
    let transactions = take_bounded_rlp_payload(&mut block, true, MAXIMUM_RECOVERY_BYTES)?;
    let receipts = take_bounded_rlp_payload(&mut block, true, MAXIMUM_RECOVERY_BYTES)?;
    if !block.is_empty() {
        return Err(RecoveryError::NonCanonicalEncoding);
    }
    let (count, transaction_bytes) = scan_rlp_byte_list(
        transactions,
        MAXIMUM_TRANSACTIONS,
        MAXIMUM_TRANSACTION_BYTES,
        MAXIMUM_NATIVE_DATA_BYTES,
    )?;
    let (receipt_count, _) = scan_rlp_byte_list(
        receipts,
        count,
        MAXIMUM_RECEIPT_BYTES,
        MAXIMUM_EXECUTION_DATA_BYTES - transaction_bytes,
    )?;
    if count != receipt_count {
        return Err(RecoveryError::MalformedEncoding);
    }
    Ok(())
}
