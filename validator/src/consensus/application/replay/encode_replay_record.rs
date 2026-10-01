// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MAX_REPLAY_BYTES, ReplayEnvelope, ReplayRecord};
use anyhow::{Result, ensure};
use eve_state::encode_state_version;
use prost::Message;

pub(in crate::consensus::application) fn encode_replay_record(
    record: &ReplayRecord,
) -> Result<Vec<u8>> {
    let envelope = match record {
        ReplayRecord::Decided(decision) => ReplayEnvelope {
            schema: 1,
            phase: 1,
            genesis: decision.target.identity.genesis.0.0.to_vec(),
            parent: encode_state_version(&decision.parent)
                .map_err(|_| anyhow::anyhow!("invalid replay parent version"))?
                .to_vec(),
            target: encode_state_version(&decision.target)
                .map_err(|_| anyhow::anyhow!("invalid replay target version"))?
                .to_vec(),
            commit_identity: decision.commit_identity.to_vec(),
            previous_consensus_hash: decision.previous_consensus_hash.to_vec(),
            request: Some(decision.request.clone()),
            proposer_owner: decision.proposer_owner.to_vec(),
            decision_sequence: 0,
            decision_hash: Vec::new(),
        },
        ReplayRecord::Synced {
            target,
            commit_identity,
            decision_cursor,
        } => ReplayEnvelope {
            schema: 1,
            phase: 2,
            genesis: target.identity.genesis.0.0.to_vec(),
            parent: Vec::new(),
            target: encode_state_version(target)
                .map_err(|_| anyhow::anyhow!("invalid replay target version"))?
                .to_vec(),
            commit_identity: commit_identity.to_vec(),
            previous_consensus_hash: Vec::new(),
            request: None,
            proposer_owner: Vec::new(),
            decision_sequence: decision_cursor.sequence,
            decision_hash: decision_cursor.content_hash.to_vec(),
        },
    };
    ensure!(
        envelope.encoded_len() <= MAX_REPLAY_BYTES,
        "consensus replay record byte limit"
    );
    Ok(envelope.encode_to_vec())
}
