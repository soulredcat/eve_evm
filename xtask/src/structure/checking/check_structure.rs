use crate::structure::{
    discovery::discover_sources::discover_sources,
    inspection::inspect_file::inspect_file,
    policy::{load_policy::load_policy, validate_policy::validate_policy},
    types::report_types::StructureReport,
};
use anyhow::{Context, Result};
use std::path::Path;

pub fn check_structure(root: &Path, policy_path: &Path) -> Result<StructureReport> {
    let root = root.canonicalize().context("Resolve repository root")?;
    let policy = load_policy(&root.join(policy_path))?;
    let sources = discover_sources(&root)?;
    let mut report = StructureReport {
        policy_version: policy.version,
        current_bulk: policy.current_bulk,
        violations: validate_policy(&policy, &sources),
        ..StructureReport::default()
    };
    report
        .violations
        .extend(super::validate_role_dependencies::validate_role_dependencies(&root, &sources));
    for path in sources {
        if let Some(exclusion) = policy.exclusions.iter().find(|entry| entry.path == path) {
            if let Some(expected) = &exclusion.sha256 {
                let actual =
                    crate::structure::inspection::compute_source_digest::compute_source_digest(
                        &root, &path,
                    )?;
                if !actual.eq_ignore_ascii_case(expected) {
                    report
                        .violations
                        .push(format!("{path}: excluded source digest mismatch"));
                }
            }
            report.exclusions.push(path);
            continue;
        }
        match inspect_file(&root, &path, &policy) {
            Ok((file, warnings)) => {
                report.violations.extend(file.violations.iter().cloned());
                report.warnings.extend(warnings);
                report.files.push(file);
            }
            Err(error) => report.violations.push(format!("{path}: {error:#}")),
        }
    }
    Ok(report)
}
