// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::encode_finalize_result;
use crate::consensus::{
    application::{
        ConsensusApplication,
        context::validate_application_request,
        processing::approval_logical_charge,
        replay::{append_decided_replay, types::ReplayDecision, validate_replay_decision},
        types::PendingBlock,
    },
    approval::{approval_request, approved_state_block, create_execution_approval, find_approval},
    transport::proposals::VerifiedLocalEngineProposal,
};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{RequestFinalizeBlock, ResponseFinalizeBlock};
use eve_state::compute_commit_identity;
use std::sync::Arc;

pub(super) fn finalize_fresh_block(
    application: &mut ConsensusApplication,
    source: VerifiedLocalEngineProposal,
    request: &RequestFinalizeBlock,
) -> Result<ResponseFinalizeBlock> {
    validate_application_request(
        application,
        request.height,
        request.time.as_ref(),
        &request.txs,
    )?;
    let hash: [u8; 32] = request
        .hash
        .as_slice()
        .try_into()
        .context("native decided hash width")?;
    let approval = {
        let signer = application
            .signer
            .lock()
            .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
        match find_approval(&application.approvals, &hash)
            .map_err(|_| anyhow::anyhow!("approval registry unavailable"))?
            .filter(|approval| approval_request(approval) == source.request())
        {
            Some(approval) => approval,
            None => Arc::new(
                create_execution_approval(
                    &signer,
                    &source,
                    &application.config.logical_budget,
                    application.config.reserved_clone_bytes,
                )
                .map_err(|_| anyhow::anyhow!("decided block canonical execution failed"))?,
            ),
        }
    };
    let prepared = approved_state_block(&approval);
    let charge = approval_logical_charge(&approval, &application.config.logical_budget)?;
    ensure!(
        charge <= application.config.maximum_cached_bytes / 2,
        "decided block preparation capacity unavailable"
    );
    let commit_identity =
        compute_commit_identity(&prepared.commit, &application.config.logical_budget)
            .map_err(|_| anyhow::anyhow!("decided block canonical identity unavailable"))?;
    let decision = ReplayDecision {
        parent: prepared
            .commit
            .parent
            .clone()
            .context("decided application parent missing")?,
        target: prepared.commit.target.clone(),
        commit_identity,
        previous_consensus_hash: source.binding().previous_consensus_hash,
        proposer_owner: source.binding().proposer_owner,
        request: request.clone(),
    };
    validate_replay_decision(
        &decision,
        &application.config,
        source.binding().previous_consensus_hash,
    )?;
    let response = encode_finalize_result(
        &prepared.commit.target,
        &prepared.commit.block,
        application.config.acceptance_fixture.as_deref(),
    )?;
    let (decision, decision_cursor) = if let Some((stored, cursor)) = &application.retained_decision
    {
        ensure!(
            *stored == decision,
            "re-executed decision differs from retained pending material"
        );
        (stored.clone(), *cursor)
    } else {
        append_decided_replay(application, decision)?
    };
    application.pending = Some(PendingBlock {
        decision,
        decision_cursor,
        approval,
        request: request.clone(),
    });
    Ok(response)
}
