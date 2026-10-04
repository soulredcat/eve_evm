// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{
    resources::EstimatedWorkingLease, segmented::SegmentedAppliedPosition,
    types::ChargedAppliedState,
};
use eve_state::StateVersion;
use eve_storage::records::segmented::checkpoints::{
    CheckpointBaseMembership, PreparedCheckpointBaseTarget,
};
use std::sync::Arc;

pub(in crate::sync::applied) struct DiscoveredCheckpointBase {
    pub(super) membership: CheckpointBaseMembership,
    pub(super) target: PreparedCheckpointBaseTarget,
    pub(super) version: StateVersion,
    pub(super) _lease: EstimatedWorkingLease,
}

pub(in crate::sync::applied) struct RecoveredCheckpointPrefix {
    pub(in crate::sync::applied) generation: Arc<ChargedAppliedState>,
    pub(in crate::sync::applied) position: SegmentedAppliedPosition,
    pub(in crate::sync::applied) checkpoint_height: u64,
    pub(in crate::sync::applied) base: Option<DiscoveredCheckpointBase>,
}
