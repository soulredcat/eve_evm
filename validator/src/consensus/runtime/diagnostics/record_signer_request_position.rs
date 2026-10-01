// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::SignerPosition;
use crate::consensus::runtime::types::RunningNode;
use anyhow::Result;
use eve_consensus_comet::wire::tendermint::privval::{Message, message::Sum};

pub(in crate::consensus::runtime) fn record_signer_request_position(
    node: &RunningNode,
    request: &Message,
) -> Result<()> {
    let position = match &request.sum {
        Some(Sum::SignVoteRequest(request)) => request.vote.as_ref().map(|vote| SignerPosition {
            height: vote.height,
            round: vote.round,
            step: vote.r#type.saturating_add(1),
        }),
        Some(Sum::SignProposalRequest(request)) => {
            request.proposal.as_ref().map(|proposal| SignerPosition {
                height: proposal.height,
                round: proposal.round,
                step: 1,
            })
        }
        _ => None,
    };
    *node
        .signer_requested
        .lock()
        .map_err(|_| anyhow::anyhow!("diagnostic position lock poisoned"))? = position;
    Ok(())
}
