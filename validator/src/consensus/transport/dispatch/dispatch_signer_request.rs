// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    handle_signer_vote_request::handle_signer_vote_request,
    refuse_signing_request::refuse_signing_request,
};
use crate::consensus::{
    approval::ApprovalRegistry,
    signing::{DurableSigner, sign_proposal, signer_status},
    transport::peer::{
        AuthenticatedEnginePeer, EngineChannel, ensure_engine_channel,
        validate_engine_peer_liveness,
    },
};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::{
    crypto::{PublicKey, public_key::Sum as KeySum},
    privval::{
        Message, PingResponse, PubKeyResponse, SignedProposalResponse, SignedVoteResponse,
        message::Sum,
    },
};
use eve_state::StateBudget;

pub(in crate::consensus) fn dispatch_signer_request(
    signer: &mut DurableSigner,
    registry: &ApprovalRegistry,
    peer: &AuthenticatedEnginePeer,
    request: Message,
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<Message> {
    ensure_engine_channel(peer, EngineChannel::Signer)?;
    validate_engine_peer_liveness(peer)?;
    let sum = match request.sum.context("empty native signer request")? {
        Sum::PingRequest(_) => Sum::PingResponse(PingResponse {}),
        Sum::PubKeyRequest(input) => {
            ensure!(
                input.chain_id == signer.config.chain_id,
                "signer chain mismatch"
            );
            Sum::PubKeyResponse(PubKeyResponse {
                pub_key: Some(PublicKey {
                    sum: Some(KeySum::Ed25519(signer.config.expected_public_key.to_vec())),
                }),
                error: None,
            })
        }
        Sum::SignVoteRequest(input) => {
            ensure!(
                input.chain_id == signer.config.chain_id,
                "signer chain mismatch"
            );
            let before = signer_status(signer).cursor;
            match handle_signer_vote_request(
                signer,
                registry,
                input.vote.context("native vote missing")?,
                budget,
                reserved_clone_bytes,
            ) {
                Ok(vote) => Sum::SignedVoteResponse(SignedVoteResponse {
                    vote: Some(vote),
                    error: None,
                }),
                Err(error) => Sum::SignedVoteResponse(SignedVoteResponse {
                    vote: None,
                    error: Some(refuse_signing_request(signer, &before, error)?),
                }),
            }
        }
        Sum::SignProposalRequest(input) => {
            ensure!(
                input.chain_id == signer.config.chain_id,
                "signer chain mismatch"
            );
            let before = signer_status(signer).cursor;
            match sign_proposal(signer, input.proposal.context("native proposal missing")?) {
                Ok(proposal) => Sum::SignedProposalResponse(SignedProposalResponse {
                    proposal: Some(proposal),
                    error: None,
                }),
                Err(error) => Sum::SignedProposalResponse(SignedProposalResponse {
                    proposal: None,
                    error: Some(refuse_signing_request(signer, &before, error)?),
                }),
            }
        }
        _ => anyhow::bail!("unexpected native signer response as request"),
    };
    Ok(Message { sum: Some(sum) })
}
