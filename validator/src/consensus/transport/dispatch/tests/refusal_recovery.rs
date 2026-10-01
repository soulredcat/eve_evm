// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    approval::{
        create_approval_registry, create_execution_approval, find_approval,
        pin_signed_vote_approval, publish_native_approval,
    },
    signing::{
        open_durable_signer, sign_vote, signer_status,
        tests::{temporary_fixture, test_key, vote_request},
    },
    transport::proposals::{EngineProposalBinding, fixture_verified_local_engine_proposal},
};
use alloy_primitives::Address;
use eve_consensus_comet::{
    consensus::certificates::validator_address,
    wire::tendermint::{
        abci::RequestProcessProposal,
        types::{BlockId, PartSetHeader},
    },
};
use eve_state::development_state_budget;
use eve_storage::records::development_opaque_record_budget;
use std::sync::Arc;

#[test]
fn missing_retained_proposal_abstains_without_signing() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let before = signer_status(&signer).cursor;
    let mut vote = vote_request(&fixture.config);
    vote.block_id = Some(BlockId {
        hash: vec![7; 32],
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![8; 32],
        }),
    });
    let error = super::super::handle_signer_vote_request::handle_signer_vote_request(
        &mut signer,
        &create_approval_registry(),
        vote,
        &development_state_budget(),
        256 * 1024 * 1024,
    )
    .unwrap_err();
    let response =
        super::super::refuse_signing_request::refuse_signing_request(&signer, &before, error)
            .unwrap();
    assert_eq!(
        response.description,
        "non-nil vote requires canonical execution/data approval"
    );
    assert_eq!(signer_status(&signer).cursor, before);
    assert!(!signer_status(&signer).fenced);
}

#[test]
fn retained_reexecution_resource_failure_stays_fatal_without_signing() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service,
        development_opaque_record_budget(),
    )
    .unwrap();
    let registry = create_approval_registry();
    let budget = development_state_budget();
    // Synthetic local-provenance fixture; actual execution, retention and signing are exercised.
    for round in 0..3_i32 {
        let hash = [u8::try_from(round + 7).unwrap(); 32];
        let source = fixture_verified_local_engine_proposal(
            RequestProcessProposal {
                height: 1,
                hash: hash.to_vec(),
                time: Some(prost_types::Timestamp {
                    seconds: 1_728_000_001,
                    nanos: 123 + round,
                }),
                proposer_address: validator_address(&fixture.config.expected_public_key).to_vec(),
                ..Default::default()
            },
            EngineProposalBinding {
                genesis_hash: fixture.config.genesis_hash,
                chain_id: fixture.config.chain_id.clone(),
                authentication: fixture.config.authentication,
                key_epoch: fixture.config.key_epoch,
                proposer_owner: Address::repeat_byte(1),
                previous_consensus_hash: [0; 32],
            },
        );
        let approval = Arc::new(
            create_execution_approval(&signer, &source, &budget, 256 * 1024 * 1024).unwrap(),
        );
        publish_native_approval(&registry, &signer, source, approval.clone()).unwrap();
        let mut vote = vote_request(&fixture.config);
        vote.round = round;
        vote.block_id = Some(BlockId {
            hash: hash.to_vec(),
            part_set_header: Some(PartSetHeader {
                total: 1,
                hash: vec![8; 32],
            }),
        });
        let signed = sign_vote(&mut signer, vote.clone(), Some(&approval)).unwrap();
        pin_signed_vote_approval(&registry, &signer, &signed).unwrap();
        if round == 0 {
            vote.r#type = 2;
            let signed = sign_vote(&mut signer, vote, Some(&approval)).unwrap();
            pin_signed_vote_approval(&registry, &signer, &signed).unwrap();
        }
    }
    assert!(find_approval(&registry, &[7; 32]).unwrap().is_none());
    let before = signer_status(&signer).cursor;
    let mut vote = vote_request(&fixture.config);
    vote.round = 3;
    vote.block_id = Some(BlockId {
        hash: vec![7; 32],
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![8; 32],
        }),
    });
    let error = super::super::handle_signer_vote_request::handle_signer_vote_request(
        &mut signer,
        &registry,
        vote,
        &budget,
        0,
    )
    .unwrap_err();
    assert_eq!(error.to_string(), "retained proposal execution failed");
    let fatal =
        super::super::refuse_signing_request::refuse_signing_request(&signer, &before, error)
            .unwrap_err();
    assert_eq!(fatal.to_string(), "retained proposal execution failed");
    assert!(fatal.root_cause().to_string().contains("CloneReservation"));
    assert_eq!(signer_status(&signer).cursor, before);
    assert!(!signer_status(&signer).fenced);
}
