use crate::verification::{
    commands::require_command_success::require_command_success,
    types::report_types::VerificationReport,
};
use anyhow::Result;
use std::path::Path;

pub(super) fn record_git_identity(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
) -> Result<()> {
    report.revision =
        require_command_success(root, artifacts, report, "git", &["rev-parse", "HEAD"])?
            .trim()
            .into();
    report.dirty = !require_command_success(
        root,
        artifacts,
        report,
        "git",
        &["status", "--porcelain=v1", "--untracked-files=all"],
    )?
    .trim()
    .is_empty();
    Ok(())
}
