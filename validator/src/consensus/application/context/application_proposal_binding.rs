// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    application::ConsensusApplication, signing::signer_status,
    transport::proposals::EngineProposalBinding,
};
use anyhow::{Context, Result, ensure};
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn application_proposal_binding(
    application: &ConsensusApplication,
    proposer_address: &[u8],
) -> Result<EngineProposalBinding> {
    ensure!(!application.fenced, "application fenced");
    let head = read_state_service(&application.service)?;
    let proposer: [u8; 20] = proposer_address
        .try_into()
        .context("native proposer address width")?;
    let owner = *application
        .config
        .proposer_owners
        .get(&proposer)
        .context("native proposer not in applicable development set")?;
    let previous = if head.commit().target.height == 0 {
        [0; 32]
    } else {
        let completed = application
            .completed
            .as_ref()
            .context("durable consensus replay metadata missing")?;
        ensure!(
            completed.target == head.commit().target,
            "consensus/application durable heads differ"
        );
        completed
            .request
            .hash
            .as_slice()
            .try_into()
            .context("retained consensus hash width")?
    };
    let signer = application
        .signer
        .lock()
        .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
    ensure!(!signer_status(&signer).fenced, "signer fenced");
    Ok(EngineProposalBinding {
        genesis_hash: head.commit().target.identity.genesis.0.0,
        chain_id: head.commit().target.identity.network_name.clone(),
        authentication: signer.config.authentication,
        key_epoch: head.commit().target.identity.key_epoch,
        proposer_owner: owner,
        previous_consensus_hash: previous,
    })
}
