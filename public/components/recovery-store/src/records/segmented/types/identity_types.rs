// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::OpaqueRecordCursor;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SegmentedRecoveryMode {
    AuthenticatedImport = 1,
}

/// Local logical-parent position and caller-selected state identity. No field
/// proves authenticated state, finality, enrollment, freshness or durability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedRecoveryAnchor {
    pub height: u64,
    pub cursor: OpaqueRecordCursor,
    pub state_binding: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedLogicalIdentity {
    pub mode: SegmentedRecoveryMode,
    pub logical_id: [u8; 32],
    pub parent: SegmentedRecoveryAnchor,
    pub target_height: u64,
    pub total_length: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedSegmentMetadata {
    pub identity: SegmentedLogicalIdentity,
    pub index: u32,
    pub count: u32,
    pub offset: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedMarkerMetadata {
    pub identity: SegmentedLogicalIdentity,
    /// Hash of local canonical target representation; not independently certified.
    pub target_state_binding: [u8; 32],
}

pub(in crate::records::segmented) struct SegmentedBase {
    pub(in crate::records::segmented) kind: super::SegmentedRecordKind,
    pub(in crate::records::segmented) mode: SegmentedRecoveryMode,
    pub(in crate::records::segmented) logical_id: [u8; 32],
    pub(in crate::records::segmented) parent: SegmentedRecoveryAnchor,
    pub(in crate::records::segmented) target_height: u64,
}
