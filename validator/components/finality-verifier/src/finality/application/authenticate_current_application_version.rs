// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::validate_application_identity::validate_application_identity;
use crate::finality::{AuthenticatedApplicationAnchor, DevelopmentFinalityVerifier, FinalityError};
use eve_consensus_comet::consensus::commitments::{ExecutionHeight, match_next_header_commitment};
use eve_protocol_config::records::{ApplicationCommitmentInput, hash_application_commitment};
use eve_state::StateVersion;

/// Authenticate only EVE_APP_V1 fields; auxiliary content digest/time remain unproven.
pub fn authenticate_current_application_version(
    verifier: &DevelopmentFinalityVerifier,
    version: &StateVersion,
) -> Result<AuthenticatedApplicationAnchor, FinalityError> {
    validate_application_identity(verifier, version)?;
    let height = i64::try_from(version.height).map_err(|_| FinalityError::HeightOverflow)?;
    let next_height = height.checked_add(1).ok_or(FinalityError::HeightOverflow)?;
    let latest = verifier
        .latest
        .as_ref()
        .ok_or(FinalityError::WrongApplicationHeight)?;
    if latest.header().height != next_height {
        return Err(FinalityError::WrongApplicationHeight);
    }
    let application = hash_application_commitment(ApplicationCommitmentInput {
        genesis: version.identity.genesis,
        protocol_version: version.identity.protocol_version,
        execution_height: version.height,
        evm_root: version.evm_root,
        system_root: version.system_root,
        execution_hash: version.execution_hash,
    })
    .map_err(FinalityError::Commitment)?;
    if version.application != Some(application) {
        return Err(FinalityError::WrongApplicationCommitment);
    }
    match_next_header_commitment(
        &verifier.identity.network_name,
        ExecutionHeight(height),
        &application.0.0,
        latest.header(),
    )
    .map_err(FinalityError::HeightMapping)?;
    Ok(AuthenticatedApplicationAnchor {
        identity: verifier.identity.clone(),
        execution_height: version.height,
        evm_root: version.evm_root,
        system_root: version.system_root,
        execution_hash: version.execution_hash,
        application,
        consensus_height: latest.header().height,
        consensus_block_id: latest.block_id().clone(),
    })
}
