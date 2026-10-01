// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SigningError, canonical_block_id, normalize_timestamp, validate_chain_id};
use crate::wire::tendermint::types::{CanonicalProposal, Proposal};
use prost::Message;

/// A proposer signature precedes ProcessProposal and grants no vote approval.
pub fn encode_proposal_sign_bytes(
    chain_id: &str,
    proposal: &Proposal,
) -> Result<Vec<u8>, SigningError> {
    validate_chain_id(chain_id)?;
    if proposal.r#type != 32 {
        return Err(SigningError::InvalidMessageType);
    }
    if proposal.height <= 0 {
        return Err(SigningError::InvalidHeight);
    }
    if proposal.round < 0
        || proposal.pol_round < -1
        || (proposal.pol_round >= 0 && proposal.pol_round >= proposal.round)
    {
        return Err(SigningError::InvalidRound);
    }
    if !proposal.signature.is_empty() && proposal.signature.len() != 64 {
        return Err(SigningError::InvalidSignatureLength);
    }
    let block_id =
        canonical_block_id(proposal.block_id.as_ref())?.ok_or(SigningError::InvalidBlockId)?;
    Ok(CanonicalProposal {
        r#type: 32,
        height: proposal.height,
        round: i64::from(proposal.round),
        pol_round: i64::from(proposal.pol_round),
        block_id: Some(block_id),
        timestamp: Some(normalize_timestamp(proposal.timestamp.as_ref())?),
        chain_id: chain_id.to_owned(),
    }
    .encode_length_delimited_to_vec())
}
