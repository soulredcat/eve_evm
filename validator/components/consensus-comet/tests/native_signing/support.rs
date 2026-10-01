// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_consensus_comet::wire::tendermint::types::{BlockId, PartSetHeader, Proposal, Vote};
use prost_types::Timestamp;

pub fn block_id() -> BlockId {
    BlockId {
        hash: vec![1; 32],
        part_set_header: Some(PartSetHeader {
            total: 2,
            hash: vec![2; 32],
        }),
    }
}

pub fn vote() -> Vote {
    Vote {
        r#type: 2,
        height: 17,
        round: 2,
        block_id: Some(block_id()),
        timestamp: Some(Timestamp {
            seconds: 1_796_083_200,
            nanos: 123_456_789,
        }),
        validator_address: vec![3; 20],
        validator_index: 0,
        ..Default::default()
    }
}

pub fn proposal() -> Proposal {
    Proposal {
        r#type: 32,
        height: 17,
        round: 2,
        pol_round: -1,
        block_id: Some(block_id()),
        timestamp: vote().timestamp,
        signature: Vec::new(),
    }
}
