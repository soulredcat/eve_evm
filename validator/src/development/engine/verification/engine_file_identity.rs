// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::EngineImageIdentity;
use std::{fs::File, io, os::unix::fs::MetadataExt};

pub(in crate::development::engine) fn engine_file_identity(
    file: &File,
) -> io::Result<EngineImageIdentity> {
    let metadata = file.metadata()?;
    if !metadata.is_file()
        || metadata.len() == 0
        || metadata.len() > super::super::types::MAXIMUM_ENGINE_BINARY_BYTES
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "engine executable type/byte limit",
        ));
    }
    Ok(EngineImageIdentity {
        device: metadata.dev(),
        inode: metadata.ino(),
        length: metadata.len(),
        modified_seconds: metadata.mtime(),
        modified_nanos: metadata.mtime_nsec(),
        changed_seconds: metadata.ctime(),
        changed_nanos: metadata.ctime_nsec(),
    })
}
