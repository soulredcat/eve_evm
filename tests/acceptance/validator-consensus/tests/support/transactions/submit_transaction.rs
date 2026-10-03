// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Cluster, rpc::rpc_json};
use alloy_primitives::Bytes;
use anyhow::Result;
use sha2::{Digest, Sha256};
pub(crate) fn submit_transaction(cluster: &Cluster, node: usize, bytes: &Bytes) -> Result<i64> {
    let result = rpc_json(
        cluster.nodes[node].rpc,
        &format!("/broadcast_tx_commit?tx=0x{}", hex::encode(bytes)),
    )?;
    let expected_hash = Sha256::digest(bytes).into();
    super::decode_committed_submission::decode_committed_submission(&result, &expected_hash)
}
