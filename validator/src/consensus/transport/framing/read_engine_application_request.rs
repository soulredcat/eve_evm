// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::peer::{
        AuthenticatedEnginePeer, EngineChannel, engine_read_deadline, ensure_engine_channel,
        shutdown_engine_peer,
    },
    types::{EngineFrameReader, MAXIMUM_APPLICATION_FRAME_BYTES},
};
use eve_consensus_comet::wire::{framing::read_abci_request, tendermint::abci::Request};
use std::io;

/// Decode only the stream owned by this sealed peer; the caller polls the child again before dispatch.
pub(in crate::consensus) fn read_engine_application_request(
    peer: &mut AuthenticatedEnginePeer,
) -> io::Result<Request> {
    ensure_engine_channel(peer, EngineChannel::Application)?;
    let mut reader = EngineFrameReader {
        deadline: engine_read_deadline(peer)?,
        peer,
    };
    let result = read_abci_request(&mut reader, MAXIMUM_APPLICATION_FRAME_BYTES);
    if result.is_err() {
        let _ = shutdown_engine_peer(peer);
    }
    result
}
