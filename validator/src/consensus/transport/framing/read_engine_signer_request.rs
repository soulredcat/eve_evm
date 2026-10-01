// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    super::peer::{
        AuthenticatedEnginePeer, EngineChannel, engine_read_deadline, ensure_engine_channel,
        shutdown_engine_peer,
    },
    types::{EngineFrameReader, MAXIMUM_SIGNER_FRAME_BYTES},
};
use eve_consensus_comet::wire::{framing::read_protobuf_message, tendermint::privval::Message};
use std::io;

pub(in crate::consensus) fn read_engine_signer_request(
    peer: &mut AuthenticatedEnginePeer,
) -> io::Result<Message> {
    ensure_engine_channel(peer, EngineChannel::Signer)?;
    let mut reader = EngineFrameReader {
        deadline: engine_read_deadline(peer)?,
        peer,
    };
    let result = read_protobuf_message(&mut reader, MAXIMUM_SIGNER_FRAME_BYTES);
    if result.is_err() {
        let _ = shutdown_engine_peer(peer);
    }
    result
}
