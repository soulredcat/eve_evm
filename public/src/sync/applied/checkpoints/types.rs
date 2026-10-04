// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    AppliedError, AppliedPublication,
    resources::{EstimatedWorkingLease, EstimatedWorkingPool},
    types::ChargedAppliedState,
};
use eve_storage::checkpoints::proofs::{
    CheckpointProofLimits, CheckpointProofManifestStats, CheckpointProofTransfer,
    CompletedCheckpointProofStore,
};
use eve_storage::checkpoints::{
    CheckpointError, CheckpointLimits, CheckpointManifestStats, CheckpointTransfer,
    CompletedCheckpointStore,
};
use std::sync::Arc;

#[derive(Debug)]
pub enum CheckpointAppliedError {
    PendingMetadataRefused,
    Applied(AppliedError),
    Storage(CheckpointError),
    State(eve_state::StateError),
    Checkpoint(eve_finality_verifier::CheckpointError),
    Witness(eve_finality_verifier::CheckpointWitnessWireError),
    StaleParent,
    InvalidArtifactBinding,
    WrongMode,
    PendingDurability,
    Segmented(crate::persistence::segmented::SegmentedError),
    InvalidAcknowledgement,
    Base(eve_storage::records::segmented::checkpoints::CheckpointBaseError),
    Recovery(eve_storage::records::segmented::recovery::SegmentedRecoveryError),
    ScanLimit,
}

/// Local limits freeze independently from the peer's untrusted manifest.
#[derive(Clone, Copy, Debug)]
pub struct AppliedCheckpointLimits {
    pub content: CheckpointLimits,
    pub proofs: CheckpointProofLimits,
    pub verification: eve_finality_verifier::CheckpointLimits,
}

/// Locally configured artifact directories and startup work bounds, never peer-selected paths.
pub struct CheckpointRecoveryConfig {
    pub content_root: std::path::PathBuf,
    pub proof_root: std::path::PathBuf,
    pub limits: AppliedCheckpointLimits,
    pub maximum_scan_records: u64,
}

/// Actual owner pool charge surrounds every locally allocated transfer/manifest object.
pub struct ChargedCheckpointTransfer {
    pub(super) storage: Arc<super::super::resources::storage_admission::StorageAdmissionPool>,
    pub(super) transfer: CheckpointTransfer,
    pub(super) parent: Arc<AppliedPublication>,
    pub(super) working: Arc<EstimatedWorkingPool>,
    pub(super) limits: AppliedCheckpointLimits,
    pub(super) content_id: [u8; 32],
    pub(super) content: CheckpointManifestStats,
    pub(super) _metadata: EstimatedWorkingLease,
    pub(super) _staging:
        super::super::resources::storage_admission::AppliedSnapshotStagingReservation,
}

/// Synced file integrity and real metadata capacity; it grants no finality authority.
pub struct ChargedCompletedCheckpoint {
    pub(super) storage: Arc<super::super::resources::storage_admission::StorageAdmissionPool>,
    pub(super) content_store: CompletedCheckpointStore,
    pub(super) parent: Arc<AppliedPublication>,
    pub(super) working: Arc<EstimatedWorkingPool>,
    pub(super) limits: AppliedCheckpointLimits,
    pub(super) content_id: [u8; 32],
    pub(super) content: CheckpointManifestStats,
    pub(super) _metadata: EstimatedWorkingLease,
    pub(super) _staging:
        super::super::resources::storage_admission::AppliedSnapshotStagingReservation,
}

pub struct ChargedCheckpointProofTransfer {
    pub(super) content: ChargedCompletedCheckpoint,
    pub(super) transfer: CheckpointProofTransfer,
    pub(super) proof_id: [u8; 32],
    pub(super) proofs: CheckpointProofManifestStats,
    pub(super) _metadata: EstimatedWorkingLease,
    pub(super) _staging:
        super::super::resources::storage_admission::AppliedSnapshotStagingReservation,
}

/// Content and proof checksums remain untrusted until canonical finality preparation succeeds.
pub struct ChargedCheckpointArtifacts {
    pub(super) content: ChargedCompletedCheckpoint,
    pub(super) proof_store: CompletedCheckpointProofStore,
    pub(super) proof_id: [u8; 32],
    pub(super) proofs: CheckpointProofManifestStats,
    pub(super) _metadata: EstimatedWorkingLease,
    pub(super) _staging:
        super::super::resources::storage_admission::AppliedSnapshotStagingReservation,
}

/// Private authenticated candidate. Captured parent and artifact/generation charges remain alive.
pub struct PreparedAppliedCheckpoint {
    pub(super) artifacts: ChargedCheckpointArtifacts,
    pub(super) generation: Arc<ChargedAppliedState>,
    pub(super) target_binding: [u8; 32],
}

/// The exclusive owner retains candidates/artifacts even if the caller stops polling.
pub(in crate::sync::applied) struct PendingCheckpointActivation {
    pub(super) prepared: PreparedAppliedCheckpoint,
    pub(super) publication: Arc<AppliedPublication>,
    pub(super) record: crate::persistence::segmented::checkpoints::SealedCheckpointRecord,
    pub(super) ticket: Option<crate::persistence::segmented::checkpoints::CheckpointTicket>,
    pub(super) acknowledged: Option<crate::persistence::segmented::checkpoints::CheckpointAck>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointActivationOutcome {
    pub height: u64,
    pub durable_cursor: eve_storage::records::OpaqueRecordCursor,
}
