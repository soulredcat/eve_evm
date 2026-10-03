// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedError;
use eve_storage::records::OpaqueRecordBudget;

/// Reserve encoded row plus decoded payload before the repository allocates either.
pub(in crate::sync::applied) fn estimated_repository_read_charge(
    budget: &OpaqueRecordBudget,
) -> Result<usize, AppliedError> {
    budget
        .maximum_read_bytes
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(4_096))
        .ok_or(AppliedError::ArithmeticOverflow)
}
