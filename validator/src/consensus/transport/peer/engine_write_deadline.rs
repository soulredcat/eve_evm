// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AuthenticatedEnginePeer;
use std::{io, time::Instant};

pub(in crate::consensus::transport) fn engine_write_deadline(
    peer: &AuthenticatedEnginePeer,
) -> io::Result<Instant> {
    Instant::now()
        .checked_add(peer.write_timeout)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "engine write deadline overflow",
            )
        })
}
