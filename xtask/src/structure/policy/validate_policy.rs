use crate::structure::types::policy_types::StructurePolicy;
use std::{
    collections::BTreeSet,
    path::{Component, Path},
};

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
        .chain(policy.adapters.iter().map(|entry| entry.path.as_str()));
    let mut unique = BTreeSet::new();
    for path in registrations {
        if !unique.insert(path) {
            violations.push(format!("Duplicate policy registration: {path}"));
        }
        if path.contains(['*', '?', '[', ']', '\\'])
            || Path::new(path)
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
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
        let source_code_root = ["public/", "master/", "validator/", "xtask/src/"]
            .iter()
            .any(|prefix| exclusion.path.starts_with(prefix));
        if !valid_kind
            || !identity_present
            || exclusion.reason.trim().is_empty()
            || exclusion.source.trim().is_empty()
            || (source_code_root && exclusion.path.ends_with(".rs"))
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
    violations
}
