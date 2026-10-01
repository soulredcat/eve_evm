// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    resolve_connection_owner::resolve_connection_owner,
    types::{Connection, ProxyState},
};
use anyhow::{Result, ensure};
use std::{
    net::{Shutdown, SocketAddr, TcpStream},
    sync::{Arc, atomic::Ordering},
    time::Duration,
};

pub(super) fn forward_connection(
    state: Arc<ProxyState>,
    target: usize,
    backend: SocketAddr,
    incoming: TcpStream,
    remote: SocketAddr,
) -> Result<()> {
    let source = resolve_connection_owner(&state, remote.port())?;
    let mut connections = state.connections.lock().unwrap();
    ensure!(
        connections.len() < 128,
        "bounded peer proxy connection capacity"
    );
    ensure!(
        !state.stop.load(Ordering::Acquire)
            && !(state.partition.load(Ordering::Acquire) && (source < 2) != (target < 2)),
        "partition rejects cross-group native socket"
    );
    let outgoing = TcpStream::connect_timeout(&backend, Duration::from_secs(2))?;
    incoming.set_nodelay(true)?;
    outgoing.set_nodelay(true)?;
    let id = state.next_connection.fetch_add(1, Ordering::Relaxed);
    connections.push(Connection {
        id,
        source,
        target,
        incoming: incoming.try_clone()?,
        outgoing: outgoing.try_clone()?,
    });
    let mut accepted = state.accepted.lock().unwrap();
    ensure!(accepted.len() < 4096, "bounded peer proxy edge evidence");
    accepted.push((source, target));
    drop(accepted);
    drop(connections);
    let mut upstream_input = incoming.try_clone()?;
    let mut upstream_output = outgoing.try_clone()?;
    let upstream = std::thread::spawn(move || {
        let _ = std::io::copy(&mut upstream_input, &mut upstream_output);
        let _ = upstream_output.shutdown(Shutdown::Both);
    });
    let mut downstream_input = outgoing;
    let mut downstream_output = incoming;
    let _ = std::io::copy(&mut downstream_input, &mut downstream_output);
    let _ = downstream_output.shutdown(Shutdown::Both);
    let _ = upstream.join();
    state
        .connections
        .lock()
        .unwrap()
        .retain(|connection| connection.id != id);
    Ok(())
}
