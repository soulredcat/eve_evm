// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::StateDeltaError;

pub(super) fn append_delta_field(
    output: &mut Vec<u8>,
    field: &[u8],
) -> Result<(), StateDeltaError> {
    let length = u32::try_from(field.len()).map_err(|_| StateDeltaError::BudgetExceeded)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(field);
    Ok(())
}
