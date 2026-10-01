// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use std::path::Path;

/// The reference contract fails closed where directory sync support has not been verified.
pub(in crate::records) fn sync_opaque_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        std::fs::File::open(path)?.sync_all()?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        anyhow::bail!("opaque-record directory durability requires the Unix reference platform");
    }
    Ok(())
}
