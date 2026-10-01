// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    runtime::types::RunningNode,
    transport::{
        dispatch::dispatch_application_request,
        framing::{read_engine_application_request, write_engine_application_response},
        peer::AuthenticatedEnginePeer,
    },
};
use anyhow::{Context, Result};

pub(in crate::consensus::runtime) fn exchange_application_frame(
    node: &RunningNode,
    peer: &mut AuthenticatedEnginePeer,
) -> Result<()> {
    super::poll_node_channel::poll_node_channel(node, peer)?;
    let request = read_engine_application_request(peer).context("application_frame_read")?;
    super::poll_node_channel::poll_node_channel(node, peer)?;
    let response = {
        let mut application = node
            .assembly
            .application
            .lock()
            .map_err(|_| anyhow::anyhow!("application actor lock poisoned"))?;
        dispatch_application_request(&mut application, peer, request)
            .context("application_dispatch")?
    };
    super::poll_node_channel::poll_node_channel(node, peer)?;
    write_engine_application_response(peer, &response).context("application_frame_write")?;
    super::poll_node_channel::poll_node_channel(node, peer)?;
    super::record_application_handshake::record_application_handshake(node, &response)
}
