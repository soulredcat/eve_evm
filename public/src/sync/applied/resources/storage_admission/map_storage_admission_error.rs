// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StorageAdmissionError;
use crate::sync::applied::AppliedError;
pub(in crate::sync::applied) fn map_storage_admission_error(
    error: StorageAdmissionError,
) -> AppliedError {
    match error {
        StorageAdmissionError::InvalidConfiguration => AppliedError::InvalidConfiguration,
        StorageAdmissionError::AccountingUnavailable => AppliedError::AccountingUnavailable,
        StorageAdmissionError::Overflow => AppliedError::ArithmeticOverflow,
        StorageAdmissionError::ReadCapacity
        | StorageAdmissionError::StagingCapacity
        | StorageAdmissionError::WorkingCapacity => AppliedError::EstimatedCapacity,
    }
}
