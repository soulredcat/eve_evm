// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::fixture;
use eve_consensus_comet::consensus::certificates::{
    CertificateError, canonicalize_validator_set, hash_consensus_header, hash_validator_set,
};

#[test]
fn rejects_duplicate_unordered_sets_and_power_overflow_without_enrollment_policy() {
    let (_, _, set) = fixture("three_of_four_absent");
    let mut duplicate = set.validators.clone();
    duplicate[1] = duplicate[0];
    assert_eq!(
        hash_validator_set(&duplicate),
        Err(CertificateError::InvalidValidatorSet)
    );
    let mut weak = set.validators.clone();
    weak[0].public_key = [0; 32];
    assert!(hash_validator_set(&canonicalize_validator_set(&weak).unwrap()).is_ok());
    let mut unordered = set.validators.clone();
    unordered.swap(0, 1);
    assert_eq!(
        hash_validator_set(&unordered),
        Err(CertificateError::InvalidValidatorSet)
    );
    assert_eq!(
        canonicalize_validator_set(&unordered).unwrap(),
        set.validators
    );
    let mut overflowing = set.validators.clone();
    overflowing[0].voting_power = i64::MAX / 8;
    assert_eq!(
        hash_validator_set(&overflowing),
        Err(CertificateError::VotingPowerOverflow)
    );
    assert_eq!(
        hash_validator_set(&[]),
        Err(CertificateError::InvalidValidatorSet)
    );
    let oversized = vec![set.validators[0]; 65];
    assert_eq!(
        hash_validator_set(&oversized),
        Err(CertificateError::InvalidValidatorSet)
    );
    for power in [0, -1, i64::MIN] {
        let mut invalid = set.validators.clone();
        invalid[0].voting_power = power;
        assert_eq!(
            hash_validator_set(&invalid),
            Err(CertificateError::InvalidValidatorSet)
        );
    }
}

#[test]
fn rejects_malformed_header_fields_without_partial_hash_acceptance() {
    let (header, _, _) = fixture("three_of_four_absent");
    let mut wrong_version = header.clone();
    wrong_version.version.as_mut().unwrap().block = 1;
    assert_eq!(
        hash_consensus_header(&wrong_version),
        Err(CertificateError::InvalidHeaderVersion)
    );
    for size in [0, 31, 33, 4096] {
        let mut invalid = header.clone();
        invalid.validators_hash = vec![0; size];
        assert_eq!(
            hash_consensus_header(&invalid),
            Err(CertificateError::InvalidHeaderHash)
        );
    }
    let mut invalid = header.clone();
    invalid.proposer_address = vec![0; 19];
    assert_eq!(
        hash_consensus_header(&invalid),
        Err(CertificateError::InvalidHeaderHash)
    );
    let mut invalid = header.clone();
    invalid.app_hash = vec![0; 1025];
    assert_eq!(
        hash_consensus_header(&invalid),
        Err(CertificateError::InvalidHeaderHash)
    );
}

#[test]
fn native_hashing_is_distinct_from_point_decoding_and_eve_enrollment() {
    use crate::native_support::{bytes, load_fixture_cases, validators};
    use eve_consensus_comet::consensus::certificates::verify_native_ed25519_signature;
    let cases = load_fixture_cases("sets", "validator_sets");
    for name in [
        "weak_identity_native_hash",
        "invalid_curve_point_native_hash",
    ] {
        let case = cases.iter().find(|case| case["name"] == name).unwrap();
        let set = validators(&case["validators"]);
        assert_eq!(
            hash_validator_set(&set).unwrap().as_slice(),
            bytes(&case["hash"])
        );
        if name == "invalid_curve_point_native_hash" {
            assert_eq!(
                verify_native_ed25519_signature(&set[0].public_key, b"bounded message", &[0; 64]),
                Err(CertificateError::InvalidPublicKey)
            );
        }
    }
}
