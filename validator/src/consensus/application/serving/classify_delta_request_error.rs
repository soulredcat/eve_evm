// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DeltaServingError;
use eve_state::StateDeltaError;

pub(super) fn classify_delta_request_error(error: StateDeltaError) -> DeltaServingError {
    match error {
        StateDeltaError::UnsupportedVersion => DeltaServingError::UnsupportedVersion,
        StateDeltaError::BudgetExceeded | StateDeltaError::AllocationFailed => {
            DeltaServingError::ResourceLimit
        }
        _ => DeltaServingError::MalformedRequest,
    }
}
