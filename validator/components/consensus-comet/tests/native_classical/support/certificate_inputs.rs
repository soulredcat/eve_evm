// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{block_id, bytes, timestamp};
use eve_consensus_comet::{
    consensus::certificates::ClassicalValidator,
    wire::tendermint::{
        types::{Commit, CommitSig, Header},
        version::Consensus,
    },
};
use serde_json::Value;

pub fn validators(value: &Value) -> Vec<ClassicalValidator> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|validator| ClassicalValidator {
            public_key: bytes(&validator["public_key"]).try_into().unwrap(),
            voting_power: validator["voting_power"].as_i64().unwrap(),
        })
        .collect()
}

pub fn header(value: &Value) -> Header {
    Header {
        version: Some(Consensus {
            block: value["version"]["block"].as_u64().unwrap(),
            app: value["version"]["app"].as_u64().unwrap(),
        }),
        chain_id: value["chain_id"].as_str().unwrap().into(),
        height: value["height"].as_i64().unwrap(),
        time: timestamp(&value["time"]),
        last_block_id: block_id(&value["last_block_id"]),
        last_commit_hash: bytes(&value["last_commit_hash"]),
        data_hash: bytes(&value["data_hash"]),
        validators_hash: bytes(&value["validators_hash"]),
        next_validators_hash: bytes(&value["next_validators_hash"]),
        consensus_hash: bytes(&value["consensus_hash"]),
        app_hash: bytes(&value["app_hash"]),
        last_results_hash: bytes(&value["last_results_hash"]),
        evidence_hash: bytes(&value["evidence_hash"]),
        proposer_address: bytes(&value["proposer_address"]),
    }
}

pub fn commit(value: &Value) -> Commit {
    Commit {
        height: value["height"].as_i64().unwrap(),
        round: value["round"].as_i64().unwrap().try_into().unwrap(),
        block_id: block_id(&value["block_id"]),
        signatures: value["signatures"]
            .as_array()
            .unwrap()
            .iter()
            .map(|signature| CommitSig {
                block_id_flag: signature["flag"].as_i64().unwrap().try_into().unwrap(),
                validator_address: bytes(&signature["address"]),
                timestamp: timestamp(&signature["timestamp"]),
                signature: bytes(&signature["signature"]),
            })
            .collect(),
    }
}
