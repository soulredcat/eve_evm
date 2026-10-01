// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    inspect_ownership_file::inspect_ownership_file,
    load_annotations::load_annotations,
    types::{OwnershipFile, OwnershipReport},
    validate_license_texts::validate_license_texts,
};
use crate::structure::{
    discovery::discover_sources::discover_sources,
    inspection::resolve_source_path::resolve_source_path, policy::load_policy::load_policy,
};
use anyhow::{Context, Result};
use std::path::Path;

/// Check the approved ownership scheme over Git's publishable inventory.
pub fn check_ownership(root: &Path) -> Result<OwnershipReport> {
    let root = root
        .canonicalize()
        .context("Resolve ownership repository root")?;
    let sources = discover_sources(&root)?;
    let inventory = load_annotations(&root, &sources)?;
    let policy = load_policy(&resolve_source_path(&root, "config/structure-policy.toml")?)?;
    let mut report = OwnershipReport {
        policy_version: 1,
        violations: inventory.violations.clone(),
        ..OwnershipReport::default()
    };
    report
        .violations
        .extend(validate_license_texts(&root, &sources, &inventory));
    for path in sources {
        let file =
            inspect_ownership_file(&root, &path, &inventory, &policy).unwrap_or_else(|error| {
                OwnershipFile {
                    path: path.clone(),
                    kind: "unreadable".into(),
                    violations: vec![format!("{path}: {error:#}")],
                }
            });
        match file.kind.as_str() {
            "inline_first_party" => report.coverage.inline_first_party += 1,
            "annotated_first_party" => report.coverage.annotated_first_party += 1,
            "upstream" => report.coverage.upstream += 1,
            "license_text" => report.coverage.license_text += 1,
            _ => {}
        }
        report.violations.extend(file.violations.iter().cloned());
        report.files.push(file);
    }
    Ok(report)
}
