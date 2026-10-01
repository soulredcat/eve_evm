// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::{
    bootstrap::open_development_store::open_development_store,
    cli::types::DevelopmentOptions,
    config::{
        development_harness_budget::development_harness_budget,
        load_development_genesis::load_development_genesis, read_bounded_input::read_bounded_input,
        types::DevelopmentBlockInput, validate_development_mode::validate_development_mode,
    },
};
use alloy_primitives::B256;
use anyhow::{Context, Result, ensure};
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_state::StateVersion;
use eve_storage::state::{commit_state, create_state_service, read_state_service, state_reader};
use std::path::Path;

/// Local development delegates canonical execution; sync does not grant finality.
pub fn apply_development_block(
    options: &DevelopmentOptions,
    input_path: &Path,
) -> Result<StateVersion> {
    validate_development_mode(&options.mode, options.acknowledge_unsafe_development)?;
    let input: DevelopmentBlockInput =
        serde_json::from_slice(&read_bounded_input(input_path, 1_048_576)?)?;
    ensure!(
        input.transactions.len() <= 1_024,
        "development input count exceeds limit"
    );
    let genesis = load_development_genesis(&options.genesis)?;
    let mut store = open_development_store(
        &options.root,
        &options.data,
        &options.genesis,
        &options.mode,
        options.acknowledge_unsafe_development,
    )?;
    let service = create_state_service(state_reader(&store));
    let view = read_state_service(&service)?;
    let result = execute_state_block(
        view.commit(),
        &ExecutionBlockInput {
            timestamp: input.timestamp,
            proposer: genesis
                .validators
                .first()
                .context("development validators missing")?
                .owner,
            previous_consensus_hash: B256::ZERO,
        },
        &input.transactions,
        &development_harness_budget().logical,
        16 * 1_048_576,
    )
    .map_err(|error| anyhow::anyhow!("development execution rejected: {error:?}"))?;
    Ok(commit_state(&mut store, &result.commit)?.committed)
}
