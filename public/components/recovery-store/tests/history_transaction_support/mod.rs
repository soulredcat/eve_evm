// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{SignableTransaction, TxEnvelope, TxLegacy};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Address, B256, Bytes, Signature, TxKind, U256, keccak256};
use eve_evm::{ExecutionBlockInput, estimate_clone_reservation, execute_state_block};
use eve_state::{
    DevelopmentGenesis, GenesisAccount, GenesisValidator, SecurityProfile, StateCommit,
    build_state_commit, development_economics, development_state_budget,
    initialize_development_state,
};
use k256::ecdsa::SigningKey;

pub(crate) fn source_fixture() -> (StateCommit, StateCommit, StateCommit, B256) {
    // Ephemeral OS-generated test key stays in memory and never controls real funds.
    let key = SigningKey::random(&mut k256::elliptic_curve::rand_core::OsRng);
    let public = key.verifying_key().to_encoded_point(false);
    let sender = Address::from_slice(&keccak256(&public.as_bytes()[1..])[12..]);
    let economics = development_economics();
    let keys = [
        "8a88e3dd7409f195fd52db2d3cba5d72ca6709bf1d94121bf3748801b40f6f5c",
        "8139770ea87d175f56a35466c34c7ecccb8d8a91b4ee37a25df60f5b8fc9b394",
        "ed4928c628d1c2c6eae90338905995612959273a5c63f93636c14614ac8737d1",
        "ca93ac1705187071d67b83c7ff0efe8108e8ec4530575d7726879333dbdabe7c",
    ];
    let mut accounts: Vec<_> = (1_u8..=4)
        .map(|number| GenesisAccount {
            address: Address::repeat_byte(number),
            funded_balance: economics.validator_self_bond * U256::from(10),
            nonce: 0,
            code: Bytes::new(),
        })
        .collect();
    accounts.push(GenesisAccount {
        address: sender,
        funded_balance: U256::from(10).pow(U256::from(20)),
        nonce: 0,
        code: Bytes::new(),
    });
    let validators = (1_u8..=4)
        .map(|number| GenesisValidator {
            owner: Address::repeat_byte(number),
            classical_public_key: keys[usize::from(number - 1)].parse::<B256>().unwrap().0,
            self_bond: economics.validator_self_bond,
            voting_power: 10_000,
        })
        .collect();
    let budget = development_state_budget();
    let genesis = initialize_development_state(
        &DevelopmentGenesis {
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
        },
        &budget,
    )
    .unwrap();
    let transaction = TxLegacy {
        chain_id: Some(31_337),
        nonce: 0,
        gas_price: 2_000_000_000,
        gas_limit: 21_000,
        to: TxKind::Call(Address::repeat_byte(99)),
        value: U256::from(1),
        input: Bytes::new(),
    };
    let (signature, recovery) = key
        .sign_prehash_recoverable(transaction.signature_hash().as_slice())
        .unwrap();
    let signature = Signature::from_bytes_and_parity(&signature.to_bytes(), recovery.is_y_odd());
    let raw: Bytes = TxEnvelope::Legacy(transaction.into_signed(signature))
        .encoded_2718()
        .into();
    let input = ExecutionBlockInput {
        timestamp: genesis.target.timestamp + 1,
        proposer: Address::ZERO,
        previous_consensus_hash: B256::ZERO,
    };
    let first = execute_state_block(
        &genesis,
        &input,
        std::slice::from_ref(&raw),
        &budget,
        estimate_clone_reservation(&genesis.state).unwrap(),
    )
    .unwrap()
    .commit;
    // Structural B1 recovery fixture with a stale-nonce duplicate. No valid second execution/finality is claimed.
    let mut block = first.block.clone();
    block.header.number += 1;
    block.header.timestamp += 1;
    block.header.parent_hash = first.target.execution_hash.0;
    block.header.base_fee_per_gas = block
        .header
        .next_block_base_fee(alloy_eips::eip1559::BaseFeeParams::ethereum());
    let second = build_state_commit(
        Some(first.target.clone()),
        first.state.clone(),
        block,
        &budget,
    )
    .unwrap();
    (genesis, first, second, keccak256(raw))
}
