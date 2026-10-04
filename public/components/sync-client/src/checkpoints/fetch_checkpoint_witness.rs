// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointWitnessHeights, DownloadedCheckpointWitness,
    fetch_checkpoint_witness_before::fetch_checkpoint_witness_before,
};
use crate::NativeRpcConfig;
use anyhow::Result;
use eve_finality_verifier::CheckpointLimits;
use eve_state::{StateBudget, StateVersion};
use std::time::Instant;

/// Preserve one configured operation deadline across metadata, native queries and encoding.
pub fn fetch_checkpoint_witness<L>(
    rpc: NativeRpcConfig,
    genesis: &StateVersion,
    heights: CheckpointWitnessHeights,
    budget: &StateBudget,
    limits: CheckpointLimits,
    reserve: &mut impl FnMut(usize) -> Result<L>,
) -> Result<DownloadedCheckpointWitness<L>> {
    let deadline = Instant::now()
        .checked_add(rpc.timeout)
        .ok_or_else(|| anyhow::anyhow!("SYNC_CHECKPOINT_DEADLINE"))?;
    fetch_checkpoint_witness_before(rpc, genesis, heights, budget, limits, reserve, deadline)
}
