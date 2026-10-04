// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{fs::File, path::Path};

/// Sync the namespace and every containing directory through its already validated
/// local root. New directory entries must not precede acknowledgement unsynced.
pub(in crate::sync) fn sync_master_namespace(path: &Path, root: &Path) -> Result<()> {
    let root = root.canonicalize()?;
    let mut current = path.canonicalize()?;
    ensure!(current.starts_with(&root), "MASTER_NAMESPACE_SYNC_SCOPE");
    loop {
        File::open(&current)?.sync_all()?;
        if current == root {
            break;
        }
        ensure!(current.pop(), "MASTER_NAMESPACE_SYNC_SCOPE");
    }
    Ok(())
}
