// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::StateError;

pub(crate) fn take_borrowed_bytes<'a>(
    input: &mut &'a [u8],
    maximum: usize,
) -> Result<&'a [u8], StateError> {
    let bytes =
        alloy_rlp::Header::decode_bytes(input, false).map_err(|_| StateError::MalformedEncoding)?;
    if bytes.len() > maximum {
        return Err(StateError::BudgetExceeded);
    }
    Ok(bytes)
}
