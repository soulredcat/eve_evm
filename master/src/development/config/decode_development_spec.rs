use super::types::DevelopmentSpecInput;
use anyhow::{Result, ensure};
use eve_protocol_config::{
    genesis::{
        DevelopmentGenesis, GenesisAccount, GenesisValidator, development_economics,
        validate_development_genesis,
    },
    network::{LaunchMode, SecurityProfile},
};

pub fn decode_development_spec(bytes: &[u8]) -> Result<DevelopmentGenesis> {
    ensure!(
        bytes.len() <= 1024 * 1024,
        "development genesis file exceeds 1 MiB"
    );
    let input: DevelopmentSpecInput = serde_json::from_slice(bytes)?;
    ensure!(
        input.profile == "CLASSICAL_DEV",
        "B1 harness requires explicit CLASSICAL_DEV"
    );
    let mut validators = Vec::with_capacity(input.validators.len());
    for validator in input.validators {
        let key = validator
            .classical_public_key
            .strip_prefix("0x")
            .ok_or_else(|| anyhow::anyhow!("consensus public key must use canonical 0x hex"))?;
        ensure!(
            key.len() == 64,
            "consensus public key must be exactly 32 bytes"
        );
        let decoded = hex::decode(key)?;
        let classical_public_key: [u8; 32] = decoded
            .try_into()
            .map_err(|_| anyhow::anyhow!("consensus public key must be exactly 32 bytes"))?;
        ensure!(
            hex::encode(classical_public_key) == key,
            "consensus public key must use lowercase canonical hex"
        );
        validators.push(GenesisValidator {
            owner: validator.owner,
            classical_public_key,
            self_bond: validator.self_bond,
            voting_power: validator.voting_power,
        });
    }
    let accounts = input
        .accounts
        .into_iter()
        .map(|account| GenesisAccount {
            address: account.address,
            funded_balance: account.funded_balance,
            nonce: account.nonce,
            code: account.code,
        })
        .collect();
    let genesis = DevelopmentGenesis {
        schema_version: input.schema_version,
        protocol_version: input.protocol_version,
        network_name: input.network_name,
        evm_chain_id: input.evm_chain_id,
        initial_timestamp: input.initial_timestamp,
        profile: SecurityProfile::ClassicalDev,
        economics: development_economics(),
        accounts,
        validators,
        upgrades: Vec::new(),
    };
    validate_development_genesis(LaunchMode::Development, &genesis)
        .map_err(|error| anyhow::anyhow!("invalid development genesis: {error:?}"))?;
    Ok(genesis)
}
