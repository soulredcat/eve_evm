// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointBaseError;

pub(super) fn read_checkpoint_base_array<const N: usize>(
    bytes: &[u8],
    position: usize,
) -> Result<[u8; N], CheckpointBaseError> {
    let end = position
        .checked_add(N)
        .ok_or(CheckpointBaseError::ArithmeticOverflow)?;
    bytes
        .get(position..end)
        .ok_or(CheckpointBaseError::MalformedEncoding)?
        .try_into()
        .map_err(|_| CheckpointBaseError::MalformedEncoding)
}
