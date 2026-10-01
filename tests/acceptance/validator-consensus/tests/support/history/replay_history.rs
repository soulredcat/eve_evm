// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CertifiedBlock;
use crate::support::Cluster;
use alloy_primitives::{B256, Bytes};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::validator_address;
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_state::{StateCommit, development_state_budget};

pub(crate) fn replay_history(
    cluster: &Cluster,
    history: &[CertifiedBlock],
) -> Result<Vec<StateCommit>> {
    let mut commits = vec![cluster.genesis.clone()];
    for certified in history {
        let header = &certified.block.header;
        let parent = commits.last().context("replay parent missing")?;
        let expected_anchor = if header.height == 1 {
            parent.target.content_digest
        } else {
            parent
                .target
                .application
                .context("non-genesis application commitment missing")?
                .0
        };
        ensure!(
            header.app_hash == expected_anchor.as_slice(),
            "actual native H+1 application anchor mismatch"
        );
        let proposer = cluster
            .nodes
            .iter()
            .find(|node| validator_address(&node.public_key).as_slice() == header.proposer_address)
            .context("proposer owner not genesis/fixture enrolled")?
            .owner;
        let previous = match header.last_block_id.as_ref().map(|id| id.hash.as_slice()) {
            Some(hash) if hash.len() == 32 => B256::from_slice(hash),
            None | Some([]) if header.height == 1 => B256::ZERO,
            _ => anyhow::bail!("invalid native preceding consensus hash"),
        };
        let transactions: Vec<Bytes> = certified
            .block
            .transactions
            .iter()
            .cloned()
            .map(Bytes::from)
            .collect();
        let input = ExecutionBlockInput {
            timestamp: u64::try_from(header.time.as_ref().context("native time missing")?.seconds)?,
            proposer,
            previous_consensus_hash: previous,
        };
        let prepared = execute_state_block(
            parent,
            &input,
            &transactions,
            &development_state_budget(),
            512 * 1_048_576,
        )
        .map_err(|error| anyhow::anyhow!("independent canonical replay failed: {error:?}"))?;
        commits.push(prepared.commit);
    }
    Ok(commits)
}
