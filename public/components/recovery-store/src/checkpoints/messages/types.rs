// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{BlockPayload, Bytes, StateBudget, StateError, StateVersion};
pub const MAXIMUM_CHECKPOINT_REQUEST_BYTES: usize = 1_048_576;
pub const MAXIMUM_CHECKPOINT_MESSAGE_BODY_BYTES: usize = 32 * 1_048_576;
pub const MAXIMUM_CHECKPOINT_MESSAGE_MANIFEST_BYTES: usize = 262_144;
pub const MAXIMUM_CHECKPOINT_MESSAGE_CHUNK_BYTES: usize = 4_194_304;
pub(super) const REQUEST_DOMAIN: &[u8] = b"EVE_CHECKPOINT_REQUEST_V1";
pub(super) const RESPONSE_DOMAIN: &[u8] = b"EVE_CHECKPOINT_RESPONSE_V1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckpointRequestKind {
    Manifest {
        chunk_bytes: u32,
    },
    Chunk {
        chunk_bytes: u32,
        manifest_id: [u8; 32],
        index: u32,
    },
    Execution,
}
/// Locally configured genesis is an equality constraint, never remotely granted trust.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CheckpointRequest {
    pub genesis: StateVersion,
    pub height: u64,
    pub kind: CheckpointRequestKind,
}
#[derive(Clone, Copy, Debug)]
pub struct CheckpointMessageLimits {
    pub logical: StateBudget,
    pub maximum_body_bytes: usize,
    pub maximum_manifest_bytes: usize,
    pub maximum_chunk_bytes: usize,
}
#[derive(Clone, Debug, PartialEq)]
pub enum CheckpointResponse {
    Manifest {
        target: StateVersion,
        durable_tip: Box<StateVersion>,
        manifest_id: [u8; 32],
        manifest: Vec<u8>,
    },
    Chunk {
        target: StateVersion,
        manifest_id: [u8; 32],
        body_sha256: [u8; 32],
        total_length: u64,
        index: u32,
        chunk_bytes: u32,
        data: Bytes,
    },
    Execution {
        target: StateVersion,
        block: Box<BlockPayload>,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CheckpointMessageError {
    UnsupportedVersion,
    MalformedEncoding,
    BudgetExceeded,
    ArithmeticOverflow,
    ReservationTooSmall,
    AllocationFailed,
    State(StateError),
    ManifestInvalid,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointResponseKind {
    Manifest,
    Chunk,
    Execution,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointResponseStats {
    pub encoded_bytes: usize,
    pub target_network_bytes: usize,
    pub tip_network_bytes: usize,
    pub payload_bytes: usize,
}
/// Sealed same-byte resource admission only; no finality, storage, freshness or lease authority.
#[derive(Debug)]
pub struct CheckpointResponsePreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) limits: CheckpointMessageLimits,
    pub(super) kind: CheckpointResponseKind,
    pub(super) target: &'a [u8],
    pub(super) tip: Option<&'a [u8]>,
    pub(super) manifest_id: Option<[u8; 32]>,
    pub(super) body_hash: Option<[u8; 32]>,
    pub(super) total: u64,
    pub(super) index: u32,
    pub(super) width: u32,
    pub(super) payload: &'a [u8],
    pub(super) stats: CheckpointResponseStats,
}
