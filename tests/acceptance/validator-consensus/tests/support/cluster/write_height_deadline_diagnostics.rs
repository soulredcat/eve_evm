// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::Cluster;
use crate::support::rpc::rpc_json;
use anyhow::Result;

pub(super) fn write_height_deadline_diagnostics(cluster: &Cluster, nodes: &[usize]) -> Result<()> {
    let diagnostics: Vec<_> = nodes.iter().map(|&node| serde_json::json!({
        "node": node,
        "status": rpc_json(cluster.nodes[node].rpc, "/status").map_err(|error| error.to_string()),
        "application": rpc_json(cluster.nodes[node].rpc, "/abci_info").map_err(|error| error.to_string()),
        "peers": rpc_json(cluster.nodes[node].rpc, "/net_info").map_err(|error| error.to_string())
    })).collect();
    std::fs::write(
        cluster.artifact.join("deadline.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "nodes": diagnostics,
            "proxy_edges": cluster.proxies.observed_edges(),
            "proxy_errors": *cluster.proxies.state.errors.lock().unwrap()
        }))?,
    )?;
    Ok(())
}
