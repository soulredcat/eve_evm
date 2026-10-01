// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    context::{final_request, final_source, process_request, process_source},
    fixture::{test_application, transaction},
};
use crate::consensus::{
    application::{
        application_info, check_transaction, commit_application, finalize_block,
        initialize_application, process_proposal,
    },
    transport::peer::{EngineChannel, tests::spawn_authenticated_test_peer},
};
use eve_consensus_comet::wire::tendermint::abci::{RequestCheckTx, RequestInitChain};
use eve_storage::state::read_state_service;

#[test]
fn real_execution_lifecycle_publishes_only_after_synced_commit() {
    let mut fixture = test_application(false);
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    let info = application_info(&fixture.application).unwrap();
    assert_eq!(info.last_block_height, 0);
    assert_eq!(
        info.last_block_app_hash,
        fixture.genesis.target.content_digest.to_vec()
    );
    let config = &fixture.application.config;
    let init = RequestInitChain {
        initial_height: 1,
        chain_id: config.genesis.target.identity.network_name.clone(),
        time: Some(config.initial_time),
        consensus_params: Some(config.consensus_params.clone()),
        validators: config.initial_validators.clone(),
        app_state_bytes: serde_json::to_vec(&config.expected_app_state).unwrap(),
    };
    assert_eq!(
        initialize_application(&mut fixture.application, &peer.peer, &init)
            .unwrap()
            .app_hash,
        info.last_block_app_hash
    );
    assert_eq!(
        check_transaction(
            &fixture.application,
            &peer.peer,
            &RequestCheckTx {
                tx: transaction(),
                r#type: 0
            }
        )
        .unwrap()
        .code,
        0
    );
    let request = process_request(&fixture.application, 1, vec![transaction()]);
    let source = process_source(&fixture.application, request.clone());
    assert_eq!(
        process_proposal(&mut fixture.application, source)
            .unwrap()
            .status,
        1
    );
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        0
    );
    let final_request = final_request(&request);
    let source = final_source(&fixture.application, &final_request);
    let response = finalize_block(&mut fixture.application, source, &final_request).unwrap();
    assert_eq!(response.tx_results.len(), 1);
    assert_eq!(response.tx_results[0].code, 0);
    assert_eq!(response.tx_results[0].gas_used, 43_106);
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        0
    );
    commit_application(&mut fixture.application, &peer.peer).unwrap();
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        1
    );
    let head = read_state_service(&fixture.application.service).unwrap();
    assert_eq!(
        head.commit()
            .target
            .application
            .as_ref()
            .unwrap()
            .0
            .0
            .to_vec(),
        response.app_hash
    );
    assert_eq!(
        head.commit().state.accounts[&"4a62316623ad457f02cdc5d997ded67a383ec569"
            .parse::<alloy_primitives::Address>()
            .unwrap()]
            .nonce,
        1
    );
}

#[test]
fn wrong_init_chain_or_wrong_peer_channel_cannot_mutate_application() {
    let mut fixture = test_application(false);
    let signer_peer = spawn_authenticated_test_peer(EngineChannel::Signer);
    assert!(
        check_transaction(
            &fixture.application,
            &signer_peer.peer,
            &RequestCheckTx {
                tx: transaction(),
                r#type: 0
            }
        )
        .is_err()
    );
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    let config = &fixture.application.config;
    let init = RequestInitChain {
        initial_height: 2,
        chain_id: "wrong-chain".into(),
        time: Some(config.initial_time),
        consensus_params: Some(config.consensus_params.clone()),
        validators: config.initial_validators.clone(),
        app_state_bytes: serde_json::to_vec(&config.expected_app_state).unwrap(),
    };
    assert!(initialize_application(&mut fixture.application, &peer.peer, &init).is_err());
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        0
    );
}
