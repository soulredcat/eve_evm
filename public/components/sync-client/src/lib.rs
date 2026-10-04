// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded own-chain development downloads. Transport data never grants finality.
#[cfg(test)]
extern crate self as eve_sync_client;
mod delta;
mod http;
mod import;
mod native;
mod required_native_rpc_reservation;
mod types;
pub use http::request_native_json;
pub use native::fetch_native_frame;
pub use required_native_rpc_reservation::required_native_rpc_reservation;
pub use types::NativeRpcConfig;

pub use delta::{
    DownloadedStateDelta, fetch_state_delta_bytes, required_state_delta_download_reservation,
};

pub use import::{
    DownloadedAuthenticatedImport, downloaded_import_target, downloaded_import_wire,
    fetch_authenticated_import_wire, required_authenticated_import_download_reservation,
};

mod validate_native_rpc_config;
pub use validate_native_rpc_config::validate_native_rpc_config;
