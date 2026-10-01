// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{Cluster, wait_for_height};
use crate::support::process::{identify_engine, node_command};
use anyhow::{Context, Result, ensure};
use std::{fs::OpenOptions, process::Stdio};

impl Cluster {
    pub(super) fn initialize(&mut self) -> Result<()> {
        for index in 0..self.nodes.len() {
            let output = node_command(self, index, "init-dev", "").output()?;
            std::fs::write(
                self.artifact.join(format!("init-{index}.stderr")),
                &output.stderr,
            )?;
            ensure!(
                output.status.success(),
                "actual validator initialization failed at node {index}; raw output at {}",
                self.artifact.display()
            );
            let identity: serde_json::Value = serde_json::from_slice(&output.stdout)?;
            self.nodes[index].node_id = identity["node_id"]
                .as_str()
                .context("native node id missing")?
                .to_owned();
            let chain = identity["chain_id"]
                .as_str()
                .context("native chain id missing")?;
            ensure!(
                self.chain_id.is_empty() || self.chain_id == chain,
                "node chain mismatch"
            );
            self.chain_id = chain.to_owned();
            ensure!(
                identity["genesis_hash"] == hex::encode(self.genesis.target.identity.genesis.0),
                "node genesis identity mismatch"
            );
        }
        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        self.reservations.clear();
        self.proxies
            .start(self.nodes.iter().map(|node| node.p2p).collect())?;
        for index in 0..self.nodes.len() {
            self.spawn_node(index)?;
        }
        for index in 0..self.nodes.len() {
            self.register_engine(index)?;
        }
        wait_for_height(self, &[0, 1, 2, 3], 2)?;
        Ok(())
    }

    pub fn restart(&mut self, index: usize) -> Result<()> {
        self.spawn_node(index)?;
        self.register_engine(index)
    }

    fn spawn_node(&mut self, index: usize) -> Result<()> {
        ensure!(self.nodes[index].child.is_none(), "node already running");
        let peers = self
            .nodes
            .iter()
            .enumerate()
            .filter(|(other, _)| *other != index)
            .map(|(_, node)| format!("{}@{}", node.node_id, node.advertised))
            .collect::<Vec<_>>()
            .join(",");
        let mut command = node_command(self, index, "serve-dev", &peers);
        command.stdout(Stdio::from(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.artifact.join(format!("node-{index}.stdout")))?,
        ));
        command.stderr(Stdio::from(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(self.artifact.join(format!("node-{index}.stderr")))?,
        ));
        let child = command.spawn()?;
        self.nodes[index].child = Some(child);
        Ok(())
    }

    fn register_engine(&mut self, index: usize) -> Result<()> {
        let parent_pid = self.nodes[index]
            .child
            .as_ref()
            .context("owned validator missing")?
            .id();
        let (engine_pid, start) = identify_engine(
            parent_pid,
            &self.comet,
            &self.nodes[index].data.join("engine"),
        )?;
        self.proxies.register(index, engine_pid);
        self.nodes[index].engine_pid = Some(engine_pid);
        self.nodes[index].engine_start = Some(start);
        Ok(())
    }
}
