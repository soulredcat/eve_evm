// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DevelopmentSpecError, decode_public_key::decode_public_key, types::DevelopmentSpecInput,
};
use crate::{
    genesis::{
        DevelopmentGenesis, GenesisAccount, GenesisValidator, development_economics,
        validate_development_genesis,
    },
    network::{LaunchMode, SecurityProfile},
};

pub fn decode_development_spec(bytes: &[u8]) -> Result<DevelopmentGenesis, DevelopmentSpecError> {
    if bytes.len() > 1_048_576 {
        return Err(DevelopmentSpecError::TooLarge);
    }
    let input: DevelopmentSpecInput =
        serde_json::from_slice(bytes).map_err(|_| DevelopmentSpecError::MalformedInput)?;
    if input.profile != "CLASSICAL_DEV" {
        return Err(DevelopmentSpecError::UnsupportedProfile);
    }
    let validators = input
        .validators
        .into_iter()
        .map(|validator| {
            Ok(GenesisValidator {
                owner: validator.owner,
                classical_public_key: decode_public_key(&validator.classical_public_key)?,
                self_bond: validator.self_bond,
                voting_power: validator.voting_power,
            })
        })
        .collect::<Result<Vec<_>, DevelopmentSpecError>>()?;
    let genesis = DevelopmentGenesis {
        schema_version: input.schema_version,
        protocol_version: input.protocol_version,
        network_name: input.network_name,
        evm_chain_id: input.evm_chain_id,
        initial_timestamp: input.initial_timestamp,
        profile: SecurityProfile::ClassicalDev,
        economics: development_economics(),
        accounts: input
            .accounts
            .into_iter()
            .map(|account| GenesisAccount {
                address: account.address,
                funded_balance: account.funded_balance,
                nonce: account.nonce,
                code: account.code,
            })
            .collect(),
        validators,
        upgrades: Vec::new(),
    };
    validate_development_genesis(LaunchMode::Development, &genesis)
        .map_err(DevelopmentSpecError::InvalidGenesis)?;
    Ok(genesis)
}
