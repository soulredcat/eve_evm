// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod compare_master_store;
mod create_master_follower;
mod create_master_namespace;
mod drop_master_follower;
mod read_master_proof;
mod spawn_master_follower;
mod stop_master_follower;
mod types;
mod verify_master_archive;
mod verify_master_status;
mod wait_master_completion;

pub(super) use create_master_namespace::create_master_namespace;

use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    replay_history, signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes};
use anyhow::{Context, Result, ensure};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// Actual one-host N05 catch-up/restart slice. Four validators continue without
/// the exited master; this proves neither HA, power loss nor complete B4 acceptance.
#[test]
fn t_n05_actual_master_cli_nonempty_catchup_and_same_archive_restart_slice() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("master-follower", ClusterOptions::default())?;
    cluster.start()?;
    let recipient = Address::repeat_byte(0x98);
    let first = signed_transaction(&cluster.authority, 0, recipient, 21_000, Bytes::new());
    let first_height = submit_transaction(&cluster, 0, &first)?;
    ensure!(
        first_height > 0 && first_height <= 128,
        "MASTER_FOLLOWER_ORACLE_PREFIX_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], first_height + 2)?;
    let first_history = collect_certified_history(&cluster, 0, first_height + 1, &BTreeMap::new())?;
    let first_oracle = replay_history(&cluster, &first_history)?;
    let mut follower = create_master_follower::create_master_follower(&cluster)?;
    ensure!(
        !follower.data.exists(),
        "MASTER_FOLLOWER_NOT_CLEAN_BOOTSTRAP"
    );
    spawn_master_follower::spawn_master_follower(&mut follower, first_height as u64)?;
    let first_status = wait_master_completion::wait_master_completion(
        &mut follower,
        Instant::now() + Duration::from_secs(90),
    )?;
    let expected_first = first_oracle
        .get(first_height as usize)
        .context("MASTER_FOLLOWER_ORACLE_HEIGHT")?;
    ensure!(
        expected_first.block.transactions.contains(&first),
        "MASTER_FOLLOWER_FIRST_NONEMPTY_TRANSACTION"
    );
    verify_master_status::verify_master_status(&first_status, expected_first)?;
    verify_master_archive::verify_master_archive(&follower, &first_status, first_height as u64)?;
    compare_master_store::compare_master_store(&follower, &first_oracle, first_height as u64)?;
    let retained_proof = read_master_proof::read_master_proof(&follower, first_height as u64)?;
    ensure!(follower.child.is_none(), "MASTER_FOLLOWER_CHILD_NOT_EXITED");
    // The master is absent during this real validator-finalized transaction.
    let second = signed_transaction(&cluster.authority, 1, recipient, 21_000, Bytes::new());
    let second_height = submit_transaction(&cluster, 0, &second)?;
    ensure!(
        second_height > first_height && second_height <= 128,
        "MASTER_FOLLOWER_CATCHUP_HEIGHT"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], second_height + 2)?;
    let second_history =
        collect_certified_history(&cluster, 0, second_height + 1, &BTreeMap::new())?;
    let second_oracle = replay_history(&cluster, &second_history)?;
    ensure!(
        second_oracle.get(first_height as usize) == Some(expected_first),
        "MASTER_FOLLOWER_REPLAY_PREFIX_CHANGED"
    );
    spawn_master_follower::spawn_master_follower(&mut follower, second_height as u64)?;
    let second_status = wait_master_completion::wait_master_completion(
        &mut follower,
        Instant::now() + Duration::from_secs(90),
    )?;
    let expected_second = second_oracle
        .get(second_height as usize)
        .context("MASTER_FOLLOWER_ORACLE_HEIGHT")?;
    ensure!(
        expected_second.block.transactions.contains(&second),
        "MASTER_FOLLOWER_SECOND_NONEMPTY_TRANSACTION"
    );
    verify_master_status::verify_master_status(&second_status, expected_second)?;
    verify_master_archive::verify_master_archive(&follower, &second_status, second_height as u64)?;
    compare_master_store::compare_master_store(&follower, &second_oracle, second_height as u64)?;
    ensure!(
        retained_proof == read_master_proof::read_master_proof(&follower, first_height as u64)?,
        "MASTER_FOLLOWER_RETAINED_PROOF_REWRITTEN"
    );
    cluster.stop()?;
    Ok(())
}
