// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Varint-length-delimited protobuf framing used by CometBFT's socket transport.

mod read_abci_request;
mod read_protobuf_message;
mod write_abci_response;
mod write_protobuf_message;

pub use read_abci_request::read_abci_request;
pub use read_protobuf_message::read_protobuf_message;
pub use write_abci_response::write_abci_response;
pub use write_protobuf_message::write_protobuf_message;
