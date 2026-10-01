// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, validate_linux_transport};
use std::{
    io::{self, Read},
    time::Instant,
};

pub(in crate::consensus::transport) fn read_engine_peer_bytes(
    peer: &AuthenticatedEnginePeer,
    deadline: Instant,
    buffer: &mut [u8],
) -> io::Result<usize> {
    validate_linux_transport()?;
    let remaining = deadline
        .checked_duration_since(Instant::now())
        .filter(|duration| !duration.is_zero())
        .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "engine frame read deadline"))?;
    #[cfg(unix)]
    {
        peer.stream.set_read_timeout(Some(remaining))?;
        (&peer.stream).read(buffer)
    }
    #[cfg(not(unix))]
    {
        let _ = (peer, remaining, buffer);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "engine transport requires Unix sockets",
        ))
    }
}
