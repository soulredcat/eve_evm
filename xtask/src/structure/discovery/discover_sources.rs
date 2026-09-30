use anyhow::{Context, Result, bail};
use std::{collections::BTreeSet, path::Path, process::Command};

pub fn discover_sources(root: &Path) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ])
        .current_dir(root)
        .output()
        .context("Git source discovery could not start")?;
    if !output.status.success() {
        bail!(
            "Git source discovery failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let encoded = String::from_utf8(output.stdout).context("Source paths must be UTF-8")?;
    let paths: BTreeSet<_> = encoded
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(str::to_owned)
        .collect();
    Ok(paths.into_iter().collect())
}
