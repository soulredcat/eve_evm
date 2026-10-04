// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{public_rpc::public_rpc, types::PublicFollower};
use alloy_primitives::B256;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

pub(in crate::cases) fn wait_public_receipt(
    follower: &mut PublicFollower,
    hash: B256,
    deadline: Instant,
) -> Result<Value> {
    loop {
        ensure!(
            Instant::now() < deadline,
            "PUBLIC_FOLLOWER_RECEIPT_DEADLINE"
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
        if let Ok(receipt) = public_rpc(follower.http, "eth_getTransactionReceipt", json!([hash]))
            && receipt.is_object()
        {
            return Ok(receipt);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}
