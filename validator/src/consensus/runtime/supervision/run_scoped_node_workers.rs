// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{consensus::runtime::types::RunningNode, development::engine::stop_engine};
use anyhow::Result;
use std::{os::unix::net::UnixListener, path::Path, sync::atomic::Ordering, thread::Scope};

pub(in crate::consensus::runtime) fn run_scoped_node_workers<'scope, 'env>(
    scope: &'scope Scope<'scope, 'env>,
    node: &'scope RunningNode,
    listener: UnixListener,
    signer_socket: &'scope Path,
    signal_runtime: tokio::runtime::Runtime,
) -> Result<()> {
    let application =
        scope.spawn(|| super::run_application_worker::run_application_worker(node, listener));
    let signer = scope.spawn(|| super::run_signer_worker::run_signer_worker(node, signer_socket));
    let outcome = signal_runtime
        .block_on(super::supervise_development_node::supervise_development_node(node));
    if let Err(error) = &outcome {
        let _ = crate::consensus::runtime::diagnostics::record_node_failure(node, error);
    }
    node.stop.store(true, Ordering::Release);
    let channels_stopped = super::shutdown_node_channels::shutdown_node_channels(node);
    let stopped = {
        let mut engine = node
            .engine
            .lock()
            .map_err(|_| anyhow::anyhow!("engine supervisor lock poisoned"))?;
        stop_engine(&mut engine)
    };
    let app_joined = application.join();
    let signer_joined = signer.join();
    outcome?;
    channels_stopped?;
    stopped?;
    app_joined.map_err(|_| anyhow::anyhow!("application channel worker panicked"))?;
    signer_joined.map_err(|_| anyhow::anyhow!("signer channel worker panicked"))?;
    Ok(())
}
