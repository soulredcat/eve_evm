// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ChargedCheckpointGenesis;
use crate::sync::applied::{AppliedReader, reserve_applied_working};
use anyhow::Result;
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, DevelopmentGenesis, StateBudget,
    estimate_genesis_initialization_reservation, initialize_development_state,
};

pub(super) fn prepare_local_checkpoint_genesis(
    reader: &AppliedReader,
    genesis: &DevelopmentGenesis,
    budget: &StateBudget,
) -> Result<ChargedCheckpointGenesis> {
    let _scratch = reserve_applied_working(reader, BOUNDED_STATE_CODEC_SCRATCH_BYTES)
        .map_err(|error| anyhow::anyhow!("checkpoint genesis scratch: {error:?}"))?;
    let required = estimate_genesis_initialization_reservation(genesis, budget)
        .map_err(|error| anyhow::anyhow!("checkpoint genesis estimate: {error:?}"))?;
    let lease = reserve_applied_working(reader, required)
        .map_err(|error| anyhow::anyhow!("checkpoint genesis reservation: {error:?}"))?;
    let initialized = initialize_development_state(genesis, budget)
        .map_err(|error| anyhow::anyhow!("checkpoint local genesis: {error:?}"))?;
    Ok(ChargedCheckpointGenesis {
        version: initialized.target.clone(),
        _lease: lease,
    })
}
