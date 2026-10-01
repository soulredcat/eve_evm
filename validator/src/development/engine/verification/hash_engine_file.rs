// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use sha2::{Digest, Sha256};
use std::{io::Read, path::Path};

pub(in crate::development::engine) fn hash_engine_file(
    path: &Path,
    maximum: u64,
) -> Result<[u8; 32]> {
    let mut file = std::fs::File::open(path)?;
    let metadata = file.metadata()?;
    ensure!(
        metadata.is_file() && metadata.len() <= maximum,
        "engine file byte/type limit"
    );
    let mut bytes = 0_u64;
    let mut buffer = [0; 16_384];
    let mut digest = Sha256::new();
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        bytes = bytes
            .checked_add(count as u64)
            .ok_or_else(|| anyhow::anyhow!("engine file length overflow"))?;
        ensure!(bytes <= maximum, "engine file byte limit");
        digest.update(&buffer[..count]);
    }
    ensure!(bytes == metadata.len(), "engine file changed during read");
    Ok(digest.finalize().into())
}
