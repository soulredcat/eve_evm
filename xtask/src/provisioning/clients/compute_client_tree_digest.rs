use crate::provisioning::artifacts::compute_artifact_digest;
use anyhow::{Result, ensure};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};

/// Hash sorted module paths and content/link identities before gate reuse.
pub fn compute_client_tree_digest(directory: &Path) -> Result<String> {
    let root = directory.canonicalize()?;
    let mut pending = vec![root.clone()];
    let mut entries = BTreeMap::new();
    while let Some(parent) = pending.pop() {
        for item in std::fs::read_dir(parent)? {
            let path = item?.path();
            let metadata = std::fs::symlink_metadata(&path)?;
            let relative = path
                .strip_prefix(&root)?
                .to_string_lossy()
                .replace('\\', "/");
            ensure!(
                entries.len() < 100_000,
                "client package entry limit exceeded"
            );
            if metadata.is_dir() {
                pending.push(path);
            } else if metadata.file_type().is_symlink() {
                ensure!(
                    path.canonicalize()?.starts_with(&root),
                    "client package link escaped module root"
                );
                entries.insert(
                    relative,
                    format!("link:{}", std::fs::read_link(path)?.display()),
                );
            } else {
                ensure!(metadata.is_file(), "client package contains special file");
                entries.insert(relative, compute_artifact_digest(&path)?);
            }
        }
    }
    let mut hash = Sha256::new();
    for (path, digest) in entries {
        hash.update((path.len() as u64).to_be_bytes());
        hash.update(path.as_bytes());
        hash.update((digest.len() as u64).to_be_bytes());
        hash.update(digest.as_bytes());
    }
    Ok(hash
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}
