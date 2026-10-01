// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    consensus::{
        runtime::types::RunningNode,
        transport::peer::{AuthenticatedEnginePeer, validate_engine_peer_child},
    },
    development::engine::engine_child,
};
use anyhow::Result;

pub(in crate::consensus::runtime) fn poll_node_channel(
    node: &RunningNode,
    peer: &AuthenticatedEnginePeer,
) -> Result<()> {
    let mut engine = node
        .engine
        .lock()
        .map_err(|_| anyhow::anyhow!("engine supervisor lock poisoned"))?;
    validate_engine_peer_child(peer, engine_child(&mut engine)?)
}
