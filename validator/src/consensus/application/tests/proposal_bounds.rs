// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    context::process_request,
    fixture::{test_application, transaction},
};
use crate::consensus::{
    application::application_info,
    transport::{
        dispatch::dispatch_application_request,
        peer::{EngineChannel, tests::spawn_authenticated_test_peer},
    },
};
use eve_consensus_comet::wire::tendermint::abci::{
    Request, request::Value as RequestValue, response::Value as ResponseValue,
};

#[test]
fn byzantine_proposal_bounds_reject_without_fencing_or_changing_state() {
    let mut fixture = test_application(false);
    let peer = spawn_authenticated_test_peer(EngineChannel::Application);
    let initial = application_info(&fixture.application).unwrap();
    for transactions in [
        vec![vec![]; 30_000_000 / 21_000 + 1],
        vec![vec![0; 131_073]],
        vec![vec![0; 131_072]; 33],
    ] {
        let request = process_request(&fixture.application, 1, transactions);
        let response = dispatch_application_request(
            &mut fixture.application,
            &peer.peer,
            Request {
                value: Some(RequestValue::ProcessProposal(request)),
            },
        )
        .unwrap();
        assert!(
            matches!(response.value, Some(ResponseValue::ProcessProposal(response)) if response.status == 2)
        );
        assert_eq!(application_info(&fixture.application).unwrap(), initial);
    }
    let request = process_request(&fixture.application, 1, vec![transaction()]);
    let response = dispatch_application_request(
        &mut fixture.application,
        &peer.peer,
        Request {
            value: Some(RequestValue::ProcessProposal(request)),
        },
    )
    .unwrap();
    assert!(
        matches!(response.value, Some(ResponseValue::ProcessProposal(response)) if response.status == 1)
    );
    assert_eq!(application_info(&fixture.application).unwrap(), initial);
}
