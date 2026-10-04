// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AppliedStorageReadReservation, StorageAdmissionError, reserve_storage_read};
use crate::sync::applied::AppliedReader;
pub fn reserve_applied_storage_read(
    reader: &AppliedReader,
) -> Result<AppliedStorageReadReservation, StorageAdmissionError> {
    reserve_storage_read(&reader.storage)
}
