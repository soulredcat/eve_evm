// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::inspection::resolve_source_path::resolve_source_path;
use crate::structure::types::policy_types::StructurePolicy;
use anyhow::{Context, Result};
use std::path::Path;

pub fn load_policy(
    root: &Path,
    relative_path: &Path,
    sources: &[String],
) -> Result<StructurePolicy> {
    let relative = relative_path
        .to_str()
        .context("Structure policy path must be UTF-8")?;
    let path = resolve_source_path(root, relative)
        .with_context(|| format!("Resolve structure policy {relative}"))?;
    let encoded = std::fs::read_to_string(&path)
        .with_context(|| format!("Read policy {}", path.display()))?;
    let mut policy = toml::from_str(&encoded).context("Structure policy is invalid")?;
    super::load_adapter_policy_fragments::load_adapter_policy_fragments(
        root,
        &mut policy,
        sources,
    )?;
    Ok(policy)
}
