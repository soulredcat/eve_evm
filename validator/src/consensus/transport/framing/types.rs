// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::peer::AuthenticatedEnginePeer;
use std::time::Instant;

pub(super) const MAXIMUM_APPLICATION_FRAME_BYTES: usize = 4 * 1_048_576 + 65_536;
pub(super) const MAXIMUM_SIGNER_FRAME_BYTES: usize = 65_536;

pub(super) struct EngineFrameReader<'a> {
    pub peer: &'a AuthenticatedEnginePeer,
    pub deadline: Instant,
}

pub(super) struct EngineFrameWriter<'a> {
    pub peer: &'a AuthenticatedEnginePeer,
    pub deadline: Instant,
}
