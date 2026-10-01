// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::types::RunningNode;
use anyhow::Result;
use eve_consensus_comet::wire::tendermint::privval::{Message, message::Sum};

pub(in crate::consensus::runtime) fn record_signer_handshake(
    node: &RunningNode,
    response: &Message,
) -> Result<()> {
    let mut progress = node
        .handshake
        .lock()
        .map_err(|_| anyhow::anyhow!("readiness lock poisoned"))?;
    match &response.sum {
        Some(Sum::PubKeyResponse(response))
            if response.error.is_none() && response.pub_key.is_some() =>
        {
            progress.public_key = true
        }
        Some(Sum::PingResponse(_)) => progress.ping = true,
        Some(Sum::SignedVoteResponse(response))
            if response.error.is_none() && response.vote.is_some() =>
        {
            progress.signing_activity = true
        }
        Some(Sum::SignedProposalResponse(response))
            if response.error.is_none() && response.proposal.is_some() =>
        {
            progress.signing_activity = true
        }
        _ => {}
    }
    Ok(())
}
