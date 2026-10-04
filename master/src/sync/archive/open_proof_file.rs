// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::fs::File;

pub(super) fn open_proof_file(directory: &File, name: &str, create: bool) -> Result<File> {
    ensure!(
        !name.is_empty()
            && name.len() <= 32
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-')),
        "MASTER_PROOF_NAME"
    );
    #[cfg(target_os = "linux")]
    {
        use rustix::fs::{Mode, OFlags, openat};
        use std::os::unix::fs::MetadataExt;
        let flags = OFlags::CLOEXEC
            | OFlags::NOFOLLOW
            | OFlags::NONBLOCK
            | if create {
                OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL
            } else {
                OFlags::RDONLY
            };
        let file = File::from(openat(directory, name, flags, Mode::from_raw_mode(0o600))?);
        let metadata = file.metadata()?;
        ensure!(
            metadata.is_file() && metadata.nlink() == 1,
            "MASTER_PROOF_REQUIRES_REGULAR_UNLINKED_FILE"
        );
        Ok(file)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (directory, create);
        anyhow::bail!("MASTER_ARCHIVE_PLATFORM_REQUIRES_LINUX_DIRECTORY_SYNC");
    }
}
