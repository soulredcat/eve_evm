// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    consensus::{
        runtime::types::RunningNode,
        transport::peer::{AuthenticatedEnginePeer, EngineChannel},
    },
    development::engine::engine_child,
};
use anyhow::{Result, ensure};
use std::{
    path::Path,
    sync::atomic::Ordering,
    time::{Duration, Instant},
};

pub(in crate::consensus::runtime) fn reconnect_signer_channel(
    node: &RunningNode,
    runtime: &tokio::runtime::Runtime,
    socket: &Path,
) -> Result<Option<AuthenticatedEnginePeer>> {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if node.stop.load(Ordering::Acquire) {
            return Ok(None);
        }
        ensure!(
            Instant::now() < deadline,
            "native signer reconnect deadline exceeded"
        );
        {
            let mut engine = node
                .engine
                .lock()
                .map_err(|_| anyhow::anyhow!("engine supervisor lock poisoned"))?;
            engine_child(&mut engine)?;
        }
        if let Ok(stream) = super::connect_signer_socket::connect_signer_socket(runtime, socket) {
            return Ok(Some(
                super::authenticate_node_channel::authenticate_node_channel(
                    node,
                    stream,
                    EngineChannel::Signer,
                )?,
            ));
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}
