// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Cluster, ClusterOptions, cluster_lease, collect_certified_history};
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{HistoricalValidatorSet, verify_commit_certificate},
};
use std::collections::BTreeMap;

#[test]
fn t_c04_actual_native_certificate_rejects_wrong_context_weight_and_signatures() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc04", ClusterOptions::default())?;
    cluster.start()?;
    let history = collect_certified_history(&cluster, 0, 1, &BTreeMap::new())?;
    let actual = &history[0];
    let header = &actual.block.header;
    let commit = &actual.commit;
    let id = &actual.block.block_id;
    let set = HistoricalValidatorSet {
        height: 1,
        authentication: ConsensusAuthenticationRequirement::ClassicalDev,
        validators: actual.validators.clone(),
    };
    let verify = |chain: &str, height, round, block_id, head, certificate, validators| {
        verify_commit_certificate(
            chain,
            height,
            round,
            block_id,
            head,
            certificate,
            validators,
        )
    };
    ensure!(
        verify(&cluster.chain_id, 1, commit.round, id, header, commit, &set).is_ok(),
        "actual certificate baseline failed"
    );
    ensure!(
        verify("wrong-chain", 1, commit.round, id, header, commit, &set).is_err(),
        "wrong chain accepted"
    );
    ensure!(
        verify(&cluster.chain_id, 2, commit.round, id, header, commit, &set).is_err(),
        "wrong height accepted"
    );
    ensure!(
        verify(
            &cluster.chain_id,
            1,
            commit.round + 1,
            id,
            header,
            commit,
            &set
        )
        .is_err(),
        "wrong round accepted"
    );
    let mut wrong_id = id.clone();
    wrong_id.part_set_header.as_mut().unwrap().total += 1;
    ensure!(
        verify(
            &cluster.chain_id,
            1,
            commit.round,
            &wrong_id,
            header,
            commit,
            &set
        )
        .is_err(),
        "wrong full part-set id accepted"
    );
    let mut bad_signature = commit.clone();
    let present = bad_signature
        .signatures
        .iter_mut()
        .find(|signature| signature.block_id_flag == 2)
        .unwrap();
    present.signature[0] ^= 1;
    ensure!(
        verify(
            &cluster.chain_id,
            1,
            commit.round,
            id,
            header,
            &bad_signature,
            &set
        )
        .is_err(),
        "forged signature accepted"
    );
    let mut underweight = commit.clone();
    let mut kept = 0;
    for signature in &mut underweight.signatures {
        if signature.block_id_flag == 2 {
            kept += 1;
        }
        if signature.block_id_flag != 2 || kept > 2 {
            signature.block_id_flag = 1;
            signature.validator_address.clear();
            signature.timestamp = None;
            signature.signature.clear();
        }
    }
    ensure!(
        verify(
            &cluster.chain_id,
            1,
            commit.round,
            id,
            header,
            &underweight,
            &set
        )
        .is_err(),
        "2/4 weight accepted"
    );
    let mut changed_set = set.clone();
    changed_set.validators[0].voting_power += 1;
    ensure!(
        verify(
            &cluster.chain_id,
            1,
            commit.round,
            id,
            header,
            commit,
            &changed_set
        )
        .is_err(),
        "wrong applicable roster accepted"
    );
    cluster.stop()?;
    Ok(())
}
