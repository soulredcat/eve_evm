// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DurableSigner,
    persist_signature::persist_signature,
    policy::{check_current_height, check_hrs, replay_vote},
    types::Hrs,
};
use crate::consensus::approval::{ExecutionApproval, check_vote_approval};
use anyhow::{Context, Result, anyhow, ensure};
use eve_consensus_comet::{
    consensus::{certificates::validator_address, signing::encode_vote_sign_bytes},
    wire::tendermint::types::{CanonicalVote, Vote},
};
use prost::Message;

pub(in crate::consensus) fn sign_vote(
    signer: &mut DurableSigner,
    mut vote: Vote,
    approval: Option<&ExecutionApproval>,
) -> Result<Vote> {
    let bytes = encode_vote_sign_bytes(&signer.config.chain_id, &vote)
        .map_err(|_| anyhow!("invalid native vote request"))?;
    ensure!(
        vote.validator_address == validator_address(&signer.config.expected_public_key),
        "vote enrolled identity mismatch"
    );
    let hrs = Hrs {
        height: vote.height,
        round: vote.round,
        step: u8::try_from(vote.r#type + 1)?,
    };
    if let Some(record) = check_hrs(signer, hrs)? {
        return replay_vote(record, &bytes, vote);
    }
    check_current_height(signer, vote.height)?;
    let canonical = CanonicalVote::decode_length_delimited(bytes.as_slice())?;
    if canonical.block_id.is_some() {
        let approval =
            approval.context("non-nil vote requires canonical execution/data approval")?;
        check_vote_approval(approval, signer, &vote)?;
    }
    vote.timestamp = Some(
        canonical
            .timestamp
            .context("canonical vote timestamp missing")?,
    );
    vote.signature = persist_signature(signer, hrs, bytes)?;
    Ok(vote)
}
