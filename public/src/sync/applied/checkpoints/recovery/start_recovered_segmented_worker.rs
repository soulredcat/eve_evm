// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::RecoveredCheckpointPrefix;
use crate::{
    persistence::segmented::{
        SegmentedPartPool, SegmentedWorker, checkpoints::start_segmented_worker_from_checkpoint,
        start_segmented_worker,
    },
    sync::applied::checkpoints::CheckpointAppliedError,
};
use eve_storage::records::{
    OpaqueRecordRepository,
    segmented::checkpoints::{
        CHECKPOINT_BASE_MAX_PAYLOAD_BYTES, CheckpointBaseLimits, checkpoint_base_membership_record,
    },
};
use std::sync::Arc;

pub(in crate::sync::applied) fn start_recovered_segmented_worker(
    repository: OpaqueRecordRepository,
    pool: Arc<SegmentedPartPool>,
    recovered: &RecoveredCheckpointPrefix,
) -> Result<SegmentedWorker, CheckpointAppliedError> {
    if let Some(base) = &recovered.base {
        let record = checkpoint_base_membership_record(&base.membership);
        if recovered.position.durable.cursor.sequence == record.sequence {
            return start_segmented_worker_from_checkpoint(
                repository,
                pool,
                &base.membership,
                &base.target,
                CheckpointBaseLimits {
                    maximum_payload_bytes: CHECKPOINT_BASE_MAX_PAYLOAD_BYTES,
                },
            )
            .map_err(CheckpointAppliedError::Segmented);
        }
    }
    start_segmented_worker(repository, pool, recovered.position.durable)
        .map_err(CheckpointAppliedError::Segmented)
}
