// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::proposal_fixture;
use crate::consensus::{
    approval::{approved_state_block, create_execution_approval},
    signing::{
        open_durable_signer, sign_vote, signer_status,
        tests::{advance_state, temporary_fixture, test_key, vote_request},
    },
};
use eve_consensus_comet::wire::tendermint::types::{BlockId, PartSetHeader};
use eve_state::development_state_budget;
use eve_storage::{records::development_opaque_record_budget, state::read_state_service};

#[test]
fn nonnil_vote_requires_execution_and_identical_block_approval_survives_rounds() {
    let (_directory, fixture) = temporary_fixture();
    let mut signer = open_durable_signer(
        &fixture.root.join("signing"),
        fixture.config.clone(),
        test_key(1),
        fixture.service.clone(),
        development_opaque_record_budget(),
    )
    .unwrap();
    let mut request = vote_request(&fixture.config);
    request.block_id = Some(BlockId {
        hash: vec![7; 32],
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![8; 32],
        }),
    });
    assert!(sign_vote(&mut signer, request.clone(), None).is_err());
    assert_eq!(signer_status(&signer).cursor.sequence, 0);
    let before = read_state_service(&fixture.service).unwrap();
    let approval = create_execution_approval(
        &signer,
        &proposal_fixture(&fixture.config, Vec::new()),
        &development_state_budget(),
        256 * 1024 * 1024,
    )
    .unwrap();
    assert_eq!(approved_state_block(&approval).commit.target.height, 1);
    assert!(std::sync::Arc::ptr_eq(
        &before,
        &read_state_service(&fixture.service).unwrap()
    ));
    sign_vote(&mut signer, request.clone(), Some(&approval)).unwrap();
    request.round += 1;
    sign_vote(&mut signer, request.clone(), Some(&approval)).unwrap();
    request.r#type = 2;
    sign_vote(&mut signer, request, Some(&approval)).unwrap();
    assert_eq!(signer_status(&signer).cursor.sequence, 3);
}

#[test]
fn wrong_block_stale_parent_and_unapproved_restart_refuse_new_nonnil_votes() {
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
    let approval = create_execution_approval(
        &signer,
        &proposal_fixture(&fixture.config, Vec::new()),
        &development_state_budget(),
        256 * 1024 * 1024,
    )
    .unwrap();
    let mut request = vote_request(&fixture.config);
    request.block_id = Some(BlockId {
        hash: vec![9; 32],
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![8; 32],
        }),
    });
    assert!(sign_vote(&mut signer, request.clone(), Some(&approval)).is_err());
    request.block_id.as_mut().unwrap().hash = vec![7; 32];
    drop(signer);
    let mut reopened = open_durable_signer(
        &path,
        fixture.config.clone(),
        test_key(1),
        fixture.service.clone(),
        development_opaque_record_budget(),
    )
    .unwrap();
    assert!(sign_vote(&mut reopened, request.clone(), None).is_err());
    advance_state(&mut fixture);
    assert!(sign_vote(&mut reopened, request, Some(&approval)).is_err());
    assert_eq!(signer_status(&reopened).cursor.sequence, 0);
}
