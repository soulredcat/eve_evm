// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointTransfer,
    types::{ChunkReference, HEADER_BYTES, REFERENCE_BYTES},
};

pub(super) fn read_checkpoint_reference(
    transfer: &CheckpointTransfer,
    index: usize,
) -> Result<ChunkReference, CheckpointError> {
    if index >= transfer.summary.chunks {
        return Err(CheckpointError::InvalidManifest);
    }
    let position = HEADER_BYTES + transfer.summary.version_bytes + index * REFERENCE_BYTES;
    let bytes = &transfer.manifest[position..position + REFERENCE_BYTES];
    Ok(ChunkReference {
        offset: usize::try_from(u64::from_be_bytes(bytes[4..12].try_into().unwrap()))
            .map_err(|_| CheckpointError::InvalidManifest)?,
        length: usize::try_from(u32::from_be_bytes(bytes[12..16].try_into().unwrap()))
            .map_err(|_| CheckpointError::InvalidManifest)?,
        hash: bytes[16..48].try_into().unwrap(),
    })
}
