// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::inspection::resolve_source_path::resolve_source_path;
use anyhow::{Result, bail};
use std::{
    collections::BTreeSet,
    path::{Component, Path, PathBuf},
};

pub fn validate_source_target(
    root: &Path,
    owner: &Path,
    target: &Path,
    reviewed: &BTreeSet<PathBuf>,
    test_only: bool,
) -> Result<()> {
    let mut normalized = PathBuf::new();
    for segment in target.strip_prefix(root)?.components() {
        match segment {
            Component::Normal(segment) => normalized.push(segment),
            Component::CurDir => {}
            Component::ParentDir if normalized.pop() => {}
            _ => bail!("Source target escapes repository: {}", target.display()),
        }
    }
    if normalized
        .extension()
        .is_none_or(|extension| extension != "rs")
    {
        bail!(
            "Rust source target must retain its .rs category: {}",
            target.display()
        );
    }
    let relative = normalized.to_string_lossy().replace('\\', "/");
    if !test_only && relative.split('/').any(|segment| segment == "tests") {
        bail!(
            "Production source cannot import a test-category target: {}",
            target.display()
        );
    }
    let canonical = resolve_source_path(root, &relative)?;
    if !canonical.starts_with(owner) {
        bail!(
            "Source target escapes its owning package/role: {}",
            target.display()
        );
    }
    if !reviewed.contains(&canonical) {
        bail!(
            "Source target is missing from reviewed inventory: {}",
            target.display()
        );
    }
    Ok(())
}
