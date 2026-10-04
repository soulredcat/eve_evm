// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod assert_completed_content_present;
mod assert_single_checkpoint_chunk;
mod drop_checkpoint_proxy_adapter;
mod finish_checkpoint_proxy;
mod forward_checkpoint_connection;
mod observe_unavailable_public_prefix;
mod recreate_checkpoint_follower;
mod relay_checkpoint_bytes;
mod run_checkpoint_proxy;
mod start_checkpoint_proxy;
mod types;
mod wait_checkpoint_proxy_gate;

use crate::{
    cases::{
        public_checkpoint::{
            create_checkpoint_follower, verify_checkpoint_current_state, verify_checkpoint_markers,
            verify_stored_checkpoint,
        },
        public_follower::{
            read_public_startup, spawn_public_follower, stop_public_follower, wait_public_durable,
        },
        public_tail_recovery::kill_public_follower,
    },
    support::{
        Cluster, ClusterOptions, cluster::wait_for_height, cluster_lease,
        collect_certified_history, replay_history, signed_transaction, submit_transaction,
    },
};
use alloy_primitives::{Address, Bytes};
use anyhow::{Context, Result, ensure};
use assert_completed_content_present::assert_completed_content_present;
use assert_single_checkpoint_chunk::assert_single_checkpoint_chunk;
use finish_checkpoint_proxy::finish_checkpoint_proxy;
use observe_unavailable_public_prefix::observe_unavailable_public_prefix;
use recreate_checkpoint_follower::recreate_checkpoint_follower;
use start_checkpoint_proxy::start_checkpoint_proxy;
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};
use wait_checkpoint_proxy_gate::wait_checkpoint_proxy_gate;

/// Actual process interruption after content sync, before any proof/base request
/// completes. Five bounded owned launches; no hardware power-loss or full B4 claim.
#[test]
fn t_n04_actual_interrupted_checkpoint_preserves_old_prefix_and_resumes_same_content() -> Result<()>
{
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("public-checkpoint-interruption", ClusterOptions::default())?;
    cluster.start()?;
    let recipient = Address::repeat_byte(0x95);
    let first = signed_transaction(&cluster.authority, 0, recipient, 21_000, Bytes::new());
    let first_height = submit_transaction(&cluster, 0, &first)?;
    ensure!(
        first_height > 0 && first_height <= 128,
        "CHECKPOINT_INTERRUPTION_FIRST_HEIGHT_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], first_height + 2)?;
    let (namespace, mut interrupted) = create_checkpoint_follower(&cluster)?;
    ensure!(
        !interrupted.data.exists(),
        "CHECKPOINT_INTERRUPTION_INITIAL_DATA_EXISTS"
    );
    spawn_public_follower(&mut interrupted)?;
    let initial = read_public_startup(&mut interrupted, Instant::now() + Duration::from_secs(90))?;
    ensure!(
        initial["height"] == 0,
        "CHECKPOINT_INTERRUPTION_INITIAL_HEIGHT"
    );
    wait_public_durable(
        &mut interrupted,
        first_height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    stop_public_follower(&mut interrupted)?;

    // A separate bounded consumer observes the actual drained prefix, rather than
    // assuming a pre-shutdown RPC status equals the final saved height/cursor.
    let mut baseline = recreate_checkpoint_follower(&cluster, &interrupted, "baseline")?;
    let old = observe_unavailable_public_prefix(&cluster, &mut baseline, 129)?;
    let saved_height = old["height"]
        .as_u64()
        .context("CHECKPOINT_INTERRUPTION_SAVED_HEIGHT")?;
    ensure!(
        saved_height >= first_height as u64,
        "CHECKPOINT_INTERRUPTION_FIRST_NOT_DURABLE"
    );
    let second = signed_transaction(&cluster.authority, 1, recipient, 21_000, Bytes::new());
    let target_height = submit_transaction(&cluster, 0, &second)?;
    ensure!(
        target_height as u64 > saved_height && target_height <= 128,
        "CHECKPOINT_INTERRUPTION_NEWER_TARGET_BOUND"
    );
    wait_for_height(&mut cluster, &[0, 1, 2, 3], target_height + 2)?;
    let history = collect_certified_history(&cluster, 0, target_height + 1, &BTreeMap::new())?;
    let oracle = replay_history(&cluster, &history)?;
    let expected = &oracle[target_height as usize];
    ensure!(
        expected.block.transactions.contains(&second)
            && !expected.block.receipts.is_empty()
            && expected
                .state
                .accounts
                .get(&cluster.authority_address)
                .is_some_and(|account| account.nonce == 2),
        "CHECKPOINT_INTERRUPTION_NONEMPTY_TARGET"
    );
    assert_single_checkpoint_chunk(expected)?;

    let mut proxy = start_checkpoint_proxy(cluster.nodes[0].rpc)?;
    interrupted.validator = proxy.address;
    interrupted.checkpoint_height = Some(target_height as u64);
    spawn_public_follower(&mut interrupted)?;
    wait_checkpoint_proxy_gate(&proxy, &mut interrupted)?;
    assert_completed_content_present(&interrupted)?;
    kill_public_follower(&mut interrupted)?;
    ensure!(
        finish_checkpoint_proxy(&mut proxy)? == 3,
        "CHECKPOINT_INTERRUPTION_PROXY_CONNECTION_COUNT"
    );
    let content_id = verify_stored_checkpoint(&interrupted, expected)?;

    let mut resumed = recreate_checkpoint_follower(&cluster, &interrupted, "restored")?;
    resumed.validator = cluster.nodes[0].rpc;
    let restored = observe_unavailable_public_prefix(&cluster, &mut resumed, target_height as u64)?;
    ensure!(
        restored == old,
        "CHECKPOINT_INTERRUPTION_OLD_PREFIX_CHANGED"
    );
    ensure!(
        verify_stored_checkpoint(&resumed, expected)? == content_id,
        "CHECKPOINT_INTERRUPTION_CONTENT_CHANGED_BEFORE_RESUME"
    );

    resumed.checkpoint_height = Some(target_height as u64);
    spawn_public_follower(&mut resumed)?;
    let activated = read_public_startup(&mut resumed, Instant::now() + Duration::from_secs(180))?;
    ensure!(
        activated["height"] == target_height as u64 && activated["authenticated_finality"] == true,
        "CHECKPOINT_INTERRUPTION_RESUMED_STARTUP_TARGET"
    );
    let durable = wait_public_durable(
        &mut resumed,
        target_height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_checkpoint_markers(&durable, target_height as u64, target_height as u64)?;
    verify_checkpoint_current_state(
        &cluster,
        &mut resumed,
        recipient,
        target_height as u64,
        target_height as u64,
        2,
        Instant::now() + Duration::from_secs(30),
    )?;
    stop_public_follower(&mut resumed)?;
    ensure!(
        verify_stored_checkpoint(&resumed, expected)? == content_id,
        "CHECKPOINT_INTERRUPTION_CONTENT_CHANGED_AFTER_ACTIVATION"
    );
    ensure!(
        interrupted.launch_count == 2 && baseline.launch_count == 1 && resumed.launch_count == 2,
        "CHECKPOINT_INTERRUPTION_EXACT_FIVE_LAUNCH_CAP"
    );
    cluster.stop()?;
    drop((interrupted, baseline, resumed, proxy));
    drop(namespace);
    Ok(())
}
