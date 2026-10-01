// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub fn dependency_targets(root: &Path, manifest: &Path) -> Result<Vec<PathBuf>> {
    let document: toml::Value = toml::from_str(&std::fs::read_to_string(manifest)?)?;
    let workspace: toml::Value = if root.join("Cargo.toml").exists() {
        toml::from_str(&std::fs::read_to_string(root.join("Cargo.toml"))?)?
    } else {
        toml::Value::Table(Default::default())
    };
    let parent = manifest.parent().context("Manifest has no parent")?;
    let mut output = Vec::new();
    let tables = super::manifest_dependency_tables::manifest_dependency_tables(&document)
        .into_iter()
        .map(|table| (table, parent))
        .collect::<Vec<_>>();
    for kind in ["patch", "replace"] {
        if let Some(value) = workspace.get(kind) {
            let configuration =
                toml::Value::Table([(kind.into(), value.clone())].into_iter().collect());
            // Workspace overrides are resolved below directly so references do not escape.
            for table in
                super::manifest_dependency_tables::manifest_dependency_tables(&configuration)
            {
                for (name, dependency) in table {
                    if let Some(relative) = dependency.get("path").and_then(toml::Value::as_str) {
                        let target = root
                            .join(relative)
                            .join("Cargo.toml")
                            .canonicalize()
                            .with_context(|| format!("Resolve workspace override {name}"))?;
                        if !target.starts_with(root) {
                            bail!("Workspace override escapes repository: {name}");
                        }
                        output.push(target);
                    }
                }
            }
        }
    }
    for (table, table_origin) in tables {
        for (name, original) in table {
            let inherited = original.get("workspace").and_then(toml::Value::as_bool) == Some(true);
            let value = if inherited {
                workspace
                    .get("workspace")
                    .and_then(|value| value.get("dependencies"))
                    .and_then(|value| value.get(name))
                    .with_context(|| format!("Missing workspace dependency {name}"))?
            } else {
                original
            };
            if let Some(relative) = value.get("path").and_then(toml::Value::as_str) {
                let origin = if inherited { root } else { table_origin };
                let target = origin
                    .join(relative)
                    .join("Cargo.toml")
                    .canonicalize()
                    .with_context(|| {
                        format!("Resolve dependency {name} in {}", manifest.display())
                    })?;
                if !target.starts_with(root) {
                    bail!("Local dependency escapes repository: {name}");
                }
                output.push(target);
            }
        }
    }
    output.sort();
    output.dedup();
    Ok(output)
}
