// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::checkpoints::CheckpointError;

pub(super) fn checkpoint_proof_file_name(index: usize) -> Result<String, CheckpointError> {
    if index >= 10_001 {
        return Err(CheckpointError::InvalidManifest);
    }
    Ok(format!("witness-{index:05}.bin"))
}
