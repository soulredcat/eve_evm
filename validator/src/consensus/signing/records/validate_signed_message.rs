// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::restore_block_id;
use crate::consensus::signing::{SignerConfig, types::SignedRecord};
use anyhow::{Result, anyhow, ensure};
use eve_consensus_comet::{
    consensus::{
        certificates::validator_address,
        signing::{encode_proposal_sign_bytes, encode_vote_sign_bytes},
    },
    wire::tendermint::types::{CanonicalProposal, CanonicalVote, Proposal, Vote},
};
use prost::Message;

pub(super) fn validate_signed_message(record: &SignedRecord, config: &SignerConfig) -> Result<()> {
    let bytes = if record.hrs.step == 1 {
        let canonical = CanonicalProposal::decode_length_delimited(record.sign_bytes.as_slice())?;
        ensure!(
            canonical.chain_id == config.chain_id
                && canonical.r#type == 32
                && canonical.height == record.hrs.height
                && canonical.round == i64::from(record.hrs.round),
            "persisted proposal metadata mismatch"
        );
        let proposal = Proposal {
            r#type: canonical.r#type,
            height: canonical.height,
            round: i32::try_from(canonical.round)?,
            pol_round: i32::try_from(canonical.pol_round)?,
            block_id: restore_block_id(canonical.block_id),
            timestamp: canonical.timestamp,
            signature: record.signature.clone(),
        };
        encode_proposal_sign_bytes(&config.chain_id, &proposal)
            .map_err(|_| anyhow!("invalid persisted native proposal"))?
    } else {
        let canonical = CanonicalVote::decode_length_delimited(record.sign_bytes.as_slice())?;
        ensure!(
            canonical.chain_id == config.chain_id
                && canonical.r#type == i32::from(record.hrs.step - 1)
                && canonical.height == record.hrs.height
                && canonical.round == i64::from(record.hrs.round),
            "persisted vote metadata mismatch"
        );
        let vote = Vote {
            r#type: canonical.r#type,
            height: canonical.height,
            round: i32::try_from(canonical.round)?,
            block_id: restore_block_id(canonical.block_id),
            timestamp: canonical.timestamp,
            validator_address: validator_address(&config.expected_public_key).to_vec(),
            validator_index: 0,
            signature: record.signature.clone(),
            ..Default::default()
        };
        encode_vote_sign_bytes(&config.chain_id, &vote)
            .map_err(|_| anyhow!("invalid persisted native vote"))?
    };
    ensure!(
        bytes == record.sign_bytes,
        "noncanonical persisted signing bytes"
    );
    Ok(())
}
