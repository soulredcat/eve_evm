use std::io::{self, Write};

use prost::Message;

use crate::wire::tendermint::abci::Response;

/// Write one bounded varint-length-delimited response and flush the connection.
pub fn write_abci_response<W: Write>(
    writer: &mut W,
    response: &Response,
    maximum_message_bytes: usize,
) -> io::Result<()> {
    if response.encoded_len() > maximum_message_bytes {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "ABCI size limit",
        ));
    }
    writer.write_all(&response.encode_length_delimited_to_vec())?;
    writer.flush()
}
