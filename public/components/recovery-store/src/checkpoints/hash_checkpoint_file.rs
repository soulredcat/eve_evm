// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointError, validate_checkpoint_file::validate_checkpoint_file};
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read};

pub(super) fn hash_checkpoint_file(
    file: &mut File,
    length: usize,
    body: &mut Sha256,
) -> Result<[u8; 32], CheckpointError> {
    if usize::try_from(validate_checkpoint_file(file)?).ok() != Some(length) {
        return Err(CheckpointError::CorruptChunk);
    }
    let mut hash = Sha256::new();
    let mut buffer = [0_u8; 65_536];
    let mut remaining = length;
    while remaining != 0 {
        let count = remaining.min(buffer.len());
        file.read_exact(&mut buffer[..count])
            .map_err(CheckpointError::Io)?;
        hash.update(&buffer[..count]);
        body.update(&buffer[..count]);
        remaining -= count;
    }
    let mut extra = [0_u8; 1];
    if file.read(&mut extra).map_err(CheckpointError::Io)? != 0
        || usize::try_from(validate_checkpoint_file(file)?).ok() != Some(length)
    {
        return Err(CheckpointError::CorruptChunk);
    }
    Ok(hash.finalize().into())
}
