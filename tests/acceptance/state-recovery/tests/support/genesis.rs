use alloy_primitives::{Address, Bytes, U256};
use ed25519_dalek::SigningKey;
use eve_protocol_config::{
    genesis::{DevelopmentGenesis, GenesisAccount, GenesisValidator, development_economics},
    network::SecurityProfile,
};

pub fn development_genesis() -> DevelopmentGenesis {
    let economics = development_economics();
    let accounts = (1..=4_u8)
        .map(|value| GenesisAccount {
            address: Address::repeat_byte(value),
            funded_balance: economics.validator_self_bond * U256::from(10),
            nonce: 0,
            code: Bytes::new(),
        })
        .collect();
    // Deliberately unsafe repeated-byte keys identify public test vectors only.
    let validators = (1..=4_u8)
        .map(|value| GenesisValidator {
            owner: Address::repeat_byte(value),
            classical_public_key: SigningKey::from_bytes(&[value; 32])
                .verifying_key()
                .to_bytes(),
            self_bond: economics.validator_self_bond,
            voting_power: 10_000,
        })
        .collect();
    DevelopmentGenesis {
        schema_version: 1,
        protocol_version: 1,
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        initial_timestamp: 1_728_000_000,
        profile: SecurityProfile::ClassicalDev,
        economics,
        accounts,
        validators,
        upgrades: Vec::new(),
    }
}

pub fn development_spec_json() -> serde_json::Value {
    let genesis = development_genesis();
    serde_json::json!({
        "schema_version": genesis.schema_version,
        "protocol_version": genesis.protocol_version,
        "network_name": genesis.network_name,
        "evm_chain_id": genesis.evm_chain_id,
        "initial_timestamp": genesis.initial_timestamp,
        "profile": "CLASSICAL_DEV",
        "accounts": genesis.accounts.iter().map(|account| serde_json::json!({ "address": account.address, "funded_balance": account.funded_balance, "nonce": account.nonce, "code": account.code })).collect::<Vec<_>>(),
        "validators": genesis.validators.iter().map(|validator| serde_json::json!({ "owner": validator.owner, "classical_public_key": format!("0x{}", hex::encode(validator.classical_public_key)), "self_bond": validator.self_bond, "voting_power": validator.voting_power })).collect::<Vec<_>>(),
    })
}
