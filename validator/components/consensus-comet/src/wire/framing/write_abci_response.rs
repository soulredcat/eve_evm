// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::write_protobuf_message;
use crate::wire::tendermint::abci::Response;
use std::io::{self, Write};

/// Write one bounded varint-length-delimited response and flush the connection.
pub fn write_abci_response<W: Write>(
    writer: &mut W,
    response: &Response,
    maximum_message_bytes: usize,
) -> io::Result<()> {
    write_protobuf_message(writer, response, maximum_message_bytes)
}
