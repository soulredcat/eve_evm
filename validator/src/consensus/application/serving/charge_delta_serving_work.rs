// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DeltaServingError;

pub(super) fn charge_delta_serving_work(
    used: &mut usize,
    additional: usize,
    maximum: usize,
) -> Result<(), DeltaServingError> {
    let total = used
        .checked_add(additional)
        .ok_or(DeltaServingError::ResourceLimit)?;
    if total > maximum {
        return Err(DeltaServingError::ResourceLimit);
    }
    *used = total;
    Ok(())
}
