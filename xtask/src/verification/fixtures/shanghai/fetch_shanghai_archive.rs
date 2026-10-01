// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{ARCHIVE_BYTES, ARCHIVE_SHA256, ARCHIVE_URL};
use crate::{
    provisioning::{compute_artifact_digest, resolve_contained_path},
    verification::{
        commands::require_command_success::require_command_success,
        types::report_types::VerificationReport,
    },
};
use anyhow::{Result, ensure};
use std::path::{Path, PathBuf};

/// This corpus has its own exact bound; the 128 MiB tool-archive limit is unchanged.
pub(super) fn fetch_shanghai_archive(
    root: &Path,
    output: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
) -> Result<PathBuf> {
    let archive = resolve_contained_path(
        output,
        Path::new("execution-specs-tests-v20.0.2-fixtures.tar.gz"),
    )?;
    if !archive.exists() {
        let partial = resolve_contained_path(output, Path::new("fixtures.tar.gz.download"))?;
        ensure!(
            !partial.exists(),
            "incomplete corpus download needs task-owned inspection"
        );
        let limit = ARCHIVE_BYTES.to_string();
        require_command_success(
            root,
            artifacts,
            report,
            "curl",
            &[
                "--fail",
                "--location",
                "--proto",
                "=https",
                "--proto-redir",
                "=https",
                "--tlsv1.2",
                "--retry",
                "2",
                "--max-time",
                "300",
                "--max-filesize",
                &limit,
                "--output",
                partial
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("UTF-8 download path"))?,
                ARCHIVE_URL,
            ],
        )?;
        ensure!(
            std::fs::metadata(&partial)?.len() == ARCHIVE_BYTES
                && compute_artifact_digest(&partial)? == ARCHIVE_SHA256,
            "downloaded Shanghai corpus identity mismatch"
        );
        std::fs::rename(partial, &archive)?;
    }
    ensure!(
        std::fs::metadata(&archive)?.len() == ARCHIVE_BYTES
            && compute_artifact_digest(&archive)? == ARCHIVE_SHA256,
        "existing Shanghai archive identity mismatch"
    );
    Ok(archive)
}
