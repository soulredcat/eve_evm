// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::application::ConsensusApplication;
use anyhow::{Context, Result};
use eve_state::compute_commit_identity;
use eve_storage::state::{
    HistoryReadBudget, RetainedBlockProjection, capture_history_snapshot, read_history_block,
    read_state_service, state_reader,
};

pub(in crate::consensus::application) fn retained_application_block(
    application: &ConsensusApplication,
    height: u64,
) -> Result<RetainedBlockProjection> {
    let current = read_state_service(&application.service)?;
    if current.commit().target.height == height {
        let identity =
            compute_commit_identity(current.commit(), &application.config.logical_budget)
                .map_err(|_| anyhow::anyhow!("invalid cached application commit"))?;
        return Ok(RetainedBlockProjection {
            version: current.commit().target.clone(),
            commit_identity: identity,
            block: current.commit().block.clone(),
        });
    }
    let reader = state_reader(&application.repository);
    let snapshot = capture_history_snapshot(
        &reader,
        HistoryReadBudget {
            maximum_block_bytes: application.config.logical_budget.maximum_commit_bytes,
            maximum_rebuild_blocks: 256,
            maximum_index_batch_bytes: application.config.logical_budget.maximum_commit_bytes,
        },
    )?;
    read_history_block(&snapshot, height)?.context("required retained application block missing")
}
