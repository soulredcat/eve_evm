// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod read_chunked_body;
mod read_rpc_bytes;
mod read_rpc_exact;
mod read_rpc_line;
mod read_rpc_response;
mod rpc_json;
mod sanitize_rpc_io_error;
mod types;
mod write_rpc_bytes;
pub(crate) use read_rpc_response::read_rpc_response;
pub(crate) use rpc_json::rpc_json;
pub(crate) use write_rpc_bytes::write_rpc_bytes;
#[cfg(test)]
mod response;
#[cfg(test)]
mod tests;
