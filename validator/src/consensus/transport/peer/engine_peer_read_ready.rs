// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, validate_engine_peer_liveness};
use std::{
    io,
    time::{Duration, Instant},
};

/// I/O readiness only: false means idle; true means bytes/EOF without consuming a frame prefix.
pub(in crate::consensus) fn engine_peer_read_ready(
    peer: &AuthenticatedEnginePeer,
    timeout: Duration,
) -> io::Result<bool> {
    if timeout > Duration::from_secs(1) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "engine idle poll limit",
        ));
    }
    validate_engine_peer_liveness(peer)?;
    let deadline = Instant::now().checked_add(timeout).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "engine idle deadline overflow")
    })?;
    #[cfg(target_os = "linux")]
    loop {
        match rustix::net::recv(
            &peer.stream,
            &mut [0; 1],
            rustix::net::RecvFlags::PEEK | rustix::net::RecvFlags::DONTWAIT,
        ) {
            Ok(_) => return Ok(true),
            Err(error) if error == rustix::io::Errno::AGAIN => {
                if Instant::now() >= deadline {
                    return Ok(false);
                }
                validate_engine_peer_liveness(peer)?;
                std::thread::sleep(
                    Duration::from_millis(5)
                        .min(deadline.saturating_duration_since(Instant::now())),
                );
            }
            Err(error) => return Err(error.into()),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (peer, deadline);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "engine idle polling requires Linux",
        ))
    }
}
