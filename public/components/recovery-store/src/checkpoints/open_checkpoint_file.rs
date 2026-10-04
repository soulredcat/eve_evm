// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointError, validate_checkpoint_file::validate_checkpoint_file};
use std::fs::File;

pub(super) fn open_checkpoint_file(
    directory: &File,
    name: &str,
    create: bool,
) -> Result<File, CheckpointError> {
    if name.is_empty()
        || name.len() > 32
        || name == "."
        || name == ".."
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.'))
    {
        return Err(CheckpointError::UnsafeEntry);
    }
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{Mode, OFlags, openat};
        let flags = OFlags::CLOEXEC
            | OFlags::NOFOLLOW
            | OFlags::NONBLOCK
            | if create {
                OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL
            } else {
                OFlags::RDONLY
            };
        let file = File::from(
            openat(directory, name, flags, Mode::from_raw_mode(0o600))
                .map_err(|error| CheckpointError::Io(error.into()))?,
        );
        validate_checkpoint_file(&file)?;
        Ok(file)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (directory, create);
        Err(CheckpointError::UnsupportedPlatform)
    }
}
