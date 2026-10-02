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
    if image.digest == [0; 32] {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "ENGINE_VERIFIED_IMAGE_MISSING",
        ));
    }
    if engine_file_identity(&image.file)? != image.identity {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "ENGINE_HELD_IMAGE_CHANGED",
        ));
    }
    let actual = engine_process_image_identity(pid)
        .map_err(|error| io::Error::new(error.kind(), "ENGINE_PROCESS_IMAGE_UNREADABLE"))?;
    let expected = image.identity;
    let mismatch = [
        (
            actual.device != expected.device,
            "ENGINE_PROCESS_DEVICE_MISMATCH",
        ),
        (
            actual.inode != expected.inode,
            "ENGINE_PROCESS_INODE_MISMATCH",
        ),
        (
            actual.length != expected.length,
            "ENGINE_PROCESS_LENGTH_MISMATCH",
        ),
        (
            actual.modified_seconds != expected.modified_seconds
                || actual.modified_nanos != expected.modified_nanos,
            "ENGINE_PROCESS_MTIME_MISMATCH",
        ),
        (
            actual.changed_seconds != expected.changed_seconds
                || actual.changed_nanos != expected.changed_nanos,
            "ENGINE_PROCESS_CTIME_MISMATCH",
        ),
    ]
    .into_iter()
    .find_map(|(different, code)| different.then_some(code));
    if let Some(code) = mismatch {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, code));
    }
    Ok(actual)
}
