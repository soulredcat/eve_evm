// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    read_discovered_checkpoint_base::read_discovered_checkpoint_base,
    types::DiscoveredCheckpointBase,
};
use crate::sync::applied::{
    AppliedError,
    checkpoints::CheckpointAppliedError,
    resources::{
        EstimatedWorkingPool, estimated_repository_read_charge, reserve_estimated_working,
    },
};
use eve_storage::records::{
    OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_bootstrap_cursor,
    opaque_record_budget, opaque_record_cursor, read_opaque_record,
};
use std::sync::Arc;

/// One bounded actual row at a time; any malformed latest base or broken physical chain fails closed.
pub(in crate::sync::applied) fn find_latest_checkpoint_base(
    repository: &OpaqueRecordRepository,
    maximum_scan_records: u64,
    working: &Arc<EstimatedWorkingPool>,
) -> Result<Option<DiscoveredCheckpointBase>, CheckpointAppliedError> {
    let budget = opaque_record_budget(repository);
    if maximum_scan_records == 0 || maximum_scan_records > budget.maximum_retained_records {
        return Err(CheckpointAppliedError::ScanLimit);
    }
    let head = opaque_record_cursor(repository)
        .map_err(|_| CheckpointAppliedError::Applied(AppliedError::StorageUnavailable))?;
    if head.sequence > maximum_scan_records {
        return Err(CheckpointAppliedError::ScanLimit);
    }
    let _read = reserve_estimated_working(
        working,
        estimated_repository_read_charge(&budget).map_err(CheckpointAppliedError::Applied)?,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let mut previous = opaque_record_bootstrap_cursor(repository);
    let mut latest = None;
    for sequence in 1..=head.sequence {
        let row = read_opaque_record(repository, sequence)
            .map_err(|_| CheckpointAppliedError::Applied(AppliedError::StorageUnavailable))?
            .ok_or(CheckpointAppliedError::Applied(
                AppliedError::InvalidDurablePrefix,
            ))?;
        if row.sequence != sequence || row.parent != previous {
            return Err(CheckpointAppliedError::Applied(
                AppliedError::InvalidDurablePrefix,
            ));
        }
        if row.payload.starts_with(b"EVE_CHECKPOINT_BASE") {
            latest = Some(read_discovered_checkpoint_base(repository, &row, working)?);
        }
        previous = OpaqueRecordCursor {
            sequence,
            content_hash: row.content_hash,
        };
    }
    if previous != head {
        return Err(CheckpointAppliedError::Applied(
            AppliedError::InvalidDurablePrefix,
        ));
    }
    Ok(latest)
}
