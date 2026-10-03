// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{EstimatedWorkingLease, EstimatedWorkingPool};
use crate::sync::applied::AppliedError;
use std::sync::Arc;

pub(in crate::sync::applied) fn reserve_estimated_working(
    pool: &Arc<EstimatedWorkingPool>,
    bytes: usize,
) -> Result<EstimatedWorkingLease, AppliedError> {
    let bytes = u64::try_from(bytes).map_err(|_| AppliedError::ArithmeticOverflow)?;
    if bytes == 0 {
        return Err(AppliedError::InvalidConfiguration);
    }
    let mut accounting = pool
        .accounting
        .lock()
        .map_err(|_| AppliedError::AccountingUnavailable)?;
    let next = accounting
        .bytes
        .checked_add(bytes)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    if next > pool.limit {
        return Err(AppliedError::EstimatedCapacity);
    }
    accounting.bytes = next;
    accounting.peak = accounting.peak.max(next);
    drop(accounting);
    Ok(EstimatedWorkingLease {
        pool: Arc::clone(pool),
        bytes,
    })
}
