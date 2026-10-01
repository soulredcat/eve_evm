// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{temporary_fixture, test_key, vote_request};
use crate::consensus::signing::{
    open_durable_signer,
    records::{decode_record, encode_record},
    sign_vote,
};
use eve_consensus_comet::consensus::authentication::ConsensusAuthenticationRequirement;
use eve_storage::records::{
    compare_and_append_opaque_records, development_opaque_record_budget, read_opaque_record,
};

#[test]
fn wrong_key_chain_genesis_epoch_and_activated_hybrid_fail_closed() {
    let (_directory, fixture) = temporary_fixture();
    let path = fixture.root.join("signing");
    assert!(
        open_durable_signer(
            &path,
            fixture.config.clone(),
            test_key(2),
            fixture.service.clone(),
            development_opaque_record_budget()
        )
        .is_err()
    );
    for index in 0..4 {
        let mut config = fixture.config.clone();
        match index {
            0 => config.chain_id = "foreign-chain".into(),
            1 => config.genesis_hash[0] ^= 1,
            2 => config.key_epoch += 1,
            _ => config.authentication = ConsensusAuthenticationRequirement::ClassicalAndMldsa65,
        }
        assert!(
            open_durable_signer(
                &path,
                config,
                test_key(1),
                fixture.service.clone(),
                development_opaque_record_budget()
            )
            .is_err()
        );
    }
    assert!(!path.exists());
}

#[test]
fn authenticated_record_payload_signature_and_monotonic_metadata_are_revalidated() {
    let (_directory, fixture) = temporary_fixture();
    let path = fixture.root.join("signing");
    let mut signer = open_durable_signer(
        &path,
        fixture.config.clone(),
        test_key(1),
        fixture.service.clone(),
        development_opaque_record_budget(),
    )
    .unwrap();
    sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
    let stored = read_opaque_record(&signer.repository, 1).unwrap().unwrap();
    let mut record = decode_record(&stored.payload).unwrap();
    record.hrs.round += 1;
    compare_and_append_opaque_records(
        &mut signer.repository,
        signer.cursor,
        &[encode_record(&record).unwrap()],
    )
    .unwrap();
    drop(signer);
    assert!(
        open_durable_signer(
            &path,
            fixture.config,
            test_key(1),
            fixture.service,
            development_opaque_record_budget()
        )
        .is_err()
    );
}

#[test]
fn unknown_or_noncanonical_record_fields_are_rejected() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
    let stored = read_opaque_record(&signer.repository, 1).unwrap().unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&stored.payload).unwrap();
    value["private_seed"] = "untrusted-field".into();
    assert!(decode_record(&serde_json::to_vec(&value).unwrap()).is_err());
    assert!(
        decode_record(
            &serde_json::to_vec_pretty(&decode_record(&stored.payload).unwrap()).unwrap()
        )
        .is_err()
    );
    let record = decode_record(&stored.payload).unwrap();
    assert!(
        !String::from_utf8(encode_record(&record).unwrap())
            .unwrap()
            .contains("private_seed")
    );
}
