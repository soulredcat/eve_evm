// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    forward_checkpoint_connection::forward_checkpoint_connection,
    types::{MAXIMUM_PROXY_CONNECTIONS, ProxyControl},
};
use anyhow::{Result, ensure};
use std::{
    io::ErrorKind,
    net::{Shutdown, SocketAddr, TcpListener},
    sync::{atomic::Ordering, mpsc::RecvTimeoutError},
    time::{Duration, Instant},
};

/// This proxy never parses or constructs protocol messages. All accepted data remains opaque.
pub(super) fn run_checkpoint_proxy(
    listener: TcpListener,
    upstream: SocketAddr,
    control: ProxyControl,
) -> Result<usize> {
    let mut connections = 0;
    while connections < MAXIMUM_PROXY_CONNECTIONS {
        if control.cancelled.load(Ordering::Acquire) {
            return Ok(connections);
        }
        ensure!(
            Instant::now() < control.deadline,
            "CHECKPOINT_PROXY_ACCEPT_DEADLINE"
        );
        let stream = match listener.accept() {
            Ok((stream, peer)) => {
                ensure!(peer.ip().is_loopback(), "CHECKPOINT_PROXY_NONLOCAL_PEER");
                stream
            }
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(error) => return Err(error.into()),
        };
        connections += 1;
        if connections == MAXIMUM_PROXY_CONNECTIONS {
            control.reached.try_send(connections)?;
            loop {
                if control.cancelled.load(Ordering::Acquire) {
                    let _ = stream.shutdown(Shutdown::Both);
                    return Ok(connections);
                }
                ensure!(
                    Instant::now() < control.deadline,
                    "CHECKPOINT_PROXY_GATE_DEADLINE"
                );
                match control.release.recv_timeout(Duration::from_millis(100)) {
                    Ok(()) => break,
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => return Ok(connections),
                }
            }
        }
        if control.cancelled.load(Ordering::Acquire) {
            let _ = stream.shutdown(Shutdown::Both);
            return Ok(connections);
        }
        forward_checkpoint_connection(stream, upstream, &control.cancelled, control.deadline)?;
    }
    Ok(connections)
}
