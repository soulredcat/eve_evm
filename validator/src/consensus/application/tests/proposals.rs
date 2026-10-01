// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    context::{final_request, final_source, process_request, process_source},
    fixture::{test_application, transaction},
};
use crate::consensus::{
    application::{
        application_info, check_transaction, commit_application, finalize_block, prepare_proposal,
        process_proposal,
    },
    transport::peer::{EngineChannel, tests::spawn_authenticated_test_peer},
};
use eve_consensus_comet::wire::tendermint::abci::{RequestCheckTx, RequestPrepareProposal};

#[test]
fn invalid_envelope_rejects_but_actual_evm_revert_remains_includable() {
    let mut fixture = test_application(true);
    let invalid = process_request(&fixture.application, 1, vec![vec![1, 2, 3]]);
    let source = process_source(&fixture.application, invalid);
    assert_eq!(
        process_proposal(&mut fixture.application, source)
            .unwrap()
            .status,
        2
    );
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
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
    let decision = final_request(&request);
    let source = final_source(&fixture.application, &decision);
    let result = finalize_block(&mut fixture.application, source, &decision).unwrap();
    assert_eq!(
        (result.tx_results[0].code, result.tx_results[0].gas_used),
        (0, 21_006)
    );
    let mut data = result.tx_results[0].data.as_slice();
    use alloy_eips::eip2718::Decodable2718;
    let receipt = alloy_consensus::ReceiptEnvelope::decode_2718(&mut data).unwrap();
    assert!(!receipt.status());
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    commit_application(&mut fixture.application, &peer.peer).unwrap();
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        1
    );
}

#[test]
fn prepare_filters_locally_without_mutation_process_accepts_supplied_data() {
    let mut fixture = test_application(false);
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    let request = process_request(&fixture.application, 1, vec![transaction()]);
    let prepare = RequestPrepareProposal {
        max_tx_bytes: 4_194_304,
        txs: vec![vec![1], transaction()],
        height: request.height,
        time: request.time,
        proposer_address: request.proposer_address.clone(),
        ..Default::default()
    };
    assert_eq!(
        prepare_proposal(&fixture.application, &peer.peer, &prepare)
            .unwrap()
            .txs,
        vec![transaction()]
    );
    assert_eq!(
        application_info(&fixture.application)
            .unwrap()
            .last_block_height,
        0
    );
    let small = RequestPrepareProposal {
        max_tx_bytes: 1,
        txs: vec![transaction()],
        ..prepare
    };
    assert!(
        prepare_proposal(&fixture.application, &peer.peer, &small)
            .unwrap()
            .txs
            .is_empty()
    );
    assert_eq!(
        check_transaction(
            &fixture.application,
            &peer.peer,
            &RequestCheckTx {
                tx: vec![1],
                r#type: 0
            }
        )
        .unwrap()
        .code,
        1
    );
    let source = process_source(&fixture.application, request);
    assert_eq!(
        process_proposal(&mut fixture.application, source)
            .unwrap()
            .status,
        1
    );
}
