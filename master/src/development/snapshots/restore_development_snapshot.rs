use crate::development::{
    cli::types::DevelopmentOptions,
    config::{
        development_harness_budget::development_harness_budget,
        load_development_genesis::load_development_genesis,
        resolve_development_directory::resolve_development_directory,
        validate_development_mode::validate_development_mode,
    },
};
use anyhow::Result;
use eve_state::{StateVersion, initialize_development_state};
use eve_storage::state::{
    activate_snapshot_namespace, create_state_service, read_state_service, state_reader,
};
use std::path::Path;

pub fn restore_development_snapshot(
    options: &DevelopmentOptions,
    source: &Path,
) -> Result<StateVersion> {
    validate_development_mode(&options.mode, options.acknowledge_unsafe_development)?;
    let source = resolve_development_directory(&options.root, source)?;
    let destination = resolve_development_directory(&options.root, &options.data)?;
    let spec = load_development_genesis(&options.genesis)?;
    let budget = development_harness_budget();
    let genesis = initialize_development_state(&spec, &budget.logical)
        .map_err(|error| anyhow::anyhow!("invalid development genesis: {error:?}"))?;
    let store = activate_snapshot_namespace(&source, &destination, &genesis, budget)?;
    let service = create_state_service(state_reader(&store));
    Ok(read_state_service(&service)?.commit().target.clone())
}
