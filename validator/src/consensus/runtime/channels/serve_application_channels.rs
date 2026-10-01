// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    runtime::types::RunningNode,
    transport::peer::{EngineChannel, engine_peer_read_ready},
};
use anyhow::{Result, ensure};
use std::{io::ErrorKind, os::unix::net::UnixListener, sync::atomic::Ordering, time::Duration};

/// One bounded application actor serializes native channels without holding process locks for I/O.
pub(in crate::consensus::runtime) fn serve_application_channels(
    node: &RunningNode,
    listener: UnixListener,
) -> Result<()> {
    listener.set_nonblocking(true)?;
    let mut peers = Vec::new();
    while !node.stop.load(Ordering::Acquire) {
        match listener.accept() {
            Ok((stream, _)) => {
                ensure!(
                    peers.len() < 4,
                    "native application connection capacity exceeded"
                );
                if let Ok(peer) = super::authenticate_node_channel::authenticate_node_channel(
                    node,
                    stream,
                    EngineChannel::Application,
                ) {
                    peers.push(peer);
                }
            }
            Err(error) if error.kind() == ErrorKind::WouldBlock => {}
            Err(error) => return Err(error.into()),
        }
        for peer in &mut peers {
            if node.stop.load(Ordering::Acquire) {
                break;
            }
            super::poll_node_channel::poll_node_channel(node, peer)?;
            if engine_peer_read_ready(peer, Duration::from_millis(10))? {
                super::exchange_application_frame::exchange_application_frame(node, peer)?;
            }
        }
        if peers.is_empty() {
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    Ok(())
}
