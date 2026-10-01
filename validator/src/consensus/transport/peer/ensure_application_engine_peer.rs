// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedEnginePeer, EngineChannel, ensure_engine_channel};
use std::io;

pub(in crate::consensus) fn ensure_application_engine_peer(
    peer: &AuthenticatedEnginePeer,
) -> io::Result<()> {
    ensure_engine_channel(peer, EngineChannel::Application)
}
