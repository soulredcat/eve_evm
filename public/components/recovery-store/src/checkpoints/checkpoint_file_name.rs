// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CheckpointError;

pub(super) fn checkpoint_file_name(index: usize) -> Result<String, CheckpointError> {
    if index >= 4_096 {
        return Err(CheckpointError::InvalidManifest);
    }
    Ok(format!("chunk-{index:04}.bin"))
}
