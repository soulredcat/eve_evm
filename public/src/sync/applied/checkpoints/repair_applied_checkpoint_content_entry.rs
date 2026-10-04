// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointAppliedError, pending_metadata_types::CheckpointMetadataRepairStatus};
use eve_storage::checkpoints::{
    CheckpointError, CheckpointManifestPreflight, CheckpointPendingMetadataKind,
    CheckpointPendingMetadataStatus, repair_invalid_checkpoint_completion_pending,
    repair_invalid_checkpoint_manifest_pending,
};
use std::fs::File;

pub(super) fn repair_applied_checkpoint_content_entry(
    base: &File,
    manifest: &CheckpointManifestPreflight<'_>,
    kind: CheckpointPendingMetadataKind,
    status: CheckpointPendingMetadataStatus,
    reserved_metadata: usize,
) -> Result<CheckpointMetadataRepairStatus, CheckpointAppliedError> {
    match status {
        CheckpointPendingMetadataStatus::Missing => {
            return Ok(CheckpointMetadataRepairStatus::Missing);
        }
        CheckpointPendingMetadataStatus::MatchingValid => {
            return Ok(CheckpointMetadataRepairStatus::MatchingValid);
        }
        CheckpointPendingMetadataStatus::PublishedCompletionPresent => {
            return Ok(CheckpointMetadataRepairStatus::PublishedCompletionPresent);
        }
        CheckpointPendingMetadataStatus::Refused => {
            return Err(CheckpointAppliedError::PendingMetadataRefused);
        }
        CheckpointPendingMetadataStatus::InvalidKnown => {}
    }
    let result = match kind {
        CheckpointPendingMetadataKind::Manifest => {
            repair_invalid_checkpoint_manifest_pending(base, manifest, reserved_metadata)
        }
        CheckpointPendingMetadataKind::Completion => {
            repair_invalid_checkpoint_completion_pending(base, manifest, reserved_metadata)
        }
    };
    match result {
        Ok(()) => Ok(CheckpointMetadataRepairStatus::RemovedInvalid),
        Err(CheckpointError::AlreadyComplete) => {
            Ok(CheckpointMetadataRepairStatus::PublishedCompletionPresent)
        }
        Err(CheckpointError::ValidEntry) => Err(CheckpointAppliedError::PendingMetadataRefused),
        Err(error) => Err(CheckpointAppliedError::Storage(error)),
    }
}
