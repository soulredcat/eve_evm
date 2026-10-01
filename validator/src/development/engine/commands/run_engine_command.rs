// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::engine::verification::read_engine_file;
use anyhow::{Result, ensure};
use std::{
    ffi::OsString,
    fs::OpenOptions,
    path::Path,
    process::{Command, Stdio},
};

pub(in crate::development::engine) fn run_engine_command(
    binary: &Path,
    arguments: &[OsString],
    data: &Path,
    operation: &str,
) -> Result<Vec<u8>> {
    ensure!(
        matches!(operation, "version" | "init" | "node-id"),
        "unsupported engine probe operation"
    );
    let prefix = crate::development::engine::home::engine_artifact_prefix(operation)?;
    let output_path = data.join(format!("{prefix}.stdout.log"));
    let error_path = data.join(format!("{prefix}.stderr.log"));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let output = options.open(&output_path)?;
    let error = options.open(&error_path)?;
    let mut child = Command::new(binary)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::from(output))
        .stderr(Stdio::from(error))
        .spawn()?;
    let status = match super::wait_engine_command(&mut child, &output_path, &error_path) {
        Ok(status) => status,
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(error);
        }
    };
    ensure!(
        status.success(),
        "verified engine command failed; inspect private owned diagnostics"
    );
    read_engine_file(&output_path, 65_536)
}
