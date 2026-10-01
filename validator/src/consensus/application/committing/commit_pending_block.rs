// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    application::{ConsensusApplication, replay::append_synced_replay},
    approval::{approved_state_block, clear_after_synced_commit},
};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::ResponseCommit;
use eve_storage::state::{commit_state, read_state_service};

pub(super) fn commit_pending_block(
    application: &mut ConsensusApplication,
) -> Result<ResponseCommit> {
    if application.pending.is_none() && application.replayed_height.take().is_some() {
        return Ok(ResponseCommit { retain_height: 0 });
    }
    let pending = application
        .pending
        .take()
        .context("Commit without a decided application candidate")?;
    let prepared = approved_state_block(&pending.approval);
    #[cfg(test)]
    if application.simulated_failure
        == Some(super::super::types::SimulatedApplicationFailure::BeforeState)
    {
        application.simulated_failure = None;
        anyhow::bail!("SIMULATED application failure before actual state sync");
    }
    let ack = commit_state(&mut application.repository, &prepared.commit)?;
    ensure!(
        ack.committed == pending.decision.target
            && ack.commit_identity == pending.decision.commit_identity,
        "synced application acknowledgment differs from decision"
    );
    #[cfg(test)]
    if application.simulated_failure
        == Some(super::super::types::SimulatedApplicationFailure::StateSynced)
    {
        application.simulated_failure = None;
        anyhow::bail!("SIMULATED lost continuation after actual state sync");
    }
    #[cfg(test)]
    if application.simulated_failure
        == Some(super::super::types::SimulatedApplicationFailure::ExitAfterState)
    {
        std::process::exit(67);
    }
    append_synced_replay(application, &pending.decision, pending.decision_cursor)?;
    #[cfg(test)]
    if application.simulated_failure
        == Some(super::super::types::SimulatedApplicationFailure::MetadataSynced)
    {
        application.simulated_failure = None;
        anyhow::bail!("SIMULATED lost acknowledgment after actual metadata/state sync");
    }
    let refreshed = read_state_service(&application.service)?;
    ensure!(
        refreshed.commit().target == pending.decision.target,
        "synced application cache refresh failed"
    );
    let signer = application
        .signer
        .lock()
        .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
    clear_after_synced_commit(&application.approvals, &signer)
        .map_err(|_| anyhow::anyhow!("stale approval retirement failed"))?;
    Ok(ResponseCommit { retain_height: 0 })
}
