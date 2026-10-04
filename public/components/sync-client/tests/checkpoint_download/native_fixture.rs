// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use base64::{Engine, engine::general_purpose::STANDARD};
use eve_development_fixtures::{native::Frame, recovery::RecoveryChain};
use serde_json::{Value, json};

fn block_id(id: &eve_consensus_comet::wire::tendermint::types::BlockId) -> Value {
    let parts = id.part_set_header.as_ref();
    json!({"hash":hex::encode(&id.hash), "parts":{"total":parts.map_or(0, |parts| parts.total),
        "hash":parts.map_or_else(String::new, |parts| hex::encode(&parts.hash))}})
}
fn header(frame: &Frame) -> Value {
    let header = &frame.header;
    let version = header.version.as_ref().unwrap();
    let previous = header
        .last_block_id
        .as_ref()
        .map(block_id)
        .unwrap_or_else(|| json!({"hash":"","parts":{"total":0,"hash":""}}));
    json!({"version":{"block":version.block.to_string(),"app":version.app.to_string()},
        "chain_id":header.chain_id,"height":header.height.to_string(),
        "time":header.time.as_ref().unwrap().to_string(),"last_block_id":previous,
        "last_commit_hash":hex::encode(&header.last_commit_hash),"data_hash":hex::encode(&header.data_hash),
        "validators_hash":hex::encode(&header.validators_hash),"next_validators_hash":hex::encode(&header.next_validators_hash),
        "consensus_hash":hex::encode(&header.consensus_hash),"app_hash":hex::encode(&header.app_hash),
        "last_results_hash":hex::encode(&header.last_results_hash),"evidence_hash":hex::encode(&header.evidence_hash),
        "proposer_address":hex::encode(&header.proposer_address)})
}
pub(super) fn block(chain: &RecoveryChain, height: usize) -> Value {
    let frame = &chain.frames[height - 1];
    let txs: Vec<String> = chain.commits[height]
        .block
        .transactions
        .iter()
        .map(|tx| STANDARD.encode(tx))
        .collect();
    json!({"result":{"block_id":block_id(&frame.id), "block":{"header":header(frame),
        "data":{"txs":txs},"evidence":{"evidence":[]},"last_commit":null}}})
}
pub(super) fn commit(chain: &RecoveryChain, height: usize) -> Value {
    let frame = &chain.frames[height - 1];
    let signatures: Vec<Value> = frame.commit.signatures.iter().map(|signature| json!({
        "block_id_flag":signature.block_id_flag,"validator_address":hex::encode(&signature.validator_address),
        "timestamp":signature.timestamp.as_ref().unwrap().to_string(),"signature":STANDARD.encode(&signature.signature)
    })).collect();
    json!({"result":{"signed_header":{"header":header(frame),"commit":{"height":frame.commit.height.to_string(),
        "round":frame.commit.round,"block_id":block_id(&frame.id),"signatures":signatures}}}})
}
