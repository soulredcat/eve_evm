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
    ensure!(
        code == Some(0),
        "{program} {} failed; exit code {code:?}; see command evidence",
        arguments.join(" ")
    );
    Ok(stdout)
}
