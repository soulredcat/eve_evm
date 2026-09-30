use std::collections::BTreeMap;

use alloy_primitives::{Address, B256, Bytes, U256};
use eve_protocol_config::{
    network::SecurityProfile,
    records::{GenesisHash, SystemNamespace, SystemRecord, SystemValue, hash_system_key},
};
use eve_state::{CompleteState, StateAccount, StateIdentity};
use serde_json::Value;

pub fn fixture(name: &str) -> Value {
    let json = match name {
        "two_slot_contract" => include_str!("../fixtures/trie-v1/two_slot_contract.json"),
        "updated_slot_preserves_other" => {
            include_str!("../fixtures/trie-v1/updated_slot_preserves_other.json")
        }
        "zero_slot_is_deleted" => include_str!("../fixtures/trie-v1/zero_slot_is_deleted.json"),
        "deleted_account" => include_str!("../fixtures/trie-v1/deleted_account.json"),
        "recreated_account" => include_str!("../fixtures/trie-v1/recreated_account.json"),
        "execution_parent" => include_str!("../fixtures/trie-v1/execution_parent.json"),
        "execution_post" => include_str!("../fixtures/trie-v1/execution_post.json"),
        _ => panic!("unknown independent fixture"),
    };
    let value: Value = serde_json::from_str(json).unwrap();
    assert_eq!(value["version"], 1);
    assert_eq!(value["name"], name);
    value
}

pub fn identity() -> StateIdentity {
    StateIdentity {
        genesis: GenesisHash(B256::repeat_byte(0x11)),
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        protocol_version: 1,
        security_profile: SecurityProfile::ClassicalDev,
        key_epoch: 0,
        configuration_digest: B256::repeat_byte(0x22),
    }
}

pub fn complete_state(name: &str) -> CompleteState {
    let input = fixture(name);
    let mut accounts = BTreeMap::new();
    let mut codes = BTreeMap::new();
    for account in input["accounts"].as_array().unwrap() {
        let code: Bytes = account["code"].as_str().unwrap().parse().unwrap();
        let code_hash: B256 = account["code_hash"].as_str().unwrap().parse().unwrap();
        if !code.is_empty() {
            codes.insert(code_hash, code);
        }
        let storage = account["slots"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|slot| {
                let value: U256 = slot[1].as_str().unwrap().parse().unwrap();
                (!value.is_zero())
                    .then(|| (slot[0].as_str().unwrap().parse::<U256>().unwrap(), value))
            })
            .collect();
        accounts.insert(
            account["address"]
                .as_str()
                .unwrap()
                .parse::<Address>()
                .unwrap(),
            StateAccount {
                nonce: account["nonce"].as_str().unwrap().parse().unwrap(),
                balance: account["balance"].as_str().unwrap().parse().unwrap(),
                code_hash,
                storage,
            },
        );
    }
    let fees = input["fee_ledger"].as_array().unwrap();
    let record = SystemRecord {
        schema_version: 1,
        namespace: SystemNamespace::Fee,
        logical_key: Bytes::from_static(b"pool"),
        value: SystemValue::Fee {
            burned: fees[0].as_str().unwrap().parse().unwrap(),
            node_pool: fees[1].as_str().unwrap().parse().unwrap(),
            validator_pool: fees[2].as_str().unwrap().parse().unwrap(),
        },
    };
    let key = hash_system_key(record.namespace, &record.logical_key).unwrap();
    CompleteState {
        identity: identity(),
        accounts,
        codes,
        system: [(key, record)].into(),
        block_hashes: BTreeMap::new(),
    }
}

pub fn expected_root(input: &Value, field: &str) -> B256 {
    input[field].as_str().unwrap().parse().unwrap()
}
