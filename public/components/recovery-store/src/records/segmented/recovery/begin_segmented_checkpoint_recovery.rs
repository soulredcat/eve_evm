// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    required_segmented_checkpoint_recovery_reservation::required_segmented_checkpoint_recovery_reservation,
    types::{SegmentedRecoveryError, SegmentedRecoveryScan},
};
use crate::records::{
    OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_budget, opaque_record_identity,
    segmented::{
        SegmentedCodecLimits, SegmentedRecoveryAnchor,
        checkpoints::{
            CHECKPOINT_BASE_MAX_PAYLOAD_BYTES, CheckpointBaseLimits, CheckpointBaseMembership,
            PreparedCheckpointBaseTarget, checkpoint_base_membership_record,
            checkpoint_base_membership_view, read_checkpoint_base_membership,
        },
    },
};

/// The caller must authenticate the complete referenced snapshot and genesis proof
/// chain before selecting this base. This rechecks local membership, never proof.
pub fn begin_segmented_checkpoint_recovery(
    repository: &OpaqueRecordRepository,
    membership: &CheckpointBaseMembership,
    target: &PreparedCheckpointBaseTarget,
    limits: &SegmentedCodecLimits,
    reserved_bytes: usize,
) -> Result<SegmentedRecoveryScan, SegmentedRecoveryError> {
    let budget = opaque_record_budget(repository);
    if reserved_bytes < required_segmented_checkpoint_recovery_reservation(&budget, limits)? {
        return Err(SegmentedRecoveryError::ReservationTooSmall);
    }
    let supplied = checkpoint_base_membership_record(membership);
    let cursor = OpaqueRecordCursor {
        sequence: supplied.sequence,
        content_hash: supplied.content_hash,
    };
    let expected = checkpoint_base_membership_view(membership).metadata;
    let base_limits = CheckpointBaseLimits {
        maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
    };
    let actual = read_checkpoint_base_membership(
        repository,
        cursor,
        expected,
        target,
        &base_limits,
        reserved_bytes,
    )
    .map_err(|_| SegmentedRecoveryError::InvalidAnchor)?;
    if checkpoint_base_membership_record(&actual) != supplied {
        return Err(SegmentedRecoveryError::InvalidAnchor);
    }
    let metadata = checkpoint_base_membership_view(&actual).metadata;
    let anchor = SegmentedRecoveryAnchor {
        height: metadata.target_height,
        cursor,
        state_binding: metadata.target_state_binding,
    };
    let mut body = Vec::new();
    body.try_reserve_exact(limits.maximum_logical_bytes)
        .map_err(|_| SegmentedRecoveryError::AllocationFailed)?;
    if body.capacity() != limits.maximum_logical_bytes {
        return Err(SegmentedRecoveryError::AllocationFailed);
    }
    Ok(SegmentedRecoveryScan {
        namespace: opaque_record_identity(repository),
        budget,
        limits: *limits,
        anchor,
        physical: cursor,
        candidate: None,
        pending: None,
        body,
        orphan_segments: 0,
        failed: false,
    })
}
