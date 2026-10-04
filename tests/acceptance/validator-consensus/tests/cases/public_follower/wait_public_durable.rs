// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{public_rpc::public_rpc, types::PublicFollower};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

pub(super) fn wait_public_durable(
    follower: &mut PublicFollower,
    height: u64,
    deadline: Instant,
) -> Result<Value> {
    loop {
        ensure!(
            Instant::now() < deadline,
            "PUBLIC_FOLLOWER_DURABLE_DEADLINE"
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
        if let Ok(status) = public_rpc(follower.http, "eve_getNodeStatus", json!([]))
            && status["durable_height"]
                .as_u64()
                .is_some_and(|durable| durable >= height)
        {
            return Ok(status);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}
