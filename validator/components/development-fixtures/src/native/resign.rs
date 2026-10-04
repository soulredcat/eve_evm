// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{Frame, sign_commit_vote::sign_commit_vote};
use eve_consensus_comet::{
    consensus::certificates::hash_consensus_header,
    wire::tendermint::types::{BlockId, Commit, PartSetHeader},
};

pub fn resign(frame: &mut Frame) {
    frame.id = BlockId {
        hash: hash_consensus_header(&frame.header).unwrap().to_vec(),
        part_set_header: Some(PartSetHeader {
            total: 1,
            hash: vec![0x71; 32],
        }),
    };
    let mut signatures = Vec::new();
    for (index, validator) in frame.validators.iter().enumerate() {
        signatures.push(sign_commit_vote(frame, index, validator));
    }
    frame.commit = Commit {
        height: frame.header.height,
        round: 0,
        block_id: Some(frame.id.clone()),
        signatures,
    };
}
