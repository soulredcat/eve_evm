// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::io::{self, Read};

use prost::Message;

use crate::wire::tendermint::abci::Request;

/// Read one bounded ABCI request without allocating an untrusted announced size.
pub fn read_abci_request<R: Read>(
    reader: &mut R,
    maximum_message_bytes: usize,
) -> io::Result<Request> {
    let mut length = 0_u64;
    for index in 0..10 {
        let mut byte = [0_u8; 1];
        reader.read_exact(&mut byte)?;
        if index == 9 && byte[0] > 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "varint overflow",
            ));
        }
        length |= u64::from(byte[0] & 0x7f) << (7 * index);
        if byte[0] & 0x80 == 0 {
            if index > 0 && byte[0] == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "overlong varint",
                ));
            }
            let length = usize::try_from(length)
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "length overflow"))?;
            if length > maximum_message_bytes {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "ABCI size limit",
                ));
            }
            let mut bytes = vec![0; length];
            reader.read_exact(&mut bytes)?;
            return Request::decode(bytes.as_slice())
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error));
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "unterminated varint",
    ))
}
