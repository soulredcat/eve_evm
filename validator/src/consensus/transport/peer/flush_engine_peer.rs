// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, validate_linux_transport};
use std::io::{self, Write};

pub(in crate::consensus::transport) fn flush_engine_peer(
    peer: &AuthenticatedEnginePeer,
) -> io::Result<()> {
    validate_linux_transport()?;
    #[cfg(unix)]
    {
        (&peer.stream).flush()
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
