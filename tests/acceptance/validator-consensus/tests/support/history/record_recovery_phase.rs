// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Cluster, rpc::rpc_json};
use anyhow::Result;
use std::{io::Write, time::SystemTime};

pub(crate) fn record_recovery_phase(cluster: &Cluster, phase: &str) -> Result<()> {
    let nodes: Vec<_> = cluster.nodes.iter().enumerate().map(|(index, node)| serde_json::json!({
        "index": index,
        "rust_pid": node.child.as_ref().map(std::process::Child::id),
        "native_pid": node.engine_pid,
        "native_start_identity": node.engine_start,
        "native_status": if node.child.is_some() { rpc_json(node.rpc, "/status").map_err(|error| error.to_string()) } else { Ok(serde_json::Value::Null) },
        "application_info": if node.child.is_some() { rpc_json(node.rpc, "/abci_info").map_err(|error| error.to_string()) } else { Ok(serde_json::Value::Null) }
    })).collect();
    let record = serde_json::json!({
        "phase": phase,
        "unix_milliseconds": SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis(),
        "validator_binary_sha256": cluster.binary_sha256,
        "comet_binary_sha256": cluster.comet_sha,
        "nodes": nodes
    });
    let mut output = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(cluster.artifact.join("recovery-phases.jsonl"))?;
    output.write_all(&serde_json::to_vec(&record)?)?;
    output.write_all(b"\n")?;
    output.sync_all()?;
    Ok(())
}
