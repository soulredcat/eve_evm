// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointMessageError;
pub(super) fn append_checkpoint_bytes(
    output: &mut Vec<u8>,
    bytes: &[u8],
) -> Result<(), CheckpointMessageError> {
    let length = u32::try_from(bytes.len()).map_err(|_| CheckpointMessageError::BudgetExceeded)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}
