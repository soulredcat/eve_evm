// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::verification::types::report_types::VerificationReport;
use anyhow::{Result, ensure};
use std::path::Path;

pub fn require_command_success(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
    program: &str,
    arguments: &[&str],
) -> Result<String> {
    let (code, stdout) = super::run_recorded_command::run_recorded_command(
        root, artifacts, report, program, arguments,
    )?;
    let diagnostic = if code != Some(0) {
        let last = report
            .commands
            .last()
            .ok_or_else(|| anyhow::anyhow!("command evidence missing"))?;
        let stderr = std::fs::read_to_string(artifacts.join(&last.stderr))?;
        super::summarize_command_failure::summarize_command_failure(&stdout, &stderr)
    } else {
        String::new()
    };
    ensure!(
        code == Some(0),
        "{program} {} failed; exit code {code:?}; diagnostic {diagnostic}; see command evidence",
        arguments.join(" ")
    );
    Ok(stdout)
}
