// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::EstimatedWorkingObservation;
use crate::sync::applied::{AppliedError, AppliedReader};

pub fn observe_estimated_working(
    reader: &AppliedReader,
) -> Result<EstimatedWorkingObservation, AppliedError> {
    let accounting = reader
        .working
        .accounting
        .lock()
        .map_err(|_| AppliedError::AccountingUnavailable)?;
    Ok(EstimatedWorkingObservation {
        reserved_estimated_bytes: accounting.bytes,
        peak_estimated_bytes: accounting.peak,
        limit: reader.working.limit,
    })
}
