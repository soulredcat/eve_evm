// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    bootstrap::assemble_development_node,
    supervision::{bind_application_listener, cleanup_application_listener, start_node_workers},
    types::{ChannelShutdown, HandshakeProgress, RunningNode},
};
use crate::development::{config::DevelopmentValidatorConfig, engine::start_engine};
use anyhow::{Context, Result};
use std::sync::{Mutex, atomic::AtomicBool};

/// Run only this owned classical development child and its authenticated foreground actors.
pub fn run_development_validator(config: DevelopmentValidatorConfig) -> Result<()> {
    let assembly = assemble_development_node(config).context("NODE_ASSEMBLY_FAILED")?;
    let suffix = std::process::id().to_string();
    let application_socket = assembly
        .config
        .data
        .join(format!("application-{suffix}.sock"));
    let signer_socket = assembly.config.data.join(format!("signer-{suffix}.sock"));
    let listener =
        bind_application_listener(&application_socket).context("NODE_APPLICATION_BIND_FAILED")?;
    let engine = start_engine(
        &assembly.config,
        &assembly.config.data.join("engine"),
        &application_socket,
        &signer_socket,
        assembly.digest,
    )
    .context("NODE_ENGINE_LAUNCH_FAILED")?;
    let node = RunningNode {
        assembly,
        engine: Mutex::new(engine),
        stop: AtomicBool::new(false),
        handshake: Mutex::new(HandshakeProgress::default()),
        failure: Mutex::new(None),
        shutdown: Mutex::new(ChannelShutdown::default()),
        signer_requested: Mutex::new(None),
    };
    // Lease remains owned by the foreground assembly until both workers and child stop.
    let _lease = &node.assembly.lease;
    let outcome = start_node_workers(&node, listener.listener.try_clone()?, &signer_socket);
    let cleaned = cleanup_application_listener(&listener);
    outcome?;
    cleaned
}
