// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::verification::{
    commands::require_command_success::require_command_success,
    discovery::{
        parse_test_inventory::parse_test_inventory, parse_test_results::parse_test_results,
    },
    types::{
        manifest_types::TestGroup,
        report_types::{GroupEvidence, VerificationReport},
    },
};
use anyhow::Result;
use std::path::Path;

pub(super) fn run_test_group(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
    group: TestGroup,
) -> Result<()> {
    let feature_names = group.features.join(",");
    let mut inventory = vec!["test", "--locked", "--all-targets", "-p", &group.package];
    if !group.features.is_empty() {
        inventory.extend(["--features", &feature_names]);
    }
    inventory.extend(["--", "--list"]);
    let listing = require_command_success(root, artifacts, report, env!("CARGO"), &inventory)?;
    let tests = parse_test_inventory(&listing, &group.tests)?;
    let mut execution = vec!["test", "--locked", "--all-targets", "-p", &group.package];
    if !group.features.is_empty() {
        execution.extend(["--features", &feature_names]);
    }
    execution.extend(["--", "--test-threads=1", "--nocapture"]);
    let program = if group.package == "eve-storage" {
        execution.insert(0, env!("CARGO"));
        execution.insert(0, "-v");
        "/usr/bin/time"
    } else {
        env!("CARGO")
    };
    let results = require_command_success(root, artifacts, report, program, &execution)?;
    let passed = parse_test_results(&results, tests.len())?
        + super::run_doc_tests::run_doc_tests(
            root,
            artifacts,
            report,
            &group.package,
            &group.doc_tests,
            &group.features,
        )?;
    report.test_count += passed;
    report.groups.push(GroupEvidence {
        package: group.package,
        requirements: group.requirements,
        discovered: tests.len() + group.doc_tests.len(),
        passed,
        ignored: 0,
    });
    Ok(())
}
