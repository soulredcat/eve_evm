// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::ReplayDecision;
use crate::consensus::application::ApplicationConfig;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::hash_transaction_data;

pub(in crate::consensus::application) fn validate_replay_decision(
    decision: &ReplayDecision,
    config: &ApplicationConfig,
    expected_previous_hash: [u8; 32],
) -> Result<()> {
    let request = &decision.request;
    ensure!(
        decision.parent.identity == config.genesis.target.identity
            && decision.target.identity == decision.parent.identity,
        "replay network/config/profile mismatch"
    );
    ensure!(
        decision.parent.height.checked_add(1) == Some(decision.target.height)
            && i64::try_from(decision.target.height).ok() == Some(request.height),
        "replay height mismatch"
    );
    ensure!(
        request.hash.len() == 32
            && decision.previous_consensus_hash == expected_previous_hash
            && (request.height == 1 || expected_previous_hash != [0; 32]),
        "replay consensus hash binding mismatch"
    );
    let proposer: [u8; 20] = request
        .proposer_address
        .as_slice()
        .try_into()
        .context("replay proposer address width")?;
    ensure!(
        config.proposer_owners.get(&proposer) == Some(&decision.proposer_owner),
        "replay proposer-owner mapping mismatch"
    );
    let time = request.time.as_ref().context("replay timestamp missing")?;
    ensure!(
        time.seconds >= 0
            && (0..1_000_000_000).contains(&time.nanos)
            && u64::try_from(time.seconds).ok() == Some(decision.target.timestamp),
        "replay execution time mismatch"
    );
    hash_transaction_data(&request.txs)
        .map_err(|_| anyhow::anyhow!("replay transaction envelope limits"))?;
    Ok(())
}
