// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    verify_checkpoint_current_state::verify_checkpoint_current_state,
    verify_checkpoint_markers::verify_checkpoint_markers,
};
use crate::cases::public_follower::{
    PublicFollower, verify_public_receipt, wait_public_durable, wait_public_receipt,
};
use crate::support::{
    Cluster, cluster::wait_for_height, collect_certified_history, replay_history,
    signed_transaction, submit_transaction,
};
use alloy_primitives::{Address, Bytes, keccak256};
use anyhow::{Context, Result, ensure};
use eve_state::StateCommit;
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

/// Finalize and authenticate one real nonempty ordinary tail after bootstrap.
pub(super) fn advance_checkpoint_tail(
    cluster: &mut Cluster,
    follower: &mut PublicFollower,
    recipient: Address,
    checkpoint: &StateCommit,
) -> Result<(u64, u64)> {
    let transaction = signed_transaction(&cluster.authority, 1, recipient, 21_000, Bytes::new());
    let height = submit_transaction(cluster, 0, &transaction)?;
    ensure!(
        height > checkpoint.target.height as i64 && height <= 128,
        "PUBLIC_CHECKPOINT_TAIL_HEIGHT_BOUND"
    );
    // Capture the actual current receipt before expensive replay can let the
    // follower supersede this height; older RPC history is intentionally GAP.
    let receipt = wait_public_receipt(
        follower,
        keccak256(&transaction),
        Instant::now() + Duration::from_secs(90),
    )?;
    wait_for_height(cluster, &[0, 1, 2, 3], height + 2)?;
    let history = collect_certified_history(cluster, 0, height + 1, &BTreeMap::new())?;
    let oracle = replay_history(cluster, &history)?;
    ensure!(
        oracle[checkpoint.target.height as usize] == *checkpoint,
        "PUBLIC_CHECKPOINT_REPLAY_PREFIX_CHANGED"
    );
    verify_public_receipt(
        &receipt,
        &oracle[height as usize],
        &transaction,
        cluster.authority_address,
        recipient,
    )?;
    let durable = wait_public_durable(
        follower,
        height as u64,
        Instant::now() + Duration::from_secs(90),
    )?;
    verify_checkpoint_markers(&durable, checkpoint.target.height, height as u64)?;
    let saved = durable["durable_height"]
        .as_u64()
        .context("PUBLIC_CHECKPOINT_DURABLE_HEIGHT")?;
    verify_checkpoint_current_state(
        cluster,
        follower,
        recipient,
        checkpoint.target.height,
        height as u64,
        2,
        Instant::now() + Duration::from_secs(30),
    )?;
    Ok((height as u64, saved))
}
