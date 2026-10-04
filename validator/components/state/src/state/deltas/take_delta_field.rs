// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::StateDeltaError;

pub(super) fn take_delta_field<'a>(
    input: &mut &'a [u8],
    maximum: usize,
) -> Result<&'a [u8], StateDeltaError> {
    let prefix = input.get(..4).ok_or(StateDeltaError::MalformedEncoding)?;
    let length = u32::from_be_bytes(
        prefix
            .try_into()
            .map_err(|_| StateDeltaError::MalformedEncoding)?,
    ) as usize;
    if length > maximum {
        return Err(StateDeltaError::BudgetExceeded);
    }
    *input = &input[4..];
    let field = input
        .get(..length)
        .ok_or(StateDeltaError::MalformedEncoding)?;
    *input = &input[length..];
    Ok(field)
}
