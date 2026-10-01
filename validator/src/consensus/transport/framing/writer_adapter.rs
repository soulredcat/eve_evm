// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::peer::{flush_engine_peer, write_engine_peer_bytes},
    types::EngineFrameWriter,
};
use std::io::{self, Write};

impl Write for EngineFrameWriter<'_> {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        write_engine_peer_bytes(self.peer, self.deadline, buffer)
    }
    fn flush(&mut self) -> io::Result<()> {
        flush_engine_peer(self.peer)
    }
}
