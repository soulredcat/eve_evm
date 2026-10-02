// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::{DevelopmentFinalityVerifier, FinalityError, VerifiedDevelopmentHeader};
use eve_consensus_comet::{
    consensus::{certificates::ClassicalValidator, history::verify_next_native_header},
    wire::tendermint::types::{BlockId, Commit, Header},
};

/// EVE development guards precede native mutation; no unsupported upgrade is activated.
pub fn verify_next_development_header(
    verifier: &mut DevelopmentFinalityVerifier,
    block_id: &BlockId,
    header: &Header,
    commit: &Commit,
    validators: &[ClassicalValidator],
    transactions: &[Vec<u8>],
) -> Result<VerifiedDevelopmentHeader, FinalityError> {
    if header.app_hash.len() != 32 {
        return Err(FinalityError::InvalidApplicationHashWidth);
    }
    // Actual B3 genesis consensus parameters and Info bind native app version to this value.
    if header
        .version
        .as_ref()
        .is_none_or(|version| version.app != u64::from(verifier.identity.protocol_version))
    {
        return Err(FinalityError::WrongNativeApplicationVersion);
    }
    let height = u64::try_from(header.height).map_err(|_| FinalityError::WrongApplicationHeight)?;
    if verifier
        .unsupported_activation
        .is_some_and(|activation| height >= activation)
    {
        return Err(FinalityError::UnsupportedActivation);
    }
    let native = verify_next_native_header(
        &mut verifier.native,
        block_id,
        header,
        commit,
        validators,
        transactions,
    )
    .map_err(FinalityError::Native)?;
    verifier.latest = Some(native.clone());
    Ok(VerifiedDevelopmentHeader {
        identity: verifier.identity.clone(),
        native,
    })
}
