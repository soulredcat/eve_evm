// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{fixture, rebind};
use crate::development::{
    acceptance::enabled::{decode_acceptance_key::decode_acceptance_key, load_acceptance_fixture},
    config::validate_development_validator_keys,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use eve_protocol_config::{
    genesis::{validate_classical_enrollment_key, validate_development_genesis},
    network::LaunchMode,
};

fn upstream_mixed_order_key() -> [u8; 32] {
    let source = include_str!(
        "../../../../../components/consensus-comet/tests/fixtures/zip215-upstream/zip215_cases-upstream_speccheck_9.json"
    );
    let vector: serde_json::Value = serde_json::from_str(source).unwrap();
    let case = &vector["zip215_cases"][0];
    assert_eq!(case["comet_zip215_accepts"], true);
    assert_eq!(case["public_key_canonical"], true);
    assert_eq!(case["public_key_small_order"], false);
    assert_eq!(case["public_key_torsion_free"], false);
    hex::decode(case["public_key"].as_str().unwrap())
        .unwrap()
        .try_into()
        .unwrap()
}

#[test]
fn canonical_native_mixed_order_key_is_rejected_by_genesis_and_runtime_enrollment() {
    let key = upstream_mixed_order_key();
    let verification = VerifyingKey::from_bytes(&key).unwrap();
    assert!(!verification.is_weak());
    assert!(!verification.to_edwards().is_torsion_free());
    assert_eq!(verification.to_edwards().compress().to_bytes(), key);
    assert!(validate_classical_enrollment_key(&key).is_err());
    let mut material = fixture();
    material.genesis.validators[0].classical_public_key = key;
    assert!(validate_development_genesis(LaunchMode::Development, &material.genesis).is_err());
    assert!(
        validate_development_validator_keys(
            &material.genesis,
            &SigningKey::from_bytes(&[2; 32]),
            None
        )
        .is_err()
    );
    assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
}

#[test]
fn future_mixed_order_key_cannot_bind_rotation_despite_native_zip215_compatibility() {
    let key = upstream_mixed_order_key();
    let mut material = fixture();
    let encoded = hex::encode(key);
    material.document["future_validators"][0]["public_key"] = encoded.clone().into();
    material.document["transitions"][0]["updates"][1]["public_key"] = encoded.clone().into();
    rebind(&mut material);
    assert!(decode_acceptance_key(&encoded).is_err());
    assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
}
