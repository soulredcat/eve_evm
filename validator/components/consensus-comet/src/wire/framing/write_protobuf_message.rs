// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use prost::Message;
use std::io::{self, Write};

/// Serialize and flush one native message only after checking its encoded body length.
pub fn write_protobuf_message<W: Write, M: Message>(
    writer: &mut W,
    message: &M,
    maximum_message_bytes: usize,
) -> io::Result<()> {
    if message.encoded_len() > maximum_message_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ABCI size limit",
        ));
    }
    writer.write_all(&message.encode_length_delimited_to_vec())?;
    writer.flush()
}
