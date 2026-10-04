// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AppliedStorageObservation, StorageAdmissionError, observe_storage_admission};
use crate::sync::applied::AppliedReader;
pub fn observe_applied_storage(
    reader: &AppliedReader,
) -> Result<AppliedStorageObservation, StorageAdmissionError> {
    observe_storage_admission(&reader.storage)
}
