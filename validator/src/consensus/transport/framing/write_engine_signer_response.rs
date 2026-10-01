// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::peer::{
        AuthenticatedEnginePeer, EngineChannel, engine_write_deadline, ensure_engine_channel,
        shutdown_engine_peer,
    },
    types::{EngineFrameWriter, MAXIMUM_SIGNER_FRAME_BYTES},
};
use eve_consensus_comet::wire::{framing::write_protobuf_message, tendermint::privval::Message};
use std::io;

pub(in crate::consensus) fn write_engine_signer_response(
    peer: &mut AuthenticatedEnginePeer,
    response: &Message,
) -> io::Result<()> {
    ensure_engine_channel(peer, EngineChannel::Signer)?;
    let mut writer = EngineFrameWriter {
        deadline: engine_write_deadline(peer)?,
        peer,
    };
    let result = write_protobuf_message(&mut writer, response, MAXIMUM_SIGNER_FRAME_BYTES);
    if result.is_err() {
        let _ = shutdown_engine_peer(peer);
    }
    result
}
