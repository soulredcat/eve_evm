// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Cluster;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::RequestFinalizeBlock;
use eve_storage::records::{
    OpaqueRecordIdentity, development_opaque_record_budget, opaque_record_cursor,
    open_opaque_record_repository, read_opaque_record,
};
use prost::Message;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Partial test inspection of the actual schema-1 replay envelope.
#[derive(Clone, PartialEq, Message)]
struct CallbackProjection {
    #[prost(uint32, tag = "1")]
    schema: u32,
    #[prost(uint32, tag = "2")]
    phase: u32,
    #[prost(message, optional, tag = "8")]
    request: Option<RequestFinalizeBlock>,
}

pub(crate) fn read_finalized_callbacks(
    cluster: &Cluster,
    node: usize,
) -> Result<BTreeMap<i64, RequestFinalizeBlock>> {
    ensure!(
        cluster.nodes[node].child.is_none(),
        "stop replay writer before callback inspection"
    );
    let identity = OpaqueRecordIdentity {
        genesis_hash: cluster.genesis.target.identity.genesis.0.0,
        owner: cluster.nodes[node].public_key,
        domain: Sha256::digest(b"EVE_VALIDATOR_CONSENSUS_REPLAY_V1").into(),
    };
    let repository = open_opaque_record_repository(
        &cluster.nodes[node].data.join("replay"),
        identity,
        development_opaque_record_budget(),
    )?;
    let mut result = BTreeMap::new();
    for sequence in 1..=opaque_record_cursor(&repository)?.sequence {
        let stored =
            read_opaque_record(&repository, sequence)?.context("retained replay record missing")?;
        ensure!(
            stored.payload.len() <= 4 * 1_048_576 + 4000,
            "retained callback bound"
        );
        let projection = CallbackProjection::decode(stored.payload.as_slice())?;
        ensure!(
            projection.schema == 1 && [1, 2].contains(&projection.phase),
            "unknown replay projection schema"
        );
        if projection.phase == 1 {
            let request = projection.request.context("decided callback missing")?;
            ensure!(
                result.insert(request.height, request).is_none(),
                "duplicate decided callback height"
            );
        }
    }
    Ok(result)
}
