// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::Cluster;
use crate::support::process::{signal_owned_process, wait_child};
use anyhow::Result;

impl Cluster {
    pub fn stop_node(&mut self, index: usize, crash: bool) -> Result<()> {
        let Some(mut child) = self.nodes[index].child.take() else {
            return Ok(());
        };
        let node = &mut self.nodes[index];
        if child.try_wait()?.is_none() {
            signal_owned_process(child.id(), if crash { "KILL" } else { "TERM" }, None)?;
        }
        if crash && let (Some(pid), Some(start)) = (node.engine_pid, node.engine_start) {
            signal_owned_process(pid, "KILL", Some((&self.comet, start)))?;
        }
        wait_child(&mut child)?;
        for entry in std::fs::read_dir(&node.data)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().ends_with(".log")
                && entry.metadata()?.len() <= 16 * 1_048_576
            {
                std::fs::copy(
                    entry.path(),
                    self.artifact.join(format!(
                        "engine-{index}-{}",
                        entry.file_name().to_string_lossy()
                    )),
                )?;
            }
        }
        self.proxies.unregister(index);
        node.engine_pid = None;
        node.engine_start = None;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        for index in 0..self.nodes.len() {
            self.stop_node(index, false)?;
        }
        Ok(())
    }
}

impl Drop for Cluster {
    fn drop(&mut self) {
        for index in 0..self.nodes.len() {
            if self.stop_node(index, false).is_err() {
                let _ = self.stop_node(index, true);
            }
        }
        self.proxies.shutdown();
        if self.retain_failed_namespace
            && let Some(namespace) = self.namespace.take()
        {
            let path = namespace.keep();
            let reference = serde_json::json!({
                "scope": "Failed C06 task-owned Linux namespace keeps its private 0700 root and 0600 signing seeds; never publish",
                "namespace": path,
                "validator_binary_sha256": self.binary_sha256,
                "comet_binary_sha256": self.comet_sha
            });
            if let Ok(bytes) = serde_json::to_vec_pretty(&reference) {
                let _ = std::fs::write(self.artifact.join("retained-namespace.json"), bytes);
            }
        }
    }
}
