// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointError, checkpoint_namespace_name::checkpoint_namespace_name};
use std::fs::File;

pub(super) fn open_checkpoint_namespace(
    base: &File,
    id: &[u8; 32],
    create: bool,
) -> Result<File, CheckpointError> {
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{FlockOperation, Mode, OFlags, flock, mkdirat, openat};
        use std::os::unix::fs::MetadataExt;
        if !base.metadata().map_err(CheckpointError::Io)?.is_dir() {
            return Err(CheckpointError::UnsafeEntry);
        }
        let name = checkpoint_namespace_name(id);
        if create {
            match mkdirat(base, name.as_str(), Mode::from_raw_mode(0o700)) {
                Ok(()) => base.sync_all().map_err(CheckpointError::Io)?,
                Err(rustix::io::Errno::EXIST) => {}
                Err(error) => return Err(CheckpointError::Io(error.into())),
            }
        }
        let directory = File::from(
            openat(
                base,
                name.as_str(),
                OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                Mode::empty(),
            )
            .map_err(|error| CheckpointError::Io(error.into()))?,
        );
        let metadata = directory.metadata().map_err(CheckpointError::Io)?;
        if metadata.uid() != rustix::process::geteuid().as_raw() || metadata.mode() & 0o077 != 0 {
            return Err(CheckpointError::UnsafeEntry);
        }
        flock(&directory, FlockOperation::NonBlockingLockExclusive)
            .map_err(|error| CheckpointError::Io(error.into()))?;
        directory.sync_all().map_err(CheckpointError::Io)?;
        base.sync_all().map_err(CheckpointError::Io)?;
        Ok(directory)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (base, id, create);
        Err(CheckpointError::UnsupportedPlatform)
    }
}
