// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AuthenticatedEnginePeer;

pub(in crate::consensus) fn engine_peer_pid(peer: &AuthenticatedEnginePeer) -> u32 {
    peer.pid
}
