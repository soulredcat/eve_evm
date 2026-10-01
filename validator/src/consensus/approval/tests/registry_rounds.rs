// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::proposal_fixture;
use crate::consensus::{
    approval::{
        approval_registry_status, clear_after_synced_commit, create_approval_registry,
        create_execution_approval, find_approval, pin_signed_vote_approval,
        publish_native_approval, reconstruct_retained_approval,
    },
    signing::{
        open_durable_signer, sign_vote,
        tests::{advance_state, temporary_fixture, test_key, vote_request},
    },
    transport::proposals::fixture_verified_local_engine_proposal,
};
use eve_consensus_comet::wire::tendermint::types::{BlockId, PartSetHeader};
use eve_state::development_state_budget;
use eve_storage::records::development_opaque_record_budget;
use std::sync::Arc;

#[test]
fn more_than_two_fresh_unlocked_rounds_replace_speculation_and_preserve_precommit_raw() {
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
    for round in 0..7_i32 {
        let template = proposal_fixture(&fixture.config, Vec::new());
        let mut request = template.request().clone();
        request.hash = vec![u8::try_from(round + 7).unwrap(); 32];
        request.time.as_mut().unwrap().nanos += round;
        let binding = crate::consensus::transport::proposals::EngineProposalBinding {
            genesis_hash: fixture.config.genesis_hash,
            chain_id: fixture.config.chain_id.clone(),
            authentication: fixture.config.authentication,
            key_epoch: fixture.config.key_epoch,
            proposer_owner: template.binding().proposer_owner,
            previous_consensus_hash: [0; 32],
        };
        let source = fixture_verified_local_engine_proposal(request, binding);
        let approval = Arc::new(
            create_execution_approval(
                &signer,
                &source,
                &development_state_budget(),
                256 * 1024 * 1024,
            )
            .unwrap(),
        );
        let hash = [u8::try_from(round + 7).unwrap(); 32];
        publish_native_approval(&registry, &signer, source, approval).unwrap();
        let found = find_approval(&registry, &hash).unwrap().unwrap();
        let mut vote = vote_request(&fixture.config);
        vote.round = round;
        vote.block_id = Some(BlockId {
            hash: hash.to_vec(),
            part_set_header: Some(PartSetHeader {
                total: 1,
                hash: vec![8; 32],
            }),
        });
        let signed = sign_vote(&mut signer, vote.clone(), Some(&found)).unwrap();
        pin_signed_vote_approval(&registry, &signer, &signed).unwrap();
        if round == 0 {
            vote.r#type = 2;
            let signed = sign_vote(&mut signer, vote, Some(&found)).unwrap();
            pin_signed_vote_approval(&registry, &signer, &signed).unwrap();
        }
        let status: super::super::ApprovalRegistryStatus =
            approval_registry_status(&registry).unwrap();
        assert!(
            status.full <= 2
                && status.raw <= 4
                && status.retained_encoded_bytes <= 16 * 1024 * 1024
        );
        assert_eq!(status.current, Some(hash));
        assert_eq!(status.prevote_pin, Some(hash));
        assert_eq!(status.precommit_pin, Some([7; 32]));
    }
    assert!(find_approval(&registry, &[7; 32]).unwrap().is_none());
    let reconstructed = reconstruct_retained_approval(
        &registry,
        &signer,
        &[7; 32],
        &development_state_budget(),
        256 * 1024 * 1024,
    )
    .unwrap();
    assert_eq!(
        super::super::approval_request(&reconstructed).hash,
        vec![7; 32]
    );
    assert!(find_approval(&registry, &[7; 32]).unwrap().is_none());
}

#[test]
fn locked_round_reuse_retains_approval_until_actual_commit_and_restart_reexecutes() {
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
    let registry = create_approval_registry();
    let source = proposal_fixture(&fixture.config, Vec::new());
    let approval = Arc::new(
        create_execution_approval(
            &signer,
            &source,
            &development_state_budget(),
            256 * 1024 * 1024,
        )
        .unwrap(),
    );
    publish_native_approval(&registry, &signer, source, approval).unwrap();
    for round in 0..3 {
        let approval = find_approval(&registry, &[7; 32]).unwrap().unwrap();
        let mut vote = vote_request(&fixture.config);
        vote.round = round;
        vote.r#type = 2;
        vote.block_id = Some(BlockId {
            hash: vec![7; 32],
            part_set_header: Some(PartSetHeader {
                total: 1,
                hash: vec![8; 32],
            }),
        });
        let signed = sign_vote(&mut signer, vote, Some(&approval)).unwrap();
        pin_signed_vote_approval(&registry, &signer, &signed).unwrap();
    }
    clear_after_synced_commit(&registry, &signer).unwrap();
    assert_eq!(approval_registry_status(&registry).unwrap().full, 1);
    drop(signer);
    drop(registry);
    let mut signer = open_durable_signer(
        &path,
        fixture.config.clone(),
        test_key(1),
        fixture.service.clone(),
        development_opaque_record_budget(),
    )
    .unwrap();
    let fresh = create_approval_registry();
    assert!(find_approval(&fresh, &[7; 32]).unwrap().is_none());
    let mut vote = vote_request(&fixture.config);
    vote.round = 3;
    vote.block_id = Some(BlockId {
        hash: vec![7; 32],
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![8; 32],
        }),
    });
    assert!(sign_vote(&mut signer, vote.clone(), None).is_err());
    let source = proposal_fixture(&fixture.config, Vec::new());
    let approval = Arc::new(
        create_execution_approval(
            &signer,
            &source,
            &development_state_budget(),
            256 * 1024 * 1024,
        )
        .unwrap(),
    );
    publish_native_approval(&fresh, &signer, source, approval).unwrap();
    sign_vote(
        &mut signer,
        vote,
        find_approval(&fresh, &[7; 32]).unwrap().as_deref(),
    )
    .unwrap();
    advance_state(&mut fixture);
    clear_after_synced_commit(&fresh, &signer).unwrap();
    let status = approval_registry_status(&fresh).unwrap();
    assert_eq!(status.full, 0);
    assert_eq!(status.raw, 0);
}
