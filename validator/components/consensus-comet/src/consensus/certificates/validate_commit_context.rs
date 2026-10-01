// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CertificateError, HistoricalValidatorSet, hash_consensus_header, hash_validator_set,
    validator_address,
};
use crate::{
    consensus::{
        authentication::require_supported_authentication,
        signing::{canonical_block_id, validate_chain_id},
    },
    wire::tendermint::types::{BlockId, Commit, Header},
};

pub(super) fn validate_commit_context(
    expected_chain: &str,
    expected_height: i64,
    expected_round: i32,
    expected_block: &BlockId,
    header: &Header,
    commit: &Commit,
    set: &HistoricalValidatorSet,
) -> Result<[u8; 32], CertificateError> {
    require_supported_authentication(set.authentication)
        .map_err(|_| CertificateError::UnsupportedAuthentication)?;
    validate_chain_id(expected_chain).map_err(CertificateError::Signing)?;
    if header.chain_id != expected_chain {
        return Err(CertificateError::WrongChain);
    }
    if expected_height <= 0
        || header.height != expected_height
        || commit.height != expected_height
        || set.height != expected_height
    {
        return Err(CertificateError::WrongHeight);
    }
    if expected_round < 0 || commit.round != expected_round {
        return Err(CertificateError::WrongRound);
    }
    if canonical_block_id(Some(expected_block))
        .map_err(CertificateError::Signing)?
        .is_none()
        || commit.block_id.as_ref() != Some(expected_block)
        || expected_block.hash.as_slice() != hash_consensus_header(header)?
    {
        return Err(CertificateError::WrongBlock);
    }
    let set_hash = hash_validator_set(&set.validators)?;
    if header.validators_hash.as_slice() != set_hash
        || !set.validators.iter().any(|validator| {
            validator_address(&validator.public_key).as_slice() == header.proposer_address
        })
    {
        return Err(CertificateError::WrongValidatorSet);
    }
    if commit.signatures.len() != set.validators.len() {
        return Err(CertificateError::InvalidSignatureLayout);
    }
    Ok(set_hash)
}
