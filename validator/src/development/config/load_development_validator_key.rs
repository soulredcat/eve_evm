// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use ed25519_dalek::SigningKey;
use std::{fs::File, io::Read, path::Path};
use zeroize::Zeroizing;

pub(crate) fn load_development_validator_key(path: &Path) -> Result<SigningKey> {
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && metadata.len() == 32,
        "signing input must be a private 32-byte development seed file"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        ensure!(
            metadata.uid() == rustix::process::geteuid().as_raw() && metadata.mode() & 0o077 == 0,
            "development signing seed ownership or permissions are unsafe"
        );
    }
    let mut seed = Zeroizing::new([0_u8; 32]);
    let mut file = File::open(path)?;
    file.read_exact(seed.as_mut())?;
    let mut extra = [0_u8; 1];
    ensure!(
        file.read(&mut extra)? == 0,
        "signing seed changed during bounded read"
    );
    Ok(SigningKey::from_bytes(&seed))
}
