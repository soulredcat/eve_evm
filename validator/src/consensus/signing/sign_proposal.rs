// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DurableSigner,
    persist_signature::persist_signature,
    policy::{check_current_height, check_hrs, replay_proposal},
    types::Hrs,
};
use anyhow::{Context, Result, anyhow};
use eve_consensus_comet::{
    consensus::signing::encode_proposal_sign_bytes,
    wire::tendermint::types::{CanonicalProposal, Proposal},
};
use prost::Message;

/// Proposal signatures precede ProcessProposal; this operation grants no vote approval.
pub(in crate::consensus) fn sign_proposal(
    signer: &mut DurableSigner,
    mut proposal: Proposal,
) -> Result<Proposal> {
    let bytes = encode_proposal_sign_bytes(&signer.config.chain_id, &proposal)
        .map_err(|_| anyhow!("invalid native proposal request"))?;
    let hrs = Hrs {
        height: proposal.height,
        round: proposal.round,
        step: 1,
    };
    if let Some(record) = check_hrs(signer, hrs)? {
        return replay_proposal(record, &bytes, proposal);
    }
    check_current_height(signer, proposal.height)?;
    let canonical = CanonicalProposal::decode_length_delimited(bytes.as_slice())?;
    proposal.timestamp = Some(
        canonical
            .timestamp
            .context("canonical proposal timestamp missing")?,
    );
    proposal.signature = persist_signature(signer, hrs, bytes)?;
    Ok(proposal)
}
