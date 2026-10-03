// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::Cluster;
use crate::support::rpc::rpc_json;
use anyhow::{Context, Result};

pub(crate) fn node_height(cluster: &Cluster, index: usize) -> Result<i64> {
    let result = rpc_json(cluster.nodes[index].rpc, "/status")?;
    result["sync_info"]["latest_block_height"]
        .as_str()
        .context("native height missing")?
        .parse()
        .context("native height invalid")
}
