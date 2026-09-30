use anyhow::{Result, ensure};
use std::{io::Read, path::Path};

pub fn read_bounded_input(path: &Path, maximum: usize) -> Result<Vec<u8>> {
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink(),
        "input must be a regular file"
    );
    ensure!(
        metadata.len() <= maximum as u64,
        "development input exceeds byte budget"
    );
    let file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.take(maximum as u64 + 1).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= maximum,
        "development input grew beyond byte budget"
    );
    Ok(bytes)
}
