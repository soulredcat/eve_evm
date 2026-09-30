use crate::provisioning::paths::resolve_contained_path;
use anyhow::{Context, Result, ensure};
use std::{
    path::Path,
    process::{Command, Stdio},
};

/// Execute explicit arguments; callers supply reviewed operations, never shell programs.
pub fn run_checked_command(command: &mut Command, output: &Path, label: &str) -> Result<()> {
    let program = command.get_program().to_owned();
    let logs = resolve_contained_path(output, Path::new("logs"))?;
    std::fs::create_dir_all(&logs)?;
    let path = resolve_contained_path(&logs, Path::new(&format!("{label}.log")))?;
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    command
        .stdout(Stdio::from(file.try_clone()?))
        .stderr(Stdio::from(file));
    let status = command
        .status()
        .with_context(|| format!("cannot execute {program:?}"))?;
    eprintln!(
        "provision {label}: {status}; local-only log {}",
        path.display()
    );
    ensure!(
        status.success(),
        "{program:?} failed with {status}; inspect local-only log {}",
        path.display()
    );
    Ok(())
}
