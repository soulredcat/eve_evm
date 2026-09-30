use anyhow::{Result, ensure};
use sha2::{Digest, Sha256};
use std::{io::Read, path::Path};

pub(crate) fn checksum_file(path: &Path) -> Result<[u8; 32]> {
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink(),
        "snapshot reference must be an ordinary file"
    );
    let mut input = std::fs::File::open(path)?;
    let mut hash = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let count = input.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash.finalize().into())
}
