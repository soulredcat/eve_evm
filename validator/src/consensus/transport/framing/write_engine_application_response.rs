// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::peer::{
        AuthenticatedEnginePeer, EngineChannel, engine_write_deadline, ensure_engine_channel,
        shutdown_engine_peer,
    },
    types::{EngineFrameWriter, MAXIMUM_APPLICATION_FRAME_BYTES},
};
use eve_consensus_comet::wire::{framing::write_abci_response, tendermint::abci::Response};
use std::io;

pub(in crate::consensus) fn write_engine_application_response(
    peer: &mut AuthenticatedEnginePeer,
    response: &Response,
) -> io::Result<()> {
    ensure_engine_channel(peer, EngineChannel::Application)?;
    let mut writer = EngineFrameWriter {
        deadline: engine_write_deadline(peer)?,
        peer,
    };
    let result = write_abci_response(&mut writer, response, MAXIMUM_APPLICATION_FRAME_BYTES);
    if result.is_err() {
        let _ = shutdown_engine_peer(peer);
    }
    result
}
