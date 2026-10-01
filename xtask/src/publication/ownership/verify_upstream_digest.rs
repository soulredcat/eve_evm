// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::upstream_license::upstream_license;
use crate::structure::{
    inspection::compute_source_digest::compute_source_digest, types::policy_types::StructurePolicy,
};
use std::path::Path;

pub(super) fn verify_upstream_digest(
    root: &Path,
    path: &str,
    policy: &StructurePolicy,
) -> Vec<String> {
    let mut violations = Vec::new();
    for pin in policy
        .exclusions
        .iter()
        .filter(|pin| pin.path == path && pin.kind == "vendor")
    {
        match (&pin.sha256, compute_source_digest(root, path)) {
            (Some(expected), Ok(actual))
                if expected.len() == 64
                    && expected.bytes().all(|byte| byte.is_ascii_hexdigit())
                    && actual.eq_ignore_ascii_case(expected) => {}
            _ => violations.push(format!(
                "{path}: preserved upstream digest mismatch or missing pin"
            )),
        }
    }
    if (path.starts_with("validator/components/consensus-comet/vendor/cometbft/")
        || path.starts_with("validator/components/consensus-comet/vendor/gogoproto/")
        || (path
            .starts_with("validator/components/consensus-comet/tests/fixtures/zip215-upstream/")
            && upstream_license(path).is_some())
        || matches!(
            path,
            "validator/components/authentication/tests/fixtures/nist-acvp/prompt.json"
                | "validator/components/authentication/tests/fixtures/nist-acvp/expectedResults.json"
                | "LICENSES/Apache-2.0.txt"
                | "LICENSES/BSD-3-Clause.txt"
        ))
        && !policy
            .exclusions
            .iter()
            .any(|pin| pin.path == path && pin.kind == "vendor")
    {
        violations.push(format!(
            "{path}: known immutable upstream file lacks its reviewed exact digest"
        ));
    }
    violations
}
