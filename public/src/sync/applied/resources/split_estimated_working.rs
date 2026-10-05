// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::EstimatedWorkingLease;
use crate::sync::applied::AppliedError;
use std::sync::Arc;

/// Transfer a retained charge without increasing or releasing the pool's total reservation.
pub(in crate::sync::applied) fn split_estimated_working(
    mut lease: EstimatedWorkingLease,
    retained: usize,
) -> Result<(EstimatedWorkingLease, EstimatedWorkingLease), AppliedError> {
    let retained = u64::try_from(retained).map_err(|_| AppliedError::ArithmeticOverflow)?;
    if retained == 0 || retained > lease.bytes {
        return Err(AppliedError::InvalidConfiguration);
    }
    lease.bytes -= retained;
    let retained = EstimatedWorkingLease {
        pool: Arc::clone(&lease.pool),
        bytes: retained,
    };
    Ok((retained, lease))
}
