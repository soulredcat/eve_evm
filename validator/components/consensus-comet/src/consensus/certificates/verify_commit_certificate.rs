// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CertificateError, HistoricalValidatorSet, VerifiedClassicalCommit,
    validate_commit_context::validate_commit_context,
    validate_validator_set::validate_validator_set,
    verify_commit_signature::verify_commit_signature,
};
use crate::wire::tendermint::types::{BlockId, Commit, Header};
use std::collections::BTreeSet;

/// Verify all nonabsent native signatures under a caller-authenticated historical set.
/// Execution validity, set-history provenance/freshness and H/H+1 anchoring remain separate.
pub fn verify_commit_certificate(
    expected_chain: &str,
    expected_height: i64,
    expected_round: i32,
    expected_block_id: &BlockId,
    header: &Header,
    commit: &Commit,
    applicable_set: &HistoricalValidatorSet,
) -> Result<VerifiedClassicalCommit, CertificateError> {
    let validator_set_hash = validate_commit_context(
        expected_chain,
        expected_height,
        expected_round,
        expected_block_id,
        header,
        commit,
        applicable_set,
    )?;
    let total = validate_validator_set(&applicable_set.validators, true)?;
    let mut signed = 0_i64;
    let mut seen = BTreeSet::new();
    for (index, (signature, validator)) in commit
        .signatures
        .iter()
        .zip(&applicable_set.validators)
        .enumerate()
    {
        if signature.block_id_flag != 1 && !seen.insert(signature.validator_address.clone()) {
            return Err(CertificateError::InvalidSignatureLayout);
        }
        if verify_commit_signature(expected_chain, commit, signature, index, validator)? {
            signed = signed
                .checked_add(validator.voting_power)
                .ok_or(CertificateError::VotingPowerOverflow)?;
        }
    }
    if u128::try_from(signed).map_err(|_| CertificateError::VotingPowerOverflow)? * 3
        <= u128::try_from(total).map_err(|_| CertificateError::VotingPowerOverflow)? * 2
    {
        return Err(CertificateError::InsufficientVotingPower);
    }
    Ok(VerifiedClassicalCommit {
        chain_id: expected_chain.to_owned(),
        height: expected_height,
        round: expected_round,
        block_id: expected_block_id.clone(),
        validator_set_hash,
        signed_voting_power: signed,
        total_voting_power: total,
    })
}
