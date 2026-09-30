#![allow(dead_code)]

use alloy_primitives::{Address, B256, Bloom, Bytes, U256};
use ed25519_dalek::SigningKey;
use eve_protocol_config::{
    genesis::{DevelopmentGenesis, GenesisAccount, GenesisValidator, development_economics},
    headers::ExecutionHeaderInput,
    network::{NetworkProfileBinding, SecurityProfile},
    records::{
        ApplicationCommitmentInput, EvmStateRoot, ExecutionBlockHash, GenesisHash, SystemNamespace,
        SystemRecord, SystemStateRoot, SystemValue,
    },
};

pub fn genesis() -> DevelopmentGenesis {
    let economics = development_economics();
    let accounts = (1_u8..=4)
        .map(|number| GenesisAccount {
            address: Address::repeat_byte(number),
            funded_balance: economics.validator_self_bond * U256::from(10),
            nonce: 0,
            code: Bytes::new(),
        })
        .collect();
    let validators = (1_u8..=4)
        .map(|number| GenesisValidator {
            owner: Address::repeat_byte(number),
            self_bond: economics.validator_self_bond,
            voting_power: 10_000,
            classical_public_key: SigningKey::from_bytes(&[number; 32])
                .verifying_key()
                .to_bytes(),
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

pub fn header() -> ExecutionHeaderInput {
    ExecutionHeaderInput {
        parent: ExecutionBlockHash(B256::repeat_byte(0x22)),
        beneficiary: Address::repeat_byte(0xaa),
        state_root: EvmStateRoot(alloy_consensus::constants::EMPTY_ROOT_HASH),
        transaction_root: alloy_consensus::constants::EMPTY_TRANSACTIONS,
        receipt_root: alloy_consensus::constants::EMPTY_RECEIPTS,
        logs_bloom: Bloom::ZERO,
        number: 1,
        gas_limit: 30_000_000,
        gas_used: 21_000,
        timestamp: 1_728_000_001,
        parent_timestamp: 1_728_000_000,
        previous_consensus_hash: B256::repeat_byte(0x33),
        base_fee: 1_000_000_000,
        protocol_version: 1,
        genesis: GenesisHash(B256::repeat_byte(0x11)),
    }
}

pub fn application() -> ApplicationCommitmentInput {
    ApplicationCommitmentInput {
        genesis: GenesisHash(B256::repeat_byte(0x11)),
        protocol_version: 1,
        execution_height: 7,
        evm_root: EvmStateRoot(B256::repeat_byte(0x22)),
        system_root: SystemStateRoot(B256::repeat_byte(0x33)),
        execution_hash: ExecutionBlockHash(B256::repeat_byte(0x44)),
    }
}

pub fn fee_record() -> SystemRecord {
    SystemRecord {
        schema_version: 1,
        namespace: SystemNamespace::Fee,
        logical_key: Bytes::from_static(b"pool"),
        value: SystemValue::Fee {
            burned: U256::ZERO,
            node_pool: U256::from(3),
            validator_pool: U256::from(4),
        },
    }
}

pub fn binding() -> NetworkProfileBinding {
    NetworkProfileBinding {
        genesis_hash: B256::repeat_byte(1),
        network_name: "eve-local-v1".into(),
        evm_chain_id: 31_337,
        protocol_version: 1,
        profile: SecurityProfile::ClassicalDev,
        activation_height: 1,
        key_epoch: 0,
    }
}
