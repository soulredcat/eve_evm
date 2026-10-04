// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod capture_durable_prefix;
mod kill_public_follower;
mod types;
mod verify_recovered_durability;
mod verify_recovered_startup;

pub(super) use kill_public_follower::kill_public_follower;

use super::public_follower::{
    create_public_follower, read_public_startup, spawn_public_follower, stop_public_follower,
    verify_public_current_state, verify_public_receipt, wait_public_durable, wait_public_receipt,
};
use crate::support::{
    Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease, collect_certified_history,
    replay_history, signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes, keccak256};
use anyhow::{Result, ensure};
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// One-host N10 process-crash and later validator-tail retrieval slice, with no
/// master. No controlled unsynced-tail loss or hardware power-loss claim is made.
#[test]
fn t_n10_actual_public_sigkill_restores_durable_prefix_and_fetches_validator_tail_slice()
-> Result<()> {
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("public-tail-recovery", ClusterOptions::default())?;
    cluster.start()?;
    let recipient = Address::repeat_byte(0x97);
    let first = signed_transaction(&cluster.authority, 0, recipient, 21_000, Bytes::new());
    let first_height = submit_transaction(&cluster, 0, &first)?;
    ensure!(
        first_height > 0 && first_height <= 256,
        "PUBLIC_TAIL_ORACLE_PREFIX_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], first_height + 2)?;
    let first_history = collect_certified_history(&cluster, 0, first_height + 1, &BTreeMap::new())?;
    let first_oracle = replay_history(&cluster, &first_history)?;
    ensure!(
        first_oracle[first_height as usize]
            .state
            .accounts
            .get(&cluster.authority_address)
            .is_some_and(|account| account.nonce == 1),
        "PUBLIC_TAIL_FIRST_REPLAY_NONCE"
    );
    let mut follower = create_public_follower(&cluster)?;
    ensure!(!follower.data.exists(), "PUBLIC_TAIL_NOT_CLEAN_BOOTSTRAP");
    spawn_public_follower(&mut follower)?;
    let started = read_public_startup(&mut follower, Instant::now() + Duration::from_secs(90))?;
    ensure!(
        started["height"] == 0 && started["authenticated_finality"] == false,
        "PUBLIC_TAIL_BOOTSTRAP_METADATA"
    );
    let first_receipt = wait_public_receipt(
        &mut follower,
        keccak256(&first),
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_public_receipt(
        &first_receipt,
        &first_oracle[first_height as usize],
        &first,
        cluster.authority_address,
        recipient,
    )?;
    let durable_status = wait_public_durable(
        &mut follower,
        first_height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    let durable =
        capture_durable_prefix::capture_durable_prefix(&durable_status, first_height as u64)?;
    verify_public_current_state(
        &cluster,
        &mut follower,
        Instant::now() + Duration::from_secs(30),
    )?;
    kill_public_follower::kill_public_follower(&mut follower)?;
    ensure!(follower.child.is_none(), "PUBLIC_TAIL_CHILD_NOT_REAPED");
    // Validators finalize the new tail while public and master are both absent.
    let second = signed_transaction(&cluster.authority, 1, recipient, 21_000, Bytes::new());
    let second_height = submit_transaction(&cluster, 0, &second)?;
    ensure!(
        second_height > first_height
            && second_height as u64 > durable.height
            && second_height <= 256,
        "PUBLIC_TAIL_NEW_HEIGHT_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], second_height + 2)?;
    let second_history =
        collect_certified_history(&cluster, 0, second_height + 1, &BTreeMap::new())?;
    let second_oracle = replay_history(&cluster, &second_history)?;
    ensure!(
        second_oracle[first_height as usize] == first_oracle[first_height as usize],
        "PUBLIC_TAIL_REPLAY_PREFIX_CHANGED"
    );
    ensure!(
        second_oracle[second_height as usize]
            .state
            .accounts
            .get(&cluster.authority_address)
            .is_some_and(|account| account.nonce == 2),
        "PUBLIC_TAIL_SECOND_REPLAY_NONCE"
    );
    spawn_public_follower(&mut follower)?;
    let restarted = read_public_startup(&mut follower, Instant::now() + Duration::from_secs(90))?;
    verify_recovered_startup::verify_recovered_startup(&restarted, durable, second_height as u64)?;
    let second_receipt = wait_public_receipt(
        &mut follower,
        keccak256(&second),
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_public_receipt(
        &second_receipt,
        &second_oracle[second_height as usize],
        &second,
        cluster.authority_address,
        recipient,
    )?;
    let caught_up = wait_public_durable(
        &mut follower,
        second_height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_recovered_durability::verify_recovered_durability(
        &caught_up,
        durable,
        second_height as u64,
    )?;
    verify_public_current_state(
        &cluster,
        &mut follower,
        Instant::now() + Duration::from_secs(30),
    )?;
    stop_public_follower(&mut follower)?;
    cluster.stop()?;
    Ok(())
}
