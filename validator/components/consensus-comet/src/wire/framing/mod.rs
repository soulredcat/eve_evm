// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Varint-length-delimited protobuf framing used by CometBFT's socket transport.

mod read_abci_request;
mod write_abci_response;

pub use read_abci_request::read_abci_request;
pub use write_abci_response::write_abci_response;
