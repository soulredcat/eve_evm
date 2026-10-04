// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Owner-charged checkpoint staging and canonical finality preparation.
mod begin_applied_checkpoint_proofs;
mod begin_applied_checkpoint_transfer;
mod build_checkpoint_base_metadata;
mod build_checkpoint_publication;
mod complete_applied_checkpoint_proofs;
mod complete_applied_checkpoint_transfer;
mod fence_checkpoint_activation;
mod observe_applied_checkpoint_chunk;
mod observe_applied_checkpoint_witness;
mod pending_metadata_types;
mod poll_applied_checkpoint_activation;
mod prepare_applied_checkpoint;
pub(in crate::sync::applied) mod recovery;
mod repair_applied_checkpoint_chunk;
mod repair_applied_checkpoint_content_entry;
mod repair_applied_checkpoint_content_pending;
mod repair_applied_checkpoint_proof_entry;
mod repair_applied_checkpoint_proof_pending;
mod repair_applied_checkpoint_witness;
mod retained_checkpoint_activation;
pub use recovery::open_segmented_applied_state_service_with_checkpoints;
mod start_applied_checkpoint_activation;
#[cfg(test)]
#[cfg(target_os = "linux")]
pub(crate) mod tests;
mod types;
mod validate_checkpoint_acknowledgement;
mod validate_checkpoint_activation_parent;
mod validate_full_checkpoint_stream;
mod verify_applied_checkpoint_witness;
mod write_applied_checkpoint_chunk;
mod write_applied_checkpoint_witness;
pub use begin_applied_checkpoint_proofs::begin_applied_checkpoint_proofs;
pub use begin_applied_checkpoint_transfer::begin_applied_checkpoint_transfer;
pub use complete_applied_checkpoint_proofs::complete_applied_checkpoint_proofs;
pub use complete_applied_checkpoint_transfer::complete_applied_checkpoint_transfer;
pub use observe_applied_checkpoint_chunk::observe_applied_checkpoint_chunk;
pub use observe_applied_checkpoint_witness::observe_applied_checkpoint_witness;
pub use pending_metadata_types::{CheckpointMetadataRepairOutcome, CheckpointMetadataRepairStatus};
pub use poll_applied_checkpoint_activation::poll_applied_checkpoint_activation;
pub use prepare_applied_checkpoint::prepare_applied_checkpoint;
pub use repair_applied_checkpoint_chunk::repair_applied_checkpoint_chunk;
pub use repair_applied_checkpoint_content_pending::repair_applied_checkpoint_content_pending;
pub use repair_applied_checkpoint_proof_pending::repair_applied_checkpoint_proof_pending;
pub use repair_applied_checkpoint_witness::repair_applied_checkpoint_witness;
pub use retained_checkpoint_activation::retained_checkpoint_activation;
pub use start_applied_checkpoint_activation::start_applied_checkpoint_activation;
pub(super) use types::PendingCheckpointActivation;
pub use types::{
    AppliedCheckpointLimits, ChargedCheckpointArtifacts, ChargedCheckpointProofTransfer,
    ChargedCheckpointTransfer, ChargedCompletedCheckpoint, CheckpointActivationOutcome,
    CheckpointAppliedError, CheckpointRecoveryConfig, PreparedAppliedCheckpoint,
};
pub use write_applied_checkpoint_chunk::write_applied_checkpoint_chunk;
pub use write_applied_checkpoint_witness::write_applied_checkpoint_witness;
