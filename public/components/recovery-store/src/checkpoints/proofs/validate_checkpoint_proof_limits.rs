// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointProofLimits;
use crate::checkpoints::CheckpointError;

pub(super) fn validate_checkpoint_proof_limits(
    limits: &CheckpointProofLimits,
) -> Result<(), CheckpointError> {
    if limits.maximum_witness_bytes == 0
        || limits.maximum_total_bytes == 0
        || limits.maximum_files == 0
        || limits.maximum_files > 10_001
        || limits.maximum_manifest_bytes < super::types::HEADER_BYTES
        || limits.maximum_manifest_bytes > 1_048_576
        || limits.maximum_disk_bytes < limits.maximum_total_bytes
    {
        return Err(CheckpointError::InvalidLimits);
    }
    Ok(())
}
