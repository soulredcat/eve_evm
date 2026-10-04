// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{CheckpointProxy, MAXIMUM_PROXY_CONNECTIONS};
use crate::cases::public_follower::PublicFollower;
use anyhow::{Result, ensure};
use std::{
    sync::mpsc::RecvTimeoutError,
    time::{Duration, Instant},
};

pub(super) fn wait_checkpoint_proxy_gate(
    proxy: &CheckpointProxy,
    follower: &mut PublicFollower,
) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(120);
    loop {
        ensure!(
            Instant::now() < deadline,
            "CHECKPOINT_INTERRUPTION_PROXY_GATE_DEADLINE"
        );
        ensure!(
            follower
                .child
                .as_mut()
                .ok_or_else(|| anyhow::anyhow!("CHECKPOINT_INTERRUPTION_CHILD_MISSING"))?
                .try_wait()?
                .is_none(),
            "CHECKPOINT_INTERRUPTION_BOOTSTRAP_EXITED"
        );
        match proxy.reached.recv_timeout(Duration::from_millis(100)) {
            Ok(connections) => {
                ensure!(
                    connections == MAXIMUM_PROXY_CONNECTIONS,
                    "CHECKPOINT_INTERRUPTION_GATE_CONNECTION"
                );
                return Ok(());
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return Err(anyhow::anyhow!("CHECKPOINT_INTERRUPTION_PROXY_EXITED"));
            }
        }
    }
}
