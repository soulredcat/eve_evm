// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::types::SignedRecord;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::types::{CanonicalVote, Vote};
use prost::Message;

/// Timestamp-only retry returns the original timestamp and signature, never a new signature.
pub(in crate::consensus::signing) fn replay_vote(
    record: &SignedRecord,
    bytes: &[u8],
    mut vote: Vote,
) -> Result<Vote> {
    let mut prior = CanonicalVote::decode_length_delimited(record.sign_bytes.as_slice())?;
    let mut candidate = CanonicalVote::decode_length_delimited(bytes)?;
    let timestamp = prior
        .timestamp
        .take()
        .context("persisted vote timestamp missing")?;
    candidate.timestamp = None;
    ensure!(
        prior == candidate,
        "conflicting vote at identical height/round/step"
    );
    vote.timestamp = Some(timestamp);
    vote.signature = record.signature.clone();
    Ok(vote)
}
