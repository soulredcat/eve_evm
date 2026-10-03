// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::take_u32::take_u32;
use crate::recovery::RecoveryError;

pub(crate) fn take_length_prefixed<'a>(
    input: &mut &'a [u8],
    maximum: usize,
) -> Result<&'a [u8], RecoveryError> {
    let size = take_u32(input)?;
    if size > maximum {
        return Err(RecoveryError::BudgetExceeded);
    }
    let value = input.get(..size).ok_or(RecoveryError::MalformedEncoding)?;
    *input = &input[size..];
    Ok(value)
}
