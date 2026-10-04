// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::verify_checkpoint_markers::verify_checkpoint_markers;
use crate::cases::public_follower::{
    PublicFollower, capture_public_observation, public_rpc, verify_public_observation,
};
use crate::support::{Cluster, collect_certified_history, replay_history};
use alloy_primitives::Address;
use anyhow::{Context, Result, ensure};
use serde_json::json;
use std::{
    collections::BTreeMap,
    time::{Duration, Instant},
};

pub(in crate::cases) fn verify_checkpoint_current_state(
    cluster: &Cluster,
    follower: &mut PublicFollower,
    recipient: Address,
    checkpoint: u64,
    minimum_durable: u64,
    expected_nonce: u64,
    deadline: Instant,
) -> Result<()> {
    loop {
        ensure!(
            Instant::now() < deadline,
            "PUBLIC_CHECKPOINT_CURRENT_ORACLE_DEADLINE"
        );
        let observed = capture_public_observation(follower, cluster.authority_address, deadline)?;
        let height = observed.roots["height"]
            .as_str()
            .context("PUBLIC_CHECKPOINT_ROOT_HEIGHT")?;
        let logical = u64::from_str_radix(
            height
                .strip_prefix("0x")
                .context("PUBLIC_CHECKPOINT_ROOT_HEIGHT")?,
            16,
        )?;
        ensure!(
            logical >= minimum_durable && logical <= 128,
            "PUBLIC_CHECKPOINT_ORACLE_PREFIX_BOUND"
        );
        let recipient_balance =
            match public_rpc(follower.http, "eth_getBalance", json!([recipient, height])) {
                Ok(balance) => balance,
                Err(_) => {
                    std::thread::sleep(Duration::from_millis(25));
                    continue;
                }
            };
        let history =
            collect_certified_history(cluster, 0, i64::try_from(logical)? + 1, &BTreeMap::new())?;
        let oracle = replay_history(cluster, &history)?;
        let expected = &oracle[logical as usize];
        verify_public_observation(&observed, expected, cluster.authority_address)?;
        verify_checkpoint_markers(&observed.status, checkpoint, minimum_durable)?;
        ensure!(
            expected
                .state
                .accounts
                .get(&cluster.authority_address)
                .is_some_and(|account| account.nonce == expected_nonce),
            "PUBLIC_CHECKPOINT_EXACT_NONCE"
        );
        let expected_balance = expected
            .state
            .accounts
            .get(&recipient)
            .map_or(alloy_primitives::U256::ZERO, |account| account.balance);
        ensure!(
            recipient_balance == format!("0x{expected_balance:x}"),
            "PUBLIC_CHECKPOINT_RECIPIENT_BALANCE"
        );
        ensure!(
            Instant::now() < deadline,
            "PUBLIC_CHECKPOINT_CURRENT_ORACLE_DEADLINE"
        );
        return Ok(());
    }
}
