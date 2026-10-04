// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod advance_checkpoint_tail;
mod create_checkpoint_follower;
mod drop_checkpoint_read_lease_adapter;
mod release_checkpoint_read;
mod reserve_checkpoint_read;
mod types;
mod verify_checkpoint_current_state;
mod verify_checkpoint_markers;
mod verify_stored_checkpoint;

pub(super) use create_checkpoint_follower::create_checkpoint_follower;
pub(super) use verify_checkpoint_current_state::verify_checkpoint_current_state;
pub(super) use verify_checkpoint_markers::verify_checkpoint_markers;
pub(super) use verify_stored_checkpoint::verify_stored_checkpoint;

use super::public_follower::{
    read_public_startup, spawn_public_follower, stop_public_follower, wait_public_durable,
};
use super::public_tail_recovery::kill_public_follower;
use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    replay_history, signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes};
use anyhow::{Result, ensure};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// Actual local checkpoint download/activation plus ordinary tail and process
/// restart. No master, hardware power-loss, PQ or complete B4 claim is made.
#[test]
fn t_n04_actual_public_checkpoint_bootstrap_nonempty_tail_and_default_restart_slice() -> Result<()>
{
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("public-checkpoint", ClusterOptions::default())?;
    cluster.start()?;
    let recipient = Address::repeat_byte(0x96);
    let first = signed_transaction(&cluster.authority, 0, recipient, 21_000, Bytes::new());
    let first_height = submit_transaction(&cluster, 0, &first)?;
    ensure!(
        first_height > 0 && first_height <= 128,
        "PUBLIC_CHECKPOINT_HEIGHT_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], first_height + 2)?;
    let first_history = collect_certified_history(&cluster, 0, first_height + 1, &BTreeMap::new())?;
    let first_oracle = replay_history(&cluster, &first_history)?;
    let expected_first = &first_oracle[first_height as usize];
    ensure!(
        expected_first.block.transactions.contains(&first)
            && !expected_first.block.receipts.is_empty(),
        "PUBLIC_CHECKPOINT_NONEMPTY_ORACLE"
    );
    let (namespace, mut follower) =
        create_checkpoint_follower::create_checkpoint_follower(&cluster)?;
    follower.checkpoint_height = Some(first_height as u64);
    ensure!(
        !follower.data.exists(),
        "PUBLIC_CHECKPOINT_NOT_CLEAN_BOOTSTRAP"
    );
    spawn_public_follower(&mut follower)?;
    let started = read_public_startup(&mut follower, Instant::now() + Duration::from_secs(180))?;
    ensure!(
        started["height"] == first_height as u64 && started["authenticated_finality"] == true,
        "PUBLIC_CHECKPOINT_STARTUP_TARGET"
    );
    let first_status = wait_public_durable(
        &mut follower,
        first_height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_checkpoint_markers::verify_checkpoint_markers(
        &first_status,
        first_height as u64,
        first_height as u64,
    )?;
    // Current RPC can already have advanced past H; historical receipts remain GAP.
    verify_checkpoint_current_state::verify_checkpoint_current_state(
        &cluster,
        &mut follower,
        recipient,
        first_height as u64,
        first_height as u64,
        1,
        Instant::now() + Duration::from_secs(30),
    )?;
    let (second_height, saved_height) = advance_checkpoint_tail::advance_checkpoint_tail(
        &mut cluster,
        &mut follower,
        recipient,
        expected_first,
    )?;
    kill_public_follower(&mut follower)?;
    ensure!(
        follower.child.is_none(),
        "PUBLIC_CHECKPOINT_CHILD_NOT_REAPED"
    );
    // The child is reaped: immutable artifact handles can be opened without
    // contending with runtime transfer/recovery directory ownership.
    let retained = verify_stored_checkpoint::verify_stored_checkpoint(&follower, expected_first)?;
    follower.checkpoint_height = None;
    spawn_public_follower(&mut follower)?;
    let restarted = read_public_startup(&mut follower, Instant::now() + Duration::from_secs(90))?;
    ensure!(
        restarted["height"]
            .as_u64()
            .is_some_and(|height| height >= saved_height && height <= 128)
            && restarted["authenticated_finality"] == true,
        "PUBLIC_CHECKPOINT_BASE_AND_TAIL_NOT_RESTORED"
    );
    let restored = wait_public_durable(
        &mut follower,
        second_height,
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_checkpoint_markers::verify_checkpoint_markers(
        &restored,
        first_height as u64,
        second_height,
    )?;
    verify_checkpoint_current_state::verify_checkpoint_current_state(
        &cluster,
        &mut follower,
        recipient,
        first_height as u64,
        second_height,
        2,
        Instant::now() + Duration::from_secs(30),
    )?;
    stop_public_follower(&mut follower)?;
    ensure!(
        verify_stored_checkpoint::verify_stored_checkpoint(&follower, expected_first)? == retained,
        "PUBLIC_CHECKPOINT_ARTIFACT_CHANGED"
    );
    cluster.stop()?;
    drop(follower);
    drop(namespace);
    Ok(())
}
