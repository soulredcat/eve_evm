#[path = "support/mod.rs"]
mod support;

use alloy_primitives::{B256, Bytes, U256};
use eve_protocol_config::{
    genesis::{GenesisError, ScheduledUpgrade, validate_development_genesis},
    native::{NODE_POOL_ADDRESS, SYSTEM_INTERFACE_ADDRESS, VALIDATOR_POOL_ADDRESS},
    network::{LaunchMode, SecurityProfile},
};

#[test]
fn accepts_funded_genesis_and_refuses_production_economics_or_identity_changes() {
    let mut genesis = support::genesis();
    let supply = validate_development_genesis(LaunchMode::Development, &genesis).unwrap();
    assert_eq!(
        supply,
        genesis.economics.validator_self_bond * U256::from(40)
    );
    assert_eq!(
        validate_development_genesis(LaunchMode::Production, &genesis),
        Err(GenesisError::ProductionNotAuthorized)
    );
    genesis.economics.burn_bps = 0;
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::InvalidEconomics)
    );
    genesis = support::genesis();
    genesis.evm_chain_id = 1;
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::UnsupportedDevelopmentProfile)
    );
}

#[test]
fn rejects_reserved_allocations_duplicates_unfunded_keys_and_overflow() {
    for address in [
        SYSTEM_INTERFACE_ADDRESS,
        NODE_POOL_ADDRESS,
        VALIDATOR_POOL_ADDRESS,
    ] {
        let mut genesis = support::genesis();
        genesis.accounts[0].address = address;
        assert_eq!(
            validate_development_genesis(LaunchMode::Development, &genesis),
            Err(GenesisError::ReservedAddress)
        );
    }
    let mut genesis = support::genesis();
    genesis.accounts[1] = genesis.accounts[0].clone();
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::InvalidAccounts)
    );
    genesis = support::genesis();
    genesis.validators[1].classical_public_key = genesis.validators[0].classical_public_key;
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::InvalidValidators)
    );
    genesis = support::genesis();
    genesis.validators[0].classical_public_key = [0; 32];
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::InvalidKey)
    );
    genesis = support::genesis();
    genesis.accounts[0].funded_balance = U256::ZERO;
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::UnderfundedBond)
    );
    genesis = support::genesis();
    genesis.accounts[0].funded_balance = U256::MAX;
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::ArithmeticOverflow)
    );
}

#[test]
fn rejects_invalid_upgrade_order_and_unsupported_hybrid_activation() {
    let mut genesis = support::genesis();
    let upgrade = ScheduledUpgrade {
        activation_height: 10,
        protocol_version: 2,
        profile: SecurityProfile::ClassicalDev,
        code_digest: B256::repeat_byte(2),
        migration_id: Bytes::from_static(b"v2"),
    };
    genesis.upgrades = vec![upgrade.clone()];
    assert!(validate_development_genesis(LaunchMode::Development, &genesis).is_ok());
    genesis.upgrades.push(upgrade);
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::InvalidUpgrade)
    );
    genesis.upgrades.truncate(1);
    genesis.upgrades[0].profile = SecurityProfile::HybridExperimental;
    assert_eq!(
        validate_development_genesis(LaunchMode::Development, &genesis),
        Err(GenesisError::InvalidUpgrade)
    );
}
