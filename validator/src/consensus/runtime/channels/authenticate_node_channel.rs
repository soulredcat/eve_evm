// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    consensus::{
        runtime::types::RunningNode,
        transport::peer::{
            AuthenticatedEnginePeer, EngineChannel, authenticate_engine_peer,
            engine_peer_shutdown_handle,
        },
    },
    development::engine::engine_authentication_context,
};
use anyhow::Result;
use std::{os::unix::net::UnixStream, time::Duration};

pub(in crate::consensus::runtime) fn authenticate_node_channel(
    node: &RunningNode,
    stream: UnixStream,
    channel: EngineChannel,
) -> Result<AuthenticatedEnginePeer> {
    let mut engine = node
        .engine
        .lock()
        .map_err(|_| anyhow::anyhow!("engine supervisor lock poisoned"))?;
    let (child, image) = engine_authentication_context(&mut engine)?;
    let peer = authenticate_engine_peer(
        stream,
        child,
        image,
        channel,
        Duration::from_secs(60),
        Duration::from_secs(10),
    )?;
    let handle = engine_peer_shutdown_handle(&peer)?;
    let mut shutdown = node
        .shutdown
        .lock()
        .map_err(|_| anyhow::anyhow!("channel shutdown lock poisoned"))?;
    match channel {
        EngineChannel::Application => {
            anyhow::ensure!(
                shutdown.application.len() < 4,
                "authenticated application shutdown capacity exceeded"
            );
            shutdown.application.push(handle);
        }
        EngineChannel::Signer => shutdown.signer = Some(handle),
    }
    Ok(peer)
}
