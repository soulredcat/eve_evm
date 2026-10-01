// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::list_git_paths::list_git_paths;
use anyhow::Result;
use std::path::Path;

pub fn discover_sources(root: &Path) -> Result<Vec<String>> {
    let mut paths = list_git_paths(
        root,
        &[
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ],
    )?;
    for deleted in list_git_paths(root, &["ls-files", "-z", "--deleted"])? {
        paths.remove(&deleted);
    }
    Ok(paths.into_iter().collect())
}
