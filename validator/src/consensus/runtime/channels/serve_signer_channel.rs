// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{runtime::types::RunningNode, transport::peer::engine_peer_read_ready};
use anyhow::Result;
use std::{path::Path, sync::atomic::Ordering, time::Duration};

/// The native engine listens; EVE dials that actual process-owned Unix signer endpoint.
pub(in crate::consensus::runtime) fn serve_signer_channel(
    node: &RunningNode,
    socket: &Path,
) -> Result<()> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let Some(mut peer) =
        super::reconnect_signer_channel::reconnect_signer_channel(node, &runtime, socket)?
    else {
        return Ok(());
    };
    while !node.stop.load(Ordering::Acquire) {
        super::poll_node_channel::poll_node_channel(node, &peer)?;
        if engine_peer_read_ready(&peer, Duration::from_millis(100))?
            && let Err(error) = super::exchange_signer_frame::exchange_signer_frame(node, &mut peer)
        {
            let reconnectable = error.downcast_ref::<std::io::Error>().is_some_and(|error| {
                matches!(
                    error.kind(),
                    std::io::ErrorKind::UnexpectedEof
                        | std::io::ErrorKind::ConnectionReset
                        | std::io::ErrorKind::ConnectionAborted
                        | std::io::ErrorKind::BrokenPipe
                )
            });
            if !reconnectable {
                return Err(error);
            }
            let Some(reconnected) =
                super::reconnect_signer_channel::reconnect_signer_channel(node, &runtime, socket)?
            else {
                return Ok(());
            };
            peer = reconnected;
        }
    }
    Ok(())
}
