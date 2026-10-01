// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Cluster, rpc::rpc_json};
use alloy_primitives::Bytes;
use anyhow::{Context, Result, ensure};
pub(crate) fn submit_transaction(cluster: &Cluster, node: usize, bytes: &Bytes) -> Result<i64> {
    let result = rpc_json(
        cluster.nodes[node].rpc,
        &format!("/broadcast_tx_commit?tx=0x{}", hex::encode(bytes)),
    )?;
    ensure!(
        result["check_tx"]["code"].as_u64().unwrap_or(0) == 0
            && result["tx_result"]["code"].as_u64().unwrap_or(0) == 0,
        "native transaction admission/execution rejected"
    );
    result["height"]
        .as_str()
        .context("committed transaction height missing")?
        .parse()
        .context("committed transaction height invalid")
}
