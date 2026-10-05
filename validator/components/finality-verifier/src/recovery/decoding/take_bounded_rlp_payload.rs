// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::RecoveryError;

/// Borrowing admission scan only; canonical state decoding remains state-owned.
pub(crate) fn take_bounded_rlp_payload<'a>(
    input: &mut &'a [u8],
    list: bool,
    maximum: usize,
) -> Result<&'a [u8], RecoveryError> {
    let header = alloy_rlp::Header::decode(input).map_err(|_| RecoveryError::MalformedEncoding)?;
    if header.list != list {
        return Err(RecoveryError::MalformedEncoding);
    }
    if header.payload_length > maximum {
        return Err(RecoveryError::BudgetExceeded);
    }
    let payload = input
        .get(..header.payload_length)
        .ok_or(RecoveryError::MalformedEncoding)?;
    *input = &input[header.payload_length..];
    Ok(payload)
}
