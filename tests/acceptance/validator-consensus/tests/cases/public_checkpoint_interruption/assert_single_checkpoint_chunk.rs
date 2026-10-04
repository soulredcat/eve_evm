// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use eve_state::{
    StateCommit, development_state_budget, encode_state_commit, measure_complete_state_bytes,
    measure_state_delta_execution_bytes,
};

/// The gate's third-connection interpretation requires this actual small body to fit one chunk.
pub(super) fn assert_single_checkpoint_chunk(expected: &StateCommit) -> Result<()> {
    let budget = development_state_budget();
    let upper = measure_complete_state_bytes(&expected.state, &budget)
        .map_err(|error| anyhow::anyhow!("CHECKPOINT_INTERRUPTION_STATE_MEASURE: {error:?}"))?
        .checked_add(
            measure_state_delta_execution_bytes(&expected.block, &budget).map_err(|error| {
                anyhow::anyhow!("CHECKPOINT_INTERRUPTION_BLOCK_MEASURE: {error:?}")
            })?,
        )
        .and_then(|bytes| bytes.checked_add(12_288))
        .ok_or_else(|| anyhow::anyhow!("CHECKPOINT_INTERRUPTION_SIZE_OVERFLOW"))?;
    ensure!(
        upper <= 32_768,
        "CHECKPOINT_INTERRUPTION_SMALL_ORACLE_BOUND"
    );
    let bytes = encode_state_commit(expected, &budget)
        .map_err(|error| anyhow::anyhow!("CHECKPOINT_INTERRUPTION_ORACLE_ENCODING: {error:?}"))?;
    ensure!(
        !bytes.is_empty() && bytes.len() < 32_768,
        "CHECKPOINT_INTERRUPTION_SINGLE_CHUNK_REQUIRED"
    );
    Ok(())
}
