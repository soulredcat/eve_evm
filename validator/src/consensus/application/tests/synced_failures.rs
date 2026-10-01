// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    context::{final_request, final_source, process_request},
    fixture::{reopen_application, test_application, transaction},
};
use crate::consensus::{
    application::{
        application_info, commit_application, finalize_block, types::SimulatedApplicationFailure,
    },
    signing::signer_status,
    transport::peer::{EngineChannel, tests::spawn_authenticated_test_peer},
};
use eve_storage::state::read_state_service;

#[test]
fn actual_state_sync_without_completion_marker_reconciles_without_duplicate_fee() {
    let fixture = test_application(false);
    let root = fixture.root;
    let genesis = fixture.genesis;
    let spec = fixture.spec;
    let mut application = fixture.application;
    let decision = final_request(&process_request(&application, 1, vec![transaction()]));
    let source = final_source(&application, &decision);
    finalize_block(&mut application, source, &decision).unwrap();
    application.simulated_failure = Some(SimulatedApplicationFailure::StateSynced);
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    assert!(commit_application(&mut application, &peer.peer).is_err());
    assert!(application_info(&application).is_err());
    assert!(signer_status(&application.signer.lock().unwrap()).fenced);
    assert_eq!(
        read_state_service(&application.service)
            .unwrap()
            .commit()
            .target
            .height,
        1
    );
    drop(application);
    let mut reopened = reopen_application(root.path(), genesis, &spec);
    assert_eq!(application_info(&reopened).unwrap().last_block_height, 1);
    let before = read_state_service(&reopened.service)
        .unwrap()
        .commit()
        .target
        .clone();
    let source = final_source(&reopened, &decision);
    finalize_block(&mut reopened, source, &decision).unwrap();
    commit_application(&mut reopened, &peer.peer).unwrap();
    assert_eq!(
        read_state_service(&reopened.service)
            .unwrap()
            .commit()
            .target,
        before
    );
}

#[test]
fn pre_sync_failure_never_advances_and_post_completion_lost_ack_is_exact_replay() {
    for failure in [
        SimulatedApplicationFailure::BeforeState,
        SimulatedApplicationFailure::MetadataSynced,
    ] {
        let fixture = test_application(false);
        let root = fixture.root;
        let genesis = fixture.genesis;
        let spec = fixture.spec;
        let mut application = fixture.application;
        let decision = final_request(&process_request(&application, 1, vec![transaction()]));
        let source = final_source(&application, &decision);
        finalize_block(&mut application, source, &decision).unwrap();
        application.simulated_failure = Some(failure);
        let peer = spawn_authenticated_test_peer(EngineChannel::Application);
        assert!(commit_application(&mut application, &peer.peer).is_err());
        let expected = if failure == SimulatedApplicationFailure::BeforeState {
            0
        } else {
            1
        };
        assert_eq!(
            read_state_service(&application.service)
                .unwrap()
                .commit()
                .target
                .height,
            expected
        );
        drop(application);
        let mut reopened = reopen_application(root.path(), genesis, &spec);
        assert_eq!(
            application_info(&reopened).unwrap().last_block_height,
            i64::try_from(expected).unwrap()
        );
        let source = final_source(&reopened, &decision);
        finalize_block(&mut reopened, source, &decision).unwrap();
        commit_application(&mut reopened, &peer.peer).unwrap();
        assert_eq!(application_info(&reopened).unwrap().last_block_height, 1);
        let head = read_state_service(&reopened.service).unwrap();
        let sender = "4a62316623ad457f02cdc5d997ded67a383ec569"
            .parse::<alloy_primitives::Address>()
            .unwrap();
        assert_eq!(head.commit().state.accounts[&sender].nonce, 1);
    }
}
