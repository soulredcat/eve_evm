// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::CertifiedBlock;
use crate::support::{
    Cluster,
    native_decoding::{
        decode_native_block, decode_native_commit, decode_native_header, decode_native_validators,
    },
    rpc::rpc_json,
};
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{
        ClassicalValidator, HistoricalValidatorSet, canonicalize_validator_set, hash_validator_set,
        verify_commit_certificate,
    },
};
use std::collections::BTreeMap;

pub(crate) fn collect_certified_history(
    cluster: &Cluster,
    node: usize,
    through: i64,
    changes: &BTreeMap<i64, Vec<ClassicalValidator>>,
) -> Result<Vec<CertifiedBlock>> {
    let initial: Vec<_> = cluster
        .nodes
        .iter()
        .take(4)
        .map(|node| ClassicalValidator {
            public_key: node.public_key,
            voting_power: 10_000,
        })
        .collect();
    let mut history = Vec::new();
    for height in 1..=through {
        let block = decode_native_block(&rpc_json(
            cluster.nodes[node].rpc,
            &format!("/block?height={height}"),
        )?)?;
        let response = rpc_json(cluster.nodes[node].rpc, &format!("/commit?height={height}"))?;
        let commit = decode_native_commit(&response["signed_header"]["commit"])?;
        let header = decode_native_header(&response["signed_header"]["header"])?;
        ensure!(header == block.header, "block/certificate header mismatch");
        let expected = changes
            .range(..=height)
            .next_back()
            .map_or(&initial, |(_, set)| set);
        let expected = canonicalize_validator_set(expected)
            .map_err(|error| anyhow::anyhow!("invalid trusted set: {error:?}"))?;
        let validators = decode_native_validators(
            &rpc_json(
                cluster.nodes[node].rpc,
                &format!("/validators?height={height}&per_page=100"),
            )?["validators"],
        )?;
        ensure!(
            validators == expected,
            "native historical set differs from genesis/receipt-derived roster at {height}"
        );
        ensure!(
            hash_validator_set(&expected)
                .map_err(|error| anyhow::anyhow!("set hash: {error:?}"))?
                .as_slice()
                == header.validators_hash,
            "historical validator hash mismatch"
        );
        let applicable = HistoricalValidatorSet {
            height,
            authentication: ConsensusAuthenticationRequirement::ClassicalDev,
            validators: expected,
        };
        verify_commit_certificate(
            &cluster.chain_id,
            height,
            commit.round,
            &block.block_id,
            &header,
            &commit,
            &applicable,
        )
        .map_err(|error| anyhow::anyhow!("actual certificate rejected at {height}: {error:?}"))?;
        if let Some(previous) = history
            .last()
            .map(|previous: &CertifiedBlock| &previous.block.block_id)
        {
            ensure!(
                header.last_block_id.as_ref() == Some(previous),
                "native hash chain broke"
            );
        }
        history.push(CertifiedBlock {
            block,
            commit,
            validators,
        });
    }
    Ok(history)
}
