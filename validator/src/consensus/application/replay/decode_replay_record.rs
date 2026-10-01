// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MAX_REPLAY_BYTES, ReplayDecision, ReplayEnvelope, ReplayRecord};
use alloy_primitives::{Address, B256};
use anyhow::{Context, Result, ensure};
use eve_state::decode_state_version;
use eve_storage::records::OpaqueRecordCursor;
use prost::Message;

pub(in crate::consensus::application) fn decode_replay_record(
    bytes: &[u8],
) -> Result<ReplayRecord> {
    ensure!(
        !bytes.is_empty() && bytes.len() <= MAX_REPLAY_BYTES,
        "consensus replay record byte limit"
    );
    let envelope = ReplayEnvelope::decode(bytes).context("consensus replay record decode")?;
    ensure!(
        envelope.schema == 1 && envelope.encode_to_vec() == bytes,
        "noncanonical consensus replay record"
    );
    let target = decode_state_version(&envelope.target)
        .map_err(|_| anyhow::anyhow!("invalid replay target version"))?;
    ensure!(
        envelope.genesis.as_slice() == target.identity.genesis.0.0,
        "replay genesis binding mismatch"
    );
    let commit_identity = B256::from(
        <[u8; 32]>::try_from(envelope.commit_identity.as_slice())
            .context("replay commit identity width")?,
    );
    match envelope.phase {
        1 => {
            ensure!(
                envelope.decision_sequence == 0 && envelope.decision_hash.is_empty(),
                "invalid decided replay fields"
            );
            Ok(ReplayRecord::Decided(Box::new(ReplayDecision {
                parent: decode_state_version(&envelope.parent)
                    .map_err(|_| anyhow::anyhow!("invalid replay parent version"))?,
                target,
                commit_identity,
                previous_consensus_hash: envelope
                    .previous_consensus_hash
                    .as_slice()
                    .try_into()
                    .context("replay previous hash width")?,
                proposer_owner: Address::from(
                    <[u8; 20]>::try_from(envelope.proposer_owner.as_slice())
                        .context("replay proposer width")?,
                ),
                request: envelope.request.context("decided replay input missing")?,
            })))
        }
        2 => {
            ensure!(
                envelope.parent.is_empty()
                    && envelope.previous_consensus_hash.is_empty()
                    && envelope.request.is_none()
                    && envelope.proposer_owner.is_empty()
                    && envelope.decision_sequence > 0,
                "invalid synced replay fields"
            );
            Ok(ReplayRecord::Synced {
                target: Box::new(target),
                commit_identity,
                decision_cursor: OpaqueRecordCursor {
                    sequence: envelope.decision_sequence,
                    content_hash: envelope
                        .decision_hash
                        .as_slice()
                        .try_into()
                        .context("replay decision hash width")?,
                },
            })
        }
        _ => anyhow::bail!("unknown consensus replay phase"),
    }
}
