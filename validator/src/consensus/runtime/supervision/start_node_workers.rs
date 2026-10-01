// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::types::RunningNode;
use anyhow::Result;
use std::{os::unix::net::UnixListener, path::Path};

pub(in crate::consensus::runtime) fn start_node_workers(
    node: &RunningNode,
    listener: UnixListener,
    signer_socket: &Path,
) -> Result<()> {
    let signal_runtime = tokio::runtime::Runtime::new()?;
    std::thread::scope(|scope| {
        super::run_scoped_node_workers::run_scoped_node_workers(
            scope,
            node,
            listener,
            signer_socket,
            signal_runtime,
        )
    })
}
