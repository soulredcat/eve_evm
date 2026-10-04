// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{reserve_estimated_working, types::AppliedWorkingReservation};
use crate::sync::applied::{AppliedError, AppliedReader};
/// Real service-pool lease for caller transport/assembly lifetimes; no verification authority or RSS claim.
pub fn reserve_applied_working(
    reader: &AppliedReader,
    bytes: usize,
) -> Result<AppliedWorkingReservation, AppliedError> {
    let lease = reserve_estimated_working(&reader.working, bytes)?;
    Ok(AppliedWorkingReservation { _lease: lease })
}
