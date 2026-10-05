// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ed25519_dalek::SigningKey;
use eve_state::{
    Address, Bytes, DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile, U256,
    development_economics,
};

/// Locally trusted, funded development genesis; keys are unsafe test-only identities.
pub fn genesis() -> DevelopmentGenesis {
    let economics = development_economics();
    DevelopmentGenesis {
        schema_version: 1,
        protocol_version: 1,
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        initial_timestamp: 1_728_000_000,
        profile: SecurityProfile::ClassicalDev,
        accounts: (1..=4_u8)
            .map(|seed| GenesisAccount {
                address: Address::repeat_byte(seed),
                funded_balance: economics.validator_self_bond * U256::from(10),
                nonce: 0,
                code: Bytes::new(),
            })
            .collect(),
        validators: (1..=4_u8)
            .map(|seed| GenesisValidator {
                owner: Address::repeat_byte(seed),
                classical_public_key: SigningKey::from_bytes(&[seed; 32])
                    .verifying_key()
                    .to_bytes(),
                self_bond: economics.validator_self_bond,
                voting_power: 10_000,
            })
            .collect(),
        economics,
        upgrades: Vec::new(),
    }
}
