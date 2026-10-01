// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{fixture, rebind};
use crate::development::acceptance::enabled::{
    acceptance_key_enrolled, decode_acceptance_key::decode_acceptance_key,
    extend_acceptance_proposer_owners, load_acceptance_fixture,
    validate_acceptance_proposer_owners,
};
use alloy_primitives::Address;
use eve_consensus_comet::consensus::certificates::{ClassicalValidator, validator_address};
use std::collections::BTreeMap;

#[test]
fn future_key_enrollment_extends_only_exact_canonical_owner_and_rejects_collision() {
    let material = fixture();
    let capability = material.load();
    let key = *capability.future.keys().next().unwrap();
    assert!(acceptance_key_enrolled(Some(&capability), &key));
    assert!(!acceptance_key_enrolled(None, &key));
    assert!(!acceptance_key_enrolled(Some(&capability), &[9; 32]));
    let mut owners = BTreeMap::new();
    extend_acceptance_proposer_owners(Some(&capability), &mut owners).unwrap();
    assert_eq!(owners[&validator_address(&key)], Address::repeat_byte(1));
    assert!(extend_acceptance_proposer_owners(Some(&capability), &mut owners).is_err());
}

#[test]
fn weak_invalid_initial_duplicate_and_unbacked_future_keys_are_rejected() {
    let mut identity = [0; 32];
    identity[0] = 1;
    assert!(decode_acceptance_key(&hex::encode(identity)).is_err());
    assert!(decode_acceptance_key(&hex::encode([2; 32])).is_err());
    for mutation in 0..3 {
        let mut material = fixture();
        match mutation {
            0 => {
                material.document["future_validators"][0]["public_key"] =
                    hex::encode(material.genesis.validators[0].classical_public_key).into()
            }
            1 => {
                material.document["future_validators"][0]["owner"] =
                    serde_json::to_value(Address::repeat_byte(9)).unwrap()
            }
            _ => {
                let duplicate = material.document["future_validators"][0].clone();
                material.document["future_validators"]
                    .as_array_mut()
                    .unwrap()
                    .push(duplicate);
            }
        }
        rebind(&mut material);
        assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
    }
}

#[test]
fn proposer_map_accepts_only_exact_sealed_future_extension_of_verified_genesis() {
    let material = fixture();
    let capability = material.load();
    let initial = material
        .genesis
        .validators
        .iter()
        .map(|validator| ClassicalValidator {
            public_key: validator.classical_public_key,
            voting_power: i64::try_from(validator.voting_power).unwrap(),
        })
        .collect::<Vec<_>>();
    // The application first verifies these four mappings against canonical genesis records.
    let genesis_owners: BTreeMap<_, _> = material
        .genesis
        .validators
        .iter()
        .map(|validator| {
            (
                validator_address(&validator.classical_public_key),
                validator.owner,
            )
        })
        .collect();
    validate_acceptance_proposer_owners(None, &initial, &genesis_owners).unwrap();
    assert!(
        validate_acceptance_proposer_owners(Some(&capability), &initial, &genesis_owners).is_err()
    );
    let mut owners = genesis_owners.clone();
    extend_acceptance_proposer_owners(Some(&capability), &mut owners).unwrap();
    validate_acceptance_proposer_owners(Some(&capability), &initial, &owners).unwrap();
    assert!(validate_acceptance_proposer_owners(None, &initial, &owners).is_err());
    let future_key = *capability.future.keys().next().unwrap();
    let future_address = validator_address(&future_key);
    let mut missing = owners.clone();
    missing.remove(&future_address);
    assert!(validate_acceptance_proposer_owners(Some(&capability), &initial, &missing).is_err());
    let mut wrong = owners.clone();
    wrong.insert(future_address, Address::repeat_byte(9));
    assert!(validate_acceptance_proposer_owners(Some(&capability), &initial, &wrong).is_err());
    let mut extra = owners.clone();
    extra.insert([9; 20], Address::repeat_byte(9));
    assert!(validate_acceptance_proposer_owners(Some(&capability), &initial, &extra).is_err());
    let mut replacement = owners;
    replacement.remove(&future_address);
    replacement.insert([9; 20], Address::repeat_byte(9));
    assert_eq!(replacement.len(), initial.len() + 1);
    assert!(
        validate_acceptance_proposer_owners(Some(&capability), &initial, &replacement).is_err()
    );
}
