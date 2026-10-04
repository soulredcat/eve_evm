// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod capture_public_observation;
mod create_public_follower;
mod drop_public_follower;
mod public_rpc;
mod read_public_startup;
mod spawn_public_follower;
mod stop_public_follower;
mod types;
mod verify_public_current_state;
mod verify_public_observation;
mod verify_public_receipt;
mod wait_public_durable;
mod wait_public_receipt;

pub(super) use capture_public_observation::capture_public_observation;
pub(super) use create_public_follower::create_public_follower;
pub(super) use public_rpc::public_rpc;
pub(super) use read_public_startup::read_public_startup;
pub(super) use spawn_public_follower::spawn_public_follower;
pub(super) use stop_public_follower::stop_public_follower;
pub(super) use types::PublicFollower;
pub(super) use verify_public_current_state::verify_public_current_state;
pub(super) use verify_public_observation::verify_public_observation;
pub(super) use verify_public_receipt::verify_public_receipt;
pub(super) use wait_public_durable::wait_public_durable;
pub(super) use wait_public_receipt::wait_public_receipt;

use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    replay_history, signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes, keccak256};
use anyhow::{Context, Result, ensure};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// Actual one-host T-N01 bootstrap/graceful-restart slice. No master process is
/// launched. It proves neither N05 master catch-up, power loss nor full B4 acceptance.
#[test]
fn t_n01_public_cli_follower_nonempty_bootstrap_and_graceful_restart_slice() -> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("public-follower", ClusterOptions::default())?;
    cluster.start()?;
    let recipient = Address::repeat_byte(0x99);
    let first = signed_transaction(&cluster.authority, 0, recipient, 21_000, Bytes::new());
    let first_height = submit_transaction(&cluster, 0, &first)?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], first_height + 2)?;
    let first_history = collect_certified_history(&cluster, 0, first_height + 1, &BTreeMap::new())?;
    let first_oracle = replay_history(&cluster, &first_history)?;
    let mut follower = create_public_follower::create_public_follower(&cluster)?;
    ensure!(
        !follower.data.exists(),
        "PUBLIC_FOLLOWER_NOT_CLEAN_BOOTSTRAP"
    );
    spawn_public_follower::spawn_public_follower(&mut follower)?;
    let started = read_public_startup::read_public_startup(
        &mut follower,
        Instant::now() + Duration::from_secs(90),
    )?;
    ensure!(
        started["height"] == 0 && started["authenticated_finality"] == false,
        "PUBLIC_FOLLOWER_BOOTSTRAP_METADATA"
    );
    let receipt = wait_public_receipt::wait_public_receipt(
        &mut follower,
        keccak256(&first),
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_public_receipt::verify_public_receipt(
        &receipt,
        &first_oracle[first_height as usize],
        &first,
        cluster.authority_address,
        recipient,
    )?;
    let durable = wait_public_durable::wait_public_durable(
        &mut follower,
        first_height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_public_current_state::verify_public_current_state(
        &cluster,
        &mut follower,
        Instant::now() + Duration::from_secs(30),
    )?;
    let recorded_durable = durable["durable_height"]
        .as_u64()
        .context("PUBLIC_FOLLOWER_DURABLE_HEIGHT")?;
    stop_public_follower::stop_public_follower(&mut follower)?;
    let second = signed_transaction(&cluster.authority, 1, recipient, 21_000, Bytes::new());
    let second_height = submit_transaction(&cluster, 0, &second)?;
    wait_for_height(&mut cluster, &[0, 1, 2, 3], second_height + 2)?;
    spawn_public_follower::spawn_public_follower(&mut follower)?;
    let restarted = read_public_startup::read_public_startup(
        &mut follower,
        Instant::now() + Duration::from_secs(90),
    )?;
    ensure!(
        restarted["height"]
            .as_u64()
            .is_some_and(|height| height >= recorded_durable)
            && restarted["authenticated_finality"] == true,
        "PUBLIC_FOLLOWER_DURABLE_PREFIX_NOT_RESTORED"
    );
    let second_receipt = wait_public_receipt::wait_public_receipt(
        &mut follower,
        keccak256(&second),
        Instant::now() + Duration::from_secs(90),
    )?;
    let second_history =
        collect_certified_history(&cluster, 0, second_height + 1, &BTreeMap::new())?;
    let second_oracle = replay_history(&cluster, &second_history)?;
    verify_public_receipt::verify_public_receipt(
        &second_receipt,
        &second_oracle[second_height as usize],
        &second,
        cluster.authority_address,
        recipient,
    )?;
    wait_public_durable::wait_public_durable(
        &mut follower,
        second_height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_public_current_state::verify_public_current_state(
        &cluster,
        &mut follower,
        Instant::now() + Duration::from_secs(30),
    )?;
    stop_public_follower::stop_public_follower(&mut follower)?;
    cluster.stop()?;
    Ok(())
}
