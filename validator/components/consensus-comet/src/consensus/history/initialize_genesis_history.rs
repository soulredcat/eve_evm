// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{HistoryError, NativeHistoryVerifier};
use crate::consensus::{
    authentication::{ConsensusAuthenticationRequirement, require_supported_authentication},
    certificates::{CertificateError, ClassicalValidator, hash_validator_set},
    signing::validate_chain_id,
};

/// Inputs must come from validated local genesis, never from the serving peer.
/// The caller binds EVE network/genesis/profile and the height-zero application hash.
pub fn initialize_genesis_history(
    chain_id: &str,
    validators: &[ClassicalValidator],
    genesis_app_hash: [u8; 32],
    authentication: ConsensusAuthenticationRequirement,
) -> Result<NativeHistoryVerifier, HistoryError> {
    require_supported_authentication(authentication)
        .map_err(|_| HistoryError::Certificate(CertificateError::UnsupportedAuthentication))?;
    validate_chain_id(chain_id)
        .map_err(|error| HistoryError::Certificate(CertificateError::Signing(error)))?;
    let next_validator_hash = hash_validator_set(validators).map_err(HistoryError::Certificate)?;
    Ok(NativeHistoryVerifier {
        chain_id: chain_id.to_owned(),
        height: 0,
        block_id: None,
        header: None,
        next_validator_hash,
        genesis_app_hash: Some(genesis_app_hash),
        authentication,
    })
}
