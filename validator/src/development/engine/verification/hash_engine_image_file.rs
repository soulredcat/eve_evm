// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{engine_file_identity, types::EngineImageIdentity};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Read, Seek},
};

pub(in crate::development::engine) fn hash_engine_image_file(
    file: &mut File,
) -> io::Result<([u8; 32], EngineImageIdentity)> {
    let identity = engine_file_identity(file)?;
    file.rewind()?;
    let mut digest = Sha256::new();
    let mut buffer = [0; 16_384];
    let mut length = 0_u64;
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        length = length.checked_add(count as u64).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "engine image length overflow")
        })?;
        if length > super::super::types::MAXIMUM_ENGINE_BINARY_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "engine image byte limit",
            ));
        }
        digest.update(&buffer[..count]);
    }
    if length != identity.length || engine_file_identity(file)? != identity {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "engine image changed during verification",
        ));
    }
    Ok((digest.finalize().into(), identity))
}
