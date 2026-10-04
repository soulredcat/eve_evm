// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Bounded uncompressed local checkpoint staging. No authentication or activation authority.
mod begin_checkpoint_transfer;
mod checkpoint_body_bytes;
mod checkpoint_completion_bytes;
mod checkpoint_file_name;
mod checkpoint_initial_resume_observation;
mod checkpoint_manifest_id;
mod checkpoint_manifest_stats;
mod checkpoint_namespace_name;
mod checkpoint_store_manifest_bytes;
mod checkpoint_target_encoding;
mod complete_checkpoint_transfer;
mod create_checkpoint_manifest;
mod ensure_uncompleted_checkpoint;
mod hash_checkpoint_file;
mod is_valid_checkpoint_pending_completion;
mod is_valid_checkpoint_pending_manifest;
pub mod messages;
mod observe_checkpoint_chunk;
mod observe_checkpoint_pending_metadata;
mod open_checkpoint_file;
mod open_checkpoint_namespace;
mod open_completed_checkpoint_store;
mod pending_metadata_types;
mod preflight_checkpoint_body;
mod preflight_checkpoint_manifest;
pub mod proofs;
mod publish_checkpoint_metadata;
mod read_checkpoint_body;
mod read_checkpoint_metadata;
mod read_checkpoint_reference;
mod repair_invalid_checkpoint_chunk;
mod repair_invalid_checkpoint_completion_pending;
mod repair_invalid_checkpoint_manifest_pending;
mod required_checkpoint_body_reservation;
mod required_checkpoint_io_reservation;
mod required_checkpoint_metadata_reservation;
mod scan_checkpoint_resume;
mod types;
mod unlink_invalid_checkpoint_entry;
mod validate_checkpoint_chunks;
mod validate_checkpoint_directory;
mod validate_checkpoint_file;
mod validate_checkpoint_limits;
mod validate_checkpoint_metadata;
mod write_checkpoint_chunk;
pub use begin_checkpoint_transfer::begin_checkpoint_transfer;
pub use checkpoint_body_bytes::checkpoint_body_bytes;
pub use checkpoint_initial_resume_observation::checkpoint_initial_resume_observation;
pub use checkpoint_manifest_id::checkpoint_manifest_id;
pub use checkpoint_manifest_stats::checkpoint_manifest_stats;
pub use checkpoint_store_manifest_bytes::checkpoint_store_manifest_bytes;
pub use checkpoint_target_encoding::checkpoint_target_encoding;
pub use complete_checkpoint_transfer::complete_checkpoint_transfer;
pub use create_checkpoint_manifest::create_checkpoint_manifest;
pub use observe_checkpoint_chunk::observe_checkpoint_chunk;
pub use observe_checkpoint_pending_metadata::observe_checkpoint_pending_metadata;
pub use open_completed_checkpoint_store::open_completed_checkpoint_store;
pub use pending_metadata_types::{CheckpointPendingMetadataKind, CheckpointPendingMetadataStatus};
pub use preflight_checkpoint_body::preflight_checkpoint_body;
pub use preflight_checkpoint_manifest::preflight_checkpoint_manifest;
pub use read_checkpoint_body::read_checkpoint_body;
pub use repair_invalid_checkpoint_chunk::repair_invalid_checkpoint_chunk;
pub use repair_invalid_checkpoint_completion_pending::repair_invalid_checkpoint_completion_pending;
pub use repair_invalid_checkpoint_manifest_pending::repair_invalid_checkpoint_manifest_pending;
pub use required_checkpoint_body_reservation::required_checkpoint_body_reservation;
pub use required_checkpoint_io_reservation::required_checkpoint_io_reservation;
pub use required_checkpoint_metadata_reservation::required_checkpoint_metadata_reservation;
pub use types::{
    CheckpointBody, CheckpointChunkStatus, CheckpointError, CheckpointLimits,
    CheckpointManifestPreflight, CheckpointManifestStats, CheckpointResumeObservation,
    CheckpointTransfer, CompletedCheckpointStore, MAXIMUM_CHECKPOINT_CHUNK_BYTES,
};
pub use write_checkpoint_chunk::write_checkpoint_chunk;
