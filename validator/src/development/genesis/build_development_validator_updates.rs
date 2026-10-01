// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use eve_consensus_comet::{
    consensus::certificates::{ClassicalValidator, canonicalize_validator_set},
    wire::tendermint::{
        abci::ValidatorUpdate,
        crypto::{PublicKey, public_key::Sum},
    },
};
use eve_protocol_config::genesis::DevelopmentGenesis;

pub(crate) fn build_development_validator_updates(
    genesis: &DevelopmentGenesis,
) -> Result<Vec<ValidatorUpdate>> {
    let validators = genesis
        .validators
        .iter()
        .map(|validator| {
            Ok(ClassicalValidator {
                public_key: validator.classical_public_key,
                voting_power: i64::try_from(validator.voting_power)?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let ordered = canonicalize_validator_set(&validators)
        .map_err(|_| anyhow::anyhow!("invalid development validator set"))?;
    Ok(ordered
        .iter()
        .map(|validator| ValidatorUpdate {
            pub_key: Some(PublicKey {
                sum: Some(Sum::Ed25519(validator.public_key.to_vec())),
            }),
            power: validator.voting_power,
        })
        .collect())
}
