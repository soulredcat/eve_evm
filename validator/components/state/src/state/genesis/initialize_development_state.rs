// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

use alloy_consensus::constants::{EMPTY_RECEIPTS, EMPTY_TRANSACTIONS};
use alloy_primitives::{Address, B256, Bloom, Bytes, U256, keccak256};
use eve_protocol_config::{
    genesis::{DevelopmentGenesis, hash_development_genesis, validate_development_genesis},
    headers::{ExecutionHeaderInput, build_execution_header},
    native::SYSTEM_INTERFACE_ADDRESS,
    network::LaunchMode,
    records::{
        ExecutionBlockHash, GenesisHash, SystemNamespace, SystemRecord, SystemValue,
        hash_system_key,
    },
};

use super::genesis_parameters::genesis_parameters;
use crate::{
    BlockPayload, CompleteState, StateAccount, StateBudget, StateCommit, StateError, StateIdentity,
    build_state_commit, compute_evm_root,
};

pub fn initialize_development_state(
    genesis: &DevelopmentGenesis,
    budget: &StateBudget,
) -> Result<StateCommit, StateError> {
    let expected_supply = validate_development_genesis(LaunchMode::Development, genesis)
        .map_err(StateError::Genesis)?;
    let genesis_hash = GenesisHash(
        hash_development_genesis(LaunchMode::Development, genesis).map_err(StateError::Genesis)?,
    );
    let identity = StateIdentity {
        genesis: genesis_hash,
        network_name: genesis.network_name.clone(),
        evm_chain_id: genesis.evm_chain_id,
        protocol_version: genesis.protocol_version,
        security_profile: genesis.profile,
        key_epoch: 0,
        configuration_digest: genesis_hash.0,
    };
    let mut state = CompleteState {
        identity,
        accounts: BTreeMap::new(),
        codes: BTreeMap::new(),
        system: BTreeMap::new(),
        block_hashes: BTreeMap::new(),
    };
    let mut escrow = U256::ZERO;
    for input in &genesis.accounts {
        let bond = genesis
            .validators
            .iter()
            .find(|validator| validator.owner == input.address)
            .map_or(U256::ZERO, |validator| validator.self_bond);
        escrow = escrow
            .checked_add(bond)
            .ok_or(StateError::ArithmeticOverflow)?;
        let code_hash = keccak256(&input.code);
        if !input.code.is_empty() {
            state.codes.insert(code_hash, input.code.clone());
        }
        state.accounts.insert(
            input.address,
            StateAccount {
                nonce: input.nonce,
                balance: input
                    .funded_balance
                    .checked_sub(bond)
                    .ok_or(StateError::CommitMismatch)?,
                code_hash,
                storage: BTreeMap::new(),
            },
        );
    }
    state.accounts.insert(
        SYSTEM_INTERFACE_ADDRESS,
        StateAccount {
            nonce: 0,
            balance: escrow,
            code_hash: alloy_trie::KECCAK_EMPTY,
            storage: BTreeMap::new(),
        },
    );
    let supply = state
        .accounts
        .values()
        .try_fold(U256::ZERO, |sum, account| {
            sum.checked_add(account.balance)
                .ok_or(StateError::ArithmeticOverflow)
        })?;
    if supply != expected_supply {
        return Err(StateError::CommitMismatch);
    }
    for validator in &genesis.validators {
        let record = SystemRecord {
            schema_version: 1,
            namespace: SystemNamespace::Validator,
            logical_key: Bytes::copy_from_slice(validator.owner.as_slice()),
            value: SystemValue::Validator {
                owner: validator.owner,
                key: validator.classical_public_key,
                power: validator.voting_power,
                activation: 1,
                removal: None,
                key_epoch: 0,
            },
        };
        let key = hash_system_key(record.namespace, &record.logical_key)
            .map_err(StateError::SystemRecord)?;
        state.system.insert(key, record);
    }
    let fee = SystemRecord {
        schema_version: 1,
        namespace: SystemNamespace::Fee,
        logical_key: Bytes::from_static(b"pool"),
        value: SystemValue::Fee {
            burned: U256::ZERO,
            node_pool: U256::ZERO,
            validator_pool: U256::ZERO,
        },
    };
    state.system.insert(
        hash_system_key(fee.namespace, &fee.logical_key).map_err(StateError::SystemRecord)?,
        fee,
    );
    for record in genesis_parameters(genesis) {
        state.system.insert(
            hash_system_key(record.namespace, &record.logical_key)
                .map_err(StateError::SystemRecord)?,
            record,
        );
    }
    let header = build_execution_header(&ExecutionHeaderInput {
        parent: ExecutionBlockHash(B256::ZERO),
        beneficiary: Address::ZERO,
        state_root: compute_evm_root(&state.accounts),
        transaction_root: EMPTY_TRANSACTIONS,
        receipt_root: EMPTY_RECEIPTS,
        logs_bloom: Bloom::ZERO,
        number: 0,
        gas_limit: 30_000_000,
        gas_used: 0,
        timestamp: genesis.initial_timestamp,
        parent_timestamp: genesis.initial_timestamp,
        previous_consensus_hash: B256::ZERO,
        base_fee: 1_000_000_000,
        protocol_version: genesis.protocol_version,
        genesis: genesis_hash,
    })
    .map_err(|_| StateError::CommitMismatch)?;
    build_state_commit(
        None,
        state,
        BlockPayload {
            header,
            transactions: Vec::new(),
            receipts: Vec::new(),
        },
        budget,
    )
}
