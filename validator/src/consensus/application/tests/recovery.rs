// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    context::{final_request, final_source, process_request},
    fixture::{reopen_application, test_application, transaction},
};
use crate::consensus::{
    application::{
        application_info, application_proposal_binding, commit_application, finalize_block,
    },
    transport::peer::{EngineChannel, tests::spawn_authenticated_test_peer},
};
use eve_storage::state::read_state_service;

#[test]
fn pending_decision_restart_keeps_old_head_and_exact_replay_charges_once() {
    let fixture = test_application(false);
    let root = fixture.root;
    let genesis = fixture.genesis;
    let spec = fixture.spec;
    let mut application = fixture.application;
    let decision = final_request(&process_request(&application, 1, vec![transaction()]));
    let source = final_source(&application, &decision);
    finalize_block(&mut application, source, &decision).unwrap();
    assert_eq!(application_info(&application).unwrap().last_block_height, 0);
    drop(application);
    let mut application = reopen_application(root.path(), genesis, &spec);
    assert_eq!(application_info(&application).unwrap().last_block_height, 0);
    let source = final_source(&application, &decision);
    let expected = finalize_block(&mut application, source, &decision).unwrap();
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    commit_application(&mut application, &peer.peer).unwrap();
    let head = read_state_service(&application.service).unwrap();
    let target = head.commit().target.clone();
    assert_eq!(
        head.commit().state.accounts[&"4a62316623ad457f02cdc5d997ded67a383ec569"
            .parse::<alloy_primitives::Address>()
            .unwrap()]
            .nonce,
        1
    );
    let binding = application_proposal_binding(&application, &decision.proposer_address).unwrap();
    assert_eq!(binding.previous_consensus_hash, [1; 32]);
    let source = final_source(&application, &decision);
    assert_eq!(
        finalize_block(&mut application, source, &decision).unwrap(),
        expected
    );
    commit_application(&mut application, &peer.peer).unwrap();
    assert_eq!(
        read_state_service(&application.service)
            .unwrap()
            .commit()
            .target,
        target
    );
}

#[test]
fn changed_finalization_cannot_reuse_pending_or_committed_data() {
    let mut fixture = test_application(false);
    let decision = final_request(&process_request(&fixture.application, 1, Vec::new()));
    let source = final_source(&fixture.application, &decision);
    finalize_block(&mut fixture.application, source, &decision).unwrap();
    let mut changed = decision.clone();
    changed.hash[0] ^= 1;
    let source = final_source(&fixture.application, &changed);
    assert!(finalize_block(&mut fixture.application, source, &changed).is_err());
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        0
    );
}
