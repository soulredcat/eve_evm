// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::segmented::{
    SEGMENTED_MAX_REFERENCES, SegmentedCodecError, SegmentedCodecLimits, SegmentedLogicalIdentity,
    SegmentedRecoveryAnchor,
};
use crate::records::{OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmentedRecoveryError {
    Codec(SegmentedCodecError),
    StorageFailed,
    InvalidAnchor,
    ForeignRepository,
    WrongPhysicalParent,
    InvalidSuffix,
    InvalidMarker,
    LogicalHashMismatch,
    ReservationTooSmall,
    AllocationFailed,
    ArithmeticOverflow,
    ConfirmationRequired,
    WrongConfirmation,
    FailedScanner,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Candidate {
    pub identity: SegmentedLogicalIdentity,
    pub references: [OpaqueRecordCursor; SEGMENTED_MAX_REFERENCES],
    pub received: usize,
    pub expected: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Completion {
    pub identity: SegmentedLogicalIdentity,
    pub target: SegmentedRecoveryAnchor,
    pub references: [OpaqueRecordCursor; SEGMENTED_MAX_REFERENCES],
    pub count: usize,
}

/// Bounded local reader state. The caller keeps actual capacity leased across
/// this object, returned bundles, verification and buffer return. No finality.
#[derive(Debug)]
pub struct SegmentedRecoveryScan {
    pub(super) namespace: OpaqueRecordIdentity,
    pub(super) budget: OpaqueRecordBudget,
    pub(super) limits: SegmentedCodecLimits,
    pub(super) anchor: SegmentedRecoveryAnchor,
    pub(super) physical: OpaqueRecordCursor,
    pub(super) candidate: Option<Candidate>,
    pub(super) pending: Option<Completion>,
    pub(super) body: Vec<u8>,
    pub(super) orphan_segments: u64,
    pub(super) failed: bool,
}

/// Complete physical membership and logical SHA identity only. Public must still
/// verify EVE_IMPORT_V2/H+1 and exact local target binding before confirmation.
#[derive(Debug)]
pub struct RecoveredSegmentedBundle {
    pub(super) namespace: OpaqueRecordIdentity,
    pub(super) completion: Completion,
    pub(super) body: Vec<u8>,
}

#[derive(Debug)]
pub enum SegmentedRecoveryStep {
    Progress { rows_scanned: usize },
    Incomplete { rows_scanned: usize },
    Complete(Box<RecoveredSegmentedBundle>),
    Exhausted,
}

#[derive(Debug)]
pub struct RejectedSegmentedRecovery {
    pub error: SegmentedRecoveryError,
    pub bundle: Box<RecoveredSegmentedBundle>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedRecoveryObservation {
    pub logical_anchor: SegmentedRecoveryAnchor,
    pub physical_cursor: OpaqueRecordCursor,
    pub received_segments: usize,
    pub expected_segments: usize,
    pub orphan_segments: u64,
    pub awaiting_confirmation: bool,
    pub failed: bool,
}
