// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::types::SignedRecord;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::wire::tendermint::types::{CanonicalProposal, Proposal};
use prost::Message;

pub(in crate::consensus::signing) fn replay_proposal(
    record: &SignedRecord,
    bytes: &[u8],
    mut proposal: Proposal,
) -> Result<Proposal> {
    let mut prior = CanonicalProposal::decode_length_delimited(record.sign_bytes.as_slice())?;
    let mut candidate = CanonicalProposal::decode_length_delimited(bytes)?;
    let timestamp = prior
        .timestamp
        .take()
        .context("persisted proposal timestamp missing")?;
    candidate.timestamp = None;
    ensure!(
        prior == candidate,
        "conflicting proposal at identical height/round/step"
    );
    proposal.timestamp = Some(timestamp);
    proposal.signature = record.signature.clone();
    Ok(proposal)
}
