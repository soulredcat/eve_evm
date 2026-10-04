// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use ed25519_dalek::SigningKey;
use eve_development_fixtures::genesis::{funded_genesis, genesis};
use eve_state::{
    Address, SecurityProfile, U256, development_state_budget, initialize_development_state,
    validate_state_commit,
};

#[test]
fn deterministic_genesis_preserves_enrolled_development_keys_and_initial_canonical_state() {
    let genesis = genesis();
    assert_eq!(genesis.network_name, "eve-local-v1");
    assert_eq!(genesis.evm_chain_id, 31_337);
    assert_eq!(genesis.initial_timestamp, 1_728_000_000);
    assert_eq!(genesis.profile, SecurityProfile::ClassicalDev);
    assert_eq!((genesis.accounts.len(), genesis.validators.len()), (4, 4));
    for (index, seed) in (1..=4_u8).enumerate() {
        let account = &genesis.accounts[index];
        let validator = &genesis.validators[index];
        assert_eq!(account.address, Address::repeat_byte(seed));
        assert_eq!(
            account.funded_balance,
            genesis.economics.validator_self_bond * U256::from(10)
        );
        assert_eq!(validator.owner, account.address);
        assert_eq!(
            validator.classical_public_key,
            SigningKey::from_bytes(&[seed; 32])
                .verifying_key()
                .to_bytes()
        );
        assert_eq!(
            (validator.self_bond, validator.voting_power),
            (genesis.economics.validator_self_bond, 10_000)
        );
    }
    let commit = initialize_development_state(&genesis, &development_state_budget()).unwrap();
    assert_eq!(commit.target.height, 0);
    assert_eq!(commit.target.timestamp, genesis.initial_timestamp);
    validate_state_commit(&commit, &development_state_budget()).unwrap();
    let funded = funded_genesis();
    assert_eq!(&funded.accounts[..4], genesis.accounts.as_slice());
    assert_eq!(funded.validators, genesis.validators);
    assert_eq!(funded.accounts.len(), 6);
    validate_state_commit(
        &initialize_development_state(&funded, &development_state_budget()).unwrap(),
        &development_state_budget(),
    )
    .unwrap();
}
