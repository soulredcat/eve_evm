// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Canonical bounded delta transport, independent of execution/finality authority.

mod append_delta_field;
mod decode_state_delta_chunk;
mod decode_state_delta_payload;
mod decode_state_delta_request;
mod encode_state_delta_chunk;
mod encode_state_delta_payload;
mod encode_state_delta_request;
mod hash_state_delta_bytes;
mod measure_state_delta_execution_bytes;
mod measure_state_delta_payload;
mod preflight_state_delta_payload;
mod scan_state_delta_byte_list;
mod scan_state_delta_execution;
mod state_delta_payload_stats;
mod take_delta_field;
mod types;
mod validate_state_delta_chunk;

pub use decode_state_delta_chunk::decode_state_delta_chunk;
pub use decode_state_delta_payload::decode_state_delta_payload;
pub use decode_state_delta_request::decode_state_delta_request;
pub use encode_state_delta_chunk::encode_state_delta_chunk;
pub use encode_state_delta_payload::encode_state_delta_payload;
pub use encode_state_delta_request::encode_state_delta_request;
pub use hash_state_delta_bytes::hash_state_delta_bytes;
pub use measure_state_delta_execution_bytes::measure_state_delta_execution_bytes;
pub use measure_state_delta_payload::measure_state_delta_payload;
pub use preflight_state_delta_payload::preflight_state_delta_payload;
pub use state_delta_payload_stats::state_delta_payload_stats;
pub use types::{
    MAXIMUM_STATE_DELTA_CHUNK_BYTES, MAXIMUM_STATE_DELTA_PAYLOAD_BYTES,
    MAXIMUM_STATE_DELTA_REQUEST_BYTES, StateDeltaChunk, StateDeltaError, StateDeltaExecutionStats,
    StateDeltaPayload, StateDeltaPayloadPreflight, StateDeltaPayloadStats, StateDeltaRequest,
};
