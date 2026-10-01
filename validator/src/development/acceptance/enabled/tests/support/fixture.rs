// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::contract_compiler::compile_transition_contract;
use crate::development::acceptance::enabled::{AcceptanceFixture, load_acceptance_fixture};
use alloy_primitives::{Address, Bytes, U256, keccak256};
use ed25519_dalek::SigningKey as ConsensusKey;
use eve_state::{
    DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile, development_economics,
};
use k256::ecdsa::SigningKey;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    path::PathBuf,
    sync::{Arc, OnceLock},
};

pub(in crate::development::acceptance::enabled::tests) struct TestFixture {
    pub root: tempfile::TempDir,
    pub path: PathBuf,
    pub document: Value,
    pub genesis: DevelopmentGenesis,
    pub authority: SigningKey,
    pub unauthorized: SigningKey,
    pub contract: Address,
}

pub(in crate::development::acceptance::enabled::tests) fn fixture() -> TestFixture {
    static CODE: OnceLock<Vec<u8>> = OnceLock::new();
    let root = tempfile::tempdir().unwrap();
    // Deliberately unsafe deterministic test keys; never node defaults or production credentials.
    let authority = SigningKey::from_bytes((&[7; 32]).into()).unwrap();
    let unauthorized = SigningKey::from_bytes((&[8; 32]).into()).unwrap();
    let public = authority.verifying_key().to_encoded_point(false);
    let address = Address::from_slice(&keccak256(&public.as_bytes()[1..])[12..]);
    let contract =
        Address::from_slice(&hex::decode("000000000000000000000000000000000000f1b3").unwrap());
    let keys: Vec<_> = (1..=5)
        .map(|index| {
            ConsensusKey::from_bytes(&[index; 32])
                .verifying_key()
                .to_bytes()
        })
        .collect();
    let document = json!({"version":1,"contract_address":contract,"authority":address,"future_validators":[{"public_key":hex::encode(keys[4]),"owner":Address::repeat_byte(1)}],"transitions":[{"action":1,"kind":"rotate","updates":[{"public_key":hex::encode(keys[0]),"power":0},{"public_key":hex::encode(keys[4]),"power":10000}]},{"action":2,"kind":"leave","updates":[{"public_key":hex::encode(keys[1]),"power":0}]},{"action":3,"kind":"jail","updates":[{"public_key":hex::encode(keys[2]),"power":0}]}],"poison":null});
    let economics = development_economics();
    let mut accounts: Vec<_> = (1..=4)
        .map(|index| GenesisAccount {
            address: Address::repeat_byte(index),
            funded_balance: economics.validator_self_bond * U256::from(10),
            nonce: 0,
            code: Bytes::new(),
        })
        .collect();
    accounts.push(GenesisAccount {
        address,
        funded_balance: U256::from(10).pow(U256::from(24)),
        nonce: 0,
        code: Bytes::new(),
    });
    let public = unauthorized.verifying_key().to_encoded_point(false);
    accounts.push(GenesisAccount {
        address: Address::from_slice(&keccak256(&public.as_bytes()[1..])[12..]),
        funded_balance: U256::from(10).pow(U256::from(24)),
        nonce: 0,
        code: Bytes::new(),
    });
    let code = CODE.get_or_init(|| {
        let solc = PathBuf::from(
            std::env::var_os("EVE_SOLC_BINARY").expect("actual pinned Solidity compiler required"),
        );
        compile_transition_contract(&solc, &root.path().join("compiler.json")).unwrap()
    });
    accounts.push(GenesisAccount {
        address: contract,
        funded_balance: U256::ZERO,
        nonce: 0,
        code: Bytes::copy_from_slice(code),
    });
    let validators = (0..4)
        .map(|index| GenesisValidator {
            owner: Address::repeat_byte(u8::try_from(index + 1).unwrap()),
            classical_public_key: keys[index],
            self_bond: economics.validator_self_bond,
            voting_power: 10000,
        })
        .collect();
    let genesis = DevelopmentGenesis {
        schema_version: 1,
        protocol_version: 1,
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31337,
        initial_timestamp: 1,
        profile: SecurityProfile::ClassicalDev,
        economics,
        accounts,
        validators,
        upgrades: Vec::new(),
    };
    let path = root.path().join("fixture.json");
    let mut fixture = TestFixture {
        root,
        path,
        document,
        genesis,
        authority,
        unauthorized,
        contract,
    };
    rebind(&mut fixture);
    fixture
}

pub(in crate::development::acceptance::enabled::tests) fn rebind(fixture: &mut TestFixture) {
    let raw = serde_json::to_vec(&fixture.document).unwrap();
    std::fs::write(&fixture.path, &raw).unwrap();
    let account = fixture
        .genesis
        .accounts
        .iter_mut()
        .find(|account| account.address == fixture.contract)
        .unwrap();
    let length = account.code.len();
    let previous = if account.code.ends_with(&Sha256::digest(&raw)) {
        length - 72
    } else {
        let tag = b"EVE_B3_ACCEPTANCE_V1";
        if length >= 72 && &account.code[length - 52..length - 32] == tag {
            length - 72
        } else {
            length
        }
    };
    let authority: Address = serde_json::from_value(fixture.document["authority"].clone()).unwrap();
    let mut code = account.code[..previous].to_vec();
    code.extend_from_slice(authority.as_slice());
    code.extend_from_slice(b"EVE_B3_ACCEPTANCE_V1");
    code.extend_from_slice(&Sha256::digest(&raw));
    account.code = code.into();
}

impl TestFixture {
    pub(in crate::development::acceptance::enabled::tests) fn load(
        &self,
    ) -> Arc<AcceptanceFixture> {
        load_acceptance_fixture(Some(&self.path), &self.genesis)
            .unwrap()
            .unwrap()
    }
}
