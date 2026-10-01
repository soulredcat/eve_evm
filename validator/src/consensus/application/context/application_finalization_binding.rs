// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::application_proposal_binding;
use crate::consensus::{
    application::{ConsensusApplication, replay::find_replay_decision},
    transport::proposals::EngineProposalBinding,
};
use anyhow::{Context, Result, ensure};
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn application_finalization_binding(
    application: &ConsensusApplication,
    height: i64,
    proposer_address: &[u8],
) -> Result<EngineProposalBinding> {
    let head = read_state_service(&application.service)?;
    if height > 0 && u64::try_from(height)? <= head.commit().target.height {
        let (decision, _) = find_replay_decision(application, u64::try_from(height)?)?
            .context("historical finalized replay decision missing")?;
        ensure!(
            decision.request.proposer_address == proposer_address,
            "historical replay proposer mismatch"
        );
        let signer = application
            .signer
            .lock()
            .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
        return Ok(EngineProposalBinding {
            genesis_hash: decision.target.identity.genesis.0.0,
            chain_id: decision.target.identity.network_name.clone(),
            authentication: signer.config.authentication,
            key_epoch: decision.target.identity.key_epoch,
            proposer_owner: decision.proposer_owner,
            previous_consensus_hash: decision.previous_consensus_hash,
        });
    }
    application_proposal_binding(application, proposer_address)
}
