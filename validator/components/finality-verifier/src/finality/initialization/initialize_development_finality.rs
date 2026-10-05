// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::finality::{DevelopmentFinalityVerifier, FinalityError};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{ClassicalValidator, canonicalize_validator_set},
    history::initialize_genesis_history,
};
use eve_protocol_config::genesis::DevelopmentGenesis;
use eve_state::{StateBudget, initialize_development_state};

/// Only locally selected development genesis enters this explicit trust boundary.
pub fn initialize_development_finality(
    genesis: &DevelopmentGenesis,
    budget: &StateBudget,
) -> Result<DevelopmentFinalityVerifier, FinalityError> {
    let initial = initialize_development_state(genesis, budget).map_err(FinalityError::State)?;
    let validators = genesis
        .validators
        .iter()
        .map(|validator| {
            Ok(ClassicalValidator {
                public_key: validator.classical_public_key,
                voting_power: i64::try_from(validator.voting_power)
                    .map_err(|_| FinalityError::HeightOverflow)?,
            })
        })
        .collect::<Result<Vec<_>, FinalityError>>()?;
    let validators = canonicalize_validator_set(&validators).map_err(FinalityError::Certificate)?;
    let native = initialize_genesis_history(
        &genesis.network_name,
        &validators,
        initial.target.content_digest.0,
        ConsensusAuthenticationRequirement::ClassicalDev,
    )
    .map_err(FinalityError::Native)?;
    Ok(DevelopmentFinalityVerifier {
        identity: initial.target.identity,
        native,
        latest: None,
        unsupported_activation: genesis
            .upgrades
            .first()
            .map(|upgrade| upgrade.activation_height),
    })
}
