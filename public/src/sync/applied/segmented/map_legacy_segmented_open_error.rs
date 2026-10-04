// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedError, checkpoints::CheckpointAppliedError};

/// Preserve the legacy error contract without granting checkpoint recovery.
pub(super) fn map_legacy_segmented_open_error(error: CheckpointAppliedError) -> AppliedError {
    match error {
        CheckpointAppliedError::Applied(error) => error,
        CheckpointAppliedError::Segmented(error) => AppliedError::Segmented(error),
        _ => AppliedError::InvalidDurablePrefix,
    }
}
