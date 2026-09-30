use alloy_primitives::U256;

use super::{
    DevelopmentGenesis, GenesisError, encode_economics::encode_economics,
    validate_development_genesis,
};
use crate::{
    encoding::encode_list,
    native::{NATIVE_GAS_V1, NODE_POOL_ADDRESS, SYSTEM_INTERFACE_ADDRESS, VALIDATOR_POOL_ADDRESS},
    network::LaunchMode,
};

pub fn encode_development_genesis(
    mode: LaunchMode,
    genesis: &DevelopmentGenesis,
) -> Result<Vec<u8>, GenesisError> {
    let supply = validate_development_genesis(mode, genesis)?;
    let mut accounts = genesis.accounts.iter().collect::<Vec<_>>();
    accounts.sort_by_key(|account| account.address);
    let accounts = accounts
        .into_iter()
        .map(|account| {
            let bond = genesis
                .validators
                .iter()
                .find(|validator| validator.owner == account.address)
                .map_or(U256::ZERO, |validator| validator.self_bond);
            encode_list(&[
                alloy_rlp::encode(account.address),
                alloy_rlp::encode(account.funded_balance - bond),
                alloy_rlp::encode(account.nonce),
                alloy_rlp::encode(account.code.as_ref()),
            ])
        })
        .collect::<Vec<_>>();
    let mut validators = genesis.validators.iter().collect::<Vec<_>>();
    validators.sort_by_key(|validator| validator.classical_public_key);
    let escrow = validators.iter().try_fold(U256::ZERO, |sum, validator| {
        sum.checked_add(validator.self_bond)
            .ok_or(GenesisError::ArithmeticOverflow)
    })?;
    let validators = validators
        .into_iter()
        .map(|validator| {
            encode_list(&[
                alloy_rlp::encode(validator.owner),
                alloy_rlp::encode(validator.classical_public_key.as_slice()),
                alloy_rlp::encode(validator.self_bond),
                alloy_rlp::encode(validator.voting_power),
            ])
        })
        .collect::<Vec<_>>();
    let upgrades = genesis
        .upgrades
        .iter()
        .map(|upgrade| {
            encode_list(&[
                alloy_rlp::encode(upgrade.activation_height),
                alloy_rlp::encode(upgrade.protocol_version),
                alloy_rlp::encode(upgrade.profile as u8),
                alloy_rlp::encode(upgrade.code_digest),
                alloy_rlp::encode(upgrade.migration_id.as_ref()),
            ])
        })
        .collect::<Vec<_>>();
    let gas = NATIVE_GAS_V1;
    Ok(encode_list(&[
        alloy_rlp::encode(b"EVE_GENESIS_V1".as_slice()),
        alloy_rlp::encode(genesis.schema_version),
        alloy_rlp::encode(genesis.protocol_version),
        alloy_rlp::encode(genesis.network_name.as_bytes()),
        alloy_rlp::encode(genesis.evm_chain_id),
        alloy_rlp::encode(genesis.initial_timestamp),
        alloy_rlp::encode(genesis.profile as u8),
        encode_list(&[
            alloy_rlp::encode(30_000_000_u64),
            alloy_rlp::encode(4_194_304_u64),
            alloy_rlp::encode(1_000_u64),
            alloy_rlp::encode(86_400_u64),
            alloy_rlp::encode(86_400_u64),
            alloy_rlp::encode(10_000_u64),
            alloy_rlp::encode(86_400_u64),
        ]),
        encode_list(&[
            alloy_rlp::encode(b"SHANGHAI".as_slice()),
            alloy_rlp::encode(131_072_u64),
            alloy_rlp::encode(1_000_000_000_u64),
            alloy_rlp::encode(1_u64),
        ]),
        encode_economics(&genesis.economics),
        encode_list(&[
            alloy_rlp::encode(SYSTEM_INTERFACE_ADDRESS),
            alloy_rlp::encode(NODE_POOL_ADDRESS),
            alloy_rlp::encode(VALIDATOR_POOL_ADDRESS),
        ]),
        encode_list(&[
            alloy_rlp::encode(gas.version),
            alloy_rlp::encode(gas.dispatch),
            alloy_rlp::encode(gas.calldata_zero_byte),
            alloy_rlp::encode(gas.calldata_nonzero_byte),
            alloy_rlp::encode(gas.proof_byte),
            alloy_rlp::encode(gas.classical_verification),
            alloy_rlp::encode(gas.mldsa65_verification),
            alloy_rlp::encode(gas.storage_read),
            alloy_rlp::encode(gas.storage_new_write),
            alloy_rlp::encode(gas.storage_existing_write),
            alloy_rlp::encode(gas.maximum_calldata_bytes),
            alloy_rlp::encode(gas.maximum_proof_bytes),
            alloy_rlp::encode(gas.maximum_signature_checks),
            alloy_rlp::encode(gas.maximum_storage_operations),
        ]),
        encode_list(&accounts),
        encode_list(&validators),
        encode_list(&upgrades),
        alloy_rlp::encode(escrow),
        alloy_rlp::encode(supply),
    ]))
}
