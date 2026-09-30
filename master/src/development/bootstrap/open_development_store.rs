use crate::development::config::{
    development_harness_budget::development_harness_budget,
    load_development_genesis::load_development_genesis,
    resolve_development_directory::resolve_development_directory,
    validate_development_mode::validate_development_mode,
};
use anyhow::{Result, ensure};
use eve_state::initialize_development_state;
use eve_storage::state::StateRepository;
use std::path::Path;

pub fn open_development_store(
    root: &Path,
    requested: &Path,
    spec: &Path,
    mode: &str,
    acknowledged: bool,
) -> Result<StateRepository> {
    validate_development_mode(mode, acknowledged)?;
    let path = resolve_development_directory(root, requested)?;
    ensure!(
        path.is_dir(),
        "development namespace is missing; use explicit initialization first"
    );
    let genesis = load_development_genesis(spec)?;
    let budget = development_harness_budget();
    let commit = initialize_development_state(&genesis, &budget.logical)
        .map_err(|error| anyhow::anyhow!("invalid development genesis: {error:?}"))?;
    eve_storage::state::open_state_repository(&path, &commit, budget)
}
