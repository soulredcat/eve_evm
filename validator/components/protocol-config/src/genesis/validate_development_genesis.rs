use std::collections::BTreeSet;

use alloy_primitives::U256;
use ed25519_dalek::VerifyingKey;

use super::{DevelopmentGenesis, GenesisError, development_economics};
use crate::{
    native::{NODE_POOL_ADDRESS, SYSTEM_INTERFACE_ADDRESS, VALIDATOR_POOL_ADDRESS},
    network::{LaunchMode, SecurityProfile, validate_security_profile},
};

pub fn validate_development_genesis(
    mode: LaunchMode,
    genesis: &DevelopmentGenesis,
) -> Result<U256, GenesisError> {
    if mode != LaunchMode::Development {
        return Err(GenesisError::ProductionNotAuthorized);
    }
    if genesis.schema_version != 1
        || genesis.protocol_version != 1
        || genesis.network_name != "eve-local-v1"
        || genesis.evm_chain_id != 31_337
        || genesis.profile != SecurityProfile::ClassicalDev
        || genesis.initial_timestamp == 0
    {
        return Err(GenesisError::UnsupportedDevelopmentProfile);
    }
    if genesis.economics != development_economics() {
        return Err(GenesisError::InvalidEconomics);
    }
    let reserved = [
        SYSTEM_INTERFACE_ADDRESS,
        NODE_POOL_ADDRESS,
        VALIDATOR_POOL_ADDRESS,
    ];
    let mut accounts = BTreeSet::new();
    let mut supply = U256::ZERO;
    if genesis.accounts.len() > 1_024 || genesis.upgrades.len() > 16 {
        return Err(GenesisError::InvalidAccounts);
    }
    for account in &genesis.accounts {
        if reserved.contains(&account.address) {
            return Err(GenesisError::ReservedAddress);
        }
        if account.address.is_zero()
            || !accounts.insert(account.address)
            || account.code.len() > 24_576
        {
            return Err(GenesisError::InvalidAccounts);
        }
        supply = supply
            .checked_add(account.funded_balance)
            .ok_or(GenesisError::ArithmeticOverflow)?;
    }
    if genesis.validators.len() != 4 {
        return Err(GenesisError::InvalidValidators);
    }
    let mut keys = BTreeSet::new();
    let mut owners = BTreeSet::new();
    let mut total_power = 0_u64;
    let mut equal_power = None;
    for validator in &genesis.validators {
        if !accounts.contains(&validator.owner)
            || !owners.insert(validator.owner)
            || !keys.insert(validator.classical_public_key)
        {
            return Err(GenesisError::InvalidValidators);
        }
        let key = VerifyingKey::from_bytes(&validator.classical_public_key)
            .map_err(|_| GenesisError::InvalidKey)?;
        if key.is_weak() {
            return Err(GenesisError::InvalidKey);
        }
        let funding = genesis
            .accounts
            .iter()
            .find(|account| account.address == validator.owner)
            .ok_or(GenesisError::InvalidValidators)?
            .funded_balance;
        if validator.self_bond < genesis.economics.validator_self_bond
            || validator.self_bond > funding
        {
            return Err(GenesisError::UnderfundedBond);
        }
        let backed_power = validator.self_bond / U256::from(1_000_000_000_000_000_000_u64);
        if validator.voting_power == 0
            || U256::from(validator.voting_power) != backed_power
            || equal_power.is_some_and(|power| power != validator.voting_power)
        {
            return Err(GenesisError::InvalidVotingPower);
        }
        equal_power = Some(validator.voting_power);
        total_power = total_power
            .checked_add(validator.voting_power)
            .ok_or(GenesisError::ArithmeticOverflow)?;
    }
    if total_power > (i64::MAX as u64) / 8 {
        return Err(GenesisError::InvalidVotingPower);
    }
    let mut previous_height = 0;
    let mut previous_version = genesis.protocol_version;
    for upgrade in &genesis.upgrades {
        if upgrade.activation_height <= previous_height
            || upgrade.protocol_version <= previous_version
            || upgrade.code_digest.is_zero()
            || upgrade.migration_id.is_empty()
            || upgrade.migration_id.len() > 128
        {
            return Err(GenesisError::InvalidUpgrade);
        }
        validate_security_profile(upgrade.profile).map_err(|_| GenesisError::InvalidUpgrade)?;
        previous_height = upgrade.activation_height;
        previous_version = upgrade.protocol_version;
    }
    Ok(supply)
}
