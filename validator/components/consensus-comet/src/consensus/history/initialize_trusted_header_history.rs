// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HistoryError, NativeHistoryVerifier};
use crate::{
    consensus::{
        authentication::{ConsensusAuthenticationRequirement, require_supported_authentication},
        certificates::{CertificateError, hash_consensus_header},
        signing::canonical_block_id,
    },
    wire::tendermint::types::{BlockId, Header},
};

/// Explicit local trust boundary: this does NOT authenticate a downloaded checkpoint.
/// Caller establishes genesis/profile/epoch provenance, trust freshness and block ID.
pub fn initialize_trusted_header_history(
    chain_id: &str,
    header: &Header,
    block_id: &BlockId,
    authentication: ConsensusAuthenticationRequirement,
) -> Result<NativeHistoryVerifier, HistoryError> {
    require_supported_authentication(authentication)
        .map_err(|_| HistoryError::Certificate(CertificateError::UnsupportedAuthentication))?;
    if header.chain_id != chain_id {
        return Err(HistoryError::Certificate(CertificateError::WrongChain));
    }
    let hash = hash_consensus_header(header).map_err(HistoryError::Certificate)?;
    if canonical_block_id(Some(block_id))
        .map_err(|error| HistoryError::Certificate(CertificateError::Signing(error)))?
        .is_none()
        || block_id.hash.as_slice() != hash
    {
        return Err(HistoryError::Certificate(CertificateError::WrongBlock));
    }
    let next_validator_hash = header
        .next_validators_hash
        .as_slice()
        .try_into()
        .map_err(|_| HistoryError::Certificate(CertificateError::InvalidHeaderHash))?;
    Ok(NativeHistoryVerifier {
        chain_id: chain_id.to_owned(),
        height: header.height,
        block_id: Some(block_id.clone()),
        header: Some(header.clone()),
        next_validator_hash,
        genesis_app_hash: None,
        authentication,
    })
}
