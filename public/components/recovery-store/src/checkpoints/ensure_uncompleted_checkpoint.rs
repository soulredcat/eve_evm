// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointError, open_checkpoint_file::open_checkpoint_file};
use std::fs::File;

pub(super) fn ensure_uncompleted_checkpoint(directory: &File) -> Result<(), CheckpointError> {
    match open_checkpoint_file(directory, "complete.bin", false) {
        Ok(_) => Err(CheckpointError::AlreadyComplete),
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}
