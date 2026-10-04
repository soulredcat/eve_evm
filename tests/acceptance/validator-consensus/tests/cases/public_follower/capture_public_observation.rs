// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    public_rpc::public_rpc,
    types::{PublicFollower, PublicObservation},
};
use alloy_primitives::Address;
use anyhow::{Result, ensure};
use serde_json::json;
use std::time::{Duration, Instant};

/// Explicit-height reads retry when a later immutable publication supersedes H.
pub(super) fn capture_public_observation(
    follower: &mut PublicFollower,
    sender: Address,
    deadline: Instant,
) -> Result<PublicObservation> {
    loop {
        ensure!(
            Instant::now() < deadline,
            "PUBLIC_FOLLOWER_OBSERVATION_DEADLINE"
        );
        ensure!(
            follower
                .child
                .as_mut()
                .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_CHILD_MISSING"))?
                .try_wait()?
                .is_none(),
            "PUBLIC_FOLLOWER_PROCESS_EXITED"
        );
        let observation = (|| -> Result<PublicObservation> {
            let roots = public_rpc(follower.http, "eve_getStateRoots", json!([]))?;
            let height = roots["height"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_ROOT_HEIGHT"))?;
            let block = public_rpc(
                follower.http,
                "eth_getBlockByNumber",
                json!([height, false]),
            )?;
            let sender_balance =
                public_rpc(follower.http, "eth_getBalance", json!([sender, height]))?;
            let sender_nonce = public_rpc(
                follower.http,
                "eth_getTransactionCount",
                json!([sender, height]),
            )?;
            let status = public_rpc(follower.http, "eve_getNodeStatus", json!([]))?;
            let logical_height = u64::from_str_radix(
                height
                    .strip_prefix("0x")
                    .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_ROOT_HEIGHT"))?,
                16,
            )?;
            ensure!(
                status["applied_height"] == logical_height,
                "PUBLIC_FOLLOWER_CAPTURE_ADVANCED"
            );
            Ok(PublicObservation {
                roots,
                block,
                sender_balance,
                sender_nonce,
                status,
            })
        })();
        if let Ok(observation) = observation {
            return Ok(observation);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}
