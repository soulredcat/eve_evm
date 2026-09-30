use anyhow::{Result, bail};
use std::path::{Component, Path};

pub fn validate_relative_path(path: &str) -> Result<()> {
    if path.is_empty()
        || path.contains(['*', '?', '[', ']', '\\', ':'])
        || path.bytes().any(|byte| byte.is_ascii_control())
        || path
            .split('/')
            .any(|segment| matches!(segment, "" | "." | ".."))
        || Path::new(path)
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
    {
        bail!("Policy requires an exact repository-relative path: {path}");
    }
    Ok(())
}
