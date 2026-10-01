// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{fixture, verify};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{CertificateError, verify_commit_certificate},
    commitments::{ExecutionHeight, match_next_header_commitment},
};

#[test]
fn rejects_wrong_chain_height_round_and_applicable_set_height() {
    let (mut header, mut commit, mut set) = fixture("three_of_four_absent");
    header.chain_id = "other-chain".into();
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongChain)
    );
    header.chain_id = "eve-local-v1".into();
    header.height = 4;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongHeight)
    );
    header.height = 3;
    commit.height = 4;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongHeight)
    );
    commit.height = 3;
    set.height = 4;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongHeight)
    );
    set.height = 3;
    commit.round = 3;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongRound)
    );
}

#[test]
fn rejects_wrong_block_parts_forged_header_and_wrong_validator_set() {
    let (mut header, mut commit, mut set) = fixture("three_of_four_absent");
    let mut expected = commit.block_id.clone().unwrap();
    expected.part_set_header.as_mut().unwrap().total += 1;
    assert_eq!(
        verify_commit_certificate("eve-local-v1", 3, 2, &expected, &header, &commit, &set),
        Err(CertificateError::WrongBlock)
    );
    header.app_hash[0] ^= 1;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongBlock)
    );
    header.app_hash[0] ^= 1;
    set.validators[0].voting_power += 1;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongValidatorSet)
    );
    set.validators[0].voting_power -= 1;
    commit.block_id.as_mut().unwrap().hash[0] ^= 1;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::WrongBlock)
    );
}

#[test]
fn rejects_activated_hybrid_instead_of_accepting_classical_certificate() {
    let (header, commit, mut set) = fixture("three_of_four_absent");
    assert!(verify(&header, &commit, &set).is_ok());
    set.authentication = ConsensusAuthenticationRequirement::ClassicalAndMldsa65;
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::UnsupportedAuthentication)
    );
}

#[test]
fn native_certificate_authentication_stays_distinct_from_h_plus_one_binding() {
    let (header, commit, set) = fixture("three_of_four_absent");
    assert!(verify(&header, &commit, &set).is_ok());
    let commitment: [u8; 32] = header.app_hash.clone().try_into().unwrap();
    assert!(
        match_next_header_commitment("eve-local-v1", ExecutionHeight(2), &commitment, &header)
            .is_ok()
    );
    let mut forged = commitment;
    forged[0] ^= 1;
    assert!(
        match_next_header_commitment("eve-local-v1", ExecutionHeight(2), &forged, &header).is_err()
    );
    assert!(
        match_next_header_commitment("eve-local-v1", ExecutionHeight(3), &commitment, &header)
            .is_err()
    );
}
