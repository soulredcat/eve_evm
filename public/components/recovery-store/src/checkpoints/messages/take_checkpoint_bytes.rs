// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointMessageError, take_checkpoint_field::take_checkpoint_field};
pub(super) fn take_checkpoint_bytes<'a>(
    input: &mut &'a [u8],
    maximum: usize,
) -> Result<&'a [u8], CheckpointMessageError> {
    let length = usize::try_from(u32::from_be_bytes(take_checkpoint_field(input)?))
        .map_err(|_| CheckpointMessageError::BudgetExceeded)?;
    if length > maximum {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    let bytes = input
        .get(..length)
        .ok_or(CheckpointMessageError::MalformedEncoding)?;
    *input = &input[length..];
    Ok(bytes)
}
