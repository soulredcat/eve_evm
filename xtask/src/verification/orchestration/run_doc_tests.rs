// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::verification::{
    commands::require_command_success::require_command_success,
    discovery::{
        parse_test_inventory::parse_test_inventory, parse_test_results::parse_test_results,
    },
    types::report_types::VerificationReport,
};
use anyhow::{Result, ensure};
use std::path::Path;

pub(super) fn run_doc_tests(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
    package: &str,
    required: &[String],
) -> Result<usize> {
    let listing = require_command_success(
        root,
        artifacts,
        report,
        env!("CARGO"),
        &["test", "--locked", "-p", package, "--doc", "--", "--list"],
    )?;
    if required.is_empty() {
        ensure!(
            !listing.lines().any(|line| line.ends_with(": test")),
            "{package}: documentation tests lack registration"
        );
        return Ok(0);
    }
    let tests = parse_test_inventory(&listing, required)?;
    let results = require_command_success(
        root,
        artifacts,
        report,
        env!("CARGO"),
        &[
            "test",
            "--locked",
            "-p",
            package,
            "--doc",
            "--",
            "--test-threads=1",
            "--nocapture",
        ],
    )?;
    parse_test_results(&results, tests.len())
}
