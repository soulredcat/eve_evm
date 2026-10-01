// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{fixture, verify};
use eve_consensus_comet::consensus::certificates::CertificateError;

#[test]
fn rejects_duplicate_unknown_reordered_and_wrong_count_signatures() {
    let (header, commit, set) = fixture("three_of_four_absent");
    let mut duplicate = commit.clone();
    duplicate.signatures[1] = duplicate.signatures[0].clone();
    assert_eq!(
        verify(&header, &duplicate, &set),
        Err(CertificateError::InvalidSignatureLayout)
    );
    let mut unknown = commit.clone();
    unknown.signatures[0].validator_address = vec![0; 20];
    assert_eq!(
        verify(&header, &unknown, &set),
        Err(CertificateError::InvalidSignatureLayout)
    );
    let mut reordered = commit.clone();
    reordered.signatures.swap(0, 1);
    assert_eq!(
        verify(&header, &reordered, &set),
        Err(CertificateError::InvalidSignatureLayout)
    );
    let mut missing = commit.clone();
    missing.signatures.pop();
    assert_eq!(
        verify(&header, &missing, &set),
        Err(CertificateError::InvalidSignatureLayout)
    );
    let mut extra = commit.clone();
    extra.signatures.push(extra.signatures[0].clone());
    assert_eq!(
        verify(&header, &extra, &set),
        Err(CertificateError::InvalidSignatureLayout)
    );
}

#[test]
fn verifies_nil_signature_even_after_quorum_power_is_already_met() {
    let (header, mut commit, set) = fixture("three_of_four_nil");
    assert!(verify(&header, &commit, &set).is_ok());
    let nil = commit
        .signatures
        .iter_mut()
        .find(|signature| signature.block_id_flag == 3)
        .unwrap();
    nil.signature.fill(0);
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::InvalidSignature)
    );
}

#[test]
fn rejects_absent_payload_invalid_flags_bad_lengths_and_changed_timestamps() {
    let (header, commit, set) = fixture("three_of_four_absent");
    let mut absent = commit.clone();
    absent.signatures[3].timestamp = Some(prost_types::Timestamp {
        seconds: 0,
        nanos: 0,
    });
    assert_eq!(
        verify(&header, &absent, &set),
        Err(CertificateError::InvalidSignatureLayout)
    );
    for flag in [0, 4, i32::MAX] {
        let mut candidate = commit.clone();
        candidate.signatures[0].block_id_flag = flag;
        assert_eq!(
            verify(&header, &candidate, &set),
            Err(CertificateError::InvalidSignatureLayout)
        );
    }
    for size in [0, 63, 65, 4096] {
        let mut candidate = commit.clone();
        candidate.signatures[0].signature = vec![1; size];
        assert_eq!(
            verify(&header, &candidate, &set),
            Err(CertificateError::InvalidSignatureLayout)
        );
    }
    let mut altered = commit.clone();
    altered.signatures[0].timestamp.as_mut().unwrap().nanos += 1;
    assert_eq!(
        verify(&header, &altered, &set),
        Err(CertificateError::InvalidSignature)
    );
}

#[test]
fn nil_votes_never_contribute_to_block_quorum_power() {
    let (header, commit, set) = fixture("three_of_four_nil");
    let proof = verify(&header, &commit, &set).unwrap();
    assert_eq!(
        (proof.signed_voting_power, proof.total_voting_power),
        (30, 40)
    );
    let (header, commit, set) = fixture("exact_two_of_three_rejected");
    assert_eq!(
        verify(&header, &commit, &set),
        Err(CertificateError::InsufficientVotingPower)
    );
}
