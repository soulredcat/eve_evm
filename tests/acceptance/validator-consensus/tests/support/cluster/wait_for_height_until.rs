// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{Cluster, node_height};
use crate::support::process::summarize_exited_validator;
use crate::support::rpc::rpc_json;
use anyhow::{Result, ensure};
use std::time::{Duration, Instant};

pub(crate) fn wait_for_height_until(
    cluster: &mut Cluster,
    nodes: &[usize],
    height: i64,
    deadline: Instant,
) -> Result<()> {
    loop {
        super::verify_height_deadline::verify_height_deadline(cluster, nodes, height, deadline)?;
        let mut complete = true;
        for &index in nodes {
            let node = &mut cluster.nodes[index];
            if let Some(child) = &mut node.child {
                ensure!(
                    child.try_wait()?.is_none(),
                    "validator {index} exited before height {height}; failure categories {}; artifacts {}",
                    summarize_exited_validator(
                        &node.data,
                        &node.data.join("validator.stderr.log"),
                        child.id()
                    ),
                    cluster.artifact.display()
                );
            }
            super::verify_height_deadline::verify_height_deadline(
                cluster, nodes, height, deadline,
            )?;
            let native = node_height(cluster, index).unwrap_or(0);
            super::verify_height_deadline::verify_height_deadline(
                cluster, nodes, height, deadline,
            )?;
            let application = rpc_json(cluster.nodes[index].rpc, "/abci_info")
                .ok()
                .and_then(|value| {
                    value["response"]["last_block_height"]
                        .as_str()
                        .and_then(|text| text.parse::<i64>().ok())
                })
                .unwrap_or(0);
            super::verify_height_deadline::verify_height_deadline(
                cluster, nodes, height, deadline,
            )?;
            complete &= native >= height && application >= height;
        }
        super::verify_height_deadline::verify_height_deadline(cluster, nodes, height, deadline)?;
        if complete {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(150));
    }
}
