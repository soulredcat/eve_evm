// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::AppliedConfig;
use eve_node_policy::SegmentedRecoveryPolicy;
use eve_storage::records::{
    OpaqueRecordCursor,
    segmented::{SegmentedCodecLimits, SegmentedRecoveryAnchor},
};

/// Common node settings plus an explicit logical/part profile; compact limit keeps its old meaning.
pub struct SegmentedAppliedConfig {
    pub application: AppliedConfig,
    pub policy: SegmentedRecoveryPolicy,
    pub codec: SegmentedCodecLimits,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedAppliedPosition {
    pub applied: SegmentedRecoveryAnchor,
    pub durable: SegmentedRecoveryAnchor,
    pub last_acknowledged_physical: OpaqueRecordCursor,
    pub missing_from: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SegmentedTailProgress {
    pub logical_target: u64,
    pub last_acknowledged_physical: OpaqueRecordCursor,
    pub complete_marker: Option<OpaqueRecordCursor>,
}
