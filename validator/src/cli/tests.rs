// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[test]
fn cli_failure_summary_reports_fixed_stage_and_errno_without_secret_text() {
    let error = anyhow::Error::from(rustix::io::Errno::PERM)
        .context("ENGINE_PIDFD_OPEN_FAILED")
        .context("NODE_ENGINE_LAUNCH_FAILED");
    assert_eq!(
        super::summarize_cli_failure(&error),
        "ENGINE_PIDFD_OPEN_FAILED,IO_PERMISSION_DENIED,NODE_ENGINE_LAUNCH_FAILED"
    );
    let error = anyhow::anyhow!("NODE_ENGINE_LAUNCH_FAILED seed=never-publish /private/path");
    assert_eq!(
        super::summarize_cli_failure(&error),
        "REDACTED_UNCLASSIFIED"
    );
    let error = anyhow::Error::from(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "seed=never-publish /private/path",
    ))
    .context("NODE_ASSEMBLY_FAILED");
    assert_eq!(
        super::summarize_cli_failure(&error),
        "IO_NOT_FOUND,NODE_ASSEMBLY_FAILED"
    );
}
