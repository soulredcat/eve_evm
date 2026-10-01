// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    manifest_source_targets::manifest_source_targets,
    source_edges::{
        source_edge_inventory_adapter::SourceEdgeInventoryAdapter, types::SourceEdgeInventory,
    },
    source_package_root::source_package_root,
    validate_source_target::validate_source_target,
};
use std::{collections::BTreeSet, path::Path};
use syn::visit::Visit;

pub fn validate_source_edges(root: &Path, sources: &[String]) -> Vec<String> {
    let reviewed = sources
        .iter()
        .filter_map(|source| root.join(source).canonicalize().ok())
        .collect::<BTreeSet<_>>();
    let mut violations = Vec::new();
    for source in sources {
        let absolute = root.join(source);
        let owner = source_package_root(root, &absolute);
        let mut inventory = SourceEdgeInventory::default();
        if source.ends_with("Cargo.toml") {
            match manifest_source_targets(&absolute) {
                Ok(targets) => inventory.targets.extend(targets),
                Err(error) => inventory.violations.push(error.to_string()),
            }
        } else if source.ends_with(".rs") {
            let parsed = std::fs::read_to_string(&absolute)
                .ok()
                .and_then(|source| syn::parse_file(&source).ok());
            let Some(file) = parsed else {
                continue;
            };
            let directory = absolute.parent().expect("source parent").to_path_buf();
            let stem = absolute
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("");
            let module_directory = if stem == "mod"
                || super::is_crate_root_source::is_crate_root_source(root, &absolute)
            {
                directory.clone()
            } else {
                directory.join(stem)
            };
            SourceEdgeInventoryAdapter {
                inventory: &mut inventory,
                attribute_directory: directory,
                module_directory,
                test_context: source.split('/').any(|segment| segment == "tests"),
            }
            .visit_file(&file);
        }
        violations.extend(
            inventory
                .violations
                .into_iter()
                .map(|message| format!("{source}: {message}")),
        );
        for target in inventory.targets {
            if let Err(error) =
                validate_source_target(root, &owner, &target.path, &reviewed, target.test_only)
            {
                violations.push(format!("{source}: invalid source edge: {error:#}"));
            }
        }
    }
    violations
}
