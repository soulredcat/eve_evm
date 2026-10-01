// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use std::path::Path;

pub(in crate::consensus::runtime) fn sync_node_directory(path: &Path) -> Result<()> {
    std::fs::File::open(path)?.sync_all()?;
    Ok(())
}
