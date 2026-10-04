// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ChargedMasterGenesis;
use crate::{
    development::config::load_development_genesis::load_development_genesis,
    sync::resources::reserve_master_bytes,
};
use anyhow::Result;
use std::{path::Path, sync::Arc};
use tokio::sync::Semaphore;

/// Separate declared 32 MiB CLI input/decode allowance for the canonical 1 MiB JSON
/// input ceiling. Retain it until the borrowed configured genesis leaves scope.
pub(super) fn load_charged_master_genesis(path: &Path) -> Result<ChargedMasterGenesis> {
    let pool = Arc::new(Semaphore::new(32 * 1_024));
    let lease = reserve_master_bytes(&pool, 32 * 1_048_576)?;
    let genesis = load_development_genesis(path)?;
    Ok(ChargedMasterGenesis {
        genesis,
        _lease: lease,
    })
}
