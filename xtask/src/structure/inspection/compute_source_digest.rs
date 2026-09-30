use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn compute_source_digest(root: &Path, path: &str) -> Result<String> {
    let bytes = std::fs::read(root.join(path))?;
    Ok(Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}
