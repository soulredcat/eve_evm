// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    cases::public_follower::{
        PublicFollower, capture_public_observation, read_public_startup, spawn_public_follower,
        stop_public_follower, verify_public_observation,
    },
    support::{Cluster, collect_certified_history, replay_history},
};
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    net::TcpListener,
    time::{Duration, Instant},
};

/// A held unserved loopback listener prevents any new source data without changing host networking.
pub(super) fn observe_unavailable_public_prefix(
    cluster: &Cluster,
    follower: &mut PublicFollower,
    upper_height: u64,
) -> Result<Value> {
    ensure!(
        follower.child.is_none() && follower.checkpoint_height.is_none(),
        "CHECKPOINT_INTERRUPTION_INSPECTION_MODE"
    );
    let unavailable = TcpListener::bind("127.0.0.1:0")?;
    let original = follower.validator;
    follower.validator = unavailable.local_addr()?;
    spawn_public_follower(follower)?;
    let started = read_public_startup(follower, Instant::now() + Duration::from_secs(90))?;
    let height = started["height"]
        .as_u64()
        .context("CHECKPOINT_INTERRUPTION_OLD_STARTUP_HEIGHT")?;
    ensure!(
        height > 0
            && height < upper_height
            && height <= 128
            && started["authenticated_finality"] == true,
        "CHECKPOINT_INTERRUPTION_OLD_STARTUP_PREFIX"
    );
    let observed = capture_public_observation(
        follower,
        cluster.authority_address,
        Instant::now() + Duration::from_secs(30),
    )?;
    let history =
        collect_certified_history(cluster, 0, i64::try_from(height)? + 1, &BTreeMap::new())?;
    let oracle = replay_history(cluster, &history)?;
    verify_public_observation(
        &observed,
        &oracle[height as usize],
        cluster.authority_address,
    )?;
    ensure!(
        observed.status["applied_height"] == height
            && observed.status["durable_height"] == height
            && observed.status["authenticated_height"] == height
            && observed.status["checkpoint_height"] == 0
            && observed.status["authenticated_snapshot_height"] == 0
            && observed.status["storage_failed"] == false
            && observed.status["ready"] == false
            && observed.status["readiness_reason"] == "head freshness unknown",
        "CHECKPOINT_INTERRUPTION_FALSE_OLD_MARKERS"
    );
    let cursor = observed.status["durable_record_sequence"]
        .as_u64()
        .context("CHECKPOINT_INTERRUPTION_OLD_CURSOR")?;
    ensure!(
        cursor > 0
            && oracle[height as usize]
                .state
                .accounts
                .get(&cluster.authority_address)
                .is_some_and(|account| account.nonce == 1),
        "CHECKPOINT_INTERRUPTION_OLD_NONEMPTY_PREFIX"
    );
    let result = serde_json::json!({"height": height, "cursor": cursor, "roots": observed.roots,
        "sender_balance": observed.sender_balance, "sender_nonce": observed.sender_nonce, "block": observed.block});
    stop_public_follower(follower)?;
    follower.validator = original;
    drop(unavailable);
    Ok(result)
}
