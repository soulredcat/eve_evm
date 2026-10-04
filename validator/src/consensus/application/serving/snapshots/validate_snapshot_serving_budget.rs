// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::DeltaServingError;
use super::types::SnapshotServingBudget;
pub(in crate::consensus::application) fn validate_snapshot_serving_budget(
    budget: SnapshotServingBudget,
) -> Result<(), DeltaServingError> {
    if budget.maximum_working_bytes == 0
        || budget.maximum_working_bytes > 64 * 1_048_576
        || budget.maximum_request_bytes == 0
        || budget.maximum_request_bytes > 1_048_576
        || budget.maximum_body_bytes == 0
        || budget.maximum_body_bytes > 32 * 1_048_576
        || budget.maximum_manifest_bytes < 76
        || budget.maximum_manifest_bytes > 262_144
        || budget.maximum_chunk_bytes == 0
        || budget.maximum_chunk_bytes > 4_194_304
    {
        return Err(DeltaServingError::ResourceLimit);
    }
    Ok(())
}
