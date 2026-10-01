// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CertificateError;
use crate::{
    consensus::signing::{canonical_block_id, normalize_timestamp, validate_chain_id},
    wire::tendermint::types::Header,
};

pub(super) fn validate_consensus_header(header: &Header) -> Result<(), CertificateError> {
    validate_chain_id(&header.chain_id).map_err(CertificateError::Signing)?;
    if header.height <= 0 {
        return Err(CertificateError::Signing(
            super::super::signing::SigningError::InvalidHeight,
        ));
    }
    if header
        .version
        .as_ref()
        .is_none_or(|version| version.block != 11)
    {
        return Err(CertificateError::InvalidHeaderVersion);
    }
    canonical_block_id(header.last_block_id.as_ref()).map_err(CertificateError::Signing)?;
    normalize_timestamp(header.time.as_ref()).map_err(CertificateError::Signing)?;
    if header.proposer_address.len() != 20
        || header.validators_hash.len() != 32
        || header.next_validators_hash.len() != 32
        || header.consensus_hash.len() != 32
        || header.app_hash.len() > 1024
        || [
            &header.last_commit_hash,
            &header.data_hash,
            &header.last_results_hash,
            &header.evidence_hash,
        ]
        .iter()
        .any(|value| !value.is_empty() && value.len() != 32)
    {
        return Err(CertificateError::InvalidHeaderHash);
    }
    Ok(())
}
