// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    engine_file_identity, engine_process_image_identity,
    types::{EngineImageIdentity, VerifiedEngineImage},
};
use std::io;

/// Exact running inode/metadata binding to already hashed held bytes; no unverified constructor.
pub(crate) fn validate_engine_image_binding(
    image: &VerifiedEngineImage,
    pid: u32,
) -> io::Result<EngineImageIdentity> {
    if image.digest == [0; 32]
        || engine_file_identity(&image.file)? != image.identity
        || engine_process_image_identity(pid)? != image.identity
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "engine image differs from preverified executable",
        ));
    }
    Ok(image.identity)
}
