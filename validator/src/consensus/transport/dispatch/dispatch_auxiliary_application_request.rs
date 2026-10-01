// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, bail};
use eve_consensus_comet::wire::tendermint::abci::{
    ResponseApplySnapshotChunk, ResponseEcho, ResponseExtendVote, ResponseFlush, ResponseInsertTx,
    ResponseListSnapshots, ResponseLoadSnapshotChunk, ResponseOfferSnapshot, ResponseQuery,
    ResponseReapTxs, ResponseVerifyVoteExtension, request::Value as RequestValue,
    response::Value as ResponseValue,
};

pub(super) fn dispatch_auxiliary_application_request(
    request: RequestValue,
) -> Result<ResponseValue> {
    Ok(match request {
        RequestValue::Echo(input) => ResponseValue::Echo(ResponseEcho {
            message: input.message,
        }),
        RequestValue::Flush(_) => ResponseValue::Flush(ResponseFlush {}),
        RequestValue::Query(_) => ResponseValue::Query(ResponseQuery {
            code: 1,
            codespace: "EVE_QUERY_UNSUPPORTED".into(),
            ..Default::default()
        }),
        RequestValue::InsertTx(_) => ResponseValue::InsertTx(ResponseInsertTx { code: 1 }),
        RequestValue::ReapTxs(_) => ResponseValue::ReapTxs(ResponseReapTxs { txs: Vec::new() }),
        RequestValue::ListSnapshots(_) => ResponseValue::ListSnapshots(ResponseListSnapshots {
            snapshots: Vec::new(),
        }),
        RequestValue::OfferSnapshot(_) => {
            ResponseValue::OfferSnapshot(ResponseOfferSnapshot { result: 3 })
        }
        RequestValue::LoadSnapshotChunk(_) => {
            ResponseValue::LoadSnapshotChunk(ResponseLoadSnapshotChunk { chunk: Vec::new() })
        }
        RequestValue::ApplySnapshotChunk(_) => {
            ResponseValue::ApplySnapshotChunk(ResponseApplySnapshotChunk {
                result: 2,
                refetch_chunks: Vec::new(),
                reject_senders: Vec::new(),
            })
        }
        RequestValue::ExtendVote(_) => ResponseValue::ExtendVote(ResponseExtendVote {
            vote_extension: Vec::new(),
        }),
        RequestValue::VerifyVoteExtension(input) => {
            ResponseValue::VerifyVoteExtension(ResponseVerifyVoteExtension {
                status: if input.vote_extension.is_empty() {
                    1
                } else {
                    2
                },
            })
        }
        _ => bail!("native request requires the canonical application handler"),
    })
}
