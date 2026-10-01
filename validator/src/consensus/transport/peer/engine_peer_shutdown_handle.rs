// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, types::EnginePeerShutdown, validate_engine_peer_liveness};
use std::io;

pub(in crate::consensus) fn engine_peer_shutdown_handle(
    peer: &AuthenticatedEnginePeer,
) -> io::Result<EnginePeerShutdown> {
    validate_engine_peer_liveness(peer)?;
    #[cfg(unix)]
    {
        Ok(EnginePeerShutdown {
            stream: peer.stream.try_clone()?,
        })
    }
    #[cfg(not(unix))]
    {
        let _ = peer;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "engine shutdown capability requires Unix sockets",
        ))
    }
}
