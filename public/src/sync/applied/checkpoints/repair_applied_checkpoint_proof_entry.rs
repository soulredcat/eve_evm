// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointAppliedError, pending_metadata_types::CheckpointMetadataRepairStatus};
use eve_storage::checkpoints::{
    CheckpointError, CheckpointPendingMetadataStatus,
    proofs::{
        CheckpointProofManifestPreflight, CheckpointProofPendingKind,
        repair_invalid_checkpoint_proof_pending,
    },
};
use std::fs::File;

pub(super) fn repair_applied_checkpoint_proof_entry(
    base: &File,
    manifest: &CheckpointProofManifestPreflight<'_>,
    kind: CheckpointProofPendingKind,
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
    match repair_invalid_checkpoint_proof_pending(base, manifest, kind, reserved_metadata) {
        Ok(()) => Ok(CheckpointMetadataRepairStatus::RemovedInvalid),
        Err(CheckpointError::AlreadyComplete) => {
            Ok(CheckpointMetadataRepairStatus::PublishedCompletionPresent)
        }
        Err(CheckpointError::ValidEntry) => Err(CheckpointAppliedError::PendingMetadataRefused),
        Err(error) => Err(CheckpointAppliedError::Storage(error)),
    }
}
