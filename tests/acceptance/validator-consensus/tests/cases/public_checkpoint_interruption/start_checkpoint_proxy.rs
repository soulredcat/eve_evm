// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    run_checkpoint_proxy::run_checkpoint_proxy,
    types::{CheckpointProxy, ProxyControl},
};
use anyhow::Result;
use std::{
    net::{SocketAddr, TcpListener},
    sync::{Arc, atomic::AtomicBool, mpsc},
    time::{Duration, Instant},
};

/// One owned loopback listener forwards opaque bytes and gates exactly connection three.
pub(super) fn start_checkpoint_proxy(upstream: SocketAddr) -> Result<CheckpointProxy> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    let address = listener.local_addr()?;
    let (reached_tx, reached) = mpsc::sync_channel(1);
    let (release, release_rx) = mpsc::sync_channel(1);
    let cancelled = Arc::new(AtomicBool::new(false));
    let control = ProxyControl {
        reached: reached_tx,
        release: release_rx,
        cancelled: Arc::clone(&cancelled),
        deadline: Instant::now() + Duration::from_secs(180),
    };
    let thread = std::thread::spawn(move || run_checkpoint_proxy(listener, upstream, control));
    Ok(CheckpointProxy {
        address,
        reached,
        release,
        cancelled,
        thread: Some(thread),
    })
}
