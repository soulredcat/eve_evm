// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::EnginePeerShutdown;
use std::io;

pub(in crate::consensus) fn shutdown_engine_peer_handle(
    handle: &EnginePeerShutdown,
) -> io::Result<()> {
    #[cfg(unix)]
    {
        handle.stream.shutdown(std::net::Shutdown::Both)
    }
    #[cfg(not(unix))]
    {
        let _ = handle;
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "engine shutdown capability requires Unix sockets",
        ))
    }
}
