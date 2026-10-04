// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, CheckpointTransfer, checkpoint_completion_bytes::checkpoint_completion_bytes,
    read_checkpoint_metadata::read_checkpoint_metadata,
};

pub(super) fn validate_checkpoint_metadata(
    transfer: &CheckpointTransfer,
) -> Result<(), CheckpointError> {
    if read_checkpoint_metadata(&transfer.directory, "manifest.bin", transfer.manifest.len())?
        != transfer.manifest
    {
        return Err(CheckpointError::NamespaceOccupied);
    }
    let completion = checkpoint_completion_bytes(&transfer.summary);
    for (name, expected) in [
        ("manifest.pending", transfer.manifest.as_slice()),
        ("complete.pending", completion.as_slice()),
        ("complete.bin", completion.as_slice()),
    ] {
        match read_checkpoint_metadata(&transfer.directory, name, expected.len()) {
            Ok(bytes) if bytes == expected => {}
            Ok(_) => return Err(CheckpointError::NamespaceOccupied),
            Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}
