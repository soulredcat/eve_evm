// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SigningError, canonical_block_id, normalize_timestamp, validate_chain_id};
use crate::wire::tendermint::types::{CanonicalVote, Vote};
use prost::Message;

/// Preserve native canonical bytes; caller owns enrollment and execution approval.
pub fn encode_vote_sign_bytes(chain_id: &str, vote: &Vote) -> Result<Vec<u8>, SigningError> {
    validate_chain_id(chain_id)?;
    if !matches!(vote.r#type, 1 | 2) {
        return Err(SigningError::InvalidMessageType);
    }
    if vote.height <= 0 {
        return Err(SigningError::InvalidHeight);
    }
    if vote.round < 0 {
        return Err(SigningError::InvalidRound);
    }
    if vote.validator_address.len() != 20 {
        return Err(SigningError::InvalidValidatorAddress);
    }
    if vote.validator_index < 0 {
        return Err(SigningError::InvalidValidatorIndex);
    }
    if !vote.signature.is_empty() && vote.signature.len() != 64 {
        return Err(SigningError::InvalidSignatureLength);
    }
    if !vote.extension.is_empty() || !vote.extension_signature.is_empty() {
        return Err(SigningError::UnsupportedVoteExtension);
    }
    Ok(CanonicalVote {
        r#type: vote.r#type,
        height: vote.height,
        round: i64::from(vote.round),
        block_id: canonical_block_id(vote.block_id.as_ref())?,
        timestamp: Some(normalize_timestamp(vote.timestamp.as_ref())?),
        chain_id: chain_id.to_owned(),
    }
    .encode_length_delimited_to_vec())
}
