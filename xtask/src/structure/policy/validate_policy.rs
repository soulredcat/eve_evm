// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::types::policy_types::StructurePolicy;
use std::collections::BTreeSet;

pub fn validate_policy(policy: &StructurePolicy, sources: &[String]) -> Vec<String> {
    let mut violations = Vec::new();
    if policy.version != 1 || policy.current_bulk > 11 {
        violations.push("Unsupported policy version or current bulk".into());
    }
    let registrations = policy
        .exclusions
        .iter()
        .map(|entry| entry.path.as_str())
        .chain(policy.size_reviews.iter().map(|entry| entry.path.as_str()))
        .chain(policy.exceptions.iter().map(|entry| entry.path.as_str()))
        .chain(policy.adapters.iter().map(|entry| entry.path.as_str()))
        .chain(
            policy
                .generated_modules
                .iter()
                .map(|entry| entry.path.as_str()),
        );
    let mut unique = BTreeSet::new();
    for path in registrations {
        if !unique.insert(path) {
            violations.push(format!("Duplicate policy registration: {path}"));
        }
        if super::validate_relative_path::validate_relative_path(path).is_err() {
            violations.push(format!(
                "Policy requires an exact repository-relative path: {path}"
            ));
        }
        if !sources.iter().any(|source| source == path) {
            violations.push(format!("Stale policy registration: {path}"));
        }
    }
    for exclusion in &policy.exclusions {
        let valid_kind = matches!(exclusion.kind.as_str(), "generated" | "vendor");
        let identity_present = if exclusion.kind == "generated" {
            exclusion
                .generator
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty())
        } else {
            exclusion.sha256.as_ref().is_some_and(|hash| {
                hash.len() == 64 && hash.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
        };
        if !valid_kind
            || !identity_present
            || exclusion.reason.trim().is_empty()
            || exclusion.source.trim().is_empty()
            || exclusion.path.ends_with(".rs")
        {
            violations.push(format!(
                "Invalid generated/vendor exclusion: {}",
                exclusion.path
            ));
        }
    }
    for adapter in &policy.adapters {
        if adapter.external_trait.trim().is_empty()
            || adapter.reason.trim().is_empty()
            || adapter.reviewer.trim().is_empty()
        {
            violations.push(format!("Incomplete adapter review: {}", adapter.path));
        }
    }
    for generated in &policy.generated_modules {
        for path in [&generated.generator, &generated.source_manifest] {
            if super::validate_relative_path::validate_relative_path(path).is_err()
                || !sources.contains(path)
            {
                violations.push(format!("Invalid generated module input: {path}"));
            }
        }
    }
    violations
}
