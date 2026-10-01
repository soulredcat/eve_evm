// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::tendermint::types::{BlockId, PartSetHeader, Proposal, Vote};
use prost_types::Timestamp;
use serde_json::Value;

pub fn bytes(value: &Value) -> Vec<u8> {
    hex::decode(value.as_str().expect("hex fixture string")).unwrap()
}

pub fn timestamp(value: &Value) -> Option<Timestamp> {
    if value.is_null() {
        None
    } else {
        Some(Timestamp {
            seconds: value["seconds"].as_i64().unwrap(),
            nanos: value["nanos"].as_i64().unwrap().try_into().unwrap(),
        })
    }
}

pub fn block_id(value: &Value) -> Option<BlockId> {
    if value.is_null() {
        None
    } else {
        Some(BlockId {
            hash: bytes(&value["hash"]),
            part_set_header: Some(PartSetHeader {
                total: value["parts"]["total"]
                    .as_u64()
                    .unwrap()
                    .try_into()
                    .unwrap(),
                hash: bytes(&value["parts"]["hash"]),
            }),
        })
    }
}

pub fn vote(value: &Value) -> Vote {
    Vote {
        r#type: value["type"].as_i64().unwrap().try_into().unwrap(),
        height: value["height"].as_i64().unwrap(),
        round: value["round"].as_i64().unwrap().try_into().unwrap(),
        block_id: block_id(&value["block_id"]),
        timestamp: timestamp(&value["timestamp"]),
        validator_address: bytes(&value["validator_address"]),
        validator_index: value["validator_index"]
            .as_i64()
            .unwrap()
            .try_into()
            .unwrap(),
        signature: bytes(&value["signature"]),
        extension: bytes(&value["extension"]),
        extension_signature: bytes(&value["extension_signature"]),
    }
}

pub fn proposal(value: &Value) -> Proposal {
    Proposal {
        r#type: value["type"].as_i64().unwrap().try_into().unwrap(),
        height: value["height"].as_i64().unwrap(),
        round: value["round"].as_i64().unwrap().try_into().unwrap(),
        pol_round: value["pol_round"].as_i64().unwrap().try_into().unwrap(),
        block_id: block_id(&value["block_id"]),
        timestamp: timestamp(&value["timestamp"]),
        signature: bytes(&value["signature"]),
    }
}
