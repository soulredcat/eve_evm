// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointMessageError, CheckpointMessageLimits};
pub(super) fn validate_checkpoint_chunk_message(
    total: u64,
    index: u32,
    width: u32,
    data_bytes: usize,
    limits: &CheckpointMessageLimits,
) -> Result<(), CheckpointMessageError> {
    let total = usize::try_from(total).map_err(|_| CheckpointMessageError::BudgetExceeded)?;
    let width = width as usize;
    if total == 0
        || total > limits.maximum_body_bytes
        || width == 0
        || width > limits.maximum_chunk_bytes
        || index >= 4_096
        || total.div_ceil(width) > 4_096
    {
        return Err(CheckpointMessageError::BudgetExceeded);
    }
    let offset = (index as usize)
        .checked_mul(width)
        .ok_or(CheckpointMessageError::ArithmeticOverflow)?;
    if offset >= total || data_bytes != width.min(total - offset) {
        return Err(CheckpointMessageError::MalformedEncoding);
    }
    Ok(())
}
