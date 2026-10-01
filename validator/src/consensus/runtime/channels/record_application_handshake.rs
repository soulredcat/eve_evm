// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::types::RunningNode;
use anyhow::Result;
use eve_consensus_comet::wire::tendermint::abci::{Response, response::Value};

/// Only a successfully written authenticated native response updates readiness.
pub(in crate::consensus::runtime) fn record_application_handshake(
    node: &RunningNode,
    response: &Response,
) -> Result<()> {
    let mut progress = node
        .handshake
        .lock()
        .map_err(|_| anyhow::anyhow!("readiness lock poisoned"))?;
    match &response.value {
        Some(Value::Info(info)) => {
            progress.info = Some((info.last_block_height, info.last_block_app_hash.clone()));
            progress.initialized |= info.last_block_height > 0;
        }
        Some(Value::InitChain(_)) => progress.initialized = true,
        _ => {}
    }
    Ok(())
}
