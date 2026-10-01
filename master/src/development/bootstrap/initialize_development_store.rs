// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::config::{
    development_harness_budget::development_harness_budget,
    load_development_genesis::load_development_genesis,
    resolve_development_directory::resolve_development_directory,
    validate_development_mode::validate_development_mode,
};
use anyhow::{Result, ensure};
use eve_state::{StateCommit, initialize_development_state};
use eve_storage::state::open_state_repository;
use std::path::Path;

pub fn initialize_development_store(
    root: &Path,
    requested: &Path,
    spec: &Path,
    mode: &str,
    acknowledged: bool,
) -> Result<StateCommit> {
    validate_development_mode(mode, acknowledged)?;
    let path = resolve_development_directory(root, requested)?;
    ensure!(
        !path.exists(),
        "development namespace already exists; inspect/recover it instead of overwriting"
    );
    let genesis = load_development_genesis(spec)?;
    let budget = development_harness_budget();
    let commit = initialize_development_state(&genesis, &budget.logical)
        .map_err(|error| anyhow::anyhow!("development state initialization failed: {error:?}"))?;
    let store = open_state_repository(&path, &commit, budget)?;
    drop(store);
    Ok(commit)
}
