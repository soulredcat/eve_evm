// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::EstimatedWorkingLease;
use crate::sync::applied::AppliedError;
use std::sync::Arc;

/// Combine retained charges in the same pool without releasing accounted capacity.
pub(in crate::sync::applied) fn merge_estimated_working(
    mut first: EstimatedWorkingLease,
    mut second: EstimatedWorkingLease,
) -> Result<EstimatedWorkingLease, AppliedError> {
    if !Arc::ptr_eq(&first.pool, &second.pool) {
        return Err(AppliedError::InvalidConfiguration);
    }
    let total = first
        .bytes
        .checked_add(second.bytes)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    first.bytes = total;
    second.bytes = 0;
    Ok(first)
}
