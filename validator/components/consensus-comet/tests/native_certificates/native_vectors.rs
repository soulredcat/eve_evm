// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::native_support::{bytes, commit, header, load_fixture_cases, validators};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{
        CertificateError, HistoricalValidatorSet, hash_consensus_header, hash_validator_set,
        validator_address, verify_commit_certificate, verify_native_ed25519_signature,
    },
};

#[test]
fn validator_hashes_and_addresses_match_independent_pinned_go_sets() {
    let cases = load_fixture_cases("sets", "validator_sets");
    assert_eq!(cases.len(), 6);
    for case in cases {
        let set = validators(&case["validators"]);
        assert_eq!(
            hash_validator_set(&set).unwrap().as_slice(),
            bytes(&case["hash"])
        );
        for (validator, address) in set.iter().zip(case["addresses"].as_array().unwrap()) {
            assert_eq!(
                validator_address(&validator.public_key).as_slice(),
                bytes(address)
            );
        }
    }
}

#[test]
fn header_hashes_match_independent_pinned_native_field_trees() {
    let cases = load_fixture_cases("headers", "header_cases");
    assert!(cases.len() >= 2);
    for case in cases {
        let candidate = header(&case["header"]);
        if candidate.version.as_ref().unwrap().block != 11 {
            assert_eq!(
                hash_consensus_header(&candidate),
                Err(CertificateError::InvalidHeaderVersion)
            );
        } else {
            assert_eq!(
                hash_consensus_header(&candidate).unwrap().as_slice(),
                bytes(&case["hash"]),
                "{}",
                case["name"]
            );
        }
    }
}

#[test]
fn native_certificates_accept_three_of_four_and_reject_exact_two_thirds() {
    let cases = load_fixture_cases("certificates", "certificates");
    assert_eq!(cases.len(), 3);
    for case in cases {
        let header = header(&case["header"]);
        let commit = commit(&case["commit"]);
        let set = HistoricalValidatorSet {
            height: header.height,
            authentication: ConsensusAuthenticationRequirement::ClassicalDev,
            validators: validators(&case["validators"]),
        };
        let result = verify_commit_certificate(
            &header.chain_id,
            header.height,
            commit.round,
            commit.block_id.as_ref().unwrap(),
            &header,
            &commit,
            &set,
        );
        if case["native_outcome"] == "PASS" {
            assert!(result.is_ok(), "{result:?}");
        } else {
            assert_eq!(result, Err(CertificateError::InsufficientVotingPower));
        }
    }
}

#[test]
fn zip215_edges_match_independent_pinned_native_verifier() {
    let cases = load_fixture_cases("", "zip215_cases");
    assert_eq!(cases.len(), 12);
    for case in cases {
        let key: [u8; 32] = bytes(&case["public_key"]).try_into().unwrap();
        let result = verify_native_ed25519_signature(
            &key,
            &bytes(&case["message"]),
            &bytes(&case["signature"]),
        );
        assert_eq!(
            result.is_ok(),
            case["comet_zip215_accepts"].as_bool().unwrap(),
            "{}",
            case["name"]
        );
    }
}

#[test]
fn admitted_mixed_order_zip215_case_proves_strict_dalek_is_not_native_verifier() {
    let case = load_fixture_cases("", "zip215_cases")
        .into_iter()
        .find(|case| case["name"] == "upstream_speccheck_9")
        .unwrap();
    let key: [u8; 32] = bytes(&case["public_key"]).try_into().unwrap();
    let dalek = ed25519_dalek::VerifyingKey::from_bytes(&key).unwrap();
    assert!(!dalek.is_weak());
    let signature = bytes(&case["signature"]);
    let message = bytes(&case["message"]);
    assert!(
        dalek
            .verify_strict(
                &message,
                &ed25519_dalek::Signature::from_slice(&signature).unwrap()
            )
            .is_err()
    );
    assert!(verify_native_ed25519_signature(&key, &message, &signature).is_ok());
}
