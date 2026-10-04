// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

//! Locally anchored, streaming certified checkpoint verification; never independent EVM replay.
mod begin_authenticated_checkpoint;
mod checkpoint_anchor;
mod checkpoint_commit;
mod finish_authenticated_checkpoint;
mod into_imported_checkpoint_state;
mod required_checkpoint_reservation;
mod types;
mod validate_checkpoint_witness_bounds;
mod verify_checkpoint_witness;
pub use begin_authenticated_checkpoint::begin_authenticated_checkpoint;
pub use checkpoint_anchor::checkpoint_anchor;
pub use checkpoint_commit::checkpoint_commit;
pub use finish_authenticated_checkpoint::finish_authenticated_checkpoint;
pub use into_imported_checkpoint_state::into_imported_checkpoint_state;
pub use required_checkpoint_reservation::required_checkpoint_reservation;
pub use types::{
    AuthenticatedCheckpoint, CheckpointError, CheckpointExecutionWitness, CheckpointLimits,
    CheckpointSession, CheckpointWitness,
};
pub use verify_checkpoint_witness::verify_checkpoint_witness;
