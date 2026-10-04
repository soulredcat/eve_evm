// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::dispatch_auxiliary_application_request::dispatch_auxiliary_application_request;
use crate::consensus::{
    application::{
        ConsensusApplication, application_finalization_binding, application_info,
        application_proposal_binding, check_transaction, commit_application, finalize_block,
        initialize_application, prepare_proposal, process_proposal,
    },
    transport::{
        peer::{AuthenticatedEnginePeer, ensure_application_engine_peer},
        proposals::{issue_verified_finalize_proposal, issue_verified_process_proposal},
    },
};
use anyhow::{Context, Result};
use eve_consensus_comet::wire::tendermint::abci::{
    Request, Response, request::Value as RequestValue, response::Value as ResponseValue,
};

/// Only decoded bytes from the same authenticated private application FD enter here.
pub(in crate::consensus) fn dispatch_application_request(
    application: &mut ConsensusApplication,
    peer: &AuthenticatedEnginePeer,
    request: Request,
) -> Result<Response> {
    ensure_application_engine_peer(peer)?;
    let value = match request.value.context("empty native application request")? {
        RequestValue::Info(_) => ResponseValue::Info(application_info(application)?),
        RequestValue::InitChain(input) => {
            ResponseValue::InitChain(initialize_application(application, peer, &input)?)
        }
        RequestValue::CheckTx(input) => {
            ResponseValue::CheckTx(check_transaction(application, peer, &input)?)
        }
        RequestValue::PrepareProposal(input) => {
            ResponseValue::PrepareProposal(prepare_proposal(application, peer, &input)?)
        }
        RequestValue::ProcessProposal(input) => {
            let binding = application_proposal_binding(application, &input.proposer_address)?;
            let source = issue_verified_process_proposal(peer, input, binding)?;
            ResponseValue::ProcessProposal(process_proposal(application, source)?)
        }
        RequestValue::FinalizeBlock(input) => {
            let binding = application_finalization_binding(
                application,
                input.height,
                &input.proposer_address,
            )?;
            let source = issue_verified_finalize_proposal(peer, &input, binding)?;
            ResponseValue::FinalizeBlock(finalize_block(application, source, &input)?)
        }
        RequestValue::Commit(_) => ResponseValue::Commit(commit_application(application, peer)?),
        auxiliary => dispatch_auxiliary_application_request(application, auxiliary)?,
    };
    Ok(Response { value: Some(value) })
}
