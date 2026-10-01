// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{runtime::types::RunningNode, transport::peer::shutdown_engine_peer_handle};
use anyhow::Result;

pub(in crate::consensus::runtime) fn shutdown_node_channels(node: &RunningNode) -> Result<()> {
    let handles = node
        .shutdown
        .lock()
        .map_err(|_| anyhow::anyhow!("channel shutdown lock poisoned"))?;
    for handle in handles.application.iter().chain(handles.signer.iter()) {
        shutdown_engine_peer_handle(handle)?;
    }
    Ok(())
}
