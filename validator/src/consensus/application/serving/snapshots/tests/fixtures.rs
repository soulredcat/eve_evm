// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    development_snapshot_serving_budget, snapshot_message_limits::snapshot_message_limits,
};
use crate::consensus::application::ConsensusApplication;
use eve_consensus_comet::wire::tendermint::abci::{RequestQuery, ResponseQuery};
use eve_state::StateVersion;
use eve_storage::checkpoints::messages::{
    CheckpointRequest, CheckpointRequestKind, CheckpointResponse, decode_checkpoint_response,
    encode_checkpoint_request, preflight_checkpoint_response,
    required_checkpoint_response_decode_reservation,
};
pub(super) fn request(
    genesis: &StateVersion,
    height: u64,
    kind: CheckpointRequestKind,
) -> RequestQuery {
    RequestQuery {
        path: "/eve/recovery/v1/checkpoint".into(),
        data: encode_checkpoint_request(&CheckpointRequest {
            genesis: genesis.clone(),
            height,
            kind,
        })
        .unwrap(),
        ..Default::default()
    }
}
pub(super) fn decode(
    application: &ConsensusApplication,
    response: &ResponseQuery,
) -> CheckpointResponse {
    assert_eq!(response.code, 0, "{}", response.log);
    let limits = snapshot_message_limits(
        application.config.logical_budget,
        development_snapshot_serving_budget(),
    );
    let preflight = preflight_checkpoint_response(&response.value, &limits).unwrap();
    let required = required_checkpoint_response_decode_reservation(&preflight).unwrap();
    decode_checkpoint_response(&preflight, required).unwrap()
}
