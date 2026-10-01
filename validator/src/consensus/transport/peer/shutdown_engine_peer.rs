// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AuthenticatedEnginePeer;
use std::io;

pub(in crate::consensus) fn shutdown_engine_peer(peer: &AuthenticatedEnginePeer) -> io::Result<()> {
    #[cfg(unix)]
    {
        peer.stream.shutdown(std::net::Shutdown::Both)
    }
    #[cfg(not(unix))]
    {
        let _ = peer;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "engine transport requires Unix sockets",
        ))
    }
}
