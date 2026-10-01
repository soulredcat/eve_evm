// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod support;
use eve_protocol_config::genesis::{
    hash_development_genesis,
    input::{DevelopmentSpecError, decode_development_spec},
};
use eve_protocol_config::network::LaunchMode;

fn specification() -> serde_json::Value {
    let genesis = support::genesis();
    serde_json::json!({
        "schema_version": genesis.schema_version, "protocol_version": genesis.protocol_version,
        "network_name": genesis.network_name, "evm_chain_id": genesis.evm_chain_id,
        "initial_timestamp": genesis.initial_timestamp, "profile": "CLASSICAL_DEV",
        "accounts": genesis.accounts.iter().map(|account| serde_json::json!({
            "address": account.address, "funded_balance": account.funded_balance,
            "nonce": account.nonce, "code": account.code,
        })).collect::<Vec<_>>(),
        "validators": genesis.validators.iter().map(|validator| serde_json::json!({
            "owner": validator.owner, "classical_public_key": format!("0x{}", hex::encode(validator.classical_public_key)),
            "self_bond": validator.self_bond, "voting_power": validator.voting_power,
        })).collect::<Vec<_>>()
    })
}

#[test]
fn pure_development_spec_decoder_preserves_canonical_genesis_identity() {
    let actual = decode_development_spec(&serde_json::to_vec(&specification()).unwrap()).unwrap();
    assert_eq!(actual, support::genesis());
    assert_eq!(
        hash_development_genesis(LaunchMode::Development, &actual).unwrap(),
        hash_development_genesis(LaunchMode::Development, &support::genesis()).unwrap()
    );
}

#[test]
fn development_spec_rejects_unknown_fields_profile_keys_duplicates_and_size() {
    let mut input = specification();
    input["signer_seed"] = serde_json::json!("prohibited-field");
    assert_eq!(
        decode_development_spec(&serde_json::to_vec(&input).unwrap()),
        Err(DevelopmentSpecError::MalformedInput)
    );
    input = specification();
    input["profile"] = serde_json::json!("HYBRID");
    assert_eq!(
        decode_development_spec(&serde_json::to_vec(&input).unwrap()),
        Err(DevelopmentSpecError::UnsupportedProfile)
    );
    input = specification();
    let key = input["validators"][0]["classical_public_key"]
        .as_str()
        .unwrap()
        .to_uppercase();
    input["validators"][0]["classical_public_key"] = serde_json::json!(key);
    assert_eq!(
        decode_development_spec(&serde_json::to_vec(&input).unwrap()),
        Err(DevelopmentSpecError::InvalidPublicKey)
    );
    let mut duplicate = serde_json::to_string(&specification()).unwrap();
    duplicate.pop();
    duplicate.push_str(",\"profile\":\"CLASSICAL_DEV\"}");
    assert_eq!(
        decode_development_spec(duplicate.as_bytes()),
        Err(DevelopmentSpecError::MalformedInput)
    );
    assert_eq!(
        decode_development_spec(&vec![b' '; 1_048_577]),
        Err(DevelopmentSpecError::TooLarge)
    );
}
