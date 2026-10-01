// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::Cluster;
use crate::support::rpc::rpc_json;
use anyhow::{Context, Result, ensure};
use std::time::{Duration, Instant};

pub(crate) fn node_height(cluster: &Cluster, index: usize) -> Result<i64> {
    let result = rpc_json(cluster.nodes[index].rpc, "/status")?;
    result["sync_info"]["latest_block_height"]
        .as_str()
        .context("native height missing")?
        .parse()
        .context("native height invalid")
}

pub(crate) fn wait_for_height(cluster: &mut Cluster, nodes: &[usize], height: i64) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(90);
    loop {
        let mut complete = true;
        for &index in nodes {
            if let Some(child) = &mut cluster.nodes[index].child {
                ensure!(
                    child.try_wait()?.is_none(),
                    "validator {index} exited before height {height}; artifacts {}",
                    cluster.artifact.display()
                );
            }
            let native = node_height(cluster, index).unwrap_or(0);
            let application = rpc_json(cluster.nodes[index].rpc, "/abci_info")
                .ok()
                .and_then(|value| {
                    value["response"]["last_block_height"]
                        .as_str()
                        .and_then(|text| text.parse::<i64>().ok())
                })
                .unwrap_or(0);
            complete &= native >= height && application >= height;
        }
        if complete {
            return Ok(());
        }
        if Instant::now() >= deadline {
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
            anyhow::bail!(
                "native/application progress deadline at {height}; artifacts {}",
                cluster.artifact.display()
            );
        }
        std::thread::sleep(Duration::from_millis(150));
    }
}
