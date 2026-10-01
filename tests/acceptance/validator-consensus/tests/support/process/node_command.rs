// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Cluster;
use std::process::Command;
pub(crate) fn node_command(
    cluster: &Cluster,
    index: usize,
    operation: &str,
    peers: &str,
) -> Command {
    let node = &cluster.nodes[index];
    let mut command = Command::new(&cluster.binary);
    command
        .args([operation, "--acknowledge-unsafe-development", "--genesis"])
        .arg(&cluster.genesis_path)
        .arg("--data")
        .arg(&node.data)
        .arg("--signing-seed")
        .arg(&node.seed)
        .arg("--comet-binary")
        .arg(&cluster.comet)
        .arg("--comet-sha256")
        .arg(&cluster.comet_sha)
        .arg("--rpc-address")
        .arg(node.rpc.to_string())
        .arg("--p2p-address")
        .arg(node.p2p.to_string())
        .arg("--advertised-p2p-address")
        .arg(node.advertised.to_string())
        .arg("--persistent-peers")
        .arg(peers)
        .args(["--zone-id", "1"])
        .env("GOMAXPROCS", "2");
    if let Some(path) = &cluster.fixture_path {
        command.arg("--acceptance-fixture").arg(path);
    }
    command
}
