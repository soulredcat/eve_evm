// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AppliedSnapshotStagingReservation, StorageAdmissionError, reserve_snapshot_staging};
use crate::sync::applied::AppliedReader;
pub fn reserve_applied_snapshot_staging(
    reader: &AppliedReader,
    bytes: usize,
) -> Result<AppliedSnapshotStagingReservation, StorageAdmissionError> {
    reserve_snapshot_staging(&reader.storage, bytes)
}
