// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, EngineChannel, validate_engine_peer_liveness};
use std::io;

pub(in crate::consensus::transport) fn ensure_engine_channel(
    peer: &AuthenticatedEnginePeer,
    expected: EngineChannel,
) -> io::Result<()> {
    if peer.channel != expected {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "engine frame belongs to a different private channel",
        ));
    }
    validate_engine_peer_liveness(peer)
}
