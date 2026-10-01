// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions, cluster_lease, collect_certified_history, replay_history,
};
use alloy_primitives::B256;
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement,
    certificates::{HistoricalValidatorSet, verify_commit_certificate},
};
use eve_state::{development_state_budget, validate_state_commit};
use std::collections::BTreeMap;

#[test]
fn t_c08_forged_master_roots_and_changed_header_cannot_replace_actual_certificate() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc08", ClusterOptions::default())?;
    cluster.start()?;
    let history = collect_certified_history(&cluster, 0, 1, &BTreeMap::new())?;
    let actual = &history[0];
    let mut forged_header = actual.block.header.clone();
    forged_header.app_hash = vec![0x77; 32];
    let set = HistoricalValidatorSet {
        height: 1,
        authentication: ConsensusAuthenticationRequirement::ClassicalDev,
        validators: actual.validators.clone(),
    };
    ensure!(
        verify_commit_certificate(
            &cluster.chain_id,
            1,
            actual.commit.round,
            &actual.block.block_id,
            &forged_header,
            &actual.commit,
            &set
        )
        .is_err(),
        "received forged root inherited actual native authority"
    );
    let replay = replay_history(&cluster, &history)?;
    let mut forged_state = replay[1].clone();
    forged_state.target.evm_root.0 = B256::repeat_byte(0x77);
    ensure!(
        validate_state_commit(&forged_state, &development_state_budget()).is_err(),
        "asserted matching root bypassed independent state validation"
    );
    cluster.stop()?;
    Ok(())
}
