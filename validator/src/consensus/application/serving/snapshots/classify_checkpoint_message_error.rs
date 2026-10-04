// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::DeltaServingError;
use eve_storage::checkpoints::messages::CheckpointMessageError;
pub(super) fn classify_checkpoint_message_error(
    error: CheckpointMessageError,
) -> DeltaServingError {
    match error {
        CheckpointMessageError::UnsupportedVersion => DeltaServingError::UnsupportedVersion,
        CheckpointMessageError::BudgetExceeded
        | CheckpointMessageError::ArithmeticOverflow
        | CheckpointMessageError::ReservationTooSmall
        | CheckpointMessageError::AllocationFailed => DeltaServingError::ResourceLimit,
        _ => DeltaServingError::MalformedRequest,
    }
}
