// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::RecoveryError;

pub(crate) fn append_length_prefixed(
    output: &mut Vec<u8>,
    input: &[u8],
) -> Result<(), RecoveryError> {
    let size = u32::try_from(input.len()).map_err(|_| RecoveryError::BudgetExceeded)?;
    output.extend_from_slice(&size.to_be_bytes());
    output.extend_from_slice(input);
    Ok(())
}
