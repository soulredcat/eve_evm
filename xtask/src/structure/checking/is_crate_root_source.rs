// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    manifest_source_targets::manifest_source_targets, source_package_root::source_package_root,
};
use std::path::Path;

pub fn is_crate_root_source(root: &Path, source: &Path) -> bool {
    if source
        .file_name()
        .is_some_and(|name| matches!(name.to_str(), Some("lib.rs" | "main.rs" | "build.rs")))
    {
        return true;
    }
    let owner = source_package_root(root, source);
    for directory in ["tests", "benches", "examples"] {
        if source.parent() == Some(owner.join(directory).as_path()) {
            return true;
        }
    }
    manifest_source_targets(&owner.join("Cargo.toml")).is_ok_and(|targets| {
        targets
            .iter()
            .any(|target| target.path.canonicalize().ok() == source.canonicalize().ok())
    })
}
