// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{advance_state, temporary_fixture, test_key, vote_request};
use crate::consensus::signing::{open_durable_signer, sign_vote, signer_status};
use eve_consensus_comet::consensus::{
    certificates::verify_native_ed25519_signature, signing::encode_vote_sign_bytes,
};
use eve_storage::records::development_opaque_record_budget;

#[test]
fn nil_vote_syncs_before_release_and_native_signature_verifies() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let vote = sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
    let bytes = encode_vote_sign_bytes(&fixture.config.chain_id, &vote).unwrap();
    assert!(
        verify_native_ed25519_signature(
            &fixture.config.expected_public_key,
            &bytes,
            &vote.signature
        )
        .is_ok()
    );
    assert_eq!(signer_status(&signer).cursor.sequence, 1);
    assert_eq!(signer_status(&signer).last_height, Some(1));
    assert_eq!(signer_status(&signer).last_round, Some(0));
    assert_eq!(signer_status(&signer).last_step, Some(2));
}

#[test]
fn durable_timestamp_retry_precedes_advanced_application_height_guard() {
    let (_directory, mut fixture) = temporary_fixture();
    let path = fixture.root.join("signing");
    let mut signer = open_durable_signer(
        &path,
        fixture.config.clone(),
        test_key(1),
        fixture.service.clone(),
        development_opaque_record_budget(),
    )
    .unwrap();
    let first = sign_vote(&mut signer, vote_request(&fixture.config), None).unwrap();
    drop(signer);
    advance_state(&mut fixture);
    let mut reopened = open_durable_signer(
        &path,
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let mut retry = vote_request(&fixture.config);
    retry.timestamp.as_mut().unwrap().seconds += 10;
    let replay = sign_vote(&mut reopened, retry, None).unwrap();
    assert_eq!(replay, first);
    assert_eq!(signer_status(&reopened).cursor.sequence, 1);
}

#[test]
fn lower_height_round_step_and_conflicting_same_hrs_reject() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let mut first = vote_request(&fixture.config);
    first.round = 2;
    sign_vote(&mut signer, first.clone(), None).unwrap();
    let mut regression = first.clone();
    regression.round = 1;
    assert!(sign_vote(&mut signer, regression, None).is_err());
    let mut conflict = first.clone();
    conflict.block_id = Some(eve_consensus_comet::wire::tendermint::types::BlockId {
        hash: vec![9; 32],
        part_set_header: Some(
            eve_consensus_comet::wire::tendermint::types::PartSetHeader {
                total: 1,
                hash: vec![8; 32],
            },
        ),
    });
    assert!(sign_vote(&mut signer, conflict, None).is_err());
    first.r#type = 2;
    sign_vote(&mut signer, first.clone(), None).unwrap();
    first.r#type = 1;
    assert!(sign_vote(&mut signer, first, None).is_err());
    assert_eq!(signer_status(&signer).cursor.sequence, 2);
}
