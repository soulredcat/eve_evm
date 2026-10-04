// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded own-chain development downloads. Transport data never grants finality.
#[cfg(test)]
extern crate self as eve_sync_client;
mod checkpoints;
mod delta;
mod http;
mod import;
mod native;
mod required_native_rpc_reservation;
mod types;
pub use checkpoints::{
    CheckpointWitnessHeights, DownloadedCheckpointResponse, DownloadedCheckpointWitness,
    downloaded_checkpoint_response, downloaded_checkpoint_witness_wire, fetch_checkpoint_response,
    fetch_checkpoint_response_before, fetch_checkpoint_witness, fetch_checkpoint_witness_before,
};
pub use http::{request_native_json, request_native_json_before};
pub use native::{fetch_native_frame, fetch_native_frame_before};
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
