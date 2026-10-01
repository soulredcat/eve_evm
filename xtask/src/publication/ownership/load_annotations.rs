// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    types::{AnnotationInventory, ReusePolicy},
    value_items::value_items,
};
use crate::structure::{
    inspection::resolve_source_path::resolve_source_path,
    policy::validate_relative_path::validate_relative_path,
};
use anyhow::{Context, Result};
use std::path::Path;

pub(super) fn load_annotations(root: &Path, sources: &[String]) -> Result<AnnotationInventory> {
    let encoded = std::fs::read_to_string(resolve_source_path(root, "REUSE.toml")?)?;
    let policy: ReusePolicy =
        toml::from_str(&encoded).context("Parse exact ownership REUSE policy")?;
    let mut inventory = AnnotationInventory::default();
    if policy.version != 1 {
        inventory
            .violations
            .push("Unsupported ownership REUSE policy version".into());
    }
    for annotation in policy.annotations {
        if annotation.precedence.as_deref().unwrap_or("closest") != "closest" {
            inventory
                .violations
                .push("Ownership annotations require closest precedence".into());
        }
        if value_items(&annotation.path).is_empty() {
            inventory
                .violations
                .push("Empty ownership annotation path list".into());
        }
        for path in value_items(&annotation.path) {
            if validate_relative_path(path).is_err() {
                inventory.violations.push(format!(
                    "{path}: ownership annotation requires an exact normal path"
                ));
                continue;
            }
            if !sources.contains(path) || resolve_source_path(root, path).is_err() {
                inventory
                    .violations
                    .push(format!("{path}: stale or unsafe ownership annotation"));
                continue;
            }
            if inventory
                .annotations
                .insert(path.clone(), annotation.clone())
                .is_some()
            {
                inventory
                    .violations
                    .push(format!("{path}: overlapping ownership annotations"));
            }
        }
    }
    Ok(inventory)
}
