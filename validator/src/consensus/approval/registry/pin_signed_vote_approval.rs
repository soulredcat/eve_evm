// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ApprovalRegistry;
use crate::consensus::{approval::ApprovalError, signing::DurableSigner};
use eve_consensus_comet::{
    consensus::{
        certificates::{validator_address, verify_native_ed25519_signature},
        signing::encode_vote_sign_bytes,
    },
    wire::tendermint::types::Vote,
};

pub(in crate::consensus) fn pin_signed_vote_approval(
    registry: &ApprovalRegistry,
    signer: &DurableSigner,
    vote: &Vote,
) -> Result<(), ApprovalError> {
    let unavailable = |reason| ApprovalError::Unavailable {
        reason,
        cause: None,
    };
    let bytes = encode_vote_sign_bytes(&signer.config.chain_id, vote)
        .map_err(|_| unavailable("invalid pin vote"))?;
    if vote.validator_address != validator_address(&signer.config.expected_public_key)
        || verify_native_ed25519_signature(
            &signer.config.expected_public_key,
            &bytes,
            &vote.signature,
        )
        .is_err()
    {
        return Err(unavailable("pin requires actual enrolled native signature"));
    }
    let Some(block) = vote
        .block_id
        .as_ref()
        .filter(|block| !block.hash.is_empty())
    else {
        return Ok(());
    };
    let hash: [u8; 32] = block
        .hash
        .as_slice()
        .try_into()
        .map_err(|_| unavailable("invalid pin block hash"))?;
    let mut state = registry
        .state
        .lock()
        .map_err(|_| unavailable("approval registry poisoned"))?;
    if state.config.as_ref() != Some(&signer.config) {
        return Err(unavailable("pin signer context mismatch"));
    }
    let raw = state
        .raw
        .get(&hash)
        .ok_or(unavailable("pin input must be retained"))?;
    if raw.source.request().height != vote.height {
        return Err(unavailable("pin input height mismatch"));
    }
    match vote.r#type {
        1 => state.prevote_pin = Some(hash),
        2 => state.precommit_pin = Some(hash),
        _ => return Err(unavailable("invalid pin vote type")),
    }
    Ok(())
}
