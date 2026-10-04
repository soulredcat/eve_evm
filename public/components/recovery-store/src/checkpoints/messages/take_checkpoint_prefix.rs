// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointMessageError, take_checkpoint_field::take_checkpoint_field};
pub(super) fn take_checkpoint_prefix(
    input: &mut &[u8],
    domain: &[u8],
) -> Result<u8, CheckpointMessageError> {
    if !input.starts_with(domain) {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    *input = &input[domain.len()..];
    let version = take_checkpoint_field::<1>(input)?[0];
    let compression = take_checkpoint_field::<1>(input)?[0];
    if version != 1 || compression != 0 {
        return Err(CheckpointMessageError::UnsupportedVersion);
    }
    Ok(take_checkpoint_field::<1>(input)?[0])
}
