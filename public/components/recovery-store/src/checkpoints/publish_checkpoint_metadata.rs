// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointError, open_checkpoint_file::open_checkpoint_file,
    read_checkpoint_metadata::read_checkpoint_metadata,
};
use std::{fs::File, io::Write};

pub(super) fn publish_checkpoint_metadata(
    directory: &File,
    published: &str,
    pending: &str,
    bytes: &[u8],
) -> Result<(), CheckpointError> {
    match read_checkpoint_metadata(directory, published, bytes.len()) {
        Ok(existing) => {
            if existing != bytes {
                return Err(CheckpointError::NamespaceOccupied);
            }
            open_checkpoint_file(directory, published, false)?
                .sync_all()
                .map_err(CheckpointError::Io)?;
            directory.sync_all().map_err(CheckpointError::Io)?;
            return Ok(());
        }
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    match read_checkpoint_metadata(directory, pending, bytes.len()) {
        Ok(existing) if existing == bytes => {}
        Ok(_) => return Err(CheckpointError::NamespaceOccupied),
        Err(CheckpointError::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            let mut file = open_checkpoint_file(directory, pending, true)?;
            file.write_all(bytes).map_err(CheckpointError::Io)?;
            file.sync_all().map_err(CheckpointError::Io)?;
        }
        Err(error) => return Err(error),
    }
    let file = open_checkpoint_file(directory, pending, false)?;
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        file.set_permissions(std::fs::Permissions::from_mode(0o400))
            .map_err(CheckpointError::Io)?;
        file.sync_all().map_err(CheckpointError::Io)?;
        rustix::fs::renameat_with(
            directory,
            pending,
            directory,
            published,
            rustix::fs::RenameFlags::NOREPLACE,
        )
        .map_err(|error| CheckpointError::Io(error.into()))?;
        directory.sync_all().map_err(CheckpointError::Io)?;
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(CheckpointError::UnsupportedPlatform)
    }
}
