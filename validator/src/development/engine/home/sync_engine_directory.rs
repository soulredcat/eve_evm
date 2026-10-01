// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::path::Path;

pub(in crate::development::engine) fn sync_engine_directory(path: &Path) -> Result<()> {
    ensure!(
        cfg!(target_os = "linux"),
        "engine namespace durability requires Linux"
    );
    std::fs::File::open(path)?.sync_all()?;
    Ok(())
}
