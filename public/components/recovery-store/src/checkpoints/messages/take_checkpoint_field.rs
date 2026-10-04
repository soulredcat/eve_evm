// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointMessageError;
pub(super) fn take_checkpoint_field<const N: usize>(
    input: &mut &[u8],
) -> Result<[u8; N], CheckpointMessageError> {
    let field = input
        .get(..N)
        .ok_or(CheckpointMessageError::MalformedEncoding)?
        .try_into()
        .map_err(|_| CheckpointMessageError::MalformedEncoding)?;
    *input = &input[N..];
    Ok(field)
}
