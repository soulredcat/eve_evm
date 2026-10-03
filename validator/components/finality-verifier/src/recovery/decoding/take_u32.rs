// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::RecoveryError;

pub(crate) fn take_u32(input: &mut &[u8]) -> Result<usize, RecoveryError> {
    let value = input.get(..4).ok_or(RecoveryError::MalformedEncoding)?;
    let value = u32::from_be_bytes(
        value
            .try_into()
            .map_err(|_| RecoveryError::MalformedEncoding)?,
    );
    *input = &input[4..];
    usize::try_from(value).map_err(|_| RecoveryError::BudgetExceeded)
}
