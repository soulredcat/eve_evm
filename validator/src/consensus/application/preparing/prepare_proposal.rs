// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    execute_prepared_selection::execute_prepared_selection,
    select_proposal_transactions::select_proposal_transactions,
};
use crate::consensus::{
    application::{
        ConsensusApplication,
        context::{application_proposal_binding, validate_application_request},
    },
    transport::peer::{AuthenticatedEnginePeer, ensure_application_engine_peer},
};
use alloy_primitives::B256;
use anyhow::{Context, Result};
use eve_consensus_comet::wire::tendermint::abci::{
    RequestPrepareProposal, ResponsePrepareProposal,
};
use eve_evm::ExecutionBlockInput;
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn prepare_proposal(
    application: &ConsensusApplication,
    peer: &AuthenticatedEnginePeer,
    request: &RequestPrepareProposal,
) -> Result<ResponsePrepareProposal> {
    ensure_application_engine_peer(peer)?;
    validate_application_request(application, request.height, request.time.as_ref(), &[])?;
    let binding = application_proposal_binding(application, &request.proposer_address)?;
    if let Some(txs) = crate::development::acceptance::poison_development_proposal(
        application.config.acceptance_fixture.as_deref(),
        request,
        &application.acceptance_poison_used,
    ) {
        return Ok(ResponsePrepareProposal { txs });
    }
    let parent = read_state_service(&application.service)?;
    let selected =
        select_proposal_transactions(parent.commit(), &request.txs, request.max_tx_bytes)?;
    let input = ExecutionBlockInput {
        timestamp: u64::try_from(
            request
                .time
                .as_ref()
                .context("native prepare time missing")?
                .seconds,
        )?,
        proposer: binding.proposer_owner,
        previous_consensus_hash: B256::from(binding.previous_consensus_hash),
    };
    let transactions = execute_prepared_selection(
        parent.commit(),
        &input,
        selected,
        &application.config.logical_budget,
        application.config.reserved_clone_bytes,
    )?;
    Ok(ResponsePrepareProposal { txs: transactions })
}
