// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    read_adapter_policy_fragment::read_adapter_policy_fragment,
    require_unignored_adapter_fragment::require_unignored_adapter_fragment,
};
use crate::structure::{
    inspection::resolve_source_path::resolve_source_path, types::policy_types::StructurePolicy,
};
use anyhow::{Result, ensure};
use std::{collections::BTreeSet, path::Path};

/// Append only reviewed adapter declarations into the existing exact-path policy.
/// Existing duplicate/staleness/trait/size validation remains mandatory afterward.
pub(super) fn load_adapter_policy_fragments(
    root: &Path,
    policy: &mut StructurePolicy,
    sources: &[String],
) -> Result<()> {
    ensure!(
        policy.adapter_files.len() <= 64,
        "Adapter policy fragment count limit"
    );
    let mut unique = BTreeSet::new();
    for relative in &policy.adapter_files {
        ensure!(
            unique.insert(relative),
            "Duplicate adapter policy fragment: {relative}"
        );
        let path = resolve_source_path(root, relative)?;
        ensure!(
            path.extension().and_then(|extension| extension.to_str()) == Some("toml"),
            "Adapter policy fragment must be a TOML file: {relative}"
        );
        ensure!(
            sources.contains(relative),
            "Adapter policy fragment is not publishable source: {relative}"
        );
        require_unignored_adapter_fragment(root, relative)?;
        let fragment = read_adapter_policy_fragment(&path)?;
        policy.adapters.extend(fragment.adapters);
    }
    Ok(())
}
