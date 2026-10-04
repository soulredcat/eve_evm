// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::io::{Error, ErrorKind};

pub(super) fn sanitize_rpc_io_error(error: Error) -> Error {
    let category = match error.kind() {
        ErrorKind::WouldBlock | ErrorKind::TimedOut => "SYNC_RPC_IO_TIMEOUT",
        ErrorKind::UnexpectedEof
        | ErrorKind::BrokenPipe
        | ErrorKind::ConnectionReset
        | ErrorKind::ConnectionAborted
        | ErrorKind::NotConnected => "SYNC_RPC_IO_DISCONNECTED",
        ErrorKind::ConnectionRefused => "SYNC_RPC_IO_REFUSED",
        ErrorKind::PermissionDenied => "SYNC_RPC_IO_PERMISSION",
        _ => "SYNC_RPC_IO_OTHER",
    };
    Error::new(error.kind(), category)
}
