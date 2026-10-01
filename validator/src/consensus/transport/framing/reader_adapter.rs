// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::peer::read_engine_peer_bytes, types::EngineFrameReader};
use std::io::{self, Read};

impl Read for EngineFrameReader<'_> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        read_engine_peer_bytes(self.peer, self.deadline, buffer)
    }
}
