// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    HistoryError, NativeHistoryVerifier, VerifiedNativeHeader,
    validate_header_successor::validate_header_successor,
    verify_block_transaction_data::verify_block_transaction_data,
};
use crate::{
    consensus::certificates::{
        ClassicalValidator, HistoricalValidatorSet, hash_validator_set, verify_commit_certificate,
    },
    wire::tendermint::types::{BlockId, Commit, Header},
};

/// Authenticate one successor under the preceding certified next-set commitment.
/// No mutation occurs until all checks pass. This proves no EVM post-state/freshness.
pub fn verify_next_native_header(
    history: &mut NativeHistoryVerifier,
    block_id: &BlockId,
    header: &Header,
    commit: &Commit,
    validators: &[ClassicalValidator],
    transactions: &[Vec<u8>],
) -> Result<VerifiedNativeHeader, HistoryError> {
    validate_header_successor(history, header)?;
    if hash_validator_set(validators).map_err(HistoryError::Certificate)?
        != history.next_validator_hash
    {
        return Err(HistoryError::WrongValidatorTransition);
    }
    verify_block_transaction_data(header, transactions)?;
    let applicable = HistoricalValidatorSet {
        height: header.height,
        authentication: history.authentication,
        validators: validators.to_vec(),
    };
    let certified = verify_commit_certificate(
        &history.chain_id,
        header.height,
        commit.round,
        block_id,
        header,
        commit,
        &applicable,
    )
    .map_err(HistoryError::Certificate)?;
    let next_validator_hash = header
        .next_validators_hash
        .as_slice()
        .try_into()
        .map_err(|_| HistoryError::WrongValidatorTransition)?;
    let verified = VerifiedNativeHeader {
        header: header.clone(),
        block_id: block_id.clone(),
        authentication: history.authentication,
        signed_voting_power: certified.signed_voting_power,
        total_voting_power: certified.total_voting_power,
    };
    history.height = header.height;
    history.block_id = Some(block_id.clone());
    history.header = Some(header.clone());
    history.next_validator_hash = next_validator_hash;
    history.genesis_app_hash = None;
    Ok(verified)
}
