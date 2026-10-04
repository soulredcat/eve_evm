// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// One profile/feature selection for listing and execution. The caller appends
/// libtest flags after `--`; no executable, shell or environment override is used.
pub fn cargo_test_arguments<'a>(
    package: &'a str,
    features: &'a str,
    doc_tests: bool,
    release: bool,
) -> Vec<&'a str> {
    let mut arguments = if doc_tests {
        vec!["test", "--locked", "-p", package, "--doc"]
    } else {
        vec!["test", "--locked", "--all-targets", "-p", package]
    };
    if release {
        arguments.push("--release");
    }
    if !features.is_empty() {
        arguments.extend(["--features", features]);
    }
    arguments
}
