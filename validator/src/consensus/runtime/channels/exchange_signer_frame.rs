// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    runtime::types::RunningNode,
    transport::{
        dispatch::dispatch_signer_request,
        framing::{read_engine_signer_request, write_engine_signer_response},
        peer::AuthenticatedEnginePeer,
    },
};
use anyhow::{Context, Result};
use eve_state::development_state_budget;

pub(in crate::consensus::runtime) fn exchange_signer_frame(
    node: &RunningNode,
    peer: &mut AuthenticatedEnginePeer,
) -> Result<()> {
    super::poll_node_channel::poll_node_channel(node, peer)?;
    let request = read_engine_signer_request(peer).context("signer_frame_read")?;
    super::poll_node_channel::poll_node_channel(node, peer)?;
    crate::consensus::runtime::diagnostics::record_signer_request_position(node, &request)?;
    let response = {
        let mut signer = node
            .assembly
            .signer
            .lock()
            .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
        dispatch_signer_request(
            &mut signer,
            &node.assembly.registry,
            peer,
            request,
            &development_state_budget(),
            64 * 1_048_576,
        )
        .context("signer_dispatch")?
    };
    super::poll_node_channel::poll_node_channel(node, peer)?;
    write_engine_signer_response(peer, &response).context("signer_frame_write")?;
    super::poll_node_channel::poll_node_channel(node, peer)?;
    super::record_signer_handshake::record_signer_handshake(node, &response)
}
