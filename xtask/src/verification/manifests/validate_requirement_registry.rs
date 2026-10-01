// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::inspection::resolve_source_path::resolve_source_path;
use crate::verification::types::manifest_types::{GateRegistry, RequirementManifest};
use anyhow::{Result, ensure};
use std::{collections::BTreeMap, path::Path};

pub fn validate_requirement_registry(root: &Path) -> Result<BTreeMap<String, String>> {
    let registry: GateRegistry = toml::from_str(&std::fs::read_to_string(resolve_source_path(
        root,
        "config/gates/registry.toml",
    )?)?)?;
    let mut status = BTreeMap::new();
    for (file, prefix, start, end) in [
        ("majority", "T-M", 1, 10),
        ("post-quantum", "T-P", 1, 10),
        ("public-persistence", "T-N", 9, 12),
    ] {
        let path = format!("config/gates/requirements/{file}.toml");
        let manifest: RequirementManifest =
            toml::from_str(&std::fs::read_to_string(resolve_source_path(root, &path)?)?)?;
        ensure!(
            manifest.version == 1 && manifest.cases.len() == end - start + 1,
            "Missing or unsupported requirement registry {path}"
        );
        for index in start..=end {
            let id = format!("{prefix}{index:02}");
            let case = manifest
                .cases
                .iter()
                .find(|case| case.id == id)
                .ok_or_else(|| anyhow::anyhow!("Missing requirement {id}"))?;
            ensure!(
                !case.expected.trim().is_empty()
                    && !case.fixture.trim().is_empty()
                    && !case.owner_gate.trim().is_empty(),
                "Incomplete expected outcome/fixture/dependency: {id}"
            );
            ensure!(
                ["NOT_IMPLEMENTED", "IMPLEMENTED"].contains(&case.status.as_str()),
                "Invalid requirement status {id}"
            );
            let owner = registry
                .gates
                .iter()
                .find(|gate| gate.id == case.owner_gate)
                .ok_or_else(|| anyhow::anyhow!("Unknown owning gate for {id}"))?;
            ensure!(
                case.status != "IMPLEMENTED" || owner.implemented,
                "{id}: implementation claim lacks its owning executable gate"
            );
            status.insert(id, case.status.clone());
        }
    }
    Ok(status)
}
