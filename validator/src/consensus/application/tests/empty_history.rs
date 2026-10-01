// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    context::{final_request, final_source, process_request, process_source},
    fixture::test_application,
};
use crate::consensus::{
    application::{
        application_info, application_proposal_binding, commit_application, finalize_block,
        process_proposal,
    },
    transport::peer::{EngineChannel, tests::spawn_authenticated_test_peer},
};
use eve_storage::state::read_state_service;

#[test]
fn empty_blocks_retain_real_preceding_consensus_hash_and_historical_replay_head() {
    let mut fixture = test_application(false);
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    let first = final_request(&process_request(&fixture.application, 1, Vec::new()));
    for height in 1..=2 {
        let decision = final_request(&process_request(&fixture.application, height, Vec::new()));
        let source = final_source(&fixture.application, &decision);
        let response = finalize_block(&mut fixture.application, source, &decision).unwrap();
        assert!(response.tx_results.is_empty());
        commit_application(&mut fixture.application, &peer.peer).unwrap();
        let head = read_state_service(&fixture.application.service).unwrap();
        assert_eq!(head.commit().block.header.gas_used, 0);
        let expected = if height == 1 { [0; 32] } else { [1; 32] };
        assert_eq!(head.commit().block.header.mix_hash.as_slice(), expected);
    }
    let binding =
        application_proposal_binding(&fixture.application, &first.proposer_address).unwrap();
    assert_eq!(binding.previous_consensus_hash, [2; 32]);
    let before = read_state_service(&fixture.application.service)
        .unwrap()
        .commit()
        .target
        .clone();
    let source = final_source(&fixture.application, &first);
    finalize_block(&mut fixture.application, source, &first).unwrap();
    commit_application(&mut fixture.application, &peer.peer).unwrap();
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        2
    );
    assert_eq!(
        read_state_service(&fixture.application.service)
            .unwrap()
            .commit()
            .target,
        before
    );
}

#[test]
fn same_native_hash_with_changed_environment_cannot_reuse_cached_execution() {
    let mut fixture = test_application(false);
    let original = process_request(&fixture.application, 1, Vec::new());
    let source = process_source(&fixture.application, original.clone());
    assert_eq!(
        process_proposal(&mut fixture.application, source)
            .unwrap()
            .status,
        1
    );
    let mut altered = original;
    altered.time.as_mut().unwrap().seconds += 1;
    let source = process_source(&fixture.application, altered);
    assert!(process_proposal(&mut fixture.application, source).is_err());
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        0
    );
}
