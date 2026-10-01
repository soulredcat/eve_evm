// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use std::path::Path;

pub(in crate::development::engine) fn sync_native_engine_files(home: &Path) -> Result<()> {
    for file in [
        "config/config.toml",
        "config/genesis.json",
        "config/node_key.json",
        "config/priv_validator_key.json",
        "data/priv_validator_state.json",
    ] {
        std::fs::File::open(home.join(file))?.sync_all()?;
    }
    for directory in ["config", "data"] {
        super::sync_engine_directory(&home.join(directory))?;
    }
    super::sync_engine_directory(home)
}
