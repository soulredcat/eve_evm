use crate::development::{
    bootstrap::open_development_store::open_development_store, cli::types::DevelopmentOptions,
    config::resolve_development_directory::resolve_development_directory,
};
use anyhow::Result;
use eve_storage::state::{capture_state_snapshot, export_state_snapshot, state_reader};
use std::path::Path;

pub fn export_development_snapshot(options: &DevelopmentOptions, requested: &Path) -> Result<()> {
    let store = open_development_store(
        &options.root,
        &options.data,
        &options.genesis,
        &options.mode,
        options.acknowledge_unsafe_development,
    )?;
    let destination = resolve_development_directory(&options.root, requested)?;
    let reader = state_reader(&store);
    let snapshot = capture_state_snapshot(&reader)?;
    export_state_snapshot(&snapshot, &destination)?;
    Ok(())
}
