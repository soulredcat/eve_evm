// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::application::tests::fixture::TestApplication;
use eve_consensus_comet::wire::tendermint::abci::RequestQuery;
use eve_state::{
    BlockPayload, StateCommit, StateDeltaRequest, StateVersion, build_state_commit,
    encode_state_delta_request,
};
use eve_storage::state::{commit_state, read_state_service};

/// Actual durable canonical storage fixture; no native finality/execution claim.
pub(super) fn advance(fixture: &mut TestApplication) -> StateCommit {
    let parent = read_state_service(&fixture.application.service).unwrap();
    let mut header = parent.commit().block.header.clone();
    header.number += 1;
    header.timestamp += 1;
    header.parent_hash = parent.commit().target.execution_hash.0;
    let commit = build_state_commit(
        Some(parent.commit().target.clone()),
        parent.commit().state.clone(),
        BlockPayload {
            header,
            transactions: Vec::new(),
            receipts: Vec::new(),
        },
        &fixture.application.config.logical_budget,
    )
    .unwrap();
    commit_state(&mut fixture.application.repository, &commit).unwrap();
    read_state_service(&fixture.application.service).unwrap();
    commit
}

pub(super) fn request(
    parent: &StateVersion,
    height: u64,
    offset: u64,
    maximum: u32,
) -> RequestQuery {
    RequestQuery {
        path: "/eve/recovery/v1/delta".into(),
        data: encode_state_delta_request(&StateDeltaRequest {
            parent: parent.clone(),
            target_height: height,
            offset,
            maximum_chunk_bytes: maximum,
        })
        .unwrap(),
        ..Default::default()
    }
}
