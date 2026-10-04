// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecord, OpaqueRecordCursor, segmented::SegmentedRecoveryAnchor};
use eve_state::Bytes;

pub(super) const DOMAIN: &[u8; 22] = b"EVE_CHECKPOINT_BASE_V1";
pub(super) const HEADER_BYTES: usize = 364;
pub const CHECKPOINT_BASE_MAX_VERSION_BYTES: usize = 4_096;
pub const CHECKPOINT_BASE_MAX_PAYLOAD_BYTES: usize =
    HEADER_BYTES + CHECKPOINT_BASE_MAX_VERSION_BYTES + 32;

/// Verification mode only. The canonical target retains its distinct security profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum CheckpointBaseMode {
    AuthenticatedImport = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointBaseLimits {
    pub maximum_payload_bytes: usize,
}

/// Caller-selected artifact references and ranges. Construction grants no authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointBaseMetadata {
    pub mode: CheckpointBaseMode,
    pub previous_opaque_cursor: OpaqueRecordCursor,
    pub previous_logical_anchor: SegmentedRecoveryAnchor,
    pub target_height: u64,
    pub target_state_binding: [u8; 32],
    pub snapshot_manifest_id: [u8; 32],
    pub snapshot_body_hash: [u8; 32],
    pub proof_manifest_id: [u8; 32],
    pub proof_root: [u8; 32],
    pub proof_genesis_height: u64,
    pub execution_start: u64,
    pub execution_end: u64,
    pub lookahead_height: u64,
    pub retained_start: u64,
    pub retained_end: u64,
}

/// Canonical encoding prepared by eve-state. Neither local metadata nor its hash is authenticated.
#[derive(Debug)]
pub struct PreparedCheckpointBaseTarget {
    pub(super) encoded: Bytes,
    pub(super) height: u64,
    pub(super) security_profile: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointBaseView<'a> {
    pub metadata: CheckpointBaseMetadata,
    pub security_profile: u8,
    pub target_version_bytes: &'a [u8],
}

/// Sealed framing/integrity result over the same immutable bytes, not a state proof.
#[derive(Debug)]
pub struct CheckpointBasePreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) view: CheckpointBaseView<'a>,
}

/// Actual stored row with validated local membership. Artifacts and finality remain unverified.
#[derive(Debug)]
pub struct CheckpointBaseMembership {
    pub(super) record: OpaqueRecord,
    pub(super) metadata: CheckpointBaseMetadata,
    pub(super) security_profile: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointBaseError {
    InvalidLimits,
    LimitExceeded,
    InvalidTarget,
    MalformedEncoding,
    UnsupportedVersion,
    UnsupportedMode,
    InvalidMetadata,
    ParentMismatch,
    TargetMismatch,
    HashMismatch,
    InsufficientReservation,
    ArithmeticOverflow,
    AllocationFailed,
    StorageFailed,
    MissingRecord,
    MembershipMismatch,
}
