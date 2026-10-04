// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StorageAdmissionError;
use crate::sync::applied::AppliedError;
pub(super) fn map_working_control_error(error: AppliedError) -> StorageAdmissionError {
    match error {
        AppliedError::AccountingUnavailable => StorageAdmissionError::AccountingUnavailable,
        AppliedError::ArithmeticOverflow => StorageAdmissionError::Overflow,
        AppliedError::InvalidConfiguration => StorageAdmissionError::InvalidConfiguration,
        _ => StorageAdmissionError::WorkingCapacity,
    }
}
