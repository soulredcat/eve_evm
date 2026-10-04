// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, open_checkpoint_file::open_checkpoint_file,
    validate_checkpoint_file::validate_checkpoint_file,
};
use std::{fs::File, io::Read};

pub(super) fn read_checkpoint_metadata(
    directory: &File,
    name: &str,
    maximum: usize,
) -> Result<Vec<u8>, CheckpointError> {
    let mut file = open_checkpoint_file(directory, name, false)?;
    let length = usize::try_from(validate_checkpoint_file(&file)?)
        .map_err(|_| CheckpointError::InvalidManifest)?;
    if length > maximum {
        return Err(CheckpointError::InvalidManifest);
    }
    let mut bytes = vec![0_u8; length];
    file.read_exact(&mut bytes).map_err(CheckpointError::Io)?;
    let mut extra = [0_u8; 1];
    if file.read(&mut extra).map_err(CheckpointError::Io)? != 0
        || validate_checkpoint_file(&file)? != length as u64
    {
        return Err(CheckpointError::InvalidManifest);
    }
    Ok(bytes)
}
