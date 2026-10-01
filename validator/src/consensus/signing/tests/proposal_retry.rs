// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{temporary_fixture, test_key};
use crate::consensus::signing::{open_durable_signer, sign_proposal, signer_status};
use eve_consensus_comet::wire::tendermint::types::{BlockId, PartSetHeader, Proposal};
use eve_storage::records::development_opaque_record_budget;

#[test]
fn proposal_retries_use_original_timestamp_and_require_no_vote_approval() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config,
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let request = Proposal {
        r#type: 32,
        height: 1,
        round: 0,
        pol_round: -1,
        block_id: Some(BlockId {
            hash: vec![1; 32],
            part_set_header: Some(PartSetHeader {
                total: 1,
                hash: vec![2; 32],
            }),
        }),
        timestamp: Some(prost_types::Timestamp {
            seconds: 1_728_000_001,
            nanos: 100,
        }),
        signature: Vec::new(),
    };
    let first = sign_proposal(&mut signer, request.clone()).unwrap();
    let mut retry = request.clone();
    retry.timestamp.as_mut().unwrap().nanos += 10;
    assert_eq!(sign_proposal(&mut signer, retry).unwrap(), first);
    let mut conflict = request;
    conflict
        .block_id
        .as_mut()
        .unwrap()
        .part_set_header
        .as_mut()
        .unwrap()
        .hash[0] ^= 1;
    assert!(sign_proposal(&mut signer, conflict).is_err());
    assert_eq!(signer_status(&signer).cursor.sequence, 1);
}

#[test]
fn future_proposal_and_wrong_native_fields_cannot_append_history() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config,
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let future = Proposal {
        r#type: 32,
        height: 2,
        round: 0,
        pol_round: -1,
        block_id: Some(BlockId {
            hash: vec![1; 32],
            part_set_header: Some(PartSetHeader {
                total: 1,
                hash: vec![2; 32],
            }),
        }),
        ..Default::default()
    };
    assert!(sign_proposal(&mut signer, future.clone()).is_err());
    let mut invalid = future;
    invalid.height = 1;
    invalid.pol_round = 0;
    assert!(sign_proposal(&mut signer, invalid).is_err());
    assert_eq!(signer_status(&signer).cursor.sequence, 0);
}
