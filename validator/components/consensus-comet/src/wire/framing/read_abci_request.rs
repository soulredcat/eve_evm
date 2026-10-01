// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::read_protobuf_message;
use crate::wire::tendermint::abci::Request;
use std::io::{self, Read};

/// Read one bounded ABCI request without allocating an untrusted announced size.
pub fn read_abci_request<R: Read>(
    reader: &mut R,
    maximum_message_bytes: usize,
) -> io::Result<Request> {
    read_protobuf_message(reader, maximum_message_bytes)
}
