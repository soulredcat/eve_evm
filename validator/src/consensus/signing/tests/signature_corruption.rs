// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{temporary_fixture, test_key, vote_request};
use crate::consensus::signing::{
    open_durable_signer,
    records::{decode_record, encode_record},
    sign_vote,
};
use ed25519_dalek::Signer;
use eve_consensus_comet::consensus::signing::encode_vote_sign_bytes;
use eve_storage::records::{
    compare_and_append_opaque_records, development_opaque_record_budget, read_opaque_record,
};

#[test]
fn rehashing_otherwise_valid_history_cannot_hide_a_corrupted_signature() {
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
    record.hrs.round = 1;
    let mut request = vote_request(&fixture.config);
    request.round = 1;
    record.sign_bytes = encode_vote_sign_bytes(&fixture.config.chain_id, &request).unwrap();
    record.signature = test_key(1).sign(&record.sign_bytes).to_bytes().to_vec();
    record.signature[0] ^= 1;
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
